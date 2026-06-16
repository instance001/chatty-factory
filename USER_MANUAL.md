# ChattyFactory User Manual

This manual assumes you know nothing about the project.

If you want the shortest possible description, ChattyFactory is a local build-and-patch factory:

- you give it a plain-language request
- it chooses a supported deterministic build lane
- it generates a real project in `output/`
- you can later select that project and ask for follow-up changes

It also contains a local planning/runtime layer and a desktop UI for managing projects, extension lanes, and lane lifecycle work.
The current UI is designed as a scrollable summary-first workspace with deeper diagnostics available through expandable sections.

One important architectural shift is now explicit:

- starter families are meant to be stable foundations
- they are not meant to be the total limit of what the factory can ever build

The long-term goal is for the main limiter to be:

- what the current model can understand and plan
- what the host can execute safely
- what verification and the negative bookshelf will allow through

That now includes a newer execution idea:

- if a frozen task is still too large for the current local model/runtime pair,
  the factory should learn to split that task into smaller children instead of
  calling it permanently unsupported
- and the long-term goal is for the factory to infer those splits from its own
  task receipts and failures instead of relying on us to hand-author every next
  decomposition path
- but those failures should not immediately become durable negative constraints:
  they should first live in a provisional vault, be retried differently, and
  only promote into the real library when the evidence converges narrowly enough

## License

ChattyFactory is licensed under the GNU Affero General Public License v3.0
only (`AGPL-3.0-only`).

The canonical license text is in the repo root:

- [LICENSE](./LICENSE)

The Rust workspace also declares the same license in:

- [Cargo.toml](./Cargo.toml)

## 1. What You Need

Before you do anything else, make sure you have:

- Rust and Cargo installed
- this repository checked out locally
- a terminal open in `chatty-factory/`

Optional but useful:

- local GGUF models in `chatty-factory/models/`
- a local `llama.cpp` runtime in `chatty-factory/runtime/`

ChattyFactory can still do build and patch work without immediately using a local model, but the planner/runtime features are better when the runtime is present.

## 2. Where Things Live

The rebuild workspace is:

- `chatty-factory/`

The most important folders are:

- `chatty-factory/output/`
  - generated projects live here
- `chatty-factory/runtime/`
  - receipts, runtime state, planner artifacts, UI state, exports, notes, favorites, recents
- `chatty-factory/models/`
  - local GGUF files
- `chatty-factory/extensions/`
  - scaffolded work bundles for new deterministic lanes
- `chatty-factory/operator_registry/`
  - lane registry state, including pending and shipped extension-lifecycle entries

## 3. Fast Start

From `chatty-factory/`, the two main ways to use the system are:

1. CLI
2. Desktop UI

### CLI quick start

Build something concrete:

```powershell
cargo run -p chatty_factory_cli -- build me a python csv report utility
```

Build with a mechanically selected starter:

```powershell
cargo run -p chatty_factory_cli -- build --starter chattyedu_native_window_module a classroom lesson dashboard module
```

Patch something:

```powershell
cargo run -p chatty_factory_cli -- patch build_me_a_python_csv_report add email delivery
```

Use planner help for vague requests:

```powershell
cargo run -p chatty_factory_cli -- make it better --auto-planner
```

If a build request is too vague, ChattyFactory may stop and emit
clarification or planner-handoff artifacts instead of pretending it knows the
target shape.

### UI quick start

Launch the desktop shell:

```powershell
cargo run -p chatty_factory_ui
```

In the UI you can:

- browse generated projects
- select the active project
- submit build and patch requests
- mechanically choose a build starter instead of leaving starter selection to routing guesses
- inspect project patchability and patch risk
- inspect extension lanes
- drive lane lifecycle actions
- pin favorites and track recent lanes
- keep notes on lanes
- export lane summaries and diffs
- launch the retry-search ladder proof directly from the proof controls

The UI and CLI surfaces now also reflect build-task governance more clearly:

- starter override vs normal routing
- plan and work-order artifacts
- model-authored microtask attempts
- decomposition recommendations when a task needs to be split further
- model-ladder posture:
  - whether a task only exhausted the current model and escalated upward
  - or exhausted the full available candidate ladder and should remain provisional evidence
- retry-search proof posture:
  - latest ladder proof status and outcome
  - factory-owned ladder ceiling
  - configured shell timeout buffer
  - recommended shell timeout for operators

