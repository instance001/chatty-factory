# Family Governance Milestone

This milestone now sits inside the shared governance scaffold documented in [Governance Model](../plans/GOVERNANCE_MODEL.md).

## Why This Is Next

The governance pattern is now real across:

- `proof_harness_bundle`
- `composition_bundle`
- `patch_recipe`
- `helper_lane`
- `chattycog_bridge_lane`

The next substrate should be:

- family manifests and family-manifest-adjacent starters

This is the strongest next target because family manifests sit above the lower deterministic lanes and define:

- base capability surfaces
- family-level primitive classes
- primitive adapter declarations
- routing expectations
- substrate identity for later proof comparison

If the lower layers are now curated, the next architectural gap is the layer that declares what those lower layers are supposed to mean in the first place.

## Design Goal

Turn family manifests from:

- live capability declarations

into:

- curated capability declarations

The host should be able to answer:

- where did this family capability surface come from?
- what build-level primitive classes does it claim to provide now?
- how far has it drifted from its original seed/reference shape?
- has it changed since the last known-good live state?
- which family surfaces are now the riskiest to trust as routing and proof anchors?

## What Counts As A Family Governance Unit

A governed family unit should carry at least:

- family id
- tool kind or family scope
- manifest path
- declared build primitive classes
- declared primitive adapters
- source scaffold or seed lineage if one exists
- drift status vs seed/reference
- baseline status vs last known-good live state
- notes explaining risky changes

This should be host-owned state, not only comments in manifest JSON.

## Architectural Scope

### 1. Lineage

Persist host-owned lineage for family manifests:

- source stub path if scaffolded
- scaffold root if created from an extension workflow
- family manifest path
- family id
- declared family primitive classes
- declared family primitive adapters
- seed/reference family if one exists

### 2. Drift

Classify family drift against the original seed/reference shape:

- `unseeded`
- `seed_aligned`
- `lightly_customized`
- `structurally_customized`
- `drifted_risky`

For family manifests, this should consider:

- build primitive class drift
- primitive adapter drift
- family capability surface drift

### 3. Last Known-Good Baseline

Track change against the last known-good live family state:

- `stable_since_last_live`
- `changed_since_last_live`
- `regressed_since_last_live`
- `baseline_recorded`
- `no_live_baseline`

For family manifests, "known-good" should usually mean:

- manifest parses and validates
- family remains resolvable by the host
- routing and adapter lookup still succeed
- compile safety still passes

### 4. Host-Owned Receipts

Persist dedicated receipts, likely under:

- `runtime/family_governance_receipts/`

Each receipt should capture:

- manifest hashes
- declared primitive classes
- declared primitive adapters
- lineage
- drift status
- baseline status
- notes
- timestamps

### 5. Bulk Refresh And Backfill

Add a host action such as:

- `refresh-family-governance`

It should:

- scan governed family-manifest entries
- recompute hashes and capability state
- recompute drift and baseline status
- persist receipts
- update registry-backed or family-registry-backed status surfaces

### 6. UI Risk Surfacing

Family governance should get the same operator affordances:

- drift badges
- baseline badges
- counts
- filters
- risk-first sort participation
- refresh freshness
- stale warnings

The goal is to keep family governance in the same registry/detail flow rather than building another side channel.

## First Proof Shape

Start with a family that already has rich declared primitive data, ideally one of:

- `static_web_dashboard`
- `chattycog_webview_module`

The first proof should be:

- host-readable
- refreshable into a governance receipt
- classified for drift and baseline
- surfaced in the UI registry/detail flow

## Success Condition

This milestone is successful when at least one governed family surface has:

- a host-owned governance receipt
- drift classification
- last-live baseline classification
- bulk refresh support
- UI surfacing equivalent in spirit to proof/composition/patch/helper/bridge governance

At that point, governance will span both:

- the deterministic lane machinery
- and the family capability layer that defines the machinery's meaning.
