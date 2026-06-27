pub mod contracts;
pub mod execution;
pub mod foreman;
pub mod heuristics;
pub mod ids;
pub mod manifests;
pub mod primitive_catalog;
pub mod project_browser;
pub mod proof_harness;
pub mod registries;
pub mod runtime;
pub mod snapshot;

pub use contracts::{
    AcceptanceCheck, AcceptancePlan, AcceptanceRecipeStatus, ApprovedConstraintShelf,
    AtomizationFloorDecision, BuildConstraintReviewReceipt, BuildExecutionWorkOrder,
    BuildFeatureSlice, BuildIntentFreeze, BuildPlanArtifact, BuildPlanReview, BuildReceipt,
    BuildVerificationReceipt, CapabilityComparisonBundle, CapabilityComparisonPolicy,
    CapabilityTransition, ChattyCogBridgeCapabilities, ChattyCogBridgeSpec, ChattyCogCommandSpec,
    ChattyCogModuleSpec, ChattyCogVisualLoadSpec, ChattyEduModuleSpec, ClarificationRequest,
    ComposableRoutePlan, CompositionRouteClass, ConstraintApprovalReceipt,
    ConstraintPromotionCandidate, ConstraintReviewReceipt, ConstraintShelfHistory,
    ConstraintShelfHistoryEntry, ConstraintShelfMutationReceipt, ConstraintViolation,
    ContextBundle, DesiredSurface, DiscoveredModel, ExecutionPolicy, ExecutionReceipt,
    ExecutionSmokeCheck, ExoskeletonTarget, FailureClass, FailureReport, FailureReportEvidence,
    FailureVaultEntry, HelperAcceptancePlan,
    HelperLaunchPolicy, HelperPrimitiveSpec,
    HelperRuntimeReceipt, HelperServiceSpec, HelperStatusSnapshot, ImplementationConstraint,
    NextAttemptBuildSpec, NextAttemptReceipt,
    ModelTaskGenerationReceipt, OperatorBundleStatus, PatchIntentFreeze, PatchLaneStatus,
    PatchPlanReview, PatchReceipt, PlanTask, PlanTaskExecutionLog, PlanTaskExecutionReceipt,
    PlanTaskList, PlanTaskModelAttemptReceipt, PlanTaskVerificationLog,
    PlanTaskVerificationReceipt, PlannedFileOperation, PlannerDispatchReceipt,
    PlannerExecutionReceipt, PlannerHandoff, PlannerResponse, PrimitiveExecutionPlan,
    PrimitiveExecutionStep, PrimitiveProofEnrichmentBinding, PrimitiveProofExecutionRecipe,
    PrimitiveProofHarnessReceipt, PrimitiveProofSubstrateRequestBinding, PrimitiveProofTemplate,
    ProjectBrowserState, ProjectCatalogEntry, ProjectPatchDiagnosis, ProjectSession,
    ProjectSnapshot, ProjectSpec, ProposedConstraintReceipt, RequestMode, RequestPlan,
    RequestRecord, RetrySearchProofReceipt, RouteDecision, RuntimeCapabilityRecord, RuntimeConfig,
    RuntimeDiscoveryReceipt, RuntimeModelAssessment, RuntimeModelCatalogReceipt,
    BuildSeedInputs, RuntimeSmokeReceipt, SnapshotGateResult, TaskDecompositionInferenceReceipt,
    TaskDecompositionProposal, TaskDecompositionReceipt, TriangulationAttempt,
    TriangulationSession,
};
pub use execution::{build_execution_policy, run_execution_policy};
pub use foreman::{
    apply_planner_response, default_request_text, derive_planner_handoff, derive_request_plan,
    derive_build_seed_inputs, infer_route_hints, normalize_patch_request, normalize_request,
    persist_json_pretty, timestamp_id,
};
pub use heuristics::{
    chattycog_valid_hosting_modes, contains_all, contains_any, infer_capabilities_from_text,
    infer_chattycog_bridge_capabilities_from_text, infer_chattycog_hosting_mode_from_text,
    infer_chattycog_hosting_modes_from_text, infer_cli_tool_kind_from_text,
    infer_explicit_stack_from_text, infer_patch_kind_from_text, infer_request_tool_kind_from_text,
    request_has_cli_shape, request_has_dashboard_shape, request_has_explicit_build_shape,
    request_has_vague_improvement, request_has_web_shape, request_looks_like_followup_action,
    request_mentions_chattycog, request_mentions_chattyedu, request_mentions_python,
    request_mentions_rust, should_route_followup_via_planner_text,
    supported_chattycog_bridge_capabilities,
};
pub use ids::{OperatorId, SubstrateKind, WrapperId};
pub use manifests::is_supported_explicit_stack;
pub use primitive_catalog::{
    helper_primitive_kind_catalog, patch_primitive_class, patch_primitive_classes_for_kinds,
    HELPER_PRIMITIVE_KINDS, PATCH_PRIMITIVE_CLASSES,
};
pub use project_browser::{
    active_project_summary_line, build_project_browser_state, discover_projects,
    load_project_session, persist_project_browser_state, persist_project_session,
};
pub use proof_harness::{
    built_in_capability_comparison_bundles, built_in_proof_templates,
    capability_comparison_bundle_by_id, capability_comparison_bundle_by_id_from_root,
    capability_comparison_bundle_manifest_path, capability_comparison_bundles_from_root,
    proof_template_by_id, proof_template_by_id_from_root, proof_template_manifest_path,
    proof_templates_from_root, repo_capability_comparison_bundle_manifest_root,
    repo_capability_comparison_bundles, repo_proof_template_manifest_root, repo_proof_templates,
};
pub use registries::{
    candidate_operator_bundle_ids_for_context, expand_operator_bundle_ids,
    operator_bundle_registry, OperatorBundleSpec,
};
pub use runtime::{
    build_runtime_model_catalog, default_runtime_config, discover_runtime, resolve_model_choice,
    run_local_planner, run_local_text_generation, run_runtime_smoke,
};
pub use snapshot::{build_context_bundle, build_project_snapshot, gate_patch_project_snapshot};
