# Negative Void Engine Rebuild Plan

This is the deterministic rebuild plan for the next ChattyFactory architecture.

It exists to keep future Codex windows from solving the hard problem by
recreating the failed prototype shape:

- positive lanes
- supported families
- nearest-template routing
- scaffold substitution
- success language over violated requirements

The rebuild target is a negative-lane factory:

> The user carries intent, the host preserves truth, the model proposes bounded
> method, and every failure becomes structured evidence.

This plan is intentionally strict. If an implementation step needs a product
family catalog to decide what is true, that step has failed.

## 1. Rebuild Doctrine

The new system is not a template picker.

It is not a supported-family router.

It is not a catalog of known app types.

It is not a model-led generator with host-side cleanup.

It is a host-governed void engine that attempts bounded work from frozen
requirements, verifies what happened, and learns from failure without replacing
the user's request with a safer familiar shape.

The operating equation is:

- user owns intent
- host owns truth
- model owns bounded method
- receipts own memory
- verification owns acceptance
- failure owns the next move

The host may use deterministic tools, parsers, formatters, scaffolding
mechanics, and generators. It must not treat any of those as product-family
truth.

## 2. Non-Negotiable Red Lines

Do not add:

- `families`
- `supported_families`
- `templates_by_family`
- `static_web_dashboard`
- `python_cli_tool`
- `rust_cli_tool`
- `nearest_family`
- `fallback_family`
- `starter`
- `known_app_type`
- `happy_path_family`
- positive-lane route tables

Do not implement routing that says:

- this request looks close to family X
- use web because it is safest
- use CLI because it is supported
- use this scaffold because it is near enough
- unsupported stack means choose the closest supported stack

Do not report success when:

- an explicit stack requirement was violated
- the emitted artifact is a substitute
- verification did not check frozen hard requirements
- the model narrated success but receipts do not prove it
- the attempt only produced a scaffold where the request required behavior

Do not let the model:

- decide whether requirements were met
- decide whether a degraded result counts as success
- mutate the constraint shelf directly
- promote broad prohibitions from one failure
- route around hard requirements by choosing a familiar product shape

## 3. Prototype Salvage Map

Carry forward concepts, not structure.

Keep these architectural lessons:

- request records
- intent freeze
- hard requirement tracking
- bounded attempts
- execution work orders
- task receipts
- verification receipts
- normalized failure classes
- recommended next actions
- decomposition receipts
- atomization floor decisions
- failure vault entries
- retry search receipts
- triangulation sessions
- constraint promotion candidates
- approved constraint shelf with mutation history
- operator/runtime visibility

Do not carry forward these prototype behaviors:

- family-owned truth
- family-owned acceptance
- template selection as route authority
- static web as default reality
- CLI/web/native as privileged lanes
- scaffold fallback as product success
- patch lanes tied to named starter artifacts
- route decisions that depend on what the prototype happens to know how to emit

Allowed salvage rule:

- A prototype type may inspire a new contract only if the new contract is
  substrate-neutral and requirement-first.

Blocked salvage rule:

- Do not copy a module whose main job is mapping request shape to a named
  family, starter, or patch lane.

## 4. Target Product Shape

The new factory must process every request through the same spine:

1. intake
2. intent freeze
3. requirement ledger
4. attempt boundary
5. method proposal
6. work-order construction
7. primitive execution
8. verification
9. outcome classification
10. failure classification
11. next-action selection
12. decomposition, retry, triangulation, constraint promotion, or honest block

No step may branch into a positive product family.

The host can still identify artifact properties:

- surface: `web`, `cli`, `desktop`, `service`, `library`, `data`, `document`,
  `unknown`
- stack: user-requested or inferred
- runtime needs: file IO, network, process, browser, database, model, none
- artifact shape: files, commands, entrypoints, tests, schemas, assets

These are descriptors, not families.

The host may say:

- "the request requires a browser-rendered artifact"
- "the request requires a command entrypoint"
- "the request requires a persistent service"
- "the requested stack is Go"

The host must not say:

- "route to static web dashboard"
- "route to Python CLI tool"
- "route to known family"
- "fallback to closest supported lane"

## 5. Core Contracts

Build contracts before behavior.

The first implementation phase must define the contract crate and schema set.
No generation engine should exist until these contracts compile and have tests.

