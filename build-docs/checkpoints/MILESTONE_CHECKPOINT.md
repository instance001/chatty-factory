# Milestone Checkpoint

Historical note:
- this is a checkpoint-era progress document
- use [../../docs/CURRENT_ARCHITECTURE.md](../../docs/CURRENT_ARCHITECTURE.md) for the current implementation shape

This document is the rebuild checkpoint after proving deterministic lane growth across multiple substrate shapes.

It exists to answer:

- what the rebuild now proves end to end
- what lifecycle a new capability lane moves through
- what we should do next before the system sprawls again

Read this alongside:
- [REBUILD_PLAN.md](../plans/REBUILD_PLAN.md)
- [NEXT_MILESTONE.md](../plans/NEXT_MILESTONE.md)
- [CHATTYCOG_HOSTING_MILESTONE.md](../milestones/CHATTYCOG_HOSTING_MILESTONE.md)
- [docs/CONTRACT_INVENTORY.md](./docs/CONTRACT_INVENTORY.md)

## Current Position

The rebuild is no longer only proving that it can:
- emit deterministic initial builds
- emit deterministic patch lanes

It now also proves that it can:
- stop honestly on unsupported requests
- scaffold the missing deterministic lane
- integrate that lane into repo-side starter files
- track the lane through a host-owned lifecycle
- turn that lane into a real implemented capability

That is a much stronger factory story than "host can build a few known families."

## What Is Now Proven

### 1) Deterministic build families

The rebuild already has working deterministic build lanes for:
- `static_web_dashboard`
- `python_cli_tool`
- `rust_cli_tool`
- `chattycog_webview_module`
- `chattycog_native_window_module`
- `chattycog_workspace_module`

### 2) Deterministic patch growth

The rebuild already has working patch lanes across different project shapes:

- Python CLI / `csv_report`
  - `json_export`
  - `column_filter`
  - `email_sender`

- Rust CLI / `log_summary`
  - `file_output`
  - `severity_filter`
  - `json_output`

- ChattyCog webview module
  - `bridge_activity_panel`
  - `metric_strip`
  - `asset_inbox_panel`

### 3) Honest fallback with next-lane artifacts

Unsupported requests no longer just fail or ask vague questions.

The host now emits:
- `ClarificationRequest`
- `FallbackBuildSpec`
- `FallbackPlanReceipt`
- runtime stub bundle under `runtime/extension_stubs/`

That means unsupported asks now produce native extension work artifacts rather than only a dead end.

### 4) Lane lifecycle tracking

A new deterministic lane now has a host-owned lifecycle:

- `pending_implementation`
- `implemented`
- `validated_ready`
- `promotion_prepared`
- `apply_patch_ready`

This state is recorded in:
- [operator_registry/pending_lanes.json](./operator_registry/pending_lanes.json)

That lets the factory distinguish:
- unsupported and not yet scaffolded
- scaffolded and still in flight
- scaffolded and validated
- ready for real Rust wiring work

### 5) Assisted lane-promotion workflow

The rebuild now supports the following flow:

1. unsupported request triggers fallback
2. fallback emits extension stub bundle
3. `scaffold-extension`
4. `scaffold-extension --integrate`
5. `scaffold-extension --integrate --promote`
6. `implement-extension`
7. `validate-extension`
8. `prepare-extension-promotion`
9. `prepare-extension-apply-patch`

That is the first real proof that the factory can help grow itself in a structured way.

## Key Proofs

The most important end-to-end proofs so far are:

### Python CLI proof

Request shape:
- unsupported follow-up for `csv_report`
- "add email sender"

Outcome:
- fallback scaffold path used
- lane promoted through lifecycle
- real patch lane implemented
- acceptance passed

Result:
- [output/build_me_a_python_csv_report/fixtures/output/report_email.eml](./output/build_me_a_python_csv_report/fixtures/output/report_email.eml)

### Rust CLI proof

Request shape:
- direct deterministic follow-up for `log_summary`
- "add json summary"

Outcome:
- new real Rust patch lane implemented
- acceptance passed

Result:
- [output/build_me_a_rust_log_summary/fixtures/output/summary.json](./output/build_me_a_rust_log_summary/fixtures/output/summary.json)

### ChattyCog hosted-module proof

Request shape:
- direct deterministic follow-up for `chattycog_webview_module`
- "add asset inbox panel"

Outcome:
- new hosted-module patch lane implemented
- bridge-aware UI surface updated
- acceptance passed

Result:
- [output/make_me_a_chattycog_webview_dashboard/index.html](./output/make_me_a_chattycog_webview_dashboard/index.html)

## What This Changes

Before this pass, the rebuild could mostly say:

- "I know some deterministic lanes"

Now it can say:

- "I know some deterministic lanes"
- "I can detect when a lane is missing"
- "I can scaffold the missing lane cleanly"
- "I can keep track of that lane while it is being built"
- "I can help prepare the final registry wiring work"

That is much closer to the long-term factory thesis.

## What Is Still Not Done

These are the biggest remaining gaps.

### 1) Prepared lanes are not auto-wired into live Rust registries

The rebuild now creates:
- Rust stub files
- `apply_patch`-ready templates

But it does not yet automatically apply those patches into:
- `crates/chatty_factory_families/src/registry.rs`
- `crates/chatty_factory_families/src/lib.rs`

That final step is still human-reviewed work.

### 2) Pending-lane state does not yet influence build routing beyond fallback messaging

The host now explains when a matching pending lane already exists.

It does not yet:
- offer a direct "continue this lane" action in UI
- prefer that lane in planning tasks as a first-class route target

### 3) Family growth is still mostly patch-lane growth

The rebuild is strongest today at:
- new patch lanes inside known families

It is not yet equally mature at:
- fully new family creation
- helper/service lane creation
- cross-family substrate expansion from fallback

### 4) ChattyCog bridge growth is still mostly UI-surface growth

We now have strong hosted-module patches.

We do not yet have the same depth for:
- deeper helper-backed bridge behavior
- richer asset processing lanes
- room-state mutation lanes

## Recommended Next Target

The next best architectural target is:

- make prepared extension lanes more executable as host work, not just better documented

Specifically:

### 1) Apply-patch consumption

Add a command that:
- reads `apply_patch`-ready templates
- applies them safely into the repo
- verifies the build still compiles
- advances lane state beyond `apply_patch_ready`

This would make the final registry-wiring step host-assisted instead of mostly manual.

### 2) UI lane-management surface

Expose pending lane lifecycle in the rebuild UI:
- pending
- implemented
- validated
- promotion prepared
- apply-patch ready

This would make the extension pipeline visible instead of mostly CLI-first.

### 3) One full helper/service lane proof

Take one request that currently exceeds the deterministic hosted contract and grow it into a real helper lane.

That would prove the extension pipeline on:
- more than patch lanes
- more than pure in-project edits

## Short Version

This checkpoint marks the moment where ChattyFactory stopped being only:

- a deterministic scaffold-and-patch host

and started becoming:

- a host that can also structure and supervise the growth of new deterministic lanes

That is a real milestone and worth preserving before we widen further.
