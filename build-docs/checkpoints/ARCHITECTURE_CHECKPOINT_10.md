# Architecture Checkpoint 10

This checkpoint comes after governance reached the family capability layer.

That matters because governance is no longer only curating:

- proof contracts
- deterministic lanes
- reusable runtime substrate

It is now also curating the family surfaces that routing, primitive selection, and proof execution depend on.

## What Is Now Real

The host now governs six substrate classes:

1. Proof harness bundles
- seed lineage
- drift vs seed
- baseline vs last passing proof
- bulk refresh/backfill
- UI quality and risk surfacing

2. Composition bundles
- lineage receipts
- drift classification
- baseline vs last known-good live
- bulk refresh/backfill
- UI counts, filters, freshness, and risk surfacing

3. Patch recipes
- lineage receipts
- drift classification
- baseline vs last known-good live
- bulk refresh/backfill
- UI counts, filters, freshness, and risk surfacing

4. Helper lanes
- lineage receipts
- drift classification
- baseline vs last known-good live
- bulk refresh/backfill
- UI counts, filters, freshness, and risk surfacing

5. Bridge lanes
- lineage receipts
- drift classification
- baseline vs last known-good live
- bulk refresh/backfill
- UI counts, filters, freshness, and risk surfacing

6. Family manifests
- manifest receipts
- drift classification
- baseline vs last known-good live
- bulk refresh/backfill
- a dedicated UI catalog with freshness and receipt access

## What Improved Since Checkpoint 9

Checkpoint 9 ended with family governance as the next missing substrate.

Since then:

### 1. Family governance is real

The rebuild now has:

- family governance receipts under:
  - `runtime/family_governance_receipts/`
- family governance refresh status under:
  - `runtime/family_governance_refresh_status.json`
- drift and baseline classification for family manifests
- bulk refresh through:
  - `refresh-family-governance`

### 2. The full built-in family set is governed

The stricter first-pass validator exposed a real gap:

- four built-in families were missing `primitive_adapters`

That gap is now closed, and governance refresh covers:

- `static_web_dashboard`
- `chattycog_webview_module`
- `chattycog_native_window_module`
- `chattycog_workspace_module`
- `python_cli_tool`
- `rust_cli_tool`

with:

- `refreshed_entries = 6`
- `skipped_entries = 0`

### 3. Family governance is operationally surfaced

Family manifests are not forced into `pending_lanes.json`.

Instead, the UI now treats family governance as a parallel governed catalog with:

- refresh action
- freshness line
- stale warning
- launch auto-refresh with cooldown
- governed/stable/changed/regressed counts
- family picker
- receipt access
- manifest access

That is the right architectural shape because families are catalog assets, not pending extension entries.

## Why This Matters

This is the first point where governance covers both:

- the machinery the factory runs
- and the capability surfaces that define what that machinery can mean

That makes the governance layer feel less like "receipt polish around generated things" and more like host-owned curation infrastructure.

The host is now maintaining:

- proof logic
- composition logic
- deterministic lane logic
- reusable helper and bridge substrate
- family capability surfaces

over time, with explicit baseline and drift models.

## What Changed Architecturally

The important shift is not just "one more governed substrate."

It is that the governance pattern now has enough coverage to expose its shared shape:

- reference seed or baseline
- current artifact snapshot
- drift classification
- last-known-good baseline classification
- bulk refresh/backfill
- visible UI freshness and risk posture

At this point, the next architectural question is no longer:

- can governance be applied somewhere else?

It is:

- what is the shared governance model across these substrates?
- which parts are universal?
- which parts remain intentionally layer-specific?

## Best Next Move

The strongest next move is:

**shared governance model consolidation**

Not by erasing the substrate differences, but by documenting the common contract pattern clearly enough that future governed catalogs can follow it with less bespoke thought.

After that, the next practical decision is whether there are any remaining repo-extensible catalogs that should join the same governance loop, or whether the right move is to harden and simplify the shared model we now have.

## Short Read

Checkpoint 10 says:

- governance now spans proofs, composition bundles, patch recipes, helper lanes, bridge lanes, and family manifests
- family governance is not just present, it is operationally surfaced in the UI with freshness parity

The next architectural move is to consolidate this into one shared governance model so the pattern is explicit, teachable, and reusable.
