# Build Docs

This folder is the long-form build history for ChattyFactory.

It is intended to keep the project's architectural evolution, milestone notes,
reviews, and checkpoints in-repo as shippable documentation rather than leaving
that history scattered across the repository root.

## What Will Live Here

This archive is the intended home for historical build-time documentation such
as:

- architecture checkpoints
- milestone documents
- review and decision notes
- evolution snapshots for major subsystem waves

Examples from the current repo root include:

- `ARCHITECTURE_CHECKPOINT_*.md`
- `*_MILESTONE.md`
- `*_REVIEW.md`
- other historical build-planning notes

## What Should Stay Outside This Folder

Some documentation should remain in its current functional home:

- `README.md` for the repo entrypoint
- `USER_MANUAL.md` for operator-facing usage
- `docs/` for active reference docs
- package-local `README.md` files that document a live subsystem in place

## Current Layout

The historical build archive now lives in grouped subfolders:

- `build-docs/checkpoints/`
- `build-docs/milestones/`
- `build-docs/reviews/`
- `build-docs/plans/`

The top-level `README.md` keeps direct pointers into this archive so the build
history stays discoverable without crowding the repo root with the documents
themselves.
