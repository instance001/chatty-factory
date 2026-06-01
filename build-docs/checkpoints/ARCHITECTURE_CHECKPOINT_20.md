# Architecture Checkpoint 20

The patchability model has moved from per-lane and per-surgery intelligence to project-level governance.

## What Changed

ChattyFactory now has a host-owned project patch readiness layer.

That includes:

- per-project receipts under `runtime/project_patch_readiness_receipts/`
- a bulk refresh status artifact at `runtime/project_patch_readiness_refresh_status.json`
- browser badges for:
  - `patch-ready`
  - `patch-stable`
  - `patch-baselined`
  - `patch-improved`
  - `patch-drifted`
  - `patch-regressed`
- browser health counts for:
  - regressed projects
  - improved projects
  - blocked projects
- browser filters for:
  - blocked only
  - regressed only
  - improved only
- selected-project patchability detail with:
  - baseline note
  - blocked lane reasons
  - freshness line
  - stale warning
  - direct refresh action

## Why This Matters

Before this wave, the system could say:

- this lane is blocked
- this patch was skipped safely
- this surgery stayed within contract

Those are valuable, but they are still local answers.

Now the system can also say:

- this whole project has become riskier to patch
- this project's patchability improved
- this project is stable against its last patchability baseline

That is a different level of operational understanding.

It turns patch safety from:

- a moment-of-execution concern

into:

- an ongoing property of the project

## Architectural Read

This is the first real project-level governance loop for patchability.

The stack now looks like:

1. lane contract
2. diagnosis
3. intent freeze
4. postcheck
5. project patchability governance

That is a much stronger reliability story than "try the patch and hope the handler is still shaped correctly."

## Recommended Next Direction

The strongest next move is probably not more browser polish.

The two healthy next directions are:

1. deepen project patchability governance

Examples:

- stronger project-level regression classification
- project patchability history and diffs
- refresh freshness parity with stronger warnings or auto-refresh policy

2. widen lane contract coverage

Examples:

- more lanes upgraded from broad or legacy-sensitive shapes to narrower declared contracts
- more family/tool combinations covered by structural anchors and surface-group expectations

If choosing between those two, the better next investment is probably lane-contract coverage.

The project-level layer is now useful and legible.
The bigger remaining reliability gain is to make more patch lanes worthy of being measured by it.
