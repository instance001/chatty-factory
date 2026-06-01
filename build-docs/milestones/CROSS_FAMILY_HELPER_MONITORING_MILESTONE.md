# Cross-Family Helper Monitoring Milestone

## Why This Is Next

The rebuild now has a strong primitive and composition spine:
- bounded composed builds
- bounded composed patches
- helper primitives and helper primitive kinds
- patch primitive classes
- family build primitive classes
- primitive-native execution plans
- adapter-aware family and in-family selection
- helper-backed acceptance that is increasingly contract-shaped

That is real progress.

But the strongest proof is still clustered around one substrate at a time.
If we want to stay true to the original design intent, the next proof has to show that the same high-level machinery can be expressed across more than one family without collapsing back into family-specific planning language.

## Goal

Prove one helper-backed monitoring intent across two families from the same primitive-native work-order shape.

The design target is:
- same high-level user ask
- same primitive intent
- different family adapters
- host-owned execution and receipts
- equivalent capability outcome without requiring identical files

## Primary Proof Shape

Recommended shared request shape:

`build me a helper-backed monitoring surface that watches two inbox lanes, filters module assets to txt, and surfaces processed file status and preview`

Run that same intent across:
1. `chattycog_webview_module`
2. `static_web_dashboard`

The host should keep the primitive work-order language stable while family adapters translate it into the right substrate-specific execution path.

## Primitive Intent To Reuse

The proof should keep the same core primitive intent across both families:
- family build primitives:
  - `summary_surface`
  - `status_chip`
- patch primitives:
  - `summary_surface`
  - `status_chip`
  - `info_chip`
  - `preview_surface`
  - `notice_surface`
- helper primitives:
  - `inbox_lane`
  - `file_filter`
  - `processed_output`
  - `summary_emitter`
  - `status_reporter`

The exact concrete adapters can differ by family.
The primitive work-order should not.

## What Must Be Proven

At the end of this milestone, ChattyFactory should be able to:

1. freeze one helper-backed monitoring request into the same primitive-native plan shape for two families

2. route each family through declared family adapters rather than family-specific ad hoc planning

3. execute both builds from primitive-native work orders

4. persist receipts that clearly show:
   - family build primitive classes
   - patch primitive classes
   - helper primitive kinds
   - chosen family adapters
   - chosen in-family adapters
   - final execution status

5. verify both outputs with contract-shaped acceptance instead of broad marker-only checks

## Acceptance Direction

This milestone should not require identical output surfaces.

Instead, it should prove equivalent capability by checking things like:
- helper summary surface exists and is wired
- helper processed output bundle is valid
- helper preview surface exists when requested
- lane/filter rules are reflected in helper outputs
- observed lane coverage matches the intended helper bundle

In other words:
- same capability class
- different family rendering
- same host-owned evidence standard

## What To Add

### 1. Cross-Family Request Routing Proof

Ensure the host can interpret one monitoring request as a primitive-native work order before family-specific execution is chosen.

That means the route should journal:
- requested primitive layers
- requested primitive classes/kinds
- candidate families
- family adapter fit reasoning

### 2. Static Dashboard Monitoring Surface Proof

Bring `static_web_dashboard` up to parity for the selected monitoring intent.

It does not need to mirror ChattyCog exactly.
It does need to expose the same helper-backed capability class:
- helper summary
- helper processed output visibility
- helper preview or equivalent processed-file visibility

### 3. Cross-Family Surface Contracts

Add one higher-level surface contract per family for this monitoring proof.

Examples:
- `chattycog_helper_monitoring_surface_contract`
- `static_dashboard_helper_monitoring_surface_contract`

These should verify capability bundles, not scattered marker trivia.

### 4. Cross-Family Comparison Receipt

Add a small host-owned comparison artifact that records:
- shared primitive intent
- chosen family
- chosen adapters
- satisfied capability classes
- acceptance outcome

This does not need to compare files line-by-line.
It should compare primitive-native capability fulfillment.

## Suggested Implementation Order

1. Define the shared monitoring primitive intent bundle
2. Ensure both families declare adapter support for that bundle
3. Add or finish the missing static dashboard monitoring surface pieces
4. Add one family-level surface contract per family
5. Run the same high-level ask across both families
6. Persist a comparison receipt showing equivalent capability fulfillment

## What Not To Do

- Do not solve this by adding a completely different request for each family
- Do not require the two families to emit identical file structures
- Do not slip back into patch-kind-only reasoning for the proof
- Do not expand into many small UI lanes before the shared primitive proof is clear

## Success Criteria

This milestone is successful when:
- one shared helper-backed monitoring intent works across at least two families
- the primitive-native work-order story stays stable across both runs
- family-specific differences are expressed as adapter mappings
- acceptance is contract-shaped and capability-focused
- receipts make the cross-family equivalence legible

## Why This Matters

This is the clearest next proof that the rebuild is becoming the kind of factory we actually want:
- host-owned conveyor belt
- GGUF beside the belt
- primitive-native planning and review
- family adapters as implementation detail
- capability equivalence across substrates

That is a stronger sign of architectural maturity than adding another isolated lane.
