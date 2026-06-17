# Negative Constraints Engine Parts List

This document describes the parts ChattyFactory would need for a fully realized
negative-constraints engine without relying on supported positive families as
the definition of what is allowed.

Use it as the end-to-end inventory for the factory shape we actually want:

- request in
- bounded attempt frozen
- host executes and verifies
- failures are metabolized into better retries, decomposition, or narrow
  constraint evidence

Supported families, templates, and starter lanes may still exist as helpful
accelerators, but they are not part of the minimum conceptual engine described
here.

## What This Excludes

This parts list intentionally excludes:

- family catalogs as the arbiter of allowed reality
- "supported lane only" routing
- silent coercion to the nearest known starter
- template-picker product logic
- model-owned truth or self-certifying success
- broad one-failure-to-one-rule negative catalogs

## Core Design Goal

The goal is not to stop bad attempts by predeclaring everything unsupported.

The goal is to let the factory honestly attempt bounded work, trap evidence,
retry differently, decompose when needed, triangulate true blockers, and only
promote narrow high-confidence constraints when reality has earned them.

## Required Parts

## 1. Intent Freeze

The factory needs a host-owned way to freeze what the user actually asked for.

This layer must:

- normalize the request
- separate hard requirements from preferences
- preserve explicit stack/toolchain constraints
- preserve requested behavior and scope
- record ambiguity instead of hiding it
- persist the frozen interpretation as a receipt

Without this, the system cannot tell the difference between:

- a valid fallback
- a degraded substitute
- a requirement-violating fake success

## 2. Bounded Task Shaper

The factory needs a host-owned mechanism that converts frozen intent into work
small enough to attempt honestly.

This layer must:

- choose the current attempt scope
- cap breadth, file count, and responsibility per step
- separate project-shape tasks from implementation tasks
- separate implementation tasks from verification tasks
- mark when a task is already at the atomization floor

This is the lens that focuses the model.
Without it, the model is asked to solve the whole build at once.

## 3. Attempt Scaffold

The factory needs a substrate-agnostic execution scaffold for new builds and
patches.

This layer must:

- create an honest bounded attempt even when no named family matches
- preserve the requested stack when that stack is a hard requirement
- surface when an attempt is scaffold-first, degraded, substituted, or
  requirement-not-met
- let patches target grounded project surfaces without demanding a named patch
  lane first

This is the core anti-prison-bar layer.
Unknown family must not mean automatic stop.

## 4. Model Proposal Interface

The factory needs a bounded place where the model may contribute shape without
owning truth.

The model may propose:

- project shape
- file plan
- implementation sequence
- likely toolchain commands
- bounded code/content changes
- decomposition candidates
- retry variants

The host must still own:

- frozen intent
- actual file execution
- verification
- receipts
- final classification

## 5. Work Order And Task Graph

The factory needs a host-owned plan artifact that turns the bounded attempt into
explicit executable units.

This layer must:

- freeze the selected task list
- label task kind
- label dependencies
- label target files/surfaces
- label expected outputs
- label verification hooks
- persist the work order and task graph

This is what stops the system from slipping into hidden ad hoc execution.

## 6. Execution Primitives

The factory needs a set of host-runnable primitive task types.

At minimum, the engine needs:

- host-sync primitives for deterministic file and state operations
- host-mechanical primitives for bounded toolchain steps
- model-authored microtasks for bounded authored changes
- patch-surgery primitives for existing project surfaces
- helper/service primitives when execution requires a bounded auxiliary process

These are primitives, not positive lanes.
They define how work is attempted, not what the product is allowed to be.

## 7. Verification Layer

The factory needs host-owned truth checks after every meaningful attempt.

This layer must:

- run syntax/build/test/postcheck validation where applicable
- distinguish execution success from requirement success
- distinguish partial progress from full acceptance
- detect fallback violations against frozen intent
- record evidence instead of trusting model narration

If the model says it worked but receipts say otherwise, the model loses.

## 8. Receipt System

The factory needs persistent host-owned receipts for every stage.

At minimum, it needs receipts for:

- frozen intent
- plan review
- work orders
- task attempts
- generation attempts
- decomposition
- retry search
- verification
- patch postcheck
- fallback/substitution classification
- failure vault entries
- triangulation sessions
- constraint promotion candidates

This is the factory memory.
Without receipts, the system cannot improve honestly over time.

## 9. Retry Search Engine

The factory needs a structured way to retry differently before declaring a true
constraint.

This layer must:

- try alternative methods, not blind repetition
- track posture/method identity
- support model ladder escalation
- preserve per-attempt evidence
- distinguish shell timeout from proven factory exhaustion
- stop only when success occurs or the bounded search space is honestly spent

This is how failure becomes search instead of shame.

## 10. Decomposition Engine

The factory needs reusable grammars that split tasks when they are too broad.

This layer must:

- detect when a task failed because it was too large
- map failure classes to reusable decomposition grammars
- split a broad task into smaller child tasks
- preserve lineage from parent task to child tasks
- rerun verification on the narrower tasks

The key rule is:

- the factory should select from reusable grammars
- the factory should not require a human to author a bespoke split rule for
  every new task class

## 11. Atomization Floor

The factory needs a hard wall that prevents decomposition into dust.

This layer must:

- define minimum useful task granularity
- detect when further splitting would destroy coherence
- classify "too small to split further" separately from "still failed"
- hand atomization-floor failures into retry search or model escalation instead
  of infinite subdivision

