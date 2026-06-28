# Fresh Rebuild Negative-Lane Audit

This document defines what a clean "off the lot" rebuild of ChattyFactory
should look like after the architecture surgery.

Use the current workspace only as a read-only example of:

- what evidence and receipts are useful
- where positive-lane scars tend to hide
- which contracts drift when host-owned shape authority leaks back in

Do not use the current workspace as a preservation target.

The rebuild goal is not:

- rename family logic
- preserve old catalogs behind softer labels
- keep compatibility museums alive
- move positive-lane authority into substrate/toolchain fields

The rebuild goal is:

- a negative-lane factory
- host-owned truth and safety
- LLM-owned positive implementation method
- evidence-driven next attempts
- no supported-family or scaffold-era runtime authority

## Deterministic Host Parts List

Build the clean ChattyFactory host from this deterministic parts list.

1. Request intake
2. Intent freeze
3. Plan freeze
4. Evidence scanner
5. Substrate and toolchain detector
6. Bounded build gauntlet
7. Bounded patch and mutation gauntlet
8. Verification runner
9. Postcheck comparator
10. Receipt writer
11. Failure classifier
12. Triangulation and retry spine
13. Constraint promotion
14. Library and runtime state system
15. Model gearbox
16. Runbook executor
17. Operator dashboard
18. Export and result viewer

If a proposed host component does not fit cleanly inside one of those parts, it
must justify itself in negative-lane terms before it is added.

The rebuild should assume that the LLM already carries the positive
implementation patterns. The host should therefore be built as a bounded funnel,
not as a second product brain.

## Deterministic Part Boundaries

Each part should stay mechanically narrow.

### 1. Request Intake

Owns:

- raw request capture
- mode selection such as build or patch
- active project targeting when patching
- basic normalization for storage and downstream processing

Must not own:

- product interpretation
- family guessing
- artifact-shape selection
- starter or template selection

### 2. Intent Freeze

Owns:

- preserved user intent
- frozen request wording for the active attempt
- allowed clarifications and explicit refinements

Must not own:

- host goal mutation
- silent intent broadening
- "near enough" reinterpretation

### 3. Plan Freeze

Owns:

- bounded attempt shape
- allowed files, tools, and verification targets
- declared constraints for this specific attempt

Must not own:

- positive artifact identity
- canonical app classification
- hidden fallback substitution

### 4. Evidence Scanner

Owns:

- current project facts
- file presence and layout facts
- receipt and runtime history lookup
- mismatch evidence between request and artifact

Must not own:

- rescue heuristics
- optimistic success narration
- family identity recovery

### 5. Substrate And Toolchain Detector

Owns:

- execution substrate facts
- toolchain facts
- language/runtime/package-manager facts
- operator-evidenced surface facts

Must not own:

- product family identity
- starter identity
- template identity
- "what this reminds the host of" logic

### 6. Bounded Build Gauntlet

Owns:

- frozen build attempt execution
- bounded method selection
- allowed build-side retries under evidence
- build receipts and verification handoff

Must not own:

- nearest-family selection
- supported-lane routing truth
- substitute scaffold authority

### 7. Bounded Patch And Mutation Gauntlet

Owns:

- same-surface mutation against existing artifacts
- requirement-gap repair
- bounded affected files
- regeneration of implementation and acceptance when frozen intent changes the
  artifact contract

Must not own:

- duplicate-skip as a terminal truth
- stale contract preservation
- deterministic patch lane worship
- host-selected substitute artifact shape

### 8. Verification Runner

Owns:

- execution of explicit verification commands
- capture of pass and fail evidence
- command result receipts

Must not own:

- success inflation
- shape-based forgiveness
- documentation-only pass logic

### 9. Postcheck Comparator

Owns:

- comparison between frozen intent and resulting implementation
- comparison between behavior, acceptance, spec, and README/operator guidance
- stale-contract rejection

Must not own:

- superficial "files changed so success" logic
- old-tool-kind bias
- permissive pass-through for stale artifact behavior

