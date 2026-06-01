# Architecture Checkpoint 25

## What Changed

Project patchability governance now classifies blocked posture more precisely.

The system no longer treats every blocked lane as the same kind of project risk.

It now distinguishes between:

- risky blockers
  - lanes that are blocked because the project shape is unsafe or mismatched for surgery
- historical blockers
  - lanes that are blocked only because they are intentionally historical variants with named modern replacements

That distinction is now host-owned and flows through:

- project patch readiness receipts
- browser summary counts
- browser filtering
- project badges
- selected-project patchability details
- CLI refresh summaries

## Why This Matters

Before this wave, project patchability governance could say:

- this project has blocked lanes
- this project regressed
- this project improved

But it still compressed two different operational meanings into one bucket:

- danger
- obsolescence

Those require different operator responses.

Risky blockers should lead to:

- diagnosis
- caution
- possible contract or project repair

Historical blockers should lead to:

- replacement guidance
- modernization choice
- or simple acceptance that the old lane is retired

The system can now express that difference directly.

## Architectural Read

This is a meaningful deepening of the diagnosis/readiness stack.

We started with:

- is patching possible

then moved to:

- is this lane safe to run

then to:

- how patchable is this whole project over time

and now to:

- what kind of blocked state are we looking at

That is the beginning of interpretation-grade governance rather than raw state reporting.

## Current Language

The project-level patchability model now supports clearer top-level states such as:

- `patch-risk`
- `patch-historical`
- `patch-ready`
- `patch-stable`
- `patch-improved`
- `patch-regressed`

That is a stronger product vocabulary than a simple blocked/not-blocked split.

## Best Next Move

The next strongest move is probably not more browser polish.

It is to continue deepening host-owned interpretation where it earns its keep, for example:

1. carry replacement guidance into more host summaries and receipts beyond the browser
2. classify historical blockers versus risky blockers in additional automation-facing surfaces
3. only then decide whether a future product wave needs new capability work or more patch catalog curation

The main lesson from this checkpoint is:

- patchability governance is no longer just counting obstacles
- it is starting to explain what kind of obstacle the operator is facing