Required contracts:

- `RequestRecord`
- `IntentFreeze`
- `RequirementLedger`
- `Requirement`
- `AttemptSpec`
- `AttemptBoundary`
- `MethodProposal`
- `WorkOrder`
- `PrimitiveStep`
- `ExecutionReceipt`
- `VerificationPlan`
- `VerificationReceipt`
- `OutcomeReceipt`
- `FailureClassification`
- `NextActionDecision`
- `DecompositionGrammar`
- `DecompositionReceipt`
- `AtomizationFloorDecision`
- `FailureVaultEntry`
- `RetrySearchReceipt`
- `TriangulationSession`
- `ConstraintPromotionCandidate`
- `ApprovedConstraint`
- `ConstraintShelfMutation`

Receipt field ownership:

- End-state receipts may carry `outcome_class` and continuation posture.
- Evidence receipts may carry `normalized_failure_class` and continuation
  posture.
- Logs and emitted-artifact receipts stay narrow.
- Do not add fields because they might be useful later.

## 6. Requirement Ledger

The requirement ledger is the first load-bearing part of the rebuild.

Every request must be split into:

- hard requirements
- preferences
- inferred conveniences
- ambiguities
- environmental assumptions

Hard requirements include:

- explicit stack
- explicit runtime
- explicit surface
- explicit output behavior
- explicit IO contract
- explicit deployment or packaging constraint
- explicit non-goals stated by the user

Every hard requirement needs:

- stable id
- source text span or source quote
- requirement kind
- required value
- satisfaction status
- verification strategy
- relaxation policy

Valid satisfaction statuses:

- `unverified`
- `satisfied`
- `violated`
- `partially_satisfied`
- `not_applicable`
- `blocked_by_environment`
- `blocked_by_capability`

Relaxation policy must be one of:

- `never_relax_without_user`
- `may_relax_if_user_prefers`
- `host_inferred_can_drop`

Default for explicit user requirements:

- `never_relax_without_user`

Acceptance gate:

- A final result cannot be `full_success` unless every hard requirement is
  `satisfied` or explicitly `not_applicable`.

## 7. Attempt Boundary

An attempt is not a project family.

An attempt is a bounded claim:

- what artifact will be changed or created
- which requirements it tries to satisfy
- which files or resources it may touch
- which tools it may invoke
- which primitives it may use
- what verification will prove
- when it must stop

Every attempt must include:

- attempt id
- source intent freeze id
- requirement ids in scope
- requirement ids explicitly out of scope
- allowed paths
- denied paths
- allowed commands
- denied commands
- expected artifact set
- verification plan id
- timeout and retry budget
- model budget, if model use is allowed

Attempt scope must be small enough that failure is informative.

If failure would not teach anything precise, the attempt is too broad.

## 8. Primitive Execution Vocabulary

The new engine needs primitives, not families.

Initial primitive classes:

- create file
- update file by structured patch
- update JSON
- update TOML
- update YAML
- update Markdown
- update HTML DOM
- update CSS rule
- update source file through parser or formatter-backed rewrite
- copy asset
- run command
- inspect command output
- render artifact
- validate schema
- run test
- run lint
- package output
- write receipt

Each primitive must declare:

- input contract
- output contract
- allowed file scope
- reversibility or backup behavior
- verification hook
- failure modes

Primitive rule:

- A primitive may know file formats.
- A primitive may know tool syntax.
- A primitive must not know product families.

## 9. Model Boundary

The model may propose method only inside a frozen attempt.

Allowed model tasks:

- propose a work-order sketch from frozen requirements
- fill a bounded file body when exact generation is required
- propose decomposition options
- explain ambiguous failure evidence
- suggest retry posture
- produce code for a single bounded primitive step

Disallowed model tasks:

- decide final success
- choose a positive family
- waive hard requirements
- promote constraints
- invent verification results
- modify receipts directly
- broaden scope after failure

Every model output must be captured as:

- prompt
- raw response
- parsed proposal
- parser status
- host review status
- accepted fields
- rejected fields

Host review must convert model output into typed contracts before execution.

## 10. Verification

Verification must be planned before execution.

Verification can include:

- file existence
- schema validation
- syntax check
- build check
- test command
- runtime smoke check
- browser render check
- CLI output check
- API response check
- snapshot comparison
- requirement-specific assertion

