# Architecture Checkpoint 34

The negative constraint shelf is now being curated by value on both the active
and archived sides.

At this checkpoint, the shelf can distinguish:

- active rules that are matching and earning their keep
- active rules that are still live but currently unmatched
- inactive rules that are unmatched and ready for retirement
- archived rules that never proved useful
- archived rules that were historically useful and may deserve restoration

What changed in this wave:

- the active shelf gained:
  - `Show low-value active rules only`
  - `[active-unmatched]`
- the shelf summary now separates unmatched rules into:
  - inactive unmatched
  - active unmatched
- the archived side now distinguishes:
  - never-matched retired rules
  - historically useful retired rules
- historically useful archived rules can now be sorted for restore triage by:
  - past match count
  - then archive recency

Why this matters:

- the shelf is no longer being curated only after rules are retired
- active rules are now subject to the same value-aware scrutiny as archived ones
- that keeps the approved shelf from drifting into “active by default forever”

Architectural read:

- proposal and approval made the shelf trustworthy
- consultation made it operational
- mutation and history made it governable
- value-aware signals now make it actively curatable

This is the right kind of pressure if the factory is going to keep learning from
real failures without turning every approved lesson into permanent live baggage.

The next healthy move from here is to let those value signals drive the next
lifecycle step directly, especially for active rules that are not matching
anything and may be ready for deactivation.