### 10. Receipt Writer

Owns:

- durable structured receipts
- attempt lineage
- evidence trail
- next-attempt truth

Must not own:

- marketing language
- family identity as active truth
- operator steering toward substitute shapes

### 11. Failure Classifier

Owns:

- honest classification of failure mode
- distinction between requirement gap, verification failure, tool failure, and
  model limitation

Must not own:

- hidden solution proposals
- artifact-type substitution
- excuse generation

### 12. Triangulation And Retry Spine

Owns:

- retry differently under evidence
- decomposition
- triangulation against prior failures and receipts
- narrowing under promoted constraints

Must not own:

- fallback-to-family logic
- fallback-to-scaffold logic
- easier-product-form substitution

### 13. Constraint Promotion

Owns:

- promotion of discovered boundaries into the next attempt
- explicit tightening after repeated failure

Must not own:

- ad hoc host preferences
- catalog-driven policy
- product-shape rescue rules

### 14. Library And Runtime State System

Owns:

- current receipts
- current run state
- current project registry or library state if still needed
- bounded historical evidence that is still useful now

Must not own:

- museum preservation
- obsolete family-era artifacts by default
- stale proof/template/fallback debris

### 15. Model Gearbox

Owns:

- choosing which model to try
- recording why escalation is or is not justified
- keeping model swaps late in the gauntlet

Must not own:

- product-shape authority
- family knowledge as routing truth
- host-side positive implementation catalogs

### 16. Runbook Executor

Owns:

- bounded execution steps
- tool invocation order
- repeatable mechanical actions

Must not own:

- freeform product invention
- shape rescue behavior
- silent scope expansion

### 17. Operator Dashboard

Owns:

- request visibility
- receipt visibility
- verification visibility
- next-attempt visibility
- runtime status visibility

Must not own:

- family badges
- lane badges
- scaffold pickers
- template pickers
- comparison-bundle education panels

### 18. Export And Result Viewer

Owns:

- artifact access
- result summaries
- receipt export
- bounded diagnostic export

Must not own:

- doctrine contamination
- stale positive-lane framing
- success narration that outruns evidence

## Do Not Add

Do not add any new host feature that behaves like a disguised positive lane.

Specifically do not add:

- families
- family ids as planning truth
- starters
- templates as routing truth
- supported lanes
- scaffold catalogs
- product categories
- canonical artifact types
- nearest-match artifact selection
- host-owned substitute build shapes
- positive-lane rescue paths
- comparison-bundle operator products
- proof-template product centers
- dashboard badges that imply product identity truth
- planner prompts that preload artifact families or host-owned product labels
- postchecks that accept stale contracts because docs were refreshed
- compatibility buckets that quietly re-enter runtime authority

If a new field, panel, manifest, registry item, prompt field, or receipt field
can answer:

- what family is this?
- what starter should we use?
- which template is closest?
- which scaffold should substitute?
- what canonical app type is this?

then it is presumptively a doctrine violation.

## Anti-Scar Rules

These are the rules that should keep a fresh rebuild from repeating the same
mistakes this iteration made.

### Rule 1: Neutral Names Are Not Enough

Do not rename a family lane into:

- `substrate_kind`
- `capability_id`
- `execution_strategy`
- `surface_type`

unless the value is derived from evidence and execution reality rather than from
host recognition of a positive product shape.

### Rule 2: Compatibility Must Stay Read-Only

Legacy parsing is allowed only when strictly necessary.

If legacy metadata survives:

- isolate it under explicit compatibility naming
- do not feed it into planning
- do not feed it into routing
- do not feed it into fallback
- do not feed it into prompt context
- do not feed it into UI framing

### Rule 3: Fallback Must Stay Mechanical

Fallback must always answer:

- what failed?
- what evidence was observed?
- what changes on the next attempt?
- what verification target applies?
- whether toolchain or model escalation is justified?

Fallback must never answer:

