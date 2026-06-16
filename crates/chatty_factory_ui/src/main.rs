use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

mod catalog_governance_panels;
mod extension_governance_panel;
mod extension_workbench_panel;
mod governance_ui;
mod patch_xray_panel;
mod proof_history_panel;
mod proof_run_panel;
mod request_action_panel;
mod runtime_registry_dashboard;

use chatty_factory_core::{
    build_starter_label, built_in_proof_templates, capability_comparison_bundle_manifest_path,
    capability_comparison_bundles_from_root, proof_template_manifest_path,
    proof_templates_from_root, AcceptanceRecipeStatus, CapabilityComparisonBundle,
    ChattyCogBridgeCapabilities, OperatorBundleStatus, PatchLaneStatus, PrimitiveProofTemplate,
    ProjectBrowserState, ProjectSpec, RuntimeConfig, RuntimeModelCatalogReceipt,
};
use chatty_factory_families::refresh_project_contract_views_for_project;
use chatty_factory_host::{
    HostActionResult, HostBridge, HostExtensionRegistryView, HostPlannerOptions,
    PendingExtensionEntry,
};
use chrono::{Local, Utc};
use eframe::{egui, App, Frame, NativeOptions};
use governance_ui::{
    governance_cooldown_warning, governance_never_refreshed_summary,
    governance_never_refreshed_warning, governance_refresh_status_summary,
    governance_stale_warning, render_governance_detail_block, render_governance_metric_strip,
    render_governance_refresh_state,
};

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "ChattyFactory UI",
        options,
        Box::new(|_cc| Box::new(ChattyFactoryUiApp::new())),
    )
}

#[derive(Debug, Clone)]
enum UiTask {
    RefreshBrowser,
    RefreshRuntime,
    RefreshProjectPatchReadiness,
    RefreshProofHarnessRegistry,
    RefreshCompositionGovernance,
    RefreshPatchGovernance,
    RefreshHelperGovernance,
    RefreshBridgeGovernance,
    RefreshFamilyGovernance,
    RefreshTemplateGovernance,
    ApproveProposedConstraint {
        request_id_or_path: String,
    },
    SetApprovedConstraintActive {
        constraint_id: String,
        active: bool,
    },
    ArchiveUnmatchedInactiveConstraints,
    DeactivateLowValueActiveConstraints,
    RestoreApprovedConstraint {
        constraint_id: String,
    },
    SelectProject {
        project_name: String,
    },
    ClearSelectedProject,
    ImplementExtension {
        entry_id: String,
    },
    ValidateExtension {
        entry_id: String,
    },
    PrepareExtensionPromotion {
        entry_id: String,
    },
    PrepareExtensionApplyPatch {
        entry_id: String,
    },
    ConsumeExtensionApplyPatch {
        entry_id: String,
    },
    ValidateLiveExtension {
        entry_id: String,
    },
    ArchiveExtension {
        entry_id: String,
        reason: String,
    },
    RunProofTemplate {
        template_id: String,
        request: String,
        auto_planner: bool,
        port: String,
        model: String,
    },
    RunRetrySearchLadderProof {
        auto_planner: bool,
        port: String,
        model: String,
    },
    BuildRequest {
        request: String,
        starter_override_id: Option<String>,
        auto_planner: bool,
        port: String,
        model: String,
    },
    PatchRequest {
        project_name: String,
        request: String,
        auto_planner: bool,
        port: String,
        model: String,
    },
}

#[derive(Debug)]
struct UiTaskResult {
    summary: String,
    stdout: String,
    stderr: String,
    browser_state: Option<ProjectBrowserState>,
    action_summary: Option<ActionSummary>,
    execution_result: Option<UiExecutionResult>,
    fallback_result: Option<UiFallbackResult>,
    extension_registry: Option<HostExtensionRegistryView>,
}

