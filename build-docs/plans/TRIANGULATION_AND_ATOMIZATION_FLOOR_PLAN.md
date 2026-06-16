# Triangulation And Atomization Floor Plan

## Why This Exists

## Current Checkpoint

The core posture described here is no longer only a plan.
The current factory now already has:

- atomization floor decisions persisted under `runtime/atomization_floor_decisions/`
- provisional failure-vault entries under `runtime/failure_vault/`
- triangulation sessions under `runtime/triangulation_sessions/`
- promotion candidates under `runtime/constraint_promotion_candidates/`
- a triangulation loop summary receipt under `runtime/triangulation_loop_summary.json`
- model-ladder exhaustion reporting that distinguishes:
  - current-model-only exhaustion
  - full model-ladder exhaustion
- retry-search ladder proof receipts under `runtime/retry_search_proofs/`
  with:
  - explicit outer timeout ceilings
  - internal-timeout evidence
  - receipt-owned final outcomes instead of shell-owned guesses

The remaining job is to keep narrowing the host toward scaffold/orchestration while
letting the frozen-task, vault, triangulation, and grammar machinery do more of
the real adaptive learning work over time.

ChattyFactory now has:

- frozen interpretation
- frozen build plans
- frozen task graphs
- adaptive task decomposition
- decomposition inference from task receipts

That is the right direction, but it introduces two new risks if left unchecked:

1. atomizing tasks into dust
2. turning the negative constraint library into a broad failure catalog

Neither is acceptable.

The factory should not:

- decompose forever
- add a real negative constraint the first time something fails

Instead it should:

- stop decomposition at a hard floor
- hold failures provisionally
- retry differently
- use xray evidence to triangulate the true narrow blocker
- only promote a constraint into the real library when confidence is high

## Architectural Intent

The intended flow is:

1. attempt a frozen task
2. if it fails, classify the failure
3. if the task is still above the atomization floor, decompose by grammar
4. if the task is already at the floor, retry by a meaningfully different method
5. store all of that evidence in a provisional vault
6. only add a real negative constraint after repeated convergent evidence

This keeps the architecture aligned with the real goal:

- the library should wall off only narrow, high-confidence non-viable patterns
- not accumulate broad memories of every failed attempt

## Core Layers

### 1. Constraint Principles

These are generic and stable.

Examples:

- do not exceed task granularity the current model/runtime pair can support
- do not let the host invent semantic reasoning
- classify failure before proceeding
- do not widen scope during repair
- do not promote one-off failures into durable constraints

These principles apply everywhere. They are not case-specific library entries.

### 2. Failure Vault

Failed attempts should first land in a provisional holding area.

The vault is:

- evidence, not authority
- provisional, not blocking
- used to compare retries and variants

The vault should record:

- task shape
- task subtype
- attempt method
- decomposition level
- failure class
- xray findings
- retry variant
- whether any alternate method later succeeded

### 3. Triangulation Session

A triangulation session is the bounded loop that decides whether a failure is:

- recoverable
- decomposable
- method-sensitive
- or converging on a genuinely non-viable pattern

The key idea is:

- fail once is not enough
- fail differently and still converge is meaningful

### 4. Real Constraint Library

The real negative library should only contain:

- narrow
- high-confidence
- triangulated
- usage-specific

entries such as:

- this specific task shape
- used in this specific way
- under this specific execution posture
- repeatedly converged on the same root failure
- without a successful alternate method in the same session

## Atomization Floor

### Why The Floor Is Needed

Without a floor, adaptive decomposition collapses into mush.

At some point, the next split stops creating meaningful leverage and starts producing trivial fragments that only create more bookkeeping.

That is the point where the system must stop decomposing and switch to triangulation.

### Floor Definition

A task has reached the atomization floor when the next decomposition would no longer create a meaningful unit of work.

Examples of floor-level tasks:

- one short semantic label
- one single-purpose literal
- one single clause with one narrow communicative job
- one host-owned composition step
- one bounded mechanical anchor insertion

