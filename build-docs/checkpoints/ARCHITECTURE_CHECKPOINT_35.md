# Architecture Checkpoint 35

The negative constraint shelf now has an explicit active-side lifecycle step,
not just labels about low-value rules.

At this checkpoint, active rules that are not matching anything can be moved
down the lifecycle deliberately instead of remaining live by inertia.

What changed:

- the shelf now supports bulk deactivation for:
  - active
  - currently unmatched
  rules
- that action is available through:
  - the host
  - the CLI
  - the desktop UI
- it emits the same audited mutation receipts as the rest of the shelf
  lifecycle

Why this matters:

- the factory is no longer only asking:
  - which rules are valuable?
- it can now act on that value signal

Architectural read:

- proposal created the shelf
- approval made it trustworthy
- consultation made it operational
- history made it reversible
- bulk low-value deactivation makes the active shelf intentionally governable

The shelf is now much closer to a real management system for reliability rules:

- propose
- approve
- activate
- deactivate
- bulk deactivate low-value active rules
- archive inactive unmatched rules
- preserve history
- restore from history

The next healthy step from here is to make recent shelf activity easier to
interpret at a glance, so operators can see the lifecycle moving without
opening raw mutation receipts.
