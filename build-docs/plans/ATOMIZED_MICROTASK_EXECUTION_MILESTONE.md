# Atomized Microtask Execution Milestone

## Purpose

This milestone defines the architectural pivot away from treating:

- deterministic starter families
- host-authored helper coverage

as the primary limiter of what ChattyFactory can build or patch.

The intended limiter should instead be:

- what the current LLM can understand
- what the current LLM can reason into a frozen plan
- what can be split into small reviewed tasks
- what survives host review, negative constraints, and verification

This is the path away from:

- unsupported-by-host hell
- positive-lane catalog growth
- helper proliferation as the main capability path

and toward:

- frozen intent
- frozen plan
- atomized execution
- per-task review and verification

## Core Decision

Capability should not scale primarily by adding:

- more deterministic build families
- more positive lanes
- more bespoke host helpers for every feature shape

Capability should scale by turning one large request into:

1. frozen interpretation
2. frozen plan
3. small execution tasks
4. host-mechanical tasks where practical
5. model-authored tasks where necessary
6. per-task xray, review, and verification

The host remains the governor.

The model becomes the author of bounded microtasks instead of the author of one
giant unsupported monolith.

## Intended End State

The target build/patch shape becomes:

1. User request in
2. Interpretation freeze
3. Plan freeze
4. Task graph derivation
5. Task-by-task execution
6. Task verification after each step
7. Continue, retry, or block based on receipts

This means:

- the LLM does not need to build the whole tool in one shot
- the host does not need a helper for every capability class
- unsupported work becomes smaller authoring problems, not whole-product dead ends

## Task Contract Layer

The missing middle layer after `BuildExecutionWorkOrder` is a frozen task
contract.

Suggested first contract:

### `PlanTask`

- `task_id`
- `request_id`
- `source_build_plan_id`
- `source_work_order_id`
- `task_kind`
- `task_title`
- `task_summary`
- `dependencies`
- `target_files`
- `allowed_boundaries`
- `expected_symbols`
- `expected_markers`
- `verification_steps`
- `replacement_guidance`
- `created_at`

### `task_kind`

Initial task kinds:

- `host_mechanical`
- `model_authored`
- `host_verification`
- `host_sync`

The point is not to freeze these forever.

The point is to force the system to say:

- what exactly is this task?
- who should do it?
- what files can it touch?
- how do we know it passed?

## Host vs Model Split

### Host-owned tasks

The host should own work that scales well as deterministic infrastructure:

- create starter files
- update manifests/specs/contracts
- sync acceptance artifacts
- find anchors
- apply bounded diffs
- run linters/formatters/checkers
- run compile/smoke/acceptance commands
- persist receipts

### Model-authored tasks

The model should own work that does not scale as a helper catalog:

- write a helper
- write a function body
- write a UI panel
- wire a data flow
- add new internal module logic
- draft bridge behavior beyond current host helpers

But always as:

- a single small task
- with frozen scope
- with reviewed target files
- with explicit verification after execution

## Why This Is Better

The current model usually knows:

- what a requested tool should roughly contain
- what parts it needs
- what order those parts belong in
- what code shapes are plausible

Where it struggles:

- giant monolithic generation
- syntax drift across a whole tool
- safe insertion into existing files
- keeping all contracts in sync by itself

Atomized microtasks exploit the model’s strengths without asking it to carry the
whole build at once.

## Relationship To The Negative Bookshelf

The negative bookshelf becomes more important here.

It should not only review:

- whole build plans
- whole patch plans

It should also review:

- individual plan tasks
- individual task methods
- repeated failure patterns at task level

The bookshelf should answer:

- do not author this helper in this stale way
- do not insert into this file at this anchor
- do not duplicate this bridge contract update
- do not change this ecosystem plug file without its partner contract file

This is how the system avoids turning microtasks into chaos.

## Task Lifecycle

Every task should move through a simple lifecycle:

1. derived
2. frozen
3. reviewed
4. executed
5. verified
6. accepted / retryable / blocked

Suggested receipts:

### `TaskExecutionReceipt`

- task id
- executor type
- touched files
- produced diff or artifact summary
- status

### `TaskReviewReceipt`

- task id
- dropped files
- narrowed boundaries
- blocked reasons
- approved method

### `TaskVerificationReceipt`

- task id
- verification commands/checks
- pass/fail
- failure classification
- next step

## First Implementation Slice

The safest first slice is not “let the model rewrite the world.”

It is:

1. derive task list from reviewed build work order
2. emit host-owned task receipts
3. mark task kind as:
   - `host_sync`
   - `host_mechanical`
   - `model_authored`
4. execute only the already-safe host tasks first
5. leave model-authored tasks frozen but not yet executed

That gives us proof of the architecture before we let model-authored tasks write
code.

## First Model-Authored Task Target

The first true model-authored task should be tiny.

Good candidates:

- write a small helper module
- add one status panel block
- add one action toolbar block
- add one bridge-related helper function

Bad candidates:

- build the whole app
- rewrite `main.rs` wholesale
- invent the entire project from scratch in one task

## Verification Strategy

Each task should verify at the smallest useful level:

- syntax / parse
- formatter
- lint
- compile
- smoke
- contract sync

Not every task needs every check.

But every task needs:

- some explicit verification step
- some explicit failure receipt

## Migration Order

### Wave 1: Freeze the Concept

- keep starters as substrates
- keep work orders as reviewed operation lists
- stop expanding host helpers as the primary scaling strategy

### Wave 2: Add `PlanTask`

- derive frozen task lists from reviewed build work orders
- persist task artifacts under `runtime/`

### Wave 3: Add Host Task Execution

- execute only host-safe task kinds first
- prove per-task receipts and verification

### Wave 4: Add Model-Authored Task Execution

- execute one tiny model-authored task at a time
- freeze scope, target files, and verification before generation

### Wave 5: Add Task-Level Learning

- task failure classification
- proposed negative constraints from repeated bad task methods
- approval into the shelf

## Success Condition

This milestone succeeds when ChattyFactory can honestly say:

- we are not limited mainly by the size of the positive family catalog
- we are not limited mainly by which bespoke helpers the host already has
- we can freeze a request into a plan
- freeze the plan into tiny tasks
- let the host do the mechanical tasks
- let the model do bounded authoring tasks
- and verify each task before moving on

That is the path from:

- “supported only”

to:

- “buildable when the current model can reason it out, one checked step at a time”
