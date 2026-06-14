# ChattyFactory

ChattyFactory is a deterministic build-and-patch factory for plain-language
software requests.

Its current product shape is:

- plain-language request in
- real generated project out
- plain-language follow-up patching against existing projects
- host-owned diagnosis and governance around patch surgery

The architectural stance is deliberate:

- the LLM should choose, triage, and review
- the host should own exact machinery wherever practical

## Current State

At the current checkpoint, ChattyFactory is a working bounded factory rather
than only a rebuild sketch.

It now has:

- 8 built-in deterministic families
- deterministic build and patch flows
- starter-plus-plan execution artifacts
- atomized microtask execution with host-sync, host-mechanical, and model-authored task kinds
- adaptive task decomposition that can replace a failed broad microtask with smaller child tasks on the next run
- diagnosis-aware patch surgery:
  - diagnosis
  - intent freeze
  - postcheck
- project-level patchability governance
- a desktop UI for project, patch, extension, proof, and governance surfaces
- a scrollable summary-first workspace with expandable deep-detail sections

The product promise is intentionally bounded:

- plain-language requests should land on supported deterministic lanes
- vague requests may trigger clarification or planner guidance instead of fake certainty
- existing-project patching is strongest on generated projects that still match declared family and patch contracts

But the capability boundary is no longer supposed to be the old positive-lane catalog alone.
The active direction is:

- starters as stable substrates
- model-planned feature work on top
- host-owned freezing, review, and verification
- adaptive decomposition when a task is still too broad for the current model/runtime pair
- automatic decomposition inference from task failure evidence so the factory can learn how to shrink work from its own receipts instead of depending on us to hand-teach each next split

The intended architecture distinction is now explicit:

- constraint principles are generic and stable
- failure classes are generic classifications selected from evidence
- decomposition grammars are reusable split patterns
- the factory should infer mappings between task shape, failure class, and grammar
  instead of accumulating bespoke negative rules for each new task class

## What It Does Well

ChattyFactory is strongest today at:

- building supported projects from plain-language requests
- patching generated projects through declared deterministic lanes
- explaining patch safety through:
  - diagnosis
  - intent freeze
  - postcheck
- surfacing project patchability risk and drift over time

It is not claiming universal safe surgery across arbitrary unknown codebases.

## Key Docs