#[derive(Debug, Clone, Default)]
struct ActionSummary {
    title: String,
    lines: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct RuntimeStatusView {
    config: Option<RuntimeConfig>,
    catalog: Option<RuntimeModelCatalogReceipt>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ProofGovernanceRefreshStatusView {
    status_id: String,
    refreshed_entries: usize,
    skipped_entries: usize,
    updated_at: String,
    #[serde(skip)]
    refreshed_at_label: Option<String>,
    #[serde(skip)]
    age_minutes: Option<u64>,
    #[serde(skip)]
    newer_proof_receipts_exist: bool,
    #[serde(skip)]
    latest_proof_receipt_label: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct CompositionGovernanceRefreshStatusView {
    status_id: String,
    refreshed_entries: usize,
    skipped_entries: usize,
    updated_at: String,
    #[serde(skip)]
    refreshed_at_label: Option<String>,
    #[serde(skip)]
    age_minutes: Option<u64>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct PatchGovernanceRefreshStatusView {
    status_id: String,
    refreshed_entries: usize,
    skipped_entries: usize,
    updated_at: String,
    #[serde(skip)]
    refreshed_at_label: Option<String>,
    #[serde(skip)]
    age_minutes: Option<u64>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct HelperGovernanceRefreshStatusView {
    status_id: String,
    refreshed_entries: usize,
    skipped_entries: usize,
    updated_at: String,
    #[serde(skip)]
    refreshed_at_label: Option<String>,
    #[serde(skip)]
    age_minutes: Option<u64>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BridgeGovernanceRefreshStatusView {
    status_id: String,
    refreshed_entries: usize,
    skipped_entries: usize,
    updated_at: String,
    #[serde(skip)]
    refreshed_at_label: Option<String>,
    #[serde(skip)]
    age_minutes: Option<u64>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct FamilyGovernanceRefreshStatusView {
    status_id: String,
    refreshed_entries: usize,
    skipped_entries: usize,
    updated_at: String,
    #[serde(skip)]
    refreshed_at_label: Option<String>,
    #[serde(skip)]
    age_minutes: Option<u64>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct FamilyUsageEntryView {
    family_id: String,
    family_display_name: String,
    family_ecosystem: Option<String>,
    project_count: usize,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct FamilyUsageSummaryView {
    summary_id: String,
    total_projects: usize,
    #[serde(default)]
    ecosystem_project_counts: BTreeMap<String, usize>,
    #[serde(default)]
    families: Vec<FamilyUsageEntryView>,
    updated_at: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct StarterUsageEntryView {
    starter_id: String,
    starter_label: String,
    starter_lifecycle: String,
    build_count: usize,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct StarterUsageSummaryView {
    summary_id: String,
    total_build_receipts: usize,
    explicit_override_builds: usize,
    auto_routed_builds: usize,
    matched_recommendation_builds: usize,
    overridden_recommendation_builds: usize,
    #[serde(default)]
    starters: Vec<StarterUsageEntryView>,
    updated_at: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TriangulationLoopSummaryView {
    summary_id: String,
    open_provisional_vault_entries: usize,
    triangulation_session_count: usize,
    floor_level_convergent_failures: usize,
    pending_promotion_candidates: usize,
    current_model_only_exhaustion_count: usize,
    full_model_ladder_exhaustion_count: usize,
    latest_floor_task_label: Option<String>,
    latest_floor_granularity: Option<String>,
    latest_floor_decision: Option<String>,
    latest_session_label: Option<String>,
    latest_session_convergence_posture: Option<String>,
    latest_candidate_label: Option<String>,
    latest_candidate_confidence_posture: Option<String>,
    latest_candidate_proposal_path: Option<String>,
    #[serde(default)]
    latest_model_ladder_task_label: Option<String>,
    #[serde(default)]
    latest_model_ladder_posture: Option<String>,
    #[serde(default)]
    latest_model_ladder_attempted_models: Vec<String>,
    updated_at: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct RetrySearchProofReceiptView {
    receipt_id: String,
    proof_kind: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    final_outcome: Option<String>,
    requested_model_selector: Option<String>,
    #[serde(default)]
    model_candidate_count: usize,
    #[serde(default)]
    retry_posture_count: usize,
    #[serde(default)]
    launch_timeout_secs: u64,
    #[serde(default)]
    request_timeout_secs: u64,
    #[serde(default)]
    cleanup_overhead_secs: u64,
    #[serde(default)]
    expected_outer_timeout_secs: u64,
    #[serde(default)]
    attempted_models: Vec<String>,
    #[serde(default)]
    attempted_methods: Vec<String>,
    successful_model_path: Option<String>,
    successful_method: Option<String>,
    #[serde(default)]
    method_space_exhausted: bool,
    #[serde(default)]
    internal_timeout_observed: bool,
    #[serde(default)]
    notes: Vec<String>,
    created_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BuildReceiptView {
    starter_override_id: Option<String>,
    starter_override_summary: Option<String>,
    recommended_starter_id: Option<String>,
    recommended_starter_summary: Option<String>,
    starter_recommendation_comparison: Option<String>,
    project_name: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct FamilyGovernanceReceiptView {
    family_id: String,
    #[serde(default)]
    family_display_name: Option<String>,
    #[serde(default)]
    family_ecosystem: Option<String>,
    manifest_path: String,
    primary_substrate: String,
    #[serde(default)]
    lifecycle_status: String,
    #[serde(default)]
    lifecycle_notes: Vec<String>,
    supported_tool_kinds: Vec<String>,
    provided_build_primitive_classes: Vec<String>,
    primitive_adapter_ids: Vec<String>,
    drift_status: String,
    drift_notes: Vec<String>,
    change_since_last_live_status: String,
    change_since_last_live_notes: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TemplateGovernanceRefreshStatusView {
    status_id: String,
    refreshed_entries: usize,
    skipped_entries: usize,
    updated_at: String,
    #[serde(skip)]
    refreshed_at_label: Option<String>,
    #[serde(skip)]
    age_minutes: Option<u64>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ProjectPatchReadinessRefreshStatusView {
    status_id: String,
    refreshed_entries: usize,
    skipped_entries: usize,
    updated_at: String,
    #[serde(skip)]
    refreshed_at_label: Option<String>,
    #[serde(skip)]
    age_minutes: Option<u64>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TemplateGovernanceReceiptView {
    template_bundle_id: String,
    template_category: String,
    template_root: String,
    artifact_paths: Vec<String>,
    drift_status: String,
    drift_notes: Vec<String>,
    change_since_last_live_status: String,
    change_since_last_live_notes: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ProjectHistoricalBlockerBundleView {
    replacement_patch_kinds: Vec<String>,
    ready_replacement_patch_kinds: Vec<String>,
    already_present_replacement_patch_kinds: Vec<String>,
    bundle_status: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ProjectPatchReadinessReceiptView {
    #[serde(default)]
    family_display_name: Option<String>,
    #[serde(default)]
    family_ecosystem: Option<String>,
    blocked_lane_reasons: BTreeMap<String, String>,
    superseded_blocked_lane_replacements: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    decomposable_historical_blocker_bundles: BTreeMap<String, ProjectHistoricalBlockerBundleView>,
    risky_blocked_lane_count: usize,
    historical_blocked_lane_count: usize,
    #[serde(default)]
    decomposable_historical_blocker_count: usize,
    change_since_patchability_baseline_status: String,
    change_since_patchability_baseline_notes: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BuildVerificationReceiptView {
    review_subject: String,
    failure_class: String,
    failure_mode: String,
    suggested_family_id: Option<String>,
    suggested_tool_kind: Option<String>,
    #[serde(default)]
    matched_approved_constraint_ids: Vec<String>,
    #[serde(default)]
    matched_approved_constraint_summaries: Vec<String>,
    #[serde(default)]
    reasons: Vec<String>,
    #[serde(default)]
    blocked_methods: Vec<String>,
    #[serde(default)]
    findings: Vec<String>,
    decision: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ProposedConstraintReceiptView {
    #[serde(default)]
    proposal_id: Option<String>,
    #[serde(default)]
    source_verification_id: Option<String>,
    status: String,
    rationale: Vec<String>,
    proposed_constraint: ProposedConstraintView,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ProposedConstraintView {
    constraint_scope: String,
    constraint_kind: String,
    forbidden_method_summary: String,
    replacement_guidance: Option<String>,
    severity: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ApprovedConstraintShelfView {
    shelf_id: String,
    #[serde(default)]
    constraints: Vec<ApprovedConstraintView>,
    updated_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ConstraintShelfHistoryEntryView {
    constraint: ApprovedConstraintView,
    archived_reason: String,
    archived_from_shelf_id: Option<String>,
    #[serde(default)]
    archived_match_count: usize,
    archived_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ConstraintShelfHistoryView {
    history_id: String,
    #[serde(default)]
    archived_constraints: Vec<ConstraintShelfHistoryEntryView>,
    updated_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ConstraintApprovalReceiptView {
    approved_constraint_id: String,
    #[serde(default)]
    proposal_origin: String,
    #[serde(default)]
    proposal_source_id: String,
    created_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ConstraintShelfMutationReceiptView {
    mutation_id: String,
    constraint_id: String,
    action: String,
    #[serde(default)]
    proposal_origin: Option<String>,
    #[serde(default)]
    proposal_source_id: Option<String>,
    status: String,
    created_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct AtomizationFloorDecisionView {
    task_id: String,
    task_shape: Option<String>,
    task_subtype: Option<String>,
    current_granularity: String,
    decision: String,
    #[serde(default)]
    alternate_methods: Vec<String>,
    #[serde(default)]
    findings: Vec<String>,
    created_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct FailureVaultEntryView {
    task_id: String,
    task_shape: Option<String>,
    task_subtype: Option<String>,
    task_kind: String,
    failure_class: String,
    trigger_class: Option<String>,
    triangulation_session_id: String,
    atomization_floor_decision_path: Option<String>,
    source_attempt_receipt_path: String,
    source_decomposition_receipt_path: Option<String>,
    status: String,
    decomposition_depth: usize,
    #[serde(default)]
    findings: Vec<String>,
    created_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TriangulationAttemptView {
    attempt_id: String,
    task_id: String,
    task_subtype: Option<String>,
    attempt_method: String,
    outcome: String,
    failure_class: Option<String>,
    source_attempt_receipt_path: Option<String>,
    source_decomposition_receipt_path: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TriangulationSessionView {
    session_id: String,
    task_shape: Option<String>,
    task_subtype: Option<String>,
    task_lineage_key: String,
    status: String,
    convergence_posture: String,
    atomization_floor_decision_path: Option<String>,
    successful_alternate_method: bool,
    #[serde(default)]
    attempts: Vec<TriangulationAttemptView>,
    #[serde(default)]
    findings: Vec<String>,
    created_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ConstraintPromotionCandidateView {
    candidate_id: String,
    triangulation_session_id: String,
    project_name: String,
    task_shape: Option<String>,
    task_subtype: Option<String>,
    failure_class: String,
    trigger_class: Option<String>,
    confidence_posture: String,
    status: String,
    #[serde(default)]
    matched_constraint_principles: Vec<String>,
    #[serde(default)]
    narrow_usage_pattern: Vec<String>,
    #[serde(default)]
    evidence_receipt_paths: Vec<String>,
    #[serde(default)]
    findings: Vec<String>,
    recommended_constraint_summary: String,
    created_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ApprovedConstraintView {
    constraint_id: String,
    constraint_scope: String,
    constraint_kind: String,
    forbidden_method_summary: String,
    replacement_guidance: Option<String>,
    severity: String,
    active: bool,
    family_id: Option<String>,
    tool_kind: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum ToastKind {
    Success,
    Error,
}

#[derive(Debug, Clone)]
struct UiToast {
    message: String,
    kind: ToastKind,
    created_at: Instant,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ExtensionActivityItem {
    entry_id: Option<String>,
    title: String,
    detail: String,
    success: bool,
    timestamp_label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExtensionRegistryScope {
    All,
    Shipped,
    Active,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExtensionRegistrySort {
    RecentFirst,
    StatusFirst,
    FamilyToolPatch,
    ProofRiskFirst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProofQualityFilter {
    All,
    Passing,
    RunnableDiverged,
    CatalogResolved,
    NeedsContractFix,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProofBaselineFilter {
    All,
    Stable,
    Changed,
    Regressed,
    NoBaseline,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompositionBaselineFilter {
    All,
    Stable,
    Changed,
    Regressed,
    NoBaseline,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatchBaselineFilter {
    All,
    Stable,
    Changed,
    Regressed,
    NoBaseline,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HelperBaselineFilter {
    All,
    Stable,
    Changed,
    Regressed,
    NoBaseline,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BridgeBaselineFilter {
    All,
    Stable,
    Changed,
    Regressed,
    NoBaseline,
    Unknown,
}

#[derive(Debug, Clone, Default)]
struct ProjectPatchReadinessSummary {
    ready_count: usize,
    already_present_count: usize,
    dependency_blocked_count: usize,
    structurally_blocked_count: usize,
    surface_mismatch_count: usize,
    unknown_count: usize,
    first_blocked_reason: Option<String>,
}

fn summarize_project_patch_readiness(spec: &ProjectSpec) -> ProjectPatchReadinessSummary {
    let mut summary = ProjectPatchReadinessSummary::default();
    for lane in &spec.patch_lanes {
        match lane.effective_preflight_readiness.as_str() {
            "ready" => summary.ready_count += 1,
            "already_present" => summary.already_present_count += 1,
            "dependency_blocked" => {
                summary.dependency_blocked_count += 1;
                if summary.first_blocked_reason.is_none() {
                    let reason = lane.preflight_readiness_reason.trim();
                    if !reason.is_empty() {
                        summary.first_blocked_reason = Some(reason.to_string());
                    }
                }
            }
            "structurally_blocked" => {
                summary.structurally_blocked_count += 1;
                if summary.first_blocked_reason.is_none() {
                    let reason = lane.preflight_readiness_reason.trim();
                    if !reason.is_empty() {
                        summary.first_blocked_reason = Some(reason.to_string());
                    }
                }
            }
            "surface_mismatch" => {
                summary.surface_mismatch_count += 1;
                if summary.first_blocked_reason.is_none() {
                    let reason = lane.preflight_readiness_reason.trim();
                    if !reason.is_empty() {
                        summary.first_blocked_reason = Some(reason.to_string());
                    }
                }
            }
            _ => summary.unknown_count += 1,
        }
    }
    summary
}

fn patch_lane_superseded_summary(lane: &PatchLaneStatus) -> Option<String> {
    if lane.superseded_by_patch_kinds.is_empty() {
        None
    } else {
        Some(lane.superseded_by_patch_kinds.join(", "))
    }
}

fn project_patchability_badge(
    receipt: Option<&ProjectPatchReadinessReceiptView>,
    readiness: &ProjectPatchReadinessSummary,
) -> (&'static str, egui::Color32) {
    if let Some(receipt) = receipt {
        match receipt.change_since_patchability_baseline_status.as_str() {
            "regressed_since_patchability_baseline" => {
                return ("patch-regressed", egui::Color32::from_rgb(190, 74, 74));
            }
            "changed_since_patchability_baseline" => {
                return ("patch-drifted", egui::Color32::from_rgb(214, 170, 72));
            }
            "improved_since_patchability_baseline" => {
                return ("patch-improved", egui::Color32::from_rgb(70, 156, 140));
            }
            "stable_since_patchability_baseline" => {
                return ("patch-stable", egui::Color32::from_rgb(84, 168, 108));
            }
            "baseline_recorded" => {
                return if receipt.risky_blocked_lane_count == 0
                    && receipt.decomposable_historical_blocker_count > 0
                {
                    ("patch-decomposable", egui::Color32::from_rgb(114, 160, 96))
                } else if receipt.risky_blocked_lane_count == 0
                    && receipt.historical_blocked_lane_count > 0
                {
                    ("patch-historical", egui::Color32::from_rgb(184, 156, 92))
                } else if readiness.structurally_blocked_count
                    + readiness.surface_mismatch_count
                    + readiness.dependency_blocked_count
                    == 0
                {
                    ("patch-ready", egui::Color32::from_rgb(84, 168, 108))
                } else {
                    ("patch-baselined", egui::Color32::from_rgb(196, 150, 74))
                };
            }
            _ => {}
        }
    }

    if let Some(receipt) = receipt {
        if receipt.risky_blocked_lane_count == 0
            && receipt.decomposable_historical_blocker_count > 0
        {
            return ("patch-decomposable", egui::Color32::from_rgb(114, 160, 96));
        }
        if receipt.risky_blocked_lane_count == 0 && receipt.historical_blocked_lane_count > 0 {
            return ("patch-historical", egui::Color32::from_rgb(184, 156, 92));
        }
    }

    if readiness.structurally_blocked_count + readiness.surface_mismatch_count > 0 {
        ("patch-risk", egui::Color32::from_rgb(196, 102, 72))
    } else if readiness.dependency_blocked_count > 0 {
        ("patch-waiting", egui::Color32::from_rgb(214, 170, 72))
    } else {
        ("patch-ready", egui::Color32::from_rgb(84, 168, 108))
    }
}

fn project_historical_blocker_badge(
    receipt: Option<&ProjectPatchReadinessReceiptView>,
) -> Option<egui::RichText> {
    let has_historical_blockers = receipt
        .map(|receipt| !receipt.superseded_blocked_lane_replacements.is_empty())
        .unwrap_or(false);
    if has_historical_blockers {
        Some(
            egui::RichText::new("[historical-blocker]")
                .small()
                .color(egui::Color32::from_rgb(156, 132, 72))
                .strong(),
        )
    } else {
        None
    }
}

fn project_decomposable_historical_badge(
    receipt: Option<&ProjectPatchReadinessReceiptView>,
) -> Option<egui::RichText> {
    let has_decomposable_historical = receipt
        .map(|receipt| receipt.decomposable_historical_blocker_count > 0)
        .unwrap_or(false);
    if has_decomposable_historical {
        Some(
            egui::RichText::new("[decomposable-historical]")
                .small()
                .color(egui::Color32::from_rgb(114, 160, 96))
                .strong(),
        )
    } else {
        None
    }
}

fn project_patchability_risk_rank(
    receipt: Option<&ProjectPatchReadinessReceiptView>,
    readiness: &ProjectPatchReadinessSummary,
) -> usize {
    if let Some(receipt) = receipt {
        match receipt.change_since_patchability_baseline_status.as_str() {
            "regressed_since_patchability_baseline" => return 5,
            "changed_since_patchability_baseline" => return 4,
            "baseline_recorded" => {
                if receipt.risky_blocked_lane_count > 0 {
                    return 3;
                }
                if receipt.decomposable_historical_blocker_count > 0 {
                    return 2;
                }
                if receipt.historical_blocked_lane_count > 0 {
                    return 1;
                }
                return 0;
            }
            "stable_since_patchability_baseline" => return 1,
            "improved_since_patchability_baseline" => return 0,
            _ => {}
        }
    }
    if let Some(receipt) = receipt {
        if receipt.risky_blocked_lane_count == 0 && receipt.historical_blocked_lane_count > 0 {
            if receipt.decomposable_historical_blocker_count > 0 {
                return 2;
            }
            return 1;
        }
    }
    if readiness.structurally_blocked_count + readiness.surface_mismatch_count > 0 {
        4
    } else if readiness.dependency_blocked_count > 0 {
        2
    } else {
        1
    }
}

#[derive(Debug, Clone, Default)]
struct UiExecutionResult {
    kind: String,
    request_id: String,
    project_name: String,
    starter_override_id: Option<String>,
    starter_override_summary: Option<String>,
    recommended_starter_id: Option<String>,
    recommended_starter_summary: Option<String>,
    starter_recommendation_comparison: Option<String>,
    family_id: Option<String>,
    tool_kind: Option<String>,
    patch_kind: Option<String>,
    followup_request_mode: Option<String>,
    followup_rationale: Vec<String>,
    plan_confidence_score: u8,
    plan_confidence_band: String,
    needs_llm_review: bool,
    acceptance_status: Option<String>,
    route_notes: Vec<String>,
    file_paths: Vec<String>,
    patch_lanes: Vec<PatchLaneStatus>,
    acceptance_recipes: Vec<AcceptanceRecipeStatus>,
    operator_bundles: Vec<OperatorBundleStatus>,
    chattycog_hosting_mode: Option<String>,
    chattycog_ui_owner: Option<String>,
    chattycog_bridge_capabilities: Option<ChattyCogBridgeCapabilities>,
    patch_diagnosis_path: Option<String>,
    patch_plan_review_path: Option<String>,
    patch_constraint_review_path: Option<String>,
    patch_intent_freeze_path: Option<String>,
    patch_postcheck_path: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct CrossFamilyPairedProofReceiptSummary {
    receipt_id: String,
    #[serde(default)]
    proof_template_id: Option<String>,
    #[serde(default)]
    proof_template_kind: Option<String>,
    #[serde(default)]
    proof_template_display_label: Option<String>,
    #[serde(default)]
    capability_comparison_bundle_id: Option<String>,
    shared_request: String,
    left_request: String,
    right_request: String,
    left_project_name: String,
    left_composable_route_plan_path: Option<String>,
    left_primitive_execution_plan_path: Option<String>,
    right_project_name: String,
    right_composable_route_plan_path: Option<String>,
    right_primitive_execution_plan_path: Option<String>,
    comparison_receipt_path: String,
    equivalent_capability_fulfillment: bool,
    notes: Vec<String>,
    created_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct CapabilityComparisonReceiptSummary {
    #[serde(default)]
    shared_capability_classes: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct PairedProofUiPreferences {
    selected_proof_template_id: String,
    proof_history_template_filter: String,
    pin_history_filter_to_selected_template: bool,
    #[serde(default)]
    project_browser_show_blocked_lanes_only: bool,
    #[serde(default)]
    project_browser_show_regressed_only: bool,
    #[serde(default)]
    project_browser_show_improved_only: bool,
    #[serde(default)]
    project_browser_show_historical_blockers_only: bool,
    #[serde(default)]
    project_browser_show_decomposable_historical_only: bool,
    #[serde(default = "default_true")]
    project_browser_sort_by_patch_risk: bool,
    #[serde(default)]
    negative_shelf_show_unmatched_only: bool,
    #[serde(default)]
    negative_shelf_show_inactive_unmatched_only: bool,
    #[serde(default)]
    negative_shelf_show_low_value_active_only: bool,
    #[serde(default)]
    negative_shelf_history_show_never_matched_only: bool,
    #[serde(default)]
    negative_shelf_history_show_historically_useful_only: bool,
    #[serde(default = "default_true")]
    auto_refresh_stale_proof_governance: bool,
    #[serde(default)]
    last_auto_proof_governance_refresh_unix_secs: Option<i64>,
    #[serde(default = "default_true")]
    auto_refresh_stale_composition_governance: bool,
    #[serde(default)]
    last_auto_composition_governance_refresh_unix_secs: Option<i64>,
    #[serde(default = "default_true")]
    auto_refresh_stale_patch_governance: bool,
    #[serde(default)]
    last_auto_patch_governance_refresh_unix_secs: Option<i64>,
    #[serde(default = "default_true")]
    auto_refresh_stale_helper_governance: bool,
    #[serde(default)]
    last_auto_helper_governance_refresh_unix_secs: Option<i64>,
    #[serde(default = "default_true")]
    auto_refresh_stale_bridge_governance: bool,
    #[serde(default)]
    last_auto_bridge_governance_refresh_unix_secs: Option<i64>,
    #[serde(default = "default_true")]
    auto_refresh_stale_family_governance: bool,
    #[serde(default)]
    last_auto_family_governance_refresh_unix_secs: Option<i64>,
    #[serde(default = "default_true")]
    auto_refresh_stale_template_governance: bool,
    #[serde(default)]
    last_auto_template_governance_refresh_unix_secs: Option<i64>,
    auto_planner: bool,
    planner_port: String,
    planner_model: String,
    #[serde(default = "default_build_starter_override_id")]
    build_starter_override_id: String,
    #[serde(default)]
    active_profile_name: Option<String>,
    #[serde(default)]
    profiles: Vec<ProofRunProfile>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ProofRunProfile {
    profile_name: String,
    selected_proof_template_id: String,
    proof_history_template_filter: String,
    pin_history_filter_to_selected_template: bool,
    auto_planner: bool,
    planner_port: String,
    planner_model: String,
}

fn default_true() -> bool {
    true
}

fn default_build_starter_override_id() -> String {
    "auto".to_string()
}

#[derive(Debug, Clone, Default)]
struct UiFallbackResult {
    request_id: String,
    mode: Option<String>,
    question: String,
    interpreted_goal: String,
    reasons: Vec<String>,
    candidate_family_ids: Vec<String>,
    requested_capabilities: Vec<String>,
    constraints: Vec<String>,
    suggested_extension_kind: String,
    suggested_family_id: Option<String>,
    suggested_tool_kind: Option<String>,
    suggested_patch_kind: Option<String>,
    suggested_bridge_capabilities: Vec<String>,
    suggested_hosting_mode: Option<String>,
    suggested_artifacts: Vec<String>,
    acceptance_targets: Vec<String>,
    implementation_notes: Vec<String>,
    recommended_next_step: String,
    pending_extension_ids: Vec<String>,
    pending_extension_scaffold_roots: Vec<String>,
    chattycog_requested_hosting_mode: Option<String>,
    chattycog_valid_hosting_modes: Vec<String>,
    chattycog_requested_bridge_capabilities: Vec<String>,
    chattycog_supported_bridge_capabilities: Vec<String>,
    stub_bundle_path: Option<String>,
    build_failure_class: Option<String>,
    build_failure_mode: Option<String>,
    matched_approved_constraint_ids: Vec<String>,
    matched_approved_constraint_summaries: Vec<String>,
    build_verification_path: Option<String>,
    proposed_constraint_path: Option<String>,
    proposed_constraint_summary: Option<String>,
    proposed_constraint_replacement_guidance: Option<String>,
}

struct ChattyFactoryUiApp {
    workspace_root: PathBuf,
    selected_project_name: Option<String>,
    browser_state: Option<ProjectBrowserState>,
    selected_project_spec: Option<ProjectSpec>,
    runtime_status: RuntimeStatusView,
    proof_governance_refresh_status: Option<ProofGovernanceRefreshStatusView>,
    composition_governance_refresh_status: Option<CompositionGovernanceRefreshStatusView>,
    patch_governance_refresh_status: Option<PatchGovernanceRefreshStatusView>,
    project_patch_readiness_refresh_status: Option<ProjectPatchReadinessRefreshStatusView>,
    helper_governance_refresh_status: Option<HelperGovernanceRefreshStatusView>,
    bridge_governance_refresh_status: Option<BridgeGovernanceRefreshStatusView>,
    family_governance_refresh_status: Option<FamilyGovernanceRefreshStatusView>,
    selected_family_governance_id: Option<String>,
    template_governance_refresh_status: Option<TemplateGovernanceRefreshStatusView>,
    selected_template_governance_id: Option<String>,
    extension_registry: Option<HostExtensionRegistryView>,
    selected_extension_entry_id: Option<String>,
    favorite_extension_entry_ids: BTreeSet<String>,
    favorite_paired_proof_receipt_ids: BTreeSet<String>,
    recent_extension_entry_ids: Vec<String>,
    extension_notes: BTreeMap<String, String>,
    paired_proof_notes: BTreeMap<String, String>,
    extension_registry_query: String,
    extension_registry_scope: ExtensionRegistryScope,
    extension_registry_sort: ExtensionRegistrySort,
    extension_proof_quality_filter: ProofQualityFilter,
    extension_proof_baseline_filter: ProofBaselineFilter,
    extension_composition_baseline_filter: CompositionBaselineFilter,
    extension_patch_baseline_filter: PatchBaselineFilter,
    extension_helper_baseline_filter: HelperBaselineFilter,
    extension_bridge_baseline_filter: BridgeBaselineFilter,
    patch_xray_blocked_only: bool,
    project_browser_show_blocked_lanes_only: bool,
    project_browser_show_regressed_only: bool,
    project_browser_show_improved_only: bool,
    project_browser_show_historical_blockers_only: bool,
    project_browser_show_decomposable_historical_only: bool,
    project_browser_sort_by_patch_risk: bool,
    negative_shelf_show_unmatched_only: bool,
    negative_shelf_show_inactive_unmatched_only: bool,
    negative_shelf_show_low_value_active_only: bool,
    negative_shelf_show_triangulated_origin_only: bool,
    negative_shelf_history_show_never_matched_only: bool,
    negative_shelf_history_show_historically_useful_only: bool,
    negative_shelf_history_show_triangulated_origin_only: bool,
    auto_refresh_stale_proof_governance: bool,
    last_auto_proof_governance_refresh_unix_secs: Option<i64>,
    auto_refresh_stale_composition_governance: bool,
    last_auto_composition_governance_refresh_unix_secs: Option<i64>,
    auto_refresh_stale_patch_governance: bool,
    last_auto_patch_governance_refresh_unix_secs: Option<i64>,
    auto_refresh_stale_helper_governance: bool,
    last_auto_helper_governance_refresh_unix_secs: Option<i64>,
    auto_refresh_stale_bridge_governance: bool,
    last_auto_bridge_governance_refresh_unix_secs: Option<i64>,
    auto_refresh_stale_family_governance: bool,
    last_auto_family_governance_refresh_unix_secs: Option<i64>,
    auto_refresh_stale_template_governance: bool,
    last_auto_template_governance_refresh_unix_secs: Option<i64>,
    request_input: String,
    build_starter_override_id: String,
    paired_proof_request_input: String,
    selected_proof_template_id: String,
    proof_history_template_filter: String,
    pin_history_filter_to_selected_template: bool,
    proof_run_profiles: Vec<ProofRunProfile>,
    selected_proof_profile_name: String,
    proof_profile_name_input: String,
    auto_planner: bool,
    planner_port: String,
    planner_model: String,
    command_log: String,
    last_action_summary: ActionSummary,
    last_execution_result: Option<UiExecutionResult>,
    last_fallback_result: Option<UiFallbackResult>,
    extension_activity: Vec<ExtensionActivityItem>,
    status_line: String,
    task_sender: Sender<UiTaskResult>,
    task_receiver: Receiver<UiTaskResult>,
    task_running: bool,
    toasts: Vec<UiToast>,
}

impl ChattyFactoryUiApp {
    fn new() -> Self {
        let workspace_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let (task_sender, task_receiver) = mpsc::channel();
        let paired_proof_ui_preferences = load_paired_proof_ui_preferences(&workspace_root);
        let mut app = Self {
            workspace_root: workspace_root.clone(),
            selected_project_name: None,
            browser_state: None,
            selected_project_spec: None,
            runtime_status: RuntimeStatusView::default(),
            proof_governance_refresh_status: load_proof_governance_refresh_status(&workspace_root),
            composition_governance_refresh_status: load_composition_governance_refresh_status(
                &workspace_root,
            ),
            patch_governance_refresh_status: load_patch_governance_refresh_status(&workspace_root),
            project_patch_readiness_refresh_status: load_project_patch_readiness_refresh_status(
                &workspace_root,
            ),
            helper_governance_refresh_status: load_helper_governance_refresh_status(
                &workspace_root,
            ),
            bridge_governance_refresh_status: load_bridge_governance_refresh_status(
                &workspace_root,
            ),
            family_governance_refresh_status: load_family_governance_refresh_status(
                &workspace_root,
            ),
            selected_family_governance_id: load_family_governance_receipts(&workspace_root)
                .first()
                .map(|receipt| receipt.family_id.clone()),
            template_governance_refresh_status: load_template_governance_refresh_status(
                &workspace_root,
            ),
            selected_template_governance_id: load_template_governance_receipts(&workspace_root)
                .first()
                .map(|receipt| receipt.template_bundle_id.clone()),
            extension_registry: load_extension_registry(&workspace_root),
            selected_extension_entry_id: None,
            favorite_extension_entry_ids: load_extension_favorites(&workspace_root),
            favorite_paired_proof_receipt_ids: load_paired_proof_favorites(&workspace_root),
            recent_extension_entry_ids: load_recent_extensions(&workspace_root),
            extension_notes: load_extension_notes(&workspace_root),
            paired_proof_notes: load_paired_proof_notes(&workspace_root),
            extension_activity: load_extension_activity(&workspace_root),
            extension_registry_query: String::new(),
            extension_registry_scope: ExtensionRegistryScope::All,
            extension_registry_sort: ExtensionRegistrySort::RecentFirst,
            extension_proof_quality_filter: ProofQualityFilter::All,
            extension_proof_baseline_filter: ProofBaselineFilter::All,
            extension_composition_baseline_filter: CompositionBaselineFilter::All,
            extension_patch_baseline_filter: PatchBaselineFilter::All,
            extension_helper_baseline_filter: HelperBaselineFilter::All,
            extension_bridge_baseline_filter: BridgeBaselineFilter::All,
            patch_xray_blocked_only: false,
            project_browser_show_blocked_lanes_only: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.project_browser_show_blocked_lanes_only)
                .unwrap_or(false),
            project_browser_show_regressed_only: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.project_browser_show_regressed_only)
                .unwrap_or(false),
            project_browser_show_improved_only: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.project_browser_show_improved_only)
                .unwrap_or(false),
            project_browser_show_historical_blockers_only: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.project_browser_show_historical_blockers_only)
                .unwrap_or(false),
            project_browser_show_decomposable_historical_only: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.project_browser_show_decomposable_historical_only)
                .unwrap_or(false),
            project_browser_sort_by_patch_risk: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.project_browser_sort_by_patch_risk)
                .unwrap_or(true),
            negative_shelf_show_unmatched_only: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.negative_shelf_show_unmatched_only)
                .unwrap_or(false),
            negative_shelf_show_inactive_unmatched_only: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.negative_shelf_show_inactive_unmatched_only)
                .unwrap_or(false),
            negative_shelf_show_low_value_active_only: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.negative_shelf_show_low_value_active_only)
                .unwrap_or(false),
            negative_shelf_show_triangulated_origin_only: false,
            negative_shelf_history_show_never_matched_only: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.negative_shelf_history_show_never_matched_only)
                .unwrap_or(false),
            negative_shelf_history_show_historically_useful_only: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.negative_shelf_history_show_historically_useful_only)
                .unwrap_or(false),
            negative_shelf_history_show_triangulated_origin_only: false,
            auto_refresh_stale_proof_governance: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.auto_refresh_stale_proof_governance)
                .unwrap_or(true),
            last_auto_proof_governance_refresh_unix_secs: paired_proof_ui_preferences
                .as_ref()
                .and_then(|prefs| prefs.last_auto_proof_governance_refresh_unix_secs),
            auto_refresh_stale_composition_governance: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.auto_refresh_stale_composition_governance)
                .unwrap_or(true),
            last_auto_composition_governance_refresh_unix_secs: paired_proof_ui_preferences
                .as_ref()
                .and_then(|prefs| prefs.last_auto_composition_governance_refresh_unix_secs),
            auto_refresh_stale_patch_governance: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.auto_refresh_stale_patch_governance)
                .unwrap_or(true),
            last_auto_patch_governance_refresh_unix_secs: paired_proof_ui_preferences
                .as_ref()
                .and_then(|prefs| prefs.last_auto_patch_governance_refresh_unix_secs),
            auto_refresh_stale_helper_governance: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.auto_refresh_stale_helper_governance)
                .unwrap_or(true),
            last_auto_helper_governance_refresh_unix_secs: paired_proof_ui_preferences
                .as_ref()
                .and_then(|prefs| prefs.last_auto_helper_governance_refresh_unix_secs),
            auto_refresh_stale_bridge_governance: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.auto_refresh_stale_bridge_governance)
                .unwrap_or(true),
            last_auto_bridge_governance_refresh_unix_secs: paired_proof_ui_preferences
                .as_ref()
                .and_then(|prefs| prefs.last_auto_bridge_governance_refresh_unix_secs),
            auto_refresh_stale_family_governance: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.auto_refresh_stale_family_governance)
                .unwrap_or(true),
            last_auto_family_governance_refresh_unix_secs: paired_proof_ui_preferences
                .as_ref()
                .and_then(|prefs| prefs.last_auto_family_governance_refresh_unix_secs),
            auto_refresh_stale_template_governance: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.auto_refresh_stale_template_governance)
                .unwrap_or(true),
            last_auto_template_governance_refresh_unix_secs: paired_proof_ui_preferences
                .as_ref()
                .and_then(|prefs| prefs.last_auto_template_governance_refresh_unix_secs),
            request_input: String::new(),
            build_starter_override_id: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.build_starter_override_id.clone())
                .unwrap_or_else(default_build_starter_override_id),
            paired_proof_request_input: String::new(),
            selected_proof_template_id: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.selected_proof_template_id.clone())
                .unwrap_or_else(|| "proof_helper_monitoring".to_string()),
            proof_history_template_filter: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.proof_history_template_filter.clone())
                .unwrap_or_else(|| "all".to_string()),
            pin_history_filter_to_selected_template: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.pin_history_filter_to_selected_template)
                .unwrap_or(true),
            proof_run_profiles: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.profiles.clone())
                .unwrap_or_default(),
            selected_proof_profile_name: paired_proof_ui_preferences
                .as_ref()
                .and_then(|prefs| prefs.active_profile_name.clone())
                .unwrap_or_else(|| "custom".to_string()),
            proof_profile_name_input: paired_proof_ui_preferences
                .as_ref()
                .and_then(|prefs| prefs.active_profile_name.clone())
                .unwrap_or_default(),
            auto_planner: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.auto_planner)
                .unwrap_or(true),
            planner_port: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.planner_port.clone())
                .unwrap_or_else(|| "8104".to_string()),
            planner_model: paired_proof_ui_preferences
                .as_ref()
                .map(|prefs| prefs.planner_model.clone())
                .unwrap_or_else(|| "fast".to_string()),
            command_log: String::new(),
            last_action_summary: ActionSummary::default(),
            last_execution_result: None,
            last_fallback_result: None,
            status_line: "Ready".to_string(),
            task_sender,
            task_receiver,
            task_running: false,
            toasts: Vec::new(),
        };
        if proof_governance_should_auto_refresh(
            &workspace_root,
            app.proof_governance_refresh_status.as_ref(),
            app.auto_refresh_stale_proof_governance,
            app.last_auto_proof_governance_refresh_unix_secs,
        ) {
            app.status_line =
                "Proof governance is stale and newer proof receipts exist. Auto-refreshing."
                    .to_string();
            app.last_auto_proof_governance_refresh_unix_secs = Some(Utc::now().timestamp());
            app.save_paired_proof_ui_preferences();
            app.spawn_task(UiTask::RefreshProofHarnessRegistry);
        }
        if composition_governance_should_auto_refresh(
            app.composition_governance_refresh_status.as_ref(),
            app.auto_refresh_stale_composition_governance,
            app.last_auto_composition_governance_refresh_unix_secs,
        ) {
            app.status_line = "Composition governance is stale. Auto-refreshing.".to_string();
            app.last_auto_composition_governance_refresh_unix_secs = Some(Utc::now().timestamp());
            app.save_paired_proof_ui_preferences();
            app.spawn_task(UiTask::RefreshCompositionGovernance);
        }
        if patch_governance_should_auto_refresh(
            app.patch_governance_refresh_status.as_ref(),
            app.auto_refresh_stale_patch_governance,
            app.last_auto_patch_governance_refresh_unix_secs,
        ) {
            app.status_line = "Patch governance is stale. Auto-refreshing.".to_string();
            app.last_auto_patch_governance_refresh_unix_secs = Some(Utc::now().timestamp());
            app.save_paired_proof_ui_preferences();
            app.spawn_task(UiTask::RefreshPatchGovernance);
        }
        if helper_governance_should_auto_refresh(
            app.helper_governance_refresh_status.as_ref(),
            app.auto_refresh_stale_helper_governance,
            app.last_auto_helper_governance_refresh_unix_secs,
        ) {
            app.status_line = "Helper governance is stale. Auto-refreshing.".to_string();
            app.last_auto_helper_governance_refresh_unix_secs = Some(Utc::now().timestamp());
            app.save_paired_proof_ui_preferences();
            app.spawn_task(UiTask::RefreshHelperGovernance);
        }
        if bridge_governance_should_auto_refresh(
            app.bridge_governance_refresh_status.as_ref(),
            app.auto_refresh_stale_bridge_governance,
            app.last_auto_bridge_governance_refresh_unix_secs,
        ) {
            app.status_line = "Bridge governance is stale. Auto-refreshing.".to_string();
            app.last_auto_bridge_governance_refresh_unix_secs = Some(Utc::now().timestamp());
            app.save_paired_proof_ui_preferences();
            app.spawn_task(UiTask::RefreshBridgeGovernance);
        }
        if family_governance_should_auto_refresh(
            app.family_governance_refresh_status.as_ref(),
            app.auto_refresh_stale_family_governance,
            app.last_auto_family_governance_refresh_unix_secs,
        ) {
            app.status_line = "Family governance is stale. Auto-refreshing.".to_string();
            app.last_auto_family_governance_refresh_unix_secs = Some(Utc::now().timestamp());
            app.save_paired_proof_ui_preferences();
            app.spawn_task(UiTask::RefreshFamilyGovernance);
        }
        if template_governance_should_auto_refresh(
            app.template_governance_refresh_status.as_ref(),
            app.auto_refresh_stale_template_governance,
            app.last_auto_template_governance_refresh_unix_secs,
        ) {
            app.status_line = "Template governance is stale. Auto-refreshing.".to_string();
            app.last_auto_template_governance_refresh_unix_secs = Some(Utc::now().timestamp());
            app.save_paired_proof_ui_preferences();
            app.spawn_task(UiTask::RefreshTemplateGovernance);
        }
        app.refresh_from_runtime_file();
        app.refresh_runtime_status();
        app
    }

    fn runtime_state_path(&self) -> PathBuf {
        self.workspace_root
            .join("runtime")
            .join("project_browser_state.json")
    }

    fn extension_favorites_path(&self) -> PathBuf {
        self.workspace_root
            .join("runtime")
            .join("extension_favorites.json")
    }

    fn extension_recent_path(&self) -> PathBuf {
        self.workspace_root
            .join("runtime")
            .join("extension_recent.json")
    }

    fn paired_proof_favorites_path(&self) -> PathBuf {
        self.workspace_root
            .join("runtime")
            .join("paired_proof_favorites.json")
    }

    fn extension_notes_path(&self) -> PathBuf {
        self.workspace_root
            .join("runtime")
            .join("extension_notes.json")
    }

    fn paired_proof_notes_path(&self) -> PathBuf {
        self.workspace_root
            .join("runtime")
            .join("paired_proof_notes.json")
    }

    fn paired_proof_ui_preferences_path(&self) -> PathBuf {
        self.workspace_root
            .join("runtime")
            .join("paired_proof_ui_preferences.json")
    }

    fn extension_activity_path(&self) -> PathBuf {
        self.workspace_root
            .join("runtime")
            .join("extension_activity.json")
    }

    fn extension_exports_dir(&self) -> PathBuf {
        self.workspace_root
            .join("runtime")
            .join("extension_exports")
    }

    fn paired_proof_exports_dir(&self) -> PathBuf {
        self.workspace_root
            .join("runtime")
            .join("paired_proof_exports")
    }

    fn refresh_from_runtime_file(&mut self) {
        let path = self.runtime_state_path();
        if let Ok(contents) = std::fs::read_to_string(path) {
            if let Ok(state) = serde_json::from_str::<ProjectBrowserState>(&contents) {
                self.sync_browser_state(state);
            }
        }
    }

    fn sync_browser_state(&mut self, state: ProjectBrowserState) {
        if let Some(selected) = &state.selected_project_session {
            self.selected_project_name = Some(selected.project_name.clone());
        } else if let Some(active) = &state.active_project_session {
            self.selected_project_name = Some(active.project_name.clone());
        } else if let Some(first) = state.projects.first() {
            self.selected_project_name = Some(first.project_name.clone());
        } else {
            self.selected_project_name = None;
        }
        self.browser_state = Some(state);
        self.refresh_selected_project_spec();
    }

    fn refresh_selected_project_spec(&mut self) {
        self.selected_project_spec = self
            .selected_project()
            .and_then(|project_name| load_project_spec(&self.workspace_root, project_name));
    }

    fn refresh_runtime_status(&mut self) {
        self.runtime_status = load_runtime_status(&self.workspace_root);
    }

    fn save_extension_favorites(&mut self) {
        if let Err(error) = save_extension_favorites(
            &self.extension_favorites_path(),
            &self.favorite_extension_entry_ids,
        ) {
            self.status_line = format!("Failed to save favorites: {error}");
            self.push_toast(
                format!("Failed to save favorites: {error}"),
                ToastKind::Error,
            );
        }
    }

    fn toggle_extension_favorite(&mut self, entry_id: &str) {
        let was_removed = self.favorite_extension_entry_ids.remove(entry_id);
        if !was_removed {
            self.favorite_extension_entry_ids
                .insert(entry_id.to_string());
        }
        self.save_extension_favorites();
        let message = if was_removed {
            "Removed lane favorite"
        } else {
            "Pinned lane favorite"
        };
        self.status_line = message.to_string();
        self.push_extension_activity(
            Some(entry_id.to_string()),
            message,
            entry_id.to_string(),
            true,
        );
        self.push_toast(message, ToastKind::Success);
    }

    fn save_paired_proof_favorites(&mut self) {
        if let Err(error) = save_extension_favorites(
            &self.paired_proof_favorites_path(),
            &self.favorite_paired_proof_receipt_ids,
        ) {
            self.status_line = format!("Failed to save proof favorites: {error}");
            self.push_toast(
                format!("Failed to save proof favorites: {error}"),
                ToastKind::Error,
            );
        }
    }

    fn toggle_paired_proof_favorite(&mut self, receipt_id: &str) {
        let was_removed = self.favorite_paired_proof_receipt_ids.remove(receipt_id);
        if !was_removed {
            self.favorite_paired_proof_receipt_ids
                .insert(receipt_id.to_string());
        }
        self.save_paired_proof_favorites();
        let message = if was_removed {
            "Removed proof favorite"
        } else {
            "Pinned proof favorite"
        };
        self.status_line = message.to_string();
        self.push_extension_activity(
            Some(receipt_id.to_string()),
            message,
            receipt_id.to_string(),
            true,
        );
        self.push_toast(message, ToastKind::Success);
    }

    fn save_recent_extensions(&mut self) {
        if let Err(error) = save_recent_extensions(
            &self.extension_recent_path(),
            &self.recent_extension_entry_ids,
        ) {
            self.status_line = format!("Failed to save recent lanes: {error}");
            self.push_toast(
                format!("Failed to save recent lanes: {error}"),
                ToastKind::Error,
            );
        }
    }

    fn save_paired_proof_notes(&mut self) {
        if let Err(error) =
            save_extension_notes(&self.paired_proof_notes_path(), &self.paired_proof_notes)
        {
            self.status_line = format!("Failed to save proof notes: {error}");
            self.push_toast(
                format!("Failed to save proof notes: {error}"),
                ToastKind::Error,
            );
        }
    }

    fn save_paired_proof_ui_preferences(&mut self) {
        let preferences = PairedProofUiPreferences {
            selected_proof_template_id: self.selected_proof_template_id.clone(),
            proof_history_template_filter: self.proof_history_template_filter.clone(),
            pin_history_filter_to_selected_template: self.pin_history_filter_to_selected_template,
            project_browser_show_blocked_lanes_only: self.project_browser_show_blocked_lanes_only,
            project_browser_show_regressed_only: self.project_browser_show_regressed_only,
            project_browser_show_improved_only: self.project_browser_show_improved_only,
            project_browser_show_historical_blockers_only: self
                .project_browser_show_historical_blockers_only,
            project_browser_show_decomposable_historical_only: self
                .project_browser_show_decomposable_historical_only,
            project_browser_sort_by_patch_risk: self.project_browser_sort_by_patch_risk,
            negative_shelf_show_unmatched_only: self.negative_shelf_show_unmatched_only,
            negative_shelf_show_inactive_unmatched_only: self
                .negative_shelf_show_inactive_unmatched_only,
            negative_shelf_show_low_value_active_only: self
                .negative_shelf_show_low_value_active_only,
            negative_shelf_history_show_never_matched_only: self
                .negative_shelf_history_show_never_matched_only,
            negative_shelf_history_show_historically_useful_only: self
                .negative_shelf_history_show_historically_useful_only,
            auto_refresh_stale_proof_governance: self.auto_refresh_stale_proof_governance,
            last_auto_proof_governance_refresh_unix_secs: self
                .last_auto_proof_governance_refresh_unix_secs,
            auto_refresh_stale_composition_governance: self
                .auto_refresh_stale_composition_governance,
            last_auto_composition_governance_refresh_unix_secs: self
                .last_auto_composition_governance_refresh_unix_secs,
            auto_refresh_stale_patch_governance: self.auto_refresh_stale_patch_governance,
            last_auto_patch_governance_refresh_unix_secs: self
                .last_auto_patch_governance_refresh_unix_secs,
            auto_refresh_stale_helper_governance: self.auto_refresh_stale_helper_governance,
            last_auto_helper_governance_refresh_unix_secs: self
                .last_auto_helper_governance_refresh_unix_secs,
            auto_refresh_stale_bridge_governance: self.auto_refresh_stale_bridge_governance,
            last_auto_bridge_governance_refresh_unix_secs: self
                .last_auto_bridge_governance_refresh_unix_secs,
            auto_refresh_stale_family_governance: self.auto_refresh_stale_family_governance,
            last_auto_family_governance_refresh_unix_secs: self
                .last_auto_family_governance_refresh_unix_secs,
            auto_refresh_stale_template_governance: self.auto_refresh_stale_template_governance,
            last_auto_template_governance_refresh_unix_secs: self
                .last_auto_template_governance_refresh_unix_secs,
            auto_planner: self.auto_planner,
            planner_port: self.planner_port.clone(),
            planner_model: self.planner_model.clone(),
            build_starter_override_id: self.build_starter_override_id.clone(),
            active_profile_name: if self.selected_proof_profile_name == "custom" {
                None
            } else {
                Some(self.selected_proof_profile_name.clone())
            },
            profiles: self.proof_run_profiles.clone(),
        };
        if let Err(error) =
            save_paired_proof_ui_preferences(&self.paired_proof_ui_preferences_path(), &preferences)
        {
            self.status_line = format!("Failed to save proof preferences: {error}");
            self.push_toast(
                format!("Failed to save proof preferences: {error}"),
                ToastKind::Error,
            );
        }
    }

    fn current_proof_run_profile(&self, profile_name: String) -> ProofRunProfile {
        ProofRunProfile {
            profile_name,
            selected_proof_template_id: self.selected_proof_template_id.clone(),
            proof_history_template_filter: self.proof_history_template_filter.clone(),
            pin_history_filter_to_selected_template: self.pin_history_filter_to_selected_template,
            auto_planner: self.auto_planner,
            planner_port: self.planner_port.clone(),
            planner_model: self.planner_model.clone(),
        }
    }

    fn apply_proof_run_profile(&mut self, profile: &ProofRunProfile) {
        self.selected_proof_template_id = profile.selected_proof_template_id.clone();
        self.proof_history_template_filter = profile.proof_history_template_filter.clone();
        self.pin_history_filter_to_selected_template =
            profile.pin_history_filter_to_selected_template;
        self.auto_planner = profile.auto_planner;
        self.planner_port = profile.planner_port.clone();
        self.planner_model = profile.planner_model.clone();
        self.selected_proof_profile_name = profile.profile_name.clone();
        self.proof_profile_name_input = profile.profile_name.clone();
    }

    fn save_current_proof_run_profile(&mut self, profile_name: &str) {
        let trimmed = profile_name.trim();
        if trimmed.is_empty() {
            self.status_line = "Proof profile name is required".to_string();
            self.push_toast("Proof profile name is required", ToastKind::Error);
            return;
        }
        let profile = self.current_proof_run_profile(trimmed.to_string());
        if let Some(existing) = self
            .proof_run_profiles
            .iter_mut()
            .find(|item| item.profile_name == trimmed)
        {
            *existing = profile;
        } else {
            self.proof_run_profiles.push(profile);
            self.proof_run_profiles
                .sort_by(|left, right| left.profile_name.cmp(&right.profile_name));
        }
        self.selected_proof_profile_name = trimmed.to_string();
        self.proof_profile_name_input = trimmed.to_string();
        self.save_paired_proof_ui_preferences();
        self.status_line = "Saved proof profile".to_string();
        self.push_toast("Saved proof profile", ToastKind::Success);
    }

    fn delete_selected_proof_run_profile(&mut self) {
        if self.selected_proof_profile_name == "custom" {
            self.status_line = "No saved proof profile selected".to_string();
            self.push_toast("No saved proof profile selected", ToastKind::Error);
            return;
        }
        let selected = self.selected_proof_profile_name.clone();
        self.proof_run_profiles
            .retain(|profile| profile.profile_name != selected);
        self.selected_proof_profile_name = "custom".to_string();
        self.proof_profile_name_input.clear();
        self.save_paired_proof_ui_preferences();
        self.status_line = "Deleted proof profile".to_string();
        self.push_toast("Deleted proof profile", ToastKind::Success);
    }

    fn duplicate_selected_proof_run_profile(&mut self, profile_name: &str) {
        let trimmed = profile_name.trim();
        if trimmed.is_empty() {
            self.status_line = "Proof profile name is required".to_string();
            self.push_toast("Proof profile name is required", ToastKind::Error);
            return;
        }
        if self
            .proof_run_profiles
            .iter()
            .any(|profile| profile.profile_name == trimmed)
        {
            self.status_line = "Proof profile name already exists".to_string();
            self.push_toast("Proof profile name already exists", ToastKind::Error);
            return;
        }
        let profile = self.current_proof_run_profile(trimmed.to_string());
        self.proof_run_profiles.push(profile);
        self.proof_run_profiles
            .sort_by(|left, right| left.profile_name.cmp(&right.profile_name));
        self.selected_proof_profile_name = trimmed.to_string();
        self.proof_profile_name_input = trimmed.to_string();
        self.save_paired_proof_ui_preferences();
        self.status_line = "Duplicated proof profile".to_string();
        self.push_toast("Duplicated proof profile", ToastKind::Success);
    }

    fn reset_proof_run_defaults(&mut self) {
        self.selected_proof_template_id = "proof_helper_monitoring".to_string();
        self.proof_history_template_filter = "all".to_string();
        self.pin_history_filter_to_selected_template = true;
        self.auto_planner = true;
        self.planner_port = "8104".to_string();
        self.planner_model = "fast".to_string();
        self.selected_proof_profile_name = "custom".to_string();
        self.proof_profile_name_input.clear();
        self.save_paired_proof_ui_preferences();
        self.status_line = "Reset proof panel defaults".to_string();
        self.push_toast("Reset proof panel defaults", ToastKind::Success);
    }

    fn mark_extension_recent(&mut self, entry_id: &str) {
        self.recent_extension_entry_ids.retain(|id| id != entry_id);
        self.recent_extension_entry_ids
            .insert(0, entry_id.to_string());
        if self.recent_extension_entry_ids.len() > 10 {
            self.recent_extension_entry_ids.truncate(10);
        }
        self.save_recent_extensions();
    }

    fn save_extension_notes(&mut self) {
        if let Err(error) =
            save_extension_notes(&self.extension_notes_path(), &self.extension_notes)
        {
            self.status_line = format!("Failed to save lane notes: {error}");
            self.push_toast(
                format!("Failed to save lane notes: {error}"),
                ToastKind::Error,
            );
        }
    }

    fn save_extension_activity(&mut self) {
        if let Err(error) =
            save_extension_activity(&self.extension_activity_path(), &self.extension_activity)
        {
            self.status_line = format!("Failed to save lane activity: {error}");
            self.push_toast(
                format!("Failed to save lane activity: {error}"),
                ToastKind::Error,
            );
        }
    }

    fn clear_lane_activity(&mut self, entry_id: &str) {
        self.extension_activity
            .retain(|item| item.entry_id.as_deref() != Some(entry_id));
        self.save_extension_activity();
        self.status_line = "Cleared lane timeline".to_string();
        self.push_toast("Cleared lane timeline", ToastKind::Success);
    }

    fn clear_proof_activity(&mut self, receipt_id: &str) {
        self.extension_activity
            .retain(|item| item.entry_id.as_deref() != Some(receipt_id));
        self.save_extension_activity();
        self.status_line = "Cleared proof timeline".to_string();
        self.push_toast("Cleared proof timeline", ToastKind::Success);
    }

    fn clear_all_activity(&mut self) {
        self.extension_activity.clear();
        self.save_extension_activity();
        self.status_line = "Cleared extension activity history".to_string();
        self.push_toast("Cleared extension activity history", ToastKind::Success);
    }

    fn export_extension_summary(
        &mut self,
        entry: &PendingExtensionEntry,
        blockers: &[String],
        mismatch_hint_count: usize,
    ) {
        let export_dir = self.extension_exports_dir();
        let latest_export_path = export_dir.join(format!("{}.md", entry.entry_id));
        let history_export_path =
            export_dir.join(format!("{}-{}.md", entry.entry_id, export_timestamp_slug()));
        let note = self
            .extension_notes
            .get(&entry.entry_id)
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let summary = build_extension_summary_markdown(entry, blockers, mismatch_hint_count, note);
        match write_text_file(&latest_export_path, &summary)
            .and_then(|_| write_text_file(&history_export_path, &summary))
        {
            Ok(()) => {
                self.status_line = "Exported lane summary".to_string();
                self.push_extension_activity(
                    Some(entry.entry_id.clone()),
                    "Export Lane Summary",
                    short_path(history_export_path.to_string_lossy().as_ref()),
                    true,
                );
                self.push_toast("Exported lane summary", ToastKind::Success);
            }
            Err(error) => {
                self.status_line = format!("Export failed: {error}");
                self.push_extension_activity(
                    Some(entry.entry_id.clone()),
                    "Export Lane Summary",
                    error.to_string(),
                    false,
                );
                self.push_toast(format!("Export failed: {error}"), ToastKind::Error);
            }
        }
    }

    fn export_paired_proof_summary(
        &mut self,
        receipt: &CrossFamilyPairedProofReceiptSummary,
        receipt_path: &Path,
    ) {
        let export_dir = self.paired_proof_exports_dir();
        let latest_export_path = export_dir.join(format!("{}.md", receipt.receipt_id));
        let history_export_path = export_dir.join(format!(
            "{}-{}.md",
            receipt.receipt_id,
            export_timestamp_slug()
        ));
        let note = self
            .paired_proof_notes
            .get(&receipt.receipt_id)
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let summary = build_paired_proof_summary_markdown(receipt, receipt_path, note);
        match write_text_file(&latest_export_path, &summary)
            .and_then(|_| write_text_file(&history_export_path, &summary))
        {
            Ok(()) => {
                self.status_line = "Exported proof summary".to_string();
                self.push_extension_activity(
                    Some(receipt.receipt_id.clone()),
                    "Export Proof Summary",
                    short_path(history_export_path.to_string_lossy().as_ref()),
                    true,
                );
                self.push_toast("Exported proof summary", ToastKind::Success);
            }
            Err(error) => {
                self.status_line = format!("Proof export failed: {error}");
                self.push_extension_activity(
                    Some(receipt.receipt_id.clone()),
                    "Export Proof Summary",
                    error.to_string(),
                    false,
                );
                self.push_toast(format!("Proof export failed: {error}"), ToastKind::Error);
            }
        }
    }

    fn copy_paired_proof_summary(
        &mut self,
        receipt: &CrossFamilyPairedProofReceiptSummary,
        receipt_path: &Path,
        ctx: &egui::Context,
    ) {
        let note = self
            .paired_proof_notes
            .get(&receipt.receipt_id)
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let summary = build_paired_proof_summary_markdown(receipt, receipt_path, note);
        ctx.copy_text(summary);
        self.status_line = "Copied proof summary to clipboard".to_string();
        self.push_extension_activity(
            Some(receipt.receipt_id.clone()),
            "Copy Proof Summary",
            receipt.receipt_id.clone(),
            true,
        );
        self.push_toast("Copied proof summary to clipboard", ToastKind::Success);
    }

    fn latest_paired_proof_export_path(&self, receipt_id: &str) -> PathBuf {
        self.paired_proof_exports_dir()
            .join(format!("{}.md", receipt_id))
    }

    fn paired_proof_export_history_paths(&self, receipt_id: &str) -> Vec<PathBuf> {
        load_extension_export_history(&self.paired_proof_exports_dir(), receipt_id)
    }

    fn latest_extension_export_path(&self, entry_id: &str) -> PathBuf {
        self.extension_exports_dir()
            .join(format!("{}.md", entry_id))
    }

    fn extension_export_history_paths(&self, entry_id: &str) -> Vec<PathBuf> {
        load_extension_export_history(&self.extension_exports_dir(), entry_id)
    }

    fn push_toast(&mut self, message: impl Into<String>, kind: ToastKind) {
        self.toasts.push(UiToast {
            message: message.into(),
            kind,
            created_at: Instant::now(),
        });
    }

    fn prune_toasts(&mut self) {
        let ttl = Duration::from_secs(4);
        self.toasts.retain(|toast| toast.created_at.elapsed() < ttl);
        if self.toasts.len() > 4 {
            let keep_from = self.toasts.len().saturating_sub(4);
            self.toasts.drain(0..keep_from);
        }
    }

    fn push_extension_activity(
        &mut self,
        entry_id: Option<String>,
        title: impl Into<String>,
        detail: impl Into<String>,
        success: bool,
    ) {
        self.extension_activity.push(ExtensionActivityItem {
            entry_id,
            title: title.into(),
            detail: detail.into(),
            success,
            timestamp_label: activity_timestamp_label(),
        });
        if self.extension_activity.len() > 8 {
            let keep_from = self.extension_activity.len().saturating_sub(8);
            self.extension_activity.drain(0..keep_from);
        }
        self.save_extension_activity();
    }

    fn reveal_governed_artifact(
        &mut self,
        path: &str,
        success_status: &str,
        failure_status_label: &str,
        success_toast: &str,
        failure_toast: &str,
        activity: Option<(Option<String>, &str)>,
    ) {
        match open_path_in_explorer(path, true) {
            Ok(()) => {
                self.status_line = success_status.to_string();
                if let Some((entry_id, title)) = activity {
                    self.push_extension_activity(entry_id, title, short_path(path), true);
                }
                self.push_toast(success_toast, ToastKind::Success);
            }
            Err(error) => {
                self.status_line = format!("{failure_status_label}: {error}");
                if let Some((entry_id, title)) = activity {
                    self.push_extension_activity(entry_id, title, error.to_string(), false);
                }
                self.push_toast(failure_toast, ToastKind::Error);
            }
        }
    }

    fn record_extension_task_result(&mut self, summary: &str, stderr: &str) {
        let lower = summary.to_ascii_lowercase();
        let is_extension_result = lower.contains("extension")
            || lower.contains("lane ")
            || lower.contains("host wire")
            || lower.contains("apply-patch");
        if !is_extension_result {
            return;
        }

        let detail = if stderr.trim().is_empty() {
            "Completed".to_string()
        } else {
            stderr.lines().next().unwrap_or("Failed").to_string()
        };
        self.push_extension_activity(None, summary, detail, stderr.trim().is_empty());
    }

    fn selected_extension_entry(&self) -> Option<PendingExtensionEntry> {
        let selected_id = self.selected_extension_entry_id.as_deref()?;
        let registry = self.extension_registry.as_ref()?;
        registry
            .active_entries
            .iter()
            .chain(registry.fully_live_entries.iter())
            .chain(registry.archived_entries.iter())
            .find(|entry| entry.entry_id == selected_id)
            .cloned()
    }

    fn spawn_task(&mut self, task: UiTask) {
        if self.task_running {
            self.status_line = "A task is already running".to_string();
            return;
        }

        let recent_entry_id = match &task {
            UiTask::ImplementExtension { entry_id }
            | UiTask::ValidateExtension { entry_id }
            | UiTask::PrepareExtensionPromotion { entry_id }
            | UiTask::PrepareExtensionApplyPatch { entry_id }
            | UiTask::ConsumeExtensionApplyPatch { entry_id }
            | UiTask::ValidateLiveExtension { entry_id } => Some(entry_id.clone()),
            UiTask::ArchiveExtension { entry_id, .. } => Some(entry_id.clone()),
            _ => None,
        };
        if let Some(entry_id) = recent_entry_id {
            self.mark_extension_recent(&entry_id);
        }

        self.task_running = true;
        self.status_line = match &task {
            UiTask::RefreshBrowser => "Refreshing project browser".to_string(),
            UiTask::RefreshRuntime => "Refreshing runtime status".to_string(),
            UiTask::RefreshProjectPatchReadiness => {
                "Refreshing project patch readiness".to_string()
            }
            UiTask::RefreshProofHarnessRegistry => {
                "Refreshing proof governance registry".to_string()
            }
            UiTask::RefreshCompositionGovernance => {
                "Refreshing composition governance registry".to_string()
            }
            UiTask::RefreshPatchGovernance => "Refreshing patch governance registry".to_string(),
            UiTask::RefreshHelperGovernance => "Refreshing helper governance registry".to_string(),
            UiTask::RefreshBridgeGovernance => "Refreshing bridge governance registry".to_string(),
            UiTask::RefreshFamilyGovernance => "Refreshing family governance registry".to_string(),
            UiTask::RefreshTemplateGovernance => {
                "Refreshing template governance registry".to_string()
            }
            UiTask::ApproveProposedConstraint { .. } => "Approving proposed constraint".to_string(),
            UiTask::SetApprovedConstraintActive { active, .. } => {
                if *active {
                    "Activating approved constraint".to_string()
                } else {
                    "Deactivating approved constraint".to_string()
                }
            }
            UiTask::ArchiveUnmatchedInactiveConstraints => {
                "Archiving unmatched inactive constraints".to_string()
            }
            UiTask::DeactivateLowValueActiveConstraints => {
                "Deactivating low-value active constraints".to_string()
            }
            UiTask::RestoreApprovedConstraint { .. } => {
                "Restoring approved constraint from history".to_string()
            }
            UiTask::SelectProject { project_name } => format!("Selecting project {project_name}"),
            UiTask::ClearSelectedProject => "Clearing selected project".to_string(),
            UiTask::ImplementExtension { entry_id } => {
                format!("Implementing extension {entry_id}")
            }
            UiTask::ValidateExtension { entry_id } => {
                format!("Validating extension {entry_id}")
            }
            UiTask::PrepareExtensionPromotion { entry_id } => {
                format!("Preparing promotion for {entry_id}")
            }
            UiTask::PrepareExtensionApplyPatch { entry_id } => {
                format!("Preparing apply-patch for {entry_id}")
            }
            UiTask::ConsumeExtensionApplyPatch { entry_id } => {
                format!("Host wiring extension {entry_id}")
            }
            UiTask::ValidateLiveExtension { entry_id } => {
                format!("Validating fully live extension {entry_id}")
            }
            UiTask::ArchiveExtension { entry_id, .. } => {
                format!("Archiving extension {entry_id}")
            }
            UiTask::RunProofTemplate { template_id, .. } => {
                format!("Running proof template {template_id}")
            }
            UiTask::RunRetrySearchLadderProof { .. } => {
                "Running retry-search ladder proof".to_string()
            }
            UiTask::BuildRequest { .. } => "Running build request".to_string(),
            UiTask::PatchRequest { .. } => "Running patch request".to_string(),
        };

        let sender = self.task_sender.clone();
        let workspace_root = self.workspace_root.clone();
        thread::spawn(move || {
            let result = run_ui_task(&workspace_root, task);
            let payload = match result {
                Ok(payload) => payload,
                Err(error) => UiTaskResult {
                    summary: "Task failed".to_string(),
                    stdout: String::new(),
                    stderr: error.to_string(),
                    browser_state: load_browser_state(&workspace_root),
                    action_summary: Some(ActionSummary {
                        title: "Task failed".to_string(),
                        lines: vec![error.to_string()],
                    }),
                    execution_result: None,
                    fallback_result: None,
                    extension_registry: load_extension_registry(&workspace_root),
                },
            };
            let _ = sender.send(payload);
        });
    }

    fn poll_task_results(&mut self) {
        loop {
            match self.task_receiver.try_recv() {
                Ok(result) => {
                    self.task_running = false;
                    self.status_line = result.summary.clone();
                    self.command_log = format_log(&result.summary, &result.stdout, &result.stderr);
                    if result.stderr.trim().is_empty() {
                        self.push_toast(result.summary.clone(), ToastKind::Success);
                    } else {
                        self.push_toast(result.summary.clone(), ToastKind::Error);
                    }
                    self.record_extension_task_result(&result.summary, &result.stderr);
                    if let Some(action_summary) = result.action_summary {
                        self.last_action_summary = action_summary;
                    }
                    self.last_execution_result = result.execution_result;
                    self.last_fallback_result = result.fallback_result;
                    self.extension_registry = result
                        .extension_registry
                        .or_else(|| load_extension_registry(&self.workspace_root));
                    self.proof_governance_refresh_status =
                        load_proof_governance_refresh_status(&self.workspace_root);
                    self.composition_governance_refresh_status =
                        load_composition_governance_refresh_status(&self.workspace_root);
                    self.patch_governance_refresh_status =
                        load_patch_governance_refresh_status(&self.workspace_root);
                    self.project_patch_readiness_refresh_status =
                        load_project_patch_readiness_refresh_status(&self.workspace_root);
                    self.helper_governance_refresh_status =
                        load_helper_governance_refresh_status(&self.workspace_root);
                    self.bridge_governance_refresh_status =
                        load_bridge_governance_refresh_status(&self.workspace_root);
                    self.family_governance_refresh_status =
                        load_family_governance_refresh_status(&self.workspace_root);
                    self.template_governance_refresh_status =
                        load_template_governance_refresh_status(&self.workspace_root);
                    if self.selected_family_governance_id.is_none() {
                        self.selected_family_governance_id =
                            load_family_governance_receipts(&self.workspace_root)
                                .first()
                                .map(|receipt| receipt.family_id.clone());
                    }
                    if self.selected_template_governance_id.is_none() {
                        self.selected_template_governance_id =
                            load_template_governance_receipts(&self.workspace_root)
                                .first()
                                .map(|receipt| receipt.template_bundle_id.clone());
                    }
                    if let Some(state) = result.browser_state {
                        self.sync_browser_state(state);
                    } else {
                        self.refresh_from_runtime_file();
                    }
                    self.refresh_runtime_status();
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.task_running = false;
                    self.status_line = "Background task channel disconnected".to_string();
                    break;
                }
            }
        }
    }

    fn selected_project(&self) -> Option<&str> {
        self.selected_project_name.as_deref()
    }
}

fn extension_entry_matches_query(entry: &PendingExtensionEntry, query: &str) -> bool {
    let query = query.trim().to_ascii_lowercase();
    if query.is_empty() {
        return true;
    }
    let mut fields = vec![
        entry.entry_id.as_str(),
        entry.status.as_str(),
        entry.extension_kind.as_str(),
        entry.family_id.as_deref().unwrap_or(""),
        entry.tool_kind.as_deref().unwrap_or(""),
        entry.patch_kind.as_deref().unwrap_or(""),
        entry.archived_reason.as_deref().unwrap_or(""),
        entry.proof_quality_status.as_deref().unwrap_or(""),
        entry.proof_seed_template_id.as_deref().unwrap_or(""),
        entry.proof_seed_bundle_id.as_deref().unwrap_or(""),
    ];
    for layer in &entry.unresolved_layers {
        fields.push(layer.as_str());
    }
    for class in &entry.missing_family_build_primitive_classes {
        fields.push(class.as_str());
    }
    for class in &entry.missing_patch_primitive_classes {
        fields.push(class.as_str());
    }
    for kind in &entry.missing_helper_primitive_kinds {
        fields.push(kind.as_str());
    }
    fields
        .iter()
        .any(|field| field.to_ascii_lowercase().contains(&query))
}

fn proof_quality_matches(entry: &PendingExtensionEntry, filter: ProofQualityFilter) -> bool {
    match filter {
        ProofQualityFilter::All => true,
        ProofQualityFilter::Passing => {
            entry.extension_kind == "proof_harness_bundle"
                && entry.proof_quality_status.as_deref() == Some("passing")
        }
        ProofQualityFilter::RunnableDiverged => {
            entry.extension_kind == "proof_harness_bundle"
                && entry.proof_quality_status.as_deref() == Some("runnable_diverged")
        }
        ProofQualityFilter::CatalogResolved => {
            entry.extension_kind == "proof_harness_bundle"
                && entry.proof_quality_status.as_deref() == Some("catalog_resolved")
        }
        ProofQualityFilter::NeedsContractFix => {
            entry.extension_kind == "proof_harness_bundle"
                && entry.proof_quality_status.as_deref() == Some("needs_contract_fix")
        }
        ProofQualityFilter::Unknown => {
            entry.extension_kind == "proof_harness_bundle"
                && entry.proof_quality_status.as_deref().unwrap_or("unknown") == "unknown"
        }
    }
}

fn proof_baseline_matches(entry: &PendingExtensionEntry, filter: ProofBaselineFilter) -> bool {
    match filter {
        ProofBaselineFilter::All => true,
        ProofBaselineFilter::Stable => {
            entry.extension_kind == "proof_harness_bundle"
                && entry.proof_change_since_last_pass_status.as_deref()
                    == Some("stable_since_last_pass")
        }
        ProofBaselineFilter::Changed => {
            entry.extension_kind == "proof_harness_bundle"
                && matches!(
                    entry.proof_change_since_last_pass_status.as_deref(),
                    Some("changed_since_last_pass") | Some("baseline_recorded")
                )
        }
        ProofBaselineFilter::Regressed => {
            entry.extension_kind == "proof_harness_bundle"
                && entry.proof_change_since_last_pass_status.as_deref()
                    == Some("regressed_since_last_pass")
        }
        ProofBaselineFilter::NoBaseline => {
            entry.extension_kind == "proof_harness_bundle"
                && entry.proof_change_since_last_pass_status.as_deref()
                    == Some("no_passing_baseline")
        }
        ProofBaselineFilter::Unknown => {
            entry.extension_kind == "proof_harness_bundle"
                && entry
                    .proof_change_since_last_pass_status
                    .as_deref()
                    .unwrap_or("unknown")
                    == "unknown"
        }
    }
}

fn composition_baseline_matches(
    entry: &PendingExtensionEntry,
    filter: CompositionBaselineFilter,
) -> bool {
    match filter {
        CompositionBaselineFilter::All => true,
        CompositionBaselineFilter::Stable => {
            entry.extension_kind == "composition_bundle"
                && entry.composition_change_since_last_live_status.as_deref()
                    == Some("stable_since_last_live")
        }
        CompositionBaselineFilter::Changed => {
            entry.extension_kind == "composition_bundle"
                && matches!(
                    entry.composition_change_since_last_live_status.as_deref(),
                    Some("changed_since_last_live") | Some("baseline_recorded")
                )
        }
        CompositionBaselineFilter::Regressed => {
            entry.extension_kind == "composition_bundle"
                && entry.composition_change_since_last_live_status.as_deref()
                    == Some("regressed_since_last_live")
        }
        CompositionBaselineFilter::NoBaseline => {
            entry.extension_kind == "composition_bundle"
                && entry.composition_change_since_last_live_status.as_deref()
                    == Some("no_live_baseline")
        }
        CompositionBaselineFilter::Unknown => {
            entry.extension_kind == "composition_bundle"
                && entry
                    .composition_change_since_last_live_status
                    .as_deref()
                    .unwrap_or("unknown")
                    == "unknown"
        }
    }
}

fn patch_baseline_matches(entry: &PendingExtensionEntry, filter: PatchBaselineFilter) -> bool {
    match filter {
        PatchBaselineFilter::All => true,
        PatchBaselineFilter::Stable => {
            entry.extension_kind == "patch_recipe"
                && entry.patch_change_since_last_live_status.as_deref()
                    == Some("stable_since_last_live")
        }
        PatchBaselineFilter::Changed => {
            entry.extension_kind == "patch_recipe"
                && matches!(
                    entry.patch_change_since_last_live_status.as_deref(),
                    Some("changed_since_last_live") | Some("baseline_recorded")
                )
        }
        PatchBaselineFilter::Regressed => {
            entry.extension_kind == "patch_recipe"
                && entry.patch_change_since_last_live_status.as_deref()
                    == Some("regressed_since_last_live")
        }
        PatchBaselineFilter::NoBaseline => {
            entry.extension_kind == "patch_recipe"
                && entry.patch_change_since_last_live_status.as_deref() == Some("no_live_baseline")
        }
        PatchBaselineFilter::Unknown => {
            entry.extension_kind == "patch_recipe"
                && entry
                    .patch_change_since_last_live_status
                    .as_deref()
                    .unwrap_or("unknown")
                    == "unknown"
        }
    }
}

fn helper_baseline_matches(entry: &PendingExtensionEntry, filter: HelperBaselineFilter) -> bool {
    match filter {
        HelperBaselineFilter::All => true,
        HelperBaselineFilter::Stable => {
            entry.extension_kind == "helper_lane"
                && entry.helper_change_since_last_live_status.as_deref()
                    == Some("stable_since_last_live")
        }
        HelperBaselineFilter::Changed => {
            entry.extension_kind == "helper_lane"
                && matches!(
                    entry.helper_change_since_last_live_status.as_deref(),
                    Some("changed_since_last_live") | Some("baseline_recorded")
                )
        }
        HelperBaselineFilter::Regressed => {
            entry.extension_kind == "helper_lane"
                && entry.helper_change_since_last_live_status.as_deref()
                    == Some("regressed_since_last_live")
        }
        HelperBaselineFilter::NoBaseline => {
            entry.extension_kind == "helper_lane"
                && entry.helper_change_since_last_live_status.as_deref() == Some("no_live_baseline")
        }
        HelperBaselineFilter::Unknown => {
            entry.extension_kind == "helper_lane"
                && entry
                    .helper_change_since_last_live_status
                    .as_deref()
                    .unwrap_or("unknown")
                    == "unknown"
        }
    }
}

fn bridge_baseline_matches(entry: &PendingExtensionEntry, filter: BridgeBaselineFilter) -> bool {
    match filter {
        BridgeBaselineFilter::All => true,
        BridgeBaselineFilter::Stable => {
            entry.extension_kind == "chattycog_bridge_lane"
                && entry.bridge_change_since_last_live_status.as_deref()
                    == Some("stable_since_last_live")
        }
        BridgeBaselineFilter::Changed => {
            entry.extension_kind == "chattycog_bridge_lane"
                && matches!(
                    entry.bridge_change_since_last_live_status.as_deref(),
                    Some("changed_since_last_live") | Some("baseline_recorded")
                )
        }
        BridgeBaselineFilter::Regressed => {
            entry.extension_kind == "chattycog_bridge_lane"
                && entry.bridge_change_since_last_live_status.as_deref()
                    == Some("regressed_since_last_live")
        }
        BridgeBaselineFilter::NoBaseline => {
            entry.extension_kind == "chattycog_bridge_lane"
                && entry.bridge_change_since_last_live_status.as_deref() == Some("no_live_baseline")
        }
        BridgeBaselineFilter::Unknown => {
            entry.extension_kind == "chattycog_bridge_lane"
                && entry
                    .bridge_change_since_last_live_status
                    .as_deref()
                    .unwrap_or("unknown")
                    == "unknown"
        }
    }
}

fn extension_entry_matches_filters(
    entry: &PendingExtensionEntry,
    query: &str,
    proof_quality_filter: ProofQualityFilter,
    proof_baseline_filter: ProofBaselineFilter,
    composition_baseline_filter: CompositionBaselineFilter,
    patch_baseline_filter: PatchBaselineFilter,
    helper_baseline_filter: HelperBaselineFilter,
    bridge_baseline_filter: BridgeBaselineFilter,
) -> bool {
    extension_entry_matches_query(entry, query)
        && proof_quality_matches(entry, proof_quality_filter)
        && proof_baseline_matches(entry, proof_baseline_filter)
        && composition_baseline_matches(entry, composition_baseline_filter)
        && patch_baseline_matches(entry, patch_baseline_filter)
        && helper_baseline_matches(entry, helper_baseline_filter)
        && bridge_baseline_matches(entry, bridge_baseline_filter)
}

fn proof_quality_badge(entry: &PendingExtensionEntry) -> Option<String> {
    if entry.extension_kind != "proof_harness_bundle" {
        return None;
    }
    Some(
        match entry.proof_quality_status.as_deref().unwrap_or("unknown") {
            "passing" => "proof:passing".to_string(),
            "runnable_diverged" => "proof:diverged".to_string(),
            "catalog_resolved" => "proof:catalog".to_string(),
            "needs_contract_fix" => "proof:fix".to_string(),
            other => format!("proof:{other}"),
        },
    )
}

fn proof_baseline_badge(entry: &PendingExtensionEntry) -> Option<String> {
    if entry.extension_kind != "proof_harness_bundle" {
        return None;
    }
    Some(
        match entry
            .proof_change_since_last_pass_status
            .as_deref()
            .unwrap_or("unknown")
        {
            "stable_since_last_pass" => "proof:stable".to_string(),
            "changed_since_last_pass" => "proof:changed".to_string(),
            "baseline_recorded" => "proof:baseline".to_string(),
            "regressed_since_last_pass" => "proof:regressed".to_string(),
            "no_passing_baseline" => "proof:no-baseline".to_string(),
            other => format!("proof:baseline:{other}"),
        },
    )
}

fn composition_drift_badge(entry: &PendingExtensionEntry) -> Option<String> {
    if entry.extension_kind != "composition_bundle" {
        return None;
    }
    Some(
        match entry
            .composition_drift_status
            .as_deref()
            .unwrap_or("unknown")
        {
            "seed_aligned" => "composition:aligned".to_string(),
            "structurally_customized" => "composition:custom".to_string(),
            "drifted_risky" => "composition:risky".to_string(),
            "unseeded" => "composition:unseeded".to_string(),
            other => format!("composition:{other}"),
        },
    )
}

fn composition_baseline_badge(entry: &PendingExtensionEntry) -> Option<String> {
    if entry.extension_kind != "composition_bundle" {
        return None;
    }
    Some(
        match entry
            .composition_change_since_last_live_status
            .as_deref()
            .unwrap_or("unknown")
        {
            "stable_since_last_live" => "composition:stable".to_string(),
            "changed_since_last_live" => "composition:changed".to_string(),
            "regressed_since_last_live" => "composition:regressed".to_string(),
            "baseline_recorded" => "composition:baseline".to_string(),
            "no_live_baseline" => "composition:no-baseline".to_string(),
            other => format!("composition:baseline:{other}"),
        },
    )
}

fn patch_drift_badge(entry: &PendingExtensionEntry) -> Option<String> {
    if entry.extension_kind != "patch_recipe" {
        return None;
    }
    Some(
        match entry.patch_drift_status.as_deref().unwrap_or("unknown") {
            "seed_aligned" => "patch:aligned".to_string(),
            "lightly_customized" => "patch:custom".to_string(),
            "structurally_customized" => "patch:structural".to_string(),
            "drifted_risky" => "patch:risky".to_string(),
            "unseeded" => "patch:unseeded".to_string(),
            other => format!("patch:{other}"),
        },
    )
}

fn patch_baseline_badge(entry: &PendingExtensionEntry) -> Option<String> {
    if entry.extension_kind != "patch_recipe" {
        return None;
    }
    Some(
        match entry
            .patch_change_since_last_live_status
            .as_deref()
            .unwrap_or("unknown")
        {
            "stable_since_last_live" => "patch:stable".to_string(),
            "changed_since_last_live" => "patch:changed".to_string(),
            "regressed_since_last_live" => "patch:regressed".to_string(),
            "baseline_recorded" => "patch:baseline".to_string(),
            "no_live_baseline" => "patch:no-baseline".to_string(),
            other => format!("patch:baseline:{other}"),
        },
    )
}

fn helper_drift_badge(entry: &PendingExtensionEntry) -> Option<String> {
    if entry.extension_kind != "helper_lane" {
        return None;
    }
    Some(
        match entry.helper_drift_status.as_deref().unwrap_or("unknown") {
            "seed_aligned" => "helper:aligned".to_string(),
            "lightly_customized" => "helper:custom".to_string(),
            "structurally_customized" => "helper:structural".to_string(),
            "drifted_risky" => "helper:risky".to_string(),
            "unseeded" => "helper:unseeded".to_string(),
            other => format!("helper:{other}"),
        },
    )
}

fn helper_baseline_badge(entry: &PendingExtensionEntry) -> Option<String> {
    if entry.extension_kind != "helper_lane" {
        return None;
    }
    Some(
        match entry
            .helper_change_since_last_live_status
            .as_deref()
            .unwrap_or("unknown")
        {
            "stable_since_last_live" => "helper:stable".to_string(),
            "changed_since_last_live" => "helper:changed".to_string(),
            "regressed_since_last_live" => "helper:regressed".to_string(),
            "baseline_recorded" => "helper:baseline".to_string(),
            "no_live_baseline" => "helper:no-baseline".to_string(),
            other => format!("helper:baseline:{other}"),
        },
    )
}

fn bridge_drift_badge(entry: &PendingExtensionEntry) -> Option<String> {
    if entry.extension_kind != "chattycog_bridge_lane" {
        return None;
    }
    Some(
        match entry.bridge_drift_status.as_deref().unwrap_or("unknown") {
            "seed_aligned" => "bridge:aligned".to_string(),
            "lightly_customized" => "bridge:custom".to_string(),
            "structurally_customized" => "bridge:structural".to_string(),
            "drifted_risky" => "bridge:risky".to_string(),
            "unseeded" => "bridge:unseeded".to_string(),
            other => format!("bridge:{other}"),
        },
    )
}

fn bridge_baseline_badge(entry: &PendingExtensionEntry) -> Option<String> {
    if entry.extension_kind != "chattycog_bridge_lane" {
        return None;
    }
    Some(
        match entry
            .bridge_change_since_last_live_status
            .as_deref()
            .unwrap_or("unknown")
        {
            "stable_since_last_live" => "bridge:stable".to_string(),
            "changed_since_last_live" => "bridge:changed".to_string(),
            "regressed_since_last_live" => "bridge:regressed".to_string(),
            "baseline_recorded" => "bridge:baseline".to_string(),
            "no_live_baseline" => "bridge:no-baseline".to_string(),
            other => format!("bridge:baseline:{other}"),
        },
    )
}

fn family_governed_artifact_set_summary(receipt: &FamilyGovernanceReceiptView) -> String {
    let manifest_count = if receipt.manifest_path.trim().is_empty() {
        0
    } else {
        1
    };
    format!(
        "Governed artifact set: {manifest_count} family manifest plus declared primitive adapter surface"
    )
}

fn template_governed_artifact_set_summary(receipt: &TemplateGovernanceReceiptView) -> String {
    format!(
        "Governed artifact set: {} template artifact(s) under the selected bundle root",
        receipt.artifact_paths.len()
    )
}

fn extension_governed_artifact_set_summary(entry: &PendingExtensionEntry) -> Option<String> {
    match entry.extension_kind.as_str() {
        "composition_bundle" => Some(format!(
            "Governed artifact set: {} integrated artifact(s) across unresolved layers",
            entry.integrated_paths.len()
        )),
        "patch_recipe" => {
            let has_acceptance = entry.integrated_paths.iter().any(|path| {
                path.contains("operator_registry") && path.contains("acceptance_recipes")
            });
            Some(format!(
                "Governed artifact set: patch recipe{}",
                if has_acceptance {
                    " plus paired acceptance contract"
                } else {
                    ""
                }
            ))
        }
        "helper_lane" => Some(format!(
            "Governed artifact set: {} integrated helper artifact(s)",
            entry.integrated_paths.len()
        )),
        "chattycog_bridge_lane" => Some(format!(
            "Governed artifact set: {} integrated bridge artifact(s)",
            entry.integrated_paths.len()
        )),
        _ => None,
    }
}

fn family_display_name(family_id: &str) -> &'static str {
    match family_id {
        "chattycog_native_window_module" => "Chatty-Cog Rust Native Dashboard",
        "chattyedu_native_window_module" => "Chatty-EDU Rust Native Dashboard",
        "chattycog_chattyedu_native_window_module" => {
            "Chatty-Cog + Chatty-EDU Rust Native Dashboard"
        }
        "chattycog_webview_module" => "Chatty-Cog Webview Module",
        "chattycog_workspace_module" => "Chatty-Cog Workspace Module",
        "static_web_dashboard" => "Static Web Dashboard",
        "rust_cli_tool" => "Rust CLI Tool",
        "python_cli_tool" => "Python CLI Tool",
        _ => "Unknown Family",
    }
}

fn family_ecosystem_badge(family_id: &str) -> Option<&'static str> {
    match family_id {
        "chattycog_native_window_module"
        | "chattycog_webview_module"
        | "chattycog_workspace_module" => Some("[ecosystem: Chatty-Cog]"),
        "chattyedu_native_window_module" => Some("[ecosystem: Chatty-EDU]"),
        "chattycog_chattyedu_native_window_module" => Some("[ecosystem: Chatty-Cog + Chatty-EDU]"),
        _ => None,
    }
}

fn family_summary_label(family_id: &str) -> String {
    if let Some(badge) = family_ecosystem_badge(family_id) {
        format!("{} {}", family_display_name(family_id), badge)
    } else {
        family_display_name(family_id).to_string()
    }
}

fn family_governance_picker_label(receipt: &FamilyGovernanceReceiptView) -> String {
    format!(
        "{} [{}]",
        family_summary_label_from_receipt(receipt),
        receipt.lifecycle_status
    )
}

fn family_summary_label_from_receipt(receipt: &FamilyGovernanceReceiptView) -> String {
    match (
        receipt.family_display_name.as_deref(),
        receipt.family_ecosystem.as_deref(),
    ) {
        (Some(display_name), Some(ecosystem)) if !display_name.trim().is_empty() => {
            format!("{display_name} [ecosystem: {ecosystem}]")
        }
        (Some(display_name), _) if !display_name.trim().is_empty() => display_name.to_string(),
        _ => family_summary_label(&receipt.family_id),
    }
}

fn family_summary_label_from_patchability_receipt(
    family_id: &str,
    receipt: Option<&ProjectPatchReadinessReceiptView>,
) -> String {
    if let Some(receipt) = receipt {
        match (
            receipt.family_display_name.as_deref(),
            receipt.family_ecosystem.as_deref(),
        ) {
            (Some(display_name), Some(ecosystem)) if !display_name.trim().is_empty() => {
                return format!("{display_name} [ecosystem: {ecosystem}]");
            }
            (Some(display_name), _) if !display_name.trim().is_empty() => {
                return display_name.to_string();
            }
            _ => {}
        }
    }
    family_summary_label(family_id)
}

fn is_canonical_ecosystem_shell_family(family_id: &str) -> bool {
    matches!(
        family_id,
        "chattycog_native_window_module"
            | "chattyedu_native_window_module"
            | "chattycog_chattyedu_native_window_module"
    )
}

fn count_proof_baseline_status(
    registry: &HostExtensionRegistryView,
    expected_status: &str,
) -> usize {
    registry
        .active_entries
        .iter()
        .chain(registry.fully_live_entries.iter())
        .chain(registry.archived_entries.iter())
        .filter(|entry| entry.extension_kind == "proof_harness_bundle")
        .filter(|entry| {
            entry.proof_change_since_last_pass_status.as_deref() == Some(expected_status)
        })
        .count()
}

fn count_composition_baseline_status(
    registry: &HostExtensionRegistryView,
    expected_status: &str,
) -> usize {
    registry
        .active_entries
        .iter()
        .chain(registry.fully_live_entries.iter())
        .chain(registry.archived_entries.iter())
        .filter(|entry| entry.extension_kind == "composition_bundle")
        .filter(|entry| {
            entry.composition_change_since_last_live_status.as_deref() == Some(expected_status)
        })
        .count()
}

fn count_patch_baseline_status(
    registry: &HostExtensionRegistryView,
    expected_status: &str,
) -> usize {
    registry
        .active_entries
        .iter()
        .chain(registry.fully_live_entries.iter())
        .chain(registry.archived_entries.iter())
        .filter(|entry| entry.extension_kind == "patch_recipe")
        .filter(|entry| {
            entry.patch_change_since_last_live_status.as_deref() == Some(expected_status)
        })
        .count()
}

fn count_helper_baseline_status(
    registry: &HostExtensionRegistryView,
    expected_status: &str,
) -> usize {
    registry
        .active_entries
        .iter()
        .chain(registry.fully_live_entries.iter())
        .chain(registry.archived_entries.iter())
        .filter(|entry| entry.extension_kind == "helper_lane")
        .filter(|entry| {
            entry.helper_change_since_last_live_status.as_deref() == Some(expected_status)
        })
        .count()
}

fn count_bridge_baseline_status(
    registry: &HostExtensionRegistryView,
    expected_status: &str,
) -> usize {
    registry
        .active_entries
        .iter()
        .chain(registry.fully_live_entries.iter())
        .chain(registry.archived_entries.iter())
        .filter(|entry| entry.extension_kind == "chattycog_bridge_lane")
        .filter(|entry| {
            entry.bridge_change_since_last_live_status.as_deref() == Some(expected_status)
        })
        .count()
}

fn proof_governance_refresh_is_stale(status: &ProofGovernanceRefreshStatusView) -> bool {
    status.age_minutes.unwrap_or(0) >= 60
}

fn proof_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs: Option<i64>) -> bool {
    let Some(last) = last_auto_refresh_unix_secs else {
        return false;
    };
    let now = Utc::now().timestamp();
    now.saturating_sub(last) < 15 * 60
}

fn latest_cross_family_proof_receipt_modified(
    workspace_root: &Path,
) -> Option<(std::time::SystemTime, String)> {
    let root = workspace_root
        .join("runtime")
        .join("cross_family_paired_proof_receipts");
    let entries = fs::read_dir(root).ok()?;
    entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
        })
        .filter_map(|path| {
            let modified = fs::metadata(&path).ok()?.modified().ok()?;
            let label = modified.into();
            let label: chrono::DateTime<Utc> = label;
            Some((
                modified,
                label
                    .with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            ))
        })
        .max_by_key(|(modified, _)| *modified)
}

fn proof_governance_should_auto_refresh(
    workspace_root: &Path,
    status: Option<&ProofGovernanceRefreshStatusView>,
    auto_refresh_enabled: bool,
    last_auto_refresh_unix_secs: Option<i64>,
) -> bool {
    if !auto_refresh_enabled
        || proof_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs)
    {
        return false;
    }
    let latest_receipt = latest_cross_family_proof_receipt_modified(workspace_root);
    match (status, latest_receipt) {
        (None, Some(_)) => true,
        (Some(status), Some(_)) => {
            proof_governance_refresh_is_stale(status) && status.newer_proof_receipts_exist
        }
        _ => false,
    }
}

fn composition_governance_refresh_is_stale(
    status: &CompositionGovernanceRefreshStatusView,
) -> bool {
    status.age_minutes.unwrap_or(0) >= 60
}

fn composition_governance_auto_refresh_in_cooldown(
    last_auto_refresh_unix_secs: Option<i64>,
) -> bool {
    let Some(last_auto_refresh_unix_secs) = last_auto_refresh_unix_secs else {
        return false;
    };
    let Some(last_auto_refresh) =
        chrono::DateTime::<Utc>::from_timestamp(last_auto_refresh_unix_secs, 0)
    else {
        return false;
    };
    Utc::now()
        .signed_duration_since(last_auto_refresh)
        .num_minutes()
        < 15
}

fn composition_governance_should_auto_refresh(
    status: Option<&CompositionGovernanceRefreshStatusView>,
    auto_refresh_enabled: bool,
    last_auto_refresh_unix_secs: Option<i64>,
) -> bool {
    if !auto_refresh_enabled
        || composition_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs)
    {
        return false;
    }
    match status {
        Some(status) => composition_governance_refresh_is_stale(status),
        None => true,
    }
}

fn patch_governance_refresh_is_stale(status: &PatchGovernanceRefreshStatusView) -> bool {
    status.age_minutes.unwrap_or(0) >= 60
}

fn patch_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs: Option<i64>) -> bool {
    let Some(last_auto_refresh_unix_secs) = last_auto_refresh_unix_secs else {
        return false;
    };
    let Some(last_auto_refresh) =
        chrono::DateTime::<Utc>::from_timestamp(last_auto_refresh_unix_secs, 0)
    else {
        return false;
    };
    Utc::now()
        .signed_duration_since(last_auto_refresh)
        .num_minutes()
        < 15
}

fn patch_governance_should_auto_refresh(
    status: Option<&PatchGovernanceRefreshStatusView>,
    auto_refresh_enabled: bool,
    last_auto_refresh_unix_secs: Option<i64>,
) -> bool {
    if !auto_refresh_enabled
        || patch_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs)
    {
        return false;
    }
    match status {
        Some(status) => patch_governance_refresh_is_stale(status),
        None => true,
    }
}

fn helper_governance_refresh_is_stale(status: &HelperGovernanceRefreshStatusView) -> bool {
    status.age_minutes.unwrap_or(0) >= 60
}

fn helper_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs: Option<i64>) -> bool {
    let Some(last_auto_refresh_unix_secs) = last_auto_refresh_unix_secs else {
        return false;
    };
    let Some(last_auto_refresh) =
        chrono::DateTime::<Utc>::from_timestamp(last_auto_refresh_unix_secs, 0)
    else {
        return false;
    };
    Utc::now()
        .signed_duration_since(last_auto_refresh)
        .num_minutes()
        < 15
}

fn helper_governance_should_auto_refresh(
    status: Option<&HelperGovernanceRefreshStatusView>,
    auto_refresh_enabled: bool,
    last_auto_refresh_unix_secs: Option<i64>,
) -> bool {
    if !auto_refresh_enabled
        || helper_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs)
    {
        return false;
    }
    match status {
        Some(status) => helper_governance_refresh_is_stale(status),
        None => true,
    }
}

fn bridge_governance_refresh_is_stale(status: &BridgeGovernanceRefreshStatusView) -> bool {
    status.age_minutes.unwrap_or(0) >= 60
}

fn bridge_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs: Option<i64>) -> bool {
    let Some(last_auto_refresh_unix_secs) = last_auto_refresh_unix_secs else {
        return false;
    };
    let Some(last_auto_refresh) =
        chrono::DateTime::<Utc>::from_timestamp(last_auto_refresh_unix_secs, 0)
    else {
        return false;
    };
    Utc::now()
        .signed_duration_since(last_auto_refresh)
        .num_minutes()
        < 15
}

fn bridge_governance_should_auto_refresh(
    status: Option<&BridgeGovernanceRefreshStatusView>,
    auto_refresh_enabled: bool,
    last_auto_refresh_unix_secs: Option<i64>,
) -> bool {
    if !auto_refresh_enabled
        || bridge_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs)
    {
        return false;
    }
    match status {
        Some(status) => bridge_governance_refresh_is_stale(status),
        None => true,
    }
}

fn family_governance_refresh_is_stale(status: &FamilyGovernanceRefreshStatusView) -> bool {
    status.age_minutes.unwrap_or(0) >= 60
}

fn family_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs: Option<i64>) -> bool {
    let Some(last_auto_refresh_unix_secs) = last_auto_refresh_unix_secs else {
        return false;
    };
    let Some(last_auto_refresh) =
        chrono::DateTime::<Utc>::from_timestamp(last_auto_refresh_unix_secs, 0)
    else {
        return false;
    };
    Utc::now()
        .signed_duration_since(last_auto_refresh)
        .num_minutes()
        < 15
}

fn family_governance_should_auto_refresh(
    status: Option<&FamilyGovernanceRefreshStatusView>,
    auto_refresh_enabled: bool,
    last_auto_refresh_unix_secs: Option<i64>,
) -> bool {
    if !auto_refresh_enabled
        || family_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs)
    {
        return false;
    }
    match status {
        Some(status) => family_governance_refresh_is_stale(status),
        None => true,
    }
}

fn template_governance_refresh_is_stale(status: &TemplateGovernanceRefreshStatusView) -> bool {
    status.age_minutes.unwrap_or(0) >= 60
}

fn template_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs: Option<i64>) -> bool {
    let Some(last_auto_refresh_unix_secs) = last_auto_refresh_unix_secs else {
        return false;
    };
    let Some(last_auto_refresh) =
        chrono::DateTime::<Utc>::from_timestamp(last_auto_refresh_unix_secs, 0)
    else {
        return false;
    };
    Utc::now()
        .signed_duration_since(last_auto_refresh)
        .num_minutes()
        < 15
}

fn template_governance_should_auto_refresh(
    status: Option<&TemplateGovernanceRefreshStatusView>,
    auto_refresh_enabled: bool,
    last_auto_refresh_unix_secs: Option<i64>,
) -> bool {
    if !auto_refresh_enabled
        || template_governance_auto_refresh_in_cooldown(last_auto_refresh_unix_secs)
    {
        return false;
    }
    match status {
        Some(status) => template_governance_refresh_is_stale(status),
        None => true,
    }
}

fn proof_lineage_status(entry: &PendingExtensionEntry) -> Option<String> {
    if entry.extension_kind != "proof_harness_bundle" {
        return None;
    }
    if let Some(drift_status) = entry.proof_drift_status.as_deref() {
        return Some(drift_status.to_string());
    }
    match (
        entry.proof_seed_template_id.as_deref(),
        entry.patch_kind.as_deref(),
    ) {
        (Some(seed_template_id), Some(template_id)) if seed_template_id == template_id => {
            Some("seed-aligned".to_string())
        }
        (Some(_), Some(_)) => Some("customized-from-seed".to_string()),
        (Some(_), None) => Some("seeded".to_string()),
        _ => Some("unseeded".to_string()),
    }
}

fn latest_proof_receipt_for_template(
    workspace_root: &Path,
    template_id: &str,
) -> Option<(CrossFamilyPairedProofReceiptSummary, PathBuf)> {
    load_cross_family_paired_proof_receipts(workspace_root)
        .into_iter()
        .find(|(receipt, _)| receipt.proof_template_id.as_deref() == Some(template_id))
}

fn extension_layers_summary(entry: &PendingExtensionEntry) -> String {
    if entry.unresolved_layers.is_empty() {
        "layers:none".to_string()
    } else {
        format!("layers:{}", entry.unresolved_layers.join("+"))
    }
}

fn extension_status_rank(status: &str) -> usize {
    match status {
        "pending_implementation" => 0,
        "implemented" => 1,
        "validated_ready" => 2,
        "promotion_prepared" => 3,
        "apply_patch_ready" => 4,
        "host_wired" => 5,
        "fully_live" => 6,
        "archived" => 7,
        _ => 99,
    }
}

fn proof_baseline_risk_rank(entry: &PendingExtensionEntry) -> usize {
    if entry.extension_kind != "proof_harness_bundle" {
        return 99;
    }
    match entry
        .proof_change_since_last_pass_status
        .as_deref()
        .unwrap_or("unknown")
    {
        "regressed_since_last_pass" => 0,
        "changed_since_last_pass" => 1,
        "no_passing_baseline" => 2,
        "baseline_recorded" => 3,
        "stable_since_last_pass" => 4,
        _ => 5,
    }
}

fn proof_drift_risk_rank(entry: &PendingExtensionEntry) -> usize {
    if entry.extension_kind != "proof_harness_bundle" {
        return 99;
    }
    match entry.proof_drift_status.as_deref().unwrap_or("unknown") {
        "drifted_risky" => 0,
        "structurally_customized" => 1,
        "lightly_customized" => 2,
        "seed_aligned" => 3,
        "unseeded" => 4,
        _ => 5,
    }
}

fn sort_extension_entries(entries: &mut Vec<&PendingExtensionEntry>, sort: ExtensionRegistrySort) {
    match sort {
        ExtensionRegistrySort::RecentFirst => {
            entries.sort_by(|a, b| b.entry_id.cmp(&a.entry_id));
        }
        ExtensionRegistrySort::StatusFirst => {
            entries.sort_by(|a, b| {
                extension_status_rank(&a.status)
                    .cmp(&extension_status_rank(&b.status))
                    .then_with(|| a.entry_id.cmp(&b.entry_id))
            });
        }
        ExtensionRegistrySort::FamilyToolPatch => {
            entries.sort_by(|a, b| {
                a.family_id
                    .as_deref()
                    .unwrap_or("")
                    .cmp(b.family_id.as_deref().unwrap_or(""))
                    .then_with(|| {
                        a.tool_kind
                            .as_deref()
                            .unwrap_or("")
                            .cmp(b.tool_kind.as_deref().unwrap_or(""))
                    })
                    .then_with(|| {
                        a.patch_kind
                            .as_deref()
                            .unwrap_or("")
                            .cmp(b.patch_kind.as_deref().unwrap_or(""))
                    })
                    .then_with(|| a.entry_id.cmp(&b.entry_id))
            });
        }
        ExtensionRegistrySort::ProofRiskFirst => {
            entries.sort_by(|a, b| {
                let a_baseline_rank = match a.extension_kind.as_str() {
                    "composition_bundle" => match a
                        .composition_change_since_last_live_status
                        .as_deref()
                        .unwrap_or("unknown")
                    {
                        "regressed_since_last_live" => 0,
                        "changed_since_last_live" => 1,
                        "no_live_baseline" => 2,
                        "baseline_recorded" => 3,
                        "stable_since_last_live" => 4,
                        _ => 5,
                    },
                    "patch_recipe" => match a
                        .patch_change_since_last_live_status
                        .as_deref()
                        .unwrap_or("unknown")
                    {
                        "regressed_since_last_live" => 0,
                        "changed_since_last_live" => 1,
                        "no_live_baseline" => 2,
                        "baseline_recorded" => 3,
                        "stable_since_last_live" => 4,
                        _ => 5,
                    },
                    "helper_lane" => match a
                        .helper_change_since_last_live_status
                        .as_deref()
                        .unwrap_or("unknown")
                    {
                        "regressed_since_last_live" => 0,
                        "changed_since_last_live" => 1,
                        "no_live_baseline" => 2,
                        "baseline_recorded" => 3,
                        "stable_since_last_live" => 4,
                        _ => 5,
                    },
                    "chattycog_bridge_lane" => match a
                        .bridge_change_since_last_live_status
                        .as_deref()
                        .unwrap_or("unknown")
                    {
                        "regressed_since_last_live" => 0,
                        "changed_since_last_live" => 1,
                        "no_live_baseline" => 2,
                        "baseline_recorded" => 3,
                        "stable_since_last_live" => 4,
                        _ => 5,
                    },
                    _ => proof_baseline_risk_rank(a),
                };
                let b_baseline_rank = match b.extension_kind.as_str() {
                    "composition_bundle" => match b
                        .composition_change_since_last_live_status
                        .as_deref()
                        .unwrap_or("unknown")
                    {
                        "regressed_since_last_live" => 0,
                        "changed_since_last_live" => 1,
                        "no_live_baseline" => 2,
                        "baseline_recorded" => 3,
                        "stable_since_last_live" => 4,
                        _ => 5,
                    },
                    "patch_recipe" => match b
                        .patch_change_since_last_live_status
                        .as_deref()
                        .unwrap_or("unknown")
                    {
                        "regressed_since_last_live" => 0,
                        "changed_since_last_live" => 1,
                        "no_live_baseline" => 2,
                        "baseline_recorded" => 3,
                        "stable_since_last_live" => 4,
                        _ => 5,
                    },
                    "helper_lane" => match b
                        .helper_change_since_last_live_status
                        .as_deref()
                        .unwrap_or("unknown")
                    {
                        "regressed_since_last_live" => 0,
                        "changed_since_last_live" => 1,
                        "no_live_baseline" => 2,
                        "baseline_recorded" => 3,
                        "stable_since_last_live" => 4,
                        _ => 5,
                    },
                    "chattycog_bridge_lane" => match b
                        .bridge_change_since_last_live_status
                        .as_deref()
                        .unwrap_or("unknown")
                    {
                        "regressed_since_last_live" => 0,
                        "changed_since_last_live" => 1,
                        "no_live_baseline" => 2,
                        "baseline_recorded" => 3,
                        "stable_since_last_live" => 4,
                        _ => 5,
                    },
                    _ => proof_baseline_risk_rank(b),
                };
                let a_drift_rank = match a.extension_kind.as_str() {
                    "composition_bundle" => {
                        match a.composition_drift_status.as_deref().unwrap_or("unknown") {
                            "drifted_risky" => 0,
                            "structurally_customized" => 1,
                            "seed_aligned" => 3,
                            "unseeded" => 4,
                            _ => 5,
                        }
                    }
                    "patch_recipe" => match a.patch_drift_status.as_deref().unwrap_or("unknown") {
                        "drifted_risky" => 0,
                        "structurally_customized" => 1,
                        "lightly_customized" => 2,
                        "seed_aligned" => 3,
                        "unseeded" => 4,
                        _ => 5,
                    },
                    "helper_lane" => match a.helper_drift_status.as_deref().unwrap_or("unknown") {
                        "drifted_risky" => 0,
                        "structurally_customized" => 1,
                        "lightly_customized" => 2,
                        "seed_aligned" => 3,
                        "unseeded" => 4,
                        _ => 5,
                    },
                    "chattycog_bridge_lane" => {
                        match a.bridge_drift_status.as_deref().unwrap_or("unknown") {
                            "drifted_risky" => 0,
                            "structurally_customized" => 1,
                            "lightly_customized" => 2,
                            "seed_aligned" => 3,
                            "unseeded" => 4,
                            _ => 5,
                        }
                    }
                    _ => proof_drift_risk_rank(a),
                };
                let b_drift_rank = match b.extension_kind.as_str() {
                    "composition_bundle" => {
                        match b.composition_drift_status.as_deref().unwrap_or("unknown") {
                            "drifted_risky" => 0,
                            "structurally_customized" => 1,
                            "seed_aligned" => 3,
                            "unseeded" => 4,
                            _ => 5,
                        }
                    }
                    "patch_recipe" => match b.patch_drift_status.as_deref().unwrap_or("unknown") {
                        "drifted_risky" => 0,
                        "structurally_customized" => 1,
                        "lightly_customized" => 2,
                        "seed_aligned" => 3,
                        "unseeded" => 4,
                        _ => 5,
                    },
                    "helper_lane" => match b.helper_drift_status.as_deref().unwrap_or("unknown") {
                        "drifted_risky" => 0,
                        "structurally_customized" => 1,
                        "lightly_customized" => 2,
                        "seed_aligned" => 3,
                        "unseeded" => 4,
                        _ => 5,
                    },
                    "chattycog_bridge_lane" => {
                        match b.bridge_drift_status.as_deref().unwrap_or("unknown") {
                            "drifted_risky" => 0,
                            "structurally_customized" => 1,
                            "lightly_customized" => 2,
                            "seed_aligned" => 3,
                            "unseeded" => 4,
                            _ => 5,
                        }
                    }
                    _ => proof_drift_risk_rank(b),
                };
                a_baseline_rank
                    .cmp(&b_baseline_rank)
                    .then_with(|| a_drift_rank.cmp(&b_drift_rank))
                    .then_with(|| {
                        extension_status_rank(&a.status).cmp(&extension_status_rank(&b.status))
                    })
                    .then_with(|| b.entry_id.cmp(&a.entry_id))
            });
        }
    }
}

fn extension_status_blockers(entry: &PendingExtensionEntry) -> Vec<String> {
    match entry.status.as_str() {
        "pending_implementation" => vec![
            "Implementation stub exists, but the lane logic is not marked implemented yet."
                .to_string(),
            "Next step: implement the lane scaffold before validation.".to_string(),
        ],
        "implemented" => vec![
            "Implementation is marked in progress, but scaffold files have not been validated."
                .to_string(),
            "Next step: validate the scaffold and integrated files.".to_string(),
        ],
        "validated_ready" => vec![
            "Lane files are validated, but promotion artifacts have not been prepared."
                .to_string(),
            "Next step: prepare promotion artifacts.".to_string(),
        ],
        "promotion_prepared" => vec![
            "Promotion artifacts exist, but apply-patch templates have not been prepared."
                .to_string(),
            "Next step: prepare apply-patch templates.".to_string(),
        ],
        "apply_patch_ready" => vec![
            "Apply-patch templates are ready, but the host has not wired them into the live Rust registry."
                .to_string(),
            "Next step: run host wiring.".to_string(),
        ],
        "host_wired" => vec![
            "The host inserted compile-safe wiring, but the lane is not yet validated as fully live."
                .to_string(),
            "Next step: replace any placeholders and validate live.".to_string(),
        ],
        "archived" => vec![
            "This lane is archived and no longer considered active work.".to_string(),
        ],
        "fully_live" => vec!["No blockers. This lane is shipped.".to_string()],
        other => vec![format!("Unknown lane status `{other}`. Inspect scaffold and notes.")],
    }
}

fn load_text_preview(path: &Path, max_chars: usize) -> Option<String> {
    let contents = std::fs::read_to_string(&path).ok()?;
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        return Some("(empty file)".to_string());
    }
    let mut preview = trimmed.chars().take(max_chars).collect::<String>();
    if trimmed.chars().count() > max_chars {
        preview.push_str("\n\n...truncated...");
    }
    Some(preview)
}

fn compare_mismatch_hints(left: &str, right: &str) -> Vec<String> {
    let mut hints = Vec::new();
    for marker in [
        "todo phrase",
        "todo_feature",
        "not wired yet",
        "implement handler logic here",
        "replace this stub with deterministic patch logic",
    ] {
        if left.contains(marker) && !right.contains(marker) {
            hints.push(format!(
                "Left side still contains placeholder marker `{marker}`."
            ));
        }
    }

    for expected in ["recipe_id", "patch_kind", "handler:", "provides_features"] {
        if left.contains(expected) && !right.contains(expected) {
            hints.push(format!(
                "Right side does not visibly include expected token `{expected}` from the left side."
            ));
        }
    }

    if left == right {
        hints.push("The previewed content matches exactly.".to_string());
    }

    hints
}

fn lane_readiness_tone(
    status: &str,
    blocker_count: usize,
    mismatch_hint_count: usize,
) -> (&'static str, egui::Color32) {
    if status == "fully_live" && blocker_count == 0 && mismatch_hint_count == 0 {
        ("Ready", egui::Color32::from_rgb(84, 168, 108))
    } else if status == "archived" {
        ("Archived", egui::Color32::from_rgb(140, 140, 140))
    } else if mismatch_hint_count > 0 || status == "host_wired" || status == "apply_patch_ready" {
        ("Needs review", egui::Color32::from_rgb(214, 170, 72))
    } else {
        ("In progress", egui::Color32::from_rgb(96, 156, 214))
    }
}

impl App for ChattyFactoryUiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.poll_task_results();
        self.prune_toasts();

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("ChattyFactory");
                ui.separator();
                let status_fill = if self.task_running {
                    egui::Color32::from_rgb(66, 74, 32)
                } else {
                    egui::Color32::from_rgb(32, 54, 68)
                };
                egui::Frame::none()
                    .fill(status_fill)
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(70)))
                    .rounding(6.0)
                    .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(&self.status_line)
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                    });
                if self.task_running {
                    ui.spinner();
                }
            });
        });

        egui::Area::new("toast_area".into())
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-16.0, 16.0))
            .show(ctx, |ui| {
                ui.set_width(320.0);
                for toast in &self.toasts {
                    let fill = match toast.kind {
                        ToastKind::Success => egui::Color32::from_rgb(28, 74, 44),
                        ToastKind::Error => egui::Color32::from_rgb(96, 34, 34),
                    };
                    egui::Frame::none()
                        .fill(fill)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_black_alpha(40)))
                        .rounding(6.0)
                        .inner_margin(egui::Margin::same(10.0))
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(&toast.message).color(egui::Color32::WHITE),
                            );
                        });
                    ui.add_space(6.0);
                }
            });

        egui::SidePanel::left("project_list")
            .resizable(true)
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.heading("Projects");
                ui.horizontal(|ui| {
                    if ui.button("Refresh").clicked() {
                        self.spawn_task(UiTask::RefreshBrowser);
                    }
                    if ui.button("Refresh Runtime").clicked() {
                        self.spawn_task(UiTask::RefreshRuntime);
                    }
                    if ui.button("Clear Selection").clicked() {
                        self.spawn_task(UiTask::ClearSelectedProject);
                    }
                });
                let blocked_filter_changed = ui
                    .checkbox(
                        &mut self.project_browser_show_blocked_lanes_only,
                        "Show projects with blocked lanes only",
                    )
                    .changed();
                let regressed_filter_changed = ui
                    .checkbox(
                        &mut self.project_browser_show_regressed_only,
                        "Show regressed projects only",
                    )
                    .changed();
                let improved_filter_changed = ui
                    .checkbox(
                        &mut self.project_browser_show_improved_only,
                        "Show improved projects only",
                    )
                    .changed();
                let historical_filter_changed = ui
                    .checkbox(
                        &mut self.project_browser_show_historical_blockers_only,
                        "Show historical blockers only",
                    )
                    .changed();
                let decomposable_historical_filter_changed = ui
                    .checkbox(
                        &mut self.project_browser_show_decomposable_historical_only,
                        "Show decomposable historical only",
                    )
                    .changed();
                let sort_changed = ui
                    .checkbox(
                        &mut self.project_browser_sort_by_patch_risk,
                        "Sort projects by patch risk",
                    )
                    .changed();
                if blocked_filter_changed
                    || regressed_filter_changed
                    || improved_filter_changed
                    || historical_filter_changed
                    || decomposable_historical_filter_changed
                    || sort_changed
                {
                    self.save_paired_proof_ui_preferences();
                }
                ui.separator();

                if let Some(state) = &self.browser_state {
                    let selected_signal = state
                        .selected_project_session
                        .as_ref()
                        .map(|session| session.project_name.clone());
                    let active_signal = state
                        .active_project_session
                        .as_ref()
                        .map(|session| session.project_name.clone());
                    let recent_override_counts =
                        load_recent_project_starter_override_counts(&self.workspace_root);
                    let all_projects = state
                        .projects
                        .iter()
                        .filter_map(|project| {
                            let spec = load_project_spec(&self.workspace_root, &project.project_name)?;
                            let readiness = summarize_project_patch_readiness(&spec);
                            let patchability_receipt = load_project_patch_readiness_receipt(
                                &self.workspace_root,
                                &project.project_name,
                            );
                            Some((project.clone(), readiness, patchability_receipt))
                        })
                        .collect::<Vec<_>>();
                    let regressed_projects = all_projects
                        .iter()
                        .filter(|(_, _, receipt)| {
                            receipt
                                .as_ref()
                                .map(|receipt| {
                                    receipt.change_since_patchability_baseline_status.as_str()
                                        == "regressed_since_patchability_baseline"
                                })
                                .unwrap_or(false)
                        })
                        .count();
                    let improved_projects = all_projects
                        .iter()
                        .filter(|(_, _, receipt)| {
                            receipt
                                .as_ref()
                                .map(|receipt| {
                                    receipt.change_since_patchability_baseline_status.as_str()
                                        == "improved_since_patchability_baseline"
                                })
                                .unwrap_or(false)
                        })
                        .count();
                    let blocked_projects = all_projects
                        .iter()
                        .filter(|(_, readiness, _)| {
                            readiness.structurally_blocked_count > 0
                                || readiness.surface_mismatch_count > 0
                                || readiness.dependency_blocked_count > 0
                        })
                        .count();
                    let risky_blocker_projects = all_projects
                        .iter()
                        .filter(|(_, _, receipt)| {
                            receipt
                                .as_ref()
                                .map(|receipt| receipt.risky_blocked_lane_count > 0)
                                .unwrap_or(false)
                        })
                        .count();
                    let historical_blocker_projects = all_projects
                        .iter()
                        .filter(|(_, _, receipt)| {
                            receipt
                                .as_ref()
                                .map(|receipt| !receipt.superseded_blocked_lane_replacements.is_empty())
                                .unwrap_or(false)
                        })
                        .count();
                    let decomposable_historical_projects = all_projects
                        .iter()
                        .filter(|(_, _, receipt)| {
                            receipt
                                .as_ref()
                                .map(|receipt| receipt.decomposable_historical_blocker_count > 0)
                                .unwrap_or(false)
                        })
                        .count();
                    let override_heavy_projects = all_projects
                        .iter()
                        .filter(|(project, _, _)| {
                            recent_override_counts
                                .get(&project.project_name)
                                .copied()
                                .unwrap_or(0)
                                > 0
                        })
                        .count();
                    let mut projects = all_projects
                        .into_iter()
                        .filter(|(_, readiness, receipt)| {
                            if self.project_browser_show_blocked_lanes_only
                                && readiness.structurally_blocked_count
                                    + readiness.surface_mismatch_count
                                    + readiness.dependency_blocked_count
                                    == 0
                            {
                                return false;
                            }
                            if self.project_browser_show_regressed_only
                                && receipt
                                    .as_ref()
                                    .map(|receipt| {
                                        receipt.change_since_patchability_baseline_status.as_str()
                                            == "regressed_since_patchability_baseline"
                                    })
                                    .unwrap_or(false)
                                    == false
                            {
                                return false;
                            }
                            if self.project_browser_show_improved_only
                                && receipt
                                    .as_ref()
                                    .map(|receipt| {
                                        receipt.change_since_patchability_baseline_status.as_str()
                                            == "improved_since_patchability_baseline"
                                    })
                                    .unwrap_or(false)
                                    == false
                            {
                                return false;
                            }
                            if self.project_browser_show_historical_blockers_only
                                && receipt
                                    .as_ref()
                                    .map(|receipt| {
                                        !receipt.superseded_blocked_lane_replacements.is_empty()
                                    })
                                    .unwrap_or(false)
                                    == false
                            {
                                return false;
                            }
                            if self.project_browser_show_decomposable_historical_only
                                && receipt
                                    .as_ref()
                                    .map(|receipt| {
                                        receipt.decomposable_historical_blocker_count > 0
                                    })
                                    .unwrap_or(false)
                                    == false
                            {
                                return false;
                            }
                            true
                        })
                        .collect::<Vec<_>>();
                    let refresh_clicked = render_governance_metric_strip(
                        ui,
                        &[
                            format!("Regressed projects: {regressed_projects}"),
                            format!("Improved projects: {improved_projects}"),
                            format!("Blocked projects: {blocked_projects}"),
                            format!("Risky blockers: {risky_blocker_projects}"),
                            format!("Historical blockers: {historical_blocker_projects}"),
                            format!("Decomposable historical: {decomposable_historical_projects}"),
                            format!("Override-heavy projects: {override_heavy_projects}"),
                        ],
                        "Refresh browser now",
                    );
                    if refresh_clicked {
                        self.spawn_task(UiTask::RefreshBrowser);
                    }
                    ui.separator();
                    if self.project_browser_sort_by_patch_risk {
                        projects.sort_by(|left, right| {
                            project_patchability_risk_rank(right.2.as_ref(), &right.1)
                                .cmp(&project_patchability_risk_rank(left.2.as_ref(), &left.1))
                                .then(
                                    recent_override_counts
                                        .get(&right.0.project_name)
                                        .copied()
                                        .unwrap_or(0)
                                        .cmp(
                                            &recent_override_counts
                                                .get(&left.0.project_name)
                                                .copied()
                                                .unwrap_or(0),
                                        ),
                                )
                                .then(
                                    right
                                        .1
                                        .structurally_blocked_count
                                        .cmp(&left.1.structurally_blocked_count),
                                )
                                .then(right.1.surface_mismatch_count.cmp(&left.1.surface_mismatch_count))
                                .then(right.1.dependency_blocked_count.cmp(&left.1.dependency_blocked_count))
                                .then(left.0.project_name.cmp(&right.0.project_name))
                        });
                    }

                    let footer_reserve = 96.0;
                    let max_list_height = (ui.available_height() - footer_reserve).max(140.0);
                    egui::ScrollArea::vertical()
                        .max_height(max_list_height)
                        .show(ui, |ui| {
                        for (project, readiness, patchability_receipt) in &projects {
                            let is_selected = self
                                .selected_project_name
                                .as_deref()
                                .map(|name| name == project.project_name)
                                .unwrap_or(false);
                            let family = project
                                .family_id
                                .as_ref()
                                .map(|id: &chatty_factory_core::FamilyId| id.as_str().to_string())
                                .unwrap_or_else(|| "unknown_family".to_string());
                            let family_label = family_summary_label_from_patchability_receipt(
                                &family,
                                patchability_receipt.as_ref(),
                            );
                            let tool = project.tool_kind.as_deref().unwrap_or("none");
                            let summary = project
                                .request_summary
                                .as_deref()
                                .unwrap_or("no summary recorded");
                            let mut badges = Vec::new();
                            if selected_signal.as_deref() == Some(project.project_name.as_str()) {
                                badges.push("selected");
                            }
                            if active_signal.as_deref() == Some(project.project_name.as_str()) {
                                badges.push("active");
                            }
                            let badge_text = if badges.is_empty() {
                                String::new()
                            } else {
                                format!(" [{}]", badges.join(", "))
                            };
                            let (patchability_badge, patchability_color) =
                                project_patchability_badge(patchability_receipt.as_ref(), readiness);
                            let label = format!(
                                "{}{}\n{} | {} | {}",
                                project.project_name, badge_text, family_label, tool, project.recency_hint
                            );
                            if ui.selectable_label(is_selected, label).clicked() {
                                self.selected_project_name = Some(project.project_name.clone());
                            }
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("[{patchability_badge}]"))
                                        .small()
                                        .color(patchability_color),
                                );
                                if let Some(historical_badge) =
                                    project_historical_blocker_badge(patchability_receipt.as_ref())
                                {
                                    ui.label(historical_badge);
                                }
                                if let Some(decomposable_badge) =
                                    project_decomposable_historical_badge(
                                        patchability_receipt.as_ref(),
                                    )
                                {
                                    ui.label(decomposable_badge);
                                }
                                if let Some(count) =
                                    recent_override_counts.get(&project.project_name)
                                {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "[starter-override x{count}]"
                                        ))
                                        .small()
                                        .color(egui::Color32::from_rgb(204, 134, 92))
                                        .strong(),
                                    );
                                }
                            });
                            ui.label(egui::RichText::new(summary).small().weak());
                            ui.label(
                                egui::RichText::new(format!(
                                    "Patch readiness: {} ready, {} blocked ({} risky, {} historical, {} decomposable), {} already present",
                                    readiness.ready_count,
                                    readiness.structurally_blocked_count + readiness.surface_mismatch_count,
                                    patchability_receipt
                                        .as_ref()
                                        .map(|receipt| receipt.risky_blocked_lane_count)
                                        .unwrap_or(0),
                                    patchability_receipt
                                        .as_ref()
                                        .map(|receipt| receipt.historical_blocked_lane_count)
                                        .unwrap_or(0),
                                    patchability_receipt
                                        .as_ref()
                                        .map(|receipt| receipt.decomposable_historical_blocker_count)
                                        .unwrap_or(0),
                                    readiness.already_present_count
                                ))
                                .small(),
                            );
                            if let Some(receipt) = patchability_receipt {
                                if let Some((lane, replacements)) = receipt
                                    .superseded_blocked_lane_replacements
                                    .iter()
                                    .next()
                                {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Historical blocker: {lane} -> {}",
                                            replacements.join(", ")
                                        ))
                                        .small()
                                        .color(egui::Color32::from_rgb(184, 156, 92)),
                                    );
                                }
                                if let Some((lane, bundle)) = receipt
                                    .decomposable_historical_blocker_bundles
                                    .iter()
                                    .find(|(_, bundle)| bundle.bundle_status != "not_decomposable")
                                {
                                    let bundle_summary = if !bundle.ready_replacement_patch_kinds.is_empty() {
                                        format!(
                                            "Historical blocker decomposable now: {lane} -> {} (still needed: {})",
                                            bundle.replacement_patch_kinds.join(", "),
                                            bundle.ready_replacement_patch_kinds.join(", ")
                                        )
                                    } else {
                                        format!(
                                            "Historical blocker already covered by modern bundle: {lane} -> {}",
                                            bundle.replacement_patch_kinds.join(", ")
                                        )
                                    };
                                    ui.label(
                                        egui::RichText::new(bundle_summary)
                                            .small()
                                            .color(egui::Color32::from_rgb(121, 171, 125)),
                                    );
                                }
                                if let Some(note) = receipt
                                    .change_since_patchability_baseline_notes
                                    .first()
                                {
                                    ui.label(
                                        egui::RichText::new(format!("Patchability: {note}"))
                                        .small()
                                        .weak(),
                                    );
                                } else if let Some((lane, reason)) =
                                    receipt.blocked_lane_reasons.iter().next()
                                {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Patchability: blocked by {lane} -> {reason}"
                                        ))
                                        .small()
                                        .weak(),
                                    );
                                }
                            }
                            if let Some(override_receipt) = latest_project_starter_override_receipt(
                                &self.workspace_root,
                                &project.project_name,
                            ) {
                                let chosen = override_receipt
                                    .starter_override_id
                                    .as_deref()
                                    .map(build_starter_label)
                                    .unwrap_or("Auto");
                                let recommended = override_receipt
                                    .recommended_starter_id
                                    .as_deref()
                                    .map(build_starter_label)
                                    .unwrap_or("none");
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Starter override: chose {chosen} instead of {recommended}"
                                    ))
                                    .small()
                                    .color(egui::Color32::from_rgb(204, 134, 92)),
                                );
                            }
                            if let Some(reason) = &readiness.first_blocked_reason {
                                ui.label(egui::RichText::new(format!("Why: {reason}")).small().weak());
                            }
                            ui.horizontal(|ui| {
                                if ui.small_button("Select").clicked() {
                                    self.spawn_task(UiTask::SelectProject {
                                        project_name: project.project_name.clone(),
                                    });
                                }
                                if ui.small_button("Patch This").clicked() {
                                    self.selected_project_name = Some(project.project_name.clone());
                                }
                            });
                            ui.separator();
                        }
                        if projects.is_empty()
                            && self.project_browser_show_blocked_lanes_only
                            && self.project_browser_show_regressed_only
                            && self.project_browser_show_improved_only
                        {
                            ui.label("No projects can be both regressed and improved under the current filters.");
                        } else if projects.is_empty()
                            && self.project_browser_show_decomposable_historical_only
                            && self.project_browser_show_regressed_only
                            && self.project_browser_show_improved_only
                        {
                            ui.label("No projects can be both regressed and improved within the decomposable-historical filter.");
                        } else if projects.is_empty()
                            && self.project_browser_show_historical_blockers_only
                            && self.project_browser_show_regressed_only
                            && self.project_browser_show_improved_only
                        {
                            ui.label("No projects can be both regressed and improved within the historical-blocker filter.");
                        } else if projects.is_empty()
                            && self.project_browser_show_decomposable_historical_only
                            && self.project_browser_show_blocked_lanes_only
                        {
                            ui.label("No projects currently expose blocked lanes that are also classified as decomposable historical blockers.");
                        } else if projects.is_empty()
                            && self.project_browser_show_historical_blockers_only
                            && self.project_browser_show_blocked_lanes_only
                        {
                            ui.label("No projects currently expose blocked lanes that are also classified as historical blockers.");
                        } else if projects.is_empty()
                            && self.project_browser_show_decomposable_historical_only
                            && self.project_browser_show_regressed_only
                        {
                            ui.label("No regressed projects currently expose decomposable historical blockers.");
                        } else if projects.is_empty()
                            && self.project_browser_show_historical_blockers_only
                            && self.project_browser_show_regressed_only
                        {
                            ui.label("No regressed projects currently expose historical blockers.");
                        } else if projects.is_empty()
                            && self.project_browser_show_decomposable_historical_only
                            && self.project_browser_show_improved_only
                        {
                            ui.label("No improved projects currently expose decomposable historical blockers.");
                        } else if projects.is_empty()
                            && self.project_browser_show_historical_blockers_only
                            && self.project_browser_show_improved_only
                        {
                            ui.label("No improved projects currently expose historical blockers.");
                        } else if projects.is_empty()
                            && self.project_browser_show_blocked_lanes_only
                            && self.project_browser_show_regressed_only
                        {
                            ui.label("No regressed projects currently expose structurally blocked patch lanes.");
                        } else if projects.is_empty()
                            && self.project_browser_show_blocked_lanes_only
                            && self.project_browser_show_improved_only
                        {
                            ui.label("No improved projects currently expose blocked patch lanes.");
                        } else if projects.is_empty()
                            && self.project_browser_show_regressed_only
                            && self.project_browser_show_improved_only
                        {
                            ui.label("No projects match both the regressed and improved filters.");
                        } else if projects.is_empty() && self.project_browser_show_regressed_only {
                            ui.label("No regressed projects match the current filter.");
                        } else if projects.is_empty() && self.project_browser_show_improved_only {
                            ui.label("No improved projects match the current filter.");
                        } else if projects.is_empty()
                            && self.project_browser_show_decomposable_historical_only
                        {
                            ui.label("No projects currently expose decomposable historical blockers.");
                        } else if projects.is_empty()
                            && self.project_browser_show_historical_blockers_only
                        {
                            ui.label("No projects currently expose historical blockers with modern replacements.");
                        } else if projects.is_empty() && self.project_browser_show_blocked_lanes_only {
                            ui.label("No projects currently expose structurally blocked patch lanes.");
                        }
                    });
                    ui.separator();
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(egui::RichText::new("Project Browser Actions").strong());
                        ui.label(
                            egui::RichText::new(format!(
                                "Selected project: {}",
                                self.selected_project().unwrap_or("none")
                            ))
                            .small()
                            .weak(),
                        );
                        ui.horizontal_wrapped(|ui| {
                            if ui.small_button("Refresh project list").clicked() {
                                self.spawn_task(UiTask::RefreshBrowser);
                            }
                            if ui.small_button("Refresh patchability").clicked() {
                                self.spawn_task(UiTask::RefreshProjectPatchReadiness);
                            }
                        });
                    });
                } else {
                    ui.label("No browser state loaded yet.");
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_source("main_workspace_scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.columns(2, |columns| {
                        columns[0].group(|ui| {
                    ui.heading("Project Details");
                    let selected_project_name = self.selected_project().map(|name| name.to_string());
                    let mut refresh_project_patchability_clicked = false;
                    if let Some(project_name) = selected_project_name.as_deref() {
                        ui.label(format!("Target: {project_name}"));
                    } else {
                        ui.label("Target: none");
                    }
                    if let Some(spec) = &self.selected_project_spec {
                        ui.label(format!(
                            "Family: {}",
                            spec.family_id
                                .as_ref()
                                .map(|id| family_summary_label(id.as_str()))
                                .unwrap_or_else(|| family_summary_label("unknown_family"))
                        ));
                        ui.label(format!(
                            "Tool: {}",
                            spec.tool_kind.as_deref().unwrap_or("none")
                        ));
                        ui.label(format!("Substrate: {}", spec.substrate));
                        if let Some(mode) = &spec.chattycog_hosting_mode {
                            ui.label(format!("Hosting mode: {mode}"));
                        }
                        if let Some(owner) = &spec.chattycog_ui_owner {
                            ui.label(format!("UI owner: {owner}"));
                        }
                        if let Some(summary) = &spec.request_summary {
                            ui.label(summary);
                        }
                        if let Some(project_name) = selected_project_name.as_deref() {
                            if let Some(receipt) = load_project_patch_readiness_receipt(
                                &self.workspace_root,
                                project_name,
                            ) {
                                ui.separator();
                                ui.label("Project Patchability");
                                let (patchability_badge, patchability_color) =
                                    project_patchability_badge(
                                        Some(&receipt),
                                        &summarize_project_patch_readiness(spec),
                                    );
                                let patchability_family_label =
                                    family_summary_label_from_patchability_receipt(
                                        spec.family_id
                                            .as_ref()
                                            .map(|id| id.as_str())
                                            .unwrap_or("unknown_family"),
                                        Some(&receipt),
                                    );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "[{patchability_badge}] {}",
                                        receipt.change_since_patchability_baseline_status
                                    ))
                                    .color(patchability_color),
                                );
                                ui.label(format!("Governed family: {patchability_family_label}"));
                                ui.label(format!(
                                    "Blocked lanes: {} risky, {} historical, {} decomposable",
                                    receipt.risky_blocked_lane_count,
                                    receipt.historical_blocked_lane_count,
                                    receipt.decomposable_historical_blocker_count
                                ));
                                if let Some(note) =
                                    receipt.change_since_patchability_baseline_notes.first()
                                {
                                    ui.label(format!("Baseline note: {note}"));
                                }
                                let freshness_line = if let Some(status) =
                                    self.project_patch_readiness_refresh_status.as_ref()
                                {
                                    governance_refresh_status_summary(
                                        "project patch readiness",
                                        &status.updated_at,
                                        status.refreshed_at_label.as_deref(),
                                        &status.status_id,
                                        status.refreshed_entries,
                                        status.skipped_entries,
                                    )
                                } else {
                                    governance_never_refreshed_summary("project patch readiness")
                                };
                                ui.label(egui::RichText::new(freshness_line).small().weak());
                                if ui.small_button("Refresh project patchability now").clicked() {
                                    refresh_project_patchability_clicked = true;
                                }
                                let freshness_warning = if let Some(status) =
                                    self.project_patch_readiness_refresh_status.as_ref()
                                {
                                    if status.age_minutes.unwrap_or(0) >= 60 {
                                        Some(governance_stale_warning(
                                            "Project patch readiness",
                                            status.age_minutes.unwrap_or(0),
                                        ))
                                    } else {
                                        None
                                    }
                                } else {
                                    Some(governance_never_refreshed_warning(
                                        "Project patch readiness",
                                    ))
                                };
                                if let Some(warning) = freshness_warning {
                                    ui.label(
                                        egui::RichText::new(warning)
                                            .small()
                                            .color(egui::Color32::from_rgb(214, 170, 72)),
                                    );
                                }
                                egui::CollapsingHeader::new("Patchability Deep View")
                                    .default_open(false)
                                    .show(ui, |ui| {
                                        if !receipt.blocked_lane_reasons.is_empty() {
                                            ui.label("Blocked lane reasons");
                                            for (patch_kind, reason) in
                                                receipt.blocked_lane_reasons.iter().take(6)
                                            {
                                                ui.label(format!("- {patch_kind}: {reason}"));
                                            }
                                        }
                                        if !receipt.superseded_blocked_lane_replacements.is_empty() {
                                            ui.label("Historical blockers with modern replacements");
                                            for (patch_kind, replacements) in receipt
                                                .superseded_blocked_lane_replacements
                                                .iter()
                                                .take(6)
                                            {
                                                ui.label(format!(
                                                    "- {patch_kind} -> {}",
                                                    replacements.join(", ")
                                                ));
                                            }
                                        }
                                        if !receipt.decomposable_historical_blocker_bundles.is_empty()
                                        {
                                            ui.label(
                                                "Historical blockers decomposable into modern bundles",
                                            );
                                            for (patch_kind, bundle) in receipt
                                                .decomposable_historical_blocker_bundles
                                                .iter()
                                                .filter(|(_, bundle)| {
                                                    bundle.bundle_status != "not_decomposable"
                                                })
                                                .take(6)
                                            {
                                                ui.label(format!(
                                                    "- {patch_kind} -> {}",
                                                    bundle.replacement_patch_kinds.join(", ")
                                                ));
                                                if !bundle.ready_replacement_patch_kinds.is_empty() {
                                                    ui.label(format!(
                                                        "  still needed now: {}",
                                                        bundle
                                                            .ready_replacement_patch_kinds
                                                            .join(", ")
                                                    ));
                                                }
                                                if !bundle
                                                    .already_present_replacement_patch_kinds
                                                    .is_empty()
                                                {
                                                    ui.label(format!(
                                                        "  already present: {}",
                                                        bundle
                                                            .already_present_replacement_patch_kinds
                                                            .join(", ")
                                                    ));
                                                }
                                                ui.label(format!(
                                                    "  bundle status: {}",
                                                    bundle.bundle_status
                                                ));
                                            }
                                        }
                                    });
                            }
                        }
                        if let Some(bridge) = &spec.chattycog_bridge_capabilities {
                            ui.separator();
                            ui.label(format!(
                                "Bridge summary: status={} log_sources={} shared_room_state={}",
                                bridge.status_enabled,
                                bridge.log_sources_enabled,
                                bridge.shared_room_state_enabled
                            ));
                        }
                        ui.separator();
                        ui.label(format!("Feature count: {}", spec.features.len()));
                        ui.label(format!(
                            "Patch lane count: {}",
                            if !spec.patch_lanes.is_empty() {
                                spec.patch_lanes.len()
                            } else {
                                spec.supported_patch_kinds.len()
                            }
                        ));
                        ui.label(format!(
                            "Acceptance recipe count: {}",
                            spec.acceptance_recipes.len()
                        ));
                        ui.label(format!(
                            "Operator bundle count: {}",
                            spec.operator_bundles.len()
                        ));
                        egui::CollapsingHeader::new("Project Surface Deep View")
                            .default_open(false)
                            .show(ui, |ui| {
                                if let Some(bridge) = &spec.chattycog_bridge_capabilities {
                                    ui.label("Bridge Capabilities");
                                    ui.label(format!("Status: {}", bridge.status_enabled));
                                    ui.label(format!("Log sources: {}", bridge.log_sources_enabled));
                                    ui.label(format!(
                                        "Shared room state: {}",
                                        bridge.shared_room_state_enabled
                                    ));
                                }
                                ui.separator();
                                ui.label("Features");
                                for feature in spec.features.iter().take(6) {
                                    ui.label(format!("- {feature}"));
                                }
                                ui.separator();
                                ui.label("Patch Lanes");
                                if !spec.patch_lanes.is_empty() {
                                    for lane in &spec.patch_lanes {
                                        ui.label(format!(
                                            "- {} [{} | {} | {}]",
                                            lane.patch_kind,
                                            lane.availability_status,
                                            lane.surgical_maturity,
                                            lane.effective_preflight_readiness
                                        ));
                                        if lane.effective_preflight_readiness != "ready" {
                                            ui.label(format!(
                                                "  reason: {}",
                                                lane.preflight_readiness_reason
                                            ));
                                        }
                                        if let Some(superseded_by) =
                                            patch_lane_superseded_summary(lane)
                                        {
                                            ui.label(format!("  use instead: {superseded_by}"));
                                        }
                                    }
                                } else {
                                    for patch_kind in &spec.supported_patch_kinds {
                                        ui.label(format!("- {patch_kind}"));
                                    }
                                }
                                if !spec.acceptance_recipes.is_empty() {
                                    ui.separator();
                                    ui.label("Acceptance Recipes");
                                    for recipe in &spec.acceptance_recipes {
                                        ui.label(format!(
                                            "- {} [{}]",
                                            recipe.recipe_id, recipe.availability_status
                                        ));
                                    }
                                }
                                if !spec.operator_bundles.is_empty() {
                                    ui.separator();
                                    ui.label("Operator Bundles");
                                    for bundle in &spec.operator_bundles {
                                        ui.label(format!(
                                            "- {} [{}]",
                                            bundle.bundle_id, bundle.availability_status
                                        ));
                                    }
                                }
                            });
                        if let Some(project_name) = selected_project_name.as_deref() {
                            self.render_recent_project_patch_xrays_section(ui, project_name);
                        }
                    } else {
                        ui.label("Select a project to inspect its ProjectSpec.");
                    }
                    if refresh_project_patchability_clicked {
                        self.spawn_task(UiTask::RefreshProjectPatchReadiness);
                    }
                        });

                        columns[1].group(|ui| {
                            self.render_runtime_registry_dashboard(ui);
                        });
                    });

                    ui.separator();
                    ui.heading("Last Result");
                    if self.last_action_summary.title.is_empty() {
                ui.label("No command summary yet.");
            } else {
                ui.label(egui::RichText::new(&self.last_action_summary.title).strong());
                for line in &self.last_action_summary.lines {
                    ui.label(line);
                }
                if let Some(result) = self.last_execution_result.clone() {
                    ui.separator();
                    ui.label(format!("Kind: {}", result.kind));
                    ui.label(format!("Project: {}", result.project_name));
                    if let Some(starter_id) = &result.starter_override_id {
                        ui.label(format!("Starter override: {starter_id}"));
                    }
                    if let Some(summary) = &result.starter_override_summary {
                        ui.label(format!("Starter note: {summary}"));
                    }
                    if let Some(starter_id) = &result.recommended_starter_id {
                        ui.label(format!("Normal routing starter: {starter_id}"));
                    }
                    if let Some(summary) = &result.recommended_starter_summary {
                        ui.label(format!("Starter recommendation: {summary}"));
                    }
                    if let Some(comparison) = &result.starter_recommendation_comparison {
                        ui.label(format!("Starter comparison: {comparison}"));
                    }
                    ui.label(format!(
                        "Family: {}",
                        family_summary_label(
                            result.family_id.as_deref().unwrap_or("unknown_family")
                        )
                    ));
                    if let Some(tool_kind) = &result.tool_kind {
                        ui.label(format!("Tool: {tool_kind}"));
                    }
                    if let Some(patch_kind) = &result.patch_kind {
                        ui.label(format!("Patch kind: {patch_kind}"));
                    }
                    if let Some(request_mode) = &result.followup_request_mode {
                        ui.label(format!("Follow-up route: {request_mode}"));
                    }
                    if let Some(mode) = &result.chattycog_hosting_mode {
                        ui.label(format!("Hosting mode: {mode}"));
                    }
                    if let Some(owner) = &result.chattycog_ui_owner {
                        ui.label(format!("UI owner: {owner}"));
                    }
                    ui.label(format!(
                        "Plan confidence: {} ({})",
                        result.plan_confidence_score, result.plan_confidence_band
                    ));
                    ui.label(format!("LLM review required: {}", result.needs_llm_review));
                    if let Some(status) = &result.acceptance_status {
                        ui.label(format!("Acceptance: {status}"));
                    }
                    ui.label(format!("Request id: {}", result.request_id));
                    ui.label(format!("Route note count: {}", result.route_notes.len()));
                    ui.label(format!(
                        "Follow-up rationale count: {}",
                        result.followup_rationale.len()
                    ));
                    ui.label(format!("Patch lane count: {}", result.patch_lanes.len()));
                    ui.label(format!(
                        "Acceptance recipe count: {}",
                        result.acceptance_recipes.len()
                    ));
                    ui.label(format!(
                        "Operator bundle count: {}",
                        result.operator_bundles.len()
                    ));
                    if let Some(bridge) = &result.chattycog_bridge_capabilities {
                        ui.label(format!(
                            "Bridge summary: status={} log_sources={} shared_room_state={}",
                            bridge.status_enabled,
                            bridge.log_sources_enabled,
                            bridge.shared_room_state_enabled
                        ));
                    }
                    egui::CollapsingHeader::new("Execution Deep View")
                        .default_open(false)
                        .show(ui, |ui| {
                            if !result.route_notes.is_empty() {
                                ui.label("Route notes");
                                for note in result.route_notes.iter().take(4) {
                                    ui.label(format!("- {note}"));
                                }
                            }
                            if !result.followup_rationale.is_empty() {
                                ui.label("Follow-up rationale");
                                for note in result.followup_rationale.iter().take(4) {
                                    ui.label(format!("- {note}"));
                                }
                            }
                            if !result.patch_lanes.is_empty() {
                                ui.label("Patch lanes");
                                for lane in result.patch_lanes.iter().take(6) {
                                    ui.label(format!(
                                        "- {} [{} | {} | {}]",
                                        lane.patch_kind,
                                        lane.availability_status,
                                        lane.surgical_maturity,
                                        lane.effective_preflight_readiness
                                    ));
                                    if lane.effective_preflight_readiness != "ready" {
                                        ui.label(format!(
                                            "  reason: {}",
                                            lane.preflight_readiness_reason
                                        ));
                                    }
                                    if let Some(superseded_by) =
                                        patch_lane_superseded_summary(lane)
                                    {
                                        ui.label(format!("  use instead: {superseded_by}"));
                                    }
                                }
                            }
                            if !result.acceptance_recipes.is_empty() {
                                ui.label("Acceptance recipes");
                                for recipe in result.acceptance_recipes.iter().take(6) {
                                    ui.label(format!(
                                        "- {} [{}]",
                                        recipe.recipe_id, recipe.availability_status
                                    ));
                                }
                            }
                            if !result.operator_bundles.is_empty() {
                                ui.label("Operator bundles");
                                for bundle in result.operator_bundles.iter().take(6) {
                                    ui.label(format!(
                                        "- {} [{}]",
                                        bundle.bundle_id, bundle.availability_status
                                    ));
                                }
                            }
                            if let Some(bridge) = &result.chattycog_bridge_capabilities {
                                ui.label("Bridge capabilities");
                                ui.label(format!("- status: {}", bridge.status_enabled));
                                ui.label(format!("- log_sources: {}", bridge.log_sources_enabled));
                                ui.label(format!(
                                    "- shared_room_state: {}",
                                    bridge.shared_room_state_enabled
                                ));
                            }
                        });
                    self.render_last_result_patch_xray_section(ui, &result);
                    egui::CollapsingHeader::new("Result Files")
                        .default_open(false)
                        .show(ui, |ui| {
                            for path in result.file_paths.iter().take(8) {
                                ui.label(format!("- {path}"));
                            }
                        });
                } else if let Some(fallback) = &self.last_fallback_result {
                    ui.separator();
                    ui.label("Fallback required");
                    ui.label(format!("Request id: {}", fallback.request_id));
                    if let Some(mode) = &fallback.mode {
                        ui.label(format!("Mode: {mode}"));
                    }
                    ui.label(format!("Goal: {}", fallback.interpreted_goal));
                    ui.label(format!("Question: {}", fallback.question));
                    ui.label(format!(
                        "Extension target: {}",
                        fallback.suggested_extension_kind
                    ));
                    ui.label(format!(
                        "Next step: {}",
                        fallback.recommended_next_step
                    ));
                    if let Some(failure_class) = &fallback.build_failure_class {
                        ui.label(format!("Build failure class: {failure_class}"));
                    }
                    if let Some(failure_mode) = &fallback.build_failure_mode {
                        ui.label(format!("Build failure mode: {failure_mode}"));
                    }
                    if let Some(family_id) = &fallback.suggested_family_id {
                        ui.label(format!(
                            "Suggested family: {}",
                            family_summary_label(family_id)
                        ));
                    }
                    if let Some(tool_kind) = &fallback.suggested_tool_kind {
                        ui.label(format!("Suggested tool kind: {tool_kind}"));
                    }
                    if let Some(patch_kind) = &fallback.suggested_patch_kind {
                        ui.label(format!("Suggested patch kind: {patch_kind}"));
                    }
                    if let Some(mode) = &fallback.suggested_hosting_mode {
                        ui.label(format!("Suggested hosting mode: {mode}"));
                    }
                    ui.label(format!("Reason count: {}", fallback.reasons.len()));
                    ui.label(format!(
                        "Candidate family count: {}",
                        fallback.candidate_family_ids.len()
                    ));
                    ui.label(format!(
                        "Requested capability count: {}",
                        fallback.requested_capabilities.len()
                    ));
                    ui.label(format!(
                        "Acceptance target count: {}",
                        fallback.acceptance_targets.len()
                    ));
                    ui.label(format!(
                        "Constraint count: {}",
                        fallback.constraints.len()
                    ));
                    ui.label(format!(
                        "Approved shelf match count: {}",
                        fallback.matched_approved_constraint_ids.len()
                    ));
                    egui::CollapsingHeader::new("Fallback Deep View")
                        .default_open(false)
                        .show(ui, |ui| {
                            if !fallback.reasons.is_empty() {
                                ui.label("Reasons");
                                for reason in fallback.reasons.iter().take(5) {
                                    ui.label(format!("- {reason}"));
                                }
                            }
                            if !fallback.candidate_family_ids.is_empty() {
                                ui.label("Candidate families");
                                for family_id in fallback.candidate_family_ids.iter().take(5) {
                                    ui.label(format!("- {}", family_summary_label(family_id)));
                                }
                            }
                            if !fallback.requested_capabilities.is_empty() {
                                ui.label("Requested capabilities");
                                for capability in fallback.requested_capabilities.iter().take(5) {
                                    ui.label(format!("- {capability}"));
                                }
                            }
                            if !fallback.suggested_bridge_capabilities.is_empty() {
                                ui.label("Suggested bridge capabilities");
                                for capability in fallback
                                    .suggested_bridge_capabilities
                                    .iter()
                                    .take(6)
                                {
                                    ui.label(format!("- {capability}"));
                                }
                            }
                            if !fallback.acceptance_targets.is_empty() {
                                ui.label("Acceptance targets");
                                for target in fallback.acceptance_targets.iter().take(6) {
                                    ui.label(format!("- {target}"));
                                }
                            }
                            if !fallback.suggested_artifacts.is_empty() {
                                ui.label("Suggested stub artifacts");
                                for artifact in fallback.suggested_artifacts.iter().take(6) {
                                    ui.label(format!("- {artifact}"));
                                }
                            }
                            if !fallback.implementation_notes.is_empty() {
                                ui.label("Implementation notes");
                                for note in fallback.implementation_notes.iter().take(6) {
                                    ui.label(format!("- {note}"));
                                }
                            }
                            if !fallback.pending_extension_ids.is_empty() {
                                ui.label("Pending matching lanes");
                                for entry_id in fallback.pending_extension_ids.iter().take(6) {
                                    ui.label(format!("- {entry_id}"));
                                }
                            }
                            if !fallback.pending_extension_scaffold_roots.is_empty() {
                                ui.label("Pending scaffold roots");
                                for root in fallback
                                    .pending_extension_scaffold_roots
                                    .iter()
                                    .take(4)
                                {
                                    ui.label(format!("- {}", short_path(root)));
                                }
                            }
                            if let Some(mode) = &fallback.chattycog_requested_hosting_mode {
                                ui.label(format!("Requested ChattyCog hosting mode: {mode}"));
                            }
                            if !fallback.chattycog_valid_hosting_modes.is_empty() {
                                ui.label("Valid ChattyCog hosting modes");
                                for mode in fallback.chattycog_valid_hosting_modes.iter().take(5) {
                                    ui.label(format!("- {mode}"));
                                }
                            }
                            if !fallback.chattycog_requested_bridge_capabilities.is_empty() {
                                ui.label("Requested ChattyCog bridge capabilities");
                                for capability in fallback
                                    .chattycog_requested_bridge_capabilities
                                    .iter()
                                    .take(6)
                                {
                                    ui.label(format!("- {capability}"));
                                }
                            }
                            if !fallback.chattycog_supported_bridge_capabilities.is_empty() {
                                ui.label("Currently supported ChattyCog bridge capabilities");
                                for capability in fallback
                                    .chattycog_supported_bridge_capabilities
                                    .iter()
                                    .take(6)
                                {
                                    ui.label(format!("- {capability}"));
                                }
                            }
                            if !fallback.constraints.is_empty() {
                                ui.label("Constraints");
                                for constraint in fallback.constraints.iter().take(5) {
                                    ui.label(format!("- {constraint}"));
                                }
                            }
                            if !fallback.matched_approved_constraint_ids.is_empty() {
                                ui.label("Approved shelf matches");
                                for constraint_id in fallback
                                    .matched_approved_constraint_ids
                                    .iter()
                                    .take(4)
                                {
                                    ui.label(format!("- {constraint_id}"));
                                }
                                for summary in fallback
                                    .matched_approved_constraint_summaries
                                    .iter()
                                    .take(2)
                                {
                                    ui.label(format!("- {summary}"));
                                }
                            }
                        });
                    if let Some(path) = &fallback.build_verification_path {
                        if let Some(verification) = load_build_verification_receipt(path) {
                            ui.label("Build verification");
                            ui.label(format!(
                                "- subject: {} [{} | {}]",
                                verification.review_subject,
                                verification.failure_class,
                                verification.failure_mode
                            ));
                            ui.label(format!("- decision: {}", verification.decision));
                            for constraint_id in
                                verification.matched_approved_constraint_ids.iter().take(4)
                            {
                                ui.label(format!("- approved match: {constraint_id}"));
                            }
                            for summary in verification
                                .matched_approved_constraint_summaries
                                .iter()
                                .take(2)
                            {
                                ui.label(format!("- approved summary: {summary}"));
                            }
                            for reason in verification.reasons.iter().take(2) {
                                ui.label(format!("- reason: {reason}"));
                            }
                            for method in verification.blocked_methods.iter().take(2) {
                                ui.label(format!("- blocked method: {method}"));
                            }
                            for finding in verification.findings.iter().take(3) {
                                ui.label(format!("- {finding}"));
                            }
                        }
                    }
                    if let Some(path) = &fallback.proposed_constraint_path {
                        if let Some(proposal) = load_proposed_constraint_receipt(path) {
                            ui.label("Proposed constraint");
                            ui.label(format!(
                                "- {} [{} / {} / {}]",
                                proposal.proposed_constraint.forbidden_method_summary,
                                proposal.proposed_constraint.constraint_scope,
                                proposal.proposed_constraint.constraint_kind,
                                proposal.status
                            ));
                            ui.label(format!(
                                "- severity: {}",
                                proposal.proposed_constraint.severity
                            ));
                            if let Some(summary) = &fallback.proposed_constraint_summary {
                                ui.label(format!("- summary: {summary}"));
                            }
                            if let Some(guidance) =
                                proposal.proposed_constraint.replacement_guidance.as_deref()
                            {
                                ui.label(format!("- guidance: {guidance}"));
                            }
                            for rationale in proposal.rationale.iter().take(2) {
                                ui.label(format!("- rationale: {rationale}"));
                            }
                        }
                    }
                    if let Some(path) = &fallback.stub_bundle_path {
                        ui.label(format!("Stub bundle: {}", short_path(path)));
                    }
                    if let Some(guidance) =
                        fallback.proposed_constraint_replacement_guidance.as_deref()
                    {
                        ui.label(format!("Constraint guidance: {guidance}"));
                    }
                    let fallback_request_id = fallback.request_id.clone();
                    let build_verification_path = fallback.build_verification_path.clone();
                    let proposed_constraint_path = fallback.proposed_constraint_path.clone();
                    ui.horizontal(|ui| {
                        if proposed_constraint_path.is_some()
                            && ui.small_button("Approve Proposed Constraint").clicked()
                        {
                            self.spawn_task(UiTask::ApproveProposedConstraint {
                                request_id_or_path: fallback_request_id.clone(),
                            });
                        }
                        if let Some(path) = &build_verification_path {
                            if ui.small_button("Reveal Build Verification").clicked() {
                                self.reveal_governed_artifact(
                                    path,
                                    "Revealed build verification receipt",
                                    "Open failed",
                                    "Revealed build verification receipt",
                                    "Open failed",
                                    None,
                                );
                            }
                        }
                        if let Some(path) = &proposed_constraint_path {
                            if ui.small_button("Reveal Proposed Constraint").clicked() {
                                self.reveal_governed_artifact(
                                    path,
                                    "Revealed proposed constraint receipt",
                                    "Open failed",
                                    "Revealed proposed constraint receipt",
                                    "Open failed",
                                    None,
                                );
                            }
                        }
                    });
                }
            }

                    self.render_request_action_panel(ui);

                    ui.separator();
                    egui::CollapsingHeader::new("Cross-Family Paired Proof")
                        .default_open(false)
                        .show(ui, |ui| {
                    let proof_templates = proof_templates_from_root(&self.workspace_root);
                    let comparison_bundles =
                        capability_comparison_bundles_from_root(&self.workspace_root);
                    let previous_selected_profile_name = self.selected_proof_profile_name.clone();
                    let previous_selected_template_id = self.selected_proof_template_id.clone();
                    let previous_follow_filter = self.pin_history_filter_to_selected_template;
                    let selected_proof_template = proof_templates
                        .iter()
                        .find(|template| template.template_id == self.selected_proof_template_id)
                        .cloned()
                        .or_else(|| proof_templates.first().cloned());
                    let selected_comparison_bundle = selected_proof_template
                        .as_ref()
                        .and_then(|template| {
                            comparison_bundle_for_template(template, &comparison_bundles)
                        });
                    self.render_proof_run_controls_section(
                        ui,
                        &proof_templates,
                        &selected_proof_template,
                        &selected_comparison_bundle,
                        &previous_selected_profile_name,
                        &previous_selected_template_id,
                        previous_follow_filter,
                    );
                    let paired_receipts =
                        load_cross_family_paired_proof_receipts(&self.workspace_root);
                    let filtered_paired_receipts = paired_receipts
                        .iter()
                        .filter(|(receipt, _)| {
                            proof_receipt_matches_filter(
                                receipt,
                                &self.proof_history_template_filter,
                            )
                        })
                        .cloned()
                        .collect::<Vec<_>>();
                    self.render_paired_proof_results_section(
                        ui,
                        &paired_receipts,
                        &filtered_paired_receipts,
                        &proof_templates,
                        &comparison_bundles,
                    );
                        });

                    ui.separator();
                    egui::CollapsingHeader::new("Negative Constraint Shelf")
                        .default_open(false)
                        .show(ui, |ui| {
                            if let Some(shelf) = load_approved_constraint_shelf(&self.workspace_root) {
                let shelf_history = load_constraint_shelf_history(&self.workspace_root);
                let recent_matches = load_recent_build_verification_matches(&self.workspace_root);
                let match_counts = approved_constraint_match_counts(&recent_matches);
                let match_breakdowns = approved_constraint_match_breakdowns(&recent_matches);
                let active_count = shelf
                    .constraints
                    .iter()
                    .filter(|constraint| constraint.active)
                    .count();
                let unmatched_count = shelf
                    .constraints
                    .iter()
                    .filter(|constraint| {
                        match_counts
                            .get(&constraint.constraint_id)
                            .copied()
                            .unwrap_or_default()
                            == 0
                    })
                    .count();
                let inactive_unmatched_count = shelf
                    .constraints
                    .iter()
                    .filter(|constraint| {
                        !constraint.active
                            && match_counts
                                .get(&constraint.constraint_id)
                                .copied()
                                .unwrap_or_default()
                                == 0
                    })
                    .count();
                let low_value_active_count = shelf
                    .constraints
                    .iter()
                    .filter(|constraint| {
                        constraint.active
                            && match_counts
                                .get(&constraint.constraint_id)
                                .copied()
                                .unwrap_or_default()
                                == 0
                    })
                    .count();
                ui.label(format!(
                    "Approved constraints: {} total, {} active",
                    shelf.constraints.len(),
                    active_count
                ));
                ui.label(format!(
                    "Unmatched approved rules: {} total, {} inactive, {} active",
                    unmatched_count, inactive_unmatched_count, low_value_active_count
                ));
                if let Some(history) = shelf_history.as_ref() {
                    ui.label(format!(
                        "Archived rules: {}",
                        history.archived_constraints.len()
                    ));
                }
                let recent_mutations = load_recent_constraint_shelf_mutations(&self.workspace_root);
                let approval_origins =
                    load_latest_constraint_approval_origins(&self.workspace_root);
                let recent_failure_vault_entries =
                    load_recent_failure_vault_entries(&self.workspace_root);
                let recent_triangulation_sessions =
                    load_recent_triangulation_sessions(&self.workspace_root);
                let recent_promotion_candidates =
                    load_recent_constraint_promotion_candidates(&self.workspace_root);
                let recent_floor_decisions =
                    load_recent_atomization_floor_decisions(&self.workspace_root);
                let total_match_events: usize = match_counts.values().sum();
                let triangulated_origin_count = shelf
                    .constraints
                    .iter()
                    .filter(|constraint| {
                        approval_origins
                            .get(&constraint.constraint_id)
                            .map(|receipt| receipt.proposal_origin == "triangulated_task_failure")
                            .unwrap_or(false)
                    })
                    .count();
                let build_verification_origin_count = shelf
                    .constraints
                    .iter()
                    .filter(|constraint| {
                        approval_origins
                            .get(&constraint.constraint_id)
                            .map(|receipt| receipt.proposal_origin == "build_verification_failure")
                            .unwrap_or(false)
                    })
                    .count();
                ui.label(format!(
                    "Recent match events: {} across {} approved rule(s)",
                    total_match_events,
                    match_counts.len()
                ));
                ui.label(format!(
                    "Approved rule origins: {} triangulated, {} build verification",
                    triangulated_origin_count, build_verification_origin_count
                ));
                ui.label(format!(
                    "Provisional vault evidence: {} entries across {} triangulation session(s)",
                    recent_failure_vault_entries.len(),
                    recent_triangulation_sessions.len()
                ));
                ui.label(format!(
                    "Promotion candidates waiting for review: {}",
                    recent_promotion_candidates.len()
                ));
                if !recent_mutations.is_empty() {
                    let mutation_counts = summarize_constraint_shelf_mutation_actions(&recent_mutations);
                    let activates = mutation_counts.get("activate").copied().unwrap_or_default();
                    let deactivates = mutation_counts.get("deactivate").copied().unwrap_or_default();
                    let bulk_deactivates = mutation_counts
                        .get("bulk_deactivate_low_value_active")
                        .copied()
                        .unwrap_or_default();
                    let archives = mutation_counts.get("archive").copied().unwrap_or_default();
                    let restores = mutation_counts.get("restore").copied().unwrap_or_default();
                    ui.label(format!(
                        "Recent shelf mutations: {} activate, {} deactivate, {} bulk-deactivate, {} archive, {} restore",
                        activates, deactivates, bulk_deactivates, archives, restores
                    ));
                }
                if let Some(updated_at) = &shelf.updated_at {
                    ui.label(format!("Updated: {updated_at}"));
                }
                ui.label(format!("Shelf id: {}", shelf.shelf_id));
                let shelf_path = self
                    .workspace_root
                    .join("runtime")
                    .join("approved_constraint_shelf.json");
                let history_path = self
                    .workspace_root
                    .join("runtime")
                    .join("constraint_shelf_history.json");
                ui.horizontal(|ui| {
                    if ui.small_button("Reveal Shelf").clicked() {
                        self.reveal_governed_artifact(
                            &shelf_path.display().to_string(),
                            "Revealed approved constraint shelf",
                            "Open failed",
                            "Revealed approved constraint shelf",
                            "Open failed",
                            None,
                        );
                    }
                    if shelf_history.is_some()
                        && ui.small_button("Reveal Shelf History").clicked()
                    {
                        self.reveal_governed_artifact(
                            &history_path.display().to_string(),
                            "Revealed constraint shelf history",
                            "Open failed",
                            "Revealed constraint shelf history",
                            "Open failed",
                            None,
                        );
                    }
                    if inactive_unmatched_count > 0
                        && ui.small_button("Archive inactive + unmatched").clicked()
                    {
                        self.spawn_task(UiTask::ArchiveUnmatchedInactiveConstraints);
                    }
                    if low_value_active_count > 0
                        && ui.small_button("Deactivate low-value active").clicked()
                    {
                        self.spawn_task(UiTask::DeactivateLowValueActiveConstraints);
                    }
                });
                if ui
                    .checkbox(
                        &mut self.negative_shelf_show_unmatched_only,
                        "Show unmatched approved rules only",
                    )
                    .changed()
                {
                    self.save_paired_proof_ui_preferences();
                }
                if ui
                    .checkbox(
                        &mut self.negative_shelf_show_inactive_unmatched_only,
                        "Show inactive + unmatched only",
                    )
                    .changed()
                {
                    self.save_paired_proof_ui_preferences();
                }
                if ui
                    .checkbox(
                        &mut self.negative_shelf_show_low_value_active_only,
                        "Show low-value active rules only",
                    )
                    .changed()
                {
                    self.save_paired_proof_ui_preferences();
                }
                ui.checkbox(
                    &mut self.negative_shelf_show_triangulated_origin_only,
                    "Show triangulated-origin approved only",
                );
                let mut sorted_constraints = shelf.constraints.clone();
                sorted_constraints.sort_by(|left, right| {
                    let right_matches = match_counts
                        .get(&right.constraint_id)
                        .copied()
                        .unwrap_or_default();
                    let left_matches = match_counts
                        .get(&left.constraint_id)
                        .copied()
                        .unwrap_or_default();
                    right_matches
                        .cmp(&left_matches)
                        .then_with(|| left.constraint_id.cmp(&right.constraint_id))
                });
                let filtered_constraints = sorted_constraints
                    .iter()
                    .filter(|constraint| {
                        let match_count = match_counts
                            .get(&constraint.constraint_id)
                            .copied()
                            .unwrap_or_default();
                        let unmatched_ok =
                            !self.negative_shelf_show_unmatched_only || match_count == 0;
                        let inactive_unmatched_ok =
                            !self.negative_shelf_show_inactive_unmatched_only
                                || (!constraint.active && match_count == 0);
                        let low_value_active_ok =
                            !self.negative_shelf_show_low_value_active_only
                                || (constraint.active && match_count == 0);
                        let triangulated_origin_ok =
                            !self.negative_shelf_show_triangulated_origin_only
                                || approval_origins
                                    .get(&constraint.constraint_id)
                                    .map(|receipt| {
                                        receipt.proposal_origin
                                            == "triangulated_task_failure"
                                    })
                                    .unwrap_or(false);
                        unmatched_ok
                            && inactive_unmatched_ok
                            && low_value_active_ok
                            && triangulated_origin_ok
                    })
                    .collect::<Vec<_>>();
                if filtered_constraints.is_empty() {
                    if self.negative_shelf_show_triangulated_origin_only {
                        ui.label(
                            "No approved constraints are currently visible from triangulated origin under this shelf filter.",
                        );
                    } else if self.negative_shelf_show_low_value_active_only {
                        ui.label(
                            "No approved constraints are currently both active and unmatched.",
                        );
                    } else if self.negative_shelf_show_inactive_unmatched_only {
                        ui.label(
                            "No approved constraints are currently both inactive and unmatched.",
                        );
                    } else if self.negative_shelf_show_unmatched_only {
                        ui.label(
                            "No approved constraints are currently unmatched under this shelf filter.",
                        );
                    } else {
                        ui.label("No approved constraints are visible under the current shelf filters.");
                    }
                }
                for constraint in filtered_constraints.into_iter().take(8) {
                    ui.separator();
                    let match_count = match_counts
                        .get(&constraint.constraint_id)
                        .copied()
                        .unwrap_or_default();
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{} [{} | {}]",
                            constraint.constraint_id,
                            constraint.constraint_scope,
                            constraint.constraint_kind
                        ));
                        ui.label(if constraint.active {
                            "[active]"
                        } else {
                            "[inactive]"
                        });
                        if constraint.active && match_count == 0 {
                            ui.label("[active-unmatched]");
                        } else if !constraint.active && match_count == 0 {
                            ui.label("[inactive-unmatched]");
                        } else if match_count == 0 {
                            ui.label("[unmatched]");
                        }
                        if constraint.active {
                            if ui.small_button("Deactivate").clicked() {
                                self.spawn_task(UiTask::SetApprovedConstraintActive {
                                    constraint_id: constraint.constraint_id.clone(),
                                    active: false,
                                });
                            }
                        } else if ui.small_button("Activate").clicked() {
                            self.spawn_task(UiTask::SetApprovedConstraintActive {
                                constraint_id: constraint.constraint_id.clone(),
                                active: true,
                            });
                        }
                    });
                    ui.label(format!("Method: {}", constraint.forbidden_method_summary));
                    ui.label(format!("Severity: {}", constraint.severity));
                    ui.label(format!("Recent matches: {match_count}"));
                    if let Some(breakdown) = match_breakdowns.get(&constraint.constraint_id) {
                        if !breakdown.by_failure_mode.is_empty() {
                            let failure_modes = breakdown
                                .by_failure_mode
                                .iter()
                                .map(|(mode, count)| format!("{mode}={count}"))
                                .collect::<Vec<_>>()
                                .join(", ");
                            ui.label(format!("Failure modes: {failure_modes}"));
                        }
                        if !breakdown.by_family_id.is_empty() {
                            let families = breakdown
                                .by_family_id
                                .iter()
                                .map(|(family, count)| format!("{family}={count}"))
                                .collect::<Vec<_>>()
                                .join(", ");
                            ui.label(format!("Families: {families}"));
                        }
                        if !breakdown.by_tool_kind.is_empty() {
                            let tools = breakdown
                                .by_tool_kind
                                .iter()
                                .map(|(tool, count)| format!("{tool}={count}"))
                                .collect::<Vec<_>>()
                                .join(", ");
                            ui.label(format!("Tools: {tools}"));
                        }
                    }
                    if let Some(family_id) = &constraint.family_id {
                        ui.label(format!("Family: {family_id}"));
                    }
                    if let Some(tool_kind) = &constraint.tool_kind {
                        ui.label(format!("Tool: {tool_kind}"));
                    }
                    if let Some(approval) = approval_origins.get(&constraint.constraint_id) {
                        let origin = if approval.proposal_origin.is_empty() {
                            "unknown-origin"
                        } else {
                            approval.proposal_origin.as_str()
                        };
                        ui.label(format!("Origin: {origin}"));
                        if !approval.proposal_source_id.is_empty() {
                            ui.label(format!("Origin source id: {}", approval.proposal_source_id));
                        }
                    }
                    if let Some(guidance) = constraint.replacement_guidance.as_deref() {
                        ui.label(format!("Guidance: {guidance}"));
                    }
                }
                if !recent_matches.is_empty() {
                    ui.separator();
                    ui.label("Recent Shelf Matches");
                    for (path, receipt) in recent_matches.into_iter().take(6) {
                        ui.separator();
                        ui.label(format!(
                            "{} [{} | {}]",
                            receipt.review_subject, receipt.failure_class, receipt.failure_mode
                        ));
                        for constraint_id in receipt.matched_approved_constraint_ids.iter().take(3)
                        {
                            ui.label(format!("- matched: {constraint_id}"));
                        }
                        for summary in receipt
                            .matched_approved_constraint_summaries
                            .iter()
                            .take(2)
                        {
                            ui.label(format!("- summary: {summary}"));
                        }
                        for reason in receipt.reasons.iter().take(1) {
                            ui.label(format!("- reason: {reason}"));
                        }
                        ui.horizontal(|ui| {
                            if ui.small_button("Reveal Match Receipt").clicked() {
                                self.reveal_governed_artifact(
                                    &path,
                                    "Revealed build verification receipt",
                                    "Open failed",
                                    "Revealed build verification receipt",
                                    "Open failed",
                                    None,
                                );
                            }
                        });
                    }
                }
                if !recent_mutations.is_empty() {
                    ui.separator();
                    ui.label("Recent Shelf Mutations");
                    for mutation in recent_mutations.iter().take(5) {
                        ui.separator();
                        ui.label(format!(
                            "{} [{}]",
                            mutation.constraint_id, mutation.action
                        ));
                        if let Some(origin) = mutation.proposal_origin.as_deref() {
                            ui.label(format!("Origin: {origin}"));
                        }
                        if let Some(source_id) = mutation.proposal_source_id.as_deref() {
                            ui.label(format!("Source id: {source_id}"));
                        }
                        ui.label(format!("Status: {}", mutation.status));
                        if let Some(created_at) = mutation.created_at.as_deref() {
                            ui.label(format!("When: {created_at}"));
                        }
                    }
                }
                if !recent_failure_vault_entries.is_empty() {
                    ui.separator();
                    ui.label("Provisional Failure Vault");
                    if let Some((_, decision)) = recent_floor_decisions.first() {
                        ui.label(format!(
                            "Latest floor decision: {} [{} | {}]",
                            decision.task_id,
                            decision.current_granularity,
                            decision.decision
                        ));
                        if let Some(shape) = decision.task_shape.as_deref() {
                            ui.label(format!("Shape: {shape}"));
                        }
                        if let Some(subtype) = decision.task_subtype.as_deref() {
                            ui.label(format!("Subtype: {subtype}"));
                        }
                        if !decision.alternate_methods.is_empty() {
                            ui.label(format!(
                                "Alternates: {}",
                                decision.alternate_methods.join(" | ")
                            ));
                        }
                        for finding in decision.findings.iter().take(1) {
                            ui.label(format!("- floor: {finding}"));
                        }
                    }
                    for (path, entry) in recent_failure_vault_entries.iter().take(4) {
                        ui.separator();
                        ui.label(format!(
                            "{} [{} | {}]",
                            entry.task_subtype.as_deref().unwrap_or(&entry.task_id),
                            entry.task_shape.as_deref().unwrap_or("unknown-shape"),
                            entry.failure_class
                        ));
                        ui.label(format!(
                            "Vault status: {} | depth {} | task kind {}",
                            entry.status, entry.decomposition_depth, entry.task_kind
                        ));
                        ui.label(format!(
                            "Triangulation session: {}",
                            entry.triangulation_session_id
                        ));
                        if let Some(trigger) = entry.trigger_class.as_deref() {
                            ui.label(format!("Trigger: {trigger}"));
                        }
                        for finding in entry.findings.iter().take(1) {
                            ui.label(format!("- finding: {finding}"));
                        }
                        ui.horizontal(|ui| {
                            if ui.small_button("Reveal Vault Entry").clicked() {
                                self.reveal_governed_artifact(
                                    path,
                                    "Revealed failure vault entry",
                                    "Open failed",
                                    "Revealed failure vault entry",
                                    "Open failed",
                                    None,
                                );
                            }
                            if let Some(floor_path) = entry.atomization_floor_decision_path.as_deref()
                            {
                                if ui.small_button("Reveal Floor Decision").clicked() {
                                    self.reveal_governed_artifact(
                                        floor_path,
                                        "Revealed atomization floor decision",
                                        "Open failed",
                                        "Revealed atomization floor decision",
                                        "Open failed",
                                        None,
                                    );
                                }
                            }
                            if ui.small_button("Reveal Task Attempt").clicked() {
                                self.reveal_governed_artifact(
                                    &entry.source_attempt_receipt_path,
                                    "Revealed task attempt receipt",
                                    "Open failed",
                                    "Revealed task attempt receipt",
                                    "Open failed",
                                    None,
                                );
                            }
                            if let Some(decomposition_path) =
                                entry.source_decomposition_receipt_path.as_deref()
                            {
                                if ui.small_button("Reveal Decomposition").clicked() {
                                    self.reveal_governed_artifact(
                                        decomposition_path,
                                        "Revealed decomposition receipt",
                                        "Open failed",
                                        "Revealed decomposition receipt",
                                        "Open failed",
                                        None,
                                    );
                                }
                            }
                        });
                    }
                }
                if !recent_triangulation_sessions.is_empty() {
                    ui.separator();
                    ui.label("Recent Triangulation Sessions");
                    for (path, session) in recent_triangulation_sessions.iter().take(4) {
                        ui.separator();
                        ui.label(format!(
                            "{} [{}]",
                            session
                                .task_subtype
                                .as_deref()
                                .unwrap_or(&session.task_lineage_key),
                            session.convergence_posture
                        ));
                        ui.label(format!(
                            "Session status: {} | success closed: {}",
                            session.status, session.successful_alternate_method
                        ));
                        if let Some(shape) = session.task_shape.as_deref() {
                            ui.label(format!("Shape: {shape}"));
                        }
                        ui.label(format!("Attempts tracked: {}", session.attempts.len()));
                        for attempt in session.attempts.iter().take(1) {
                            ui.label(format!(
                                "- latest: {} via {} [{}]",
                                attempt.task_id, attempt.attempt_method, attempt.outcome
                            ));
                            if let Some(subtype) = attempt.task_subtype.as_deref() {
                                ui.label(format!("  subtype: {subtype}"));
                            }
                            if let Some(failure_class) = attempt.failure_class.as_deref() {
                                ui.label(format!("  failure class: {failure_class}"));
                            }
                        }
                        for finding in session.findings.iter().take(1) {
                            ui.label(format!("- session: {finding}"));
                        }
                        ui.horizontal(|ui| {
                            if ui.small_button("Reveal Session").clicked() {
                                self.reveal_governed_artifact(
                                    path,
                                    "Revealed triangulation session",
                                    "Open failed",
                                    "Revealed triangulation session",
                                    "Open failed",
                                    None,
                                );
                            }
                            if let Some(floor_path) =
                                session.atomization_floor_decision_path.as_deref()
                            {
                                if ui.small_button("Reveal Session Floor").clicked() {
                                    self.reveal_governed_artifact(
                                        floor_path,
                                        "Revealed session floor decision",
                                        "Open failed",
                                        "Revealed session floor decision",
                                        "Open failed",
                                        None,
                                    );
                                }
                            }
                            if let Some(attempt) = session.attempts.first() {
                                if let Some(attempt_path) =
                                    attempt.source_attempt_receipt_path.as_deref()
                                {
                                    if ui.small_button("Reveal Attempt").clicked() {
                                        self.reveal_governed_artifact(
                                            attempt_path,
                                            "Revealed triangulation attempt",
                                            "Open failed",
                                            "Revealed triangulation attempt",
                                            "Open failed",
                                            None,
                                        );
                                    }
                                }
                                if let Some(decomposition_path) =
                                    attempt.source_decomposition_receipt_path.as_deref()
                                {
                                    if ui.small_button("Reveal Attempt Decomposition").clicked() {
                                        self.reveal_governed_artifact(
                                            decomposition_path,
                                            "Revealed attempt decomposition",
                                            "Open failed",
                                            "Revealed attempt decomposition",
                                            "Open failed",
                                            None,
                                        );
                                    }
                                }
                                ui.label(format!("Attempt id: {}", attempt.attempt_id));
                            }
                        });
                    }
                }
                if !recent_promotion_candidates.is_empty() {
                    ui.separator();
                    ui.label("Constraint Promotion Candidates");
                    for (path, candidate) in recent_promotion_candidates.iter().take(4) {
                        ui.separator();
                        ui.label(format!(
                            "{} [{} | {}]",
                            candidate
                                .task_subtype
                                .as_deref()
                                .unwrap_or(&candidate.candidate_id),
                            candidate
                                .task_shape
                                .as_deref()
                                .unwrap_or("unknown-shape"),
                            candidate.failure_class
                        ));
                        ui.label(format!(
                            "Confidence: {} | Status: {}",
                            candidate.confidence_posture, candidate.status
                        ));
                        ui.label(format!("Project: {}", candidate.project_name));
                        ui.label(format!(
                            "Summary: {}",
                            candidate.recommended_constraint_summary
                        ));
                        if let Some(trigger) = candidate.trigger_class.as_deref() {
                            ui.label(format!("Trigger: {trigger}"));
                        }
                        if !candidate.narrow_usage_pattern.is_empty() {
                            ui.label(format!(
                                "Usage pattern: {}",
                                candidate.narrow_usage_pattern.join(" | ")
                            ));
                        }
                        if !candidate.matched_constraint_principles.is_empty() {
                            ui.label(format!(
                                "Principles: {}",
                                candidate.matched_constraint_principles.join(" | ")
                            ));
                        }
                        for finding in candidate.findings.iter().take(1) {
                            ui.label(format!("- candidate: {finding}"));
                        }
                        if let Some((proposal_path, proposal)) =
                            find_proposed_constraint_for_triangulation_session(
                                &self.workspace_root,
                                &candidate.triangulation_session_id,
                            )
                        {
                            ui.label(format!(
                                "Derived proposal: {} [{}]",
                                proposal
                                    .proposed_constraint
                                    .constraint_scope,
                                proposal.status
                            ));
                            ui.label(format!(
                                "Proposal origin: {}",
                                proposal_origin_label(&proposal)
                            ));
                            if let Some(proposal_id) = proposal.proposal_id.as_deref() {
                                ui.label(format!("Proposal id: {proposal_id}"));
                            }
                            ui.horizontal(|ui| {
                                if ui.small_button("Reveal Promotion Candidate").clicked() {
                                    self.reveal_governed_artifact(
                                        path,
                                        "Revealed promotion candidate",
                                        "Open failed",
                                        "Revealed promotion candidate",
                                        "Open failed",
                                        None,
                                    );
                                }
                                if ui.small_button("Reveal Derived Proposal").clicked() {
                                    self.reveal_governed_artifact(
                                        &proposal_path,
                                        "Revealed proposed constraint receipt",
                                        "Open failed",
                                        "Revealed proposed constraint receipt",
                                        "Open failed",
                                        None,
                                    );
                                }
                                if let Some(evidence_path) =
                                    candidate.evidence_receipt_paths.first()
                                {
                                    if ui.small_button("Reveal Evidence").clicked() {
                                        self.reveal_governed_artifact(
                                            evidence_path,
                                            "Revealed promotion evidence",
                                            "Open failed",
                                            "Revealed promotion evidence",
                                            "Open failed",
                                            None,
                                        );
                                    }
                                }
                                if proposal.status != "approved"
                                    && ui.small_button("Approve Candidate").clicked()
                                {
                                    self.spawn_task(UiTask::ApproveProposedConstraint {
                                        request_id_or_path: proposal_path,
                                    });
                                }
                            });
                        } else {
                            ui.horizontal(|ui| {
                                if ui.small_button("Reveal Promotion Candidate").clicked() {
                                    self.reveal_governed_artifact(
                                        path,
                                        "Revealed promotion candidate",
                                        "Open failed",
                                        "Revealed promotion candidate",
                                        "Open failed",
                                        None,
                                    );
                                }
                            });
                        }
                    }
                }
                if let Some(history) = shelf_history {
                    if !history.archived_constraints.is_empty() {
                        let never_matched_archived_count = history
                            .archived_constraints
                            .iter()
                            .filter(|entry| entry.archived_match_count == 0)
                            .count();
                        let historically_useful_archived_count = history
                            .archived_constraints
                            .iter()
                            .filter(|entry| entry.archived_match_count > 0)
                            .count();
                        let triangulated_archived_count = history
                            .archived_constraints
                            .iter()
                            .filter(|entry| {
                                approval_origins
                                    .get(&entry.constraint.constraint_id)
                                    .map(|receipt| {
                                        receipt.proposal_origin
                                            == "triangulated_task_failure"
                                    })
                                    .unwrap_or(false)
                            })
                            .count();
                        let build_verification_archived_count = history
                            .archived_constraints
                            .iter()
                            .filter(|entry| {
                                approval_origins
                                    .get(&entry.constraint.constraint_id)
                                    .map(|receipt| {
                                        receipt.proposal_origin
                                            == "build_verification_failure"
                                    })
                                    .unwrap_or(false)
                            })
                            .count();
                        ui.separator();
                        ui.label(format!("Shelf History [{}]", history.history_id));
                        if let Some(updated_at) = history.updated_at.as_deref() {
                            ui.label(format!("History updated: {updated_at}"));
                        }
                        ui.label(format!(
                            "Archived rules: {} never matched, {} historically useful",
                            never_matched_archived_count, historically_useful_archived_count
                        ));
                        ui.label(format!(
                            "Archived rule origins: {} triangulated, {} build verification",
                            triangulated_archived_count, build_verification_archived_count
                        ));
                        if ui
                            .checkbox(
                                &mut self.negative_shelf_history_show_never_matched_only,
                                "Show never matched before retirement only",
                            )
                            .changed()
                        {
                            self.save_paired_proof_ui_preferences();
                        }
                        if ui
                            .checkbox(
                                &mut self.negative_shelf_history_show_historically_useful_only,
                                "Show historically useful archived only",
                            )
                            .changed()
                        {
                            self.save_paired_proof_ui_preferences();
                        }
                        ui.checkbox(
                            &mut self.negative_shelf_history_show_triangulated_origin_only,
                            "Show triangulated-origin archived only",
                        );
                        let mut sorted_history = history
                            .archived_constraints
                            .iter()
                            .filter(|entry| {
                                let never_matched_ok =
                                    !self.negative_shelf_history_show_never_matched_only
                                        || entry.archived_match_count == 0;
                                let historically_useful_ok =
                                    !self
                                        .negative_shelf_history_show_historically_useful_only
                                        || entry.archived_match_count > 0;
                                let triangulated_origin_ok =
                                    !self
                                        .negative_shelf_history_show_triangulated_origin_only
                                        || approval_origins
                                            .get(&entry.constraint.constraint_id)
                                            .map(|receipt| {
                                                receipt.proposal_origin
                                                    == "triangulated_task_failure"
                                            })
                                            .unwrap_or(false);
                                never_matched_ok
                                    && historically_useful_ok
                                    && triangulated_origin_ok
                            })
                            .collect::<Vec<_>>();
                        sorted_history.sort_by(|left, right| {
                            if self.negative_shelf_history_show_historically_useful_only
                                && !self.negative_shelf_history_show_never_matched_only
                            {
                                right
                                    .archived_match_count
                                    .cmp(&left.archived_match_count)
                                    .then_with(|| {
                                        right
                                            .archived_at
                                            .as_deref()
                                            .unwrap_or("")
                                            .cmp(left.archived_at.as_deref().unwrap_or(""))
                                    })
                                    .then_with(|| {
                                        left.constraint
                                            .constraint_id
                                            .cmp(&right.constraint.constraint_id)
                                    })
                            } else {
                                right
                                    .archived_at
                                    .as_deref()
                                    .unwrap_or("")
                                    .cmp(left.archived_at.as_deref().unwrap_or(""))
                                    .then_with(|| {
                                        left.constraint
                                            .constraint_id
                                            .cmp(&right.constraint.constraint_id)
                                    })
                            }
                        });
                        if sorted_history.is_empty() {
                            if self.negative_shelf_history_show_triangulated_origin_only {
                                ui.label(
                                    "No archived constraints of triangulated origin match the current shelf-history filter.",
                                );
                            } else {
                                ui.label(
                                    "No archived constraints match the current shelf-history filter.",
                                );
                            }
                        } else {
                            if self.negative_shelf_history_show_historically_useful_only
                                && !self.negative_shelf_history_show_never_matched_only
                            {
                                ui.label(
                                    "Historically useful archived rules sorted by past match count, then recency",
                                );
                            } else {
                                ui.label("Most recent archived rules first");
                            }
                        }
                        for entry in sorted_history.into_iter().take(5) {
                            ui.separator();
                            ui.label(format!(
                                "{} [{} | {}]",
                                entry.constraint.constraint_id,
                                entry.constraint.constraint_scope,
                                entry.constraint.constraint_kind
                            ));
                            ui.label(format!("Archived reason: {}", entry.archived_reason));
                            ui.label(format!(
                                "Past matches before retirement: {}",
                                entry.archived_match_count
                            ));
                            if entry.archived_match_count == 0 {
                                ui.label("[never-matched-before-retirement]");
                            }
                            if let Some(archived_at) = entry.archived_at.as_deref() {
                                ui.label(format!("Archived at: {archived_at}"));
                            }
                            if let Some(approval) =
                                approval_origins.get(&entry.constraint.constraint_id)
                            {
                                let origin = if approval.proposal_origin.is_empty() {
                                    "unknown-origin"
                                } else {
                                    approval.proposal_origin.as_str()
                                };
                                ui.label(format!("Origin: {origin}"));
                                if !approval.proposal_source_id.is_empty() {
                                    ui.label(format!(
                                        "Origin source id: {}",
                                        approval.proposal_source_id
                                    ));
                                }
                            }
                            if let Some(shelf_id) = entry.archived_from_shelf_id.as_deref() {
                                ui.label(format!("Archived from shelf: {shelf_id}"));
                            }
                            ui.label(format!(
                                "Method: {}",
                                entry.constraint.forbidden_method_summary
                            ));
                            if let Some(guidance) =
                                entry.constraint.replacement_guidance.as_deref()
                            {
                                ui.label(format!("Guidance: {guidance}"));
                            }
                            ui.horizontal(|ui| {
                                if ui.small_button("Restore").clicked() {
                                    self.spawn_task(UiTask::RestoreApprovedConstraint {
                                        constraint_id: entry.constraint.constraint_id.clone(),
                                    });
                                }
                            });
                        }
                    }
                }
                            } else {
                                ui.label("No approved negative constraint shelf has been created yet.");
                            }
                        });

                    ui.separator();
                    ui.heading("Command Output");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.command_log)
                                .font(egui::TextStyle::Monospace)
                                .desired_rows(12)
                                .interactive(false),
                        );
                    });
                    ui.add_space(24.0);
                });
        });
    }
}

