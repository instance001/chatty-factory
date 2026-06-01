# Acceptance Governance Decision Review

This review comes after governance reached:

- proof harness bundles
- composition bundles
- patch recipes
- helper lanes
- bridge lanes
- family manifests
- template bundles

The question now is whether:

- `operator_registry/acceptance_recipes/`

should become its own independently governed substrate.

## Short Answer

Not yet.

The current architecture is better served by keeping acceptance recipes governed indirectly through:

- patch governance
- composition governance

for now.

Acceptance governance should become independent only if acceptance recipes start behaving like first-class reusable contracts in their own right rather than mostly paired support artifacts for patch lanes and mixed bundles.

## Why This Review Is Needed

Acceptance recipes are the strongest remaining coupled surface in the rebuild.

They matter because:

- they shape what "done" means
- they are durable registry assets
- they can drift
- they influence trust in deterministic lanes

But they are different from the substrates already given full independent governance.

The key question is not:

- are acceptance recipes important?

They clearly are.

The key question is:

- are acceptance recipes currently independent enough to justify their own governance loop?

## Current State

Acceptance recipes currently behave mostly as:

- paired lane artifacts
- patch-lane companions
- mixed-bundle support artifacts

They are already pulled into governance indirectly because:

1. patch governance hashes and validates the paired acceptance artifact
2. composition governance includes patch-side acceptance through the mixed bundle
3. live validation already treats patch + acceptance as a coupled deterministic contract

So the rebuild is not ignoring acceptance.

It is governing acceptance through the stronger parent unit that currently owns its lifecycle.

## Arguments For Independent Governance

Independent governance would eventually make sense if acceptance recipes become:

1. independently reusable
- one acceptance recipe meaningfully shared by multiple patch lanes

2. independently promoted
- acceptance changes landing without being conceptually subordinate to a patch lane update

3. independently risky
- acceptance drift becoming a major source of regressions even when the paired patch contract stays stable

4. independently inspectable
- operators wanting to scan acceptance quality/risk as its own catalog, not just through patch details

If those things become common, acceptance governance should likely become its own wave.

## Arguments Against Independent Governance Right Now

Right now, the case against a new independent governance subsystem is stronger.

### 1. Acceptance is still mostly subordinate

In the current rebuild shape, acceptance recipes are usually:

- tied to one patch recipe
- promoted with that patch recipe
- validated with that patch recipe

That means the patch recipe remains the natural governed unit.

### 2. Separate governance would duplicate the curation loop

A new acceptance-governance subsystem would likely repeat:

- artifact hashing
- drift classification
- baseline classification
- refresh status
- UI catalog surfacing

without yet giving us proportionate architectural leverage.

### 3. It would blur the real ownership boundary

Today, the stronger ownership line is:

- patch lane owns its paired acceptance contract

Creating a separate governance loop too early risks suggesting:

- acceptance is already a peer catalog

when it is still mostly a paired artifact.

### 4. The current indirect model is already useful

Because patch governance already captures paired acceptance artifacts, we already know:

- when the patch-side contract changed
- when the acceptance-side artifact changed
- whether the combined lane moved away from its last known-good baseline

That is enough governance signal for the current maturity level.

## Recommended Decision

Keep acceptance recipes governed indirectly for now.

That means:

- patch governance remains the primary governance owner for patch + acceptance pairs
- composition governance continues to inherit acceptance relevance through its patch-side bundle
- acceptance should be reviewed as part of those parent receipts rather than split into a separate catalog

## What To Improve Without Full Independent Governance

We can still make acceptance more visible without introducing a full new subsystem.

Recommended lightweight improvements:

1. enrich patch governance receipts
- make the paired acceptance artifact more explicit in summaries and notes

2. enrich composition governance receipts
- call out which acceptance artifacts were implicitly part of the mixed bundle

3. add UI copy in patch detail
- make it clearer that acceptance is already part of the governed patch contract

Those changes preserve the current ownership model while improving operator clarity.

## What Would Change The Decision Later

Revisit this decision if any of these become true:

1. acceptance recipes are reused across multiple lanes
2. acceptance artifacts are promoted or edited independently from patch lanes
3. acceptance-only regressions become a common operational risk
4. operators need an acceptance-first catalog view rather than lane-first inspection

If two or more of those become true, independent acceptance governance would likely be justified.

## Architectural Read

The important insight is:

- not every durable artifact deserves its own first-class governance subsystem at the same time

The rebuild is stronger when governance boundaries follow real ownership and lifecycle boundaries.

Right now that boundary is:

- patch lane plus paired acceptance contract

not:

- acceptance recipe as an isolated governance peer

## Recommended Next Move

Do not start `ACCEPTANCE_GOVERNANCE_MILESTONE.md` yet.

Instead:

1. keep acceptance governed indirectly
2. tighten patch/composition receipt visibility around acceptance coupling
3. only revisit independent acceptance governance if reuse or promotion patterns change

## Short Read

This decision review says:

- acceptance recipes matter
- they are already governed indirectly through stronger parent substrates
- they are not yet independent enough to justify their own full governance loop

So the right move is:

- **no independent acceptance governance yet**
- improve visibility inside patch and composition governance instead
