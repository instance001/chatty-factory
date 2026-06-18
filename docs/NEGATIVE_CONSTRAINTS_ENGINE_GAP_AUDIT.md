# Negative Constraints Engine Gap Audit

This document audits the current ChattyFactory implementation against the
target shape described in
[Negative Constraints Engine Parts List](./NEGATIVE_CONSTRAINTS_ENGINE_PARTS_LIST.md).

It is written as a practical status board for a senior developer joining cold:

- what already exists
- what is partial
- what is still missing
- what matters most to close the gap

Status labels used here:

- `Real`: present in contracts, runtime, and product flow today
- `Partial`: meaningfully present, but still incomplete or too narrow
- `Missing`: conceptually intended but not yet trustworthy as a product part

## Overall Read

ChattyFactory is closer than it may feel.

The host-owned spine is real:

- frozen plans and reviews
- task graphs
- task receipts
- verification receipts
- retry-search proofs
- decomposition receipts
- failure vaulting
- triangulation sessions
- constraint promotion artifacts
- runtime/UI surfaces for inspecting that evidence

The main remaining gap is not "add more lanes."
The main remaining gap is turning the existing evidence plumbing into a fully
self-improving negative engine that:

- preserves hard user requirements more explicitly
- classifies fallback/degradation more honestly
- selects reusable decomposition grammars automatically from failure evidence
- triangulates repeated failures into narrow durable constraints without us
  hand-authoring the next rule

## Part-By-Part Audit

## 1. Intent Freeze

Status: `Partial`

What exists:

- request/plan contracts already carry interpreted goal, constraints, route
  reasons, candidate families, and review state
- patch-side intent freeze is real in `runtime/patch_intent_freezes/`
- request normalization is reflected in `runtime/requests/`, `runtime/routes/`,
  `runtime/plans/`, and patch diagnosis/review artifacts

What is still missing:

- a more explicit universal build-side "intent freeze" artifact with first-class
  hard requirement fields rather than relying on distributed plan fields
- stronger separation between:
  - hard requirement
  - preference
  - inferred convenience
- more explicit requirement preservation for stack/toolchain choices

Why it matters:

Without a first-class requirement freeze, fallback honesty can still blur.

## 2. Bounded Task Shaper

Status: `Real`

What exists:

- reviewed build plans
- build execution work orders
- frozen `plan_tasks/`
- task-level execution and verification receipts
- model-authored microtask envelopes

What is strong already:

- the system no longer jumps straight from request to giant freeform generation
- there is now a host-owned task boundary before model-authored work proceeds

Remaining concern:

- atomization policy is still stronger on paper and receipts than as a visibly
  generalized cross-task strategy

## 3. Attempt Scaffold

Status: `Partial`

What exists:

- bounded soft-review continuation now allows scaffold-first build attempts and
  substrate-first patch attempts
- fallback scaffolding and stub artifacts exist:
  - `runtime/fallback_build_specs/`
  - `runtime/fallback_plan_receipts/`
  - `runtime/extension_stubs/`

What is still missing:

- fully substrate-agnostic build scaffolding independent of family assumptions
- stronger preservation of explicit hard stack requests without drifting toward
  nearest familiar starters
- a universal classification layer that says:
  - full success
  - degraded
  - substituted
  - requirement-not-met

Why it matters:

This is one of the most important remaining anti-prison-bar gaps.

## 4. Model Proposal Interface

Status: `Real`

What exists:

- planner artifacts
- model task prompts/raw responses/generation receipts
- model-authored microtask attempt receipts
- bounded review gates before model work is accepted as truth

What is strong already:

- the model is no longer the sole truth-owner
- host receipts outvote model narration

Main next step:

- keep reducing any remaining places where model prose is implicitly trusted

## 5. Work Order And Task Graph

Status: `Real`

Evidence:

- `runtime/build_execution_work_orders/`
- `runtime/plan_tasks/`
- `runtime/task_execution_receipts/`
- `runtime/task_verification_receipts/`
- `BuildExecutionWorkOrder`, `PlanTaskList`, and `PlanTask` contracts

