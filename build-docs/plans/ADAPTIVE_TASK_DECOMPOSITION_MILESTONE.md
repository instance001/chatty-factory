# Adaptive Task Decomposition Milestone

## Purpose

This milestone defines the next layer after atomized execution:

- not only atomize the whole build or patch request
- but atomize an already-atomized task when evidence shows that the task is
  still too large or too entangled for the current model/runtime pair

The goal is to support smaller and mid-sized local models without:

- falling back into unsupported-by-host walls
- hardcoding every capability as a helper
- pre-decomposing everything into useless dust

This is the middle path:

- attempt the current task at its current granularity first
- let xray and verification decide whether that task class is still too broad
- store that lesson as a decomposition rule for next time

## Current Proven State

This milestone is no longer only theoretical.

The current factory has already proven one real build-side decomposition ladder:

1. broad `toolbar_ui_block`
2. decomposed `toolbar_label_sentence`
3. decomposed clause children:
   - `action_toolbar_label_clause_run_action`
   - `action_toolbar_label_clause_clear_action`
4. host composition of the final toolbar label sentence
5. host rendering of the final Rust `ui.group(...)` block

That matters because it proves the intended behavior:

- a failing broad task does not automatically become "unsupported"
- a smaller child task can earn its own further decomposition rule
- the host can keep owning syntax glue while the model supplies only the
  smallest remaining semantic fragments

## Core Decision

Task decomposition should be:

- adaptive
- evidence-driven
- task-type-specific
- reviewable

It should **not** be:

- universal
- infinite
- mandatory for every task from the outset

The factory should first try the current frozen task.

If the task fails in a way that strongly suggests:

- too many concerns at once
- too much syntax burden
- repeated missed literals or structure
- repeated broad-shape failure
- repeated review blocking for the same task class

then that task class should earn a decomposition rule.

## Intended Shape

The execution stack becomes:

1. User request
2. Interpretation freeze
3. Plan freeze
4. Task graph derivation
5. Attempt task at current granularity
6. Xray / review / verification classify outcome
7. If the task is too large, derive or apply decomposition
8. Replace parent task with child tasks
9. Retry through the same lifecycle

So the question is no longer:

- "is this supported?"

It becomes:

- "is this task small enough yet for a reliable attempt?"

## Why This Matters

The current local model usually knows:

- what feature parts are needed
- roughly how they fit together
- which broad method is plausible

But even after plan atomization, some task types are still too broad, for example:

- build a helper
- author a toolbar block
- wire bridge state and events
- write results-panel behavior
- add a multi-part status surface

Those should not automatically become:

- impossible
- unsupported
- blocked by host absence

They should become:

- decomposable task classes

## Decomposition Trigger Philosophy

The host should not eagerly split every task forever.

The host should attempt a task once at its current size and then ask:

- did the model/runtime pair actually cope?
- if not, was the failure because the task was still too large or too mixed?

If yes:

- add a decomposition rule

If no:

- keep the current task shape

This keeps the system from over-atomizing simple tasks while still helping
smaller models on harder ones.

## New Contract Layer

Suggested first contracts:

### `TaskDecompositionRule`

- `rule_id`
- `task_kind`
- `task_subtype`
- `trigger_failure_classes`
- `trigger_review_findings`
- `decomposition_strategy`
- `child_task_templates`
- `replacement_guidance`
- `active`
- `created_at`

### `TaskDecompositionReceipt`

- `receipt_id`
- `request_id`
- `parent_task_id`
- `rule_id`
- `trigger_evidence`
- `child_task_ids`
- `decision`
- `notes`
- `created_at`

## Task Subtype Examples

Examples of decomposition-worthy task subtypes:

- `helper_build`
- `toolbar_ui_block`
- `results_panel_block`
- `bridge_shared_room_state_wiring`
- `bridge_shared_room_events_wiring`
- `status_surface_wiring`
- `contract_file_pair_sync`

This is intentionally about task classes, not product lanes.

## Example Decomposition Shapes

### `toolbar_ui_block`

Potential child tasks:

1. section title and label
2. primary button
3. secondary button
4. local wiring behavior

### `helper_build`

Potential child tasks:

1. helper interface shape
2. helper state/data model
3. helper command surface
4. helper integration wiring

### `bridge_shared_room_state_wiring`

Potential child tasks:

1. contract file update
2. UI surface placeholder
3. state-holder insertion
4. event or refresh hook

## Trigger Classes

Adaptive decomposition should be considered when task receipts show patterns like:

- repeated `blocked_by_review` for the same task subtype
- repeated syntax failure
- repeated missing required literals
- repeated wrong structural shape
- repeated mixed prose/code output
- repeated multi-concern output that fails narrow review

Examples:

- a toolbar block keeps missing required buttons
- a helper build keeps returning imports, structs, and functions all at once
- a bridge task keeps trying to touch UI, state, and contract files in one step

## Relationship To The Negative Bookshelf

The negative bookshelf remains the arbiter of:

- what cannot possibly work
- what methods are stale or invalid
- what repeated bad patterns should be disallowed

Adaptive task decomposition adds a new kind of learnable guidance:

- not "never do this"
- but "this task type must be smaller before attempting"

So the bookshelf and decomposition layer work together:

- bookshelf says what is invalid
- decomposition says when a valid goal still needs a smaller execution shape

## Guardrail Against Infinite Atomization

The system should not decompose forever.

Suggested stopping principles:

- only decompose when failure evidence supports it
- only decompose task subtypes that recur
- only decompose one level deeper at a time
- keep each decomposition strategy reviewable
- prefer stable child templates over ad hoc explosion

If a child task is still too large, that can earn its own decomposition rule
later, but only from evidence.

## Initial Implementation Order

### Wave 1

- define `TaskDecompositionRule`
- define `TaskDecompositionReceipt`
- store rules in runtime
- no automatic decomposition yet

Status:
- exceeded in a lightweight host-owned form
- runtime decomposition receipts now exist
- decomposition recommendations are persisted from real model-authored task attempts

### Wave 2

- detect repeated failure classes on one subtype
- emit proposed decomposition receipts
- allow operator approval

Status:
- partially exceeded
- the host is already deriving decomposition from real generation-mode and
  review evidence on the toolbar proof path

### Wave 3

- automatically apply approved decomposition rules
- replace one parent task with reviewed child tasks
- persist the replacement receipt

Status:
- partially exceeded on the proved toolbar path
- earned decomposition now changes the next task graph automatically for:
  - `toolbar_ui_block`
  - `toolbar_label_sentence`

### Wave 4

- feed decomposition outcomes back into task governance and shelf summaries
- track which decomposition rules are actually helping smaller models succeed

## First Likely Proof Target

The strongest first proof target is:

- `toolbar_ui_block`

Why:

- it is small
- it already has real repeated review evidence
- it is easy to tell whether decomposition helped
- it does not require inventing a whole helper/runtime ecosystem first

## Intended End State

The end state should feel like:

- the host focuses the model on one task
- if that task is still too large, the system learns to split it
- future attempts start at the better granularity

That keeps the system:

- open-ended
- model-aware
- evidence-driven
- and protected from both helper sprawl and infinite decomposition mush
