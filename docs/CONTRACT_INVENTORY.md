# Contract Inventory

This document defines the first machine-owned records the rebuild should standardize.

The immediate goal is not to freeze every field forever. The goal is to make the first milestone native-data-driven from the start instead of hiding semantics inside prompt text.

## Contract Principles

- typed first
- machine-writable
- easy to inspect
- schema-validated where practical
- stable ids over vague prose
- explicit status fields
- enough structure to support deterministic routing and acceptance

## Milestone-One Contracts

### 1) `RequestRecord`

Purpose:
- normalized representation of the user's request plus current UI/workspace state

Why first:
- all later routing and family selection depends on this

Suggested fields:
- `request_id`
- `raw_request`
- `mode`
- `active_project`
- `explicit_stack`
- `desired_surface`
- `requested_capabilities`
- `exoskeleton_target`
- `candidate_family_ids`
- `ambiguity_flags`
- `created_at`

## Notes

- this record should be produced before any route graph work
- it should not contain model prose as required structure

### 2) `RouteDecision`

Purpose:
- explicit record of what route the control plane chose and why

Why first:
- route visibility is part of the product, not just debugging

Suggested fields:
- `route_id`
- `request_id`
- `selected_family_id`
- `selected_operator_ids`
- `selected_wrapper_ids`
- `selected_behavior_kind`
- `capability_transition`
- `decision_reasons`
- `fallback_level`
- `needs_llm_review`
- `created_at`

### 3) `ScaffoldInputs`

Purpose:
- concrete inputs passed into family template rendering/build machinery

Why first:
- separates request interpretation from build emission

Suggested fields:
- `family_id`
- `project_name`
- `title`
- `summary`
- `copy_bundle`
- `feature_tokens`
- `style_preset`
- `wrapper_target`
- `entrypoint_config`
- `fixture_config`

### 4) `AcceptancePlan`

Purpose:
- family-owned proof-of-success plan

Why first:
- acceptance should be deterministic and inspectable

Suggested fields:
- `acceptance_id`
- `request_id`
- `family_id`
- `checks`
- `required_files`
- `required_markers`
- `commands`
- `expected_outputs`
- `helper_checks`
- `schema_checks`

### 5) `FailureReport`

Purpose:
- typed result when verification fails

Why first:
- failure classification must happen before model judgment

Suggested fields:
- `failure_id`
- `request_id`
- `family_id`
- `failure_class`
- `failing_check_id`
- `evidence`
- `recommended_lane`
- `needs_llm_review`
- `created_at`

### 6) `ProjectSpec`

Purpose:
- ongoing contract for built projects

Why first:
- patch mode depends on it
- this already proved its value in the prototype

Suggested fields:
- `spec_id`
- `project_name`
- `family_id`
- `substrate`
- `entrypoints`
- `expected_files`
- `features`
- `acceptance_commands`
- `acceptance_checks`
- `wrapper_metadata`
- `updated_at`

### `PatchLaneStatus`

Purpose:
- describe one patch lane's current availability and surgical-contract maturity inside a project spec or execution result

Why:
- operators need to tell the difference between lanes that are tightly bounded surgeries and lanes that still depend on broader or legacy structural assumptions

Current fields:
- `recipe_id`
- `patch_kind`
- `dependency_mode`
- `requires_features`
- `provides_features`
- `availability_status`
- `surgical_maturity`
- `superseded_by_patch_kinds`
- `effective_preflight_readiness`
- `preflight_readiness_reason`

Current `surgical_maturity` tiers:
- `narrow_surface_contract`
- `broad_surface_contract`
- `legacy_shape_sensitive`
- `anchor_only_contract`
- `uncontracted`

### `ProjectPatchReadinessReceipt`

Purpose:
- persist host-owned patchability posture for one generated project over time

Why:
- project governance needs to distinguish between:
  - genuinely risky blocked lanes
  - and intentionally historical blocked lanes with known modern replacements

Current notable fields:
- `family_id`
- `family_display_name`
- `family_ecosystem`
- `readiness_counts`
- `patch_surgical_maturity_counts`
- `blocked_lane_reasons`
- `superseded_blocked_lane_replacements`
- `decomposable_historical_blocker_bundles`
- `risky_blocked_lane_count`
- `historical_blocked_lane_count`
- `decomposable_historical_blocker_count`
- `change_since_patchability_baseline_status`
- `change_since_patchability_baseline_notes`

### `FamilyGovernanceReceipt`

