# Architecture Checkpoint 5

This checkpoint comes after the proof harness crossed an important boundary:

- proof templates and comparison bundles are no longer only code-catalog entries
- the host and UI can now load repo-backed proof manifests
- a manifest-defined proof template has been executed successfully through the same generic proof runner

## What Is Now Real

- Built-in proof templates still exist in `crates/chatty_factory_core/src/proof_harness.rs`.
- Repo-backed proof templates now load from:
  - `proof_harness/templates/*.json`
- Repo-backed comparison bundles now load from:
  - `proof_harness/bundles/*.json`
- Host proof execution now resolves templates and bundles from the workspace-aware catalog, not only the built-in catalog.
- The UI proof picker and comparison-bundle summary now also resolve from the workspace-aware catalog.
- The latest manifest-defined proof:
  - `proof_manifest_filtered_reporting`
  - executed successfully through `run-proof-template`
  - produced a passing paired-proof receipt and comparison receipt

That means the proof harness is no longer merely “declarative inside code.”
It is now beginning to act like repo-extensible factory machinery.

## Why This Matters

This is a meaningful step toward the original design intent.

It means the factory can start treating proof work more like other host-owned capabilities:
- the host executes it
- contracts define it
- repo-native artifacts extend it
- the model can supervise bounded choices around it instead of inventing bespoke proof logic on demand

This is closer to a real conveyor-belt architecture than a clever proof demo baked into one crate.

## What Got Better Since Checkpoint 4

Checkpoint 4 concluded that the next question was whether proof templates would remain code-catalog-only or become repo-extensible.

We now have an answer:

- they are beginning to become repo-extensible

Specifically:

1. Proof catalog loading
- now merges built-ins with repo manifests

2. Proof receipt readability
- paired-proof receipts now persist:
  - `proof_template_display_label`
  - `capability_comparison_bundle_id`
- which keeps manifest-defined proofs readable in the UI/history path

3. UI proof surfacing
- manifest-defined proofs now appear in the same proof picker/history system without bespoke UI additions

## What Is Still Incomplete

This is a real step, but not the end state yet.

Remaining gaps:
- proof manifests are repo-extensible, but not yet integrated into the pending-extension / promotion lifecycle
- built-in and external proof templates currently merge by id, but there is not yet a dedicated proof-registry validation flow
- “Open Template Contract” and “Open Comparison Bundle Contract” now support manifest paths, but the broader authoring workflow is still minimal
- the compatibility command `run-cross-family-helper-monitoring-proof` still exists as a legacy wrapper over the generic runner

So proof machinery is repo-extensible now, but not yet fully lifecycle-managed the way patch/family/helper lanes are.

## Best Next Move

The next strongest move is:

**proof template lifecycle integration**

That would likely mean:
- define validation rules for proof manifests and bundle manifests
- decide whether proof templates belong in the same scaffold/pending/promotion flow as other deterministic machinery
- make proof catalog changes inspectable and safe in the same host-owned way as other extension work

## Short Read

Checkpoint 5 says:

- the proof harness is not just broader and more declarative now
- it is also starting to become repo-extensible

That is a major architectural milestone.

The next move is not proving that repo-backed proof manifests can run.

They already can.

The next move is deciding how those proof manifests should participate in the broader factory lifecycle.
