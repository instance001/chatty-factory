# Current Architecture

This is the primary technical overview for the current `chatty-factory/`
system.

Use this document first if you are onboarding to the project.

Historical checkpoint and milestone docs in `build-docs/` still exist, but they
are supporting context rather than the primary source of truth.

The highest-level architectural intent is captured in
[Factory Shape](./FACTORY_SHAPE.md) and the stricter doctrine pivot is captured
in [Negative Lane Runtime Pivot](./NEGATIVE_LANE_RUNTIME_PIVOT.md).

## What It Is

ChattyFactory is a Rust workspace for a governed local build-and-patch factory.

At a high level it does four things:

1. turns a plain-language request into a bounded build attempt
2. applies follow-up patch requests to existing generated projects
3. records host-owned runtime artifacts and governance receipts for those actions
4. uses decomposition, triangulation, and verification to narrow failure before
   escalating method, toolchain, or model

The intended split is:

- the user carries the intent
- the model carries the method
- the host carries the funnel
- the output carries the artifact

The host owns:

- attempt freezing
- execution boundaries
- receipts
- verification
- normalized failure classification
- next-attempt selection posture

The host is not meant to own a positive-lane family catalog as runtime truth.

## Workspace Map

The workspace root is defined in
[Cargo.toml](/C:/Users/User/Desktop/github_portal/chatty-factory/Cargo.toml:1).

Current crates:

- `chatty_factory_core`
  - shared contracts, runtime config, heuristics, and core execution primitives
- `chatty_factory_control`
  - route/control-plane support
- `chatty_factory_host`
  - orchestration layer and main product engine
- `chatty_factory_verify`
  - verification helpers
- `chatty_factory_cli`
  - operator/developer CLI
- `chatty_factory_ui`
  - egui desktop control surface
- `chatty_factory_families`
  - residual compatibility/mechanical code still being reduced during surgery
- `chatty_factory_templates`
  - residual template support still being reduced during surgery

The last two crates are not supposed to reintroduce host-owned positive-lane
authority.

## Runtime Artifacts

The live operational state of the system is persisted under `runtime/` when the
factory runs.

Important runtime surfaces include:

- `clarifications/`
- `planner_handoffs/`
- `composable_route_plans/`
- `next_attempt_build_specs/`
- `next_attempt_receipts/`
- `build_verification_receipts/`
- `build_intent_freezes/`
- `build_execution_work_orders/`
- `plan_tasks/`
- `model_task_attempts/`
- `task_decomposition_receipts/`
- `failure_vault/`
- `triangulation_sessions/`
- `constraint_promotion_candidates/`
- `patch_diagnoses/`
- `patch_intent_freezes/`
- `patch_receipts/`

For debugging behavior, the runtime receipts are often more useful than the UI
because they preserve the host's actual evidence trail and next-attempt
reasoning.

## Request Flow

The build side is roughly:

1. normalize the request
2. freeze a bounded substrate or execution attempt if one can be justified
3. record route and intent artifacts
4. review the bounded plan
5. execute bounded work across host-mechanical and model-authored steps
6. verify the result
7. classify failure or success from receipts
8. choose the next attempt mechanically from evidence when needed
9. decompose, triangulate, escalate toolchain, or escalate model only when justified

The patch side is roughly:

1. diagnose the existing project
2. freeze patch intent
3. review the bounded patch posture
4. execute the bounded patch
5. run postcheck and verification
6. classify result and select the next attempt mechanically when needed

The continuation policy is documented in
[Bounded Soft-Review Continuation](./BOUNDED_SOFT_REVIEW_CONTINUATION.md), but
the architectural direction is now gauntlet-first rather than fallback-to-shape.

## Outcome And Continuation Posture

The host now distinguishes between:

- `outcome_class`
  - what happened
- `normalized_failure_class`
  - why the bounded attempt failed
- `recommended_next_action`
  - what kind of next move is justified
- `recommended_next_step`
  - the human-readable wording of that move

Those fields are meant to carry evidence-driven continuation posture.
They are not meant to carry product-family substitution authority.

## Current Model Posture

The project no longer treats the model as a whole-feature agent that the host
simply trusts.

Instead it is moving toward:

- smaller frozen tasks
- bounded retries
- adaptive decomposition
- triangulation before durable negative promotion
- host-owned next-attempt posture derived from evidence

This keeps the model responsible for method inside a bounded lane while the host
remains responsible for truth.

## Operator Surfaces

The main operator surfaces are:

- CLI for build, patch, proof, governance, and runtime operations
- desktop UI for project, verification, proof, governance, and receipt-oriented workflows

The UI is meant to expose runtime truth, not teach a positive-lane worldview.

## Known Engineering Risks

The biggest current risks are structural rather than conceptual:

1. File concentration
   - `chatty_factory_host/src/lib.rs` is very large
   - `chatty_factory_ui/src/main.rs` remains a large composition root
2. Thin automated coverage relative to orchestration surface area
3. Receipt sprawl and retention discipline
4. Historical docs that can still be mistaken for current truth if left unframed

## Live Docs

Treat these as the main current references:

- [README.md](/C:/Users/User/Desktop/github_portal/chatty-factory/README.md)
- [USER_MANUAL.md](/C:/Users/User/Desktop/github_portal/chatty-factory/USER_MANUAL.md)
- [Factory Shape](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/FACTORY_SHAPE.md)
- [Negative Lane Runtime Pivot](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/NEGATIVE_LANE_RUNTIME_PIVOT.md)
- [Negative Constraints Engine Parts List](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/NEGATIVE_CONSTRAINTS_ENGINE_PARTS_LIST.md)
- [Negative Constraints Engine Gap Audit](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/NEGATIVE_CONSTRAINTS_ENGINE_GAP_AUDIT.md)
- [Negative Constraints Engine Implementation Sequence](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/NEGATIVE_CONSTRAINTS_ENGINE_IMPLEMENTATION_SEQUENCE.md)
- [Receipt Field Ownership Audit](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/RECEIPT_FIELD_OWNERSHIP_AUDIT.md)
- [Receipt Design Gate](/C:/Users/User/Desktop/github_portal/chatty-factory/docs/RECEIPT_DESIGN_GATE.md)

Treat `build-docs/` and older route sketches as historical/supporting context
unless they have been explicitly refreshed.