Verification receipts must say:

- what was checked
- which requirement ids were checked
- what command or tool ran
- observed output
- pass/fail
- failure evidence
- normalized failure class if failed
- recommended next action if resumable

Verification cannot rely on:

- model confidence
- generated README claims
- presence of plausible files
- scaffold existence alone

## 11. Outcome Taxonomy

End-state surfaces must classify what happened honestly.

Required outcome classes:

- `full_success`
- `partial_success`
- `degraded_fallback`
- `substituted_stack`
- `requirement_not_met`
- `blocked_pending_clarification`
- `blocked_pending_retry`
- `blocked_pending_decomposition`
- `blocked_pending_triangulation`
- `blocked_by_environment`
- `blocked_by_capability`

Outcome classification must derive from:

- requirement ledger
- attempt receipt
- verification receipt
- failure classification
- explicit user-approved relaxations

Rule:

- If stack changed without user approval, outcome is `substituted_stack`.
- If hard behavior is missing, outcome is `requirement_not_met` or
  `partial_success`.
- If only a scaffold exists, outcome is not `full_success`.
- If verification could not run, outcome is blocked or partial, not full.

## 12. Failure Taxonomy

Failure classes must drive next actions mechanically.

Required normalized failure classes:

- `requirement_ambiguity`
- `task_too_broad`
- `attempt_boundary_invalid`
- `model_output_format_failure`
- `model_method_failure`
- `tool_invocation_failure`
- `missing_dependency_or_toolchain`
- `environment_instability`
- `contract_mismatch`
- `verification_failure`
- `syntax_failure`
- `build_failure`
- `runtime_failure`
- `permission_boundary_violation`
- `artifact_incomplete`
- `requirement_violation`
- `atomization_floor_reached`
- `method_space_exhausted`
- `capability_gap`
- `unknown`

Each class must map to a default next action.

Do not create a new failure class until:

- existing classes are insufficient
- at least one receipt demonstrates the distinction
- the distinction changes the next action

## 13. Next-Action Selector

The next-action selector is the central negative-lane engine.

Inputs:

- failure class
- requirement ledger
- attempt boundary
- verification evidence
- retry history
- decomposition history
- triangulation history
- constraint shelf
- environment state

Outputs:

- action id
- reason
- next attempt posture
- required evidence
- stop condition

Required actions:

- `clarify_with_user`
- `retry_same_boundary_different_method`
- `retry_with_stricter_output_contract`
- `repair_toolchain`
- `decompose_task`
- `stop_decomposition_at_floor`
- `escalate_model`
- `triangulate_failure`
- `vault_failure`
- `propose_constraint`
- `block_honestly`

Selector rule:

- The next action must be derived from receipt evidence, not from a preferred
  artifact shape.

## 14. Decomposition

Decomposition must use reusable grammars.

Initial grammar ids:

- `artifact_inventory_split`
- `file_creation_split`
- `host_mechanical_vs_model_authored_split`
- `schema_logic_render_split`
- `parse_transform_emit_split`
- `backend_frontend_contract_split`
- `io_core_presentation_split`
- `toolchain_setup_then_behavior_split`
- `verification_repair_split`
- `patch_diagnosis_application_postcheck_split`

A decomposition receipt must record:

- parent attempt id
- selected grammar id
- reason for selection
- child attempts
- requirement ids per child
- floor assessment
- expected learning value

Do not decompose just to keep working.

Decompose only when the child attempts make failure more informative.

## 15. Atomization Floor

The atomization floor prevents infinite splitting.

A task is at the floor when further split would remove one of:

- coherent artifact meaning
- independently verifiable behavior
- useful failure signal
- requirement traceability

At floor, the system may:

- retry differently
- escalate model
- triangulate
- vault
- block honestly

At floor, the system must not:

- split further
- invent a smaller positive family
- replace the requirement with a scaffold

## 16. Retry Search

Retry search is bounded evidence collection.

Retry dimensions:

- prompt posture
- output contract strictness
- parser recovery
- primitive choice
- toolchain repair
- model candidate

Retry search must record:

- attempts
- changed variable per attempt
- unchanged boundary
- observed result
- failure class
- stopping reason

Retry rule:

- Change one major variable at a time when possible.
- Do not treat a successful different artifact as success unless requirements
  are still satisfied.