Purpose:
- persist host-owned governance state for one family manifest over time

Why:
- family governance should carry product-facing family identity, not just raw ids, so ecosystem-native adoption and drift can be discussed in operator language

Current notable fields:
- `family_id`
- `family_display_name`
- `family_ecosystem`
- `primary_substrate`
- `lifecycle_status`
- `supported_tool_kinds`
- `provided_build_primitive_classes`
- `primitive_adapter_ids`
- `drift_status`
- `change_since_last_live_status`

### `FamilyUsageSummaryReceipt`

Purpose:
- summarize which governed families and ecosystems are actually represented in built projects

Why:
- starter and family governance should be able to answer not just what exists, but what is being used over time in product-facing terms

Current notable fields:
- `total_projects`
- `ecosystem_project_counts`
- `families`
- `families[].family_id`
- `families[].family_display_name`
- `families[].family_ecosystem`
- `families[].project_count`

### `StarterUsageSummaryReceipt`

Purpose:
- summarize how often builds were run with an explicit mechanical starter override versus adaptive routing

Why:
- family adoption and starter adoption are related but not identical, and governance should distinguish deliberate starter choice from the family that ended up on disk

Current notable fields:
- `total_build_receipts`
- `explicit_override_builds`
- `auto_routed_builds`
- `matched_recommendation_builds`
- `overridden_recommendation_builds`
- `starters`
- `starters[].starter_id`
- `starters[].starter_label`
- `starters[].starter_lifecycle`
- `starters[].build_count`

## Family-Specific Early Contracts

These can arrive shortly after the main six.

### `HelperServiceSpec`

Purpose:
- typed definition of a bounded helper or service lane attached to a project or family

Why:
- helper/service behavior should not be an untyped side effect or a vague launch command

Suggested fields:
- `helper_spec_id`
- `helper_id`
- `helper_kind`
- `attached_family_id`
- `attached_tool_kind`
- `attached_project_name`
- `purpose`
- `entrypoint`
- `working_directory`
- `input_paths`
- `output_paths`
- `status_paths`
- `launch_policy`
- `primitives`
- `expected_files`
- `notes`
- `created_at`

### `HelperPrimitiveSpec`

Purpose:
- define one reusable host-owned helper capability inside a bounded helper bundle

Why:
- helper behavior should become composable machinery, not stay an opaque family-specific blob

Suggested fields:
- `primitive_id`
- `primitive_kind`
- `purpose`
- `input_paths`
- `output_paths`
- `status_paths`
- `dependency_mode`
- `requires_primitives`
- `notes`
- `created_at`

### `FamilyPrimitiveAdapter`

Purpose:
- declare how one family satisfies one primitive class or kind at one composition layer

Why:
- primitive-native execution needs host-owned adapter metadata instead of inferring every mapping after the fact

Suggested fields:
- `adapter_id`
- `composition_layer`
- `primitive_name`
- `adapter_kind`
- `support_level`
- `requires_helpers`
- `notes`

### `PrimitiveExecutionPlan`

Purpose:
- host-owned primitive-native work order for bounded composition execution

Why:
- the rebuild now needs an execution artifact that records primitive intent and adapter choice before execution falls back into family-specific patch machinery

Suggested fields:
- `execution_plan_id`
- `composition_plan_id`
- `request_id`
- `target_family_id`
- `target_tool_kind`
- `composition_work_order_kind`
- `composition_layers`
- `selected_patch_kinds`
- `selected_helper_ids`
- `selected_acceptance_recipe_ids`
- `steps`
- `overall_status`
- `created_at`

### `PrimitiveExecutionStep`

Purpose:
- one adapter-mapped primitive step inside a primitive-native execution plan

Why:
- composition should become inspectable at the primitive step level, not only at the patch lane level

Suggested fields:
- `step_id`
- `composition_layer`
- `primitive_name`
- `adapter_id`
- `adapter_kind`
- `support_level`
- `source_kind`
- `source_value`
- `execution_order`
- `execution_status`
- `notes`

### `ProjectPatchDiagnosis`

Purpose:
- host-owned pre-patch X-ray of an existing project before deterministic or bounded patch execution

Why:
- patch safety now depends on understanding the target project before editing it, not only on matching a patch lane

