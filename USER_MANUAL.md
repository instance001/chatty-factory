# ChattyFactory User Manual

For the current technical architecture and crate/runtime ownership map, start
with:
- [Current Architecture](./docs/CURRENT_ARCHITECTURE.md)

If you want the shortest possible description, ChattyFactory is a local
build-and-patch factory:

- you give it a plain-language request
- the host freezes a bounded attempt
- the runtime either emits a real artifact or fails honestly with receipts
- later requests can patch an existing generated project under governed review

The host is not meant to choose a nearest positive family or substitute a
"close enough" product shape. It is meant to preserve evidence and drive the
next attempt mechanically.

## 1. What You Need

Before you do anything else, make sure you have:

- Rust and Cargo installed
- this repository checked out locally
- a terminal open in `chatty-factory/`

Optional but useful:

- local GGUF models in `chatty-factory/models/`
- a local `llama.cpp` runtime in `chatty-factory/runtime/`

In a source checkout, ChattyFactory uses the repository root as its workspace.
In a packaged binary release, it uses the folder beside the executable.
Advanced users can set `CHATTY_FACTORY_BASE_PATH` to force an explicit
portable or shared workspace. First run creates `output/`, `runtime/`,
`models/`, `operator_registry/`, and `extensions/` when they are missing.

## 2. Where Things Live

The most important folders are:

- `output/`
  - generated project artifacts
- `runtime/`
  - receipts, next-attempt artifacts, planner handoffs, verification results,
    governance state, notes, and UI state
- `models/`
  - local GGUF files
- `operator_registry/`
  - governed lane and registry state used by the host surfaces
- `crates/`
  - the Rust workspace itself

## 3. Fast Start

From `chatty-factory/`, the two main ways to use the system are:

1. CLI
2. Desktop UI

### CLI quick start

Build something concrete:

```powershell
cargo run -p chatty_factory_cli -- build me a python csv report utility
```

Patch an existing generated project:

```powershell
cargo run -p chatty_factory_cli -- patch build_me_a_python_csv_report add email delivery
```

If a request is too vague or the substrate cannot be frozen honestly, the host
will emit clarification, planner-handoff, or next-attempt receipts instead of
pretending it knows the target shape.

### UI quick start

Launch the desktop shell:

```powershell
cargo run -p chatty_factory_ui
```

In the UI you can:

- browse generated projects
- select the active project
- submit build and patch requests
- inspect verification and runtime evidence
- inspect governance and next-attempt posture
- review receipt trails without opening the raw files manually

## 4. What ChattyFactory Does Today

At this checkpoint, the workspace is strongest at:

- bounded plain-language builds
- governed follow-up patching against generated projects
- host-owned diagnosis, intent freeze, and postcheck flows
- bounded task execution and decomposition
- evidence-driven next-attempt shaping
- preserving receipts for later review, triangulation, and constraint promotion

It is not claiming universal safe surgery across arbitrary unknown codebases.

## 5. Negative-Lane Fallback

Fallback is not "host picks an easier thing."

The intended gauntlet is:

1. attempt under frozen intent
2. fail honestly
3. classify the failure
4. retry with a specific evidence-driven change
5. decompose smaller when needed
6. compare against receipts and prior failures
7. change toolchain or model only when justified

That is why receipts now carry continuation posture such as:

- `normalized_failure_class`
- `recommended_next_action`
- `recommended_next_step`

Those fields describe how the next attempt should differ.
They should not smuggle positive product shape back into the host.

## 6. Day-To-Day Commands

Common commands:

- `cargo run -p chatty_factory_cli -- build <request>`
- `cargo run -p chatty_factory_cli -- patch <project_name> <request>`
- `cargo run -p chatty_factory_cli -- reverify-build <project_name>`
- `cargo run -p chatty_factory_ui`

The CLI also contains proof, governance, helper, registry, and runtime-model
commands for deeper operator workflows. For those, inspect
[crates/chatty_factory_cli/src/main.rs](crates/chatty_factory_cli/src/main.rs)
until a cleaner dedicated CLI reference exists.

## 7. Reading Results

When a run succeeds, look first at:

- the generated project under `output/`
- the verification receipts under `runtime/`

When a run does not succeed, look first at:

- clarification receipts
- planner handoffs
- next-attempt receipts
- verification receipts

The point is not only to know that a run failed.
The point is to know what failed, what evidence was observed, and how the next
attempt must change.

## 8. Key Mental Model

The working equation for this factory is:

- the user carries the intent
- the LLM carries the method
- the host carries the funnel
- the output carries the artifact

Anything more than that tends to become product-shape bloat that weakens the
negative-lane architecture.
