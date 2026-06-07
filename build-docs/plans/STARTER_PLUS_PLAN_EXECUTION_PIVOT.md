# Starter Plus Plan Execution Pivot

## Purpose

This plan defines the architectural pivot away from treating deterministic
families as the main limiter of what ChattyFactory can build.

The intended limiter should be:

- what the current LLM can understand
- what the current LLM can reason into a build plan
- what the host can execute safely
- what the negative bookshelf can rule out or redirect

The intent is not to delete all deterministic structure.

The intent is to change what deterministic structure means.

## Core Decision

Deterministic families should become:

- stable starter substrates
- host-owned contract and acceptance baselines
- ecosystem compatibility layers

They should stop acting like:

- the total list of product capabilities
- the only shapes the factory is allowed to create

Capability growth should primarily come from:

1. LLM request interpretation
2. LLM feature/build planning
3. host-owned execution helpers
4. negative constraint review
5. verification and repair learning

## New Operating Model

The target operating split is:

1. User
   - requests a tool or follow-up capability in plain language

2. Starter selection
   - operator chooses a starter mechanically when needed
   - or normal routing recommends one

3. LLM
   - freezes intent
   - proposes a build plan against the selected starter
   - sequences feature slices and file changes

4. Host
   - owns scaffold emission
   - owns syntax-sensitive file operations
   - owns contract file updates
   - owns acceptance and execution verification

5. Negative bookshelf
   - blocks known-bad implementation methods
   - narrows the search space
   - turns failures into reusable constraints

This allows the current model’s code knowledge to matter, while keeping the
fragile mechanical work inside deterministic host logic.

## What Stays Deterministic

The deterministic layer should still own:

- starter family selection
- emitted starter file sets
- ecosystem plug files and module specs
- contract inventory and acceptance plans
- patch-plan review
- constraint review
- verification receipts
- governance receipts

This is still a governed system.

It just stops pretending the family catalog is the same thing as the total
capability catalog.

## What Shrinks

The following should shrink over time:

- positive capability lanes as the main build-growth path
- family proliferation for every product idea
- pressure to add a new family whenever a build fails

The following should grow instead:

- starter quality
- build-plan quality
- host execution helpers
- verification taxonomy
- negative bookshelf coverage

## Starter Catalog Posture

The long-term starter catalog should stay intentionally small.

Current likely primary starters:

- `rust_cli_tool`
- `static_web_dashboard` if still useful as a fast browser substrate
- `chattycog_native_window_module`
- `chattyedu_native_window_module`
- `chattycog_chattyedu_native_window_module`

Everything else should be evaluated as either:

- transitional legacy starter
- regression fixture
- removable catalog weight

## New Build Contract Layer

The missing middle layer is a typed build-plan contract.

Suggested first artifact:

### `BuildPlanIntent`

- `plan_id`
- `request_id`
- `starter_family_id`
- `starter_tool_kind`
- `interpreted_goal`
- `feature_slices`
- `target_files`
- `planned_file_operations`
- `acceptance_goals`
- `risk_notes`
- `recommended_constraints`
- `created_at`

### `BuildFeatureSlice`

- `slice_id`
- `summary`
- `why_it_exists`
- `files_to_create`
- `files_to_update`
- `expected_symbols`
- `acceptance_markers`
- `dependencies`

### `PlannedFileOperation`

- `operation_id`
- `operation_kind`
- `path`
- `content_source`
- `target_anchor`
- `ownership_boundary`
- `syntax_sensitive`

This gives the host something typed to review and execute instead of jumping
straight from request interpretation into family-bound scaffolding.

## New Host Role

The host should consume a build plan through bounded operations like:

- create file from template
- create file from model-produced content
- update manifest/spec
- add import/use/module line
- insert block at reviewed anchor
- append helper function
- wire UI panel into reviewed container
- update acceptance plan

The host should reject any operation that:

- crosses ownership boundaries
- violates known constraints
- cannot identify safe anchors
- would leave verification-critical files out of sync

## Relationship To The Negative Bookshelf

The negative bookshelf becomes more important after this pivot, not less.

Why:

- the model gains more room to propose
- so the host must get better at pruning invalid methods

The bookshelf should review:

- build plans before execution
- execution failures after verification
- repeated repair failures

The bookshelf should answer:

- what method should not be attempted here?
- what structural move is stale?
- what ecosystem contract would this violate?
- what syntax or ownership shortcut tends to fail?

## Verification Loop

The execution loop should become:

1. choose starter
2. derive build plan
3. self-review build plan
4. constraint-review build plan
5. execute bounded file operations
6. verify compile / run / contract / acceptance
7. classify failure if verification fails
8. propose new negative constraint if warranted

This is the path away from:

- “we can only build what we already hard-coded”

and toward:

- “we can build what the current model can plan, as long as the plan survives
  host execution and the negative funnel”

## Migration Order

### Wave 1: Reframe Families

- mark current positive families as starters or legacy starters
- stop using family growth as the default answer to new capability requests
- keep only a small canonical starter set as forward posture

### Wave 2: Add Build Plan Contract

- introduce typed build-plan artifacts
- persist them under `runtime/`
- expose them in CLI/UI receipts

### Wave 3: Add Host Build Execution Helpers

- implement bounded file operations
- thread them through starter-aware execution
- keep acceptance-plan sync host-owned

### Wave 4: Add Build Plan Review

- self-review build plans the same way patches gained self-review
- add constraint review before execution

### Wave 5: Verification-Driven Learning

- reuse build verification receipts
- generate proposed constraints from bad plans and bad outputs
- approve into the shelf deliberately

## Success Criteria

This pivot is working when:

- the number of starter families remains small
- new product capability stops requiring new family definitions by default
- build failures more often become:
  - better constraints
  - better build-plan review
  - better host execution helpers
- the current model can express more capability than the old family catalog
  without the system devolving into unsafe free-form codegen

## Honest Boundary

This pivot does not mean:

- unlimited arbitrary codegen
- no deterministic guardrails
- no verification

It means:

- deterministic starters
- model-driven plans
- host-driven execution
- constraint-driven pruning
- verification-driven improvement

That is the practical route toward a factory whose main limit is the current
pilot model’s knowledge and reasoning quality, not the size of a positive lane
catalog.
