# Architecture Checkpoint 16

This checkpoint marks the point where the UI extraction wave moved from governance-only structure into the adjacent lane workbench surface.

That matters because `main.rs` is no longer shrinking only by helper consolidation.

It is shrinking by subsystem boundary.

## What Changed

The latest UI extraction pass added:

- `crates/chatty_factory_ui/src/extension_workbench_panel.rs`

That module now owns the non-governance lane workbench surface:

- scaffold and source-spec reveal actions
- integrated / promotion / apply-patch artifact lists
- blocker display
- notes and acceptance-target reveal actions
- source spec preview
- implementation notes preview
- acceptance target preview
- promotion compare
- apply-patch compare
- first integrated file preview

This sits beside the already-extracted UI modules:

- `governance_ui.rs`
- `catalog_governance_panels.rs`
- `extension_governance_panel.rs`

## Why This Matters

Checkpoint 15 established that lane detail rendering had started to form more than one real UI subsystem boundary.

Checkpoint 16 confirms that this was not a one-off cleanup.

The extraction pattern is now strong enough to carry neighboring lane-detail surfaces too:

- governance evidence
- workbench actions
- artifact preview and compare panes

That is a healthier path than waiting until `main.rs` becomes too brittle to safely touch.

## Architectural Read

The current UI shape is now easier to describe honestly:

- `main.rs` is the composition root
- governance presentation has its own module layer
- catalog-backed governance has its own panel layer
- governed lane evidence has its own panel layer
- non-governance lane workbench and preview rendering has its own panel layer

That does not mean the whole UI is fully modularized.

It does mean the extraction strategy is now repeatable.

## Best Next Move

The strongest next candidates are now:

1. proof history and proof export rendering
2. lane action rows outside the extracted workbench section
3. registry-list row rendering

The important discipline stays the same:

- extract by cohesive UI behavior
- keep orchestration in `main.rs`
- avoid slicing off tiny fragments just to reduce line count

So this checkpoint closes another good wave:

- governance moved out first
- then the neighboring workbench surface followed
- and `main.rs` is now being reduced by real subsystem boundaries instead of by opportunistic helper churn.