fn run_ui_task(workspace_root: &Path, task: UiTask) -> anyhow::Result<UiTaskResult> {
    let bridge = HostBridge::new(workspace_root.to_path_buf());
    match task {
        UiTask::RefreshBrowser => map_host_action_result(bridge.refresh_project_browser()?),
        UiTask::RefreshRuntime => {
            map_host_action_result(bridge.refresh_runtime(&HostPlannerOptions::default())?)
        }
        UiTask::RefreshProjectPatchReadiness => {
            map_host_action_result(bridge.refresh_project_patch_readiness_registry()?)
        }
        UiTask::RefreshProofHarnessRegistry => {
            map_host_action_result(bridge.refresh_proof_harness_registry()?)
        }
        UiTask::RefreshCompositionGovernance => {
            map_host_action_result(bridge.refresh_composition_governance_registry()?)
        }
        UiTask::RefreshPatchGovernance => {
            map_host_action_result(bridge.refresh_patch_governance_registry()?)
        }
        UiTask::RefreshHelperGovernance => {
            map_host_action_result(bridge.refresh_helper_governance_registry()?)
        }
        UiTask::RefreshBridgeGovernance => {
            map_host_action_result(bridge.refresh_bridge_governance_registry()?)
        }
        UiTask::RefreshFamilyGovernance => {
            map_host_action_result(bridge.refresh_family_governance_registry()?)
        }
        UiTask::RefreshTemplateGovernance => {
            map_host_action_result(bridge.refresh_template_governance_registry()?)
        }
        UiTask::ApproveProposedConstraint { request_id_or_path } => {
            map_host_action_result(bridge.approve_proposed_constraint(&request_id_or_path)?)
        }
        UiTask::SetApprovedConstraintActive {
            constraint_id,
            active,
        } => map_host_action_result(bridge.set_approved_constraint_active(&constraint_id, active)?),
        UiTask::ArchiveUnmatchedInactiveConstraints => {
            map_host_action_result(bridge.archive_unmatched_inactive_constraints()?)
        }
        UiTask::DeactivateLowValueActiveConstraints => {
            map_host_action_result(bridge.deactivate_low_value_active_constraints()?)
        }
        UiTask::RestoreApprovedConstraint { constraint_id } => {
            map_host_action_result(bridge.restore_constraint_from_history(&constraint_id)?)
        }
        UiTask::SelectProject { project_name } => {
            map_host_action_result(bridge.select_project(&project_name)?)
        }
        UiTask::ClearSelectedProject => map_host_action_result(bridge.clear_selected_project()?),
        UiTask::ImplementExtension { entry_id } => {
            map_host_action_result(bridge.mark_pending_extension_implemented(&entry_id)?)
        }
        UiTask::ValidateExtension { entry_id } => {
            map_host_action_result(bridge.validate_extension(&entry_id)?)
        }
        UiTask::PrepareExtensionPromotion { entry_id } => {
            map_host_action_result(bridge.prepare_extension_promotion(&entry_id)?)
        }
        UiTask::PrepareExtensionApplyPatch { entry_id } => {
            map_host_action_result(bridge.prepare_extension_apply_patch(&entry_id)?)
        }
        UiTask::ConsumeExtensionApplyPatch { entry_id } => {
            map_host_action_result(bridge.consume_extension_apply_patch(&entry_id)?)
        }
        UiTask::ValidateLiveExtension { entry_id } => {
            map_host_action_result(bridge.validate_live_extension(&entry_id)?)
        }
        UiTask::ArchiveExtension { entry_id, reason } => {
            map_host_action_result(bridge.archive_extension(&entry_id, Some(&reason))?)
        }
        UiTask::RunProofTemplate {
            template_id,
            request,
            auto_planner,
            port,
            model,
        } => map_host_action_result(bridge.run_proof_template(
            &template_id,
            if request.trim().is_empty() {
                None
            } else {
                Some(request.as_str())
            },
            &planner_options(auto_planner, &port, &model),
        )?),
        UiTask::RunRetrySearchLadderProof {
            auto_planner,
            port,
            model,
        } => map_host_action_result(bridge.run_retry_search_model_escalation_proof(
            &planner_options(auto_planner, &port, &model),
        )?),
        UiTask::BuildRequest {
            request,
            starter_override_id,
            auto_planner,
            port,
            model,
        } => map_host_action_result(bridge.build_request_with_starter_override(
            &request,
            starter_override_id.as_deref(),
            &planner_options(auto_planner, &port, &model),
        )?),
        UiTask::PatchRequest {
            project_name,
            request,
            auto_planner,
            port,
            model,
        } => map_host_action_result(bridge.patch_request(
            &project_name,
            &request,
            &planner_options(auto_planner, &port, &model),
        )?),
    }
}

