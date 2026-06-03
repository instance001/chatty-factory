# Architecture Checkpoint 32

The negative constraint shelf now has a full first lifecycle instead of a
one-way accumulation path.

At this checkpoint, approved rules can move through a managed loop:

- proposed
- approved
- activated or deactivated
- matched and reuse-tracked
- archived out of the active shelf
- preserved in shelf history
- restored back into the active shelf if needed

What changed:

- archived rules now persist in:
  - `runtime/constraint_shelf_history.json`
- shelf retirement is no longer destructive
- the desktop UI now exposes:
  - archived rule count
  - `Reveal Shelf History`
  - recent shelf-history entries
  - `Restore` on archived entries
- restoration now emits the same style of audited shelf mutation receipt as
  other shelf edits

Why this matters:

- the shelf is no longer just a rule cabinet
- it is now a governed reliability lifecycle
- operators can clean the active shelf without losing institutional memory
- and they can reverse that cleanup when a retired rule becomes relevant again

Architectural read:

- the negative constraint shelf is now behaving more like a real subsystem than
  a runtime convenience file
- this is the right kind of discipline if the factory is going to learn from
  failures over time without turning that learning process into clutter

The next healthy move from here is not merely adding more rules. It is making
the shelf and its history easier to interpret as they grow, especially around:

- recency
- match value
- restoration candidates
