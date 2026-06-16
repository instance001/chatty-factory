# Bounded Soft-Review Continuation

This note defines one narrow policy in the current ChattyFactory rebuild:

- clarification or planner-review pressure is not always an automatic stop
- the host may continue when it can still freeze an honest bounded attempt
- unresolved ambiguity must be carried forward explicitly in receipts and route notes

## Why This Exists

Without this policy, the factory can drift back into a softer version of the old
"supported lanes only" trap:

- request does not cleanly match a named lane
- `needs_llm_review` stays true
- host immediately falls back to clarification, stubs, or extension paperwork

That keeps the system safe, but it also blocks honest substrate attempts that the
host already knows how to emit.

The intended behavior is:

- do not fake certainty
- do not overstate support
- do not stop if a bounded substrate attempt is still honest

## Build-Side Rule

For new builds, the host may continue under soft review when all of these are true:

1. a family candidate still exists
2. a tool kind still exists
3. the remaining blockers are soft ambiguity or capability-gap reasons
4. the host can still emit a real starter substrate

Examples of soft reasons:

- `surface_unclear`
- unsupported explicit stack, when a starter substrate is still honest
- helper/backend capability gaps, when a starter substrate is still honest

Examples of hard blockers that should still stop:

- no family candidates at all
- no tool kind at all
- hard host-contract conflicts
- bounded composition review denial

## Patch-Side Rule

For follow-up patch requests, the host may continue under soft review when all of
these are true:

1. the project is already grounded to a known family
2. the project already advertises supported patch lanes
3. the follow-up request did not cleanly match a named patch lane
4. the host can still prepare a substrate-first patch attempt

This is intentionally narrower than the build-side rule.

It exists so the factory can treat:

- "no named patch lane matched yet"

as:

- "continue through a bounded substrate-first patch attempt"

instead of automatically treating it as:

- "unsupported until a new lane is hand-authored"

## What Must Be Recorded

Soft-review continuation is only acceptable when the uncertainty is preserved as
evidence.

The host should record that decision in:

- `route_notes`
- build or patch plan artifacts
- task execution and verification receipts
- any later fallback or triangulation artifacts if the bounded attempt still fails

The UI should show this as:

- a bounded continuation decision
- not as a clean named-lane success that never had ambiguity

## Relationship To The Negative Bookshelf

This policy does not weaken the negative constraint architecture.

It supports it.

The bookshelf should decide:

- what truly cannot proceed

The host should not prematurely convert:

- "uncertain but still scaffoldable"

into:

- "blocked forever until someone adds one more positive lane"

## Operational Summary

The short version is:

- hard conflicts still stop
- honest bounded substrates may continue
- uncertainty must stay visible
- failures from those bounded attempts become evidence for decomposition,
  triangulation, or later constraint promotion
