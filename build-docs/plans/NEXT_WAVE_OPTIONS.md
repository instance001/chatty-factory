# Next Wave Options

This document is the planning baseline after the current milestone checkpoint.

It exists to answer:

- what the most sensible next expansion waves are
- what each wave would improve
- what each wave would cost or risk
- which wave should come first

Read this alongside:
- [MILESTONE_CHECKPOINT.md](../checkpoints/MILESTONE_CHECKPOINT.md)
- [NEXT_MILESTONE.md](./NEXT_MILESTONE.md)
- [CHATTYCOG_HOSTING_MILESTONE.md](../milestones/CHATTYCOG_HOSTING_MILESTONE.md)

## Current Baseline

The rebuild now proves three important things at once:

1. it can emit and patch deterministic families
2. it can stop honestly on unsupported requests
3. it can scaffold, track, and help promote new deterministic lanes

That means the next wave should not be chosen just by "what feature sounds cool."

It should be chosen by asking:
- what most improves the factory as a factory
- what reduces the most future manual work
- what keeps the architecture honest as the surface widens

## Option 1: Host-Consumed `apply_patch_ready` Lanes

### Goal

Turn the current assisted promotion flow into an actually executable host step.

Right now the rebuild can generate:
- pending lane lifecycle state
- Rust promotion stubs
- `apply_patch`-ready templates

But it still stops short of:
- safely consuming those templates
- applying them into the live repo
- compiling after the change
- advancing lane state again only if the change holds

### Why It Matters

This is the strongest multiplier on the work already done.

It would move the rebuild from:
- "host can help a human wire the lane"

closer to:
- "host can complete most of the deterministic lane promotion path itself"

That is directly aligned with the factory thesis.

### Scope

Suggested steps:

1. add a new host command for consuming `apply_patch_ready` lane artifacts
2. load `registry.apply_patch.txt` and `handler.apply_patch.txt`
3. validate the target files still match the expected shape
4. apply the generated patches
5. run `cargo check`
6. if successful, advance lane state to something like `host_wired`
7. if unsuccessful, persist a typed promotion failure receipt

### Risks

- patch templates can drift if registry layout changes
- we need careful guardrails so the host does not blindly mutate source files
- this is the first place the host would be directly editing live Rust registry code

### Acceptance

- a lane can move from `apply_patch_ready` to `host_wired`
- the host refuses to apply stale templates if the target file shape drifted
- the host records success or failure receipts
- `cargo check` is required before lane state can advance

### Recommendation

This should be the first next-wave target.

It gives the biggest return on everything we already built.

## Option 2: UI Lane-Management Surface

### Goal

Make the extension-lane lifecycle visible and usable from the rebuild UI, not just the CLI.

### Why It Matters

Right now the host owns strong lane lifecycle state, but most of it is only convenient if you are already working through the CLI and receipts manually.

A UI surface for lane management would:
- reduce friction
- make the extension pipeline feel real
- make the factory’s self-growth legible

### Scope

Suggested steps:

1. show pending lane registry state in the UI
2. group lanes by lifecycle state:
   - pending
   - implemented
   - validated
   - promotion prepared
   - apply-patch ready
3. add buttons for:
   - implement
   - validate
   - prepare promotion
   - prepare apply-patch
4. surface the relevant generated artifacts inline
5. optionally show route/fallback links back to the originating request

### Risks

- easy to spend time on presentation before the automation step is strong enough
- could make the UI look more capable than the underlying host really is if we over-polish it early

### Acceptance

- all current lane lifecycle commands are usable from the UI
- generated artifact paths are visible from the UI
- state refresh is host-driven, not duplicated in UI logic

### Recommendation

This should come after host-consumed `apply_patch_ready` lanes.

It is important, but it is more valuable once the next lifecycle step is real.

## Option 3: One Full Helper/Service Lane Proof

### Goal

Take a currently unsupported helper/service request and grow it into a real deterministic helper lane.

### Why It Matters

So far, the strongest proofs are:
- in-project patch lanes
- hosted module surface growth

What we have not yet fully proven is:
- helper-backed or service-backed deterministic growth

That is a major missing proof if the long-term goal is broad plain-language build capability.

### Good Candidate Shapes

Examples:
- ChattyCog hosted module plus local helper process
- filesystem watcher helper
- background job bridge lane
- deterministic local API helper for a module

### Scope

Suggested steps:

1. choose one helper-shaped request that currently falls back honestly
2. push it through the full extension pipeline
3. add the helper lane contract
4. add acceptance that proves the helper boundary is real
5. make the hosted/main family and helper lane cooperate without conflating their responsibilities

### Risks

- this is a bigger architecture step than another patch lane
- helper/service boundaries can sprawl fast if contracts are not tight
- it is easier to accidentally reintroduce "LLM in the machinery" pressure here

### Acceptance

- one helper/service request becomes a real deterministic lane
- helper execution is policy-guarded and receipt-backed
- the main family and helper lane remain explicitly separated

### Recommendation

This should be the third next-wave target.

It is strategically important, but it will go better after the host can consume its own promotion artifacts more directly.

## Recommended Order

The cleanest next-wave order is:

1. host-consumed `apply_patch_ready` lanes
2. UI lane-management surface
3. one full helper/service lane proof

That order does three good things:

- reduces manual promotion work first
- makes the new lifecycle visible second
- then spends the harder proof effort on helper/service work with better internal tooling already in place

## Why This Order Wins

If we jump to helper/service work first:
- we widen the architecture before we finish the self-growth loop

If we jump to UI first:
- we risk polishing orchestration around a still-manual promotion step

If we finish host-consumed promotion first:
- every later lane becomes cheaper to land

That is the highest-leverage next move.

## Short Version

The rebuild has now proven:
- deterministic builds
- deterministic patches
- deterministic lane growth scaffolding

The smartest next wave is to make the host consume more of the final promotion path itself before we widen the feature surface again.