This is one of the clearest areas where the architecture is already aligned.

## 6. Execution Primitives

Status: `Partial`

What exists:

- host-sync tasks
- host-mechanical tasks
- model-authored tasks
- patch execution flows
- helper runtime receipts in `runtime/helper_runtime_receipts/`
- primitive execution artifacts in `runtime/primitive_execution_plans/`

What is still missing:

- a cleaner substrate-agnostic primitive layer that is clearly the main
  execution vocabulary rather than a layer still partly framed by starter/family
  machinery
- broader proof that unknown build requests can be expressed through primitive
  combinations without silently leaning on positive lanes

## 7. Verification Layer

Status: `Real`

Evidence:

- `runtime/build_verification_receipts/`
- `runtime/task_verification_receipts/`
- `runtime/acceptance_plans/`
- `runtime/acceptance_results/`
- patch postcheck flow and `runtime/patch_receipts/`

What is strong already:

- shell outcome is not final truth
- proof receipt is authoritative
- patch/build verification is persistent and inspectable

Main remaining gap:

- requirement satisfaction versus fallback substitution still needs stronger
  first-class classification

## 8. Receipt System

Status: `Real`

Evidence:

- runtime contains a broad receipt lattice already:
  - plans
  - reviews
  - work orders
  - prompts/responses
  - execution and verification
  - decomposition
  - retry proofs
  - vaulting
  - triangulation
  - promotion candidates

This is arguably the project’s strongest architectural asset.

## 9. Retry Search Engine

Status: `Real`

Evidence:

- `runtime/retry_search_proofs/`
- generation receipts
- model ladder posture tracking
- outer timeout budgeting
- Qwen3-8B hostile proof

What is strong already:

- retry search is receipt-owned
- shell timeout is no longer treated as truth by itself
- multiple methods and multiple models are accounted for

Main next step:

- keep integrating retry-search results into later decomposition and promotion
  decisions more automatically

## 10. Decomposition Engine

Status: `Partial`

Evidence:

- `runtime/task_decomposition_receipts/`
- `runtime/task_decomposition_proposals/`
- `runtime/task_decomposition_inference_receipts/`
- docs and contracts explicitly describe decomposition grammar intent

What is already real:

- failed tasks can trigger decomposition evidence
- the system can persist proposals and receipts instead of dropping that state

What is still missing:

- reliable generic grammar selection from failure class
- broader proof that decomposition is not just implemented for a narrow subset
  of task types
- stronger automatic reuse of previously learned split patterns

This is one of the biggest remaining architecture gaps.

## 11. Atomization Floor

Status: `Partial`

Evidence:

- `runtime/atomization_floor_decisions/`
- contract inventory calls out floor-level handling

What is still missing:

- a clearly enforced universal floor policy across all task classes
- stronger routing once floor is reached:
  - retry differently
  - escalate model
  - triangulate
  - do not split further

This is critical to avoiding atomization into dust.

## 12. Failure Vault

Status: `Real`

Evidence:

- `runtime/failure_vault/`
- UI reads and surfaces recent vault entries
- docs explicitly frame unresolved failures as provisional evidence

Main next step:

- improve continuity across repeated related failures so vault evidence becomes
  easier to resume and extend

## 13. Failure Classification

Status: `Partial`

What exists:

- failure class and mode concepts already appear in fallback and review flows
- receipts distinguish model-only exhaustion vs full ladder exhaustion
- reasoning fallback vs usable content is now surfaced in proof receipts

What is still missing:

- a clearer generic classification taxonomy used consistently across build,
  patch, retry-search, decomposition, and promotion
- tighter linkage from failure class to next action across every governed flow:
  - retry posture
  - decomposition grammar
  - vault continuation
  - promotion candidate

Update:

- build-side fallback and verification flows now have a first host-owned pass at
  this through:
  - `normalized_failure_class`
  - `recommended_next_action`
  - `recommended_next_step`
- patch preflight/postcheck and retry-search ladder proof receipts now also
  emit the same host-owned fields
- task-attempt vault and triangulation findings also now record the selected
  next action mechanically instead of leaving it implicit

