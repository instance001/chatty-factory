# Bridge Governance Milestone

This milestone now sits inside the shared governance scaffold documented in [Governance Model](../plans/GOVERNANCE_MODEL.md).

## Why This Is Next

The governance pattern is now real across:

- `proof_harness_bundle`
- `composition_bundle`
- `patch_recipe`
- `helper_lane`

The next subsystem should be:

- bridge lanes

This is the strongest next target because bridge lanes sit at the runtime-facing coordination layer where deterministic machinery has to stay honest across:

- host/runtime contracts
- ChattyCog-facing integrations
- surface ownership boundaries
- capability declarations that can drift separately from build families

If helper governance proved the host can curate reusable runtime-adjacent service substrate, bridge governance should prove it can curate reusable runtime-adjacent coordination substrate.

## Design Goal

Turn bridge lanes from:

- live integration machinery

into:

- curated integration machinery

The host should be able to answer:

- where did this bridge lane come from?
- which bridge capabilities does it claim to provide now?
- how far has it drifted from its original seed/reference shape?
- has it changed since the last known-good live state?
- which bridge lanes are now the riskiest to trust?

## What Counts As A Bridge Governance Unit

A governed bridge unit should carry at least:

- bridge lane id
- family id or hosting scope
- declared bridge capabilities
- source scaffold/promotion lineage
- integrated artifact paths
- drift status vs seed/reference
- baseline status vs last known-good live state
- notes explaining risky changes

This should be host-owned state, not only comments in bridge stubs.

## Architectural Scope

### 1. Lineage

Persist host-owned lineage for bridge lanes:

- source stub path
- scaffold root
- bridge lane id
- family id or hosting scope
- declared bridge capabilities
- integrated artifact paths
- seed/reference bridge pattern if one exists

### 2. Drift

Classify bridge drift against the original seed/reference shape:

- `unseeded`
- `seed_aligned`
- `lightly_customized`
- `structurally_customized`
- `drifted_risky`

For bridge lanes, this should consider:

- capability-set drift
- hosting-mode drift
- bridge artifact drift

### 3. Last Known-Good Baseline

Track change against the last known-good live bridge state:

- `stable_since_last_live`
- `changed_since_last_live`
- `regressed_since_last_live`
- `baseline_recorded`
- `no_live_baseline`

For bridge lanes, "known-good" should usually mean:

- integrated bridge artifacts present
- host/runtime validation passing
- compile-safe
- capability contract still coherent

### 4. Host-Owned Receipts

Persist dedicated receipts, likely under:

- `runtime/bridge_governance_receipts/`

Each receipt should capture:

- bridge artifact hashes
- declared capability set
- lineage
- drift status
- baseline status
- notes
- timestamps

### 5. Bulk Refresh And Backfill

Add a host action such as:

- `refresh-bridge-governance`

It should:

- scan governed bridge lanes
- recompute hashes and capability state
- recompute drift and baseline status
- persist receipts
- update registry-backed status surfaces

### 6. UI Risk Surfacing

Bridge governance should get the same operator affordances:

- drift badges
- baseline badges
- counts
- filters
- risk-first sort participation
- refresh freshness
- stale warnings

The goal is to keep bridge governance in the same registry/detail flow rather than building another side channel.

## First Proof Shape

Start with a helper- or ChattyCog-adjacent bridge lane that already carries explicit bridge capabilities, ideally one produced from the existing bridge scaffold path rather than a hand-authored artifact.

The first proof should be:

- promoted
- host-wired
- fully live
- then refreshed into a recorded governance baseline

## Success Condition

This milestone is successful when at least one live bridge lane has:

- a host-owned governance receipt
- drift classification
- last-live baseline classification
- bulk refresh support
- UI surfacing equivalent in spirit to proof/composition/patch/helper governance

At that point, governance will span both reusable deterministic runtime services and reusable deterministic runtime coordination layers.