Suggested fields:
- `diagnosis_id`
- `request_id`
- `project_name`
- `family_id`
- `tool_kind`
- `substrate`
- `request_summary`
- `entrypoints`
- `expected_files`
- `candidate_target_files`
- `candidate_insertion_points`
- `diagnosed_artifact_groups`
- `declared_surface_groups`
- `patch_surgical_maturity`
- `expected_anchor_markers`
- `present_anchor_markers`
- `structural_guard_source`
- `conflicting_anchor_markers`
- `present_conflicting_anchor_markers`
- `preserve_invariants`
- `declared_ownership_boundaries`
- `project_structure_notes`
- `risk_notes`
- `pre_patch_artifact_hashes`
- `post_patch_checks`
- `created_at`

### `PatchIntentFreeze`

Purpose:
- host-owned pre-op patch intent record that binds interpreted patch meaning to diagnosed project structure

Why:
- request interpretation alone is not enough for safe patching; the chosen patch intent should explicitly reference diagnosed files, insertion points, and invariants before execution

Suggested fields:
- `freeze_id`
- `request_id`
- `project_name`
- `family_id`
- `tool_kind`
- `interpreted_goal`
- `intended_patch_kind`
- `candidate_patch_kinds`
- `diagnosis_id`
- `diagnosed_target_files`
- `diagnosed_insertion_points`
- `declared_surface_groups`
- `confirmed_surface_groups`
- `patch_surgical_maturity`
- `superseded_by_patch_kinds`
- `structural_guard_source`
- `required_anchor_markers`
- `confirmed_anchor_markers`
- `conflicting_anchor_markers`
- `present_conflicting_anchor_markers`
- `preserve_invariants`
- `risk_notes`
- `contract_confidence_summary`
- `replacement_guidance_summary`
- `freeze_notes`
- `created_at`

### `PatchPlanReview`

Purpose:
- host-owned self-review receipt for one frozen patch plan before preflight and execution

Why:
- diagnosis and freeze are stronger when the host can critique the first draft, narrow the surgical surface, and record whether it proceeded with the original or refined plan

Suggested fields:
- `review_id`
- `request_id`
- `project_name`
- `diagnosis_id`
- `freeze_id`
- `original_intended_patch_kind`
- `reviewed_intended_patch_kind`
- `decision`
- `promoted_candidate_patch_kinds`
- `original_target_files`
- `reviewed_target_files`
- `dropped_target_files`
- `original_insertion_points`
- `reviewed_insertion_points`
- `recommended_replacement_patch_kinds`
- `reviewed_replacement_bundle_patch_kinds`
- `reviewed_replacement_bundle_status`
- `findings`
- `blocked_reasons`
- `created_at`

### `ImplementationConstraint`

Purpose:
- host-owned negative rule describing one implementation method that should not survive planning or self-review

Why:
- scaling toward broader language and codebase coverage is more realistic when the host can reject known bad implementation methods instead of only trying to enumerate perfect positive lanes

Suggested fields:
- `constraint_id`
- `constraint_scope`
- `constraint_origin`
- `family_id`
- `tool_kind`
- `language_id`
- `constraint_kind`
- `forbidden_method_summary`
- `forbidden_markers`
- `required_markers`
- `forbidden_surface_groups`
- `violation_reason_template`
- `replacement_guidance`
- `severity`
- `active`
- `created_at`

### `ConstraintReviewReceipt`

Purpose:
- host-owned review receipt for the constraint shelf stage between patch self-review and execution

Why:
- diagnosis and self-review should not be the final arbiters of what proceeds; the host should be able to record which forbidden implementation methods were detected and what safer survivors remained

Suggested fields:
- `review_id`
- `request_id`
- `project_name`
- `family_id`
- `tool_kind`
- `review_subject`
- `original_intended_patch_kind`
- `reviewed_intended_patch_kind`
- `selected_constraints`
- `violations`
- `surviving_patch_kinds`
- `surviving_composition_patch_kinds`
- `blocked_methods`
- `recommended_replacements`
- `decision`
- `findings`
- `created_at`

### `BuildVerificationReceipt`

Purpose:
- host-owned verification receipt for deterministic build requests that fall back before a trustworthy build can complete

Why:
- the negative bookshelf needs a typed way to say not just that a build failed, but what method the factory should stop attempting for this failure class

Suggested fields:
- `verification_id`
- `request_id`
- `review_subject`
- `interpreted_goal`
- `candidate_family_ids`
- `suggested_family_id`
- `suggested_tool_kind`
- `suggested_extension_kind`
- `failure_class`
- `failure_mode`
- `reasons`
- `blocked_methods`
- `findings`
- `decision`
- `created_at`

