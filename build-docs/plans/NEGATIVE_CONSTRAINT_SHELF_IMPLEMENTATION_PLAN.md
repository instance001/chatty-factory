# Negative Constraint Shelf Implementation Plan

## Purpose

This plan translates the negative constraint shelf milestone into an executable
 engineering sequence.

The goal is not to replace deterministic families, patch lanes, or diagnosis.

The goal is to add a reusable host-owned layer that:

- rejects implementation methods known to fail
- narrows the model's implementation search space
- learns from real build and patch failures
- proposes reusable constraint additions instead of only failing locally

This is the practical route toward a factory that succeeds most of the time,
not a fantasy route to 100% success.

## Strategic Model

The operating split should be:

1. LLM:
   - freeze user intent
   - propose broad implementation strategy
   - reason about likely code surfaces

2. Host:
   - own scaffolding
   - own syntax-sensitive boilerplate
   - own contracts and artifact structure
   - own execution, verification, and persistence

3. Constraint shelf:
   - remove methods that should not be attempted
   - block stale splice shapes
   - block impossible structural moves
   - redirect toward safer surviving options

4. Verification/X-ray:
   - classify failures
   - infer candidate new constraints
   - present proposed bookshelf additions for approval

This keeps the model doing what it is good at, while moving the brittle
 mechanical correctness burden into reusable host-owned guardrails.

## Overall Delivery Order

The implementation should happen in four waves:

1. Constraint contract and review pipeline
2. Build verification and failure taxonomy
3. Proposed constraint generation and approval flow
4. Iterative growth from real failures

Each wave should ship with:

- one typed contract
- one persisted receipt
- one proof path
- one operator-visible explanation surface

## Wave 1: Constraint Contract And Review Pipeline

### Goal

Add an explicit constraint-review step after self-review and before execution.

### Deliverables

1. `ImplementationConstraint`
2. `ConstraintViolation`
3. `ConstraintReviewReceipt`
4. host `constraint_review(...)` step
5. persistence under `runtime/`

### Suggested contract shape

#### `ImplementationConstraint`

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

#### `ConstraintViolation`

- `constraint_id`
- `constraint_kind`
- `scope`
- `matched_markers`
- `missing_required_markers`
- `violated_surface_groups`
- `reason`
- `replacement_guidance`

#### `ConstraintReviewReceipt`

- `review_id`
- `request_id`
- `project_name`
- `family_id`
- `tool_kind`
- `review_subject`
- `selected_constraints`
- `violations`
- `surviving_patch_kinds`
- `surviving_composition_patch_kinds`
- `blocked_methods`
- `recommended_replacements`
- `decision`
- `created_at`

### Review insertion point

The new pipeline order should be:

1. request interpretation
2. diagnosis
3. freeze
4. self-review
5. constraint review
6. preflight
7. patch/composition execution
8. postcheck

### First proof cases

Start only from already-proven failure classes:

- `helper_status_chip`
- `helper_summary_badges`
- Rust legacy parser splice lanes

### Success criteria

- host can explain that a method was blocked by a named constraint, not only
  by ad hoc logic
- receipt persists the blocked method and surviving alternatives
- UI/CLI can display the constraint-review summary

## Wave 2: Build Verification And Failure Taxonomy

### Goal

Extend X-ray beyond patch surgery into build verification so the factory can
 explain why a fresh build failed.

### Deliverables

1. `BuildVerificationReceipt`
2. `BuildFailureClassification`
3. `BuildConstraintProposal`
4. new verify-built-project host step

### Required verification classes

At minimum, classify:

- syntax/build failure
- missing entrypoint
- malformed project structure
- missing contract artifact
- acceptance mismatch
- runtime smoke failure
- dependency/config failure
- wrong-family implementation shape
- structural glue failure

### Important rule

Do not reduce verification to "did it compile?"

The shelf gets smarter only if failure classes are specific enough to point at
 reusable "how not to" rules.

### Success criteria

- every failed build receives a typed failure class
- the system can say what kind of implementation mistake likely happened
- the output is structured enough to seed a candidate negative constraint

## Wave 3: Proposed Constraint Generation And Approval Flow

### Goal

When build or patch verification fails, the host should be able to suggest a
 candidate bookshelf addition rather than only record a local failure.

### Deliverables