## 17. Failure Vault

The failure vault is live memory, not a junk drawer.

Vault entries must include:

- failure signature
- requirement ids
- attempt boundary
- failure class
- observed evidence
- tried methods
- remaining hypotheses
- resumption posture

When a similar failure occurs, the host must search the vault before starting a
fresh loop.

Similarity should consider:

- requirement kind
- artifact shape descriptors
- primitive class
- failure class
- verification failure surface
- toolchain
- model candidate

## 18. Triangulation

Triangulation narrows culprits across failures.

It compares:

- same requirement, different method
- same method, different model
- same primitive, different artifact
- same toolchain, different boundary
- same verification failure, different generated content

Triangulation outputs:

- converged culprit signature
- confidence level
- scope boundary
- disproven hypotheses
- recommended next action

Promotion rule:

- A broad failure cluster is not a constraint.
- Only a narrow converged culprit can become a promotion candidate.

## 19. Constraint Shelf

The constraint shelf stores earned negative constraints.

Constraints must be:

- narrow
- evidence-linked
- scoped
- reversible
- reviewable
- non-punitive

Every approved constraint must include:

- constraint id
- exact condition
- prohibited action
- reason
- source triangulation ids
- source vault ids
- replacement guidance
- activation status
- expiry or review policy

Do not approve constraints from:

- one-off failures
- model-specific weakness only
- vague distrust
- broad product category
- unsupported family absence

## 20. Repository Shape For Rebuild

Start a clean v2 workspace rather than cutting the current host monolith apart
first.

Recommended initial crates:

- `chatty_void_core`
- `chatty_void_runtime`
- `chatty_void_verify`
- `chatty_void_primitives`
- `chatty_void_host`
- `chatty_void_cli`

Do not add UI until:

- contracts are stable
- receipts are readable
- CLI can run deterministic probes
- failure loops persist evidence correctly

Suggested crate ownership:

- `core`: contracts, ids, enums, requirement ledger
- `runtime`: paths, receipt persistence, vault lookup, shelf loading
- `verify`: verification plan execution and requirement assertions
- `primitives`: substrate-neutral file/tool primitives
- `host`: orchestration, selector, decomposition, retry, triangulation
- `cli`: thin command surface only

Dependency direction:

- CLI depends on host.
- Host depends on core, runtime, verify, primitives.
- Runtime depends on core.
- Verify depends on core.
- Primitives depend on core.
- Core depends on serde and narrow utilities only.

## 21. Implementation Phases

### Phase 0. Create The Clean Workspace

Deliverables:

- new v2 workspace or branch-local crate set
- no family crate
- no template registry
- no current host monolith dependency
- strict lint/test command documented

Acceptance:

- `cargo test` passes
- searching for red-line terms returns no active routing implementation

### Phase 1. Contracts And Schemas

Deliverables:

- all core contracts
- JSON schema snapshots if useful
- receipt field ownership tests
- serialization round-trip tests

Acceptance:

- every receipt has a declared ownership bucket
- no end-state/evidence/log field mixing without explicit test coverage

### Phase 2. Requirement Ledger

Deliverables:

- deterministic request intake for explicit requirements
- hard/preference/inferred split
- requirement satisfaction status
- relaxation policy

Acceptance probes:

- "build a Go CLI todo app" preserves Go as hard stack
- "make something like a dashboard" records ambiguity instead of selecting web
- "do not use Python" records a negative hard requirement

### Phase 3. Attempt Boundary And Work Orders

Deliverables:

- attempt spec
- allowed/denied paths
- allowed/denied commands
- work-order construction
- receipt persistence

Acceptance:

- an attempt can be created without selecting a family
- too-broad attempts are rejected with `task_too_broad`

### Phase 4. Primitive Executor

Deliverables:

- create/update/copy/run/validate primitives
- primitive failure receipts
- path boundary enforcement

Acceptance:

- primitive tests prove boundaries
- primitives do not import host route logic
- primitives do not know product families

### Phase 5. Verification Engine

Deliverables:

- verification plans
- requirement-specific assertions
- command checks
- schema checks
- syntax/build smoke hooks where available

Acceptance:

- full success is impossible without requirement verification
- missing toolchain reports `missing_dependency_or_toolchain`

### Phase 6. Outcome And Failure Classification

