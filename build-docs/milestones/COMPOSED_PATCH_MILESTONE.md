# Composed Patch Milestone

## Why This Is Next

ChattyFactory now has a real bounded-composition build path:
- host classifies direct lane vs bounded composition vs fallback
- host assembles a bounded composition work order
- the GGUF can review that bounded plan
- the host executes it and corrects unsafe drift
- the host persists the resulting route and step receipts

That is a meaningful architectural step, but it is still strongest on fresh builds.

The next proof should be a composed patch against an existing active project.

That is the most important user-facing missing middle:
- not a direct deterministic patch lane
- not a totally unsupported request
- a nearby follow-up the host can satisfy by combining existing primitives

If we can prove that cleanly, bounded composition becomes a real cross-cutting execution layer rather than a build-only special case.

## Design Goal

Take a patch request that is:
- too broad for one existing patch lane
- close enough to current machinery that it should not fall straight to extension scaffolding

Then have the factory:
1. freeze the request and active-project grounding
2. classify it as `bounded_composition_candidate`
3. assemble a host-owned composed patch work order from known primitives
4. optionally run narrow GGUF review on that work order
5. execute from the work order
6. run acceptance
7. persist the full composition ledger

The GGUF stays beside the belt:
- reviewing primitive choice
- narrowing or reordering when appropriate
- flagging suspicious omissions

The host stays on the belt:
- building the work order
- enforcing dependency and safety rules
- executing primitives
- running verification
- journaling receipts

## Target Proof Shape

The first proof should use an existing family with enough primitive depth to make composition realistic.

Best candidate:
- `chattycog_webview_module`

Why:
- it already has a helper-backed surface
- it already has bridge and helper primitives
- it already has summary, filter, inbox, and preview primitives
- it already has one successful composed build proof

Good proof request shape:
- patch an active helper-backed ChattyCog webview module to add a nearby bundle of behavior such as:
  - filtered helper monitoring
  - selected preview behavior
  - helper status and lane metadata

The proof should require:
- multiple primitives
- dependency ordering
- GGUF review
- host safety correction when needed

But it should not require inventing a brand-new lane.

## Architectural Scope

### 1. First-Class Composed Patch Work Order

The host should be able to build a `ComposableRoutePlan` for patch flows, not only new builds.

Minimum requirements:
- active project name
- target family/tool
- candidate patch primitives
- reviewed patch primitives
- selected helper primitives
- selected bridge primitives
- selected acceptance bundle choices
- route class
- review outcome
- execution status

### 2. Patch-Specific Primitive Semantics

The host already knows some bounded composition semantics:
- required vs optional companion
- dependency ordering
- simple exclusivity

For composed patches, those semantics should explicitly apply to:
- already-present project features
- newly selected primitives
- helper-backed runtime expectations

### 3. Patch Composition Receipts

The host should persist a clean ledger for composed patch execution:
- plan receipt
- review receipt
- per-step patch receipts
- helper/runtime step receipts when relevant
- final acceptance result

The ledger should be easy to inspect without inferring state from logs.

### 4. Honest Escalation Rules

The host must still stop cleanly when a patch request is:
- too broad
- missing required helper/service machinery
- asking for mutually incompatible primitive bundles
- effectively requesting a new deterministic lane rather than composition

Composed patching should expand the missing middle, not hide unsupported requests.

## Implementation Order

1. Add composed patch classification and host routing.
   - let patch flows enter `bounded_composition_candidate`
   - do not force immediate fallback just because no single patch lane matched

2. Build a patch-side composition primitive selector.
   - candidate primitive list from existing registry data
   - grounded to the active project’s current features

3. Reuse bounded GGUF composition review for patch work orders.
   - review candidate patch primitive list
   - allow reorder / trim
   - keep host dependency correction

4. Add patch-specific execution from `ComposableRoutePlan`.
   - execute from selected primitives
   - persist per-step receipts

5. Prove one real composed patch request.
   - active project
   - multiple primitives
   - acceptance passing

## What Counts As Success

This milestone is successful when all of the following are true:

- A real patch request is classified as `bounded_composition_candidate`.
- The host assembles a patch work order from existing primitives.
- The GGUF reviews that bounded work order without writing machinery.
- The host executes the reviewed patch plan.
- Dependency-safe correction happens host-side when the reviewed plan is incomplete.
- Acceptance passes.
- The resulting route, review, and execution receipts make the composition understandable after the fact.

## What Not To Do

- Do not solve this by adding one giant direct patch lane that hardcodes the whole proof.
- Do not move freeform code-writing back into the GGUF.
- Do not hide composition logic inside prompt text alone.
- Do not blur fallback and composition; unsupported patch requests still need honest stops.

## Why This Matters

This is the next architectural hinge point.

Without composed patch execution, the factory remains:
- strong at direct deterministic lanes
- strong at composed builds
- weaker where users most naturally iterate: follow-up change requests

With composed patch execution, the factory gets much closer to the intended shape:
- host owns the conveyor
- GGUF guides and critiques bounded work orders beside it
- nearby unsupported requests can be satisfied without pretending every useful behavior must already exist as one named lane
