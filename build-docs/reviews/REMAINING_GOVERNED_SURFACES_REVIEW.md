# Remaining Governed Surfaces Review

This review comes after the governance pattern reached:

- proof harness bundles
- composition bundles
- patch recipes
- helper lanes
- bridge lanes
- family manifests

The goal here is not to create more governance work for its own sake.

The goal is to identify which remaining repo-extensible surfaces are:

- still strategically unguided
- already governed indirectly through another substrate
- or not important enough to treat as governed infrastructure yet

## Current Read

The biggest remaining unguided catalog is:

- **templates**

Everything else is either:

- already governed directly
- governed indirectly through a stronger substrate
- or still too lightweight / too design-oriented to justify full governance

## Already Governed Directly

These surfaces already have a real governance loop:

1. `proof_harness/`
- proof template manifests
- comparison bundle manifests

2. `operator_registry/pending_lanes.json`
- proof-harness bundles
- composition bundles
- patch recipes
- helper lanes
- bridge lanes

3. `families/manifests/`
- family capability manifests

These are the core curated deterministic catalogs now.

## Governed Indirectly Enough For Now

These surfaces matter, but they are already covered strongly enough through another governed parent.

### 1. `operator_registry/acceptance_recipes/`

Why not separate yet:

- acceptance recipes currently travel with patch governance
- patch governance already hashes and validates the paired acceptance artifact
- composition bundles also pull them in through the patch-side portion of mixed bundles

Current recommendation:

- keep acceptance governance coupled to patch governance
- only split it into its own governed substrate if acceptance recipes become independently reusable and independently promoted

### 2. `operator_registry/helper_lanes/`

Why not separate from helper governance:

- this is already the governed helper substrate

No extra action needed.

### 3. `operator_registry/bridge_lanes/`

Why not separate from bridge governance:

- this is already the governed bridge substrate

No extra action needed.

### 4. family-adjacent starter docs and integrated family markdown files

Examples:

- `families/*.md`
- scaffolded family starter notes

Why not separate yet:

- the meaningful governed artifact is the family manifest
- the markdown is descriptive or transitional, not the authoritative executable contract

Current recommendation:

- keep the manifest as the governed source of truth

## Strongest Remaining Ungoverned Catalog

## `templates/`

This is the clearest next candidate.

Why it matters:

- templates are versioned product assets
- the runtime loader treats them as real build machinery
- they shape generated projects directly
- they can drift independently from family manifests and lane metadata

The templates tree already has meaningful reusable catalogs:

1. `templates/families/`
- family build templates
- family-starter and promoted family-specific template variants

2. `templates/patches/`
- patch-lane template surfaces
- composition-bundle-adjacent template starters

3. `templates/helpers/`
- helper-lane starter templates

4. `templates/wrappers/chattycog/`
- wrapper/runtime bridge template assets
- handshake, manifest, visual-load, bridge JS

Why this is the strongest next step:

- templates are primary product assets, not just metadata
- they sit beneath many already governed catalogs
- drift here can quietly invalidate otherwise healthy manifests and lanes

So if we want one more major governance wave, **template governance** is the best next move.

## What Template Governance Would Mean

The shared pattern from [Governance Model](../plans/GOVERNANCE_MODEL.md) would apply cleanly:

1. define governed template units
2. validate template artifact sets
3. hash template files
4. compare against seed/reference or last known-good baseline
5. persist template governance receipts
6. bulk refresh/backfill
7. UI freshness and risk surfacing

Likely governed template families:

- family template bundles
- patch template bundles
- helper template bundles
- wrapper template bundles

## Plausible Next-But-Not-Yet

These are real surfaces, but not the best immediate next governance target.

### 1. `proof_harness/rust_registry_promotion/`

Why not yet:

- this is promotion support material, not the canonical proof contract
- the proof template and comparison bundle are already governed

### 2. `extensions/`

Why not yet:

- extension work bundles are implementation scaffolds and lifecycle artifacts
- the governed asset should usually be the promoted deterministic substrate, not the temporary work bundle

### 3. `output/`

Why not yet:

- generated projects are product output, not reusable deterministic catalog assets
- they should stay acceptance-verified, not governance-managed as shared infrastructure

### 4. `examples/`

Why not yet:

- examples are reference/demo material
- not currently part of the live host-owned deterministic belt

## Not Worth Governing Yet

These should stay lightweight for now.

### 1. `schemas/`

Current state:

- only a README is present
- there is no active schema catalog yet

Recommendation:

- do not create governance around an unrealized schema subsystem

### 2. `contracts/`

Current state:

- design visibility only
- not a host-managed reusable catalog

Recommendation:

- keep as docs until a concrete machine-consumed contract catalog exists

### 3. `operator_registry/README.md`

Why not:

- explanatory documentation only

## Recommended Order

If we keep pushing governance outward, the clean order is:

1. template governance
2. only after that, reconsider whether acceptance recipes deserve independent governance
3. only after that, revisit schemas/contracts if they become real machine-managed catalogs

## Short Read

The review says:

- most strategically important deterministic catalogs are now governed
- acceptance recipes are governed indirectly enough through patch governance for now
- the strongest remaining unguided product catalog is `templates/`

So the best next governance wave is:

- **template governance**
