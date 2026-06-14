pub mod contracts;
pub mod execution;
pub mod foreman;
pub mod heuristics;
pub mod ids;
pub mod manifests;
pub mod primitive_catalog;
pub mod proof_harness;
pub mod project_browser;
pub mod registries;
pub mod runtime;
pub mod snapshot;
pub mod starters;

pub use contracts::{
    AcceptanceCheck, AcceptancePlan, AcceptanceRecipeStatus, BuildConstraintReviewReceipt,
    BuildExecutionWorkOrder, BuildFeatureSlice, BuildPlanArtifact, BuildPlanReview, BuildReceipt,
    PlanTask, PlanTaskExecutionLog, PlanTaskExecutionReceipt, PlanTaskList,
    PlanTaskModelAttemptReceipt, PlanTaskVerificationLog, PlanTaskVerificationReceipt,
    TaskDecompositionInferenceReceipt, TaskDecompositionProposal, TaskDecompositionReceipt,
    CapabilityTransition, ChattyCogBridgeCapabilities, ChattyCogBridgeSpec,
    ChattyCogCommandSpec, ChattyCogModuleSpec, ChattyCogVisualLoadSpec, ChattyEduModuleSpec,
    ClarificationRequest,
    ComposableRoutePlan, CompositionRouteClass, ContextBundle, DesiredSurface, DiscoveredModel, ExecutionPolicy, ExecutionReceipt,
    ExecutionSmokeCheck, ExoskeletonTarget, FailureClass, FailureReport, FailureReportEvidence,
    FamilyCapabilityManifest, FamilyPrimitiveAdapter, FallbackBuildSpec, FallbackPlanReceipt, HelperAcceptancePlan,
    HelperLaunchPolicy, HelperPrimitiveSpec, HelperRuntimeReceipt, HelperServiceSpec, HelperStatusSnapshot,
    ApprovedConstraintShelf, BuildVerificationReceipt, ConstraintApprovalReceipt,
    ConstraintReviewReceipt, ConstraintShelfHistory, ConstraintShelfHistoryEntry,
    ConstraintShelfMutationReceipt, ConstraintViolation, ImplementationConstraint,
    PlannedFileOperation, ProposedConstraintReceipt,
    ModelTaskGenerationReceipt, OperatorBundleStatus, PatchLaneStatus, PatchReceipt, PlannerDispatchReceipt, PlannerExecutionReceipt,
    PatchIntentFreeze, PatchPlanReview, ProjectPatchDiagnosis,
    PlannerHandoff, PlannerResponse, ProjectBrowserState, ProjectCatalogEntry, ProjectSession,
    ProjectSnapshot, ProjectSpec, RequestMode, RequestPlan, RequestRecord, RouteDecision,
    PrimitiveExecutionPlan, PrimitiveExecutionStep, PrimitiveProofEnrichmentBinding,
    PrimitiveProofExecutionRecipe, PrimitiveProofFamilyRequestBinding,
    PrimitiveProofHarnessReceipt, PrimitiveProofTemplate, CapabilityComparisonBundle,
    CapabilityComparisonPolicy,
    SnapshotGateResult,
    RuntimeCapabilityRecord, RuntimeConfig, RuntimeDiscoveryReceipt, RuntimeModelAssessment,
    RuntimeModelCatalogReceipt, RuntimeSmokeReceipt, ScaffoldInputs,
};
pub use execution::{build_execution_policy, run_execution_policy};
pub use foreman::{
    apply_planner_response, default_request_text, derive_planner_handoff, derive_request_plan,
    derive_scaffold_inputs, infer_route_hints, normalize_patch_request, normalize_request,
    persist_json_pretty, timestamp_id,
};
pub use heuristics::{
    chattycog_valid_hosting_modes, contains_any, contains_all, infer_capabilities_from_text,
    infer_chattycog_bridge_capabilities_from_text,
    infer_chattycog_hosting_mode_from_text, infer_chattycog_hosting_modes_from_text,
    infer_cli_tool_kind_from_text, infer_explicit_stack_from_text, infer_patch_kind_from_text,
    infer_request_tool_kind_from_text, request_has_cli_shape, request_has_dashboard_shape,
    request_has_explicit_build_shape, request_has_vague_improvement, request_has_web_shape,
    request_looks_like_followup_action, request_mentions_chattycog, request_mentions_chattyedu,
    request_mentions_python, request_mentions_rust, should_route_followup_via_planner_text,
    supported_chattycog_bridge_capabilities,
};
pub use ids::{FamilyId, OperatorId, WrapperId};
pub use manifests::{
    build_primitive_classes_for_family, built_in_family_manifests, is_supported_explicit_stack,
    manifest_for_family, primitive_adapter_names_for_family_layer, primitive_adapters_for_family,
    rank_family_candidates,
};
pub use primitive_catalog::{
    helper_primitive_kind_catalog, patch_primitive_class, patch_primitive_classes_for_kinds,
    HELPER_PRIMITIVE_KINDS, PATCH_PRIMITIVE_CLASSES,
};
pub use proof_harness::{
    built_in_capability_comparison_bundles, built_in_proof_templates,
    capability_comparison_bundle_by_id, capability_comparison_bundle_by_id_from_root,
    capability_comparison_bundle_manifest_path, capability_comparison_bundles_from_root,
    proof_template_by_id, proof_template_by_id_from_root, proof_template_manifest_path,
    proof_templates_from_root, repo_capability_comparison_bundle_manifest_root,
    repo_capability_comparison_bundles, repo_proof_template_manifest_root, repo_proof_templates,
};
pub use project_browser::{
    active_project_summary_line, build_project_browser_state, discover_projects,
    load_project_session, persist_project_browser_state, persist_project_session,
};
pub use registries::{
    candidate_operator_bundle_ids_for, expand_operator_bundle_ids, operator_bundle_registry,
    OperatorBundleSpec,
};
pub use runtime::{
    build_runtime_model_catalog, default_runtime_config, discover_runtime, resolve_model_choice,
    run_local_planner, run_local_text_generation, run_runtime_smoke,
};
pub use snapshot::{
    build_context_bundle, build_project_snapshot, gate_patch_project_snapshot,
};
pub use starters::{
    build_starter_best_for, build_starter_choices, build_starter_label, build_starter_lifecycle,
    build_starter_picker_label, is_known_build_starter_id, BuildStarterChoice,
};
