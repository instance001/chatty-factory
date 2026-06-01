# Architecture Checkpoint 6

This checkpoint comes after the proof harness stopped being only:

- executable
- extensible
- lifecycle-managed

and started becoming **governed infrastructure**.

## What Is Now Real

- Proof-harness entries now carry proof-quality state in the pending registry.
- Proof-harness entries now carry host-owned lineage and drift state:
  - seed template id
  - seed bundle id
  - drift status vs seed
  - drift notes
- The host now persists proof lineage receipts under:
  - `runtime/proof_lineage_receipts/`
- Lineage receipts now also track:
  - last passing paired-proof receipt
  - last passing manifest hashes
  - change status vs last passing baseline
  - change notes vs last pass
- The host now supports bulk proof-governance refresh through:
  - `refresh-proof-harness-registry`
- The UI now surfaces proof governance as a first-class operational view:
  - proof quality badges
  - baseline badges
  - proof-quality filters
  - proof-baseline filters
  - proof-risk sort
  - top-level regression summary
  - proof-governance refresh freshness
  - stale-data warnings
  - newer-proof-receipt warnings
  - launch auto-refresh with cooldown

That means the proof harness is no longer just a place where proofs can run.

It is now a place where proofs can be:

- seeded
- scaffolded
- promoted
- validated
- compared
- refreshed
- reviewed for drift
- watched for regression

## Why This Matters

This is a meaningful shift in architectural maturity.

Earlier checkpoints proved that the host could:

- execute bounded proof work
- generalize proof templates
- load repo-defined proof manifests
- lifecycle-manage proof bundles

Checkpoint 6 proves something different:

- the host can now **govern proof assets over time**

That is much closer to the original design intent than a one-shot proof runner.

It means the factory is learning not only how to produce deterministic machinery, but how to keep that machinery honest after it exists.

## What Improved Since Checkpoint 5

Checkpoint 5 established that proof manifests were repo-extensible and lifecycle-managed.

Since then, we added four important layers:

1. Proof quality gates
- proof manifests are now classified as:
  - `needs_contract_fix`
  - `catalog_unresolved`
  - `catalog_resolved`
  - `runnable_diverged`
  - `passing`

2. Drift governance
- proof manifests are now classified against their seeds:
  - `seed_aligned`
  - `lightly_customized`
  - `structurally_customized`
  - `drifted_risky`
  - `unseeded`

3. Baseline governance
- proof manifests are now classified against their last passing run:
  - `stable_since_last_pass`
  - `changed_since_last_pass`
  - `regressed_since_last_pass`
  - `baseline_recorded`
  - `no_passing_baseline`

4. Operational visibility
- the UI can now answer:
  - which proofs are regressed
  - which proofs changed since last pass
  - which proofs have no baseline
  - whether the governance picture is stale
  - whether newer proof receipts exist than the current governance snapshot

Those four layers are what transform proof manifests from “live repo content” into “maintained factory assets.”

## What Is Still Missing

The proof-governance path is strong now, but it is still localized.

The biggest remaining gap is:

- this governance pattern mostly exists for `proof_harness_bundle`

Other deterministic machinery still does not have comparable host-owned change governance.

Examples:

- patch recipes can be pending/live, but they do not yet carry seed lineage or drift vs last known-good state
- composition bundles can be scaffolded/promoted/live, but they do not yet carry baseline regression signals
- helper lanes are increasingly structured, but their lifecycle still lacks the same curation layer that proofs now have

So the host can now curate proofs well, but not yet the broader deterministic lane ecosystem at the same level.

## Best Next Move

The strongest next move is:

**generalized deterministic lane governance**

That likely means reusing the proof-governance pattern for at least one more subsystem, probably in this order:

1. `composition_bundle`
- because it already has mixed-layer lifecycle data
- and it is the closest non-proof analog to governed bounded work

2. patch recipes
- especially ones promoted from scaffold/fallback paths

3. helper lanes
- where runtime behavior and acceptance drift matter over time

The goal would be to give those subsystems proof-like governance primitives:

- seed/source lineage
- drift vs seed
- change vs last known-good baseline
- bulk refresh/backfill
- UI badges, filters, and risk ordering

## Recommended Order

1. Extract the common governance pattern from proof-harness entries.
2. Apply it first to `composition_bundle`.
3. Add registry/UI risk badges and refresh for mixed deterministic bundles.
4. Only then decide whether patch and helper lanes should share the same generalized contract directly or via family-specific wrappers.

## Short Read

Checkpoint 6 says:

- the proof harness is no longer only executable and extensible
- it is now governed over time

That is a meaningful architectural threshold.

The next move is not more proof-template count.

The next move is to carry this same governance discipline into another deterministic subsystem so the factory learns how to curate more than just proofs.
