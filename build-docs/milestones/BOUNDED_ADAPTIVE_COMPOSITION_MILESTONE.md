# Bounded Adaptive Composition Milestone

This document defines the next corrective milestone for the ChattyFactory rebuild.

The short version:

- the rebuild now has a strong conveyor belt
- the host owns a lot of deterministic machinery
- the GGUF is no longer doing most of the belt work
- but the system is still too dependent on predeclared lanes when a request is nearby but not directly covered

This milestone is about fixing that gap without putting the GGUF back on the belt.

Read this alongside:

- [DESIGN_INTENT_REVIEW.md](../reviews/DESIGN_INTENT_REVIEW.md)
- [REBUILD_PLAN.md](../plans/REBUILD_PLAN.md)
- [MILESTONE_CHECKPOINT.md](../checkpoints/MILESTONE_CHECKPOINT.md)
- [HELPER_SERVICE_MILESTONE.md](./HELPER_SERVICE_MILESTONE.md)
- [../ARCHITECTURE_LEDGER.md](../ARCHITECTURE_LEDGER.md)
- [../RELIABILITY.md](../RELIABILITY.md)

## 1. Why This Is Next

The rebuild is already strong at:

- deterministic family builds
- deterministic patch lanes
- helper-backed proof lanes
- fallback scaffolding for missing lanes
- host-owned receipts and lifecycle state

But it is still weak in the middle category:

- host cannot fully do the request as-is
- host has some relevant machinery
- request is still close enough that a bounded composition should be possible

Today, these cases tend to fall into one of two buckets:

- a direct deterministic lane exists
- or the request becomes future-lane scaffolding work

That is reliable, but it is not yet the intended design shape.

The intended design shape needs a third bucket:

- host can assemble a bounded solution from existing parts
- GGUF helps classify, rank, critique, and tune that bounded assembly
- host still owns execution and verification

That is what this milestone is for.

## 2. Milestone Goal

Add a bounded adaptive composition layer that can handle:

- nearby unsupported requests
- partial host capability
- helper/bridge/runtime compositions

without collapsing into:

- freeform model authorship
- or immediate "go build a brand-new deterministic lane first"

The key architectural shift is:

- the GGUF should help choose and critique bounded host-owned composition primitives
- the host should assemble, run, and verify the composed path

## 3. The Intended Split

### The host should own

- request freezing
- project grounding
- route classification
- known primitive inventory
- work order assembly
- execution policy
- helper launch/stop/status
- acceptance selection and execution
- receipts
- escalation and fallback artifacts

### The GGUF should own

- bounded interpretation refinement
- selecting among composition options
- ranking nearby routes
- critiquing incomplete plans
- identifying the smallest honest helper/bridge transition
- judging whether host-assembled steps are likely good enough

### The GGUF should not own

- direct arbitrary code execution
- open-ended step authoring as the default
- broad unconstrained re-planning inside execution
- ad hoc machinery assembly without host-readable structure

## 4. What Counts As Bounded Adaptive Composition

For this milestone, a composition means:

- a request is not handled by one direct deterministic lane
- but the host can express a candidate path as a small set of known primitives

Examples of primitives:

- family seed
- helper primitive
- bridge primitive
- operator bundle
- acceptance recipe
- runtime capability
- composition note or route reason

The composed path must stay:

- typed
- inspectable
- reversible
- acceptance-backed

This is not:

- asking the model to invent arbitrary infrastructure
- letting the model emit raw code as the fallback default
- treating composition as a new hidden planning free-for-all

## 5. New Route Categories

The host should classify requests into four categories:

### 1. Direct deterministic lane

Definition:
- a family or patch lane already exists

Current rebuild strength:
- strong

### 2. Bounded composable path

Definition:
- no single deterministic lane covers the ask
- but known host primitives can be composed into an honest path

Current rebuild strength:
- weak

### 3. Helper-needed-but-missing

Definition:
- the host can see the rough shape
- but a required helper/service/bridge primitive is missing

Current rebuild strength:
- medium, mostly through fallback scaffolding

### 4. Truly unsupported

Definition:
- neither direct lane nor bounded composition is honest yet

Current rebuild strength:
- strong fallback behavior

