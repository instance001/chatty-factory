# ChattyFactory Rebuild Plan

Historical note:
- this is a planning-era execution document
- use [../../docs/CURRENT_ARCHITECTURE.md](../../docs/CURRENT_ARCHITECTURE.md) for the current implementation shape

This document is the working plan for rebuilding ChattyFactory inside `chatty-factory/` as a cleaner standalone product that can also emit a drop-in ChattyCog module wrapper.

It is not a retrospective on the old prototype. It is the execution document for replacing the prototype's most entangled LLM-driven machinery with deterministic host-owned systems wherever practical.

Read this alongside:
- `../ARCHITECTURE_LEDGER.md`
- `../PARTS_MAP.md`
- `../RELIABILITY.md`
- `../GRAPH_CONTROL_PLANE_SCOUT.md`
- `../DOM_QUERY_SCOUT.md`
- `../JSON_CONTRACT_SCOUT.md`
- `../MINIJINJA_SCOUT.md`

## Product Goal

Plain-language request in.
Real working build out.
Plain-language follow-up patch requests after that.

The rebuild should optimize for:
- local/offline operation
- high inspectability
- deterministic execution
- bounded retries
- minimal LLM surface area inside exact machinery
- substrate agnosticism wherever deterministic host machinery can support it
- standalone output first
- optional ChattyCog compatibility second

## Rebuild Thesis

The prototype proved the most important architectural split:
- the local LLM should choose, triage, and review
- the host should assemble, edit, validate, execute, and classify

The rebuild should therefore move the LLM upward into:
- request interpretation
- route choice when ambiguity is real
- capability tradeoff explanation
- quality-control review on machine outputs
- bounded fallback planning for unsupported cases

The rebuild should move exact machinery downward into:
- templates
- DOM operators
- AST/code operators
- JSON patchers
- schemas
- acceptance builders
- helper generators
- failure classifiers
- route graphs
- deterministic executors

## Non-Goals

The rebuild is not trying to:
- preserve prototype prompt structure as a legacy contract
- keep broad freeform planning as the default path
- make the LLM author all implementation code for common families
- support every UI stack equally well on day one
- chase maximum flexibility at the cost of auditability

It is also not trying to:
- remain web-only just because the prototype had to collapse there for GGUF survival

## North-Star Behavior

For common requests, the ideal flow is:
1. User makes a plain-language request.
2. Host classifies build vs patch from explicit UI state.
3. Host extracts structured request facts into native data.
4. LLM only resolves genuine ambiguity or confirms the likely route.
5. Host chooses a deterministic family or operator lane.
6. Host emits scaffold, helper files, wrapper files, contracts, and acceptance.
7. Host runs checks and classifies failures.
8. LLM only reviews when route choice or retry judgment is still needed.
9. User receives a real working build or an honest, typed failure state.

For uncommon requests, broad planning remains a fallback, not the emotional default.

## Product Stance On Build Agnosticism

The rebuild should aim to move back toward agnostic builds as far as host-owned machinery can take us.

That means:
- web should remain the strongest early lane
- web should not remain the permanent identity of the system
- Python, Rust, Node, and other bounded families should come back as first-class routes when the host can own enough of their machinery

The prototype collapsed toward webview-heavy output mostly because:
- local GGUFs struggled with broad multi-file authorship
- browser scaffolds were easier to stabilize
- the host had more deterministic leverage there

The rebuild changes that equation by moving exact work away from the model.

So the new stance should be:
- be substrate-agnostic in product ambition
- be substrate-ranked in implementation maturity

In practice:
- choose the easiest honest family for the request
- prefer mechanical family lanes over web coercion
- only pool a request to web when web is actually the smallest truthful fit
- do not hide unsupported native-family gaps by silently reinterpreting everything as a browser app

## Substrate Strategy

The rebuild should treat build substrates as ranked families, not as one default plus awkward exceptions.

Likely early substrate groups:
- static web
- helper-backed web
- Python CLI
- Rust CLI
- Node CLI
- later bounded native desktop families

For each substrate, the host should increasingly own:
- project templates
- file layouts
- standard entrypoints
- config/manifests
- syntax/build checks
- family-specific acceptance
- repair lanes for common failures

