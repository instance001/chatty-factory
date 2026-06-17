use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::ids::{FamilyId, OperatorId, WrapperId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestMode {
    NewBuild,
    Patch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesiredSurface {
    Web,
    Cli,
    Desktop,
    HelperWeb,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExoskeletonTarget {
    None,
    ChattyCog,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityTransition {
    None,
    HelperFoundation,
    WrapperEmission,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureClass {
    InvalidMetadata,
    MissingExpectedFiles,
    OperatorMismatch,
    DomEligibilityMismatch,
    PolicyViolation,
    UnsupportedFamilyCapability,
    ToolchainMismatch,
    StructuredCodeGenerationMismatch,
    HelperWiringFailure,
    SyntaxFailure,
    BuildFailure,
    RuntimeAssertionFailure,
    RouteSelectionMismatch,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestRecord {
    pub request_id: String,
    pub raw_request: String,
    pub mode: Option<RequestMode>,
    pub active_project: Option<String>,
    pub explicit_stack: Option<String>,
    pub desired_surface: Option<DesiredSurface>,
    pub requested_capabilities: Vec<String>,
    pub exoskeleton_target: Option<ExoskeletonTarget>,
    pub candidate_family_ids: Vec<FamilyId>,
    pub ambiguity_flags: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RouteDecision {
    pub route_id: String,
    pub request_id: String,
    pub selected_family_id: Option<FamilyId>,
    pub selected_operator_ids: Vec<OperatorId>,
    pub selected_wrapper_ids: Vec<WrapperId>,
    pub selected_behavior_kind: Option<String>,
    pub capability_transition: Option<CapabilityTransition>,
    pub decision_reasons: Vec<String>,
    pub fallback_level: Option<String>,
    pub needs_llm_review: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestPlan {
    pub plan_id: String,
    pub request_id: String,
    pub mode: Option<RequestMode>,
    pub interpreted_goal: String,
    pub inferred_family_candidates: Vec<FamilyId>,
    pub inferred_tool_kind: Option<String>,
    pub intended_patch_kind: Option<String>,
    pub available_patch_kinds: Vec<String>,
    #[serde(default)]
    pub planner_patch_recipe_ids: Vec<String>,
    #[serde(default)]
    pub planner_operator_bundle_ids: Vec<String>,
    #[serde(default)]
    pub planner_operator_ids: Vec<String>,
    #[serde(default)]
    pub planner_acceptance_recipe_ids: Vec<String>,
    pub execution_steps: Vec<String>,
    pub constraints: Vec<String>,
    pub rationale: Vec<String>,
    #[serde(default)]
    pub planner_acceptance_checks: Vec<AcceptanceCheck>,
    #[serde(default)]
    pub planner_required_markers: Vec<String>,
    #[serde(default)]
    pub planner_acceptance_commands: Vec<String>,
    #[serde(default)]
    pub planner_expected_outputs: Vec<String>,
    #[serde(default)]
    pub planner_suggested_patch_kinds: Vec<String>,
    #[serde(default)]
    pub planner_suggested_features: Vec<String>,
    pub confidence_score: u8,
    pub confidence_band: String,
    pub escalation_reasons: Vec<String>,
    pub needs_llm_review: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BuildIntentFreeze {
    pub freeze_id: String,
    pub request_id: String,
    pub source_request_plan_id: String,
    pub mode: Option<RequestMode>,
    pub raw_request: String,
    pub project_name: String,
    pub selected_family_id: Option<FamilyId>,
    pub starter_override_id: Option<String>,
    pub recommended_starter_id: Option<String>,
    pub explicit_stack: Option<String>,
    pub desired_surface: Option<DesiredSurface>,
    pub exoskeleton_target: Option<ExoskeletonTarget>,
    pub inferred_tool_kind: Option<String>,
    pub interpreted_goal: String,
    #[serde(default)]
    pub candidate_family_ids: Vec<FamilyId>,
    #[serde(default)]
    pub requested_capabilities: Vec<String>,
    #[serde(default)]
    pub hard_requirements: Vec<String>,
    #[serde(default)]
    pub preference_notes: Vec<String>,
    #[serde(default)]
    pub ambiguity_flags: Vec<String>,
    #[serde(default)]
    pub constraint_notes: Vec<String>,
    #[serde(default)]
    pub freeze_notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BuildFeatureSlice {
    pub slice_id: String,
    #[serde(default)]
    pub slice_kind: String,
    pub title: String,
    pub summary: String,
    #[serde(default)]
    pub why_it_exists: String,
    #[serde(default)]
    pub requested_capabilities: Vec<String>,
    #[serde(default)]
    pub planner_suggested_features: Vec<String>,
    #[serde(default)]
    pub execution_steps: Vec<String>,
    #[serde(default)]
    pub acceptance_targets: Vec<String>,
    #[serde(default)]
    pub files_to_create: Vec<String>,
    #[serde(default)]
    pub files_to_update: Vec<String>,
    #[serde(default)]
    pub expected_symbols: Vec<String>,
    #[serde(default)]
    pub acceptance_markers: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlannedFileOperation {
    pub operation_id: String,
    pub path: String,
    pub operation_kind: String,
    pub rationale: String,
    pub source: String,
    #[serde(default)]
    pub content_source: String,
    #[serde(default)]
    pub target_anchor: Option<String>,
    #[serde(default)]
    pub ownership_boundary: String,
    #[serde(default)]
    pub syntax_sensitive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BuildPlanArtifact {
    pub build_plan_id: String,
    pub request_id: String,
    pub source_request_plan_id: String,
    pub source_build_intent_freeze_id: String,
    pub project_name: String,
    pub family_id: Option<FamilyId>,
    pub starter_override_id: Option<String>,
    pub recommended_starter_id: Option<String>,
    pub desired_surface: Option<DesiredSurface>,
    pub exoskeleton_target: Option<ExoskeletonTarget>,
    pub tool_kind: Option<String>,
    pub interpreted_goal: String,
    #[serde(default)]
    pub feature_slices: Vec<BuildFeatureSlice>,
    #[serde(default)]
    pub planned_file_operations: Vec<PlannedFileOperation>,
    #[serde(default)]
    pub acceptance_targets: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub rationale: Vec<String>,
    #[serde(default)]
    pub route_decision_reasons: Vec<String>,
    pub confidence_score: u8,
    pub confidence_band: String,
    pub needs_llm_review: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BuildPlanReview {
    pub review_id: String,
    pub request_id: String,
    pub build_intent_freeze_id: String,
    pub source_build_plan_id: String,
    pub project_name: String,
    pub family_id: Option<FamilyId>,
    pub starter_override_id: Option<String>,
    pub recommended_starter_id: Option<String>,
    pub decision: String,
    pub original_feature_slice_count: usize,
    pub reviewed_feature_slice_count: usize,
    #[serde(default)]
    pub dropped_feature_slice_ids: Vec<String>,
    pub original_file_operation_count: usize,
    pub reviewed_file_operation_count: usize,
    #[serde(default)]
    pub dropped_file_operation_ids: Vec<String>,
    #[serde(default)]
    pub findings: Vec<String>,
    #[serde(default)]
    pub blocked_reasons: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BuildConstraintReviewReceipt {
    pub review_id: String,
    pub request_id: String,
    pub build_plan_id: String,
    pub project_name: String,
    pub family_id: Option<FamilyId>,
    pub tool_kind: Option<String>,
    pub review_subject: String,
    #[serde(default)]
    pub selected_constraints: Vec<ImplementationConstraint>,
    #[serde(default)]
    pub violations: Vec<ConstraintViolation>,
    #[serde(default)]
    pub blocked_methods: Vec<String>,
    #[serde(default)]
    pub recommended_replacements: Vec<String>,
    pub decision: String,
    #[serde(default)]
    pub findings: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BuildExecutionWorkOrder {
    pub work_order_id: String,
    pub request_id: String,
    pub build_intent_freeze_id: String,
    pub build_plan_id: String,
    pub build_plan_review_id: String,
    pub build_constraint_review_id: String,
    pub project_name: String,
    pub family_id: Option<FamilyId>,
    pub tool_kind: Option<String>,
    pub starter_override_id: Option<String>,
    pub decision: String,
    #[serde(default)]
    pub feature_slice_ids: Vec<String>,
    #[serde(default)]
    pub feature_capabilities: Vec<String>,
    #[serde(default)]
    pub operations: Vec<PlannedFileOperation>,
    #[serde(default)]
    pub findings: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlanTask {
    pub task_id: String,
    pub request_id: String,
    pub source_build_plan_id: String,
    pub source_work_order_id: String,
    pub task_kind: String,
    pub task_title: String,
    pub task_summary: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub target_files: Vec<String>,
    #[serde(default)]
    pub allowed_boundaries: Vec<String>,
    #[serde(default)]
    pub expected_symbols: Vec<String>,
    #[serde(default)]
    pub expected_markers: Vec<String>,
    #[serde(default)]
    pub verification_steps: Vec<String>,
    pub replacement_guidance: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlanTaskList {
    pub task_list_id: String,
    pub request_id: String,
    pub build_plan_id: String,
    pub build_plan_review_id: String,
    pub build_constraint_review_id: String,
    pub build_work_order_id: String,
    pub project_name: String,
    pub family_id: Option<FamilyId>,
    pub tool_kind: Option<String>,
    #[serde(default)]
    pub tasks: Vec<PlanTask>,
    #[serde(default)]
    pub findings: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlanTaskExecutionReceipt {
    pub receipt_id: String,
    pub request_id: String,
    pub task_id: String,
    pub task_kind: String,
    pub status: String,
    #[serde(default)]
    pub touched_files: Vec<String>,
    #[serde(default)]
    pub findings: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlanTaskExecutionLog {
    pub log_id: String,
    pub request_id: String,
    pub task_list_id: String,
    pub decision: String,
    #[serde(default)]
    pub receipts: Vec<PlanTaskExecutionReceipt>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlanTaskVerificationReceipt {
    pub receipt_id: String,
    pub request_id: String,
    pub task_id: String,
    pub status: String,
    #[serde(default)]
    pub verification_steps: Vec<String>,
    #[serde(default)]
    pub findings: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlanTaskVerificationLog {
    pub log_id: String,
    pub request_id: String,
    pub task_list_id: String,
    pub decision: String,
    #[serde(default)]
    pub receipts: Vec<PlanTaskVerificationReceipt>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlanTaskModelAttemptReceipt {
    pub attempt_id: String,
    pub request_id: String,
    pub task_id: String,
    pub project_name: String,
    pub target_file: Option<String>,
    pub prompt_path: String,
    pub generation_receipt_path: Option<String>,
    pub decomposition_receipt_path: Option<String>,
    pub raw_response_path: Option<String>,
    pub model_path: Option<String>,
    pub status: String,
    #[serde(default)]
    pub review_findings: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskDecompositionReceipt {
    pub decomposition_id: String,
    pub request_id: String,
    pub task_id: String,
    pub project_name: String,
    pub task_shape: Option<String>,
    pub task_subtype: String,
    #[serde(default)]
    pub constraint_principles: Vec<String>,
    pub matched_grammar: Option<String>,
    pub trigger_class: String,
    pub decision: String,
    pub source_generation_receipt_path: Option<String>,
    #[serde(default)]
    pub findings: Vec<String>,
    #[serde(default)]
    pub recommended_child_tasks: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskDecompositionProposal {
    pub proposal_id: String,
    pub request_id: String,
    pub task_id: String,
    pub project_name: String,
    pub task_shape: String,
    pub task_subtype: String,
    pub decomposition_pattern: String,
    #[serde(default)]
    pub constraint_principles: Vec<String>,
    #[serde(default)]
    pub required_fields: Vec<String>,
    #[serde(default)]
    pub proposed_child_tasks: Vec<String>,
    pub proposed_host_composition_task: Option<String>,
    pub confidence_posture: String,
    #[serde(default)]
    pub findings: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskDecompositionInferenceReceipt {
    pub inference_id: String,
    pub request_id: String,
    pub task_id: String,
    pub project_name: String,
    pub task_shape: String,
    pub task_subtype: String,
    pub inference_pattern: String,
    #[serde(default)]
    pub failure_class: String,
    #[serde(default)]
    pub constraint_principles: Vec<String>,
    pub trigger_class: String,
    pub decision: String,
    pub source_generation_receipt_path: Option<String>,
    pub proposal_path: Option<String>,
    #[serde(default)]
    pub findings: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AtomizationFloorDecision {
    pub decision_id: String,
    pub request_id: String,
    pub task_id: String,
    pub project_name: String,
    pub task_shape: Option<String>,
    pub task_subtype: Option<String>,
    pub current_granularity: String,
    pub decision: String,
    #[serde(default)]
    pub alternate_methods: Vec<String>,
    #[serde(default)]
    pub findings: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FailureVaultEntry {
    pub vault_entry_id: String,
    pub request_id: String,
    pub task_id: String,
    pub project_name: String,
    pub task_shape: Option<String>,
    pub task_subtype: Option<String>,
    pub task_kind: String,
    pub failure_class: String,
    pub trigger_class: Option<String>,
    pub triangulation_session_id: String,
    pub atomization_floor_decision_path: Option<String>,
    pub attempt_method: Option<String>,
    pub source_attempt_receipt_path: String,
    pub source_decomposition_receipt_path: Option<String>,
    pub status: String,
    pub decomposition_depth: usize,
    #[serde(default)]
    pub narrow_usage_pattern: Vec<String>,
    #[serde(default)]
    pub findings: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TriangulationAttempt {
    pub attempt_id: String,
    pub task_id: String,
    pub task_subtype: Option<String>,
    pub attempt_method: String,
    pub outcome: String,
    pub failure_class: Option<String>,
    pub source_attempt_receipt_path: Option<String>,
    pub source_decomposition_receipt_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TriangulationSession {
    pub session_id: String,
    pub request_id: String,
    pub project_name: String,
    pub task_shape: Option<String>,
    pub task_subtype: Option<String>,
    pub task_lineage_key: String,
    pub status: String,
    pub convergence_posture: String,
    pub atomization_floor_decision_path: Option<String>,
    pub successful_alternate_method: bool,
    #[serde(default)]
    pub attempts: Vec<TriangulationAttempt>,
    #[serde(default)]
    pub findings: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConstraintPromotionCandidate {
    pub candidate_id: String,
    pub request_id: String,
    pub triangulation_session_id: String,
    pub project_name: String,
    pub task_shape: Option<String>,
    pub task_subtype: Option<String>,
    pub failure_class: String,
    pub trigger_class: Option<String>,
    pub confidence_posture: String,
    pub status: String,
    #[serde(default)]
    pub matched_constraint_principles: Vec<String>,
    #[serde(default)]
    pub narrow_usage_pattern: Vec<String>,
    #[serde(default)]
    pub evidence_receipt_paths: Vec<String>,
    #[serde(default)]
    pub findings: Vec<String>,
    pub recommended_constraint_summary: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlannerHandoff {
    pub handoff_id: String,
    pub request_id: String,
    pub source_plan_id: String,
    pub mode: Option<RequestMode>,
    pub active_project: Option<String>,
    pub interpreted_goal: String,
    pub inferred_family_candidates: Vec<FamilyId>,
    pub inferred_tool_kind: Option<String>,
    pub available_patch_kinds: Vec<String>,
    #[serde(default)]
    pub candidate_request_modes: Vec<String>,
    #[serde(default)]
    pub candidate_active_projects: Vec<String>,
    #[serde(default)]
    pub candidate_active_project_summaries: Vec<String>,
    #[serde(default)]
    pub candidate_patch_recipe_ids: Vec<String>,
    #[serde(default)]
    pub candidate_composition_patch_kinds: Vec<String>,
    #[serde(default)]
    pub candidate_composition_patch_primitive_classes: Vec<String>,
    #[serde(default)]
    pub candidate_composition_family_build_primitive_classes: Vec<String>,
    #[serde(default)]
    pub candidate_composition_layers: Vec<String>,
    #[serde(default)]
    pub candidate_composition_helper_primitive_ids: Vec<String>,
    #[serde(default)]
    pub candidate_composition_helper_primitive_kinds: Vec<String>,
    #[serde(default)]
    pub candidate_composition_adapter_semantics: Vec<String>,
    #[serde(default)]
    pub candidate_operator_bundle_ids: Vec<String>,
    #[serde(default)]
    pub candidate_acceptance_recipe_ids: Vec<String>,
    pub rationale: Vec<String>,
    pub escalation_reasons: Vec<String>,
    pub requested_output: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlannerResponse {
    pub response_id: String,
    pub source_handoff_id: String,
    pub source_plan_id: String,
    pub approved: bool,
    pub recommended_request_mode: Option<String>,
    pub recommended_active_project: Option<String>,
    pub recommended_family_id: Option<FamilyId>,
    pub recommended_tool_kind: Option<String>,
    pub recommended_patch_kind: Option<String>,
    #[serde(default)]
    pub recommended_patch_recipe_ids: Vec<String>,
    #[serde(default)]
    pub recommended_composition_patch_kinds: Vec<String>,
    #[serde(default)]
    pub recommended_composition_patch_primitive_classes: Vec<String>,
    #[serde(default)]
    pub recommended_composition_family_build_primitive_classes: Vec<String>,
    #[serde(default)]
    pub recommended_composition_layers: Vec<String>,
    #[serde(default)]
    pub recommended_composition_helper_primitive_ids: Vec<String>,
    #[serde(default)]
    pub recommended_composition_helper_primitive_kinds: Vec<String>,
    #[serde(default)]
    pub recommended_operator_bundle_ids: Vec<String>,
    pub recommended_operator_ids: Vec<String>,
    #[serde(default)]
    pub recommended_acceptance_recipe_ids: Vec<String>,
    pub rationale: Vec<String>,
    pub execution_steps: Vec<String>,
    pub acceptance_notes: Vec<String>,
    #[serde(default)]
    pub acceptance_checks_to_add: Vec<AcceptanceCheck>,
    #[serde(default)]
    pub required_markers_to_add: Vec<String>,
    #[serde(default)]
    pub acceptance_commands_to_add: Vec<String>,
    #[serde(default)]
    pub expected_outputs_to_add: Vec<String>,
    #[serde(default)]
    pub suggested_patch_kinds: Vec<String>,
    #[serde(default)]
    pub suggested_features: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ScaffoldInputs {
    pub family_id: Option<FamilyId>,
    pub project_name: String,
    pub title: String,
    pub summary: String,
    pub copy_bundle: Vec<String>,
    pub feature_tokens: Vec<String>,
    pub style_preset: Option<String>,
    pub wrapper_target: Option<ExoskeletonTarget>,
    pub entrypoint_config: Vec<String>,
    pub fixture_config: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AcceptanceCheck {
    pub check_id: String,
    pub kind: String,
    pub target: String,
    pub expected: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AcceptancePlan {
    pub acceptance_id: String,
    pub request_id: String,
    pub family_id: Option<FamilyId>,
    pub checks: Vec<AcceptanceCheck>,
    pub required_files: Vec<String>,
    pub required_markers: Vec<String>,
    pub commands: Vec<String>,
    pub expected_outputs: Vec<String>,
    pub helper_checks: Vec<String>,
    pub schema_checks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FailureReportEvidence {
    pub summary: String,
    pub details: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FailureReport {
    pub failure_id: String,
    pub request_id: String,
    pub family_id: Option<FamilyId>,
    pub failure_class: Option<FailureClass>,
    pub failing_check_id: Option<String>,
    pub evidence: Option<FailureReportEvidence>,
    pub recommended_lane: Option<String>,
    pub needs_llm_review: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProjectSpec {
    pub spec_id: String,
    pub project_name: String,
    pub family_id: Option<FamilyId>,
    pub substrate: String,
    pub tool_kind: Option<String>,
    pub request_summary: Option<String>,
    pub entrypoints: Vec<String>,
    pub expected_files: Vec<String>,
    pub features: Vec<String>,
    pub acceptance_commands: Vec<String>,
    pub acceptance_checks: Vec<AcceptanceCheck>,
    pub wrapper_metadata: Vec<String>,
    pub chattycog_hosting_mode: Option<String>,
    pub chattycog_ui_owner: Option<String>,
    pub chattycog_bridge_capabilities: Option<ChattyCogBridgeCapabilities>,
    #[serde(default)]
    pub helper_services: Vec<HelperServiceSpec>,
    #[serde(default)]
    pub supported_patch_kinds: Vec<String>,
    #[serde(default)]
    pub patch_lanes: Vec<PatchLaneStatus>,
    #[serde(default)]
    pub acceptance_recipes: Vec<AcceptanceRecipeStatus>,
    #[serde(default)]
    pub operator_bundles: Vec<OperatorBundleStatus>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HelperLaunchPolicy {
    pub helper_id: String,
    pub helper_kind: String,
    pub allowed_root: String,
    pub program: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub working_directory: Option<String>,
    #[serde(default)]
    pub status_paths: Vec<String>,
    #[serde(default)]
    pub expected_files: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HelperServiceSpec {
    pub helper_spec_id: String,
    pub helper_id: String,
    pub helper_kind: String,
    pub attached_family_id: Option<FamilyId>,
    pub attached_tool_kind: Option<String>,
    pub attached_project_name: Option<String>,
    pub purpose: String,
    pub entrypoint: String,
    pub working_directory: String,
    #[serde(default)]
    pub input_paths: Vec<String>,
    #[serde(default)]
    pub output_paths: Vec<String>,
    #[serde(default)]
    pub status_paths: Vec<String>,
    pub launch_policy: Option<HelperLaunchPolicy>,
    #[serde(default)]
    pub allowed_extensions: Vec<String>,
    #[serde(default)]
    pub lane_allowed_extensions: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub expected_files: Vec<String>,
    #[serde(default)]
    pub primitives: Vec<HelperPrimitiveSpec>,
    #[serde(default)]
    pub notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HelperPrimitiveSpec {
    pub primitive_id: String,
    pub primitive_kind: String,
    pub purpose: String,
    #[serde(default)]
    pub input_paths: Vec<String>,
    #[serde(default)]
    pub output_paths: Vec<String>,
    #[serde(default)]
    pub status_paths: Vec<String>,
    pub dependency_mode: String,
    #[serde(default)]
    pub requires_primitives: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HelperStatusSnapshot {
    pub helper_id: String,
    pub helper_kind: String,
    pub status: String,
    pub summary: String,
    #[serde(default)]
    pub observed_inputs: Vec<String>,
    #[serde(default)]
    pub observed_outputs: Vec<String>,
    #[serde(default)]
    pub observed_status_files: Vec<String>,
    #[serde(default)]
    pub observed_primitive_ids: Vec<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HelperRuntimeReceipt {
    pub receipt_id: String,
    pub request_id: String,
    pub helper_id: String,
    pub helper_kind: String,
    pub attached_project_name: Option<String>,
    pub launch_status: String,
    pub process_id: Option<u32>,
    #[serde(default)]
    pub observed_status_files: Vec<String>,
    #[serde(default)]
    pub observed_output_files: Vec<String>,
    #[serde(default)]
    pub observed_primitive_ids: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HelperAcceptancePlan {
    pub helper_id: String,
    #[serde(default)]
    pub required_files: Vec<String>,
    #[serde(default)]
    pub required_status_paths: Vec<String>,
    #[serde(default)]
    pub required_markers: Vec<String>,
    #[serde(default)]
    pub commands: Vec<String>,
    #[serde(default)]
    pub checks: Vec<AcceptanceCheck>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PatchLaneStatus {
    pub recipe_id: String,
    pub patch_kind: String,
    pub dependency_mode: String,
    #[serde(default)]
    pub requires_features: Vec<String>,
    #[serde(default)]
    pub provides_features: Vec<String>,
    pub availability_status: String,
    #[serde(default)]
    pub surgical_maturity: String,
    #[serde(default)]
    pub superseded_by_patch_kinds: Vec<String>,
    #[serde(default)]
    pub effective_preflight_readiness: String,
    #[serde(default)]
    pub preflight_readiness_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AcceptanceRecipeStatus {
    pub recipe_id: String,
    pub feature_id: String,
    pub command_id: String,
    pub availability_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OperatorBundleStatus {
    pub bundle_id: String,
    #[serde(default)]
    pub operator_ids: Vec<String>,
    #[serde(default)]
    pub provides_features: Vec<String>,
    pub availability_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BuildReceipt {
    pub receipt_id: String,
    pub request_id: String,
    pub family_id: Option<FamilyId>,
    pub starter_override_id: Option<String>,
    pub starter_override_summary: Option<String>,
    pub recommended_starter_id: Option<String>,
    pub recommended_starter_summary: Option<String>,
    pub starter_recommendation_comparison: Option<String>,
    pub project_name: String,
    pub project_dir: String,
    pub tool_kind: Option<String>,
    pub emitted_files: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProjectCatalogEntry {
    pub project_name: String,
    pub family_id: Option<FamilyId>,
    pub tool_kind: Option<String>,
    pub request_summary: Option<String>,
    pub recency_hint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProjectSession {
    pub project_name: String,
    pub family_id: Option<FamilyId>,
    pub tool_kind: Option<String>,
    pub request_summary: Option<String>,
    pub last_action: String,
    #[serde(default)]
    pub source_kind: String,
    pub source_request_id: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProjectBrowserState {
    pub state_id: String,
    pub selected_project_session: Option<ProjectSession>,
    pub active_project_session: Option<ProjectSession>,
    pub projects: Vec<ProjectCatalogEntry>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PatchReceipt {
    pub patch_id: String,
    pub request_id: String,
    pub family_id: Option<FamilyId>,
    pub project_name: String,
    pub patch_kind: String,
    pub request_summary: String,
    pub modified_files: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProjectPatchDiagnosis {
    pub diagnosis_id: String,
    pub request_id: String,
    pub project_name: String,
    pub family_id: Option<FamilyId>,
    pub tool_kind: Option<String>,
    pub substrate: String,
    pub request_summary: String,
    #[serde(default)]
    pub entrypoints: Vec<String>,
    #[serde(default)]
    pub expected_files: Vec<String>,
    #[serde(default)]
    pub candidate_target_files: Vec<String>,
    #[serde(default)]
    pub candidate_insertion_points: Vec<String>,
    #[serde(default)]
    pub diagnosed_artifact_groups: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub declared_surface_groups: Vec<String>,
    #[serde(default)]
    pub patch_surgical_maturity: Option<String>,
    #[serde(default)]
    pub expected_anchor_markers: Vec<String>,
    #[serde(default)]
    pub present_anchor_markers: Vec<String>,
    #[serde(default)]
    pub structural_guard_source: Option<String>,
    #[serde(default)]
    pub conflicting_anchor_markers: Vec<String>,
    #[serde(default)]
    pub present_conflicting_anchor_markers: Vec<String>,
    #[serde(default)]
    pub preserve_invariants: Vec<String>,
    #[serde(default)]
    pub declared_ownership_boundaries: Vec<String>,
    #[serde(default)]
    pub project_structure_notes: Vec<String>,
    #[serde(default)]
    pub risk_notes: Vec<String>,
    #[serde(default)]
    pub pre_patch_artifact_hashes: BTreeMap<String, String>,
    #[serde(default)]
    pub post_patch_checks: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PatchIntentFreeze {
    pub freeze_id: String,
    pub request_id: String,
    pub project_name: String,
    pub family_id: Option<FamilyId>,
    pub tool_kind: Option<String>,
    pub interpreted_goal: String,
    pub intended_patch_kind: Option<String>,
    #[serde(default)]
    pub candidate_patch_kinds: Vec<String>,
    pub diagnosis_id: String,
    #[serde(default)]
    pub diagnosed_target_files: Vec<String>,
    #[serde(default)]
    pub diagnosed_insertion_points: Vec<String>,
    #[serde(default)]
    pub declared_surface_groups: Vec<String>,
    #[serde(default)]
    pub confirmed_surface_groups: Vec<String>,
    #[serde(default)]
    pub patch_surgical_maturity: Option<String>,
    #[serde(default)]
    pub superseded_by_patch_kinds: Vec<String>,
    #[serde(default)]
    pub structural_guard_source: Option<String>,
    #[serde(default)]
    pub required_anchor_markers: Vec<String>,
    #[serde(default)]
    pub confirmed_anchor_markers: Vec<String>,
    #[serde(default)]
    pub conflicting_anchor_markers: Vec<String>,
    #[serde(default)]
    pub present_conflicting_anchor_markers: Vec<String>,
    #[serde(default)]
    pub preserve_invariants: Vec<String>,
    #[serde(default)]
    pub declared_ownership_boundaries: Vec<String>,
    #[serde(default)]
    pub risk_notes: Vec<String>,
    #[serde(default)]
    pub contract_confidence_summary: Option<String>,
    #[serde(default)]
    pub replacement_guidance_summary: Option<String>,
    #[serde(default)]
    pub freeze_notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PatchPlanReview {
    pub review_id: String,
    pub request_id: String,
    pub project_name: String,
    pub diagnosis_id: String,
    pub freeze_id: String,
    pub original_intended_patch_kind: Option<String>,
    pub reviewed_intended_patch_kind: Option<String>,
    pub decision: String,
    #[serde(default)]
    pub promoted_candidate_patch_kinds: Vec<String>,
    #[serde(default)]
    pub original_target_files: Vec<String>,
    #[serde(default)]
    pub reviewed_target_files: Vec<String>,
    #[serde(default)]
    pub dropped_target_files: Vec<String>,
    #[serde(default)]
    pub original_insertion_points: Vec<String>,
    #[serde(default)]
    pub reviewed_insertion_points: Vec<String>,
    #[serde(default)]
    pub recommended_replacement_patch_kinds: Vec<String>,
    #[serde(default)]
    pub reviewed_replacement_bundle_patch_kinds: Vec<String>,
    #[serde(default)]
    pub reviewed_replacement_bundle_status: Option<String>,
    #[serde(default)]
    pub findings: Vec<String>,
    #[serde(default)]
    pub blocked_reasons: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ImplementationConstraint {
    pub constraint_id: String,
    pub constraint_scope: String,
    #[serde(default)]
    pub constraint_origin: String,
    pub family_id: Option<FamilyId>,
    pub tool_kind: Option<String>,
    pub language_id: Option<String>,
    pub constraint_kind: String,
    pub forbidden_method_summary: String,
    #[serde(default)]
    pub forbidden_markers: Vec<String>,
    #[serde(default)]
    pub required_markers: Vec<String>,
    #[serde(default)]
    pub forbidden_surface_groups: Vec<String>,
    pub violation_reason_template: String,
    pub replacement_guidance: Option<String>,
    pub severity: String,
    pub active: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConstraintViolation {
    pub constraint_id: String,
    pub constraint_kind: String,
    pub scope: String,
    #[serde(default)]
    pub matched_markers: Vec<String>,
    #[serde(default)]
    pub missing_required_markers: Vec<String>,
    #[serde(default)]
    pub violated_surface_groups: Vec<String>,
    pub reason: String,
    pub replacement_guidance: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConstraintReviewReceipt {
    pub review_id: String,
    pub request_id: String,
    pub project_name: String,
    pub family_id: Option<FamilyId>,
    pub tool_kind: Option<String>,
    pub review_subject: String,
    pub original_intended_patch_kind: Option<String>,
    pub reviewed_intended_patch_kind: Option<String>,
    #[serde(default)]
    pub selected_constraints: Vec<ImplementationConstraint>,
    #[serde(default)]
    pub violations: Vec<ConstraintViolation>,
    #[serde(default)]
    pub surviving_patch_kinds: Vec<String>,
    #[serde(default)]
    pub surviving_composition_patch_kinds: Vec<String>,
    #[serde(default)]
    pub blocked_methods: Vec<String>,
    #[serde(default)]
    pub recommended_replacements: Vec<String>,
    pub decision: String,
    #[serde(default)]
    pub findings: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildVerificationReceipt {
    pub verification_id: String,
    pub request_id: String,
    pub review_subject: String,
    pub interpreted_goal: String,
    #[serde(default)]
    pub candidate_family_ids: Vec<FamilyId>,
    pub suggested_family_id: Option<FamilyId>,
    pub suggested_tool_kind: Option<String>,
    pub suggested_extension_kind: String,
    pub failure_class: FailureClass,
    #[serde(default)]
    pub normalized_failure_class: String,
    #[serde(default)]
    pub recommended_next_action: String,
    pub failure_mode: String,
    #[serde(default)]
    pub matched_approved_constraint_ids: Vec<String>,
    #[serde(default)]
    pub matched_approved_constraint_summaries: Vec<String>,
    #[serde(default)]
    pub reasons: Vec<String>,
    #[serde(default)]
    pub blocked_methods: Vec<String>,
    #[serde(default)]
    pub findings: Vec<String>,
    #[serde(default)]
    pub recommended_next_step: String,
    pub decision: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProposedConstraintReceipt {
    pub proposal_id: String,
    pub request_id: String,
    pub source_verification_id: String,
    pub status: String,
    pub rationale: Vec<String>,
    pub proposed_constraint: ImplementationConstraint,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ApprovedConstraintShelf {
    pub shelf_id: String,
    #[serde(default)]
    pub constraints: Vec<ImplementationConstraint>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConstraintShelfHistoryEntry {
    pub constraint: ImplementationConstraint,
    pub archived_reason: String,
    pub archived_from_shelf_id: Option<String>,
    #[serde(default)]
    pub archived_match_count: usize,
    pub archived_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConstraintShelfHistory {
    pub history_id: String,
    #[serde(default)]
    pub archived_constraints: Vec<ConstraintShelfHistoryEntry>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintApprovalReceipt {
    pub approval_id: String,
    pub request_id: String,
    pub proposal_id: String,
    #[serde(default)]
    pub proposal_origin: String,
    #[serde(default)]
    pub proposal_source_id: String,
    pub approved_constraint_id: String,
    pub status: String,
    pub shelf_path: String,
    pub proposal_path: String,
    pub rationale: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintShelfMutationReceipt {
    pub mutation_id: String,
    pub constraint_id: String,
    pub action: String,
    #[serde(default)]
    pub proposal_origin: Option<String>,
    #[serde(default)]
    pub proposal_source_id: Option<String>,
    pub shelf_path: String,
    pub status: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyCapabilityManifest {
    pub family_id: FamilyId,
    pub primary_substrate: String,
    pub supports_chattycog_wrapper: bool,
    #[serde(default = "default_family_lifecycle_status")]
    pub lifecycle_status: String,
    #[serde(default)]
    pub lifecycle_notes: Vec<String>,
    #[serde(default)]
    pub supported_stack_ids: Vec<String>,
    #[serde(default)]
    pub provided_build_primitive_classes: Vec<String>,
    #[serde(default)]
    pub primitive_adapters: Vec<FamilyPrimitiveAdapter>,
    pub explicit_stack_keywords: Vec<String>,
    pub route_keywords: Vec<String>,
    pub supported_tool_kinds: Vec<String>,
    #[serde(default)]
    pub forbids_capabilities: Vec<String>,
    #[serde(default)]
    pub requires_helper_for: Vec<String>,
}

fn default_family_lifecycle_status() -> String {
    "active".into()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FamilyPrimitiveAdapter {
    pub adapter_id: String,
    pub composition_layer: String,
    pub primitive_name: String,
    pub adapter_kind: String,
    pub support_level: String,
    #[serde(default)]
    pub requires_helpers: Vec<String>,
    #[serde(default)]
    pub requires_primitives: Vec<String>,
    #[serde(default)]
    pub companion_primitives: Vec<String>,
    pub execution_hint: Option<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RuntimeConfig {
    pub config_id: String,
    pub runtime_root: String,
    pub models_root: String,
    pub server_executable: String,
    pub cli_executable: String,
    pub default_model_path: Option<String>,
    pub host: String,
    pub port: u16,
    pub context_size: u32,
    pub gpu_layers: i32,
    pub launch_timeout_secs: u64,
    pub planner_request_timeout_secs: u64,
    pub model_task_request_timeout_secs: u64,
    pub shell_timeout_buffer_secs: u64,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DiscoveredModel {
    pub file_name: String,
    pub path: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RuntimeCapabilityRecord {
    pub backend_kind: String,
    pub vulkan_available: bool,
    pub server_available: bool,
    pub cli_available: bool,
    pub discovered_model_count: usize,
    pub preferred_model_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RuntimeDiscoveryReceipt {
    pub discovery_id: String,
    pub runtime_root: String,
    pub models_root: String,
    pub server_executable: String,
    pub cli_executable: String,
    pub vulkan_backend_present: bool,
    pub server_version_output: String,
    pub cli_version_output: String,
    pub discovered_models: Vec<DiscoveredModel>,
    pub preferred_model_path: Option<String>,
    pub planner_runtime_capability: RuntimeCapabilityRecord,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RuntimeModelAssessment {
    pub file_name: String,
    pub path: String,
    pub size_bytes: u64,
    pub planner_profile: String,
    pub suitability_tags: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RuntimeModelCatalogReceipt {
    pub catalog_id: String,
    pub models_root: String,
    pub models: Vec<RuntimeModelAssessment>,
    pub preferred_fast_model_path: Option<String>,
    pub preferred_balanced_model_path: Option<String>,
    pub preferred_heavy_model_path: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RuntimeSmokeReceipt {
    pub smoke_id: String,
    pub config_id: String,
    pub model_path: Option<String>,
    pub launch_args: Vec<String>,
    pub launch_timeout_secs: u64,
    pub version_probe_ok: bool,
    pub server_launch_attempted: bool,
    pub server_started: bool,
    pub http_probe_ok: bool,
    pub process_killed: bool,
    pub timeout_cause: Option<String>,
    pub final_outcome: Option<String>,
    pub launch_elapsed_ms: Option<u64>,
    pub total_elapsed_ms: Option<u64>,
    pub stdout_log_path: Option<String>,
    pub stderr_log_path: Option<String>,
    pub notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlannerExecutionReceipt {
    pub execution_id: String,
    pub config_id: String,
    pub source_handoff_id: String,
    pub model_path: String,
    pub launch_args: Vec<String>,
    pub launch_timeout_secs: u64,
    pub request_timeout_secs: u64,
    pub response_path: Option<String>,
    pub raw_response_path: Option<String>,
    pub server_started: bool,
    pub http_request_ok: bool,
    pub process_killed: bool,
    pub timeout_cause: Option<String>,
    pub final_outcome: Option<String>,
    pub launch_elapsed_ms: Option<u64>,
    pub request_elapsed_ms: Option<u64>,
    pub total_elapsed_ms: Option<u64>,
    pub parse_mode: Option<String>,
    pub finish_reason: Option<String>,
    pub degraded_recovery_used: bool,
    pub should_escalate: bool,
    pub notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ModelTaskGenerationReceipt {
    pub execution_id: String,
    pub request_id: String,
    pub task_id: String,
    pub model_path: String,
    pub launch_args: Vec<String>,
    pub launch_timeout_secs: u64,
    pub request_timeout_secs: u64,
    pub raw_response_path: Option<String>,
    pub server_started: bool,
    pub http_request_ok: bool,
    pub process_killed: bool,
    pub timeout_cause: Option<String>,
    pub final_outcome: Option<String>,
    pub launch_elapsed_ms: Option<u64>,
    pub request_elapsed_ms: Option<u64>,
    pub total_elapsed_ms: Option<u64>,
    pub finish_reason: Option<String>,
    pub response_content_mode: Option<String>,
    pub content_present: bool,
    pub reasoning_content_present: bool,
    pub notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlannerDispatchReceipt {
    pub dispatch_id: String,
    pub source_handoff_id: String,
    pub source_plan_id: String,
    pub requested_model_selector: Option<String>,
    pub attempted_model_paths: Vec<String>,
    pub successful_model_path: Option<String>,
    pub fallback_used: bool,
    pub degraded_response_used: bool,
    pub final_response_path: Option<String>,
    pub run_receipt_paths: Vec<String>,
    pub notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ExecutionPolicy {
    pub policy_id: String,
    pub request_id: String,
    pub family_id: Option<FamilyId>,
    pub project_dir: String,
    pub allowed_root: String,
    pub allowed_entrypoints: Vec<String>,
    pub allowed_commands: Vec<String>,
    pub substrate_smoke_checks: Vec<String>,
    pub notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ExecutionSmokeCheck {
    pub check_id: String,
    pub kind: String,
    pub status: String,
    pub summary: String,
    pub details: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ExecutionReceipt {
    pub receipt_id: String,
    pub request_id: String,
    pub policy_id: String,
    pub family_id: Option<FamilyId>,
    pub project_dir: String,
    pub status: String,
    pub smoke_checks: Vec<ExecutionSmokeCheck>,
    pub notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProjectSnapshot {
    pub snapshot_id: String,
    pub project_name: String,
    pub project_dir: String,
    pub family_id: Option<FamilyId>,
    pub tool_kind: Option<String>,
    pub entrypoints: Vec<String>,
    pub expected_files: Vec<String>,
    pub discovered_files: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ContextBundle {
    pub context_id: String,
    pub request_id: String,
    pub project_name: String,
    pub family_id: Option<FamilyId>,
    pub tool_kind: Option<String>,
    pub request_summary: Option<String>,
    pub entrypoints: Vec<String>,
    pub expected_files: Vec<String>,
    pub snapshot_file_count: usize,
    pub snapshot_preview: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SnapshotGateResult {
    pub gate_id: String,
    pub request_id: String,
    pub project_name: String,
    pub status: String,
    pub checked_paths: Vec<String>,
    pub missing_paths: Vec<String>,
    pub rationale: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CompositionRouteClass {
    DirectDeterministicLane,
    BoundedCompositionCandidate,
    HelperNeededMissing,
    #[default]
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ComposableRoutePlan {
    pub composition_plan_id: String,
    pub request_id: String,
    pub route_class: CompositionRouteClass,
    pub mode: Option<RequestMode>,
    pub active_project: Option<String>,
    pub interpreted_goal: String,
    pub target_family_id: Option<FamilyId>,
    pub target_tool_kind: Option<String>,
    pub target_patch_kind: Option<String>,
    #[serde(default)]
    pub candidate_family_ids: Vec<FamilyId>,
    #[serde(default)]
    pub helper_ids: Vec<String>,
    #[serde(default)]
    pub helper_primitive_ids: Vec<String>,
    #[serde(default)]
    pub helper_primitive_kinds: Vec<String>,
    #[serde(default)]
    pub bridge_capabilities: Vec<String>,
    #[serde(default)]
    pub operator_bundle_ids: Vec<String>,
    #[serde(default)]
    pub acceptance_recipe_ids: Vec<String>,
    #[serde(default)]
    pub selected_patch_kinds: Vec<String>,
    #[serde(default)]
    pub patch_primitive_classes: Vec<String>,
    #[serde(default)]
    pub family_build_primitive_classes: Vec<String>,
    #[serde(default)]
    pub composition_layers: Vec<String>,
    #[serde(default)]
    pub composition_work_order_kind: String,
    #[serde(default)]
    pub selected_helper_ids: Vec<String>,
    #[serde(default)]
    pub selected_helper_primitive_ids: Vec<String>,
    #[serde(default)]
    pub selected_helper_primitive_kinds: Vec<String>,
    #[serde(default)]
    pub selected_bridge_capabilities: Vec<String>,
    #[serde(default)]
    pub selected_operator_bundle_ids: Vec<String>,
    #[serde(default)]
    pub selected_acceptance_recipe_ids: Vec<String>,
    #[serde(default)]
    pub selected_patch_primitive_classes: Vec<String>,
    #[serde(default)]
    pub selected_family_build_primitive_classes: Vec<String>,
    #[serde(default)]
    pub selected_composition_layers: Vec<String>,
    #[serde(default)]
    pub runtime_requirements: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    pub confidence_band: String,
    pub should_escalate: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PrimitiveExecutionStep {
    pub step_id: String,
    pub composition_layer: String,
    pub primitive_name: String,
    pub adapter_id: String,
    pub adapter_kind: String,
    pub support_level: String,
    pub source_kind: String,
    pub source_value: String,
    pub execution_order: usize,
    pub execution_status: String,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PrimitiveExecutionPlan {
    pub execution_plan_id: String,
    pub composition_plan_id: String,
    pub request_id: String,
    pub target_family_id: Option<FamilyId>,
    pub target_tool_kind: Option<String>,
    pub composition_work_order_kind: String,
    #[serde(default)]
    pub composition_layers: Vec<String>,
    #[serde(default)]
    pub selected_patch_kinds: Vec<String>,
    #[serde(default)]
    pub selected_helper_ids: Vec<String>,
    #[serde(default)]
    pub selected_acceptance_recipe_ids: Vec<String>,
    #[serde(default)]
    pub steps: Vec<PrimitiveExecutionStep>,
    pub overall_status: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveProofFamilyRequestBinding {
    pub family_id: FamilyId,
    pub family_label: String,
    pub request_template: String,
    pub empty_request_fallback: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveProofEnrichmentBinding {
    pub family_id: FamilyId,
    #[serde(default)]
    pub missing_capability_classes: Vec<String>,
    #[serde(default)]
    pub patch_requests: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PrimitiveProofExecutionRecipe {
    pub comparison_bundle_id: String,
    pub request_generation_kind: String,
    pub enrichment_kind: String,
    #[serde(default)]
    pub family_request_bindings: Vec<PrimitiveProofFamilyRequestBinding>,
    #[serde(default)]
    pub enrichment_bindings: Vec<PrimitiveProofEnrichmentBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PrimitiveProofTemplate {
    pub template_id: String,
    pub template_kind: String,
    pub display_label: String,
    pub description: String,
    pub shared_request_seed: String,
    #[serde(default)]
    pub target_family_ids: Vec<FamilyId>,
    #[serde(default)]
    pub required_composition_layers: Vec<String>,
    #[serde(default)]
    pub required_family_build_primitive_classes: Vec<String>,
    #[serde(default)]
    pub required_patch_primitive_classes: Vec<String>,
    #[serde(default)]
    pub required_helper_primitive_kinds: Vec<String>,
    #[serde(default)]
    pub required_capability_classes: Vec<String>,
    #[serde(default)]
    pub optional_capability_classes: Vec<String>,
    #[serde(default)]
    pub optional_enrichment_steps: Vec<String>,
    #[serde(default)]
    pub execution_recipe: PrimitiveProofExecutionRecipe,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CapabilityComparisonPolicy {
    pub comparison_receipt_prefix: String,
    pub comparison_label: String,
    pub shared_note_label: String,
    pub left_only_note_label: String,
    pub right_only_note_label: String,
    pub required_bundle_note_label: String,
    pub success_note_template: String,
    pub failure_note_template: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CapabilityComparisonBundle {
    pub bundle_id: String,
    pub bundle_kind: String,
    #[serde(default)]
    pub required_shared_capability_classes: Vec<String>,
    #[serde(default)]
    pub optional_shared_capability_classes: Vec<String>,
    #[serde(default)]
    pub tolerated_left_only_capability_classes: Vec<String>,
    #[serde(default)]
    pub tolerated_right_only_capability_classes: Vec<String>,
    pub minimum_shared_capability_count: usize,
    pub equivalence_mode: String,
    #[serde(default)]
    pub policy: CapabilityComparisonPolicy,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PrimitiveProofHarnessReceipt {
    pub receipt_id: String,
    pub proof_template_id: String,
    pub proof_template_kind: String,
    pub proof_template_display_label: String,
    pub capability_comparison_bundle_id: String,
    pub shared_request: String,
    pub left_request: String,
    pub right_request: String,
    #[serde(default)]
    pub target_family_ids: Vec<FamilyId>,
    pub left_project_name: String,
    pub left_family_id: Option<FamilyId>,
    pub left_request_id: String,
    pub left_composable_route_plan_path: Option<String>,
    pub left_primitive_execution_plan_path: Option<String>,
    pub right_project_name: String,
    pub right_family_id: Option<FamilyId>,
    pub right_request_id: String,
    pub right_composable_route_plan_path: Option<String>,
    pub right_primitive_execution_plan_path: Option<String>,
    pub comparison_receipt_path: String,
    pub equivalent_capability_fulfillment: bool,
    #[serde(default)]
    pub notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RetrySearchProofReceipt {
    pub receipt_id: String,
    pub proof_kind: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub final_outcome: Option<String>,
    pub requested_model_selector: Option<String>,
    #[serde(default)]
    pub model_candidate_count: usize,
    #[serde(default)]
    pub retry_posture_count: usize,
    #[serde(default)]
    pub launch_timeout_secs: u64,
    #[serde(default)]
    pub request_timeout_secs: u64,
    #[serde(default)]
    pub cleanup_overhead_secs: u64,
    #[serde(default)]
    pub expected_outer_timeout_secs: u64,
    #[serde(default)]
    pub attempted_models: Vec<String>,
    #[serde(default)]
    pub attempted_methods: Vec<String>,
    #[serde(default)]
    pub generation_receipt_paths: Vec<String>,
    pub successful_model_path: Option<String>,
    pub successful_method: Option<String>,
    pub forced_initial_rejection: bool,
    pub method_space_exhausted: bool,
    #[serde(default)]
    pub internal_timeout_observed: bool,
    #[serde(default)]
    pub normalized_failure_class: String,
    #[serde(default)]
    pub recommended_next_action: String,
    #[serde(default)]
    pub recommended_next_step: String,
    #[serde(default)]
    pub notes: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ClarificationRequest {
    pub clarification_id: String,
    pub request_id: String,
    pub mode: Option<RequestMode>,
    pub question: String,
    pub reasons: Vec<String>,
    pub candidate_family_ids: Vec<FamilyId>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FallbackBuildSpec {
    pub fallback_spec_id: String,
    pub request_id: String,
    pub mode: Option<RequestMode>,
    pub composition_route_class: Option<CompositionRouteClass>,
    pub interpreted_goal: String,
    pub candidate_family_ids: Vec<FamilyId>,
    pub suggested_extension_kind: String,
    pub suggested_family_id: Option<FamilyId>,
    pub suggested_tool_kind: Option<String>,
    pub suggested_patch_kind: Option<String>,
    #[serde(default)]
    pub suggested_bridge_capabilities: Vec<String>,
    pub suggested_hosting_mode: Option<String>,
    pub requested_capabilities: Vec<String>,
    pub constraints: Vec<String>,
    #[serde(default)]
    pub missing_family_build_primitive_classes: Vec<String>,
    #[serde(default)]
    pub missing_patch_primitive_classes: Vec<String>,
    #[serde(default)]
    pub missing_helper_primitive_kinds: Vec<String>,
    #[serde(default)]
    pub suggested_artifacts: Vec<String>,
    #[serde(default)]
    pub acceptance_targets: Vec<String>,
    #[serde(default)]
    pub implementation_notes: Vec<String>,
    pub suggested_proof_seed_template_id: Option<String>,
    pub suggested_proof_seed_bundle_id: Option<String>,
    pub stub_bundle_path: Option<String>,
    #[serde(default)]
    pub recommended_next_action: String,
    pub recommended_next_step: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FallbackPlanReceipt {
    pub fallback_receipt_id: String,
    pub request_id: String,
    pub mode: Option<RequestMode>,
    pub status: String,
    pub reasons: Vec<String>,
    pub clarification_path: Option<String>,
    pub build_spec_path: Option<String>,
    pub planner_handoff_path: Option<String>,
    pub stub_bundle_path: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChattyCogCommandSpec {
    pub program: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub cwd: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChattyCogVisualLoadSpec {
    pub kind: String,
    pub auto_launch: bool,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub file: Option<String>,
    pub url: Option<String>,
    pub window_title_contains: Option<String>,
    #[serde(rename = "build")]
    pub build_command: Option<ChattyCogCommandSpec>,
    #[serde(rename = "launch")]
    pub launch_command: Option<ChattyCogCommandSpec>,
    #[serde(rename = "serve")]
    pub serve_command: Option<ChattyCogCommandSpec>,
    pub serve_wait_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChattyCogBridgeCapabilities {
    pub status_enabled: bool,
    pub log_sources_enabled: bool,
    pub shared_room_state_enabled: bool,
    pub shared_room_events_enabled: bool,
    pub outgoing_room_events_enabled: bool,
    #[serde(default)]
    pub incoming_asset_lanes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChattyCogBridgeSpec {
    pub status_path: String,
    pub script_path: Option<String>,
    pub log_sources_path: Option<String>,
    pub capabilities: ChattyCogBridgeCapabilities,
    #[serde(default)]
    pub recommended_runtime_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChattyCogModuleSpec {
    pub module_spec_id: String,
    pub project_name: String,
    pub module_id: String,
    pub display_name: String,
    pub description: String,
    pub visual_kind: String,
    pub visual_title: String,
    pub visual_file: String,
    pub handshake_path: String,
    pub manifest_path: String,
    pub visual_load_path: Option<String>,
    pub visual_load: Option<ChattyCogVisualLoadSpec>,
    pub bridge: ChattyCogBridgeSpec,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChattyEduModuleSpec {
    pub module_spec_id: String,
    pub project_name: String,
    pub module_id: String,
    pub display_name: String,
    pub description: String,
    pub visual_kind: String,
    pub visual_title: String,
    pub visual_file: String,
    pub handshake_path: String,
    pub manifest_path: String,
    pub visual_load_path: String,
    pub network_capabilities_path: String,
    pub bridge_status_env_var: String,
    pub created_at: Option<String>,
}
