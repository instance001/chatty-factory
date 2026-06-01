# Architecture Checkpoint 24

## What Changed

Project patchability governance now distinguishes between two different kinds of blocked patch posture:

- projects that are risky because lanes are structurally mismatched or otherwise unsafe
- projects that are blocked only by intentionally historical lanes that already have modern replacements

That distinction is now host-owned instead of being inferred ad hoc from UI text.

The host now persists:

- `blocked_lane_reasons`
- `superseded_blocked_lane_replacements`

inside each per-project patchability receipt.

The UI now uses that model in three places:

- project browser summary counts
- project browser filtering
- selected-project patchability details

and the browser rows now visibly mark projects with:

- `[historical-blocker]`

when their blockers are guided obsolescence rather than raw structural risk.

## Why This Matters

Before this wave, project patchability governance could answer:

- is this project blocked
- is it regressed
- is it improved

But it could not answer a more important operator question:

- is this blockage dangerous, or is it just the residue of an old lane we should no longer use?

That difference matters because the operator response is different:

- risky blockers should trigger caution and diagnosis
- historical blockers should trigger replacement guidance

The system now owns that distinction explicitly.

## Architectural Read

This is a meaningful step deeper into diagnosis/readiness governance.

We are no longer only measuring:

- whether surgery is safe

We are also classifying:

- what kind of unsafety or non-readiness we are looking at
- whether the right next move is repair, modernization, or replacement

That is closer to the intended product behavior:

- not just patchability scoring
- but patchability interpretation

## Best Next Move

The next strongest move is probably to keep deepening the interpretation layer rather than widening the browser again.

Two especially good candidates are:

1. add project-level counts for:
   - risky blockers
   - historical blockers

2. carry replacement guidance into richer host-owned receipts or summaries beyond the browser, so the same distinction can be consumed by CLI and future automation more directly

The main architectural lesson from this checkpoint is:

- blocked is no longer one bucket
- and that makes the patchability model much more useful