### `ProposedConstraintReceipt`

Purpose:
- host-owned proposal artifact for a new negative-rule candidate derived from a failed or fallbacked build verification

Why:
- the factory should improve from real failures in a reviewable way instead of silently mutating the active shelf

Suggested fields:
- `proposal_id`
- `request_id`
- `source_verification_id`
- `status`
- `rationale`
- `proposed_constraint`
- `created_at`

### `ApprovedConstraintShelf`

Purpose:
- host-owned on-disk bookshelf of approved negative constraints that were deliberately promoted from proposals

Why:
- proposed constraints should not silently become active knowledge; the factory needs a reviewed shelf with stable persistence and auditable contents

Suggested fields:
- `shelf_id`
- `constraints`
- `updated_at`

### `ConstraintShelfHistory`

Purpose:
- host-owned history file for approved negative constraints that have been retired from the active shelf

Why:
- shelf cleanup should not erase reliability history; retired rules still deserve provenance even when they no longer belong in the active managed set

Suggested fields:
- `history_id`
- `archived_constraints`
- `updated_at`

### `ConstraintShelfHistoryEntry`

Purpose:
- one archived approved constraint plus the reason it was retired from the active shelf

Why:
- a history file is only useful if each archived rule preserves both its content and the retirement context

Suggested fields:
- `constraint`
- `archived_reason`
- `archived_from_shelf_id`
- `archived_match_count`
- `archived_at`

### `ConstraintApprovalReceipt`

Purpose:
- host-owned approval audit receipt for promoting one proposed constraint into the approved shelf

Why:
- the negative bookshelf becomes trustworthy when each promotion has provenance, not just a mutated proposal file

Suggested fields:
- `approval_id`
- `request_id`
- `proposal_id`
- `approved_constraint_id`
- `status`
- `shelf_path`
- `proposal_path`
- `rationale`
- `created_at`

### `ConstraintShelfMutationReceipt`

Purpose:
- host-owned audit receipt for activating or deactivating one approved negative constraint in the shelf

Why:
- once the shelf becomes editable, state changes should be traceable instead of silently mutating the active rule set

Suggested fields:
- `mutation_id`
- `constraint_id`
- `action`
- `shelf_path`
- `status`
- `created_at`

### `PatchDiagnosisPostcheckReceipt`

Purpose:
- host-owned post-op verification record for one patch surgery

Why:
- successful patching should explain not just that the change passed, but which surface contract it stayed within and how mature that contract was

Suggested fields:
- `receipt_id`
- `diagnosis_id`
- `request_id`
- `project_name`
- `patch_kind`
- `declared_surface_groups`
- `patch_surgical_maturity`
- `contract_confidence_summary`
- `modified_artifact_groups`
- `out_of_contract_modified_files`
- `verified_invariants`
- `warnings`
- `modified_files`
- `passed`
- `created_at`

### `PrimitiveProofTemplate`

Purpose:
- declare one reusable proof template in primitive-native capability terms

Why:
- paired proofs should generalize into a proof harness instead of staying one-off orchestration flows

Suggested fields:
- `template_id`
- `template_kind`
- `display_label`
- `description`
- `shared_request_seed`
- `target_family_ids`
- `required_composition_layers`
- `required_family_build_primitive_classes`
- `required_patch_primitive_classes`
- `required_helper_primitive_kinds`
- `required_capability_classes`
- `optional_capability_classes`
- `optional_enrichment_steps`
- `execution_recipe`
- `created_at`

### `PrimitiveProofExecutionRecipe`

Purpose:
- declare how one proof template should be executed without requiring bespoke host branching

Why:
- proof templates should carry their own request shaping, enrichment, and comparison receipt recipe

Suggested fields:
- `comparison_bundle_id`
- `request_generation_kind`
- `enrichment_kind`
- `family_request_bindings`
- `enrichment_bindings`

### `PrimitiveProofFamilyRequestBinding`

Purpose:
- declare family-specific proof request phrasing in catalog data instead of host branching

Why:
- proof templates should be able to describe per-family request wording while keeping a shared intent seed

Suggested fields:
- `family_id`
- `family_label`
- `request_template`
- `empty_request_fallback`

### `PrimitiveProofEnrichmentBinding`

Purpose:
- declare family-specific proof enrichment steps in catalog data instead of host branching

Why:
- proof templates should be able to specify which patch requests fill missing capability gaps for each target family

