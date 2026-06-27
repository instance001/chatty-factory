# Bounded Soft-Review Continuation

This note defines one narrow policy in the current ChattyFactory runtime:

- clarification pressure is not always an automatic stop
- the host may continue when it can still freeze an honest bounded attempt
- unresolved ambiguity must remain explicit in receipts and route notes

## Why This Exists

Without this policy, the host can drift into a softer version of the old
"positive lanes define reality" trap:

- request does not cleanly map to a proven lane
- uncertainty appears in review
- host immediately stops even though a bounded attempt is still honest

The intended behavior is:

- do not fake certainty
- do not overstate support
- do not stop if a bounded attempt is still truthful and reviewable

## Build-Side Rule

For new builds, the host may continue under soft review when all of these are
true:

1. a concrete substrate or execution surface is still evidenced
2. a tool kind or capability posture is still evidenced
3. the remaining blockers are soft ambiguity or capability-gap reasons
4. the host can still freeze a real bounded build attempt

Examples of soft reasons:

- `surface_unclear`
- explicit stack not yet proven, when a bounded substrate attempt is still honest
- helper or backend capability gaps, when the current attempt can still be frozen honestly

Examples of hard blockers that should still stop:

- no bounded substrate evidence at all
- no credible tool kind at all
- hard host-contract conflicts
- bounded composition review denial

## Patch-Side Rule

For follow-up patch requests, the host may continue under soft review when all
of these are true:

1. the project is already diagnosed well enough to support governed patching
2. the project already advertises patchable structure or bounded patch posture
3. the follow-up request did not cleanly match a named patch lane
4. the host can still prepare a bounded patch attempt honestly

This is intentionally narrower than the build-side rule.

The idea is to treat:

- "no named patch lane matched yet"

as:

- "continue through a bounded patch attempt if the evidence still supports one"

instead of:

- "blocked until the host invents a new positive lane"

## What Must Be Recorded

Soft-review continuation is only acceptable when the uncertainty is preserved as
evidence.

The host should record that decision in:

- `route_notes`
- build or patch plan artifacts
- execution and verification receipts
- later next-attempt, decomposition, or triangulation artifacts if the attempt still fails

The UI should show this as:

- a bounded continuation decision
- not a clean certainty story that hides the ambiguity

## Relationship To The Negative Shelf

This policy does not weaken the negative-lane architecture.

It supports it.

The host should not prematurely convert:

- "uncertain but still bounded"

into:

- "blocked forever until someone adds one more positive lane"

## Operational Summary

The short version is:

- hard conflicts still stop
- honest bounded attempts may continue
- uncertainty must stay visible
- failures from those attempts become evidence for decomposition,
  triangulation, or later constraint promotion
