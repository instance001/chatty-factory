# Helper Governance Milestone

This milestone now sits inside the shared governance scaffold documented in [Governance Model](../plans/GOVERNANCE_MODEL.md).

## Why This Is Next

The governance pattern is now real across:

- `proof_harness_bundle`
- `composition_bundle`
- `patch_recipe`

The next subsystem should be:

- helper lanes / helper service bundles

This is the strongest next target because helpers are where deterministic machinery stops being only:

- declarative
- promoted
- compile-safe

and becomes:

- runtime-active
- cross-family
- acceptance-shaping
- drift-prone over time

That makes helper governance the next real test of whether the host can curate deterministic runtime substrate, not only deterministic manifests.

## Design Goal

Turn helper lanes from:

- live reusable helper machinery

into:

- curated reusable helper machinery

The host should be able to answer:

- where did this helper lane/service bundle come from?
- which primitive kinds does it currently provide?
- how far has it drifted from its original seed/reference shape?
- has it changed since the last known-good live runtime baseline?
- which helpers are now the riskiest to trust?

## What Counts As A Helper Governance Unit

A governed helper unit should carry at least:

- helper id
- family scope or cross-family scope
- declared helper primitive ids
- declared helper primitive kinds
- helper runtime/status artifact paths
- source scaffold or promotion lineage
- drift status vs seed/reference
- baseline status vs last known-good live runtime state
- notes explaining risky changes

This has to be host-owned state, not just comments in helper specs.

## Architectural Scope

### 1. Lineage

Persist host-owned lineage for helper lanes:

- source stub path
- scaffold root
- helper id
- family id or shared scope
- primitive ids
- primitive kinds
- runtime receipt paths
- seed/reference helper pattern if one exists

### 2. Drift

Classify helper drift against the original seed/reference shape:

- `unseeded`
- `seed_aligned`
- `lightly_customized`
- `structurally_customized`
- `drifted_risky`

For helpers, this should consider more than file hashes alone.

It should also consider:

- helper primitive-set drift
- helper input/output path drift
- helper status/summary contract drift

### 3. Last Known-Good Baseline

Track change against the last known-good live helper state:

- `stable_since_last_live`
- `changed_since_last_live`
- `regressed_since_last_live`
- `baseline_recorded`
- `no_live_baseline`

For helpers, "known-good" should usually mean:

- helper spec resolves
- helper runtime status is healthy
- helper summary/output artifacts are coherent
- acceptance still passes

### 4. Host-Owned Receipts

Persist dedicated receipts, likely under:

- `runtime/helper_governance_receipts/`

Each receipt should capture:

- helper spec hash
- observed runtime/status/summary hashes
- primitive ids and kinds
- lineage
- drift status
- baseline status
- notes
- timestamps

### 5. Bulk Refresh And Backfill

Add a host action such as:

- `refresh-helper-governance`

It should:

- scan governed helper lanes
- recompute hashes and helper primitive state
- recompute drift and baseline status
- persist receipts
- update registry or helper catalog-backed status surfaces

### 6. UI Risk Surfacing

Helper governance should get the same operator affordances:

- drift badges
- baseline badges
- counts
- filters
- risk-first sort participation
- refresh freshness
- stale warnings

The goal is to keep helper governance in the same registry/detail flow, not to create another isolated admin tool.

## First Proof Shape

Start with the current reusable local inbox helper bundle:

- `local_inbox_helper`

and its cross-family usage across:

- `chattycog_webview_module`
- `static_web_dashboard`

That is the best first proof because:

- it already has explicit helper primitive ids and kinds
- it already has runtime receipts and status snapshots
- it already participates in acceptance
- it is already used across more than one family

If helper governance works there, it is immediately testing real shared runtime substrate instead of a one-off helper.

## Success Condition

This milestone is successful when at least one live helper bundle has:

- a host-owned governance receipt
- drift classification
- last-live baseline classification
- bulk refresh support
- UI surfacing equivalent in spirit to proof/composition/patch governance

At that point, governance will have crossed from static deterministic assets into reusable deterministic runtime assets.
