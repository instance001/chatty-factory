# Template Governance Milestone

This milestone now sits inside the shared governance scaffold documented in [Governance Model](../plans/GOVERNANCE_MODEL.md).

## Why This Is Next

The governance pattern now covers:

- proof harness bundles
- composition bundles
- patch recipes
- helper lanes
- bridge lanes
- family manifests

The strongest remaining unguided product catalog is:

- `templates/`

That matters because templates are not just supporting notes or transient implementation artifacts.

They are:

- versioned product assets
- directly consumed by the runtime template loader
- upstream of generated project shape
- capable of drifting independently from manifests and lane metadata

If we want governance to cover the full host-owned deterministic belt, templates are the next logical substrate.

## Design Goal

Turn templates from:

- reusable versioned product assets

into:

- curated reusable versioned product assets

The host should be able to answer:

- which template bundle is this?
- what governed substrate does it belong to?
- how far has it drifted from its seed or reference shape?
- has it changed since the last known-good baseline?
- which templates are now the riskiest to trust?

## What Counts As A Template Governance Unit

A governed template unit should carry at least:

- template bundle id
- template category
- template root path
- template artifact paths
- source scaffold or seed lineage if one exists
- drift status vs seed/reference
- baseline status vs last known-good baseline
- notes explaining risky changes

This should be host-owned state, not just naming convention.

## Candidate Template Catalogs

The first template governance wave should focus on the reusable template roots already shaping live outputs:

1. `templates/families/`
- base family build template bundles
- family-starter and promoted family-specific template variants

2. `templates/patches/`
- patch-lane template bundles
- composition-adjacent patch template starters

3. `templates/helpers/`
- helper-lane starter template bundles

4. `templates/wrappers/chattycog/`
- wrapper/runtime bridge template assets
- handshake, manifest, visual-load, bridge JS

These are primary build assets, not merely documentation.

## Architectural Scope

### 1. Lineage

Persist host-owned lineage for each governed template bundle:

- template category
- template bundle id or slug
- root directory
- artifact file list
- source scaffold or promotion lineage if applicable
- seed/reference template archetype if one exists

### 2. Drift

Classify template drift against the original seed/reference shape:

- `unseeded`
- `seed_aligned`
- `lightly_customized`
- `structurally_customized`
- `drifted_risky`

For templates, this should consider:

- file-set drift
- template artifact hash drift
- category-specific surface drift

Examples:

- family templates may drift in output surface structure
- patch templates may drift in supported operator shape
- wrapper templates may drift in runtime contract surface

### 3. Last Known-Good Baseline

Track change against the last known-good governed template state:

- `stable_since_last_live`
- `changed_since_last_live`
- `regressed_since_last_live`
- `baseline_recorded`
- `no_live_baseline`

For templates, "known-good" should usually mean:

- template bundle exists and validates
- expected file set is present
- template catalog remains host-readable
- dependent generation paths still compile or validate where applicable

### 4. Host-Owned Receipts

Persist dedicated receipts, likely under:

- `runtime/template_governance_receipts/`

Each receipt should capture:

- artifact hashes
- template category
- lineage
- drift status
- baseline status
- notes
- timestamps

### 5. Bulk Refresh And Backfill

Add a host action such as:

- `refresh-template-governance`

It should:

- scan governed template bundles
- recompute hashes and file-set state
- recompute drift and baseline status
- persist receipts
- update refresh-status state

This matters because many template bundles predate governance and should be backfilled without manual repair.

### 6. UI Risk Surfacing

Template governance should get the same operator affordances:

- refresh action
- refresh freshness line
- stale warning
- optional launch auto-refresh with cooldown
- counts
- template picker or category grouping
- receipt access

The UI shape may end up closer to family governance than extension-row governance, because template bundles are catalog assets rather than pending lanes.

## First Proof Shape

The cleanest first proof is:

- `templates/families/static_web_dashboard/`

Why this is the best first proof:

- it is a core family template bundle
- it already participates in active builds
- it is easy to identify as one template unit
- it has a clear path to later family-template catalog expansion

The second likely proof after that should be:

- `templates/wrappers/chattycog/`

because wrapper/runtime contract drift is one of the riskiest template categories.

## Success Condition

This milestone is successful when:

- at least one family template bundle has a host-owned governance receipt
- at least one wrapper or patch/helper template bundle follows it
- the host can bulk refresh governed template bundles
- the UI can surface template-governance freshness and risk without opening files manually
- older template bundles can be backfilled into the governance model

At that point, governance will cover not only manifests and lanes, but also the reusable product templates that those manifests and lanes ultimately depend on.

## Recommended Order

1. Add template governance receipt and refresh-status contracts.
2. Define governed template bundle discovery by category.
3. Implement template artifact validation and hashing.
4. Implement drift classification.
5. Implement baseline classification.
6. Add bulk refresh/backfill host action.
7. Surface the result in the UI with catalog-style freshness and receipt access.

## What Not To Do

- Do not govern every individual template file as if it were its own first-class substrate.
- Do not force templates into `pending_lanes.json`; they are catalog assets, not lane entries.
- Do not split acceptance-template or promotion-template material into independent governance too early if the real governed unit is the broader template bundle.
- Do not skip wrapper templates just because they are smaller; they are contract-sensitive and high leverage.

## Why This Matters

Right now the rebuild governs:

- what capabilities are declared
- what deterministic lanes are promoted
- what proofs are trusted

Template governance adds the missing layer beneath those:

- what reusable product assets are actually shaping the generated builds

That is the next strong step toward a factory that not only governs its contracts and lanes, but also governs the reusable template machinery those contracts and lanes rely on.
