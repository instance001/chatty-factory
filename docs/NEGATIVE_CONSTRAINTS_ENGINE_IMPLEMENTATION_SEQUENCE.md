# Negative Constraints Engine Implementation Sequence

This document turns the gap audit into the next implementation sequence.

It is deliberately aimed at the final factory shape:

- the host is the safety machine
- the model provides bounded shape
- positive lanes are optional accelerators, not governing law

That means the transition goal is not "delete supported lanes first."
The goal is:

1. make positive lanes non-essential
2. prove the factory can operate honestly without depending on them
3. then detach and delete them when they are genuinely dead weight

## Transition Rule

Do not remove positive lanes while they still secretly carry core safety,
truth, or execution responsibilities.

Remove them only after those responsibilities are owned by:

- frozen intent
- bounded task shaping
- primitive execution
- verification
- retry search
- decomposition
- atomization floor control
- failure vaulting
- triangulation
- narrow constraint promotion
- honest fallback classification

If those parts own the safety, then deleting positive lanes removes convenience,
not structural integrity.

## Current Read

The factory is already strong on:

- receipts
- work orders and task graphs
- verification
- retry-search proofs
- patch truth
- failure vaulting
- runtime/operator visibility

The highest-leverage remaining gaps are:

1. hard requirement preservation
2. honest fallback outcome classification
3. generic failure classification
4. automatic decomposition grammar selection
5. enforced atomization-floor routing
6. stronger triangulation narrowing
7. narrow promotion discipline for durable constraints

## Implementation Order

## Phase 1. Freeze Requirements As First-Class Truth

Goal:
- stop silent drift between what the user asked for and what the factory later
  claims to have delivered

Why first:
- every later fallback, degradation, substitution, verification, and constraint
  decision depends on this

Deliverables:

- add a build-side intent freeze artifact parallel to patch intent freeze
- split request interpretation into:
  - hard requirements
  - preferences
  - inferred convenience
- preserve explicit stack/toolchain/surface requirements as structured fields
- persist that freeze under `runtime/`
- thread the freeze id through build plans, fallback receipts, verification
  receipts, and final result summaries

Success condition:
- the host can always say exactly which requirements were preserved, relaxed, or
  violated

Why this helps lane detachment:
- once stack/surface truth is explicit, the factory can stop "helpfully"
  collapsing unknown requests toward familiar positive shapes

## Phase 2. Add First-Class Outcome Truth Labels

Goal:
- classify what happened honestly instead of letting success language blur over
  substitution

Deliverables:

- define a universal result taxonomy for build and patch flows:
  - full_success
  - partial_success
  - degraded_fallback
  - substituted_stack
  - requirement_not_met
  - blocked_pending_retry_search
  - blocked_pending_triangulation
- persist this outcome class in final receipts and UI summaries
- derive the label from:
  - frozen requirements
  - actual execution receipts
  - verification receipts
  - fallback/substitution decisions

Success condition:
- if a user asked for Go and the factory produced a static dashboard scaffold,
  the result cannot be misreported as clean success

Why this helps lane detachment:
- positive lanes stop being seductive when they can no longer masquerade as
  equivalent success

## Phase 3. Install A Generic Failure Classification Layer

Goal:
- make failure class the selector for next action instead of ad hoc rule drift

Deliverables:

- define a generic host-owned failure taxonomy shared across build, patch,
  retry-search, decomposition, and verification flows
- classify at least:
  - task_too_broad
  - malformed_tool_use
  - missing_dependency_or_toolchain
  - contract_mismatch
  - verification_failure
  - model_output_format_failure
  - environment_instability
  - method_space_exhausted
  - atomization_floor_reached
- persist the normalized failure class in vault entries, triangulation sessions,
  decomposition receipts, and final blocked outcomes

Success condition:
- every meaningful failure now has a reusable class, not just prose

Why this helps lane detachment:
- the host can route by evidence instead of "which familiar starter do we know"

## Phase 4. Make Failure Class Drive The Next Move

Goal:
- stop relying on human-authored per-case reasoning after each failure

Deliverables:

- add a routing table from failure class to next-action posture:
  - retry differently
  - decompose
  - escalate model ladder
  - vault and resume later
  - promote to triangulation
- ensure the next step is host-chosen and receipt-recorded

Example posture mapping:

- `task_too_broad` -> decomposition grammar selection
- `model_output_format_failure` -> retry posture or next model candidate
- `missing_dependency_or_toolchain` -> host-mechanical/tooling branch
- `atomization_floor_reached` -> no further split, retry or triangulate
- `verification_failure` -> patch/implementation retry with grounded evidence

Success condition:
- the system no longer needs us to narrate what kind of retry to attempt next

Coverage note:

- build fallback and verification
- task-attempt vault and triangulation
- patch plan review and patch constraint review
- patch preflight/postcheck
- declarative paired-proof result receipts
- retry-search ladder proof
- extension governance refresh/drift receipts for proof/composition/patch/helper/bridge
- constraint approval receipts and shelf mutation receipts