fn planner_options(auto_planner: bool, port: &str, model: &str) -> HostPlannerOptions {
    HostPlannerOptions {
        auto_planner,
        requested_model: if model.trim().is_empty() {
            None
        } else {
            Some(model.trim().to_string())
        },
        requested_port: port.trim().parse::<u16>().ok(),
    }
}

fn map_host_action_result(result: HostActionResult) -> anyhow::Result<UiTaskResult> {
    let details_text = if result.details.is_empty() {
        String::new()
    } else {
        result.details.join("\n")
    };
    Ok(UiTaskResult {
        summary: result.summary.clone(),
        stdout: details_text.clone(),
        stderr: String::new(),
        browser_state: result.browser_state,
        action_summary: Some(ActionSummary {
            title: result.summary,
            lines: if details_text.is_empty() {
                Vec::new()
            } else {
                details_text.lines().map(|line| line.to_string()).collect()
            },
        }),
        execution_result: result.execution_result.map(|execution| {
            let file_paths = execution.file_paths.clone();
            let patch_diagnosis_path = first_patch_receipt_path(
                &file_paths,
                "runtime/patch_diagnoses/",
                "-diagnosis.json",
            );
            let patch_intent_freeze_path = first_patch_receipt_path(
                &file_paths,
                "runtime/patch_intent_freezes/",
                "-freeze.json",
            );
            let patch_plan_review_path = first_patch_receipt_path(
                &file_paths,
                "runtime/patch_plan_reviews/",
                "-review.json",
            );
            let patch_constraint_review_path = first_patch_receipt_path(
                &file_paths,
                "runtime/patch_constraint_reviews/",
                "-constraint-review.json",
            );
            let patch_postcheck_path = first_patch_receipt_path(
                &file_paths,
                "runtime/patch_diagnoses/",
                "-postcheck.json",
            );
            UiExecutionResult {
                kind: execution.kind,
                request_id: execution.request_id,
                project_name: execution.project_name,
                starter_override_id: execution.starter_override_id,
                starter_override_summary: execution.starter_override_summary,
                recommended_starter_id: execution.recommended_starter_id,
                recommended_starter_summary: execution.recommended_starter_summary,
                starter_recommendation_comparison: execution.starter_recommendation_comparison,
                family_id: execution.family_id,
                tool_kind: execution.tool_kind,
                patch_kind: execution.patch_kind,
                followup_request_mode: execution.followup_request_mode,
                followup_rationale: execution.followup_rationale,
                plan_confidence_score: execution.plan_confidence_score,
                plan_confidence_band: execution.plan_confidence_band,
                needs_llm_review: execution.needs_llm_review,
                acceptance_status: execution.acceptance_status,
                route_notes: execution.route_notes,
                file_paths,
                patch_lanes: execution.patch_lanes,
                acceptance_recipes: execution.acceptance_recipes,
                operator_bundles: execution.operator_bundles,
                chattycog_hosting_mode: execution.chattycog_hosting_mode,
                chattycog_ui_owner: execution.chattycog_ui_owner,
                chattycog_bridge_capabilities: execution.chattycog_bridge_capabilities,
                patch_diagnosis_path,
                patch_plan_review_path,
                patch_constraint_review_path,
                patch_intent_freeze_path,
                patch_postcheck_path,
            }
        }),
        fallback_result: result.fallback_result.map(|fallback| UiFallbackResult {
            request_id: fallback.request_id,
            mode: fallback.mode,
            question: fallback.question,
            interpreted_goal: fallback.interpreted_goal,
            reasons: fallback.reasons,
            candidate_family_ids: fallback.candidate_family_ids,
            requested_capabilities: fallback.requested_capabilities,
            constraints: fallback.constraints,
            suggested_extension_kind: fallback.suggested_extension_kind,
            suggested_family_id: fallback.suggested_family_id,
            suggested_tool_kind: fallback.suggested_tool_kind,
            suggested_patch_kind: fallback.suggested_patch_kind,
            suggested_bridge_capabilities: fallback.suggested_bridge_capabilities,
            suggested_hosting_mode: fallback.suggested_hosting_mode,
            suggested_artifacts: fallback.suggested_artifacts,
            acceptance_targets: fallback.acceptance_targets,
            implementation_notes: fallback.implementation_notes,
            recommended_next_step: fallback.recommended_next_step,
            pending_extension_ids: fallback.pending_extension_ids,
            pending_extension_scaffold_roots: fallback.pending_extension_scaffold_roots,
            chattycog_requested_hosting_mode: fallback.chattycog_requested_hosting_mode,
            chattycog_valid_hosting_modes: fallback.chattycog_valid_hosting_modes,
            chattycog_requested_bridge_capabilities: fallback
                .chattycog_requested_bridge_capabilities,
            chattycog_supported_bridge_capabilities: fallback
                .chattycog_supported_bridge_capabilities,
            stub_bundle_path: fallback.stub_bundle_path,
            build_failure_class: fallback.build_failure_class,
            build_failure_mode: fallback.build_failure_mode,
            matched_approved_constraint_ids: fallback.matched_approved_constraint_ids,
            matched_approved_constraint_summaries: fallback.matched_approved_constraint_summaries,
            build_verification_path: fallback.build_verification_path,
            proposed_constraint_path: fallback.proposed_constraint_path,
            proposed_constraint_summary: fallback.proposed_constraint_summary,
            proposed_constraint_replacement_guidance: fallback
                .proposed_constraint_replacement_guidance,
        }),
        extension_registry: result.extension_registry,
    })
}

