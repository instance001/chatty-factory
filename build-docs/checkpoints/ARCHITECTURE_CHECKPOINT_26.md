# Architecture Checkpoint 26

## What Changed

Replacement guidance now reaches the per-surgery layer, not just project patchability governance.

The system already knew, at the project level, which blocked lanes were:

- risky blockers
- historical blockers with named modern replacements

This wave carried that distinction into the patch surgery artifacts themselves.

`PatchIntentFreeze` now carries:

- `superseded_by_patch_kinds`
- `replacement_guidance_summary`

And patch-skip results now surface that guidance directly in host-facing summaries and route notes.

## Why This Matters

Before this wave:

- the project browser could explain that a blocker was historical
- the selected-project patchability view could show the modern replacement path

But the per-surgery record still mostly said:

- this patch was blocked
- here is the structural reason

That meant the operator still had to jump from:

- surgery-level evidence

back to:

- project-level governance

to understand whether the right response was repair or replacement.

Now the surgical artifact itself can say:

- this was a historical blocker
- here is the modern lane to use instead

## Architectural Read

This is a small but important interpretation upgrade.

The patch pipeline is becoming more semantically complete:

1. diagnosis explains structure
2. freeze explains the surgical contract
3. skip/apply results explain the outcome
4. replacement guidance explains the next best action

That means a patch skip is no longer only a rejection.

It can now be:

- a guided redirect

which is a much stronger product behavior.

## Best Next Move

The next strongest move is probably to keep pushing this same distinction into the remaining automation-facing surfaces:

- CLI/project-browser summaries
- future machine-readable host summaries
- any future planner-facing patchability handoff

The core lesson from this checkpoint is:

- replacement guidance should live wherever the blocked outcome is observed
- not only in a deeper governance panel the operator may never open
