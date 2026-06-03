# Architecture Checkpoint 31

Negative-shelf management has moved beyond observation and into deliberate
curation.

At this checkpoint, the approved constraint shelf is no longer just helping the
operator answer:

- which rules exist
- which rules are matching
- which rules are unmatched

It is starting to answer:

- which rules are likely dead weight
- which rules can be retired safely

What changed:

- the shelf UI now marks low-value entries directly with:
  - `[unmatched]`
  - `[inactive-unmatched]`
- the shelf now supports focused triage views:
  - `Show unmatched approved rules only`
  - `Show inactive + unmatched only`
- the shelf is therefore beginning to separate:
  - active reusable reliability rules
  - from approved-but-unused rules
  - from inactive rules that are not carrying any current verification load

Why this matters:

- the shelf should not become a one-way accumulation surface
- if every proposed rule is approved forever, the factory will slowly collect
  reliability folklore instead of a disciplined negative funnel
- explicit curation keeps the shelf:
  - auditable
  - interpretable
  - and worth consulting

Architectural read:

- the approved shelf is now becoming a governed reliability subsystem
- not just a proposal archive
- and not just a dashboard

The next healthy step from here is to pair this visibility with a deliberate
retirement path, so rules that are both inactive and unmatched can be archived
cleanly instead of only labeled as low-value.