can now be treated as the first real selector-covered surfaces.

The next wave after Phase 4 is not inventing the selector again.
It is extending the same selector coverage into:

## Phase 5. Generalize Decomposition Grammars

Goal:
- make decomposition reusable, selected, and learnable rather than bespoke

Deliverables:

- define a grammar library for common split shapes such as:
  - file-creation split
  - host-tooling-vs-model-authorship split
  - backend/frontend split
  - schema/logic/render split
  - helper-definition/helper-integration split
  - patch-diagnosis/patch-application/postcheck split
- map failure class plus task shape to grammar candidates
- persist selected grammar id in decomposition receipts
- store parent-child lineage so later success/failure can teach reuse

Success condition:
- broad tasks decompose through reusable grammar selection instead of one-off
  human repair

Why this helps lane detachment:
- the host can narrow unknown requests mechanically instead of steering them
  back into familiar starter families

## Phase 6. Enforce The Atomization Floor

Goal:
- prevent decomposition into dust

Deliverables:

- make atomization floor decisions a real hard routing gate
- define universal floor rules for:
  - minimum task scope
  - minimum coherent artifact set
  - minimum verification meaning
- once floor is reached, prohibit further split and force one of:
  - retry different method
  - escalate model
  - triangulate
  - block provisionally

Success condition:
- no task can be subdivided forever just because the model is struggling

Why this helps lane detachment:
- smaller models stay usable without forcing the factory back into narrow
  positive lane comfort

## Phase 7. Upgrade The Failure Vault Into A Resumeable Search Memory

Goal:
- turn provisional failure holding into a live continuity mechanism

Deliverables:

- link related vault entries by lineage, failure class, task shape, and
  sticking-point signature
- when a matching failure returns, resume the search from prior evidence instead
  of starting from scratch
- distinguish:
  - same symptom
  - same method failure
  - same floor-level blocker

Success condition:
- a later repeat of the same failure continues the hunt instead of merely
  duplicating notes

## Phase 8. Make Triangulation Narrow Culprits, Not Just Group Failures

Goal:
- move from evidence accumulation to actual culprit narrowing

Deliverables:

- compare retries across:
  - method used
  - model used
  - task shape
  - dependency posture
  - verification failure surface
- derive a narrower culprit signature from convergent failures
- distinguish:
  - current-model exhaustion
  - full-ladder exhaustion
  - floor-level convergent blocker
  - unresolved multi-cause noise

Success condition:
- the system can say "this exact thing in this exact posture is the blocker"
  rather than "that area feels bad"

## Phase 9. Tighten Constraint Promotion

Goal:
- ensure durable negative constraints are earned and narrow

Deliverables:

- require triangulated evidence before approval
- require exact failure conditions
- require scope boundaries
- require provenance links back to vault and triangulation receipts
- require replacement guidance where possible
- reject promotion when evidence is still broad, noisy, or model-specific only

Success condition:
- the approved shelf becomes a precision instrument, not a negative-lane
  graveyard

## Phase 10. Prove Positive-Lane Independence Before Deletion

Goal:
- verify that positive lanes are optional accelerators rather than hidden load
  bearing walls

Deliverables:

- add explicit "lane-disabled" proof runs or request probes
- test unknown stack requests with positive-lane preference disabled
- test that outcomes remain:
  - bounded
  - honest
  - receipt-owned
  - non-fabricated
- measure whether execution primitives, decomposition, retry search, and
  fallback classification still hold up

Success condition:
- the factory can attempt unknown requests without being pulled back into
  familiar positive-family shapes by default

Only after this phase should we consider deleting positive lanes outright.

## What Not To Do

Do not:

- replace old positive lanes with new hidden positive lanes
- encode every new failure as a bespoke negative rule
- treat unsupported family as forbidden reality
- let fallback artifacts report as full success against violated hard
  requirements
- keep decomposing once the floor has been reached
- delete positive lanes before the host has actually absorbed their safety role

## Recommended Immediate Next Slice

If we want the highest-leverage implementation run next, do these in order:

1. build-side intent freeze
2. universal outcome classification
3. generic failure taxonomy

That trio creates the truth layer needed for the later self-improvement loop.

After that, the next cluster is:

1. failure-class-to-next-action routing
2. decomposition grammar generalization
3. hard atomization-floor enforcement

That cluster creates the mechanical learning loop.

Then:

1. resumable vault continuity
2. culprit-narrowing triangulation
3. stricter promotion gate

That cluster creates the trustworthy negative shelf.

## Short Version

The path off positive lanes is not:

- delete them and hope

It is:

- freeze requirement truth
- classify outcomes honestly
- classify failures generically
- route next actions mechanically
- decompose through reusable grammars
- stop at a real atomization floor
- resume unresolved failures from the vault
- triangulate exact culprits
- promote only narrow converged constraints
- then prove positive lanes are optional
- then detach them

That is the clean path to the final form.