Remaining gap:

- remaining governed flows still need to converge on the same selector rather
  than carrying partial bespoke routing, especially where patch/proof/build
  still differ in receipt shape and reuse depth

## 14. Triangulation Engine

Status: `Partial`

Evidence:

- `runtime/triangulation_sessions/`
- `runtime/triangulation_loop_summary.json`
- UI/runtime dashboard surfaces triangulation counts and summaries
- docs clearly define the intended triangulation-first posture

What is already real:

- repeated failure evidence is being grouped and summarized
- model-ladder exhaustion is distinguished from convergent floor-level evidence

What is still missing:

- stronger automated narrowing from "same area failed again" to "this exact
  culprit is the blocker"
- better reuse of triangulation findings when the same sticking point reappears

This is another major remaining gap.

## 15. Constraint Promotion Gate

Status: `Partial`

Evidence:

- `runtime/proposed_constraint_receipts/`
- `runtime/constraint_approvals/`
- `runtime/constraint_shelf_mutations/`
- `runtime/approved_constraint_shelf.json`

What is already real:

- there is already a separation between proposed and approved constraints
- shelf mutation is tracked instead of silently edited

What is still missing:

- fully trustworthy automatic promotion criteria based on converged evidence
- stronger guarantees that promotion remains narrow and never broadens into
  fear-driven prohibition

## 16. Negative Constraint Shelf

Status: `Partial`

What exists:

- an approved shelf and shelf mutation trail
- the conceptual direction is explicit in docs

What is still missing:

- confidence that the shelf is populated from triangulated narrow evidence
  rather than mixed historical mechanisms
- stronger schema discipline for exact failure conditions and replacement
  guidance
- clearer guardrails against negative-lane sprawl

## 17. Fallback Classification

Status: `Partial`

What exists:

- fallback receipts
- fallback result UI
- soft-review route notes
- build verification and proposed constraint attachment to fallback state

What is still missing:

- first-class outcome taxonomy consistently used across the product:
  - full success
  - partial success
  - degraded fallback
  - substituted stack
  - requirement-not-met
- stronger coupling between explicit user requirements and final result label

This is one of the most user-visible remaining gaps.

## 18. Patch Truth Layer

Status: `Real`

Evidence:

- `runtime/patch_diagnoses/`
- `runtime/patch_intent_freezes/`
- `runtime/patch_plan_reviews/`
- `runtime/patch_receipts/`
- postcheck and xray UI surfaces

This side of the product is meaningfully governed already.

Main next step:

- keep generalizing substrate-first patch attempts beyond named patch lanes

## 19. Runtime Budgeting

Status: `Real`

Evidence:

- runtime config timeout fields
- retry-search outer timeout budgeting
- proof UI recommendation logic
- hostile proof documentation and receipts

This area has come a long way and is now structurally much healthier.

## 20. Operator Surfaces

Status: `Real`

Evidence:

- desktop UI exposes runtime, proof, fallback, patch xray, and triangulation
  surfaces
- CLI exposes build, patch, proof, refresh, and scaffold flows

Main next step:

- keep simplifying presentation so the signal stays readable as runtime evidence
  grows

## 21. Self-Improvement Loop

Status: `Partial`

What exists:

- bounded attempts
- verification
- retry search
- decomposition receipts
- failure vault
- triangulation sessions
- promotion candidate artifacts

What is still missing:

- end-to-end automatic closure of the loop without us hand-guiding the next
  reasoning step
- stronger automatic mapping:
  - failure class -> retry posture
  - failure class -> decomposition grammar
  - converged evidence -> promotion candidate quality

This is the biggest strategic remaining gap.

## 22. Governance Rule Set

Status: `Partial`

What exists:

- the principles are well-described in docs
- the host already owns much more truth than before
- bounded soft-review continuation reduced some positive-lane prison behavior

What is still missing:

- making those principles the dominant implementation truth everywhere
- eliminating remaining places where family/starter assumptions still act like
  hidden law instead of optional accelerator

## Where The Factory Is Strongest Right Now

These parts are already solid enough to treat as real architectural wins:

