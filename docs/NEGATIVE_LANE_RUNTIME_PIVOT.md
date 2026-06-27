## Negative-Lane Runtime Pivot

ChattyFactory is being pulled back to the architecture it was supposed to have:

- the LLM carries the positive implementation patterns
- the host owns safety, truth, and evidence

The host should not keep a parallel product identity built around positive
families, starter catalogs, or mechanical starter choice.

## What Stays Host-Owned

The host still owns:

- bounded execution
- intent freeze
- verification
- receipts
- fallback honesty
- failure classification
- negative constraints
- retry, decomposition, and triangulation evidence

Those are the parts that make a negative-lane factory safe.

## What Stops Being Host-Owned

The host is being retired from:

- family choice as runtime truth
- starter choice as operator control
- positive-lane catalogs as the product center
- "supported family" framing as the definition of allowed reality
- positive artifact substitution disguised as fallback

The model can already supply positive patterns. The host should constrain and
verify them rather than duplicating them.

## Fallback Spine

Fallback is not host selection.

Fallback is the gauntlet.

The host does not say:

- the nearest family is good enough
- the host should substitute an easier scaffold
- this request probably meant a web app, CLI, or dashboard instead
- a near-enough positive template counts as honest recovery

The host does say:

- this attempt failed for these specific reasons
- the next attempt must differ in this specific constrained way
- these constraints were promoted from evidence
- this retry is narrower, smaller, or differently decomposed for a reason

The host carries:

- receipts
- failure classes
- constraint memory
- retry rules
- permission boundaries
- verification gates

The host does not carry:

- positive template fallback
- family rescue paths
- "near enough" scaffold authority
- build-shape substitution pretending to be fallback

## Fallback Order

The negative-lane fallback order is:

1. attempt under frozen intent
2. fail honestly
3. classify failure
4. retry differently under evidence
5. decompose smaller
6. triangulate against failure library and prior receipts
7. compare against prior receipts
8. narrow or reshape the attempt under promoted constraints
9. alter method or toolchain only when justified by evidence
10. swap model only when the pattern suggests model limitation

Never fallback:

- the host invents the desired positive artifact shape
- the host silently broadens or mutates the user goal
- the host chooses an easier product form and calls it recovery

That is the negative lane.

## Current Removal Status

Already removed or reduced:

- operator-facing starter controls in the CLI and UI
- starter-focused usage summaries and recommendation surfaces
- product docs that framed the system around eight supported families
- starter metadata in the build freeze, build plan, review, work order, and
  receipt artifacts

Still deeply coupled:

- core request normalization still produces `candidate_family_ids`
- request planning still revolves around `inferred_family_candidates`
- control-plane routing still chooses a `selected_family_id`
- planner handoff and planner response still ask for
  `recommended_family_id`
- host execution and fallback logic still treat family identity as the central
  build primitive
- UI fallback views still expose candidate family lists

## Remaining Coupling Hotspots

The main places still encoding positive-lane routing are:

- `crates/chatty_factory_core/src/ids.rs`
  `FamilyId` is still the canonical positive routing enum
- `crates/chatty_factory_core/src/contracts.rs`
  request, route, planner, fallback, execution, and proof contracts still
  persist family identity directly
- `crates/chatty_factory_core/src/foreman.rs`
  request normalization, plan derivation, scaffold naming, and planner override
  application are still family-first
- `crates/chatty_factory_core/src/runtime.rs`
  planner prompts and parser recovery still tell the model to choose
  `recommended_family_id`
- `crates/chatty_factory_control/src/lib.rs`
  the control graph still encodes a `ChooseFamily` step and `family_supported`
  edges
- `crates/chatty_factory_host/src/lib.rs`
  host execution, proof seeding, fallback specs, capability receipts, and
  emitter selection still center on family ids
- `crates/chatty_factory_ui/src/main.rs`
  fallback displays still surface family candidates to the operator

## Gutting Plan

The removal sequence needs to preserve safety while deleting positive-lane
authority from the host.

### Phase 1: Rename the Runtime Truth

Replace family-first routing with neutral substrate-first routing in core
contracts.

Do:

1. add neutral routing fields such as substrate kind, candidate substrates, and
   selected substrate
2. thread those fields through request, plan, route, and planner handoff
   contracts
3. keep `FamilyId` only as temporary compatibility glue where the host still
   needs it to emit an existing deterministic scaffold

Do not:

- let family ids remain the only routing truth in any new or edited contract
- introduce new starter abstractions under a different name