## 4. What ChattyFactory Can Do Today

At this checkpoint, ChattyFactory can already build and patch several real
deterministic families.

The current built-in family count is 8.

### Supported project shapes

- standalone web dashboards
- ChattyCog webview modules
- ChattyCog native-window modules
- Chatty-EDU native-window modules
- dual-host Chatty-Cog + Chatty-EDU native-window modules
- ChattyCog workspace modules
- Python CLI tools
- Rust CLI tools

### Supported behavior pattern

The factory is strongest at:

- creating a supported project from plain language
- building explicit ecosystem-native shells from a mechanically selected starter
- verifying the result through host-owned checks
- applying supported follow-up bolt-ons to an existing project
- diagnosing project structure before patch surgery
- blocking unsafe, duplicate, or superseded patch cuts before handler code mutates the project

The intended next step beyond this is not “keep adding more positive families forever.”

It is:

- keep the starter catalog small
- let the model plan richer capability on top of those starters
- let the host and the negative bookshelf decide what is safe to execute
- let adaptive task decomposition shrink a task further when even the atomized
  task is still too broad for the current model/runtime pair

### Honest fallback behavior

If a request is outside supported deterministic lanes, ChattyFactory should not pretend.
Instead it will produce fallback artifacts, clarification information, and extension scaffolds for the missing lane.

That is expected behavior, not a failure of the architecture.

One important nuance now:

- "not yet proven at this task size" should increasingly become a decomposition
  or runtime-mode lesson
- it should not automatically become "unsupported forever"

## 4A. Adaptive Decomposition In Plain English

Sometimes even a small frozen task is still too much for a local model.

When that happens, ChattyFactory is moving toward this behavior:

1. let the model try the current task size
2. inspect the attempt with narrow review and verification
3. decide whether the failure means:
   - the method is bad
   - or the task itself is still too broad
4. if the task is too broad, earn a decomposition rule
5. next time, replace the parent task with smaller child tasks

This is now proven on one real build-side example:

- broad toolbar task
- then toolbar label sentence
- then clause-level toolbar tasks
- then host composition of the final label and Rust syntax

That proof now goes all the way through:

- one clause can succeed while another still fails
- the failing clause can be tightened and retried separately

The next architectural step is important:

- ChattyFactory should stop needing us to manually teach each next task split
- repeated failure evidence should increasingly produce automatic decomposition
  proposals and generic split patterns
- the factory should learn how to narrow work for the current model/runtime
  pair from its own receipts over time

The intended split of responsibility is:

- constraint principles:
  - generic architecture rules like "do not exceed current task granularity"
- failure classes:
  - generic labels chosen from evidence like reasoning fallback or omitted fields
- decomposition grammars:
  - reusable patterns like field split or clause split

The goal is for the factory to match:

- task shape
- failure class
- reusable grammar

instead of asking a human to author one more specific rule every time a new
task class stumbles.
- once both clauses pass, the host composes the final sentence and renders the final toolbar block

## 5. Basic CLI Usage

### Build a project

Use plain language directly:

```powershell
cargo run -p chatty_factory_cli -- build me a rust log summary utility
```

Another example:

```powershell
cargo run -p chatty_factory_cli -- make me a ChattyCog webview dashboard module
```

Mechanical starter examples:

```powershell
cargo run -p chatty_factory_cli -- build --starter chattycog_native_window_module a shared context handoff dashboard module
```

```powershell
cargo run -p chatty_factory_cli -- build --starter chattyedu_native_window_module a kid facing lesson desk module
```

```powershell
cargo run -p chatty_factory_cli -- build --starter chattycog_chattyedu_native_window_module a native dashboard module that should load in both chatty-cog and chatty-edu
```

### Patch a project

Patch mode is:

```powershell
cargo run -p chatty_factory_cli -- patch <project_name> <request>
```

Example:

```powershell
cargo run -p chatty_factory_cli -- patch build_me_a_rust_log_summary add markdown export
```

### Use automatic planner help

If your request is vague, add `--auto-planner`:

```powershell
cargo run -p chatty_factory_cli -- make it better --auto-planner
```

You can also set a model lane and port:

```powershell
cargo run -p chatty_factory_cli -- patch build_me_a_python_csv_report make it better --auto-planner --model fast --port 8108
```

