# Architecture Checkpoint 33

The negative constraint shelf has now crossed from simple rule storage into a
curated lifecycle with value-aware history.

At this checkpoint, the shelf and shelf-history surfaces can distinguish:

- active rules that are still matching real failures
- inactive rules that are not earning their keep
- archived rules that never proved useful
- archived rules that were historically useful and may be worth restoring

What changed in this wave:

- the active shelf gained curation-oriented signals:
  - `[unmatched]`
  - `[inactive-unmatched]`
  - `Show unmatched approved rules only`
  - `Show inactive + unmatched only`
- shelf retirement became non-destructive through:
  - `runtime/constraint_shelf_history.json`
- archived entries now carry:
  - `archived_match_count`
- the UI shelf history can now filter for:
  - never-matched archived rules
  - historically useful archived rules

Why this matters:

- the shelf is no longer just accumulating “negative knowledge”
- it is starting to govern that knowledge by value
- that matters if ChattyFactory is going to learn continuously without turning
  the bookshelf into noise

Architectural read:

- approval made the shelf trustworthy
- consultation made it operational
- mutation and history made it governable
- value-aware history makes it restorable in an intelligent way

This is an important maturity step because the factory now has the beginnings
of a real rule-lifecycle posture:

- propose
- approve
- match
- measure
- retire
- preserve
- restore

The next healthy direction from here is to keep strengthening the “measure” and
“restore” parts of that loop, especially by surfacing which archived rules were
historically most valuable and which current rules are not paying for
themselves.
