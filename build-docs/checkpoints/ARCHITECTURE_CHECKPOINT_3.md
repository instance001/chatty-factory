# Architecture Checkpoint 3

This checkpoint comes after the proof harness moved beyond one special paired proof and now supports multiple executable proof templates through the same host-owned runner.

## What Is Now Real

- Proof execution is template-driven rather than monitoring-only.
- The host can run multiple cross-family proof templates from the same `run_proof_template(...)` entrypoint.
- The harness now has at least three executable built-in templates:
  - `proof_helper_monitoring`
  - `proof_summary_reporting`
  - `proof_status_reporting`
- Capability equivalence is being judged through explicit comparison bundles instead of ad hoc proof-specific logic.
- The UI can:
  - choose a proof template
  - inspect its template and bundle contract
  - filter proof history by template
  - persist proof preferences and named proof profiles

That means the rebuild is no longer just proving one special cross-family milestone. It is starting to look like a reusable proof substrate.

## Why This Matters

This is an important architectural step toward the original design intent.

We now have:
- host-owned proof orchestration
- host-owned proof receipts
- primitive-oriented capability comparison
- multiple proof shapes over the same machinery

So the factory is becoming better at proving that two different families can satisfy the same intent class without demanding identical implementation details.

That is much closer to:
- the host owning the belt
- the model supervising bounded choices beside the belt
- capability generalization being explicit and inspectable

## What Is Still Not General Enough

Even with three templates, the host still has too much template-specific behavior in the execution layer.

Current drift points:
- family-specific request wording is still built with handwritten helpers
- proof enrichment is still template-family specific
- comparison receipts still reuse a monitoring-shaped receipt struct name
- proof templates are cataloged, but the host still knows too much about which template needs which execution path

So the harness is reusable in practice, but not yet fully declarative.

## Best Next Move

The next architectural target should be:

**declarative proof execution plans**

That means moving more of this out of host `match` branches and into proof-template metadata:
- per-family request shaping rules
- optional enrichment rules
- comparison receipt kind/label
- preferred execution family pair semantics

The ideal next step is not ten more templates.
It is making the next templates cheaper because the host needs less bespoke logic per template.

## Recommended Order

1. Generalize proof-template execution metadata.
2. Reduce template-specific host branching.
3. Add one more proof cluster only after the metadata path is stronger.
4. Then revisit whether proof templates should become partially repo-extensible through the same extension lifecycle.

## Short Read

Checkpoint 3 says:

- the proof harness is real
- the proof harness is broader
- but the proof harness is still too code-shaped in the host

So the next move is not more panel polish and not blind template expansion.
The next move is making proof execution more declarative.
