# Next Milestone

This document is the immediate working checkpoint for the ChattyFactory rebuild.

It exists to answer one question before we keep building:

- what did the old prototype already earn the hard way,
- what has the rebuild already replaced cleanly,
- and what still needs to be rebuilt before we sprint into more families and features?

Read this alongside:
- [REBUILD_PLAN.md](./REBUILD_PLAN.md)
- [docs/CONTRACT_INVENTORY.md](./docs/CONTRACT_INVENTORY.md)
- [../ARCHITECTURE_LEDGER.md](../ARCHITECTURE_LEDGER.md)
- [../RELIABILITY.md](../RELIABILITY.md)
- [../PARTS_MAP.md](../PARTS_MAP.md)

## Current Position

The rebuild is already proving the right architectural split:

- the host owns routing receipts, family scaffolds, acceptance, runtime discovery, planner handoffs, project session state, and UI-facing state
- the local LLM is being pushed into tiny chooser tasks instead of broad authorship

That is real progress.

The main risk now is not "the rebuild is off course."

The main risk is:
- accidentally rebuilding a cleaner-looking prototype without fully restoring the prototype's hard-earned safety, grounding, and fallback spine

## What The Rebuild Already Covers Well

These areas are no longer the main concern:

### 1) Foreman-style planning direction

The rebuild already has:
- `RequestRecord`
- `RequestPlan`
- `RouteDecision`
- planner handoffs and planner responses
- confidence scoring
- tiny-choice planner tasks
- explicit build vs patch routing

This is aligned with the core thesis.

### 2) Deterministic family output

The rebuild already has working deterministic lanes for:
- `static_web_dashboard`
- `chattycog_basic_dashboard`
- `python_cli_tool`
- `rust_cli_tool`

It also already supports deterministic patch lanes for selected family/tool combinations.

### 3) Shared host layer

The rebuild already has a meaningful host API surface through:
- `chatty_factory_host`
- `chatty_factory_cli`
- `chatty_factory_ui`

This is important because the UI and CLI are no longer drifting into separate machinery stacks.

### 4) Local planner integration

The rebuild already knows how to:
- discover local runtime assets
- select from a model catalog
- run local planner calls
- fall back across model tiers
- record planner receipts

That gives us a real local planning loop rather than just a paper architecture.

## What The Old Prototype Still Does Better

These are the main gaps that still matter.

### 1) Execution safety and policy enforcement

The old prototype runner has stronger guardrails around:
- path confinement
- safe relative paths
- nested Cargo project handling
- Python syntax smoke checks
- post-step smoke checks
- failure shaping before retries

The rebuild currently has acceptance and receipts, but not yet the same host-owned execution spine.

### 2) Snapshot grounding

The old prototype has stronger grounding around:
- frozen interpretation
- anchor context
- project snapshots
- snapshot gate checks before execution

The rebuild currently relies more on project receipts and `ProjectSpec`, which is useful but not enough on its own for future broader execution.

### 3) Honest unsupported-case fallback

The old prototype has more explicit lifecycle handling for:
- clarification
- planning conversation
- compile/run boundaries
- recovery phases

The rebuild currently has a good narrow planner handoff, but not yet a fully honest fallback lane for requests that do not fit a deterministic family or patch registry.

### 4) Stronger ecosystem contract awareness

The old prototype and surrounding docs carry more detail about:
- module packaging expectations
- wrapper compatibility
- bridge/reporting behavior
- hosted module assumptions

The rebuild currently emits useful ChattyCog wrapper files, but not yet a fully modeled ChattyCog module contract.

## Main Risks If We Ignore This

If we keep adding new families and features before filling these gaps, we risk:

- growing a second spaghetti system with better receipts
- making route/build behavior wider without making it safer
- having planner-guided output that still lacks strong workspace grounding
- shipping ChattyCog wrappers that look right but miss important hosted-module behaviors

## Next Milestone Goal

The next milestone should be:

- restore the prototype's safety, grounding, and fallback strengths
- in rebuild-native, host-owned form

