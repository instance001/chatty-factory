# Architecture Checkpoint 14

This checkpoint marks the moment where governance UI modularization became real enough to count as architecture, not just cleanup.

The important shift is:

- governance presentation is no longer concentrated only in `crates/chatty_factory_ui/src/main.rs`

It now has its own small local module layer.

## What Changed

The latest UI extraction wave split the governance-oriented presentation into three focused pieces:

1. shared governance UI helpers
   - `crates/chatty_factory_ui/src/governance_ui.rs`

2. catalog-backed governance panels
   - `crates/chatty_factory_ui/src/catalog_governance_panels.rs`

3. extension-backed governance detail rendering
   - `crates/chatty_factory_ui/src/extension_governance_panel.rs`

The main UI file still owns orchestration, state, and the broader extension workbench, but it no longer needs to inline the full governance rendering surface.

## Why This Matters

The rebuild already had a shared governance model in the host and docs.

But until now, the UI side was still at risk of becoming:

- accurate
- useful
- and increasingly hard to maintain

because repeated governance rendering was still living mostly in one enormous file.

This checkpoint says that risk is now being addressed in the right way:

- not by over-abstracting the whole UI
- but by carving out the genuinely shared governance seams first

## What Is Now Shared In The UI

The governance layer now shares:

- artifact-set wording
- drift and baseline wording
- freshness, stale, and cooldown wording
- metric-strip rendering
- refresh-state rendering
- governed artifact reveal behavior
- governance detail-block rendering

And on top of that, the family/template catalog panels and the governed extension subsection are no longer hand-inlined in `main.rs`.

## Architectural Read

This is a healthy consolidation checkpoint.

It does not claim the UI is fully modularized.

It does say something narrower and more important:

- the governance subsystem now has a recognizable presentation boundary

That makes future work safer:

- future governed surfaces can reuse the governance layer more easily
- future wording changes are less likely to drift
- future `main.rs` extractions can proceed incrementally instead of as one risky rewrite

## Best Next Move

The strongest next move is not to split everything immediately.

It is to keep extracting by honest subsystem boundaries.

The most likely next candidates are:

1. extension workbench and artifact-preview rendering
2. proof-history and proof-export rendering
3. shared lane action rows outside governance-specific sections

So this checkpoint closes a good wave:

- governance is now shared in policy
- shared in receipts
- shared in UI vocabulary
- and increasingly shared in UI structure too.
