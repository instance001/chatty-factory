# Patch Plan Self-Review Milestone

This milestone adds a new reliability layer between patch diagnosis and patch
execution:

- the factory should review its own patch plan
- refine that plan when it is too broad or too stale
- and only then proceed to preflight and execution

## Why This Milestone Exists

The existing surgery chain already has:

1. diagnosis
2. intent freeze
3. preflight
4. execution
5. postcheck

That is strong, but it still assumes the first frozen plan is already the best
one.

This milestone upgrades that assumption.

The factory should be able to:

- critique the first patch plan
- narrow the surgical surface to the declared contract
- promote modern replacement lanes into the candidate set
- record whether it proceeded with the original or refined plan

## Core New Artifact

- `PatchPlanReview`

This artifact is host-owned and should answer:

- what the first frozen plan looked like
- what the self-review changed
- whether the reviewed plan was safer or more precise
- whether the plan should proceed unchanged, proceed refined, or stop pending replan

## First Implementation Slice

The first slice is intentionally bounded.

It should:

- review `PatchIntentFreeze` before preflight
- narrow target files to declared surface groups when possible
- narrow insertion points to the surviving file set
- promote superseding modern patch kinds into the frozen candidate set
- persist a `PatchPlanReview` receipt under `runtime/`

The first slice does not need to do free-form replanning.
It only needs to make the frozen surgical plan more structurally honest before
execution.

## Success Criteria

This milestone is successful when:

1. a patch request emits a `PatchPlanReview` artifact before patch execution
2. the review can refine the frozen target file set
3. the review can enrich the candidate patch set with modern replacements
4. the result path records whether the factory proceeded with:
   - the original plan
   - a refined plan
   - or a blocked pending replan outcome

## Why This Matters

This is the next step in making ChattyFactory safer than a blind patch router.

The patch flow becomes:

1. diagnose the patient
2. draft the surgical plan
3. review the surgical plan
4. verify preflight conditions
5. operate
6. run post-op checks

That is much closer to the product behavior we want:

- the factory should patch its own patch plan before it patches the project