This is more important than adding lots of new families right now.

## Milestone Workstreams

## 1) Host Safety And Execution Layer

Purpose:
- rebuild the old prototype's guardrail spine cleanly inside the new host

Scope:
- path confinement
- safe workspace-relative file policy
- allowed command policy by family/substrate
- nested project guardrails
- Python/Rust smoke checks
- typed execution receipts

Suggested outputs:
- `ExecutionPolicy`
- `ExecutionReceipt`
- `CommandAllowance`
- `SubstrateSmokePlan`

Acceptance:
- build and patch execution always records what command/file actions were allowed
- nested Rust project generation is explicitly guarded
- Python family outputs get a deterministic syntax smoke lane
- failure reports distinguish policy rejection from acceptance failure

## 2) Snapshot Grounding Layer

Purpose:
- ensure build and patch lanes operate against explicit grounded workspace context

Scope:
- project snapshot receipt
- active project context bundle
- file/path gating against known project state
- route-time capability honesty

Suggested outputs:
- `ProjectSnapshot`
- `ContextBundle`
- `SnapshotGateResult`

Acceptance:
- patch lanes can explain which files were eligible and why
- broader execution lanes cannot target paths outside grounded project scope
- host can persist a compact project context artifact before execution

## 3) Honest Fallback Planning Lane

Purpose:
- support plain-language requests that do not fit known deterministic families without collapsing into hidden chaos

Scope:
- clarify lane
- structured build-spec lane
- planner-produced fallback plan artifact
- bounded execution handoff

Suggested outputs:
- `ClarificationRequest`
- `FallbackBuildSpec`
- `FallbackPlanReceipt`

Acceptance:
- unsupported requests do not silently coerce into the wrong family
- the system can stop and explain why a deterministic lane was unavailable
- the planner fallback stays typed and inspectable

## 4) ChattyCog Module Contract Pass

Purpose:
- turn current wrapper emission into a first-class, honest module compatibility lane

Scope:
- `manifest.json`
- `visual_load.json`
- `HANDSHAKE.md`
- optional bridge contract expectations
- module packaging expectations

Suggested outputs:
- `ChattyCogModuleSpec`
- `ChattyCogBridgeSpec`
- wrapper validation checks

Acceptance:
- generated ChattyCog-targeted outputs map cleanly onto documented module expectations
- wrapper files are validated as a coherent bundle, not just emitted individually
- module-family docs clearly state what is standalone, what is hosted, and what is optional bridge behavior

## 5) Registry Lift And Cleanup

Purpose:
- keep the rebuild from re-hardcoding product behavior in Rust branches as it grows

Scope:
- move more patch/operator/acceptance behavior into registries
- reduce code-branch routing where ids and manifests should drive decisions
- define durable output vs disposable build byproducts

Suggested outputs:
- expanded registry files
- artifact retention policy
- output hygiene rules

Acceptance:
- family capability/patch/operator data is increasingly registry-owned
- output projects do not blur durable product files with disposable build clutter without an explicit policy

## Recommended Build Order

Implement in this order:

1. Host safety and execution layer
2. Snapshot grounding layer
3. Honest fallback planning lane
4. ChattyCog module contract pass
5. Registry lift and cleanup

This order matters.

It keeps us from widening the build surface before the host regains the prototype's strongest protective muscles.

## Not The Main Priority Right Now

These can keep moving, but they should not outrank the milestone above:

- adding many more build families
- making the UI prettier
- adding deeper planner cleverness
- increasing patch breadth without stronger grounding

Those are good later multipliers, but they are not the most important structural work this minute.

## Practical Next Implementation Pass

If we continue immediately after this planning checkpoint, the best next concrete pass is:

1. add a rebuild-native execution/policy module under `chatty_factory_core` or `chatty_factory_host`
2. add typed execution receipts and substrate smoke checks
3. wire those checks into existing Python/Rust build and patch lanes
4. only then begin the snapshot grounding pass

That gives the rebuild a stronger backbone before we expand its reach.
