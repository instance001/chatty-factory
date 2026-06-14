# Automatic Decomposition Inference Milestone

## Purpose

This milestone defines the next architectural jump after the first manual
adaptive-decomposition proofs.

The problem is now clear:

- the current starter-plus-plan-plus-task-plus-decomposition shape is correct
- but too much of the decomposition learning is still being taught into the
  factory by hand

That is useful for proof, but it is not the intended end state.

The intended end state is:

- the factory attempts bounded work
- xray and verification explain why it failed
- the factory infers when the task was still too broad
- the factory maps the failure to a reusable grammar
- the factory proposes or promotes an inferred mapping
- the next run becomes smaller automatically

This is how the factory stops depending on us to manually author every next
task split.

## Problem Statement

Right now, we risk recreating old positive-lane behavior in decomposition form:

- one bespoke decomposition path for toolbar
- one bespoke decomposition path for metric card
- one bespoke decomposition path for results panel
- and so on

That would still be lane sprawl, only at the task layer instead of the family
layer.

The next system we need is:

- generic decomposition inference
- generic decomposition grammars
- host-owned composition/rendering where fragile syntax glue is required

The host should not keep becoming the author of each next task class.

The host should become:

- a decomposition synthesizer
- a composition synthesizer
- a reviewer/verifier

The model should keep owning:

- the semantic work inside those bounded task shapes

## Design Intent

The design intent for this wave is:

1. user request in
2. interpretation freeze
3. plan freeze
4. task freeze
5. attempt current task granularity
6. classify failure
7. if failure implies "task still too broad":
   - infer a decomposition rule
8. persist that inference
9. allow review/approval/promotion
10. apply it automatically on later runs

That means the main source of improvement should become:

- the factory's own failure evidence

not:

- us continuing to manually carve up feature classes one by one

## Core Principles

### 1. Do not pre-decompose everything

We do not want infinite atomization mush.

The system should only split tasks further when evidence says the current
granularity is still too large for the current model/runtime pair.

### 2. Do not treat host-helper absence as unsupported

The host should not say:

- "unsupported because no helper exists"

The host should say:

- "attempt the current task"
- "classify why it failed"
- "infer whether decomposition is appropriate"

### 3. Learn from task shape, not product lane

The important learning unit is not:

- Chatty-Cog toolbar

It is:

- semantic object task with repeated omitted fields
- sentence task with repeated reasoning spill
- multi-concern block task
- mixed UI-plus-behavior task

Those are reusable task shapes.

### 4. Let the host own grammar, not feature catalogs

The host should know decomposition grammars like:

- object field split
- sentence to clause split
- list/panel split
- semantic-part composition

It should not need one custom hardcoded authoring path per final feature.

## Phase 1: Decomposition Proposal Contracts

Add explicit contracts for:

- `TaskDecompositionProposal`
- `TaskDecompositionInferenceReceipt`

These are separate from the current end-state receipts because they capture the
factory's concrete mapping from failure evidence to reusable grammar:

- which constraint principles fired
- what task shape was recognized
- what generic split grammar applies
- what child tasks are proposed
- what confidence the inference had

Required fields:

- source task id
- task shape
- task subtype
- constraint principles
- failure class
- generation mode
- review findings
- inferred decomposition pattern
- proposed child tasks
- proposed host composition task
- confidence / promotion posture

## Phase 2: Generic Decomposition Patterns

Teach the host a small generic decomposition grammar library.

Initial patterns:

1. `semantic_object_field_split`
- trigger:
  - JSON/object task
  - repeated omitted fields
  - reasoning fallback / empty content
- child tasks:
  - one per required field
  - one host composition/render task

2. `sentence_clause_split`
- trigger:
  - sentence-level semantic task
  - repeated failure on whole-sentence ask
- child tasks:
  - clause A
  - clause B
  - host composition task

3. `multi_surface_panel_split`
- trigger:
  - broad UI/panel task with multiple visible concerns
- child tasks:
  - heading/title
  - summary/value
  - empty-state or context text
  - host render task

4. `ui_behavior_separation`
- trigger:
  - mixed UI copy plus behavior task
- child tasks:
  - semantic UI copy
  - behavior intent or control descriptors
  - host render / bounded mechanical hook

These are grammars, not feature lanes.

## Phase 3: Inference Heuristics

Teach the factory to infer decomposition when receipts show patterns like:

- empty `content`
- reasoning spillover
- repeated omitted required fields
- repeated failure to satisfy exact semantic keys
- repeated placeholder-grade output
- broad task with mixed concerns

This should produce:

- an inference receipt
- a proposed grammar application

instead of a plain blocked review result.

## Phase 4: Promotion And Governance

The factory should not silently hard-promote every inferred split forever.

We need:

- decomposition proposal receipts
- approved decomposition rule registry
- usage counts
- success counts
- dead-weight detection

Suggested artifacts:

- `task_decomposition_proposals/`
- `approved_task_decomposition_rules.json`
- `task_decomposition_usage_summary.json`

Promotion modes:

- operator approved
- confidence threshold approved
- repeated-evidence approved

## Phase 5: Automatic Application

Once a rule is approved or promoted, the task graph should apply it
mechanically on the next run.

That means:

- broad parent task is replaced by child tasks
- child tasks inherit boundaries and verification steps
- host composition task is added when required

This keeps the system self-improving without pretending every future split
needs manual intervention.

## First Generic Proof Targets

The first target classes should be:

1. `toolbar_ui_block`
- already manually proven
- use as the reference case for object and sentence decomposition

2. `metric_card_block`
- currently proven as a decomposition recommendation
- use as the first generic semantic-object field-split proof

3. `results_panel_block`
- likely next multi-surface proof

These should be re-expressed as:

- applications of generic patterns

not:

- permanent bespoke feature handlers

## Success Criteria

We should consider this milestone working when:

1. a failed semantic-object task can infer field-level decomposition
2. the next run applies that split automatically
3. a decomposition rule is tracked as a governed artifact
4. at least two task classes use the same generic pattern
5. the host no longer needs a bespoke decomposition path for each new feature

## Non-Goals

This milestone is not trying to:

- remove all host-owned helpers
- eliminate manual oversight entirely
- achieve universal build success

It is trying to ensure:

- the factory starts learning how to shrink work from its own failures
- instead of depending on us to keep doing that reasoning for it by hand

## Practical Next Slice

The first implementation wave after this milestone should be:

1. add decomposition proposal/inference contracts
2. add one generic pattern:
   - `semantic_object_field_split`
3. route current `metric_card_block` failure into that generic pattern
4. persist the inferred children:
   - title
   - value
   - context
   - host render
5. apply it on the next run automatically

That would be the first real proof that the factory is beginning to improve
its own decomposition strategy instead of relying on us to hand-author each
next rescue path.