The milestone target is to make category 2 real and reliable.

## 6. Proposed Composition Contract

The exact Rust shapes can evolve, but the host should eventually know something like:

```text
ComposableRoutePlan
- request_id
- route_mode
- composition_kind
- target_family_id
- target_tool_kind
- helper_ids
- bridge_capabilities
- operator_bundle_ids
- acceptance_recipe_ids
- runtime_requirements
- notes
- confidence_band
- should_escalate
```

And a narrower model-facing input like:

```text
ComposableRouteChoice
- route_class
- chosen_family_id
- chosen_helper_ids
- chosen_bridge_capabilities
- chosen_operator_bundle_ids
- chosen_acceptance_recipe_ids
- rationale
```

The host should wrap the model's narrow choice into the fuller host-owned plan.

## 7. Planner Role In This Milestone

The planner should not be asked:

- "write the whole composition plan"

It should be asked smaller questions such as:

- choose among 2-5 candidate composition paths
- choose whether a helper is required now
- choose whether a bridge capability is necessary or optional
- choose whether the current path is good enough or should escalate

This preserves the existing good principle:

- tiny bounded model jobs

while giving the model more meaningful supervisory work than pure lane-id selection.

## 8. Host Deliverables

This milestone should produce:

### A. Composition inventory

A host-readable inventory of reusable primitives such as:

- helper primitives
- bridge primitives
- operator bundles
- acceptance bundles
- runtime facts

### B. Composition classifier

A route classifier that can distinguish:

- direct lane
- bounded composition
- helper-needed-but-missing
- truly unsupported

### C. Composition plan artifact

A typed host-owned artifact for composed paths, persisted in runtime receipts.

### D. Acceptance for composed paths

The host should be able to choose and run acceptance for:

- the seed family
- the helper/bridge attachments
- the resulting visible surface or output

### E. Escalation boundary

When composition is not honest enough, the host should stop and:

- explain why
- emit fallback artifacts
- preserve the attempted composition rationale

## 9. Recommended First Proof

The first proof should be:

- close to existing machinery
- clearly not a direct single lane
- clearly possible through composition

Best candidate shape:

- a helper-backed ChattyCog or static-web request that needs:
  - existing web/module family
  - existing local inbox helper
  - existing helper summary surface lanes
  - existing bridge capabilities

For example:

- "make me a helper-backed monitoring module that watches two inboxes and surfaces filtered file status"

The point is not that the words are special.
The point is that the host should be able to compose:

- existing family
- existing helper
- existing multi-lane helper support
- existing helper summary lanes

without requiring a brand-new handcrafted lane first.

## 10. What Success Looks Like

The milestone succeeds if one real request can:

1. enter as plain language
2. freeze into a host-owned interpretation
3. be classified as bounded composition
4. get a small planner choice over known composition options
5. be assembled by the host into a typed composed path
6. execute with receipts and acceptance
7. avoid both:
   - full freeform model authorship
   - immediate fallback to future-lane scaffolding

That is the real missing proof.

## 11. Risks

### 1. Hidden broad planning

If we let the model describe whole compositions in freeform text, we are backsliding.

### 2. Fake composition

If composition is just "actually a new hardcoded lane in disguise," we have not solved the gap.

### 3. Host complexity explosion

If we add too many primitive types before one proof works, the system gets harder to reason about.

### 4. Dishonest capability blur

If the system cannot clearly say whether it:

- directly supports
- composes from existing parts
- or does not yet support

then we lose reliability and operator trust.

## 12. Recommended Implementation Order

1. define composition route categories in contracts
2. define a small host composition primitive inventory
3. add a narrow planner choice shape for composition selection
4. add host-owned `ComposableRoutePlan` receipts
5. prove one helper-backed composed request
6. only then expand composition breadth

## 13. Bottom Line

This milestone is the next correction needed to match the original ChattyFactory thesis.

We have already proven:

- the belt exists
- the GGUF is off the belt
- the host can do a lot of churn and scaffolding

Now we need to prove:

- the GGUF can do meaningful bounded supervisory work when host machinery is only partial

without returning to:

- model-led freeform execution

and without settling for:

- a pure deterministic mold factory