## 6. Project Selection Commands

These commands control which project the system treats as the user-selected target.

Select a project:

```powershell
cargo run -p chatty_factory_cli -- select-project build_me_a_python_csv_report
```

Show current selection:

```powershell
cargo run -p chatty_factory_cli -- selected-project
```

Clear selection:

```powershell
cargo run -p chatty_factory_cli -- clear-selected-project
```

Refresh the project browser:

```powershell
cargo run -p chatty_factory_cli -- project-browser
```

Machine-friendly output is also available:

```powershell
cargo run -p chatty_factory_cli -- project-browser --json
```

## 7. Runtime and Model Commands

Show the discovered model catalog:

```powershell
cargo run -p chatty_factory_cli -- runtime-models
```

Run a runtime smoke check without launching the model:

```powershell
cargo run -p chatty_factory_cli -- runtime-smoke --skip-launch
```

Run the full runtime smoke path:

```powershell
cargo run -p chatty_factory_cli -- runtime-smoke
```

The runtime layer now records enough metadata in its receipts to distinguish:

- launch timeout before the local model server became responsive
- planner request timeout after launch
- model-task generation timeout after launch
- normal completion followed by intentional host cleanup of the temporary server

The default runtime budgets are currently:

- launch wait: 90 seconds
- planner request: 420 seconds
- model-task generation request: 300 seconds
- shell timeout buffer recommendation: 60 seconds

When you want to inspect a slow or failed run, the most useful receipt folders are:

- `runtime/runtime_checks/`
- `runtime/planner_runs/`
- `runtime/model_task_generation_receipts/`
- `runtime/retry_search_proofs/`

For the retry-search model escalation proof specifically, the proof receipt now
records a worst-case outer timeout budget computed from:

- retry posture count
- model candidate count
- launch timeout
- request timeout
- cleanup overhead

The recommended operator shell timeout is:

- `expected_outer_timeout_secs + shell_timeout_buffer_secs`

The default shell timeout buffer recommendation is currently:

- `60 seconds`

Use that receipt as the source of truth.
A shell timeout or terminal cutoff is not a factory failure by itself unless the
receipt shows:

- `final_outcome=full_model_ladder_exhausted`
- `final_outcome=internal_timeout_observed`

The desktop UI now surfaces this in two places:

- `Runtime Status`
  - current runtime config
  - shell timeout buffer
  - latest ladder proof posture
- `Cross-Family Paired Proof -> Proof Runtime Posture`
  - current runtime request budgets
  - latest ladder proof outcome
  - factory-owned ladder ceiling
  - recommended shell timeout
  - `Run Retry-Search Ladder Proof`

Run a planner handoff manually:

```powershell
cargo run -p chatty_factory_cli -- planner-run runtime\planner_handoffs\<handoff-file>.json --model fast --port 8106
```

## 8. Extension Lane Lifecycle

One of the major rebuild goals is self-growth: when something is unsupported, the factory can scaffold the next deterministic lane instead of only failing.

### Lifecycle states

An extension lane can move through:

- `pending_implementation`
- `implemented`
- `validated_ready`
- `promotion_prepared`
- `apply_patch_ready`
- `host_wired`
- `fully_live`
- `archived`

### Main commands

Create a work bundle from a fallback stub:

```powershell
cargo run -p chatty_factory_cli -- scaffold-extension <stub_dir_or_extension_spec_json>
```

Also emit live starter files:

```powershell
cargo run -p chatty_factory_cli -- scaffold-extension --integrate <stub_dir_or_extension_spec_json>
```

Also register it as a pending lane:

```powershell
cargo run -p chatty_factory_cli -- scaffold-extension --integrate --promote <stub_dir_or_extension_spec_json>
```

Show extension registry summary:

```powershell
cargo run -p chatty_factory_cli -- pending-extensions
```

Advance a lane through the lifecycle:

```powershell
cargo run -p chatty_factory_cli -- implement-extension <entry_id>
cargo run -p chatty_factory_cli -- validate-extension <entry_id>
cargo run -p chatty_factory_cli -- prepare-extension-promotion <entry_id>
cargo run -p chatty_factory_cli -- prepare-extension-apply-patch <entry_id>
cargo run -p chatty_factory_cli -- consume-extension-apply-patch <entry_id>
cargo run -p chatty_factory_cli -- validate-live-extension <entry_id>
```

