# Architecture Checkpoint 11

This checkpoint comes after governance reached the template layer.

That matters because the governance scaffold no longer stops at:

- proofs
- manifests
- promoted deterministic lanes
- reusable runtime substrate

It now also covers the reusable product assets that actually shape generated outputs.

## What Is Now Real

The rebuild now has host-owned governance across seven substrate classes:

1. proof harness bundles
2. composition bundles
3. patch recipes
4. helper lanes
5. bridge lanes
6. family manifests
7. template bundles

This is the first point where governance spans:

- contract catalogs
- lane catalogs
- runtime-facing reusable substrate
- family capability surfaces
- reusable product templates

That is broad enough to count as real infrastructure, not just an accumulation of subsystem-specific improvements.

## What Improved Since Checkpoint 10

Checkpoint 10 ended by identifying the strongest remaining unguided catalog:

- `templates/`

Since then, three important things became true.

### 1. Template governance is real

The host now supports:

- template governance receipts under:
  - `runtime/template_governance_receipts/`
- template governance refresh status under:
  - `runtime/template_governance_refresh_status.json`
- template bundle discovery across:
  - `templates/families/`
  - `templates/patches/`
  - `templates/helpers/`
  - `templates/wrappers/`
- template drift classification
- template last-live baseline classification
- bulk refresh through:
  - `refresh-template-governance`

### 2. Template governance has real coverage

The first live refresh now governs:

- `refreshed_entries = 13`
- `skipped_entries = 0`

That means governance is already covering real template bundles like:

- `families_static_web_dashboard`
- `wrappers_chattycog`

So this is not just a contract skeleton.

It is operating against the live template tree.

### 3. Template governance is operationally surfaced

The UI now treats template governance as a parallel catalog, like family governance.

It has:

- refresh action
- freshness line
- stale warning
- launch auto-refresh with cooldown
- governed/stable/changed/regressed counts
- template-bundle picker
- receipt access
- template-root access

That is the correct shape because template bundles are reusable catalog assets, not pending lanes.

## Why This Matters

This checkpoint is the first one where governance covers the full host-owned deterministic stack from top to bottom:

- proof trust
- lane trust
- family capability trust
- template trust

That is a stronger architectural statement than the earlier waves:

- the factory is no longer only learning how to execute and extend its machinery
- it is learning how to curate the machinery, the contracts, and the product assets together over time

## What The Shared Pattern Now Proves

The shared governance model in [Governance Model](../plans/GOVERNANCE_MODEL.md) is no longer theoretical.

It now maps cleanly across all current governed substrates:

- current artifact snapshot
- validation
- hashing
- drift vs seed/reference
- baseline vs last known-good state
- persisted receipts
- persisted refresh status
- visible freshness and risk posture

That is now a repo-proven pattern, not just a design ambition.

## Remaining Question

The next question is no longer:

- can another important catalog be governed?

We already answered that repeatedly.

The next question is:

- which remaining coupled surfaces deserve independent governance, and which should stay governed indirectly through stronger parent substrates?

The strongest candidate still left in that category is:

- acceptance recipes

But the important nuance is that acceptance recipes are not as clearly independent as templates were.

They are currently governed indirectly through patch governance and composition bundle governance.

So the next move should be chosen deliberately, not automatically.

## Best Next Move

The strongest next move is not necessarily another immediate implementation wave.

The best next move is a small decision checkpoint on:

- whether acceptance recipes deserve independent governance
- or whether the better payoff is consolidating and simplifying the current seven-substrate governance system

That should happen before creating another full governance milestone.

## Short Read

Checkpoint 11 says:

- governance now spans proofs, composition bundles, patch recipes, helper lanes, bridge lanes, family manifests, and template bundles
- template governance is real, refreshed, and UI-surfaced

The next decision is no longer "what obvious unguided catalog is left?"

It is "which remaining coupled surfaces are worth governing independently, and which should stay governed through their parent substrates?"