This is the key mechanism that lets agnostic builds come back without returning to "LLM writes everything."

## Family Selection Policy

The control plane should choose families by honest fit, not just by what used to be easiest for the model.

Selection order should consider:
- explicit user stack request
- required capabilities
- active project substrate for patching
- availability of deterministic family machinery
- acceptance confidence
- expected repairability if something fails

Example policy:
- if the user explicitly asks for a Python CLI and the `python_cli_tool` family is supported, build Python CLI
- if the user explicitly asks for Rust and the `rust_cli_tool` family is supported, build Rust
- if the user asks for an agnostic "tool" and the smallest honest fit is CLI, do not force web
- if the user asks for browser-style interactivity, web remains a strong default
- if the user asks for local-native capability plus a browser face, route to helper-backed web instead of pretending plain web can do it

Web should remain a strong route, not a distortion field around every request.

## Core Architecture

The rebuild should be organized into six major layers.

### 1) Request Normalizer

Purpose:
- convert plain language plus UI state into a typed request record

Native outputs should include:
- request mode: `new_build` or `patch`
- desired surface: `web`, `cli`, `desktop`, `helper_web`, `unknown`
- explicit stack constraints
- requested capabilities
- exoskeleton target
- confidence / ambiguity markers
- candidate family ids
- candidate operator ids

LLM involvement:
- optional
- only when ambiguity cannot be resolved from state and deterministic heuristics

### 2) Control Plane

Purpose:
- represent route selection and retry flow as a visible typed graph

Likely substrate:
- `petgraph` first

Responsibilities:
- route selection
- capability-gated transitions
- alternate lane selection
- retry limits
- stop-success and stop-fail nodes
- route explanation receipts

This should replace scattered branching with explicit nodes and edges.

### 3) Operator Engine

Purpose:
- perform exact build/patch machinery mechanically

Responsibilities:
- scaffold emission
- DOM structure edits
- code-family scaffold emission
- AST-aware or structured source edits
- JSON contract edits
- bounded behavior insertion
- helper bridge insertion
- wrapper emission
- metadata updates

This is the main layer where the prototype's LLM-written implementation should disappear.

### 4) Verification Plane

Purpose:
- define and execute deterministic proof of success

Responsibilities:
- family-specific acceptance builders
- schema validation
- syntax/build checks
- helper self-tests
- output token checks
- file existence/content checks
- route-specific final QC receipts

### 5) Failure Intelligence

Purpose:
- classify failures before any model sees them

Responsibilities:
- parse logs
- match known failure classes
- route to repair lanes
- stop impossible retries early
- distinguish unsupported capability from transient failure

Likely substrate:
- `regex_automata`
- later selective `tree-sitter` lanes

### 6) LLM Foreman

Purpose:
- stay above the machinery

Allowed jobs:
- clarify ambiguous intent
- choose among plausible routes
- explain tradeoffs
- review machine outputs
- greenlight or redlight a retry
- generate fallback plans only when no bounded lane exists

Disallowed default jobs:
- writing routine scaffolds
- hand-authoring common wrapper files
- editing JSON manifests freehand
- string-surgery on HTML structure
- inventing acceptance from scratch for known families
- freeform diagnosis when typed failure classes already exist

## Standalone And ChattyCog Positioning

The rebuild should treat standalone output as the real product.

ChattyCog compatibility should be:
- optional
- deterministic
- emitted from templates/operators
- never the only product shape

Rule:
- every ChattyCog-compatible output must also make sense as a standalone project
- ChattyCog files are wrappers and integration artifacts, not the project's whole identity

## Role Mechanization Matrix

This section maps prototype roles to rebuild targets.

### `PROJECT_BLUEPRINT`

Current value:
- narrow project shape early

Rebuild direction:
- keep concept
- shrink into `request normalizer + route selector`

Target split:
- host extracts candidate families and capabilities
- LLM only arbitrates if multiple plausible shapes remain

Status:
- keep as hybrid, but mostly mechanized

### `CODE_FILE`

Current value:
- lets small models focus on one file

Rebuild direction:
- remove as default for known families

Replacement:
- `minijinja` scaffold templates
- deterministic behavior fillers
- helper generators
- operator bundles
- family-specific code emitters