fn load_browser_state(workspace_root: &Path) -> Option<ProjectBrowserState> {
    let state_path = workspace_root
        .join("runtime")
        .join("project_browser_state.json");
    let contents = std::fs::read_to_string(state_path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn load_project_spec(workspace_root: &Path, project_name: &str) -> Option<ProjectSpec> {
    let spec_path = workspace_root
        .join("output")
        .join(project_name)
        .join("ProjectSpec.json");
    let contents = std::fs::read_to_string(spec_path).ok()?;
    let mut spec: ProjectSpec = serde_json::from_str(&contents).ok()?;
    let project_dir = workspace_root.join("output").join(project_name);
    refresh_project_contract_views_for_project(Some(&project_dir), &mut spec);
    Some(spec)
}

fn load_project_patch_readiness_receipt(
    workspace_root: &Path,
    project_name: &str,
) -> Option<ProjectPatchReadinessReceiptView> {
    let path = workspace_root
        .join("runtime")
        .join("project_patch_readiness_receipts")
        .join(format!("{project_name}.json"));
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn load_build_verification_receipt(path: &str) -> Option<BuildVerificationReceiptView> {
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn load_proposed_constraint_receipt(path: &str) -> Option<ProposedConstraintReceiptView> {
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn proposal_origin_label(proposal: &ProposedConstraintReceiptView) -> &'static str {
    match proposal.source_verification_id.as_deref() {
        Some(source) if source.starts_with("triangulation-") => "triangulated",
        Some(_) => "build-verification",
        None => "unknown-origin",
    }
}

fn load_approved_constraint_shelf(workspace_root: &Path) -> Option<ApprovedConstraintShelfView> {
    let path = workspace_root
        .join("runtime")
        .join("approved_constraint_shelf.json");
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn load_constraint_shelf_history(workspace_root: &Path) -> Option<ConstraintShelfHistoryView> {
    let path = workspace_root
        .join("runtime")
        .join("constraint_shelf_history.json");
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn load_latest_constraint_approval_origins(
    workspace_root: &Path,
) -> BTreeMap<String, ConstraintApprovalReceiptView> {
    let path = workspace_root.join("runtime").join("constraint_approvals");
    let Ok(entries) = std::fs::read_dir(path) else {
        return BTreeMap::new();
    };
    let mut latest = BTreeMap::new();
    for entry in entries.flatten() {
        let contents = match std::fs::read_to_string(entry.path()) {
            Ok(contents) => contents,
            Err(_) => continue,
        };
        let receipt = match serde_json::from_str::<ConstraintApprovalReceiptView>(&contents) {
            Ok(receipt) => receipt,
            Err(_) => continue,
        };
        let replace = latest
            .get(&receipt.approved_constraint_id)
            .map(|existing: &ConstraintApprovalReceiptView| {
                receipt.created_at.as_deref().unwrap_or("")
                    > existing.created_at.as_deref().unwrap_or("")
            })
            .unwrap_or(true);
        if replace {
            latest.insert(receipt.approved_constraint_id.clone(), receipt);
        }
    }
    latest
}

fn load_recent_constraint_shelf_mutations(
    workspace_root: &Path,
) -> Vec<ConstraintShelfMutationReceiptView> {
    let path = workspace_root
        .join("runtime")
        .join("constraint_shelf_mutations");
    let Ok(entries) = std::fs::read_dir(path) else {
        return Vec::new();
    };
    let mut receipts = entries
        .flatten()
        .filter_map(|entry| {
            let contents = std::fs::read_to_string(entry.path()).ok()?;
            serde_json::from_str::<ConstraintShelfMutationReceiptView>(&contents).ok()
        })
        .collect::<Vec<_>>();
    receipts.sort_by(|left, right| {
        right
            .created_at
            .as_deref()
            .unwrap_or("")
            .cmp(left.created_at.as_deref().unwrap_or(""))
            .then_with(|| left.mutation_id.cmp(&right.mutation_id))
    });
    receipts
}

fn load_recent_failure_vault_entries(
    workspace_root: &Path,
) -> Vec<(String, FailureVaultEntryView)> {
    let path = workspace_root.join("runtime").join("failure_vault");
    let Ok(entries) = std::fs::read_dir(path) else {
        return Vec::new();
    };
    let mut receipts = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let path_text = path.display().to_string();
            let contents = std::fs::read_to_string(&path).ok()?;
            let receipt = serde_json::from_str::<FailureVaultEntryView>(&contents).ok()?;
            Some((path_text, receipt))
        })
        .collect::<Vec<_>>();
    receipts.sort_by(|left, right| {
        right
            .1
            .created_at
            .as_deref()
            .unwrap_or("")
            .cmp(left.1.created_at.as_deref().unwrap_or(""))
            .then_with(|| left.1.task_id.cmp(&right.1.task_id))
    });
    receipts
}

fn load_recent_triangulation_sessions(
    workspace_root: &Path,
) -> Vec<(String, TriangulationSessionView)> {
    let path = workspace_root
        .join("runtime")
        .join("triangulation_sessions");
    let Ok(entries) = std::fs::read_dir(path) else {
        return Vec::new();
    };
    let mut receipts = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let path_text = path.display().to_string();
            let contents = std::fs::read_to_string(&path).ok()?;
            let receipt = serde_json::from_str::<TriangulationSessionView>(&contents).ok()?;
            Some((path_text, receipt))
        })
        .collect::<Vec<_>>();
    receipts.sort_by(|left, right| {
        right
            .1
            .created_at
            .as_deref()
            .unwrap_or("")
            .cmp(left.1.created_at.as_deref().unwrap_or(""))
            .then_with(|| left.1.session_id.cmp(&right.1.session_id))
    });
    receipts
}

fn load_recent_constraint_promotion_candidates(
    workspace_root: &Path,
) -> Vec<(String, ConstraintPromotionCandidateView)> {
    let path = workspace_root
        .join("runtime")
        .join("constraint_promotion_candidates");
    let Ok(entries) = std::fs::read_dir(path) else {
        return Vec::new();
    };
    let mut receipts = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let path_text = path.display().to_string();
            let contents = std::fs::read_to_string(&path).ok()?;
            let receipt =
                serde_json::from_str::<ConstraintPromotionCandidateView>(&contents).ok()?;
            Some((path_text, receipt))
        })
        .collect::<Vec<_>>();
    receipts.sort_by(|left, right| {
        right
            .1
            .created_at
            .as_deref()
            .unwrap_or("")
            .cmp(left.1.created_at.as_deref().unwrap_or(""))
            .then_with(|| left.1.candidate_id.cmp(&right.1.candidate_id))
    });
    receipts
}