Archive a bad or superseded lane:

```powershell
cargo run -p chatty_factory_cli -- archive-extension <entry_id> [reason]
```

## 9. Using the Desktop UI

Launch:

```powershell
cargo run -p chatty_factory_ui
```

### What the UI shows

- project list
- project details
- runtime status
- project patchability badges and filters
- extension registry
- lane detail/workbench
- governance panels
- proof panels
- patch X-ray summaries and project surgery history
- last result
- command log

### Extension registry features

- search by family, tool, patch, status, id, or archive reason
- scope by `All`, `Shipped`, `Active`, `Archived`
- sort by `Recent`, `Status`, `Family/Tool`
- `Favorites`
- `Recent`
- archive actions for active lanes

### Lane workbench features

For a selected lane, the UI currently supports:

- lifecycle summary and readiness strip
- blocker list
- source spec preview
- implementation notes preview
- acceptance target preview
- promotion compare
- apply-patch compare
- mismatch hints
- lane note editor
- export summary
- copy summary
- open latest export
- export history
- latest-vs-previous export diff
- lane timeline

### Lane file shortcuts

The workbench can jump to:

- scaffold root
- source spec
- first integrated file
- promotion artifact
- apply-patch artifact
- implementation notes
- acceptance target file

## 10. Tips for Best Results

### Be direct and concrete

Better:

- `build me a rust log summary utility`
- `patch build_me_a_rust_log_summary add markdown export`

Less useful:

- `make it good`
- `do more stuff`

Those kinds of requests may still work through planner help, but they are more
likely to produce clarification artifacts than an immediate deterministic run.

### Use `--auto-planner` when you are intentionally vague

This helps when you want the local planner/runtime to narrow an ambiguous request:

```powershell
cargo run -p chatty_factory_cli -- make it better --auto-planner
```

### Select your project before vague follow-ups

If you are going to say things like:

- `make it better`
- `add export`
- `improve the module`

then explicitly selecting the target project first helps the router stay honest.

### Check `output/` first

If you want to inspect what was actually generated, go straight to:

- `chatty-factory/output/<project_name>/`

That is the durable generated artifact area.

### Check `runtime/` when you want receipts

If you want to understand what the host thought it was doing, inspect:

- `runtime/requests/`
- `runtime/routes/`
- `runtime/plans/`
- `runtime/acceptance_results/`
- `runtime/planner_handoffs/`
- `runtime/planner_responses/`
- `runtime/execution_receipts/`
- `runtime/project_snapshots/`

## 11. Troubleshooting

### "Nothing happened" or the request stopped early

This usually means:

- the request was outside supported deterministic lanes
- clarification was required
- the planner/runtime path needed help

Check:

- terminal output
- `runtime/fallback_plan_receipts/`
- `runtime/clarifications/`
- `runtime/extension_stubs/`

### Planner/runtime problems

Check:

- your GGUFs are in `chatty-factory/models/`
- your runtime files are in `chatty-factory/runtime/`
- `runtime-models` works
- `runtime-smoke --skip-launch` works

### Build or patch target confusion

Use:

```powershell
cargo run -p chatty_factory_cli -- selected-project
cargo run -p chatty_factory_cli -- project-browser --json
```

### UI state issues

The UI stores local convenience state in `runtime/`, including:

- `extension_favorites.json`
- `extension_recent.json`
- `extension_notes.json`
- `extension_exports/`

If the UI gets into a weird state, these are good places to inspect.

## 12. What This Is Not

This is not a general magic coding agent that can already build any software request on earth.

It is a growing deterministic factory with:

- honest supported lanes
- honest fallback
- host-owned receipts and checks
- a structured path for adding new lanes

That honesty is intentional.

## 13. Recommended First Session

If you want a practical first run, do this:

1. Open a terminal in `chatty-factory/`.
2. Run:

```powershell
cargo run -p chatty_factory_cli -- build me a python csv report utility
```

3. Then run:

```powershell
cargo run -p chatty_factory_cli -- patch build_me_a_python_csv_report add email delivery
```

4. Then launch the UI:

```powershell
cargo run -p chatty_factory_ui
```

5. Inspect the generated project, the extension registry, and the lane workbench.

That gives you the full flavor of the rebuild:

- initial build
- deterministic patch
- generated output
- host receipts
- UI inspection and management