Examples of tasks that are still above the floor:

- semantic object bundles with multiple fields
- sentence tasks with multiple communicative concerns
- mixed UI plus behavior tasks
- helper creation tasks that still combine interface, state, and wiring

### Floor Rule

If a task is above the floor:

- decomposition is allowed

If a task is at the floor:

- decomposition stops
- the next move is alternate-method retry, not smaller children

## Triangulation Logic

### Promotion Should Not Happen On First Failure

The library should not receive:

- one broad failure
- one noisy runtime glitch
- one model spill into reasoning text

Those belong in the vault.

### What Counts As A Meaningfully Different Retry

A retry only helps triangulation if it changes method, not just repeats the same call.

Examples:

- host render from semantic parts instead of raw code emission
- decomposed child tasks instead of the broad parent task
- alternate prompt posture
- alternate extraction/recovery mode
- alternate grammar
- alternate execution helper

### Promotion Confidence

A provisional failure becomes a library candidate only when the session has enough convergent evidence.

Suggested promotion conditions:

- same task shape
- same usage pattern
- floor reached or no more useful grammar available
- repeated distinct attempts
- convergent failure class or root cause
- no successful alternate method in the same triangulation session

If an alternate method succeeds:

- the vault entry is downgraded or closed
- no real library promotion happens

## Proposed Contracts

The next contract layer should separate provisional evidence from durable library state.

Suggested additions:

- `AtomizationFloorDecision`
- `FailureVaultEntry`
- `TriangulationSession`
- `TriangulationAttempt`
- `ConstraintPromotionCandidate`

### AtomizationFloorDecision

Should answer:

- current task is above floor
- current task is at floor
- why
- what alternate methods remain

### FailureVaultEntry

Should record:

- the failed task
- the task shape
- the execution posture
- the failure class
- xray findings
- triangulation session id
- whether the evidence is still provisional, downgraded, or closed

### TriangulationSession

Should record:

- parent task lineage
- decomposition depth
- retry variants attempted
- convergence or divergence across attempts
- whether a success resolved the session
- whether confidence reached promotion level

### ConstraintPromotionCandidate

Should be created only when:

- the triangulation session converges strongly enough

It should include:

- narrow usage pattern
- confidence rationale
- failed variants considered
- matched constraint principles
- matched failure class
- recommended library wording

## Decision Ladder

The intended runtime ladder is:

1. attempt task
2. classify failure
3. check atomization floor
4. if above floor:
   - match decomposition grammar
   - decompose
5. if at floor:
   - retry differently
   - record triangulation evidence
6. if alternate method succeeds:
   - close or downgrade the vault entry
7. if repeated methods converge on the same narrow blocker:
   - emit a promotion candidate
8. only then:
   - allow library addition

## What This Prevents

This plan is designed to prevent three bad outcomes:

### 1. Infinite decomposition

The floor stops the task graph from collapsing into meaningless fragments.

### 2. Negative-lane catalog drift

The vault and triangulation loop stop every fresh failure from becoming a durable prohibition.

### 3. Premature pessimism

A successful alternate attempt prevents the system from falsely concluding that a pattern is broadly impossible.

## First Implementation Wave

1. Add explicit atomization-floor decisions to task execution.
2. Add a provisional failure vault artifact under `runtime/`.
3. Group retries under triangulation sessions.
4. Make successful alternate attempts close or downgrade vault entries.
5. Add promotion candidates instead of direct library writes.
6. Only later, wire approved promotion candidates into the real negative constraint shelf/library.

## Success Condition

This plan succeeds when ChattyFactory can say:

- this task failed, but only provisionally
- this is the current triangulation session
- this is the floor decision
- these alternate methods were tried
- this later attempt succeeded, so no library addition is warranted

or:

- this task pattern reached the floor
- retries by materially different methods still converged on the same narrow root failure
- confidence is now high enough to propose a real constraint

That is the line between:

- a thoughtful negative architecture

and:

- a growing catalog of “don’t try things.”
