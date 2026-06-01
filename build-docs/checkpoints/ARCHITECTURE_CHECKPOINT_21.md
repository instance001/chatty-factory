# Architecture Checkpoint 21

Patch-lane contract coverage is now strong enough that the remaining risk cluster is small, visible, and mostly honest.

## What Changed

This wave widened declarative structural contracts across more patch lanes and tightened several existing ones.

The biggest practical shift is:

- more lanes now classify as `narrow_surface_contract`
- more old handler-shape failures now freeze safely before surgery
- fewer lanes rely on optimistic broad-surface assumptions

Two especially important results:

1. ChattyCog helper-summary lanes were tightened to the actual surfaces they modify.

That moved many from:

- `broad_surface_contract`

to:

- `narrow_surface_contract`

because their real footprint is mostly:

- `entrypoints`
- `contract_files`
- `style_surfaces`

2. Rust CLI legacy lanes now fail safely instead of crashing.

In particular:

- `file_output`
- `severity_filter`

now freeze safely when newer structured-output surfaces are already present, instead of reaching a stale handler shape and failing deep inside patch code.

## Why This Matters

Earlier, the system still had a gap where:

- lane availability looked plausible
- but the underlying project shape had already evolved
- and the handler could still crash before preflight caught it

That gap is shrinking.

The patch stack is now more consistent:

- lane contract declares assumptions
- diagnosis checks those assumptions
- freeze blocks stale or conflicting shapes
- postcheck verifies the actual surgical footprint

## Current Read

The remaining `legacy_shape_sensitive` lanes now look much more like a bounded set of honest exceptions than a broad reliability problem.

That is good architectural news.

It means we are no longer mainly fighting:

- missing contract coverage

We are increasingly deciding:

- which remaining legacy-sensitive lanes should stay legacy-sensitive because their old insertion shape is genuinely obsolete
- and which should be re-authored into a modern narrow contract instead

## Recommended Next Direction

The best next move is not blind widening by momentum.

The healthy next step is:

1. review the remaining legacy-sensitive lanes as a small named cluster
2. decide which should:
   - stay blocked as historical lanes
   - be superseded by newer lanes
   - or be rewritten into modern narrow-contract variants

At this point, the architecture is ready for that more selective judgment.