This keeps the engine usable for smaller models without collapsing into mush.

## 12. Failure Vault

The factory needs a provisional holding area for failures that are not yet good
enough to become durable constraints.

This layer must:

- store failed attempts and their evidence
- preserve partial hypotheses about likely blockers
- link related failures across runs
- keep unresolved failures provisional
- allow the search to resume later if the same sticking point returns

This is where "maybe this is the blocker" lives until reality proves it.

## 13. Failure Classification

The factory needs a generic classifier for why an attempt failed.

These classes should be broad and reusable, such as:

- task too broad
- tool invocation malformed
- missing dependency/toolchain
- contract mismatch
- verification failure
- environment instability
- model output format failure
- method space exhausted
- atomization floor reached

The point is not bespoke rules.
The point is generic classes that can select the next grammar or retry posture.

## 14. Triangulation Engine

The factory needs a way to narrow repeated failures to the actual culprit.

This layer must:

- compare multiple failed attempts at the same sticking point
- track what changed between retries
- narrow the failure from broad symptom to specific blocker
- distinguish correlated failure from converged causal evidence
- persist triangulation sessions and summaries

This is the difference between:

- "something around helpers failed"
- and
- "this helper type used in this exact way fails under these conditions"

## 15. Constraint Promotion Gate

The factory needs a promotion gate between provisional evidence and the real
negative shelf.

This layer must:

- refuse one-failure direct promotion
- require converged triangulated evidence
- require narrow scope
- require replacement guidance when possible
- persist the promotion candidate and approval rationale

Only after this gate should something become a durable negative constraint.

## 16. Negative Constraint Shelf

The factory needs a real constraint library, but it must be narrow and earned.

This shelf should contain:

- specific "will not work" findings
- exact failure conditions
- scope boundaries
- confidence/provenance metadata
- recommended alternatives or decomposition posture

This shelf must not become:

- a broad fear catalog
- a disguised positive lane matrix
- a list of everything the current model once struggled with

## 17. Fallback Classification

The factory needs an honest way to describe what kind of result it produced.

This layer must classify outcomes such as:

- full success
- partial success
- degraded fallback
- substituted stack
- requirement-not-met
- blocked pending narrower retry or escalation

This classification must be receipt-owned and visible to the operator.

## 18. Patch Truth Layer

The factory needs the same architecture on the patch side, not only builds.

This layer must:

- diagnose the current project state
- freeze patch intent
- ground the patch against actual files/surfaces
- perform bounded surgery
- run postcheck
- classify whether the requested change really landed

Without this, patching becomes story-telling instead of governed modification.

## 19. Runtime Budgeting

The factory needs realistic ceilings for planning, retry search, model ladders,
server launch, and cleanup.

This layer must:

- size budgets for multiple models and retry postures
- avoid treating outer shell timeout as authoritative failure by itself
- preserve internal timeout evidence in receipts
- allow longer planning windows for genuinely large requests

A negative-constraints engine cannot learn honestly if runs are starved before
their search space is actually spent.

## 20. Operator Surfaces

The factory needs UI and CLI surfaces that expose truth rather than theater.

Operators need to see:

- frozen intent
- fallback/substitution status
- task graph
- attempt lineage
- receipts
- decomposition decisions
- retry-search posture
- failure vault state
- triangulation summaries
- promotion candidates
- accepted constraints

If the operator cannot inspect the evidence trail, the engine is not governable.

## 21. Self-Improvement Loop

The factory needs a closed loop where failures improve future attempts without
humans authoring every next rule by hand.

That loop is:

1. attempt bounded work
2. verify honestly
3. if failure, classify it
4. retry differently where possible
5. decompose if task size is the issue
6. vault unresolved evidence
7. triangulate repeated evidence over time
8. promote only narrow converged constraints
9. reuse the learned grammar/constraint on future attempts

This is the core of the system learning how to focus itself.

## 22. Governance Rule Set

The factory needs a small set of generic principles that apply to everything.

Those principles should look like:

- do not let the model own truth
- do not exceed bounded task granularity
- do not atomize below the floor
- do not silently substitute hard requirements
- do not promote one-off failures into durable constraints
- do not stop at the first failed method if bounded alternatives remain
- do not treat unsupported family as forbidden reality

These principles are the real constraints architecture.

## Minimal End-To-End Definition

If you stripped out every positive family and starter catalog tomorrow, a fully
realized negative-constraints factory would still need:

- frozen intent
- bounded task shaping
- substrate-agnostic attempt scaffolding
- executable task primitives
- host-owned verification
- persistent receipts
- retry search
- decomposition grammars
- an atomization floor
- a provisional failure vault
- generic failure classification
- triangulation
- narrow constraint promotion
- honest fallback classification
- patch truth
- runtime budgeting
- inspectable operator surfaces

If those parts exist, the factory can honestly try unknown requests and improve
from its own evidence.

If those parts do not exist, removing positive lanes just creates chaos instead
of freedom.

## Short Version

The fully realized engine is not:

- "supported lanes, but negative"

It is:

- host-owned truth
- bounded model-authored work
- reusable decomposition grammars
- retry and model-ladder search
- atomization floor discipline
- provisional failure vaulting
- triangulated narrow constraint promotion
- honest surfaced fallback classification

That is the shape required to build a real negative-constraints factory.