- which family should be tried?
- which scaffold should substitute?
- which known artifact category is closest?

### Rule 4: Patch Success Must Be Behavioral

Patch success requires alignment between:

- source behavior
- project spec
- acceptance plan
- README or run instructions
- verification results

It is not enough to:

- copy new request text into docs
- preserve old implementation
- preserve old acceptance commands
- claim full success because the patch route executed

### Rule 5: Delete Museums

During surgery, prefer deletion over preservation for:

- old runtime state
- old generated projects
- old proof runs
- old family fixtures
- old template catalogs
- old library entries
- old migration artifacts without current tests

### Rule 6: UI Is Runtime Authority

If the operator can see it during normal work, it participates in runtime
doctrine whether intended or not.

That means normal UI must not display:

- family identity
- lane identity
- scaffold identity
- template identity
- cross-family proof identity
- comparison bundle identity
- substitute artifact recommendations

## Core Equation

The rebuild must preserve this equation:

- the user carries the intent
- the LLM carries the method
- the host carries the funnel
- the output carries the artifact

Anything beyond that needs strong justification.

## Read-Only Example Rule

This iteration may be inspected for:

- receipt formats worth keeping
- failure modes worth preventing
- examples of stale contract carryover
- examples of host contamination in runtime, UI, prompt context, or patch flow

This iteration must not be treated as proof that a surviving surface deserves to
exist in the rebuild.

If a surface exists now but violates negative-lane doctrine, the rebuild should
delete it rather than faithfully recreate it.

## Must Exist

The fresh rebuild must include the following host-owned machinery.

### Intent And Truth

- request capture
- frozen intent for build and patch flows
- preserved follow-up intent for mutation/repair work
- explicit permission boundaries
- explicit execution boundaries
- bounded file targeting for patch work

### Evidence And Receipts

- request receipts
- execution receipts
- verification receipts
- failure receipts
- next-attempt receipts
- patch diagnosis receipts
- mutation/repair receipts when patching existing surfaces

Each receipt should explain:

- what was attempted
- what evidence was observed
- what constraints were active
- what failed or passed
- what changes on the next attempt

### Negative-Lane Retry Spine

- attempt under frozen intent
- honest failure classification
- retry rules driven by evidence
- decomposition when the attempt is still too large
- triangulation against prior failures and receipts
- constraint promotion when failure reveals a missing boundary
- toolchain or model escalation only when justified

### Verification

- explicit verification commands attached to a bounded attempt
- result classification based on actual verification outcome
- postchecks that compare implementation, contract files, and acceptance against
  frozen intent
- refusal to report success when docs/specs change but behavior does not

### Patch Repair

For patching an existing project, the rebuild must support a generic repair
gauntlet:

- existing surface
- preserved frozen follow-up request
- observed requirement gap
- bounded affected files
- regenerated implementation and acceptance contract
- verification against the repaired request

Unmatched deterministic patch lanes must flow into this repair gauntlet rather
than die at "no lane matched yet."

### Neutral Runtime And UI Surfaces

Normal operator surfaces may expose:

- request
- planner/model status
- build actions
- patch actions
- last result
- verification receipts
- runbook or execution status
- next-attempt or gauntlet status

Those surfaces must speak in terms of evidence, constraints, receipts,
verification, and bounded attempts.

## Must Not Carry Over

The rebuild must not preserve any host-owned positive-lane authority.

### No Family Authority

Do not carry over:

- family ids as active planning truth
- family ids in routing decisions
- family ids in fallback contracts
- family ids in patch contracts
- family ids in proof seeding
- family ids in registry selection
- family ids in UI framing
- family ids in prompt context

If old receipts ever need parsing, family metadata may survive only inside an
explicit legacy compatibility bucket and must remain read-only.

### No Supported-Shape Doctrine

Do not carry over:

- supported family catalogs as product center
- nearest-family routing
- scaffold substitution as recovery
- template substitution as recovery
- starter choice as runtime truth
- build-shape guessing disguised as safety

