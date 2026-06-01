# Design Intent Review

This document checks the current rebuild against the original ChattyFactory intent.

It exists to answer:

- where the rebuild is genuinely aligned
- where it is drifting
- what is still missing
- what correction should come next

Read this alongside:

- [REBUILD_PLAN.md](../plans/REBUILD_PLAN.md)
- [MILESTONE_CHECKPOINT.md](../checkpoints/MILESTONE_CHECKPOINT.md)
- [HELPER_SERVICE_MILESTONE.md](../milestones/HELPER_SERVICE_MILESTONE.md)
- [../ARCHITECTURE_LEDGER.md](../ARCHITECTURE_LEDGER.md)
- [../RELIABILITY.md](../RELIABILITY.md)

## Original Intent

The intended shape is not:

- the GGUF writes the whole product directly
- the host is a thin wrapper around model output
- the factory becomes a rigid template mill

The intended shape is:

- user request comes in
- the system freezes a concrete interpretation of that request
- the host owns as much grunt, churn, scaffolding, verification, and state handling as possible
- the GGUF sits beside the conveyor belt, not on it
- the GGUF guides:
  - planning
  - routing
  - quality control
  - bounded gap-filling
- the GGUF only falls onto the belt itself when the machinery truly cannot yet handle the gap

The long-term goal is a factory that is:

- more code/build agnostic than a fixed template system
- still reliable with small or local models
- able to use the model's native dataset as a meaningful capability amplifier when host machinery is incomplete

## Where The Rebuild Is Aligned

### 1. Interpretation and grounding are much stronger

The rebuild now has real host-owned grounding through:

- request normalization
- route-mode selection
- selected project session
- active project session
- project browser state
- project snapshots
- context bundles
- snapshot gates
- typed project contracts

That is much closer to the intended:

- "freeze the interpretation"
- then operate deterministically against known state

than the old prototype's looser conversational drift.

### 2. The host now owns a large amount of machinery

The host currently owns:

- deterministic family builds
- deterministic patch lanes
- runtime discovery and smoke checks
- acceptance execution
- execution policy
- helper receipts
- fallback artifacts
- extension lane lifecycle
- lane promotion workflow
- UI-facing state and registry surfaces

This is strongly aligned with the intended division of labor:

- host does boring exact work
- model does narrower supervisory work

### 3. The GGUF is increasingly beside the belt, not on it

The local planner is now used mainly for bounded choice tasks such as:

- follow-up request mode
- family/tool choice
- patch recipe choice
- operator bundle choice
- acceptance recipe choice

That is a major improvement over broad "model writes everything" behavior.

### 4. Honest fallback exists

Unsupported asks now produce:

- clarification artifacts
- fallback build specs
- fallback receipts
- extension stub bundles
- promotion-ready lane work

So the rebuild no longer needs to pretend that every request can be satisfied directly by improvisation.

This is strongly aligned with the intended factory philosophy.

## Where The Rebuild Is Drifting

The main drift is not "too much model."
The main drift is "too much deterministic mold."

### 1. Success is still heavily lane-shaped

The rebuild is strongest when:

- a family already exists
- a patch lane already exists
- a helper lane already exists
- a registry-backed route already exists

This gives reliability, but it also means capability is still closely tied to:

- what we have already turned into host machinery

rather than:

- what the model can responsibly help compose when machinery is partial

### 2. The GGUF is often choosing levers, not yet shaping bounded compositions

Right now the GGUF is good at:

- selecting a route
- selecting a family
- selecting a patch recipe
- selecting a bundle
- selecting an acceptance recipe

That is useful, but it is still closer to:

- "smart selector over known lanes"

than:

- "bounded adaptive supervisor when the host only has partial machinery"

### 3. Unsupported asks mostly become future-lane work

Fallback is strong, but the current pattern is often:

- unsupported request
- scaffold next lane
- promote next lane
- implement future host capability

That is excellent for factory growth.
But it means the model's dataset is still helping us:

- build tomorrow's machinery

more than it is helping with:

- today's near-miss request

when the machinery is only partially present.

