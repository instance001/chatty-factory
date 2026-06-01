# Architecture Checkpoint 13

This checkpoint marks a smaller but important kind of maturity step:

- the governance program is no longer only broad
- the governance UI is starting to become internally modular

That matters because the rebuild now has a genuine shared governance presentation layer, not just several panels that happen to say similar things.

## What Changed

The latest UI passes centralized three layers of shared governance behavior:

1. governed artifact-set wording
2. drift and baseline wording
3. freshness, stale, and cooldown wording

Then the next passes moved beyond wording alone and started sharing structure:

4. metric-strip rendering
5. refresh-state rendering
6. governed artifact reveal behavior
7. governance detail-block rendering

And those helpers are no longer only inline inside one giant file. The shared governance UI layer now lives in a local module:

- `crates/chatty_factory_ui/src/governance_ui.rs`

## Why This Matters

Up to this point, the governance model had become broad and operational across:

- proofs
- composition bundles
- patch recipes
- helper lanes
- bridge lanes
- family manifests
- template bundles

But broad governance coverage brings a second risk:

- the UI that explains that governance can drift into repetitive, slightly inconsistent hand-built panels

This checkpoint is the first real answer to that risk.

The governance layer is now being treated as a reusable UI subsystem, not just as repeated local formatting inside `main.rs`.

## Architectural Read

This is not a big new capability wave.

It is a consolidation wave, and that is healthy.

The rebuild now has:

- a shared governance model in docs
- shared host-side governance loops across substrate classes
- a growing shared governance presentation layer in the UI

That means future governed surfaces should require less bespoke UI code and less wording cleanup.

## Best Next Move

The strongest next move is not another big feature by default.

It is to keep the same discipline:

- extract only the UI seams that are now genuinely shared
- avoid premature over-abstraction
- keep enough local detail so each governed substrate still feels honest

If the UI keeps growing, the likely next clean step is:

- split more of the governance-oriented registry rendering out of `main.rs` into small local modules

But this checkpoint says the first important threshold has already been crossed:

- governance is now shared in presentation, not only in policy.
