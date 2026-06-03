# Negative Constraint Shelf Milestone

## Why This Milestone Exists

ChattyFactory has now proved something important:

- deterministic build can work
- deterministic patch can work
- diagnosis-aware patch surgery can work
- self-review can refine and even modernize stale patch intent

But this also exposes the scaling problem with a purely positive route:

- if every new capability must be represented as one more narrow "correct"
  patch lane, family lane, or glue path
- the system risks turning into hundreds of overly specific positive lanes
- and that does not really solve language-agnostic or codebase-agnostic work

The model is often already good enough at the positive side:

- what capability the user wants
- what kind of code change is probably needed
- what family of implementation is plausible

Where it more often fails is on the negative side:

- exact insertion shape
- local syntax integrity
- block/bracket structure
- stale splice points
- framework-specific anti-patterns
- "looks plausible but is wrong here" implementations
- patching an evolved codebase with a legacy method

So the next scaling move is not to enumerate every right answer.

It is to grow a host-owned shelf of:

- implementation methods that should not be used
- structural shapes that should block or redirect execution
- framework/language/codebase anti-patterns that are locally testable

This is the beginning of a negative constraint architecture.

## Core Idea

Instead of treating the system as:

- request
- choose the perfect positive lane
- hope the glue holds

the system should increasingly act like:

1. request interpretation proposes the positive goal
2. diagnosis describes the project and surgical surface
3. self-review proposes a candidate implementation path
4. the constraint shelf removes impossible or unsafe methods
5. the host executes only inside the surviving bounded space

That means the model remains useful for broad intent and code reasoning, while
the host increasingly owns the "do not do it this way" boundary.

## Architectural Thesis

The fastest path toward more language-agnostic and code-agnostic capability is:

- not "teach ChattyFactory every correct implementation recipe"
- but "teach ChattyFactory more of the specific wrong methods it must reject"

This does not duplicate the model's native training.

It complements it by encoding:

- local structural impossibilities
- known stale insertion methods
- family/framework anti-patterns
- project-specific temporary no-go zones

## Constraint Shelf Model

The shelf should become a host-owned contract surface, something like:

- `ImplementationConstraint`
- `ConstraintViolation`
- `ConstraintScope`
- `ConstraintReviewReceipt`

The shelf is intended to be:

- explicit
- inspectable
- testable
- explainable
- incrementally expandable from real failures

It should not become vague advice.

Bad:

- `avoid messy code`

Good:

- `do not add a Rust CLI flag through legacy text splice shape once the parser block already declares json_out and markdown_out`
- `do not mutate helper-summary legacy status markup when helper-summary status-chip markers are already present`
- `do not insert JSX siblings outside the returned root`
- `do not append duplicate JS listeners inside rerendered update paths`
- `do not edit contract files without updating paired acceptance or manifest surfaces`

## Constraint Scopes

The shelf should support at least four scopes.

### 1. Universal Constraints

Cross-language, cross-family anti-patterns that are broadly true.

Examples:

- do not edit outside the diagnosed surgical file set
- do not mutate the project without preserving declared contract files
- do not proceed if self-review empties the target surface

### 2. Language Constraints

Language-specific "how not to" rules.

Examples:

- Rust:
  - do not splice parser blocks by stale text shape
  - do not modify match arms without preserving exhaustiveness
- Python:
  - do not shadow imported names through duplicate local helper insertion
  - do not patch import blocks by blind append that changes ordering semantics
- JavaScript/TypeScript:
  - do not duplicate event listener wiring inside rerendered code paths
  - do not insert DOM accesses before the owning surface exists

### 3. Family / Framework Constraints

Constraints tied to the current deterministic family or framework shape.

Examples:

- `chattycog_webview_module`
  - do not patch legacy helper-summary blocks once newer summary chips exist
- `rust_cli_tool`
  - do not apply old flag-insertion lanes once newer output-shape surfaces exist
- `static_web_dashboard`
  - do not insert UI surfaces outside declared panel anchors

### 4. Project-Derived Temporary Constraints

Dynamic constraints created from diagnosis for one specific patient.

Examples:

- do not touch a diagnosed evolved helper-summary surface with historical lane X
- do not use insertion anchor Y because it no longer exists in this project
- do not mutate files outside the confirmed ownership boundary for this surgery

This last scope is especially important because many failures are not universal;
they are local to one evolved project state.

## How This Fits Current Architecture

This is not a separate philosophy from the patch X-ray work.

It is the generalization of what already exists:

- diagnosis
- intent freeze
- self-review
- structural guard metadata
- preflight
- postcheck
- project patchability governance

Those mechanisms are already negative-constraint friendly.

The constraint shelf should become the more explicit, reusable version of the
same idea:

- not just "is this patch lane available?"
- but "what methods are forbidden here, and why?"

## First Implementation Slice

The first slice should stay tightly bounded.

### Step 1: Constraint Contract

Define a first host-owned contract for one implementation constraint, likely
with fields like:

- `constraint_id`
- `constraint_scope`
- `family_id`
- `tool_kind`
- `language_id`
- `constraint_kind`
- `forbidden_method_summary`
- `forbidden_markers`
- `required_markers`
- `violation_reason_template`
- `replacement_guidance`
- `severity`
- `created_at`

### Step 2: Constraint Evaluation Point

Add one explicit constraint-review phase after self-review and before execution.

Candidate order:

- diagnosis
- freeze
- self-review
- constraint review
- preflight
- patch / composition execution
- postcheck

### Step 3: Constraint Receipt

Persist a host-owned receipt for what the shelf rejected or narrowed, something
like:

- `ConstraintReviewReceipt`
- selected constraints
- violated constraints
- surviving execution options
- recommended modern replacements

### Step 4: First Proof Cases

Start from known failures we already understand, not generic theory.

Best seed cases:

- `helper_status_chip`
  - historical helper-summary status patch blocked by newer summary-chip shape
- `helper_summary_badges`
  - historical helper-summary bundle blocked by missing legacy anchor shape
- Rust legacy flag/output lanes
  - old parser-splice methods invalid once newer output surfaces exist

These are good first constraints because:

- the failure mode is already known
- the observable markers already exist
- the replacement path is already real
- the constraint can be machine-checked

## Relationship to Positive Lanes

This milestone does **not** mean positive lanes go away.

Positive lanes still matter for:

- deterministic family scaffolding
- declared patch capabilities
- bounded composition
- host-owned acceptance and runtime contracts

The shift is that positive lanes should no longer carry the whole burden of
generalization.

Instead:

- lanes declare capability
- diagnosis exposes the patient
- the constraint shelf rejects unsafe implementation methods
- self-review and composition keep only what survives

That is a more scalable route to broader language and framework coverage than
trying to hand-author every exact "correct path" upfront.

## Success Criteria

This milestone is successful when:

- the host can reject implementation methods through a named constraint model,
  not only ad hoc guard logic
- at least a few current failure classes are represented as explicit negative
  constraints
- the system can explain:
  - what was forbidden
  - why it was forbidden
  - what modern/safe alternative survived
- the new layer reduces reliance on proliferating narrow positive lanes

## Near-Term Outcome

If this milestone lands well, the factory should become better at:

- crossing more languages without exploding the lane catalog
- patching evolved projects more safely
- using the model for broad intent and code reasoning
- while using host-owned constraints to prune impossible local execution paths

That is a more realistic path toward true build/code-language agnosticism than
trying to author a giant universal positive grimoire.