The host must never decide:

- this is close enough to a known artifact family
- this easier scaffold counts as success
- this substitute product shape is probably what the user meant

### No Positive-Lane Patch Authority

Do not carry over:

- duplicate-skip as a dead end for mismatched intent
- stale active `tool_kind` overriding repaired request authority
- old acceptance contracts surviving after behavior changed
- postchecks that only validate file touch or shape continuity
- deterministic patch lane absence being treated as final truth

Patch success must require agreement between:

- source behavior
- acceptance plan
- project spec
- README or operator run instructions
- verification outcome

### No Proof-Template Product Center

Do not carry over into normal runtime/UI:

- proof template pickers
- template scope panels
- comparison bundle panels
- cross-family paired proof panels
- cross-surface proof framing as a standard operator surface
- lane/template/scaffold/operator education panels from the family era

If any historical proof machinery survives temporarily for compile reasons, it
must be explicitly marked legacy or historical and kept out of normal operator
flow.

### No Runtime Museum

Prefer deletion over preservation for:

- old runtime receipts not needed by current tests
- old generated projects
- old library entries
- old migration fixtures
- old family manifests
- old template/scaffold catalogs
- old proof runs
- old fallback artifacts from positive-lane eras

The rebuild should start from clean runtime truth, not layered historical
sediment.

## Rebuild Acceptance Tests

A fresh rebuild is only credible if these conditions hold.

### Contract Acceptance

- new build contracts are substrate/toolchain/capability/evidence driven
- new patch contracts are evidence/constraint/verification driven
- new receipts do not emit active family/starter/template/scaffold authority
- LLM-facing context bundles do not include family/starter/template/scaffold
  steering

### Behavioral Acceptance

- a bounded build can succeed without host-owned family routing
- a bounded patch can repair an existing surface without a special positive lane
- unmatched deterministic patch lanes fall into bounded repair rather than
  clarification theater
- success requires implementation and acceptance to align with frozen request
- failure produces receipts that justify the next attempt mechanically

### UI Acceptance

- normal UI contains no active family/lane/template/scaffold framing
- normal UI does not expose cross-family proof as a default operator feature
- operator-facing result language describes evidence and next-attempt posture,
  not substitute artifact recommendations

### Workspace Acceptance

- no live workspace-family builders remain
- no live family manifests remain
- no active scaffold/template catalogs remain as routing truth
- no stale runtime/build/library fixtures survive unless a current test needs
  them

## Rebuild Audit Checklist

Run these checks on the rebuilt workspace.

### Compile

```powershell
cargo check -p chatty_factory_core
cargo check -p chatty_factory_host
cargo check -p chatty_factory_ui
cargo check -p chatty_factory_cli
```

### Residue Search

```powershell
rg "candidate_family|suggested_family|FamilyId|family_id|family_ids|starter|template|scaffold|catalog|supported family|workspace module|comparison bundle|cross-surface|fallback"
```

Every remaining match must be classified as one of:

- deliberate current doctrine
- explicit legacy read-only compatibility metadata
- harmless historical or documentation reference scheduled for deletion
- runtime-context contamination
- doctrine violation

The target state is that runtime-context contamination and doctrine violations go
to zero.

## Decision Rule

When a rebuild choice is unclear, use this filter:

Keep it only if it is necessary for one of these:

- freezing intent
- bounding execution
- preserving receipts
- classifying failure
- promoting constraints
- verifying behavior
- shaping the next attempt from evidence

Delete it if it exists mainly to:

- remember a favored artifact family
- rescue the run with a scaffold
- preload positive product identity into prompts
- preserve old host-owned catalogs
- make the host feel smarter by guessing shape

## End State

The correct fresh rebuild should feel smaller, stricter, and more honest.

It should be incapable of quietly smuggling positive-lane authority back into:

- planning
- routing
- fallback
- patching
- proof seeding
- prompt context
- UI framing
- receipts

That is the standard for "off the lot" negative-lane ChattyFactory.