### Phase 2: Make Planning Substrate-First

Move `foreman` logic from positive-family ranking to negative-lane substrate
selection.

Do:

1. stop treating `rank_family_candidates(...)` as the primary planning result
2. infer neutral shape from request constraints first
3. use tool kind, requested surface, wrapper need, and hosting constraints as
   the core route signals
4. map substrate to an existing family only at the last compatibility boundary

Expected result:

- the plan says what kind of bounded build is safe to attempt
- it does not claim the host already knows the exact positive implementation
  family

### Phase 3: Remove Family Choice From the Control Plane

The control graph should route by negative execution shape, not by product
family.

Do:

1. rename `ChooseFamily` to a neutral routing step
2. replace `family_supported` edges with substrate-safe route edges
3. make route reasons speak in terms of surface, wrapper, helper, and
   constraint fit

Expected result:

- route receipts explain why a bounded lane is safe
- they stop reading like a starter picker

### Phase 4: Reframe Planner Handoff

The planner should no longer be asked to choose from host-owned positive
families.

Do:

1. change planner prompts from `candidate_families` and
   `recommended_family_id` to neutral substrate or bounded shape choices
2. keep family parsing only as a degraded compatibility fallback while the host
   still emits legacy scaffolds
3. remove planner language that implies the host owns the positive build
   catalog

Expected result:

- the model contributes implementation pattern direction
- the host keeps final authority over bounded execution and verification

### Phase 5: Collapse Host Family Authority

Once substrate routing is the truth, gut the remaining family-owned host logic.

Do:

1. move emitter selection behind substrate-safe execution adapters
2. stop persisting candidate family lists in fallback specs and receipts
3. stop using family identity as the center of proof seeds, route proofs, and
   fallback recommendations
4. narrow any remaining family references to legacy adapter internals only

This is the largest cut. Most of the remaining positive-lane architecture lives
inside `crates/chatty_factory_host/src/lib.rs`.

### Phase 6: Remove User-Facing Family Language

After the host no longer thinks in families, finish cleaning the operator
surface.

Do:

1. remove fallback displays that show family candidate lists
2. rewrite route, fallback, and review text to describe bounded build shape and
   safety constraints instead of positive family support
3. update old milestone sketches and inventory docs that still canonize family
   routing

## Guardrails During Removal

This pivot is not a relaxation of safety.

It is a narrowing of host responsibility so the host focuses on:

- execution boundaries
- receipts
- verification
- negative evidence

and stops pretending it must also be the source of positive build pattern truth.

The host should still be allowed to say:

- this request exceeds the bounded lane
- this project shape cannot be verified safely
- this scaffold path is unsupported
- this fallback is only a stub, not a truthful build

The host should stop saying:

- here are the eight positive families that define reality
- here is the correct starter identity for your request
- here is the host-owned catalog of positive implementation patterns

## Suggested Cut Order

If we continue the gutting in code, the lowest-risk order is:

1. core contracts and `foreman`
2. planner prompt and parser compatibility in `runtime`
3. control-plane rename and route-decision cleanup
4. host execution and fallback collapse
5. remaining UI and historical docs

## Execution Tracker

Use this as the actual removal order. Each phase assumes the previous one is
done first.

### Phase 0: Completed Prep

- [x] remove operator-facing starter controls from CLI
- [x] remove operator-facing starter controls from UI
- [x] remove starter recommendation and usage summaries
- [x] strip product docs that present ChattyFactory as an eight-family system
- [x] remove starter metadata from build freeze, plan, review, work order, and
  receipt artifacts

### Phase 1: Core Contract Pivot

Goal:
Make neutral substrate routing the new source of truth in core contracts while
leaving temporary family compatibility fields in place only where needed.

- [x] add neutral substrate identifiers and candidate substrate fields in
  `crates/chatty_factory_core/src/ids.rs` and
  `crates/chatty_factory_core/src/contracts.rs`
- [x] add neutral selected substrate fields to `RequestRecord`,
  `RequestPlan`, `RouteDecision`, and `PlannerHandoff`
- [x] add neutral planner recommendation fields to `PlannerResponse`
- [x] mark family-bearing routing fields as compatibility-only in comments and
  usage
- [x] make sure serialization still works for legacy host readers during the
  transition
 

Exit condition:
Core contracts can describe bounded routing without requiring family identity
as the primary truth.

### Phase 2: Foreman Pivot

Goal:
Make request normalization and planning substrate-first instead of
family-first.