Status:
- mechanize for all standard families
- retain only as fallback for unsupported novel work

### `PATCH_TARGET`

Current value:
- choose file from bounded set

Rebuild direction:
- heavily reduce

Replacement:
- project snapshot
- contract metadata
- operator eligibility checks
- DOM/JSON/AST-aware file routing

Status:
- hybrid at first
- target full mechanization for common lanes

### `PATCH_INTENT`

Current value:
- separate what from how

Rebuild direction:
- keep

Reason:
- intent selection is still a good foreman-style model job

Status:
- retain as one of the main LLM jobs

### `PATCH_FILE`

Current value:
- exact diff hunk generation

Rebuild direction:
- demote hard

Replacement:
- DOM operators
- JSON patchers
- template re-renders
- later AST-based structural edits

Status:
- fallback only

### `PATCH_ACCEPTANCE`

Current value:
- tie patch to definition of done

Rebuild direction:
- mostly mechanize

Replacement:
- family-specific acceptance builders
- operator-contributed acceptance tokens
- helper-kind acceptance packs

Status:
- mechanize for standard families and operators
- LLM only supplements when route is novel

### `RECOVERY`

Current value:
- bounded repair steps

Rebuild direction:
- reduce to final fallback

Replacement:
- failure classifier
- repair lane selection
- deterministic retry recipes

Status:
- keep only for unresolved unknown-class failures

### `BUILD_REPAIR`

Current value:
- focused fix generation

Rebuild direction:
- split by repair lane

Replacement:
- typed repair actions per family and failure class

Status:
- mechanize standard classes
- keep LLM lane only for unclassified failures

### `RECOVERY_DONE`

Current value:
- cheap decision after retry

Rebuild direction:
- keep concept, mechanize signal

Replacement:
- typed retry outcome from verification plane

Status:
- machine first, LLM only if evidence is mixed

### `CLARIFY`

Current value:
- ask one useful question

Rebuild direction:
- keep

Reason:
- this remains one of the highest-value model jobs

Status:
- retain

### `BUILD_SPEC`

Current value:
- structured handoff/spec text

Rebuild direction:
- narrow

Replacement:
- native family descriptors
- operator manifests
- helper manifests
- route receipts

Status:
- fallback only for unsupported or genuinely custom work

## Deterministic Families To Build First

The rebuild should not try to make every family equally good immediately.

Start with the highest-leverage substrate and build outward from there.

### Tier 1 Families

- `static_web_tool`
- `static_web_dashboard`
- `static_web_helper_foundation`
- `chattycog_basic_dashboard`

Reason:
- strongest fit with current proven path
- easiest place to remove LLM code authorship
- best overlap with ChattyCog module compatibility

Important note:
- Tier 1 is the opening wedge, not the final worldview
- this tier exists because it is the fastest way to prove the host-owned architecture

### Tier 2 Families

- `python_cli_tool`
- `node_cli_tool`
- `rust_cli_tool`

Reason:
- bounded and deterministic
- useful for utility asks
- simpler acceptance surface than desktop GUI
- strong candidates for reclaiming substrate agnosticism once templates/contracts/checks exist

### Tier 3 Families

- `python_tkinter_dashboard`
- additional native desktop niches

Reason:
- still useful
- lower priority than the web-first deterministic substrate

## First-Class ChattyCog Starter Family

This should be a real family, not a prompt trick.

Proposed family id:
- `chattycog_basic_dashboard`

Purpose:
- produce a starter base that works standalone
- emit the standard ChattyCog wrapper files and bridge wiring
- prefill the minimum files most new module builds repeatedly need

Expected outputs:
- base dashboard scaffold
- `manifest.json`
- `visual_load.json`
- `HANDSHAKE.md`
- bridge JS file
- starter `index.html`
- starter `app.js`
- starter `styles.css`
- project contract
- acceptance pack

Default operator bundle:
- metric cards
- status panel
- action toolbar
- results/data panel

Default acceptance:
- standalone page loads structurally
- required wrapper files exist
- manifest/schema validation passes
- bridge file reference is present

This family should become the standard starting point for:
- "make me a basic ChattyCog module"
- "start a dashboard module"
- "make a basic module base and we will patch it up from there"

## External Tooling Adoption Plan