- host-owned receipts
- work orders and task graphs
- verification and postcheck
- patch governance
- retry-search ladder proofs
- provisional failure vaulting
- runtime/operator visibility

## Biggest Remaining Gaps

If we want to push the factory over the line toward the intended design, the
highest-value gaps are:

1. Hard requirement preservation
   - explicit stack/toolchain requirements need to survive interpretation and
     final classification more cleanly

2. Honest fallback taxonomy
   - the product needs first-class degraded/substituted/requirement-not-met
     outcome labels

3. Generic failure classification
   - failure classes need to become the real selector for next action

4. Automatic decomposition grammar selection
   - the factory should infer the split pattern from evidence instead of waiting
     for us to teach each next one

5. Enforced atomization floor behavior
   - once the floor is reached, the system must stop subdividing and move to a
     different search posture

6. Stronger triangulation narrowing
   - repeated failures need to converge on exact blockers, not just accumulate
     in a holding pen

7. Constraint promotion discipline
   - durable shelf entries must be narrow, earned, and clearly proven

## Remaining Governed Surfaces Outside The Full Shared Selector

After the latest build, patch, and retry-search proof work, the shared
host-owned selector is now visibly real in:

- build fallback
- build verification
- task-attempt vaulting
- triangulation findings
- patch plan review and patch constraint review
- patch preflight skip/substrate flows
- patch postcheck
- declarative paired-proof receipts
- retry-search ladder proof receipts
- extension governance refresh/drift receipts for proof/composition/patch/helper/bridge
- constraint approval receipts and shelf mutation receipts

That is strong progress, but some governed surfaces still carry bespoke
decision wording, partial evidence mapping, or receipt shapes that are not yet
fully normalized.

### 1. Outcome-class versus next-action reuse

Current state:

- `outcome_class` is now useful across execution and fallback views
- execution results now carry explicit host-owned `recommended_next_action`
  and `recommended_next_step`, not just fallback receipts
- `recommended_next_action` is now useful across several receipt types

What is still missing:

- a clean universal rule for when:
  - outcome class is the operator summary
  - normalized failure class is the routing key
  - next action is the continuation choice

Working rule now frozen:

- operator-facing end-state surfaces get `outcome_class`
  - execution results
  - fallback results
- governed evidence receipts get `normalized_failure_class` when the host is
  classifying why a bounded attempt, review, or governance posture failed
- any receipt that the host expects operators or later orchestration to resume
  from should carry `recommended_next_action` and `recommended_next_step`
- simple artifact receipts such as `BuildReceipt`, `PatchReceipt`, and
  `HelperRuntimeReceipt` should stay narrow and not grow outcome/continuation
  fields unless they become routing surfaces

Detailed family-by-family classification now lives in
[Receipt Field Ownership Audit](./RECEIPT_FIELD_OWNERSHIP_AUDIT.md).

The forward-looking checklist for new receipt families lives in
[Receipt Design Gate](./RECEIPT_DESIGN_GATE.md).

Why this matters:

- the split is now real in the main execution and fallback surfaces
- but some secondary receipt families still need a firm rule about whether they
  should carry outcome summary, continuation posture, or both

## Practical Meaning Of The Remaining Gap

The project is no longer missing the selector.

The remaining issue is uneven coverage.

Right now the factory has:

- a real generic selector in some core failure paths
- partial or prose-heavy governance in others

So the remaining architectural task is not "invent the idea."

It is:

1. finish normalizing the remaining governed surfaces onto the same receipt
   pattern
2. apply the frozen field-ownership rule consistently instead of letting new
   receipt families accrete both outcome and continuation by default
3. keep outcome summary separate from continuation routing

## Short Conclusion

ChattyFactory is no longer "missing the architecture."

The architecture skeleton is already there.

What remains is turning the existing receipt/governance/decomposition/retry
infrastructure into a truly self-improving negative-constraints engine that can:

- preserve user requirements honestly
- attempt unknown requests without lane prison
- learn reusable decompositions from its own failures
- triangulate exact blockers over time
- and promote only narrow evidence-earned constraints

That is the remaining climb.
