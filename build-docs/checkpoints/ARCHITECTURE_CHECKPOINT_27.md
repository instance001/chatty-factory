# Architecture Checkpoint 27

## What Changed

Patch planning is no longer a single freeze-and-go step.

The host now performs a self-review pass between:

- `ProjectPatchDiagnosis`
- and `PatchIntentFreeze` preflight/execution

That self-review persists a new host-owned artifact:

- `PatchPlanReview`

The first slice can:

- narrow target files to declared surface groups
- narrow insertion points to the reviewed file set
- promote modern replacement lanes into the frozen candidate set
- record whether the factory proceeded with the original or refined plan

## Why This Matters

Before this checkpoint, the patch pipeline could explain:

- what the project looked like
- what the frozen plan intended
- why execution was blocked or allowed

But it did not yet show that the factory could critique its own patch plan
before surgery.

Now the patch pipeline reads more like:

1. diagnosis
2. freeze
3. plan self-review
4. preflight
5. execution or skip
6. postcheck

That is a meaningful reliability upgrade.

## Architectural Read

This is the point where the patch engine starts behaving less like:

- "pick a lane and hope the freeze was already perfect"

and more like:

- "draft a plan, then check whether that draft is too broad, stale, or missing a better modern route"

The change is modest in code shape, but important in product shape.

The factory is beginning to revise its own operational intent before acting.

## Best Next Move

The strongest next step is to deepen what self-review can critique.

The most obvious future directions are:

- review declared ownership boundaries more aggressively
- compare requested capability against modern replacements more semantically
- detect when the reviewed plan should redirect instead of only enrich candidates

The main lesson from this checkpoint is:

- patch safety should not depend only on a first frozen draft
- the factory should be able to refine its own surgical plan before incision
