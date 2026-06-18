# Current Architecture

This is the primary technical overview for the current `chatty-factory/` system.

Use this document first if you are a developer onboarding to the project.

Historical checkpoint and milestone docs in `build-docs/` are still useful, but
they are supporting context, not the primary source of truth for the current
implementation shape.

The highest-level architectural intent is captured in
[Factory Shape](./FACTORY_SHAPE.md).

## What It Is

ChattyFactory is a Rust workspace for a governed local build-and-patch factory.

At a high level it does four things:

1. turns a plain-language request into a generated project
2. applies follow-up patch requests to existing generated projects
3. records host-owned runtime artifacts and governance receipts for those actions
4. uses bounded model-authored work, decomposition, and triangulation to narrow
   failures instead of treating the model as an unconstrained agent

The intended split is:

- the model interprets, proposes, and reviews within bounded tasks
- the host freezes, executes, verifies, classifies, and records

That split is not a minor implementation detail.
It is the core product boundary:

- the model proposes bounded shape
- the host owns truth

That host-owned truth now includes first-class build outcome labeling.
Execution and fallback flows can surface labels such as:

- `full_success`
- `partial_success`
- `degraded_fallback`
- `requirement_not_met`

based on frozen requirements and actual host receipts rather than optimistic
summary wording alone.

Host-owned truth now also includes a first-class mechanical next-action layer.
The host records:

- normalized failure class
- recommended next action
- recommended next step

from trapped evidence, instead of leaving "what to do next" scattered across ad
hoc fallback wording.

This is now present across:

- build fallback and build verification flows
- execution result summaries for build/patch/proof/helper surfaces
- task-attempt vault and triangulation findings
- patch preflight skip and substrate-attempt flows
- patch postcheck receipts
- retry-search ladder proof receipts

There is now a clearer division of labor between outcome summary and
continuation routing:

- `outcome_class` belongs on operator-facing end-state surfaces
  - execution results
  - fallback results
- `normalized_failure_class` belongs on governed evidence receipts where the
  host is classifying why a bounded attempt or review posture failed
- `recommended_next_action` and `recommended_next_step` belong wherever the
  host must preserve a concrete continuation posture from trapped evidence

In practice that means:

- final execution/fallback surfaces carry both outcome summary and continuation
  posture
- review, verification, triangulation, proof, governance, and shelf-mutation
  receipts usually carry continuation posture without also needing an
  `outcome_class`
- simple artifact receipts such as emitted build or patch receipts should carry
  neither unless they become a real routing surface

The frozen receipt-family audit for this rule lives in
[Receipt Field Ownership Audit](./RECEIPT_FIELD_OWNERSHIP_AUDIT.md).

The practical checklist for future receipt changes lives in
[Receipt Design Gate](./RECEIPT_DESIGN_GATE.md).

## Workspace Map

The workspace root is defined in [Cargo.toml](/C:/Users/User/Desktop/github_portal/chatty-factory/Cargo.toml:1).

Current crates:

- `chatty_factory_core`
  - shared contracts, runtime config, heuristics, proof/runtime primitives
- `chatty_factory_control`
  - route/control-plane support
- `chatty_factory_host`
  - orchestration layer and main product engine
- `chatty_factory_families`
  - built-in starter/family implementations
- `chatty_factory_templates`
  - template/rendering support
- `chatty_factory_verify`
  - acceptance/verification helpers
- `chatty_factory_cli`
  - operator/developer CLI
- `chatty_factory_ui`
  - egui desktop control surface

## Source Of Truth

The live operational state of the system is not only in memory.
It is persisted under `runtime/` as host-owned receipts and summaries.

Important runtime families include:

- `build_plans/`
- `build_intent_freezes/`
- `build_plan_reviews/`
- `build_execution_work_orders/`
- `plan_tasks/`
- `model_task_attempts/`
- `task_decomposition_receipts/`
- `failure_vault/`
- `triangulation_sessions/`
- `constraint_promotion_candidates/`
- `retry_search_proofs/`
- `build_verification_receipts/`
- `fallback_build_specs/`
- `fallback_plan_receipts/`
- `patch_diagnoses/`
- `patch_intent_freezes/`
- `patch_receipts/`

For a developer debugging behavior, the runtime receipts are often more useful
than the UI because they preserve the host’s actual decisions and evidence trail.

## Current Request Flow

The current build side is roughly:

1. request normalization and route/starter choice
2. if there is no exact family match, the host may still select a nearest honest
   starter substrate and continue as a scaffold-first attempt
3. frozen build intent
4. frozen build plan artifact
5. plan self-review and constraint review
6. build execution work order
7. frozen task graph
8. task execution across:
   - host-sync
   - host-mechanical
   - model-authored microtasks
9. verification and acceptance receipts
10. normalized failure classification and host-owned next-action selection
11. decomposition or triangulation if work still fails narrowly

The current patch side is roughly:

1. project diagnosis
2. intent freeze
3. patch-plan review
4. if no named patch lane matches but the project is still grounded, the host
   may continue as a substrate-first patch attempt under bounded soft review
5. bounded patch execution
6. postcheck and patch receipt classification
7. normalized failure classification and host-owned next-action selection

The exact continuation policy is documented in
[Bounded Soft-Review Continuation](./BOUNDED_SOFT_REVIEW_CONTINUATION.md).

## Current Model Posture

The project no longer treats the model as the sole planner/executor for whole
features.

Instead it is moving toward:

- smaller frozen tasks
- bounded retry postures
- model ladder escalation when one model posture is exhausted
- decomposition when a task is still too broad
- triangulation when repeated failures need narrow classification
- host-owned mechanical next-action selection keyed from normalized failure class

This is visible in:

- host proof flow in [crates/chatty_factory_host/src/lib.rs](/C:/Users/User/Desktop/github_portal/chatty-factory/crates/chatty_factory_host/src/lib.rs:7319)
- triangulation summary derivation in [crates/chatty_factory_host/src/lib.rs](/C:/Users/User/Desktop/github_portal/chatty-factory/crates/chatty_factory_host/src/lib.rs:6413)
- retry-search proof receipt loading in [crates/chatty_factory_ui/src/main.rs](/C:/Users/User/Desktop/github_portal/chatty-factory/crates/chatty_factory_ui/src/main.rs:6968)

## Triangulation And Negative Constraints

The current design direction is not “add one more negative rule every time a
task fails.”

The intended chain is:

1. failure enters a provisional vault
2. alternate methods or decomposed variants are tried
3. the host selects the next action mechanically from normalized failure class
4. evidence is grouped into triangulation sessions
5. only repeated convergent floor-level evidence becomes a promotion candidate
6. only then does something approach the real constraint shelf

The key distinction is:

- constraint principles are generic
- failure classes are generic
- decomposition grammars are reusable
- next-action routing is host-owned and mechanical
- durable negative constraints should be narrow and triangulated

See also:
- [Contract Inventory](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/CONTRACT_INVENTORY.md:294)
- [Triangulation And Atomization Floor Plan](/C:/Users/User/Desktop/github_portal/chatty-factory/build-docs/plans/TRIANGULATION_AND_ATOMIZATION_FLOOR_PLAN.md:5)

## Runtime And Proof Posture

The runtime layer now tracks:

- launch timeout
- planner request timeout
- model-task request timeout
- shell timeout buffer recommendation

The retry-search ladder proof additionally records:

- model candidate count
- retry posture count
- per-attempt budget
- expected outer timeout ceiling
- final outcome
- internal timeout evidence
- normalized failure class
- recommended next action
- recommended next step

A concrete repeatable example is documented in
[Qwen3-8B Retry-Search Hostile Proof](./QWEN3_8B_RETRY_SEARCH_HOSTILE_PROOF.md).

That means shell timeout alone is not the authoritative proof result.
The receipt is.

## Current Supported Families

The current built-in families are documented in [README.md](/C:/Users/User/Desktop/github_portal/chatty-factory/README.md:18) and include:

- static web dashboard
- Chatty-Cog webview module
- Chatty-Cog native-window module
- Chatty-EDU native-window module
- Chatty-Cog + Chatty-EDU native-window module
- Chatty-Cog workspace module
- Python CLI tool
- Rust CLI tool

## Operator Surfaces

The main operator surfaces are:

- CLI for direct build, patch, proof, and refresh commands
- Desktop UI for project, runtime, governance, proof, and receipt-oriented workflows

The UI currently surfaces:

- runtime status
- proof runtime posture
- retry-search ladder proof launch
- project patchability
- extension/governance panels
- negative shelf and triangulation summaries

Extension governance is no longer only descriptive drift/status output.
Proof/composition/patch/helper/bridge governance receipts now persist
normalized failure class plus host-selected next action and next step, and the
desktop governance panels surface that selector posture directly.

## Known Engineering Risks

The biggest current risks are structural rather than conceptual.

1. File concentration
   - `chatty_factory_host/src/lib.rs` is very large
   - `chatty_factory_ui/src/main.rs` remains a large composition root

2. Thin automated coverage
   - the workspace tests pass, but the number of tests is small relative to the
     orchestration surface

3. Receipt sprawl
   - the runtime artifact model is a strength, but it needs explicit retention
     and lifecycle discipline

4. Documentation layering
   - many historical planning/checkpoint docs still exist and can be mistaken
     for current implementation truth if not framed clearly

## Live Docs

Treat these as the main current references:

- [README.md](/C:/Users/User/Desktop/github_portal/chatty-factory/README.md)
- [USER_MANUAL.md](/C:/Users/User/Desktop/github_portal/chatty-factory/USER_MANUAL.md)
- [Negative Constraints Engine Parts List](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/NEGATIVE_CONSTRAINTS_ENGINE_PARTS_LIST.md)
- [Negative Constraints Engine Gap Audit](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/NEGATIVE_CONSTRAINTS_ENGINE_GAP_AUDIT.md)
- [Negative Constraints Engine Implementation Sequence](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/NEGATIVE_CONSTRAINTS_ENGINE_IMPLEMENTATION_SEQUENCE.md)
- [Contract Inventory](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/CONTRACT_INVENTORY.md)
- [UI Module Structure](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/UI_MODULE_STRUCTURE.md)
- this document

Treat these as historical/supporting context:

- `build-docs/checkpoints/*`
- older milestone plans in `build-docs/plans/*`
- repository layout proposal docs

## Developer Start Order

For a senior developer joining cold, the fastest path is:

1. read this document
2. read [README.md](/C:/Users/User/Desktop/github_portal/chatty-factory/README.md)
3. read [USER_MANUAL.md](/C:/Users/User/Desktop/github_portal/chatty-factory/USER_MANUAL.md)
4. inspect [docs/CONTRACT_INVENTORY.md](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/CONTRACT_INVENTORY.md)
5. inspect `runtime/` receipts from a recent run
6. then open host/UI implementation code