1. `ProposedConstraintReceipt`
2. host proposal generation from verification failure
3. approval flow
4. persistence to a candidate constraint shelf location

### Suggested proposal fields

- `proposal_id`
- `source_receipt_kind`
- `source_receipt_path`
- `failure_class`
- `project_name`
- `family_id`
- `tool_kind`
- `candidate_constraint`
- `evidence_summary`
- `confidence`
- `approval_status`
- `created_at`

### Approval model

The user should be able to:

- approve
- reject
- defer

No automatic promotion straight into the production shelf.

That keeps the system from overfitting a one-off failure into a global rule.

### Success criteria

- failed build/patch can emit a concrete proposed rule
- the rule is inspectable before activation
- the proposed rule includes evidence and a human-readable reason

## Wave 4: Iterative Shelf Growth From Real Failures

### Goal

Move from a seeded shelf to a living constraint system driven by recurring
 failure classes.

### Operating pattern

1. build or patch fails
2. verification classifies failure
3. host proposes constraint
4. user approves
5. future runs avoid the same implementation trap

### Important discipline

Do not add rules because they sound wise.

Add rules because:

- the failure was real
- the failure was repeatable
- the bad method is observable
- the rule is machine-checkable
- the rule narrows bad options without strangling valid ones

### Success criteria

- shelf grows through evidence, not guesswork
- repeated failures decrease in the same family/class
- the system starts succeeding more often without exploding the positive lane
  catalog

## Shelf Organization

To avoid chaos, constraints should be grouped by scope.

### Universal

- apply broadly
- structural and contract integrity rules

### Language

- Rust
- Python
- JavaScript/TypeScript
- HTML/CSS

### Family / Framework

- `chattycog_webview_module`
- `static_web_dashboard`
- `rust_cli_tool`
- `python_cli_tool`

### Project-derived temporary constraints

- diagnosis-owned
- patient-specific
- usually not promoted directly to permanent shelf rules without review

## Runtime Storage Plan

Suggested layout:

- `runtime/constraint_reviews/`
- `runtime/build_verification_receipts/`
- `runtime/proposed_constraints/`
- `runtime/approved_constraints/`

If a static shelf directory is introduced later, consider:

- `constraints/universal/`
- `constraints/languages/`
- `constraints/families/`
- `constraints/candidates/`

## UI / Operator Surfaces

The first UI slice does not need to be huge.

Good first surfaces:

- latest build/patch result
  - show constraint-review summary
- selected project details
  - show active temporary constraints from diagnosis
- proposed constraint queue
  - approve / reject / defer

This should remain evidence-first, not abstract policy-first.

## Concrete First Proof Targets

### Patch-side

1. `helper_status_chip`
2. `helper_summary_badges`
3. Rust legacy `file_output` / `severity_filter` style splice failures

### Build-side

Use existing failed or fragile build cases where:

- syntax was plausible but wrong
- glue was incomplete
- entrypoint/contract pairing broke
- structure was malformed even if some files existed

The exact first build proof should be whichever current failure is:

- easiest to classify
- easiest to detect automatically
- easiest to turn into a reusable negative rule

## Decision Gates

After Wave 1, stop and check:

- did the explicit constraint-review layer replace any ad hoc logic cleanly?

After Wave 2, stop and check:

- are failure classes concrete enough to produce real candidate rules?

After Wave 3, stop and check:

- are proposed rules high-signal enough that a user would trust approving them?

After Wave 4, stop and check:

- are we reducing repeated failures, or just collecting noisy constraint junk?

## Near-Term Execution Order

The first concrete coding sequence should be:

1. add `ImplementationConstraint` + `ConstraintReviewReceipt`
2. add host constraint-review step after patch-plan self-review
3. seed it with current patch-side known failures
4. surface the receipt in CLI/UI
5. add `BuildVerificationReceipt`
6. add build failure classification
7. emit `ProposedConstraintReceipt` from failed build verification
8. add approval path

## Intended Outcome

If this plan works, ChattyFactory should stop improving mainly by:

- adding more and more narrow positive lanes

and start improving more by:

- learning which implementation methods must not survive planning

That is the more realistic route to a factory that can build what users ask for
most of the time, across more languages and project shapes, without pretending
the model will ever be perfect at the low-level glue by itself.
