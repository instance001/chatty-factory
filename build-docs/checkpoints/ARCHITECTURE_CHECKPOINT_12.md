# Architecture Checkpoint 12

This checkpoint comes after two things became true:

1. template governance reached freshness-policy parity
2. acceptance governance was reviewed as a deliberate decision rather than assumed as the next automatic wave

That matters because the architecture is no longer just expanding governance outward.

It is starting to decide where governance boundaries should stop.

## What Is Now Real

The rebuild now has operational governance across seven substrate classes:

1. proof harness bundles
2. composition bundles
3. patch recipes
4. helper lanes
5. bridge lanes
6. family manifests
7. template bundles

And every one of those now has the full operational loop:

- receipts
- refresh status
- freshness visibility
- stale warning
- optional launch auto-refresh
- cooldown-aware manual bypass

That means governance is no longer only structurally present.

It is operationally complete across the current substrate set.

## What Improved Since Checkpoint 11

Checkpoint 11 established that template governance was the last obvious unguided catalog.

Since then:

### 1. Template governance reached freshness parity

Template governance now has:

- refresh status under:
  - `runtime/template_governance_refresh_status.json`
- a dedicated UI catalog
- stale warning
- launch auto-refresh with cooldown
- manual refresh bypass

That puts templates on the same operational footing as family governance and the governed extension-backed substrates.

### 2. Governance boundaries are now being chosen, not just expanded

The acceptance review at:

- [Acceptance Governance Decision Review](../reviews/ACCEPTANCE_GOVERNANCE_DECISION_REVIEW.md)

established an important architectural rule:

- not every durable artifact deserves its own independent governance subsystem immediately

Acceptance recipes are important, but they are still better governed indirectly through:

- patch governance
- composition governance

for now.

That is the first clear boundary-setting decision in the governance era.

## Why This Matters

This checkpoint is the first one where the governance program feels mature enough to:

- expand where necessary
- stop where ownership boundaries are still coupled

That is a healthier architecture than a simple rule of:

- "if it exists and lasts, govern it separately"

The rebuild is now choosing governance boundaries based on:

- lifecycle ownership
- reuse shape
- promotion shape
- operational risk

instead of only on durability.

## Current Best Read

The seven-substrate governance scaffold now looks stable.

The next improvements are no longer about finding another obvious catalog.

They are about:

- making coupled contracts easier to read inside the parent governance loop
- tightening shared governance ergonomics
- only splitting out new subsystems if the ownership boundary becomes genuinely independent

That is a more mature phase of the rebuild.

## Best Next Move

The strongest immediate move is:

- improve visibility of acceptance coupling inside patch and composition governance

That is better than starting a standalone acceptance-governance subsystem because it:

- respects the current ownership boundary
- improves operator clarity
- avoids duplicating the governance loop too early

## Short Read

Checkpoint 12 says:

- the seven-substrate governance scaffold is now operationally complete
- the architecture has started making deliberate boundary decisions, not just adding more governed surfaces

The next best work is governance clarity inside coupled parent substrates, not automatic subsystem proliferation.
