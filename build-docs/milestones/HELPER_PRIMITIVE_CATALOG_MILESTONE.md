# Helper Primitive Catalog Milestone

## Why This Is Next

ChattyFactory now has real bounded composition:
- composed builds
- composed patches
- GGUF review of bounded plans
- host correction and execution receipts

That is a major architectural win.

But helper-backed behavior is still too family-shaped.
Right now, the strongest proof lives around `chattycog_webview_module` and its helper-backed patch lanes.

The next step is to make helper behavior a reusable host primitive layer rather than a set of one-off family-specific conveniences.

This is the next big move toward the original design intent:
- the host owns machinery
- the GGUF supervises bounded plans
- capability growth becomes less tied to one family or substrate

## Design Goal

Define a first-class helper primitive catalog that the host can compose across more than one family.

The host should be able to reason about helpers in terms like:
- input lane
- filter
- summary emitter
- status reporter
- preview source
- runtime watcher

instead of only in terms like:
- webview helper patch
- helper summary chip
- helper lane-scoped filter notice

Families should still matter, but increasingly as adapters that render or expose helper primitives, not as the only place helper behavior exists.

## What Counts As A Helper Primitive

A helper primitive should be:
- bounded
- host-supervised
- receipt-producing
- composable
- family-agnostic in intent, even if family-specific in rendering

Examples:
- `inbox_lane`
- `lane_filter`
- `summary_snapshot`
- `status_snapshot`
- `processed_file_index`
- `preview_payload`
- `auto_refresh_trigger`

These are not UI widgets.
They are machinery units the host can supervise and expose to different family surfaces.

## Core Architectural Shift

Today the factory can say:
- “apply `helper_summary_status_chip`”

The next version should also be able to say:
- “compose `summary_snapshot + status_snapshot + lane_filter`”

and then let the chosen family decide how those primitives show up:
- a webview surface
- a static dashboard surface
- a CLI/report surface

This is the beginning of true capability abstraction.

## Scope Of This Milestone

### 1. Define Helper Primitive Contracts

Add a host-owned helper primitive contract layer for:
- primitive id
- primitive kind
- launch/runtime policy
- required inputs
- provided outputs
- dependency semantics
- acceptance hooks

Minimum semantics:
- `required`
- `optional_companion`
- `exclusive_group`
- `runtime_requires`

### 2. Build A Helper Primitive Catalog

Start by extracting the current helper-backed ChattyCog machinery into a catalog of reusable primitives.

Likely first entries:
- `inbox_lane_primary`
- `inbox_lane_secondary`
- `lane_filter_txt_only`
- `summary_snapshot`
- `status_snapshot`
- `processed_file_index`

### 3. Bind Helper Primitives To Families

Families should declare how they consume or expose primitives.

Examples:
- `chattycog_webview_module`
  - render helper primitives as UI surfaces
- `static_web_dashboard`
  - render helper primitives as static status/reporting panels
- later:
  - `python_cli_tool`
  - `rust_cli_tool`
  can expose helper primitives as report artifacts instead of UI

### 4. Add Helper Composition Receipts

The host should persist a clean helper composition ledger:
- candidate helper primitives
- reviewed helper primitives
- final helper primitive bundle
- runtime launch/stop/status results
- acceptance outcomes

This should parallel the new composition review receipts, not hide helper composition inside general route notes.

### 5. Prove Cross-Family Reuse

The key proof is not “add another ChattyCog helper patch.”

The key proof is:
- one helper primitive bundle
- reused across at least two families

Best first pair:
- `chattycog_webview_module`
- `static_web_dashboard`

That keeps the second proof close enough to current machinery while still proving the abstraction is real.

## Recommended First Proof

Build and verify a reusable helper bundle such as:
- `inbox_lane_primary`
- `inbox_lane_secondary`
- `lane_filter_txt_only`
- `summary_snapshot`
- `status_snapshot`

Then expose that bundle in:
1. a ChattyCog webview module
2. a static web dashboard

Success means both families are consuming the same helper machinery concepts, not just duplicating similar ad hoc behavior.

## Implementation Order

1. Define helper primitive contract structs.
2. Extract the current `local_inbox_helper` behaviors into primitive ids and primitive metadata.
3. Add helper primitive candidate/review/final receipts.
4. Teach one second family to consume the same helper primitive bundle.
5. Prove one cross-family composed request using helper primitives.

## What Not To Do

- Do not just add more ChattyCog-only helper UI lanes.
- Do not blur helper primitives and UI patch primitives into one registry.
- Do not let the GGUF author helper machinery directly.
- Do not skip receipts; helper composition must stay inspectable.

## Success Criteria

This milestone is successful when:
- helper machinery is described as reusable primitives
- the host can compose those primitives explicitly
- the GGUF can review helper primitive bundles in bounded form
- at least two families can consume the same helper primitive bundle
- receipts clearly show:
  - candidate helper primitives
  - reviewed helper primitives
  - final helper primitive bundle
  - runtime outcomes

## Why This Matters

This is the bridge from:
- “bounded composition exists”

to:
- “bounded composition is becoming substrate-agnostic machinery”

That is exactly the direction the original conveyor-belt design wants:
- host owns the grunt and churn
- GGUF supervises bounded work
- capabilities become less template-shaped and more machinery-shaped