### 4. We risk becoming a very good template-and-operator factory

This is the central design risk.

If we only keep adding:

- more families
- more patch lanes
- more helper lanes
- more registry entries

without a stronger bounded-composition layer, then the rebuild becomes:

- more reliable
- more inspectable
- but still mainly a deterministic mold system

That would fall short of the deeper intent.

## What Is Still Missing

### 1. A bounded adaptive composition layer

The rebuild still needs a middle layer between:

- "fully deterministic lane exists"
- and "fallback into future-lane scaffolding"

The missing layer should let the GGUF help compose a bounded solution from partial machinery such as:

- known helper primitives
- known bridge primitives
- known operator bundles
- known runtime capabilities
- known acceptance shapes

without requiring a brand-new fully hardcoded lane every time.

### 2. Stronger helper/service decomposition

The `local_inbox_helper` proof is the right beginning.

But one helper proof is not yet the same as a general helper/service foundation.

What is still missing:

- helper composition patterns
- helper-backed route selection beyond a single proof shape
- host-readable helper capability inventory
- helper-aware planner critique over partial helper assemblies

### 3. More meaningful GGUF work when machinery is partial

The next intended role for the GGUF should be:

- decompose a nearby unsupported ask into bounded host-owned pieces
- rank or critique available partial routes
- identify the smallest honest helper/bridge transition
- review whether a host-assembled path is good enough

That is a better use of the model's dataset than simply asking it to hand-author whole code artifacts.

### 4. A clearer distinction between:

- "host can already do this"
- "host can compose this from existing parts"
- "host cannot do this and must grow a new lane"

Today, the rebuild is strong at the first and third categories.

The second category is still weak, and that is the biggest remaining gap against the original intent.

## Current Assessment

If we score the rebuild against the intended philosophy:

- request freezing and grounding: strong
- host-owned machinery: strong
- GGUF off the belt: strong
- GGUF as route/quality-control neighbor: medium
- code/build agnostic bounded gap-filling: weak to medium
- adaptive helper/service decomposition: early but promising

So the rebuild is directionally right, but not yet fully there.

It is already much healthier than the old prototype.
But it is still more:

- deterministic lane factory

than:

- adaptive host machinery with model-supervised bounded composition

## Recommended Correction

The next corrective milestone should not mainly be:

- more tiny patch lanes
- more UI polish
- more registry surfaces

The next corrective milestone should be:

## Bounded Adaptive Composition

That means adding a host-and-planner layer that can classify a request into:

- direct deterministic lane
- helper-backed composition from existing machinery
- helper-needed-but-missing
- truly unsupported

And then, when a request is nearby but not directly covered:

- let the GGUF choose from a small host-owned set of composition primitives
- let the host assemble the actual work order
- let the GGUF critique, refine, or rank the bounded plan
- only fall back to future-lane scaffolding when composition is genuinely insufficient

This keeps the GGUF:

- beside the belt
- valuable
- dataset-amplifying

without putting it back on the belt as the main mechanical worker.

## Concrete Next Milestone

The next milestone should prove one real "nearby but not pre-laned" request that succeeds through bounded composition instead of immediately requiring a brand-new deterministic lane.

Good candidate shapes:

- helper-backed dashboard ask that can be assembled from:
  - existing webview module
  - existing local inbox helper
  - existing helper summary surfaces
  - existing bridge lanes

- small service/helper ask that can be decomposed into:
  - helper
  - summary/status bridge
  - operator surface
  - acceptance bundle

Success would mean:

- the host does the machinery
- the GGUF meaningfully improves route quality
- the system does not collapse into freeform model authorship
- the request does not need a totally new handcrafted lane first

## Bottom Line

The rebuild has already succeeded at:

- building a real conveyor belt
- moving the GGUF off the belt
- making the host do much more of the mechanical work

But it has not yet fully succeeded at:

- giving the GGUF enough bounded adaptive supervisory work when host machinery is only partial

That is the next correction to make if ChattyFactory is going to become:

- more than a reliable template-and-lane factory
- while still staying deterministic and inspectable
