# Architecture Checkpoint 22

## What Changed

The remaining legacy-sensitive ChattyCog helper-summary lanes are now explicitly curated as historical variants instead of looking like unresolved reliability debt.

This wave did not try to modernize them blindly.

Instead, it made their modern replacement path first-class:

- `helper_summary_badges` is explicitly superseded by:
  - `helper_summary_count_delta`
  - `helper_summary_lane_count_chip`
  - `helper_summary_types_chip`
- `helper_status_chip` is explicitly superseded by:
  - `helper_summary_status_chip`

That supersession signal now lives in `PatchLaneStatus` and flows through:

- patch-lane readiness reasoning
- CLI patch-lane reporting
- selected-project patch lane lists
- last-result patch lane lists
- patch X-ray summaries

## Why This Matters

This is a different kind of reliability improvement than the Rust lane modernization.

For the Rust CLI lanes, the right move was:

- rewrite the stale handler shape
- keep the capability
- restore honest narrow-contract status

For these two ChattyCog helper-summary lanes, the healthier move was:

- keep the old lane blocked
- explain that it is historical
- point directly at the modern replacement surface

That means the system is no longer treating every legacy-sensitive lane as “waiting to be fixed.”

Some lanes are now intentionally retired shapes with:

- safe preflight blocking
- named replacement lanes
- operator-visible migration guidance

## Architectural Read

The patch catalog is becoming curated, not just guarded.

That is an important threshold:

- diagnosis/freeze/postcheck explain what is unsafe
- contract maturity explains how trustworthy a lane is
- supersession explains whether a blocked lane should be revived at all

Together, that gives the operator a better answer than “this failed”:

- this lane is old
- this project evolved past it
- here is the modern patch path instead

## Best Next Move

The next strongest move is not another blanket checkpoint.

It is to keep applying this same selective standard:

- modernize legacy lanes that still deserve to live
- explicitly supersede legacy lanes that no longer should

The best likely candidates are any remaining legacy-sensitive lanes outside this ChattyCog cluster that still represent valuable user-facing capabilities rather than obsolete insertion shapes.