fn load_recent_atomization_floor_decisions(
    workspace_root: &Path,
) -> Vec<(String, AtomizationFloorDecisionView)> {
    let path = workspace_root
        .join("runtime")
        .join("atomization_floor_decisions");
    let Ok(entries) = std::fs::read_dir(path) else {
        return Vec::new();
    };
    let mut receipts = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let path_text = path.display().to_string();
            let contents = std::fs::read_to_string(&path).ok()?;
            let receipt = serde_json::from_str::<AtomizationFloorDecisionView>(&contents).ok()?;
            Some((path_text, receipt))
        })
        .collect::<Vec<_>>();
    receipts.sort_by(|left, right| {
        right
            .1
            .created_at
            .as_deref()
            .unwrap_or("")
            .cmp(left.1.created_at.as_deref().unwrap_or(""))
            .then_with(|| left.1.task_id.cmp(&right.1.task_id))
    });
    receipts
}

fn find_proposed_constraint_for_triangulation_session(
    workspace_root: &Path,
    triangulation_session_id: &str,
) -> Option<(String, ProposedConstraintReceiptView)> {
    let path = workspace_root
        .join("runtime")
        .join("proposed_constraint_receipts");
    let Ok(entries) = std::fs::read_dir(path) else {
        return None;
    };
    entries.flatten().find_map(|entry| {
        let path = entry.path();
        let path_text = path.display().to_string();
        let contents = std::fs::read_to_string(&path).ok()?;
        let receipt = serde_json::from_str::<ProposedConstraintReceiptView>(&contents).ok()?;
        if receipt.source_verification_id.as_deref() == Some(triangulation_session_id) {
            Some((path_text, receipt))
        } else {
            None
        }
    })
}

