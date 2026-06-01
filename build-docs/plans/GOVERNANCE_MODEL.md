# Governance Model

This doc captures the shared governance pattern now used across the rebuild.

It does not try to pretend every governed substrate is identical.
It defines the common shape that keeps the system coherent while leaving room for layer-specific policy.

## Governed Substrates

The current governed substrate set is:

1. proof harness bundles
2. composition bundles
3. patch recipes
4. helper lanes
5. bridge lanes
6. family manifests

Those substrates are governed because they are:

- deterministic
- reusable
- influential in routing, execution, or validation
- capable of drifting over time

## Shared Governance Loop

Every governed substrate follows the same host-owned loop:

1. identify the current artifact set
2. validate the governed artifact shape
3. hash the current artifact set
4. compare against a reference baseline or seed
5. compare against the last known-good live or passing baseline
6. persist a governance receipt
7. persist refresh status for bulk governance scans
8. surface the result in the UI with freshness and risk cues

That loop is the real shared contract, even when the concrete files differ.

## Shared Governance Concepts

### 1. Current artifact snapshot

Each governed substrate identifies the files or manifests that define its current state.

Examples:

- proof bundle:
  - proof template manifest
  - comparison bundle manifest
- composition bundle:
  - integrated family/patch/helper composition files
- family governance:
  - family manifest

### 2. Reference seed or archetype

Some governed substrates have an explicit seed model.

Examples:

- proof bundles can be scaffolded from a suggested seed template and bundle
- composition bundles can be evaluated against their scaffolded or integrated starter shape
- family manifests are often compared against a narrower capability archetype

This supports drift classification against intended origin, not just current validity.

### 3. Last known-good baseline

Each governed substrate tracks its most recent trusted good state.

Examples:

- proof bundles:
  - last passing paired proof
- composition / patch / helper / bridge / family:
  - last known-good live baseline

This supports change governance over time, not just one-time provenance.

### 4. Drift classification

Drift answers:

- how far did this asset move from its seed or reference shape?

Current drift labels vary slightly by substrate, but usually include patterns like:

- `seed_aligned`
- `lightly_customized`
- `structurally_customized`
- `drifted_risky`
- `unseeded`

### 5. Baseline change classification

Baseline change answers:

- how does the current asset compare to the last trusted good state?

Typical labels include:

- `baseline_recorded`
- `stable_since_last_live`
- `changed_since_last_live`
- `regressed_since_last_live`
- `no_live_baseline`

Proof governance uses the proof-specific equivalent:

- `stable_since_last_pass`
- `changed_since_last_pass`
- `regressed_since_last_pass`
- `no_passing_baseline`

### 6. Bulk refresh and backfill

Every governed subsystem has a bulk refresh path that:

- recalculates governance receipts
- backfills missing seed or baseline metadata
- updates refresh status files

This prevents governance from becoming a manual per-entry maintenance burden.

### 7. UI freshness and risk posture

Every governed subsystem should expose:

- refresh action
- refresh freshness line
- stale warning
- optional launch auto-refresh
- cooldown-aware messaging
- at least one visible risk or baseline signal

The exact UI form can differ:

- extension-backed substrates often use row badges and filters
- family governance uses a parallel catalog panel

## What Is Universal

These parts should be treated as the universal governance model:

- governed artifact identification
- artifact validation
- artifact hashing
- drift vs seed/reference
- change vs last known-good baseline
- receipt persistence
- refresh status persistence
- freshness and risk surfacing

If a future subsystem cannot support most of those steps, it probably should not yet be treated as a governed substrate.

## What Remains Layer-Specific

These parts are intentionally substrate-specific:

- what counts as the governed artifact set
- what "good" means
- what the seed or archetype is
- what the last known-good baseline means
- which labels are most natural for the substrate
- whether UI surfacing belongs in extension rows or a parallel catalog

That layer-specific policy is a feature, not a flaw.
The goal is shared governance shape, not forced sameness.

## Current Host Pattern

Across the rebuild, the host now tends to provide for each governed substrate:

- a receipt type
- a refresh status type
- artifact validation helpers
- artifact hashing helpers
- drift classification helpers
- baseline classification helpers
- a bulk refresh command
- runtime receipt storage
- UI loading and surfacing

That has become a real architectural scaffold.

## Why This Model Matters

Without this model, the rebuild risks becoming:

- good at generating assets
- but weak at maintaining them

With this model, the rebuild can:

- scaffold assets
- promote them
- wire them live
- compare them to intended origin
- compare them to last trusted good state
- warn when curation falls behind reality

That is a meaningful step toward a factory that governs its own machinery instead of only producing it.

## Practical Reading

When adding governance to a new subsystem, the implementation order should usually be:

1. define the governed artifact set
2. define receipt and refresh-status persistence
3. add artifact validation
4. add hashing
5. add drift classification
6. add baseline classification
7. add bulk refresh/backfill
8. add UI freshness and risk surfacing
9. only then decide whether broader unification is worth it

## Short Read

The governance model is:

- current snapshot
- seed/reference comparison
- last-good baseline comparison
- persisted receipts
- persisted refresh status
- visible freshness and risk posture

That is now the shared curation scaffold across the rebuild.