Deliverables:

- outcome classifier
- failure classifier
- selector input bundle
- tests for substituted stack and unmet requirement

Acceptance:

- Go requested and JavaScript emitted becomes `substituted_stack`
- scaffold-only result cannot become `full_success`

### Phase 7. Next-Action Selector

Deliverables:

- failure class to action mapping
- selector receipt
- stop conditions

Acceptance:

- selector never consults product family names
- selector action changes when failure class changes

### Phase 8. Decomposition And Atomization Floor

Deliverables:

- grammar library
- decomposition receipts
- floor decisions

Acceptance:

- broad task decomposes through grammar id
- floor-level task refuses further split
- floor routes to retry, triangulate, vault, or block

### Phase 9. Retry Search

Deliverables:

- bounded retry sessions
- posture changes
- model candidate hooks
- retry receipts

Acceptance:

- retry changes one major variable when possible
- method exhaustion is distinguished from model exhaustion

### Phase 10. Vault And Triangulation

Deliverables:

- failure signature matching
- resumable vault entries
- triangulation sessions
- culprit narrowing

Acceptance:

- repeated related failure resumes prior evidence
- triangulation can reject broad promotion

### Phase 11. Constraint Promotion

Deliverables:

- promotion candidates
- approval gate
- shelf mutation trail
- constraint matching

Acceptance:

- one-off failure cannot become approved constraint
- approved constraint links to triangulation/vault evidence

### Phase 12. CLI Operator Surface

Deliverables:

- `intake`
- `freeze`
- `attempt`
- `verify`
- `classify`
- `next`
- `decompose`
- `retry`
- `vault`
- `triangulate`
- `constraints`

Acceptance:

- each command emits or reads receipts
- no command hides a family route

## 22. Codex Window Instructions

Every future Codex session working from this plan must do this first:

1. Read this file.
2. Read current architecture only as prototype context.
3. Search for red-line terms before editing.
4. State which phase is being worked.
5. State which red lines are relevant to the phase.
6. Implement the smallest phase-complete slice.
7. Add tests for the phase acceptance probes.
8. Run verification.
9. Report any temptation to add a positive lane as a design failure, not as a
   missing convenience.

If a Codex session proposes a family, starter, or supported app type, stop that
line of work.

If a Codex session says "for now we can route unknown requests to static web,"
stop that line of work.

If a Codex session says "we need templates for each supported output type,"
replace that with primitives, descriptors, and requirement-specific
verification.

## 23. Global Acceptance Probes

These probes must remain in the suite.

### Explicit Unsupported Stack

Request:

```text
Build a small Go CLI that reads a todo.txt file and prints overdue tasks.
```

Expected:

- Go is preserved as hard requirement.
- No fallback to Python, Rust, or web.
- If Go toolchain is missing, outcome is `blocked_by_environment`.
- If Go generation fails, failure routes through retry/decomposition.

### Negative Requirement

Request:

```text
Build a local report tool, but do not use Python.
```

Expected:

- `do_not_use_python` is a hard requirement.
- Python output is a requirement violation.
- No success can be reported for Python output.

### Vague Request

Request:

```text
Make me something useful for tracking work.
```

Expected:

- ambiguity is recorded.
- host may ask clarification or freeze a narrow exploratory attempt.
- host must not choose dashboard because dashboard is familiar.

### Scaffold Trap

Request:

```text
Build a working expense tracker that imports CSV and calculates monthly totals.
```

Expected:

- empty UI shell is not success.
- CSV import and monthly total behavior must be verified or marked unmet.

### Repeated Failure

Request:

```text
Build a parser for this malformed input format.
```

Expected:

- repeated parser failures enter vault.
- retry history is preserved.
- triangulation narrows culprit before constraint promotion.

## 24. Definition Of Done

The rebuild is not done when it can generate a nice artifact.

It is done when it can:

- preserve hard requirements
- attempt bounded work without family routing
- verify against frozen requirements
- classify outcomes honestly
- classify failures generically
- select next actions from evidence
- decompose through reusable grammars
- stop at an atomization floor
- retry under bounded search
- resume from failure memory
- triangulate repeated failures
- promote only narrow earned constraints
- block honestly when it cannot proceed

The success condition is not optimism.

The success condition is receipt-owned truth under uncertainty.