fn summarize_constraint_shelf_mutation_actions(
    receipts: &[ConstraintShelfMutationReceiptView],
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for receipt in receipts.iter().take(20) {
        *counts.entry(receipt.action.clone()).or_insert(0) += 1;
    }
    counts
}

fn load_recent_build_verification_matches(
    workspace_root: &Path,
) -> Vec<(String, BuildVerificationReceiptView)> {
    let receipts_dir = workspace_root
        .join("runtime")
        .join("build_verification_receipts");
    let Ok(entries) = std::fs::read_dir(receipts_dir) else {
        return Vec::new();
    };
    let mut receipts = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let path_text = path.display().to_string();
            let contents = std::fs::read_to_string(&path).ok()?;
            let receipt = serde_json::from_str::<BuildVerificationReceiptView>(&contents).ok()?;
            if receipt.matched_approved_constraint_ids.is_empty() {
                return None;
            }
            Some((path_text, receipt))
        })
        .collect::<Vec<_>>();
    receipts.sort_by(|left, right| right.0.cmp(&left.0));
    receipts
}

fn approved_constraint_match_counts(
    matches: &[(String, BuildVerificationReceiptView)],
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for (_, receipt) in matches {
        for constraint_id in &receipt.matched_approved_constraint_ids {
            *counts.entry(constraint_id.clone()).or_insert(0) += 1;
        }
    }
    counts
}

#[derive(Debug, Clone, Default)]
struct ApprovedConstraintMatchBreakdown {
    total_matches: usize,
    by_failure_mode: BTreeMap<String, usize>,
    by_family_id: BTreeMap<String, usize>,
    by_tool_kind: BTreeMap<String, usize>,
}

fn approved_constraint_match_breakdowns(
    matches: &[(String, BuildVerificationReceiptView)],
) -> BTreeMap<String, ApprovedConstraintMatchBreakdown> {
    let mut breakdowns: BTreeMap<String, ApprovedConstraintMatchBreakdown> = BTreeMap::new();
    for (_, receipt) in matches {
        for constraint_id in &receipt.matched_approved_constraint_ids {
            let breakdown = breakdowns.entry(constraint_id.clone()).or_default();
            breakdown.total_matches += 1;
            *breakdown
                .by_failure_mode
                .entry(receipt.failure_mode.clone())
                .or_insert(0) += 1;
            let family_id = receipt
                .suggested_family_id
                .clone()
                .unwrap_or_else(|| "unknown_family".to_string());
            *breakdown.by_family_id.entry(family_id).or_insert(0) += 1;
            let tool_kind = receipt
                .suggested_tool_kind
                .clone()
                .unwrap_or_else(|| "unknown_tool".to_string());
            *breakdown.by_tool_kind.entry(tool_kind).or_insert(0) += 1;
        }
    }
    breakdowns
}

fn load_runtime_status(workspace_root: &Path) -> RuntimeStatusView {
    let runtime_root = workspace_root.join("runtime");
    let config = std::fs::read_to_string(runtime_root.join("runtime_config.json"))
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok());
    let catalog = latest_catalog_path(&runtime_root)
        .and_then(|path| std::fs::read_to_string(path).ok())
        .and_then(|contents| serde_json::from_str(&contents).ok());
    RuntimeStatusView { config, catalog }
}

fn load_proof_governance_refresh_status(
    workspace_root: &Path,
) -> Option<ProofGovernanceRefreshStatusView> {
    let path = workspace_root
        .join("runtime")
        .join("proof_governance_refresh_status.json");
    let contents = std::fs::read_to_string(&path).ok()?;
    let mut status = serde_json::from_str::<ProofGovernanceRefreshStatusView>(&contents).ok()?;
    let mut refresh_modified: Option<std::time::SystemTime> = None;
    if let Ok(metadata) = std::fs::metadata(&path) {
        if let Ok(modified) = metadata.modified() {
            refresh_modified = Some(modified);
            let modified_utc: chrono::DateTime<Utc> = modified.into();
            let modified_local = modified_utc.with_timezone(&Local);
            status.refreshed_at_label =
                Some(modified_local.format("%Y-%m-%d %H:%M:%S").to_string());
            if let Ok(elapsed) = modified.elapsed() {
                status.age_minutes = Some(elapsed.as_secs() / 60);
            }
        }
    }
    if let Some((latest_receipt_modified, latest_receipt_label)) =
        latest_cross_family_proof_receipt_modified(workspace_root)
    {
        status.latest_proof_receipt_label = Some(latest_receipt_label);
        status.newer_proof_receipts_exist = refresh_modified
            .map(|refresh_modified| latest_receipt_modified > refresh_modified)
            .unwrap_or(true);
    }
    Some(status)
}

