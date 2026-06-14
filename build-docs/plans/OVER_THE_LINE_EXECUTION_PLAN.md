# Over The Line Execution Plan

## Purpose

This plan defines the shortest realistic path from:

- first real architecture proof

to:

- generalized factory behavior that matches the design intent closely enough to
  treat the core shape as established

The goal is not "ship universal build anything" in one jump.

The goal is:

- make the proven starter-plus-plan-plus-task-plus-decomposition pattern
  general enough that future success comes from widening task-class coverage,
  not from returning to lane sprawl

## Current Honest State

The factory has already proven the following stack:

1. starter substrate selection
2. interpreted request freeze
3. build plan artifact
4. build plan self-review
5. build constraint review
6. build execution work order
7. frozen task graph
8. host sync / host mechanical / model-authored task lifecycle
9. task execution and verification receipts
10. adaptive decomposition from failure evidence
11. automatic application of earned decomposition rules on the next run

Most importantly, one build-side semantic ladder is now real:

1. broad `toolbar_ui_block`
2. decomposed `toolbar_label_sentence`
3. decomposed clause children
4. host composition of the final sentence
5. host rendering of the final Rust block

That means the architecture is no longer speculative.

The remaining gap is:

- coverage
- governance maturity
- patch-side symmetry
- better runtime-mode handling

## Definition Of "Over The Line"

For this project, "over the line" should mean:

1. starter families are clearly substrates, not the capability boundary
2. the host behaves mainly as scaffold/lens/governor, not feature arbiter
3. model-authored work is routinely attempted as bounded tasks
4. adaptive decomposition is reusable across multiple task classes
5. build-side and patch-side both support the same task/decomposition logic
6. failure produces reusable bookshelf or decomposition lessons instead of
   collapsing into "unsupported"

It does **not** require:

- 100 percent build success
- no fallbacks
- no local model limitations

## Core Remaining Gaps

### 1. Build-side pattern is still too concentrated

Right now the best proof is mostly on one task family:

- Rust native action toolbar generation

We need the same pattern to apply to at least a few more meaningful classes.

### 2. Patch-side microtasking is behind build-side microtasking

Patching already has strong diagnosis and governance, but not the same mature
model-authored microtask ladder.

### 3. Decomposition governance is still lightweight

We have decomposition receipts and automatic application for real paths, but
not yet:

- curated decomposition-rule inventory
- match counts
- usefulness reporting
- deactivate/archive/restore lifecycle like the negative shelf

### 4. Runtime-mode adaptation is still narrow

We now detect:

- empty `content`
- reasoning spillover

but runtime-mode response to that is still mostly task-local.

### 5. Build-task coverage is still thin

The current host/model split is strongest on:

- starter sync
- one host-mechanical insertion
- one proven decomposed UI feature path

We need a few more generalized feature classes to call the architecture truly
established.

## Phase 1: Finish Build-Side Proof Expansion

Objective:

- prove the same task ladder on 2 to 3 more task classes

Target classes:

- `results_panel_block`
- `metric_card`
- one bridge-oriented state/event wiring class

Success criteria:

- each class has at least one frozen model-authored task
- at least one class earns decomposition
- at least one class succeeds after decomposition
- host composition owns the fragile syntax glue where appropriate

Why first:

- this spreads the proof beyond one UI-toolbar path

## Phase 2: Bring Patching Onto The Same Task Model

Objective:

- give patching the same task-graph and decomposition posture as builds

Scope:

- derive patch task lists from reviewed patch work
- persist patch task execution and verification receipts
- allow bounded model-authored patch microtasks
- allow adaptive decomposition on patch task subtypes

Suggested first patch task classes:

- helper label/status insertion
- small dashboard panel addition
- bounded bridge surface follow-up

Success criteria:

- one patch-side model-authored microtask is proven
- one patch-side decomposition ladder is proven

Why second:

- it closes the biggest architectural asymmetry left in the factory

## Phase 3: Promote Decomposition To A Governed Subsystem

Objective:

- treat decomposition the way the negative shelf is already treated

Add:

- decomposition rule summary artifact
- decomposition match/usefulness counts
- UI surface for recent decomposition events
- operator-facing review/curation state

Suggested artifacts:

- `task_decomposition_rule_registry.json`
- `task_decomposition_usage_summary.json`

Success criteria:

- we can answer:
  - which task classes keep decomposing
  - which decompositions actually improve success
  - which decompositions are dead weight

Why third:

- once decomposition spreads, it needs lifecycle governance rather than just
  raw receipts on disk

## Phase 4: Runtime-Mode Adaptation

Objective:

- respond more deliberately when the local model fails by generation mode rather
  than by task semantics

Examples:

- empty `content` with reasoning spillover
- repeated JSON-shape miss
- repeated truncation on one task subtype

Response options:

- immediate decomposition recommendation
- skip wasted retry
- switch prompt style
- switch model profile if configured

Success criteria:

- generation-mode failures produce distinct decisions, not generic review noise

Why fourth:

- this improves the lens without changing the architecture

## Phase 5: Task-Class Starter Pack

Objective:

- define a small, reusable first-class set of task families that prove the
  engine is general enough

Suggested minimum set:

- one UI block family
- one helper family
- one bridge/state family
- one results/reporting family
- one patch-side family

This is **not** a return to positive lanes.

It is a proof pack for the task engine.

Success criteria:

- each class has:
  - task derivation
  - receipts
  - at least one model-authored path
  - decomposition support where needed

## Phase 6: Declare The Core Shape Stable

We should consider the design intent substantially achieved when:

- multiple task classes use the starter/plan/task/decomposition loop
- build and patch both use the same core logic
- runtime-mode failures produce specific routing/decomposition decisions
- the host is clearly a scaffold and governor, not a narrow feature arbiter
- the old positive-lane catalog is no longer the practical growth path

At that point, the work changes from:

- inventing the architecture

to:

- widening task-class coverage
- improving model/runtime quality
- growing bookshelf and decomposition governance

## Concrete Next Moves

In strict order:

1. prove one more build-side task class beyond toolbar
2. derive patch task lists and receipts
3. prove one patch-side model-authored microtask
4. add decomposition governance summary
5. add runtime-mode-specific adaptation rules

## What We Should Avoid

Do not:

- solve new feature asks by defaulting back to positive family growth
- add large bespoke host helpers for every new capability shape
- let "not yet proven at this task size" collapse into "unsupported"
- decompose everything forever without evidence

## Short Summary

We are close because the shape is now correct.

To get over the line, we do **not** need a brand-new architecture.

We need to:

- spread the proved pattern across more task classes
- bring patching into the same task/decomposition loop
- govern decomposition like a real subsystem
- and keep teaching the host to focus the model, not replace it