- [User Manual](./USER_MANUAL.md)
- [Build Docs Archive](./build-docs/README.md)
- [REBUILD_PLAN.md](./build-docs/plans/REBUILD_PLAN.md)
- [Initial GitHub Release Plan](./build-docs/plans/INITIAL_GITHUB_RELEASE_PLAN.md)
- [GitHub Upload Metadata](./build-docs/plans/GITHUB_UPLOAD_METADATA.md)
- [Release Notes v0.1.0](./build-docs/plans/RELEASE_NOTES_v0.1.0.md)
- [Architecture Checkpoint 2](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_2.md)
- [Architecture Checkpoint 3](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_3.md)
- [Architecture Checkpoint 4](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_4.md)
- [Architecture Checkpoint 5](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_5.md)
- [Architecture Checkpoint 6](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_6.md)
- [Architecture Checkpoint 7](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_7.md)
- [Architecture Checkpoint 8](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_8.md)
- [Architecture Checkpoint 9](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_9.md)
- [Architecture Checkpoint 10](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_10.md)
- [Architecture Checkpoint 11](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_11.md)
- [Architecture Checkpoint 12](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_12.md)
- [Architecture Checkpoint 13](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_13.md)
- [Architecture Checkpoint 14](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_14.md)
- [Architecture Checkpoint 15](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_15.md)
- [Architecture Checkpoint 16](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_16.md)
- [Architecture Checkpoint 17](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_17.md)
- [Architecture Checkpoint 18](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_18.md)
- [Architecture Checkpoint 19](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_19.md)
- [Architecture Checkpoint 20](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_20.md)
- [Architecture Checkpoint 21](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_21.md)
- [Architecture Checkpoint 22](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_22.md)
- [Architecture Checkpoint 23](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_23.md)
- [Architecture Checkpoint 24](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_24.md)
- [Architecture Checkpoint 25](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_25.md)
- [Architecture Checkpoint 26](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_26.md)
- [Architecture Checkpoint 27](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_27.md)
- [Architecture Checkpoint 28](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_28.md)
- [Architecture Checkpoint 29](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_29.md)
- [Architecture Checkpoint 30](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_30.md)
- [Architecture Checkpoint 31](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_31.md)
- [Architecture Checkpoint 32](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_32.md)
- [Architecture Checkpoint 33](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_33.md)
- [Architecture Checkpoint 34](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_34.md)
- [Architecture Checkpoint 35](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_35.md)
- [Architecture Checkpoint 36](./build-docs/checkpoints/ARCHITECTURE_CHECKPOINT_36.md)
- [Design Intent Review](./build-docs/reviews/DESIGN_INTENT_REVIEW.md)
- [Generalized Primitive Proof Harness Milestone](./build-docs/milestones/GENERALIZED_PRIMITIVE_PROOF_HARNESS_MILESTONE.md)
- [Bounded Adaptive Composition Milestone](./build-docs/milestones/BOUNDED_ADAPTIVE_COMPOSITION_MILESTONE.md)
- [Composed Patch Milestone](./build-docs/milestones/COMPOSED_PATCH_MILESTONE.md)
- [Helper Primitive Catalog Milestone](./build-docs/milestones/HELPER_PRIMITIVE_CATALOG_MILESTONE.md)
- [Primitive-Native Execution Milestone](./build-docs/milestones/PRIMITIVE_NATIVE_EXECUTION_MILESTONE.md)
- [Cross-Family Helper Monitoring Milestone](./build-docs/milestones/CROSS_FAMILY_HELPER_MONITORING_MILESTONE.md)
- [Cross-Family Paired Proof Milestone](./build-docs/milestones/CROSS_FAMILY_PAIRED_PROOF_MILESTONE.md)
- [Composition Governance Milestone](./build-docs/milestones/COMPOSITION_GOVERNANCE_MILESTONE.md)
- [Patch Governance Milestone](./build-docs/milestones/PATCH_GOVERNANCE_MILESTONE.md)
- [Helper Governance Milestone](./build-docs/milestones/HELPER_GOVERNANCE_MILESTONE.md)
- [Bridge Governance Milestone](./build-docs/milestones/BRIDGE_GOVERNANCE_MILESTONE.md)
- [Family Governance Milestone](./build-docs/milestones/FAMILY_GOVERNANCE_MILESTONE.md)
- [Template Governance Milestone](./build-docs/milestones/TEMPLATE_GOVERNANCE_MILESTONE.md)
- [Patch Diagnosis X-Ray Milestone](./build-docs/milestones/PATCH_DIAGNOSIS_XRAY_MILESTONE.md)
- [Patch Plan Self-Review Milestone](./build-docs/milestones/PATCH_PLAN_SELF_REVIEW_MILESTONE.md)
- [Project Patch Readiness Governance Milestone](./build-docs/milestones/PROJECT_PATCH_READINESS_GOVERNANCE_MILESTONE.md)
- [Negative Constraint Shelf Milestone](./build-docs/milestones/NEGATIVE_CONSTRAINT_SHELF_MILESTONE.md)
- [ChattyCog Module Skeleton Integration Milestone](./build-docs/milestones/CHATTYCOG_MODULE_SKELETON_INTEGRATION_MILESTONE.md)
- [Chatty-EDU Native Starter Integration Milestone](./build-docs/milestones/CHATTYEDU_NATIVE_STARTER_INTEGRATION_MILESTONE.md)
- [MILESTONE_CHECKPOINT.md](./build-docs/checkpoints/MILESTONE_CHECKPOINT.md)
- [NEXT_WAVE_OPTIONS.md](./build-docs/plans/NEXT_WAVE_OPTIONS.md)
- [Negative Constraint Shelf Implementation Plan](./build-docs/plans/NEGATIVE_CONSTRAINT_SHELF_IMPLEMENTATION_PLAN.md)
- [Starter Plus Plan Execution Pivot](./build-docs/plans/STARTER_PLUS_PLAN_EXECUTION_PIVOT.md)
- [Atomized Microtask Execution Milestone](./build-docs/plans/ATOMIZED_MICROTASK_EXECUTION_MILESTONE.md)
- [Adaptive Task Decomposition Milestone](./build-docs/plans/ADAPTIVE_TASK_DECOMPOSITION_MILESTONE.md)
- [Automatic Decomposition Inference Milestone](./build-docs/plans/AUTOMATIC_DECOMPOSITION_INFERENCE_MILESTONE.md)
- [Over The Line Execution Plan](./build-docs/plans/OVER_THE_LINE_EXECUTION_PLAN.md)
- [Positive Lane Deprecation Plan](./build-docs/plans/POSITIVE_LANE_DEPRECATION_PLAN.md)
- [HELPER_SERVICE_MILESTONE.md](./build-docs/milestones/HELPER_SERVICE_MILESTONE.md)
- [Repository Layout](./docs/REPOSITORY_LAYOUT.md)
- [Contract Inventory](./docs/CONTRACT_INVENTORY.md)
- [Primitive Catalog](./docs/PRIMITIVE_CATALOG.md)
- [Proof Harness Manifests](./proof_harness/README.md)
- [Milestone One Route Sketch](./docs/MILESTONE_ONE_ROUTE_SKETCH.md)
- [Family Spec Template](./docs/FAMILY_SPEC_TEMPLATE.md)
- [Tooling Policy](./docs/TOOLING_POLICY.md)
- [Artifact Policy](./docs/ARTIFACT_POLICY.md)
- [Governance Model](./build-docs/plans/GOVERNANCE_MODEL.md)
- [UI Module Structure](./docs/UI_MODULE_STRUCTURE.md)
- [UI Remaining Boundaries Review](./docs/UI_REMAINING_BOUNDARIES_REVIEW.md)
- [Next Pivot Recommendation](./docs/NEXT_PIVOT_RECOMMENDATION.md)
- [Remaining Governed Surfaces Review](./build-docs/reviews/REMAINING_GOVERNED_SURFACES_REVIEW.md)
- [Acceptance Governance Decision Review](./build-docs/reviews/ACCEPTANCE_GOVERNANCE_DECISION_REVIEW.md)
- [Remaining Legacy-Sensitive Patch Lanes Review](./build-docs/reviews/REMAINING_LEGACY_SENSITIVE_PATCH_LANES_REVIEW.md)

