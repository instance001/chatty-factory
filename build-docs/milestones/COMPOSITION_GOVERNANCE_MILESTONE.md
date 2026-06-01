# Composition Governance Milestone

This milestone now sits inside the shared governance scaffold documented in [Governance Model](../plans/GOVERNANCE_MODEL.md).

## Why This Is Next

The proof harness now has a real governance layer:

- quality status
- seed lineage
- drift vs seed
- baseline vs last passing run
- bulk refresh/backfill
- UI risk badges, filters, and summaries

That is a meaningful architectural win, but right now it is still localized to:

- `proof_harness_bundle`

The next strongest move is to carry the same governance discipline into the nearest non-proof deterministic subsystem:

- `composition_bundle`

This is the best next target because composition bundles already have:

- mixed-layer lifecycle data
- host-owned promotion and validation flow
- bounded work-order semantics
- user-visible importance when nearby requests do not fit one direct lane

So this milestone is about turning composition bundles from:

- live deterministic work bundles

into:

- curated deterministic work bundles

## Design Goal

Give `composition_bundle` the same broad governance capabilities that proofs now have:

- source lineage
- seed/reference lineage
- drift classification
- last known-good baseline comparison
- bulk refresh/backfill
- UI risk surfacing

The host should be able to answer questions like:

- what fallback/scaffold request produced this composition bundle?
- what mixed-layer profile seeded it?
- how far has it drifted from that original intended shape?
- has it changed since the last known-good live state?
- which composition bundles are the riskiest right now?

## What Counts As A Composition Governance Unit

A governed composition bundle should carry at least:

- bundle id / pending entry id
- unresolved layers
- requested capabilities
- integrated artifact paths
- reference seed profile
- drift status vs seed
- baseline status vs last known-good live state
- notes explaining risky changes

This is not just documentation.

It should be host-owned state that survives refresh, promotion, and UI inspection.

## Architectural Scope

### 1. Source And Seed Lineage

Composition bundles should persist:

- source stub path
- scaffold root
- unresolved layer profile
- missing family build classes
- missing patch classes
- missing helper kinds

And they should gain a seed/reference notion similar to proofs, likely based on:

- closest built-in mixed-layer profile
- or closest previously successful composition bundle pattern

The point is not to force every bundle to have a canonical seed.

The point is to let the host classify:

- seed-aligned
- lightly customized
- structurally customized
- drifted risky
- unseeded

for mixed deterministic bundles too.

### 2. Last Known-Good Baseline

Composition bundles should also persist comparison against the last known-good live state.

For this subsystem, "known-good" probably means:

- `fully_live`
- integrated artifacts present
- validation passing
- compile-safe

The host should then classify:

- `stable_since_last_live`
- `changed_since_last_live`
- `regressed_since_last_live`
- `baseline_recorded`
- `no_live_baseline`

This is the composition-bundle analog to proof last-pass governance.

### 3. Host-Owned Governance Receipts

The host should persist dedicated receipts, likely under something like:

- `runtime/composition_governance_receipts/`

Each receipt should capture:

- current artifact hashes
- seed/reference lineage
- drift status
- baseline status
- notes
- timestamps

This keeps governance separate from:

- promotion artifacts
- apply-patch artifacts
- composition execution receipts

### 4. Bulk Refresh/Backfill

Like proofs, composition bundles need a host action that can refresh the whole subsystem.

Example:

- `refresh-composition-governance`

It should:

- load all composition-bundle entries
- recompute seed/reference lineage
- recompute current artifact hashes
- recompute baseline state
- persist receipts
- update registry fields

This is especially important for older composition bundles that predate the governance layer.

### 5. UI Risk Surfacing

The extension registry should gain composition-governance scan features parallel to proof governance:

- drift/baseline badges
- filters
- risk sort
- top-level counts
- freshness/refresh visibility

Not necessarily a giant new panel.

The goal is to make composition risk visible in the same list/detail flow operators already use.

## Suggested Contracts

This milestone does not have to copy proof contracts exactly, but it should mirror their roles.

Likely additions:

- composition governance receipt struct
- registry fields on `PendingExtensionEntry`
- refresh status receipt for composition governance

Potential registry fields:

- `composition_lineage_receipt_path`
- `composition_drift_status`
- `composition_drift_notes`
- `composition_change_since_last_live_status`
- `composition_change_since_last_live_notes`

## First Proof Shape

Use the existing mixed static-dashboard metadata-row example first:

- `pending-lane-1779405687159-0`
- `extension_kind = composition_bundle`
- unresolved layers:
  - `family_build`
  - `patch`

Why this is a good first proof:

- it is already fully live
- it already has meaningful mixed-layer identity
- it already came from the scaffold/promotion flow
- it is structurally close to proof governance in spirit, but not identical

Success on that lane means the governance layer is not proof-only anymore.

## Implementation Order

1. Add composition governance receipt and registry fields.
2. Implement drift/reference classification for composition bundles.
3. Implement last-known-good live baseline classification.
4. Add bulk refresh/backfill host action.
5. Surface the result in the UI with badges, filters, and risk sort.

## What Counts As Success

This milestone is successful when:

- at least one existing `composition_bundle` entry gets a full governance receipt
- registry entries persist drift and baseline states
- the host can bulk refresh all composition bundles
- the UI can surface composition-governance risk without opening files manually
- older composition bundles can be backfilled into the governance model

## What Not To Do

- Do not treat composition governance as just more proof governance under a renamed field.
- Do not skip baseline comparison; seed-only drift is not enough.
- Do not hide governance only in runtime receipts without surfacing it in the registry/UI.
- Do not force perfect canonical seeds where the right answer is "unseeded but understandable."

## Why This Matters

This is the clearest next extension of the architecture we just proved.

If proof governance remains isolated, then we've improved one subsystem.

If composition bundles get the same governance treatment, then we've shown the factory is learning a broader pattern:

- host-owned deterministic machinery
- host-owned deterministic governance

That is a major step toward a factory that does not just execute and extend work, but curates it over time.