These are the most promising external parts for the rebuild.

### Adopt Early

- `minijinja`
  - scaffold templates
  - helper templates
  - wrapper templates

- `json-patch`
  - contract and manifest edits

- `jsonschema`
  - schema-backed validation for contracts and metadata

- `regex_automata`
  - failure classification
  - route heuristics

### Adopt Soon After

- `dom_query`
  - structural HTML operator engine
  - helper bridge DOM insertion
  - selector-based eligibility

- `petgraph`
  - visible typed route graph
  - retry transitions
  - route receipts

### Adopt Selectively Later

- `tree-sitter`
  - AST-aware patch lanes for Python/Rust/JS

- `notify`
  - live state refresh
  - runtime/output watches

## Tooling Adoption Policy

The rebuild should actively prefer free/open host-side tools when they remove a real class of fragile LLM work.

Adoption rule:
- if a tool lets the host perform structure, validation, patching, classification, rendering, or verification more reliably than the local model, it should be considered a first-class candidate

Preferred tool qualities:
- offline-friendly
- scriptable
- deterministic
- permissive or otherwise usable licensing
- good enough maintenance health
- narrow enough scope to wrap cleanly

We should not add tools just because they are impressive.
We should add them when they let ChattyFactory:
- reduce freeform model authorship
- reduce repair entropy
- increase inspectability
- widen honest substrate coverage

This means the rebuild is explicitly allowed to pull in:
- Rust crates
- CLI utilities
- parsers
- schema validators
- templating engines
- source/DOM/JSON manipulation tools

when doing so helps move the LLM out of exact machinery and back into foreman/QC work.

## Native Data Contracts

The rebuild should define explicit machine-owned artifacts from the start.

Likely early contracts:
- `RequestRecord.json`
- `RouteDecision.json`
- `ProjectFamily.json`
- `OperatorSelection.json`
- `AcceptancePlan.json`
- `FailureReport.json`
- `ProjectSpec.json`

Likely near-term family metadata:
- `FamilyCapabilities.json`
- `FamilyRepairLanes.json`
- `ScaffoldInputs.json`

Design rules:
- machine-writable
- schema-validated where practical
- easy to diff
- stable ids
- no hidden semantics trapped only in prompts

## Acceptance Strategy

Acceptance should increasingly be built from family and operator knowledge, not improvised by the model.

Each family should contribute:
- required files
- required references
- syntax/build checks
- runtime smoke checks
- feature tokens
- substrate-specific entrypoint checks
- family repair hints

Each operator should contribute:
- structural assertions
- expected ids/classes/content markers
- optional behavior markers

Each helper kind should contribute:
- helper script existence
- bridge wiring checks
- helper self-test
- expected data file contract

The LLM should only add acceptance when:
- a request is outside the known family/operator system
- a human explicitly asks for custom success criteria

## Failure Classification Strategy

The rebuild should classify first and only then retry.

Early failure classes should include:
- invalid metadata/schema
- missing expected files
- DOM/operator eligibility mismatch
- unsupported family capability
- substrate toolchain mismatch
- structured code generation mismatch
- helper wiring failure
- syntax failure
- build failure
- runtime assertion failure
- unsupported capability for current family
- route selection mismatch
- unknown

Each class should map to:
- stop immediately
- retry same lane with deterministic fix
- switch lane
- ask human
- ask LLM foreman for judgment

## Migration Strategy

This is not a one-shot rewrite. Build the rebuild in slices.

### Phase 0: Foundation

Deliverables:
- repository layout under `chatty-factory/`
- crate/app structure
- base docs
- configuration model
- core type definitions

Exit criteria:
- clear module boundaries exist
- no prototype code copied blindly

### Phase 1: Request And Route Core

Deliverables:
- request normalizer
- control-plane graph skeleton
- route receipts
- basic family registry

Exit criteria:
- requests can resolve to typed candidate families without broad prompts

### Phase 2: Template And Contract Core

Deliverables:
- `minijinja` scaffold rendering
- JSON patch + schema validation
- standalone and ChattyCog wrapper templates

Exit criteria:
- standard wrappers and metadata can be emitted without LLM-authored text

### Phase 3: Static Web Family

