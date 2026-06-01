# Patch Governance Milestone

This milestone now sits inside the shared governance scaffold documented in [Governance Model](../plans/GOVERNANCE_MODEL.md).

## Why This Is Next

Proof bundles now have real governance.

Composition bundles now have the first non-proof governance loop:

- lineage receipts
- drift vs seed/reference
- baseline vs last known-good live state
- bulk refresh
- UI counts, filters, badges, and freshness

The next logical subsystem is:

- `patch_recipe`

Patch recipes are the right next target because they sit closer to the main deterministic belt than proof manifests do, and they are reused constantly by:

- direct patch routing
- bounded composition
- promotion lifecycles
- acceptance shaping

If proofs gave us governance for extensible verification, and composition bundles gave us governance for mixed deterministic work orders, patch governance gives us governance for the everyday deterministic edit units the factory depends on most.

## Design Goal

Turn patch recipes from:

- live deterministic lane entries

into:

- curated deterministic lane entries

The host should be able to answer:

- where did this patch recipe come from?
- was it scaffolded, promoted, or hand-authored?
- how far has it drifted from its original intended shape?
- has it changed since the last known-good live baseline?
- which patch recipes are now the riskiest?

## Governance Shape

Patch governance should mirror the broad pattern already proven on proofs and composition bundles.

### 1. Lineage

Persist host-owned lineage for each governed patch recipe:

- source scaffold or extension id
- family id
- tool kind
- patch recipe id
- acceptance recipe pairing
- integrated artifact paths
- seed/reference recipe id if one exists

### 2. Drift

Classify drift against the original seed/reference shape:

- `unseeded`
- `seed_aligned`
- `lightly_customized`
- `structurally_customized`
- `drifted_risky`

This should be based on patch contract content, not just timestamps.

### 3. Last Known-Good Baseline

Track comparison against the last known-good live state for the patch lane:

- `stable_since_last_live`
- `changed_since_last_live`
- `regressed_since_last_live`
- `baseline_recorded`
- `no_live_baseline`

For patch recipes, "known-good" should generally mean:

- integrated
- host-wired or fully live as applicable
- validation passing
- compile-safe

### 4. Host-Owned Receipts

Persist dedicated receipts, likely under:

- `runtime/patch_governance_receipts/`

Each receipt should capture:

- recipe hashes
- paired acceptance hash
- lineage
- drift status
- baseline status
- notes
- timestamps

### 5. Bulk Refresh And Backfill

Add a host action such as:

- `refresh-patch-governance`

It should:

- scan governed patch recipes
- recompute lineage where possible
- recompute drift and baseline state
- persist receipts
- update registry or catalog-backed status surfaces

This matters because many existing patch lanes predate the governance layer.

### 6. UI Risk Surfacing

Give patch governance the same operational ergonomics:

- badges
- filters
- top-level counts
- risk-first sort participation
- refresh freshness

The goal is not a brand new admin screen.

The goal is to make patch risk visible in the same registry/detail flow operators already trust.

## Suggested First Proof

Start with an already-promoted deterministic patch lane that has real reuse pressure.

Good candidates are one of the helper-monitoring patch recipes already exercised by composed patch flows, such as:

- `helper_summary_status_chip`
- `helper_summary_lane_count_chip`
- `helper_summary_types_chip`

That keeps the first proof close to active bounded-composition traffic instead of choosing an isolated lane.

## Success Condition

This milestone is successful when at least one live patch recipe has:

- a host-owned governance receipt
- drift classification
- last-live baseline classification
- bulk refresh support
- UI surfacing equivalent in spirit to proof/composition governance

At that point, governance will have crossed from:

- proof infrastructure
- mixed deterministic bundles

into:

- the reusable patch-lane substrate itself
