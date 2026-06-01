# Next Pivot Recommendation

This note answers the question:

What should we do next now that the UI extraction wave has reached a healthy stop point?

## Recommendation

Do not keep shrinking `chatty_factory_ui` by default.

Pivot to a smaller, higher-signal cleanup and architecture pass instead.

## Why

The current UI split already covers the strongest subsystem boundaries:

- governance surface
- proof run surface
- proof history surface
- request/action surface
- runtime/registry dashboard

The next remaining extractions are weaker and carry more risk of fragmentation.

That means the highest-value next work is no longer more module carving. It is tightening signal and reducing ambiguity around the surfaces we already extracted.

## Best Next Investment Order

### 1. Warning Cleanup

Clean up the remaining known warnings:

- unused helper in `chatty_factory_families`
- unused fields in `CapabilityComparisonReceiptSummary`

Why first:

- low risk
- improves signal
- makes later warnings more trustworthy

### 2. UI Surface Consolidation Review

Do a light pass over the extracted UI modules and ask:

- which ones are stable
- which ones are still too broad
- which ones should stay exactly as they are

This is not another extraction wave. It is a sanity pass.

### 3. Return To Product / Architecture Work

After the warning cleanup, the strongest likely non-UI pivots are:

- governance behavior improvements
- proof-harness operational polish
- deterministic lane lifecycle improvements
- primitive-native execution / family-surface capability work

Those produce more architectural value now than another round of UI splitting.

## What Not To Do Next

Avoid these as the immediate next step:

- extracting tiny row renderers
- splitting one-off widgets into files
- continuing the UI wave just because `main.rs` can still be smaller

## Bottom Line

The right next move is:

1. warning cleanup
2. very light consolidation sanity pass
3. pivot back to product / host architecture work

That keeps momentum without crossing into decorative refactoring.