## License

ChattyFactory is licensed under the GNU Affero General Public License v3.0
only (`AGPL-3.0-only`).

- Full license text: [LICENSE](./LICENSE)
- Workspace declaration: [Cargo.toml](./Cargo.toml)

This repo is intended to ship with that license clearly attached in both the
source tree and the public-facing documentation.

## Quick Start

From `chatty-factory/`:

Build a project:

```powershell
cargo run -p chatty_factory_cli -- build me a python csv report utility
```

Build with a mechanically selected starter:

```powershell
cargo run -p chatty_factory_cli -- build --starter chattycog_native_window_module a shared context handoff dashboard module
```

```powershell
cargo run -p chatty_factory_cli -- build --starter chattycog_chattyedu_native_window_module a dual-host native dashboard module
```

Patch an existing generated project:

```powershell
cargo run -p chatty_factory_cli -- patch build_me_a_python_csv_report add email delivery
```

Launch the desktop UI:

```powershell
cargo run -p chatty_factory_ui
```

The UI now defaults to a summary-first layout:

- the main workspace scrolls cleanly
- dense governance and diagnostic sections are collapsible
- project, result, and fallback views keep the most important signal visible first

The build side now also persists reviewed task artifacts under `runtime/`, including:

- build plans
- build plan reviews
- build constraint reviews
- build execution work orders
- frozen task lists
- task execution and verification logs
- model-authored task receipts
- adaptive task decomposition receipts
- decomposition inference should increasingly become generic and self-improving rather than hand-authored task by task

If a request is too vague, ChattyFactory may emit clarification or planner
handoff artifacts instead of guessing.

## Initial Family Specs

- [static_web_dashboard](./families/static_web_dashboard.md)
- [chattycog_basic_dashboard](./families/chattycog_basic_dashboard.md)
- [python_cli_tool](./families/python_cli_tool.md)
- [rust_cli_tool](./families/rust_cli_tool.md)

## Supported Families

The current built-in families are:

- `static_web_dashboard`
- `chattycog_webview_module` (frozen legacy starter)
- `chattycog_native_window_module` (primary forward skeleton)
- `chattyedu_native_window_module` (primary forward skeleton for school-facing native modules)
- `chattycog_chattyedu_native_window_module` (primary forward skeleton for dual-host native modules)
- `chattycog_workspace_module` (frozen legacy starter)
- `python_cli_tool`
- `rust_cli_tool`

## Current Direction

Web remains the strongest early lane because it is the fastest place to prove host-owned machinery.

But the rebuild is not philosophically web-only.
The architecture is being designed so honest non-web families can return as soon as templates, contracts, checks, and repair lanes are strong enough to support them.

Current architectural posture:

- older positive families proved the build/patch/x-ray/governance stack
- they are now considered transitional rather than sacred
- template-first skeletons plus the negative bookshelf are the preferred route
  for future reliability gains
- starter families are being reframed as stable substrates, not the total
  capability boundary
- the long-term limiter should be what the current LLM can plan and what the
  host can execute safely, not the size of the positive family catalog
- the next execution pivot is toward atomized microtasks, so unsupported work
  becomes small reviewed tasks instead of giant all-or-nothing generations
- adaptive decomposition is now real for at least one proven task family:
  - broad `toolbar_ui_block`
  - to `toolbar_label_sentence`
  - to clause-level child tasks
  - with the host composing the final toolbar sentence and Rust block
- one full build-side semantic proof now exists where:
  - the model supplies only tiny clause-level semantic drafts
  - the host composes the final sentence
  - the host renders the final Rust UI block
- ecosystem-native starters now exist as explicit mechanical choices:
  - `chattycog_native_window_module`
  - `chattyedu_native_window_module`
  - `chattycog_chattyedu_native_window_module`

