# Architecture Checkpoint 30

The negative constraint shelf has crossed from a passive receipt archive into a
real operational subsystem.

At this checkpoint, the shelf is no longer just:

- a place where proposed rules land after failures

It is now a managed loop that can:

- capture proposed constraints from build or reverification failures
- approve those proposals into a persisted shelf
- consult approved rules during later build verification
- surface approved rules in the UI
- activate or deactivate rules deliberately
- audit shelf mutations
- track which rules are actually matching later failures
- rank rules by recent reuse rather than simple insertion order

What changed in this wave:

- build verification receipts now record approved shelf matches
- proposed constraints can be approved into:
  - `runtime/approved_constraint_shelf.json`
- approved constraints can be:
  - activated
  - deactivated
- every shelf mutation now emits an audit receipt under:
  - `runtime/constraint_shelf_mutations`
- the desktop UI now has a `Negative Constraint Shelf` panel that shows:
  - approved rule count
  - active rule count
  - recent match events
  - recent shelf matches
  - per-rule recent match counts
  - per-rule reuse breakdown by failure mode, family, and tool

Why this matters:

- the factory is no longer only collecting lessons from failures
- it is starting to manage those lessons as a reusable reliability layer
- this is the first real step from:
  - `we observed a failure`
- toward:
  - `we have an approved negative rule, and we can see whether it is paying for
    itself`

Architecturally, this is important because the shelf is becoming the right kind
of scaling surface for broader code and language coverage:

- not an endlessly expanding positive lane catalog
- but a host-owned negative funnel of:
  - forbidden methods
  - stale shapes
  - known bad implementation paths

The next healthy move from here is to keep improving shelf interpretation, not
just shelf accumulation. In practical terms that means:

- surfacing unmatched approved rules
- understanding which rules are carrying real load
- and eventually letting verified failures propose new constraints in a way that
  stays specific, auditable, and reviewable
