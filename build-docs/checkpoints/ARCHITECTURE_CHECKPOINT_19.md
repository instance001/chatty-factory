# Architecture Checkpoint 19

The patch diagnosis / X-ray layer now has a real UI subsystem boundary.

## What Changed

The new module:

- [patch_xray_panel.rs](./crates/chatty_factory_ui/src/patch_xray_panel.rs)

now owns the patch-surgery evidence surface instead of leaving that logic to accrete inside `main.rs`.

That module covers two related but distinct views:

- latest-result patch X-ray evidence
- selected-project recent patch X-ray history

and keeps the shared patch-diagnosis concepts together:

- diagnosis receipt loading
- intent-freeze receipt loading
- postcheck receipt loading
- outcome classification
- blocked / duplicate / applied summaries
- operator reveal actions

## Why This Boundary Is Honest

This is not a line-count extraction.

The patch X-ray layer is now a real product concept:

- patches are no longer treated as fire-and-forget edits
- the host now performs a diagnosis-before-surgery flow
- the UI now exposes that diagnosis as inspectable evidence

That means the rendering concerns around patch surgery form a real subsystem:

- surgery history for a project
- surgery evidence for the latest run
- surgery triage for blocked operations

Those concerns are more cohesive with each other than with the surrounding:

- governance panels
- proof panels
- generic request controls
- runtime registry dashboard

So splitting them into a dedicated module improves the architecture rather than merely shortening `main.rs`.

## Current Read

The rebuild is now at an interesting threshold:

- build and patch flows are operational
- governance spans the major deterministic substrates
- patch reliability is starting to gain its own observability layer

The X-ray module is the first visible sign that the patch flow is becoming more surgical and less optimistic.

## Recommended Next Direction

The best next move is not more UI decomposition by momentum.

The stronger direction is to deepen the patch diagnosis model itself, for example:

- richer project-structure summaries before patching
- more declarative lane-specific structural guards
- stronger post-op verification against diagnosis-derived invariants

If the UI changes next, they should follow those host-side diagnosis improvements rather than racing ahead of them.