fn load_composition_governance_refresh_status(
    workspace_root: &Path,
) -> Option<CompositionGovernanceRefreshStatusView> {
    let path = workspace_root
        .join("runtime")
        .join("composition_governance_refresh_status.json");
    let contents = std::fs::read_to_string(&path).ok()?;
    let mut status =
        serde_json::from_str::<CompositionGovernanceRefreshStatusView>(&contents).ok()?;
    if let Ok(metadata) = std::fs::metadata(&path) {
        if let Ok(modified) = metadata.modified() {
            let modified_utc: chrono::DateTime<Utc> = modified.into();
            let modified_local = modified_utc.with_timezone(&Local);
            status.refreshed_at_label =
                Some(modified_local.format("%Y-%m-%d %H:%M:%S").to_string());
            if let Ok(elapsed) = modified.elapsed() {
                status.age_minutes = Some(elapsed.as_secs() / 60);
            }
        }
    }
    Some(status)
}

fn load_patch_governance_refresh_status(
    workspace_root: &Path,
) -> Option<PatchGovernanceRefreshStatusView> {
    let path = workspace_root
        .join("runtime")
        .join("patch_governance_refresh_status.json");
    let contents = std::fs::read_to_string(&path).ok()?;
    let mut status = serde_json::from_str::<PatchGovernanceRefreshStatusView>(&contents).ok()?;
    if let Ok(metadata) = std::fs::metadata(&path) {
        if let Ok(modified) = metadata.modified() {
            let modified_utc: chrono::DateTime<Utc> = modified.into();
            let modified_local = modified_utc.with_timezone(&Local);
            status.refreshed_at_label =
                Some(modified_local.format("%Y-%m-%d %H:%M:%S").to_string());
            if let Ok(elapsed) = modified.elapsed() {
                status.age_minutes = Some(elapsed.as_secs() / 60);
            }
        }
    }
    Some(status)
}

fn load_project_patch_readiness_refresh_status(
    workspace_root: &Path,
) -> Option<ProjectPatchReadinessRefreshStatusView> {
    let path = workspace_root
        .join("runtime")
        .join("project_patch_readiness_refresh_status.json");
    let contents = std::fs::read_to_string(&path).ok()?;
    let mut status =
        serde_json::from_str::<ProjectPatchReadinessRefreshStatusView>(&contents).ok()?;
    if let Ok(metadata) = std::fs::metadata(&path) {
        if let Ok(modified) = metadata.modified() {
            let modified_utc: chrono::DateTime<Utc> = modified.into();
            let modified_local = modified_utc.with_timezone(&Local);
            status.refreshed_at_label =
                Some(modified_local.format("%Y-%m-%d %H:%M:%S").to_string());
            if let Ok(elapsed) = modified.elapsed() {
                status.age_minutes = Some(elapsed.as_secs() / 60);
            }
        }
    }
    Some(status)
}

fn load_helper_governance_refresh_status(
    workspace_root: &Path,
) -> Option<HelperGovernanceRefreshStatusView> {
    let path = workspace_root
        .join("runtime")
        .join("helper_governance_refresh_status.json");
    let contents = std::fs::read_to_string(&path).ok()?;
    let mut status = serde_json::from_str::<HelperGovernanceRefreshStatusView>(&contents).ok()?;
    if let Ok(metadata) = std::fs::metadata(&path) {
        if let Ok(modified) = metadata.modified() {
            let modified_utc: chrono::DateTime<Utc> = modified.into();
            let modified_local = modified_utc.with_timezone(&Local);
            status.refreshed_at_label =
                Some(modified_local.format("%Y-%m-%d %H:%M:%S").to_string());
            if let Ok(elapsed) = modified.elapsed() {
                status.age_minutes = Some(elapsed.as_secs() / 60);
            }
        }
    }
    Some(status)
}

fn load_bridge_governance_refresh_status(
    workspace_root: &Path,
) -> Option<BridgeGovernanceRefreshStatusView> {
    let path = workspace_root
        .join("runtime")
        .join("bridge_governance_refresh_status.json");
    let contents = std::fs::read_to_string(&path).ok()?;
    let mut status = serde_json::from_str::<BridgeGovernanceRefreshStatusView>(&contents).ok()?;
    if let Ok(metadata) = std::fs::metadata(&path) {
        if let Ok(modified) = metadata.modified() {
            let modified_utc: chrono::DateTime<Utc> = modified.into();
            let modified_local = modified_utc.with_timezone(&Local);
            status.refreshed_at_label =
                Some(modified_local.format("%Y-%m-%d %H:%M:%S").to_string());
            if let Ok(elapsed) = modified.elapsed() {
                status.age_minutes = Some(elapsed.as_secs() / 60);
            }
        }
    }
    Some(status)
}

fn load_family_governance_refresh_status(
    workspace_root: &Path,
) -> Option<FamilyGovernanceRefreshStatusView> {
    let path = workspace_root
        .join("runtime")
        .join("family_governance_refresh_status.json");
    let contents = std::fs::read_to_string(&path).ok()?;
    let mut status = serde_json::from_str::<FamilyGovernanceRefreshStatusView>(&contents).ok()?;
    if let Ok(metadata) = std::fs::metadata(&path) {
        if let Ok(modified) = metadata.modified() {
            let modified_utc: chrono::DateTime<Utc> = modified.into();
            let modified_local = modified_utc.with_timezone(&Local);
            status.refreshed_at_label =
                Some(modified_local.format("%Y-%m-%d %H:%M:%S").to_string());
            if let Ok(elapsed) = modified.elapsed() {
                status.age_minutes = Some(elapsed.as_secs() / 60);
            }
        }
    }
    Some(status)
}

fn load_family_usage_summary(workspace_root: &Path) -> Option<FamilyUsageSummaryView> {
    let path = workspace_root
        .join("runtime")
        .join("family_usage_summary.json");
    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str::<FamilyUsageSummaryView>(&contents).ok()
}

fn load_starter_usage_summary(workspace_root: &Path) -> Option<StarterUsageSummaryView> {
    let path = workspace_root
        .join("runtime")
        .join("starter_usage_summary.json");
    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str::<StarterUsageSummaryView>(&contents).ok()
}

fn load_triangulation_loop_summary(workspace_root: &Path) -> Option<TriangulationLoopSummaryView> {
    let path = workspace_root
        .join("runtime")
        .join("triangulation_loop_summary.json");
    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str::<TriangulationLoopSummaryView>(&contents).ok()
}

fn load_recent_build_receipts(workspace_root: &Path) -> Vec<BuildReceiptView> {
    let receipts_dir = workspace_root.join("runtime").join("build_receipts");
    let Ok(entries) = fs::read_dir(receipts_dir) else {
        return Vec::new();
    };
    let mut receipts = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let modified = entry.metadata().ok()?.modified().ok()?;
            let contents = fs::read_to_string(entry.path()).ok()?;
            let receipt = serde_json::from_str::<BuildReceiptView>(&contents).ok()?;
            Some((modified, receipt))
        })
        .collect::<Vec<_>>();
    receipts.sort_by(|left, right| right.0.cmp(&left.0));
    receipts.into_iter().map(|(_, receipt)| receipt).collect()
}

fn load_recent_project_starter_override_counts(workspace_root: &Path) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for receipt in load_recent_build_receipts(workspace_root)
        .into_iter()
        .take(20)
        .filter(|receipt| {
            receipt.starter_recommendation_comparison.as_deref() == Some("overrode_normal_routing")
        })
    {
        *counts.entry(receipt.project_name).or_insert(0) += 1;
    }
    counts
}

fn latest_project_starter_override_receipt(
    workspace_root: &Path,
    project_name: &str,
) -> Option<BuildReceiptView> {
    load_recent_build_receipts(workspace_root)
        .into_iter()
        .find(|receipt| {
            receipt.project_name == project_name
                && receipt.starter_recommendation_comparison.as_deref()
                    == Some("overrode_normal_routing")
        })
}

fn load_family_governance_receipts(workspace_root: &Path) -> Vec<FamilyGovernanceReceiptView> {
    let receipts_dir = workspace_root
        .join("runtime")
        .join("family_governance_receipts");
    let Ok(entries) = std::fs::read_dir(receipts_dir) else {
        return Vec::new();
    };
    let mut receipts = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        let Ok(contents) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(receipt) = serde_json::from_str::<FamilyGovernanceReceiptView>(&contents) else {
            continue;
        };
        receipts.push(receipt);
    }
    receipts.sort_by(|left, right| left.family_id.cmp(&right.family_id));
    receipts
}

fn load_template_governance_refresh_status(
    workspace_root: &Path,
) -> Option<TemplateGovernanceRefreshStatusView> {
    let path = workspace_root
        .join("runtime")
        .join("template_governance_refresh_status.json");
    let contents = std::fs::read_to_string(&path).ok()?;
    let mut status = serde_json::from_str::<TemplateGovernanceRefreshStatusView>(&contents).ok()?;
    if let Ok(metadata) = std::fs::metadata(&path) {
        if let Ok(modified) = metadata.modified() {
            let modified_utc: chrono::DateTime<Utc> = modified.into();
            let modified_local = modified_utc.with_timezone(&Local);
            status.refreshed_at_label =
                Some(modified_local.format("%Y-%m-%d %H:%M:%S").to_string());
            if let Ok(elapsed) = modified.elapsed() {
                status.age_minutes = Some(elapsed.as_secs() / 60);
            }
        }
    }
    Some(status)
}

fn load_template_governance_receipts(workspace_root: &Path) -> Vec<TemplateGovernanceReceiptView> {
    let receipts_dir = workspace_root
        .join("runtime")
        .join("template_governance_receipts");
    let Ok(entries) = std::fs::read_dir(receipts_dir) else {
        return Vec::new();
    };
    let mut receipts = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        let Ok(contents) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(receipt) = serde_json::from_str::<TemplateGovernanceReceiptView>(&contents) else {
            continue;
        };
        receipts.push(receipt);
    }
    receipts.sort_by(|left, right| left.template_bundle_id.cmp(&right.template_bundle_id));
    receipts
}

fn load_extension_registry(workspace_root: &Path) -> Option<HostExtensionRegistryView> {
    let registry_path = workspace_root
        .join("operator_registry")
        .join("pending_lanes.json");
    let contents = std::fs::read_to_string(registry_path).ok()?;
    let registry = serde_json::from_str::<PendingExtensionRegistryFile>(&contents).ok()?;
    Some(HostExtensionRegistryView {
        pending_count: registry
            .entries
            .iter()
            .filter(|entry| entry.status == "pending_implementation")
            .count(),
        implemented_count: registry
            .entries
            .iter()
            .filter(|entry| entry.status == "implemented")
            .count(),
        validated_count: registry
            .entries
            .iter()
            .filter(|entry| entry.status == "validated_ready")
            .count(),
        promotion_prepared_count: registry
            .entries
            .iter()
            .filter(|entry| entry.status == "promotion_prepared")
            .count(),
        apply_patch_ready_count: registry
            .entries
            .iter()
            .filter(|entry| entry.status == "apply_patch_ready")
            .count(),
        host_wired_count: registry
            .entries
            .iter()
            .filter(|entry| entry.status == "host_wired")
            .count(),
        fully_live_count: registry
            .entries
            .iter()
            .filter(|entry| entry.status == "fully_live")
            .count(),
        archived_count: registry
            .entries
            .iter()
            .filter(|entry| entry.status == "archived")
            .count(),
        active_entries: registry
            .entries
            .iter()
            .filter(|entry| entry.status != "fully_live" && entry.status != "archived")
            .cloned()
            .collect(),
        fully_live_entries: registry
            .entries
            .iter()
            .filter(|entry| entry.status == "fully_live")
            .cloned()
            .collect(),
        archived_entries: registry
            .entries
            .iter()
            .filter(|entry| entry.status == "archived")
            .cloned()
            .collect(),
    })
}

fn load_cross_family_paired_proof_receipts(
    workspace_root: &Path,
) -> Vec<(CrossFamilyPairedProofReceiptSummary, PathBuf)> {
    let dir = workspace_root
        .join("runtime")
        .join("cross_family_paired_proof_receipts");
    let mut paths = fs::read_dir(dir)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(|entry| entry.ok()))
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    paths.sort_by(|a, b| b.cmp(a));
    paths
        .into_iter()
        .filter_map(|path| {
            let contents = fs::read_to_string(&path).ok()?;
            let receipt =
                serde_json::from_str::<CrossFamilyPairedProofReceiptSummary>(&contents).ok()?;
            Some((receipt, path))
        })
        .collect()
}

fn load_latest_retry_search_proof_receipt(
    workspace_root: &Path,
) -> Option<(RetrySearchProofReceiptView, PathBuf)> {
    let dir = workspace_root.join("runtime").join("retry_search_proofs");
    let mut paths = fs::read_dir(dir)
        .ok()?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    paths.sort_by(|a, b| b.cmp(a));
    paths.into_iter().find_map(|path| {
        let contents = fs::read_to_string(&path).ok()?;
        let receipt = serde_json::from_str::<RetrySearchProofReceiptView>(&contents).ok()?;
        Some((receipt, path))
    })
}

fn load_capability_comparison_receipt(path: &str) -> Option<CapabilityComparisonReceiptSummary> {
    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str::<CapabilityComparisonReceiptSummary>(&contents).ok()
}

fn proof_template_label(template: &PrimitiveProofTemplate) -> String {
    if !template.display_label.trim().is_empty() {
        template.display_label.clone()
    } else {
        format!("{} ({})", template.template_id, template.template_kind)
    }
}

fn proof_history_filter_label(filter: &str, templates: &[PrimitiveProofTemplate]) -> String {
    if filter == "all" {
        "All Templates".to_string()
    } else {
        templates
            .iter()
            .find(|template| template.template_id == filter)
            .map(proof_template_label)
            .unwrap_or_else(|| filter.to_string())
    }
}

fn proof_profile_label(profile_name: &str) -> String {
    if profile_name == "custom" {
        "Custom".to_string()
    } else {
        profile_name.to_string()
    }
}

fn proof_template_description(template: &PrimitiveProofTemplate) -> &str {
    if template.description.trim().is_empty() {
        "Runs a generalized primitive proof across the template's declared family pair."
    } else {
        template.description.as_str()
    }
}

fn comparison_bundle_for_template(
    template: &PrimitiveProofTemplate,
    bundles: &[CapabilityComparisonBundle],
) -> Option<CapabilityComparisonBundle> {
    bundles
        .iter()
        .find(|bundle| bundle.bundle_id == template.execution_recipe.comparison_bundle_id)
        .cloned()
}

fn proof_receipt_comparison_bundle(
    receipt: &CrossFamilyPairedProofReceiptSummary,
    bundles: &[CapabilityComparisonBundle],
) -> Option<CapabilityComparisonBundle> {
    let bundle_id = if let Some(bundle_id) = receipt.capability_comparison_bundle_id.as_deref() {
        bundle_id.to_string()
    } else {
        let template_id = receipt.proof_template_id.as_deref()?;
        built_in_proof_templates()
            .into_iter()
            .find(|template| template.template_id == template_id)
            .map(|template| template.execution_recipe.comparison_bundle_id)?
    };
    bundles
        .iter()
        .find(|bundle| bundle.bundle_id == bundle_id)
        .cloned()
}

fn required_bundle_status_lines(
    bundle: &CapabilityComparisonBundle,
    comparison_receipt: &CapabilityComparisonReceiptSummary,
) -> Vec<(String, bool)> {
    bundle
        .required_shared_capability_classes
        .iter()
        .map(|capability| {
            (
                capability.clone(),
                comparison_receipt
                    .shared_capability_classes
                    .contains(capability),
            )
        })
        .collect()
}

fn compact_required_bundle_summary(
    bundle: &CapabilityComparisonBundle,
    comparison_receipt: &CapabilityComparisonReceiptSummary,
) -> String {
    let required = &bundle.required_shared_capability_classes;
    let shared = &comparison_receipt.shared_capability_classes;
    let satisfied = required
        .iter()
        .filter(|capability| shared.contains(*capability))
        .count();
    let missing = required
        .iter()
        .filter(|capability| !shared.contains(*capability))
        .cloned()
        .collect::<Vec<_>>();
    if missing.is_empty() {
        format!("{satisfied}/{} satisfied", required.len())
    } else {
        format!(
            "{satisfied}/{} satisfied, missing: {}",
            required.len(),
            missing.join(", ")
        )
    }
}

fn proof_receipt_template_label(receipt: &CrossFamilyPairedProofReceiptSummary) -> Option<String> {
    if let Some(label) = receipt.proof_template_display_label.as_ref() {
        if !label.trim().is_empty() {
            return Some(label.clone());
        }
    }
    let templates = built_in_proof_templates();
    if let Some(template_id) = receipt.proof_template_id.as_deref() {
        if let Some(template) = templates
            .iter()
            .find(|template| template.template_id == template_id)
        {
            return Some(proof_template_label(template));
        }
    }
    match (
        receipt.proof_template_id.as_deref(),
        receipt.proof_template_kind.as_deref(),
    ) {
        (Some(template_id), Some(template_kind)) => {
            Some(format!("{template_id} ({template_kind})"))
        }
        (Some(template_id), None) => Some(template_id.to_string()),
        (None, Some(template_kind)) => Some(template_kind.to_string()),
        (None, None) => None,
    }
}

fn proof_receipt_matches_filter(
    receipt: &CrossFamilyPairedProofReceiptSummary,
    filter: &str,
) -> bool {
    filter == "all" || receipt.proof_template_id.as_deref() == Some(filter)
}

fn paired_proof_artifact_count(
    receipt: &CrossFamilyPairedProofReceiptSummary,
    receipt_path: &Path,
) -> usize {
    [
        Some(receipt_path.to_string_lossy().to_string()),
        Some(receipt.comparison_receipt_path.clone()),
        receipt.left_primitive_execution_plan_path.clone(),
        receipt.right_primitive_execution_plan_path.clone(),
        receipt.left_composable_route_plan_path.clone(),
        receipt.right_composable_route_plan_path.clone(),
    ]
    .into_iter()
    .flatten()
    .count()
}

fn build_paired_proof_diff_summary(
    latest: &CrossFamilyPairedProofReceiptSummary,
    latest_path: &Path,
    previous: &CrossFamilyPairedProofReceiptSummary,
    previous_path: &Path,
) -> Vec<String> {
    let mut lines = Vec::new();

    if latest.equivalent_capability_fulfillment != previous.equivalent_capability_fulfillment {
        lines.push(format!(
            "Outcome changed: {} -> {}",
            previous.equivalent_capability_fulfillment, latest.equivalent_capability_fulfillment
        ));
    }
    if latest.shared_request != previous.shared_request {
        lines.push("Shared request changed.".to_string());
    }
    if latest.left_request != previous.left_request {
        lines.push("Left family request changed.".to_string());
    }
    if latest.right_request != previous.right_request {
        lines.push("Right family request changed.".to_string());
    }
    if latest.left_project_name != previous.left_project_name
        || latest.right_project_name != previous.right_project_name
    {
        lines.push(format!(
            "Project pair changed: {} -> {} / {} -> {}",
            previous.left_project_name,
            latest.left_project_name,
            previous.right_project_name,
            latest.right_project_name
        ));
    }

    let latest_artifacts = paired_proof_artifact_count(latest, latest_path);
    let previous_artifacts = paired_proof_artifact_count(previous, previous_path);
    if latest_artifacts != previous_artifacts {
        lines.push(format!(
            "Linked artifact count changed: {} -> {}",
            previous_artifacts, latest_artifacts
        ));
    }

    let latest_notes = latest.notes.join(" | ");
    let previous_notes = previous.notes.join(" | ");
    if latest_notes != previous_notes {
        lines.push("Proof notes changed.".to_string());
    }

    if lines.is_empty() {
        lines.push("No high-signal proof summary changes detected.".to_string());
    }

    lines
}

#[derive(Debug, Clone, serde::Deserialize)]
struct PendingExtensionRegistryFile {
    entries: Vec<PendingExtensionEntry>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct ExtensionFavoritesFile {
    entry_ids: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct ExtensionRecentFile {
    entry_ids: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct ExtensionNotesFile {
    notes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct ExtensionActivityFile {
    entries: Vec<ExtensionActivityItem>,
}

fn latest_catalog_path(runtime_root: &Path) -> Option<PathBuf> {
    let checks_dir = runtime_root.join("runtime_checks");
    let mut candidates = std::fs::read_dir(checks_dir)
        .ok()?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.contains("catalog"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|path| {
        std::fs::metadata(path)
            .and_then(|metadata| metadata.modified())
            .ok()
    });
    candidates.pop()
}

fn load_extension_favorites(workspace_root: &Path) -> BTreeSet<String> {
    let path = workspace_root
        .join("runtime")
        .join("extension_favorites.json");
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str::<ExtensionFavoritesFile>(&contents).ok())
        .map(|file| file.entry_ids.into_iter().collect())
        .unwrap_or_default()
}

fn load_paired_proof_favorites(workspace_root: &Path) -> BTreeSet<String> {
    let path = workspace_root
        .join("runtime")
        .join("paired_proof_favorites.json");
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str::<ExtensionFavoritesFile>(&contents).ok())
        .map(|file| file.entry_ids.into_iter().collect())
        .unwrap_or_default()
}

fn load_paired_proof_ui_preferences(workspace_root: &Path) -> Option<PairedProofUiPreferences> {
    let path = workspace_root
        .join("runtime")
        .join("paired_proof_ui_preferences.json");
    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str::<PairedProofUiPreferences>(&contents).ok()
}

fn save_paired_proof_ui_preferences(
    path: &Path,
    preferences: &PairedProofUiPreferences,
) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_string_pretty(preferences)?)?;
    Ok(())
}

fn load_paired_proof_notes(workspace_root: &Path) -> BTreeMap<String, String> {
    let path = workspace_root
        .join("runtime")
        .join("paired_proof_notes.json");
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str::<ExtensionNotesFile>(&contents).ok())
        .map(|file| file.notes)
        .unwrap_or_default()
}

fn save_extension_favorites(path: &Path, favorites: &BTreeSet<String>) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let payload = ExtensionFavoritesFile {
        entry_ids: favorites.iter().cloned().collect(),
    };
    fs::write(path, serde_json::to_string_pretty(&payload)?)?;
    Ok(())
}

fn load_recent_extensions(workspace_root: &Path) -> Vec<String> {
    let path = workspace_root.join("runtime").join("extension_recent.json");
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str::<ExtensionRecentFile>(&contents).ok())
        .map(|file| file.entry_ids)
        .unwrap_or_default()
}

fn save_recent_extensions(path: &Path, recents: &[String]) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let payload = ExtensionRecentFile {
        entry_ids: recents.to_vec(),
    };
    fs::write(path, serde_json::to_string_pretty(&payload)?)?;
    Ok(())
}

fn load_extension_notes(workspace_root: &Path) -> BTreeMap<String, String> {
    let path = workspace_root.join("runtime").join("extension_notes.json");
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str::<ExtensionNotesFile>(&contents).ok())
        .map(|file| file.notes)
        .unwrap_or_default()
}

fn save_extension_notes(path: &Path, notes: &BTreeMap<String, String>) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let payload = ExtensionNotesFile {
        notes: notes.clone(),
    };
    fs::write(path, serde_json::to_string_pretty(&payload)?)?;
    Ok(())
}

fn load_extension_activity(workspace_root: &Path) -> Vec<ExtensionActivityItem> {
    let path = workspace_root
        .join("runtime")
        .join("extension_activity.json");
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str::<ExtensionActivityFile>(&contents).ok())
        .map(|file| file.entries)
        .unwrap_or_default()
}

fn save_extension_activity(path: &Path, activity: &[ExtensionActivityItem]) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let payload = ExtensionActivityFile {
        entries: activity.to_vec(),
    };
    fs::write(path, serde_json::to_string_pretty(&payload)?)?;
    Ok(())
}

fn write_text_file(path: &Path, contents: &str) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}

fn activity_timestamp_label() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn export_timestamp_slug() -> String {
    Utc::now().format("%Y%m%d-%H%M%S").to_string()
}

fn load_extension_export_history(export_dir: &Path, entry_id: &str) -> Vec<PathBuf> {
    let prefix = format!("{entry_id}-");
    let mut candidates = fs::read_dir(export_dir)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(|entry| entry.ok()))
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with(&prefix) && name.ends_with(".md"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|a, b| b.cmp(a));
    candidates
}

fn build_export_diff(latest_path: &Path, previous_path: &Path) -> String {
    let latest = fs::read_to_string(latest_path).unwrap_or_else(|_| String::new());
    let previous = fs::read_to_string(previous_path).unwrap_or_else(|_| String::new());
    let latest_lines = latest.lines().collect::<Vec<_>>();
    let previous_lines = previous.lines().collect::<Vec<_>>();

    let mut lines = vec![
        format!(
            "latest: {}",
            short_path(latest_path.to_string_lossy().as_ref())
        ),
        format!(
            "previous: {}",
            short_path(previous_path.to_string_lossy().as_ref())
        ),
        String::new(),
    ];

    for line in latest_lines
        .iter()
        .filter(|line| !previous_lines.contains(line))
    {
        lines.push(format!("+ {}", line));
    }
    for line in previous_lines
        .iter()
        .filter(|line| !latest_lines.contains(line))
    {
        lines.push(format!("- {}", line));
    }

    if lines.len() == 3 {
        lines.push("(no line-level differences detected)".to_string());
    }

    lines.join("\n")
}

fn build_extension_summary_markdown(
    entry: &PendingExtensionEntry,
    blockers: &[String],
    mismatch_hint_count: usize,
    note: Option<String>,
) -> String {
    let mut lines = vec![
        format!("# Lane Summary: {}", entry.entry_id),
        String::new(),
        format!("- Status: {}", entry.status),
        format!("- Extension kind: {}", entry.extension_kind),
        format!(
            "- Unresolved layers: {}",
            if entry.unresolved_layers.is_empty() {
                "none".to_string()
            } else {
                entry.unresolved_layers.join(", ")
            }
        ),
        format!(
            "- Family: {}",
            entry.family_id.as_deref().unwrap_or("unknown_family")
        ),
        format!("- Tool: {}", entry.tool_kind.as_deref().unwrap_or("none")),
        format!("- Patch: {}", entry.patch_kind.as_deref().unwrap_or("none")),
        format!("- Compare hints: {}", mismatch_hint_count),
        format!("- Scaffold root: {}", entry.scaffold_root),
        format!("- Source spec: {}", entry.source_stub_path),
    ];

    if let Some(reason) = entry.archived_reason.as_deref() {
        lines.push(format!("- Archive reason: {}", reason));
    }

    if !entry.missing_family_build_primitive_classes.is_empty() {
        lines.push(format!(
            "- Missing family build classes: {}",
            entry.missing_family_build_primitive_classes.join(", ")
        ));
    }
    if !entry.missing_patch_primitive_classes.is_empty() {
        lines.push(format!(
            "- Missing patch classes: {}",
            entry.missing_patch_primitive_classes.join(", ")
        ));
    }
    if !entry.missing_helper_primitive_kinds.is_empty() {
        lines.push(format!(
            "- Missing helper kinds: {}",
            entry.missing_helper_primitive_kinds.join(", ")
        ));
    }

    lines.push(String::new());
    lines.push("## Blockers".to_string());
    if blockers.is_empty() {
        lines.push("- None".to_string());
    } else {
        for blocker in blockers {
            lines.push(format!("- {}", blocker));
        }
    }

    lines.push(String::new());
    lines.push("## Note".to_string());
    lines.push(note.unwrap_or_else(|| "(no note)".to_string()));

    if !entry.integrated_paths.is_empty() {
        lines.push(String::new());
        lines.push("## Integrated Files".to_string());
        for path in &entry.integrated_paths {
            lines.push(format!("- {}", path));
        }
    }

    if !entry.promotion_artifacts.is_empty() {
        lines.push(String::new());
        lines.push("## Promotion Artifacts".to_string());
        for path in &entry.promotion_artifacts {
            lines.push(format!("- {}", path));
        }
    }

    if !entry.apply_patch_artifacts.is_empty() {
        lines.push(String::new());
        lines.push("## Apply Patch Artifacts".to_string());
        for path in &entry.apply_patch_artifacts {
            lines.push(format!("- {}", path));
        }
    }

    lines.join("\n")
}

fn build_paired_proof_summary_markdown(
    receipt: &CrossFamilyPairedProofReceiptSummary,
    receipt_path: &Path,
    note: Option<String>,
) -> String {
    let mut lines = vec![
        format!("# Paired Proof Summary: {}", receipt.receipt_id),
        String::new(),
        format!("- Shared request: {}", receipt.shared_request),
        format!(
            "- Equivalent capability fulfillment: {}",
            receipt.equivalent_capability_fulfillment
        ),
        format!("- Left project: {}", receipt.left_project_name),
        format!("- Right project: {}", receipt.right_project_name),
        format!("- Paired receipt: {}", receipt_path.display()),
        format!("- Comparison receipt: {}", receipt.comparison_receipt_path),
    ];

    if let Some(created_at) = &receipt.created_at {
        lines.push(format!("- Created: {}", created_at));
    }
    if let Some(path) = &receipt.left_composable_route_plan_path {
        lines.push(format!("- Left route plan: {}", path));
    }
    if let Some(path) = &receipt.right_composable_route_plan_path {
        lines.push(format!("- Right route plan: {}", path));
    }
    if let Some(path) = &receipt.left_primitive_execution_plan_path {
        lines.push(format!("- Left primitive execution plan: {}", path));
    }
    if let Some(path) = &receipt.right_primitive_execution_plan_path {
        lines.push(format!("- Right primitive execution plan: {}", path));
    }

    lines.push(String::new());
    lines.push("## Requests".to_string());
    lines.push(format!("- Left request: {}", receipt.left_request));
    lines.push(format!("- Right request: {}", receipt.right_request));

    if !receipt.notes.is_empty() {
        lines.push(String::new());
        lines.push("## Notes".to_string());
        for note_line in &receipt.notes {
            lines.push(format!("- {}", note_line));
        }
    }

    lines.push(String::new());
    lines.push("## Proof Note".to_string());
    lines.push(note.unwrap_or_else(|| "(no note)".to_string()));

    lines.join("\n")
}

fn short_path(value: &str) -> String {
    Path::new(value)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(value)
        .to_string()
}

fn first_patch_receipt_path(
    paths: &[String],
    required_fragment: &str,
    required_suffix: &str,
) -> Option<String> {
    paths
        .iter()
        .find(|path| path.contains(required_fragment) && path.ends_with(required_suffix))
        .cloned()
}

fn open_path_in_explorer(path: &str, reveal_file: bool) -> anyhow::Result<()> {
    let target = PathBuf::from(path);
    if reveal_file {
        if !target.exists() {
            anyhow::bail!("path does not exist: {}", target.display());
        }
        let status = Command::new("explorer.exe")
            .arg("/select,")
            .arg(&target)
            .status()?;
        if !status.success() {
            anyhow::bail!("explorer returned a non-zero status");
        }
    } else {
        let open_target = if target.is_dir() {
            target
        } else {
            target
                .parent()
                .map(Path::to_path_buf)
                .ok_or_else(|| anyhow::anyhow!("path has no parent: {}", path))?
        };
        let status = Command::new("explorer.exe").arg(&open_target).status()?;
        if !status.success() {
            anyhow::bail!("explorer returned a non-zero status");
        }
    }
    Ok(())
}

fn format_log(summary: &str, stdout: &str, stderr: &str) -> String {
    let mut parts = vec![format!("Summary: {summary}")];
    if !stdout.trim().is_empty() {
        parts.push(format!("stdout:\n{}", stdout.trim()));
    }
    if !stderr.trim().is_empty() {
        parts.push(format!("stderr:\n{}", stderr.trim()));
    }
    parts.join("\n\n")
}