Deliverables:
- base web scaffold family
- bounded behavior filler lane
- family-specific acceptance

Exit criteria:
- common web-tool asks can build through a mostly mechanical path
- the family format is strong enough to be reused by non-web families

### Phase 4: DOM Operator Engine

Deliverables:
- `dom_query` structural operators
- selector eligibility checks
- patch receipts

Exit criteria:
- common structural patch requests stop depending on freeform HTML rewrites

### Phase 5: Helper Foundations

Deliverables:
- helper kinds
- helper templates
- bridge operators
- helper acceptance packs

Exit criteria:
- capability-gated browser requests can transition honestly to helper foundations

### Phase 6: ChattyCog Starter Family

Deliverables:
- `chattycog_basic_dashboard`
- prefilled module files
- acceptance pack

Exit criteria:
- starter module asks become one deterministic build lane

### Phase 7: CLI Families

Deliverables:
- Python CLI family
- Node CLI family
- Rust CLI family
- family-specific scaffold inputs
- family-specific acceptance and repair packs

Exit criteria:
- utility asks no longer require broad planning in normal cases
- explicit non-web requests can take honest non-web routes without falling back to web coercion

### Phase 7.5: Structured Source Editing

Deliverables:
- targeted source-edit lanes for Python/Rust/JS
- AST-aware or similarly structured patch utilities where worth it
- family-safe patch routing for non-web projects

Exit criteria:
- patching non-web families is no longer dependent on freeform diff-hunk generation in common cases

### Phase 8: Reduced LLM Fallback Surface

Deliverables:
- final fallback planner lane
- LLM QC loop
- failure-class-based retry decisions

Exit criteria:
- broad model-authored build logic is clearly exceptional, not mainstream

## Initial Repository Shape

The rebuild folder should likely grow into something like:

- `chatty-factory/README.md`
- `chatty-factory/REBUILD_PLAN.md`
- `chatty-factory/docs/`
- `chatty-factory/crates/`
- `chatty-factory/templates/`
- `chatty-factory/schemas/`
- `chatty-factory/operator_registry/`
- `chatty-factory/families/`
- `chatty-factory/examples/`

Exact crate names can be decided later, but the separation should reflect architecture, not prototype history.

## Design Rules

- Prefer native typed data over prompt-only state.
- Prefer family/operator ids over vague prose.
- Prefer deterministic receipts over hidden decisions.
- Prefer route explanation artifacts over silent fallback.
- Prefer one honest capability transition over bluffing through the wrong substrate.
- Prefer fallback scarcity.
- Prefer standalone output first, wrapper output second.
- Prefer adding a new operator/family over expanding prompt complexity.
- Prefer honest substrate choice over reflexive web pooling.
- Prefer host-supported non-web families whenever they are mature enough to be truthful and repairable.

## Open Questions To Resolve Early

- How many crates should the rebuild use at the start?
- Do we want one GUI immediately, or first ship a core plus minimal shell?
- Which family registry format should be data-driven vs hardcoded Rust?
- Which contracts deserve schema validation on day one?
- How much of ChattyCog wrapper metadata should be emitted from templates versus JSON patch transforms?
- When should `tree-sitter` enter the patch stack?
- How much of the current bookshelf survives as-is versus becoming structured manifests and examples?

## First Buildable Milestone

The first milestone should be small but meaningful:

Input:
- plain-language request for a basic dashboard/module

Output:
- standalone static web dashboard
- optional ChattyCog wrapper files
- project contract
- deterministic acceptance
- route receipt

No broad code-writing LLM lane should be required for that milestone beyond route choice and QC.

After that milestone, the next major proof point should be:
- one Python CLI family
- one Rust CLI family or Node CLI family

The rebuild should prove early that the architecture can expand outward from web, not merely perfect web.

## Immediate Next Tasks

Before implementation starts, we should produce:
1. A concrete folder/crate layout for `chatty-factory/`.
2. A type-and-contract inventory for the first milestone.
3. A family spec for `static_web_dashboard`.
4. A family spec for `chattycog_basic_dashboard`.
5. A shortlist of schemas and templates to create first.
6. A route graph sketch for the first supported build lanes.

That should give us enough shape to begin building without dragging prototype spaghetti into the new codebase.
