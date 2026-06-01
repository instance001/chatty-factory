# Architecture Checkpoint 4

This checkpoint comes after the proof harness moved one step further from “generic runner with nicer receipts” toward a genuinely declarative proof substrate.

## What Is Now Real

- The harness now has at least four executable built-in proof templates:
  - `proof_helper_monitoring`
  - `proof_summary_reporting`
  - `proof_status_reporting`
  - `proof_filtered_reporting`
- The fourth template was added without a new host proof runner branch.
- Per-family proof request wording now lives in template metadata.
- Per-family proof enrichment now lives in template metadata.
- Comparison receipt naming and note phrasing now live in comparison-bundle policy metadata.
- The proof UI picked up the fourth template automatically because it already reads from the template catalog rather than a hardcoded list.

That means the current harness is no longer merely:
- one or two proof flows behind a generic command

It is now meaningfully:
- catalog-driven proof orchestration
- catalog-driven family request shaping
- catalog-driven enrichment
- catalog-driven comparison policy

## Why This Matters

This is one of the stronger confirmations yet that the rebuild is tracking the original design intent.

We are now seeing the desired pattern:
- the host owns execution
- proof contracts describe the bounded work
- the model can stay beside the belt
- new proof shapes become cheaper because the machinery is more data-shaped than code-shaped

That is a healthier trajectory than growing a large proof-specific host switchboard.

## What Improved Since Checkpoint 3

Checkpoint 3 said the harness was real but still too code-shaped in the host.

Since then, we moved three important proof concerns out of bespoke host logic:

1. Family request shaping
- no longer primarily handwritten host helper behavior
- now expressed in `family_request_bindings`

2. Enrichment behavior
- no longer primarily a proof-kind switch in the host
- now expressed in `enrichment_bindings`

3. Comparison presentation policy
- no longer passed down as ad hoc labels/prefixes from the runner
- now expressed in bundle policy metadata

These are exactly the kinds of shifts that make a harness cheaper to extend honestly.

## What Is Still Not Fully Declarative

Even with four templates, a few host-shaped residues remain:

- the core proof catalog is still code-native in `proof_harness.rs`, not yet repo-extensible through manifests or the extension lifecycle
- request generation still has a host fallback layer for templates that omit bindings
- enrichment still has a host fallback layer for templates that omit bindings
- the comparison receipt struct is still named `CrossFamilyMonitoringComparisonReceipt`, which no longer reflects the broader proof surface
- the legacy helper command `run_cross_family_helper_monitoring_proof` still exists as a compatibility wrapper over the generic runner

These are not blockers, but they are the main places where the proof harness still leaks earlier milestone history.

## Best Next Move

The best next move is not “add ten more templates.”

The best next move is:

**make the proof catalog and receipts more substrate-neutral**

That likely means:
- rename the comparison receipt type to something proof-general
- consider moving proof-template and comparison-bundle definitions toward a manifest or extension-backed catalog layer
- decide whether proof templates should eventually participate in the same pending-lane / promotion lifecycle as other deterministic machinery

## Recommended Order

1. Normalize the remaining proof-general naming and receipt contracts.
2. Decide whether proof templates stay code-catalog-only or become repo-extensible artifacts.
3. Only then push into broader template expansion or proof-family growth.

## Short Read

Checkpoint 4 says:

- the harness now survives a fourth template without new host proof branches
- the UI is already catalog-driven enough to pick that up automatically
- the proof system is much less code-shaped than it was at Checkpoint 3

So the next move is no longer “prove the harness can generalize.”

It just did.

The next move is deciding how far we want to take that generalization:
- code-catalog only
- or truly repo-extensible proof machinery