## Local Folders

- `models/`: future local GGUFs and related model files
- `runtime/`: local receipts, logs, and other rebuild runtime state
- `output/`: generated rebuild outputs
- `templates/`: versioned family and wrapper template assets
- `extensions/`: scaffolded implementation work bundles for new deterministic families, patch lanes, and bridge lanes

Artifact handling is now explicit:
- durable generated project files belong in `output/`
- factory supervision state belongs in `runtime/`
- local compile caches and similar byproducts are not treated as canonical product output

## Project Session Commands

- `cargo run -p chatty_factory_cli -- select-project <project_name>`: mark a project as explicitly selected by the user
- `cargo run -p chatty_factory_cli -- selected-project`: show the current explicit selection and last touched project
- `cargo run -p chatty_factory_cli -- clear-selected-project`: clear the explicit selection
- `cargo run -p chatty_factory_cli -- project-browser`: refresh and show the host-owned project browser state
- `cargo run -p chatty_factory_cli -- refresh-project-patch-readiness`: refresh host-owned per-project patchability receipts and browser-facing patchability baselines

Each of those commands also supports `--json` to emit machine-friendly state for a future UI or wrapper process.

## Extension Scaffolds

- `cargo run -p chatty_factory_cli -- scaffold-extension <stub_dir_or_extension_spec_json>`: turn a fallback stub bundle into a repo-side implementation work bundle under `extensions/`
- `cargo run -p chatty_factory_cli -- scaffold-extension --integrate <stub_dir_or_extension_spec_json>`: also emit live starter files into repo manifest/template/registry-adjacent paths
- `cargo run -p chatty_factory_cli -- scaffold-extension --integrate --promote <stub_dir_or_extension_spec_json>`: also register the starter as a pending deterministic lane in `operator_registry/pending_lanes.json`
- `cargo run -p chatty_factory_cli -- register-proof-harness-bundle <template_manifest_json> <comparison_bundle_manifest_json>`: register a repo-defined proof template and comparison bundle as a pending `proof_harness_bundle`
- `cargo run -p chatty_factory_cli -- pending-extensions`: show the current pending deterministic lane registry summary
- `cargo run -p chatty_factory_cli -- implement-extension <entry_id>`: mark a pending deterministic lane entry as implemented
- `cargo run -p chatty_factory_cli -- archive-extension <entry_id> [reason]`: retire a superseded or bad lane entry without deleting its history
- `cargo run -p chatty_factory_cli -- validate-extension <entry_id>`: verify the expected scaffold and integrated files exist, then mark the lane `validated_ready`
- `cargo run -p chatty_factory_cli -- prepare-extension-promotion <entry_id>`: generate Rust-side promotion stubs and notes for a `validated_ready` lane, then mark it `promotion_prepared`
- `cargo run -p chatty_factory_cli -- prepare-extension-apply-patch <entry_id>`: generate `apply_patch`-ready wiring templates for `registry.rs` and `lib.rs`, then mark the lane `apply_patch_ready`
- `cargo run -p chatty_factory_cli -- consume-extension-apply-patch <entry_id>`: apply the prepared wiring into the live Rust family crate, run `cargo check`, roll back on failure, and mark the lane `host_wired` on success
- `cargo run -p chatty_factory_cli -- validate-live-extension <entry_id>`: verify that a `host_wired` lane no longer contains placeholder recipe markers or stub handler logic, rerun `cargo check`, and mark it `fully_live`

This is the bridge between:
- honest fallback saying "not supported yet"
- and the next deterministic lane implementation work

The runtime keeps these signals separately:
- `runtime/selected_project_session.json`: strongest user-selected project signal
- `runtime/active_project_session.json`: last project touched by a build or patch run
- `runtime/project_browser_state.json`: UI-facing project catalog plus selection/session state

## Desktop UI

- `cargo run -p chatty_factory_ui`: open the rebuild desktop shell

The desktop UI currently:
- reads `runtime/project_browser_state.json`
- lists discovered output projects
- lets you refresh, select, and clear the active user project
- sends build and patch requests through `chatty_factory_cli`
- shows the extension registry with shipped, active, and archived lanes
- lets you inspect a lane and drive its next lifecycle action from the shell
- shows command output in-app
- shows project patchability badges, filters, and refresh state
- shows patch X-ray summaries and recent patch surgeries
- shows proof and governance surfaces beyond the original thin-shell project browser

## Release Notes

The initial public release plan and notes live at:

- [Initial GitHub Release Plan](./build-docs/plans/INITIAL_GITHUB_RELEASE_PLAN.md)
- [GitHub Upload Metadata](./build-docs/plans/GITHUB_UPLOAD_METADATA.md)
- [Release Notes v0.1.0](./build-docs/plans/RELEASE_NOTES_v0.1.0.md)
