# Receipt Field Ownership Audit

This document freezes the current field-ownership rule for host/runtime
receipts so new receipt families do not quietly accrete both outcome and
continuation posture by default.

Use this together with:

- [Current Architecture](./CURRENT_ARCHITECTURE.md)
- [Negative Constraints Engine Gap Audit](./NEGATIVE_CONSTRAINTS_ENGINE_GAP_AUDIT.md)
- [Receipt Design Gate](./RECEIPT_DESIGN_GATE.md)

## Core Rule

- `outcome_class` is for operator-facing end-state surfaces.
- `normalized_failure_class` is for governed evidence receipts where the host is
  classifying why a bounded attempt, review, or governance posture failed.
- `recommended_next_action` and `recommended_next_step` are for surfaces the
  host expects operators or later orchestration to resume from.
- Narrow artifact receipts should stay narrow unless they become true routing
  surfaces.

## Bucket A: Keep Both Outcome And Continuation

These are operator-facing end-state summaries. They should carry both:

- `HostExecutionResult`
- `HostFallbackResult`

Reason:

- they answer both "what happened?" and "what should the host do next?"

## Bucket B: Keep Continuation Only

These are governed evidence or routing receipts. They should keep
`normalized_failure_class` plus `recommended_next_action` and
`recommended_next_step`, but should not grow `outcome_class`.

Build and patch governed receipts:

- `BuildVerificationReceipt`
- `PatchPlanReview`
- `ConstraintReviewReceipt`
- `PatchDiagnosisPostcheckReceipt`

Proof and retry governed receipts:

- `PrimitiveProofHarnessReceipt`
- `RetrySearchProofReceipt`

Governance and shelf governed receipts:

- `ProofLineageReceipt`
- `CompositionGovernanceReceipt`
- `PatchGovernanceReceipt`
- `HelperGovernanceReceipt`
- `BridgeGovernanceReceipt`
- `ConstraintApprovalReceipt`
- `ConstraintShelfMutationReceipt`

Fallback prescription surfaces:

- `FallbackBuildSpec`

Reason:

- these receipts are not final operator summaries
- they are host-owned routing evidence
- adding `outcome_class` here would blur evidence classification with end-state
  reporting

## Bucket C: Keep Narrow

These are artifact, execution-log, or catalog receipts. They should not grow
`outcome_class`, `normalized_failure_class`, or `recommended_next_*` unless
their job changes into a true routing surface.

Artifact receipts:

- `BuildReceipt`
- `PatchReceipt`
- `HelperRuntimeReceipt`
- `FallbackPlanReceipt`

Execution graph receipts:

- `BuildExecutionWorkOrder`
- `PlanTask`
- `PlanTaskList`
- `PlanTaskExecutionReceipt`
- `PlanTaskExecutionLog`
- `PlanTaskVerificationReceipt`
- `PlanTaskVerificationLog`

Runtime/catalog receipts:

- `RuntimeDiscoveryReceipt`
- `RuntimeModelCatalogReceipt`
- `RuntimeSmokeReceipt`
- `FamilyUsageSummaryReceipt`
- `StarterUsageSummaryReceipt`
- `ProjectPatchReadinessReceipt`

Governance/reference summaries:

- `FamilyGovernanceReceipt`
- `TemplateGovernanceReceipt`
- `CrossFamilyMonitoringComparisonReceipt`

Reason:

- these receipts primarily record what exists, what ran, or what was emitted
- they are supporting artifacts for later classification, not the
  classification surface itself

## Bucket D: Watchlist

These are the only current families that may warrant future reconsideration,
but should stay as-is for now.

- `PlanTaskModelAttemptReceipt`
- `TaskDecompositionReceipt`
- `TaskDecompositionInferenceReceipt`
- `AtomizationFloorDecision`
- `FailureVaultEntry`
- `TriangulationSession`
- `ConstraintPromotionCandidate`

Current guidance:

- keep them decision/evidence-oriented for now
- do not add `outcome_class`
- only add `recommended_next_*` if the host starts resuming directly from those
  receipts instead of deriving the next move elsewhere

## Immediate Conclusion

No obvious current receipt family is carrying fields that should be removed
right now.

The current cleanup result is:

- execution/fallback surfaces own outcome summary
- governed routing/evidence surfaces own continuation posture
- narrow artifact receipts remain narrow

That means the best next use of this audit is as a gate for future receipt
design, not a large field-removal refactor today.