- [x] replace family-first request normalization with neutral substrate
  inference in `crates/chatty_factory_core/src/foreman.rs`
- [x] stop treating `rank_family_candidates(...)` as the primary route result
- [x] update plan rationale and execution steps to describe bounded substrate
  selection instead of family ranking
- [x] update confidence and escalation logic so missing substrate candidates,
  not missing family candidates, drives review
- [x] reduce family mapping to a last-mile compatibility step for existing
  scaffold emitters only

Exit condition:
`normalize_request`, `derive_request_plan`, and `apply_planner_response` can
run without family-first reasoning.

### Phase 3: Planner Handoff And Runtime Parser

Goal:
Stop asking the planner to act like a host-owned family selector.

- [x] update `PlannerHandoff` payload contents to send neutral substrate
  choices instead of `candidate_families`
- [x] change planner prompt examples in
  `crates/chatty_factory_core/src/runtime.rs` away from
  `recommended_family_id`
- [x] add parser support for neutral substrate recommendations
- [x] keep legacy `recommended_family_id` parsing only as degraded
  compatibility fallback
- [x] update inferred-text fallback logic to infer substrate-first choices

Exit condition:
Planner interaction no longer depends on positive-lane family catalogs.

### Phase 4: Control Plane Cleanup

Goal:
Remove family choice as the named routing primitive in the control graph.

- [x] rename `ChooseFamily` to a neutral routing step in
  `crates/chatty_factory_control/src/lib.rs`
- [x] replace `family_supported` edges with substrate-safe route labels
- [x] rewrite route decision reasons in neutral safety language
- [x] keep any remaining family reference only inside compatibility glue paths

Exit condition:
Control receipts describe safe bounded routing, not starter-family selection.

### Phase 5: Host Routing Collapse

Goal:
Move `chatty_factory_host` away from family authority and into bounded adapter
authority.

- [ ] replace host assumptions that `plan.inferred_family_candidates` is the
  primary route truth
- [ ] stop writing candidate family lists into new fallback and review artifacts
- [ ] replace route proof and build-proof logic that centers family identity
- [ ] move emitter selection behind substrate-safe adapter resolution
- [ ] remove starter override remnants that still rewrite request and plan
  family candidates
- [ ] narrow any remaining family references to legacy scaffold adapter internals
- [ ] stop treating fallback as host-selected build-shape substitution

Exit condition:
The host can execute and verify bounded lanes without presenting family
selection as product truth.

### Phase 6: Fallback And Proof Reframe

Goal:
Make fallback honesty and proof generation evidence-driven gauntlet artifacts
instead of family-catalog or host-selection artifacts.

- [ ] rewrite fallback specs so they recommend bounded extension or substrate
  work from evidence, not host-selected artifact substitution
- [ ] remove family-centered candidate lists from clarification and fallback
  contracts where possible
- [ ] make fallback sequencing explicitly mechanical: classify, constrain,
  decompose, retry, triangulate, compare receipts, then escalate method/model
- [ ] reframe proof seed selection to target capability and bounded execution
  shape first
- [ ] update failure messaging so unsupported work is explained as a bounded
  safety gap, not missing family support

Exit condition:
Fallback artifacts stay truthful without rebuilding a positive-lane taxonomy or
pretending the host is allowed to choose a replacement positive artifact shape.

### Phase 7: UI And Historical Doc Cleanup

Goal:
Remove the last operator-visible traces of positive-lane framing.

- [ ] remove fallback family candidate displays from
  `crates/chatty_factory_ui/src/main.rs`
- [ ] rewrite UI route and review text to use substrate and safety language
- [ ] update docs that still canonize family routing such as
  `docs/CONTRACT_INVENTORY.md` and any
  bounded-review docs that still assume family candidates
- [ ] remove stale references to "supported families" from any remaining
  readmes or workflow notes

Exit condition:
The user-facing system no longer teaches a positive-lane family worldview.

### Phase 8: Compatibility Tombstone

Goal:
Delete the temporary family compatibility layer once the host no longer needs
it.

- [ ] remove transitional family routing fields from core contracts
- [ ] remove family fallback parsing from planner runtime
- [ ] remove compatibility mapping code from `foreman` and host route handling
- [ ] delete dead helper functions that only existed to preserve family-era
  behavior
- [ ] run a final wording pass to ensure "substrate" does not become a renamed
  starter catalog

Exit condition:
Positive-lane family architecture is fully gutted rather than merely renamed.