Suggested fields:
- `family_id`
- `missing_capability_classes`
- `patch_requests`

### `CapabilityComparisonBundle`

Purpose:
- define one reusable capability-equivalence bundle for proof comparison

Why:
- proof equivalence should be capability-bundle driven instead of remaining one monitoring-specific comparison shape

Suggested fields:
- `bundle_id`
- `bundle_kind`
- `required_shared_capability_classes`
- `optional_shared_capability_classes`
- `tolerated_left_only_capability_classes`
- `tolerated_right_only_capability_classes`
- `minimum_shared_capability_count`
- `equivalence_mode`
- `policy`
- `created_at`

### `CapabilityComparisonPolicy`

Purpose:
- declare comparison receipt naming and note phrasing in bundle data instead of host branching

Why:
- capability comparison behavior should travel with the comparison bundle rather than being reconstructed by each proof runner

Suggested fields:
- `comparison_receipt_prefix`
- `comparison_label`
- `shared_note_label`
- `left_only_note_label`
- `right_only_note_label`
- `required_bundle_note_label`
- `success_note_template`
- `failure_note_template`

### `PrimitiveProofHarnessReceipt`

Purpose:
- host-owned receipt for one proof-harness execution across multiple families

Why:
- generalized proof work needs a first-class orchestration record above individual comparison receipts

Suggested fields:
- `harness_receipt_id`
- `proof_template_id`
- `proof_template_kind`
- `capability_comparison_bundle_id`
- `shared_request`
- `left_request`
- `right_request`
- `target_family_ids`
- `left_project_name`
- `left_family_id`
- `left_request_id`
- `left_composable_route_plan_path`
- `left_primitive_execution_plan_path`
- `right_project_name`
- `right_family_id`
- `right_request_id`
- `right_composable_route_plan_path`
- `right_primitive_execution_plan_path`
- `comparison_receipt_paths`
- `equivalent_capability_fulfillment`
- `notes`
- `created_at`

### `HelperLaunchPolicy`

Purpose:
- explicit host-owned launch boundary for a helper lane

Why:
- helper execution needs the same honesty and policy surface as the rest of the rebuild

Suggested fields:
- `helper_id`
- `helper_kind`
- `allowed_root`
- `program`
- `args`
- `working_directory`
- `status_paths`
- `expected_files`
- `created_at`

### `HelperRuntimeReceipt`

Purpose:
- typed receipt for a helper launch/check run

Why:
- background behavior must still be inspectable and host-supervised

Suggested fields:
- `receipt_id`
- `request_id`
- `helper_id`
- `helper_kind`
- `attached_project_name`
- `launch_status`
- `process_id`
- `observed_status_files`
- `observed_output_files`
- `observed_primitive_ids`
- `notes`
- `created_at`

### `HelperStatusSnapshot`

Purpose:
- compact current-state view of a helper lane

Why:
- helpers need a readable “what is happening now” contract, not just receipts

Suggested fields:
- `helper_id`
- `helper_kind`
- `status`
- `summary`
- `observed_inputs`
- `observed_outputs`
- `observed_status_files`
- `updated_at`

### `HelperAcceptancePlan`

Purpose:
- helper-specific acceptance requirements for bounded service behavior

Why:
- helper lanes need deterministic verification, not only “process launched”

Suggested fields:
- `helper_id`
- `required_files`
- `required_status_paths`
- `required_markers`
- `commands`
- `checks`

### `FamilyCapabilities`

Purpose:
- define what a family can honestly support

Why:
- avoids bluffing through the wrong substrate

Suggested fields:
- `family_id`
- `supports_capabilities`
- `requires_helper_for`
- `forbids_capabilities`

### `FamilyRepairLanes`

Purpose:
- define typed repair options per family

Suggested fields:
- `family_id`
- `repair_lanes`
- `supported_failure_classes`
- `retry_limits`

## Milestone-One Priority Order

Build in this order:
1. `RequestRecord`
2. `RouteDecision`
3. `ScaffoldInputs`
4. `AcceptancePlan`
5. `FailureReport`
6. `ProjectSpec`
7. `HelperServiceSpec`
8. `HelperLaunchPolicy`
9. `HelperRuntimeReceipt`
10. `HelperStatusSnapshot`
11. `HelperAcceptancePlan`

That sequence supports:
- classification
- family selection
- mechanical build emission
- deterministic acceptance
- typed failure handling
- future patch grounding
