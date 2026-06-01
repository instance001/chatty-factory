# Architecture Checkpoint 15

This checkpoint marks the point where the UI extraction wave moved beyond governance-only rendering and started carving out the adjacent lane workbench surface too.

The important shift is:

- `main.rs` is no longer only shedding governance helpers
- it is now shedding neighboring lane-detail subsystems by honest boundary

## What Changed

The latest UI pass extracted the extension workbench and artifact-preview slice into:

- `crates/chatty_factory_ui/src/extension_workbench_panel.rs`

That module now owns:

- scaffold and source-spec reveal actions
- integrated / promotion / apply-patch artifact lists
- lane workbench blocker display
- notes and acceptance-target reveal actions
- source spec preview
- implementation notes preview
- acceptance target preview
- promotion compare
- apply-patch compare
- first integrated file preview

This sits alongside the earlier governance-oriented split:

- `governance_ui.rs`
- `catalog_governance_panels.rs`
- `extension_governance_panel.rs`

## Why This Matters

Checkpoint 14 established that governance presentation had become modular enough to count as architecture.

Checkpoint 15 says something slightly broader:

- lane detail rendering now has more than one real subsystem boundary

That is important because the biggest risk in the UI was not only repeated governance wording.

It was the steady accumulation of multiple heavy detail surfaces inside one file:

- governance evidence
- artifact reveals
- workbench actions
- preview panes
- compare panes

The current extraction wave is now attacking that risk directly.

## Architectural Read

This is still an incremental modularization strategy, not a rewrite.

That is the right choice here.

The rebuild now has:

- a composition root in `main.rs`
- focused governance presentation helpers
- focused governance catalog panels
- focused governed extension rendering
- focused workbench and artifact-preview rendering

That is enough structure to keep moving without prematurely turning the UI into a maze of tiny files.

## Best Next Move

The strongest next step is still restraint plus continuity:

1. keep extracting by honest UI boundary
2. avoid splitting thin one-off fragments
3. document each new boundary as it becomes stable

The most likely next candidates are now:

- proof history / proof export rendering
- non-governance lane action rows
- registry-list row rendering

So this checkpoint closes a second good UI-architecture wave:

- governance got its own module layer
- and the neighboring lane workbench surface has now started to leave `main.rs` too.
