# ChattyCog Hosting Milestone

This document is the next concrete implementation target for ChattyFactory's ChattyCog compatibility lane.

It exists because the rebuild already emits a valid starter wrapper bundle, but the read-only ChattyCog reference material makes it clear that "wrapper files exist" is not the same thing as "the module contract is honestly modeled."

Read this alongside:
- [NEXT_MILESTONE.md](../plans/NEXT_MILESTONE.md)
- [REBUILD_PLAN.md](../plans/REBUILD_PLAN.md)
- [../chattycog/chattycog_gui/docs/MODULES.md](../chattycog/chattycog_gui/docs/MODULES.md)
- [../chattycog/chattycog_gui/docs/MODULE_BRIDGE.md](../chattycog/chattycog_gui/docs/MODULE_BRIDGE.md)
- [../chattycog/chattycog_gui/docs/MODULE_VISUAL_LOAD.md](../chattycog/chattycog_gui/docs/MODULE_VISUAL_LOAD.md)
- [../chattycog/chattycog_gui/docs/MODULE_BUILDER_CHECKLIST.md](../chattycog/chattycog_gui/docs/MODULE_BUILDER_CHECKLIST.md)
- [../chattycog/chattycog_gui/docs/MODULE_UI.md](../chattycog/chattycog_gui/docs/MODULE_UI.md)

## Why This Is Next

The rebuild already covers:
- `manifest.json`
- `visual_load.json`
- `HANDSHAKE.md`
- `bridge/status.json`
- `ChattyCogModuleSpec.json`
- basic contract validation

That is enough for a starter lane.

It is not yet enough to honestly represent the full ChattyCog module ecosystem described by the reference docs.

The biggest remaining gap is that ChattyCog modules are not one shape. The docs define three distinct hosting paths:
- hosted `webview`
- hosted `native_window`
- no hosted UI, fallback `ui.json` / workspace surface

Right now ChattyFactory still treats ChattyCog module output as one mostly-web wrapper family. That is useful, but it flattens real differences the host needs to understand.

## What The Reference Docs Say

The reference material establishes a few important truths.

### 1) Modules are standalone first

From the checklist and module docs:
- the module should run on its own
- ChattyCog should host it
- ChattyCog should not become the module's real runtime brain

That means ChattyFactory should keep producing:
- a real standalone tool first
- then a deterministic ChattyCog compatibility shell around it

It should not produce "ChattyCog-only pretend modules" unless the family is explicitly a workspace-only module.

### 2) Hosting is not one thing

The docs clearly distinguish:
- `webview`
- `native_window`
- fallback `ui.json` / workspace

Those are different contracts, different validation needs, and different launch metadata.

### 3) The bridge is richer than `status.json`

The docs define a portable bridge with optional but meaningful lanes:
- `bridge/status.json`
- `bridge/log_sources.json`
- `bridge/shared_room_state.json`
- `bridge/shared_room_events.json`
- `bridge/outgoing_room_events.json`
- `bridge/incoming_assets/<lane_id>/...`

ChattyFactory should not pretend all of these are always required.

But it should model them explicitly, so the planner and validator know which hosted features a module family actually supports.

### 4) `visual_load.json` is a real host contract

The docs define a richer `visual_load.json` than the rebuild currently validates.

Important fields include:
- `kind`
- `auto_launch`
- `title`
- `file`
- `url`
- `notes`
- `window_title_contains`
- `build`
- `launch`
- `serve`
- `serve_wait_ms`

Those should become typed rebuild contracts, not loose emitted JSON blobs.

### 5) `ui.json` is a real module path

The docs treat `ui.json` as the recommended fallback for modules without their own hosted UI.

That means ChattyFactory should eventually support a deterministic "workspace module" family, not just webview-ish outputs.

## Milestone Goal

Turn the current ChattyCog compatibility lane into an honest hosted-module system with:
- explicit hosting mode
- richer visual-load contract
- richer optional bridge contract
- validation aligned with the real ChattyCog docs

## Families To Model

This milestone should split ChattyCog output into three distinct deterministic module families.

### 1) `chattycog_webview_module`

Use when:
- the module's real UI is HTML/CSS/JS
- ChattyCog should host that real browser-style UI

Should emit:
- `manifest.json`
- `visual_load.json` with `kind: "webview"`
- web assets
- `HANDSHAKE.md`
- `bridge/status.json`
- optional `bridge/log_sources.json`
- `ChattyCogModuleSpec.json`

Validation focus:
- valid webview load target
- valid module-relative file path or URL/serve pairing
- stable hosted title metadata
- bridge files coherent with hosted webview assumptions

### 2) `chattycog_native_window_module`

Use when:
- the module's real UI is a native desktop window
- ChattyCog should dock that real standalone app

Should emit:
- `manifest.json`
- `visual_load.json` with `kind: "native_window"`
- standalone app launch metadata
- `HANDSHAKE.md`
- `bridge/status.json`
- optional `bridge/log_sources.json`
- `ChattyCogModuleSpec.json`

Validation focus:
- launch command present
- relative launch/build paths
- stable `window_title_contains`
- bridge portability expectations

### 3) `chattycog_workspace_module`

Use when:
- the module has no hosted app
- ChattyCog should provide the practical surface

Should emit:
- `manifest.json`
- `ui.json` or `STATE_TEMPLATE.md`
- `HANDSHAKE.md`
- optional bridge files only if helpful
- `ChattyCogModuleSpec.json`

Validation focus:
- no dishonest hosted claims
- usable module workspace contract
- clear "ChattyCog-provided UI" semantics

## New Contracts To Add

### 1) `ChattyCogVisualLoadSpec`

Purpose:
- typed representation of `visual_load.json`

Suggested fields:
- `kind`
- `auto_launch`
- `title`
- `notes`
- `file`
- `url`
- `window_title_contains`
- `build_command`
- `launch_command`
- `serve_command`
- `serve_wait_ms`

Acceptance:
- emitted `visual_load.json` can round-trip through this contract
- validator can distinguish valid `webview` vs `native_window`

### 2) `ChattyCogBridgeCapabilities`

Purpose:
- typed representation of optional bridge lanes

Suggested fields:
- `status_enabled`
- `log_sources_enabled`
- `shared_room_state_enabled`
- `shared_room_events_enabled`
- `outgoing_room_events_enabled`
- `incoming_asset_lanes`

Acceptance:
- the host can explain what bridge lanes a module exposes
- validator can reject impossible/partial bridge claims

### 3) `ChattyCogWorkspaceSpec`

Purpose:
- model `ui.json` / workspace-backed modules honestly

Suggested fields:
- `ui_json_present`
- `workspace_fallback_present`
- `state_persistence_mode`
- `hosted_surface_kind`

Acceptance:
- workspace modules do not need fake `visual_load.json`
- the host can validate "host provides surface" separately from "module provides surface"

## Validation Work

Add new acceptance checks beyond the current broad module contract check.

Suggested acceptance check kinds:
- `chattycog_visual_load_contract`
- `chattycog_bridge_contract`
- `chattycog_workspace_contract`
- `chattycog_module_packaging_contract`

Validation should cover:
- required files by hosting mode
- relative path correctness
- `visual_load.json` field coherence
- optional bridge file structure
- standalone-first honesty

## Host / Planner Behavior Changes

### 1) Route-time hosting honesty

The planner/host should stop treating all ChattyCog asks as "basic dashboard wrapper" by default.

It should distinguish:
- dashboard hosted in webview
- standalone desktop app to dock
- workspace-only module shell

### 2) Module capability reporting

The host result surface should eventually expose:
- hosting mode
- bridge lanes
- whether the module owns the UI or ChattyCog does

This should show up in:
- `ProjectSpec`
- host execution results
- UI result cards

### 3) Fallback honesty

If the user asks for:
- native docking behavior from a family that only supports webview
- bridge lanes the chosen family does not model
- room-aware behavior without a helper/bridge lane

the host should stop cleanly and explain the mismatch.

## Recommended Implementation Order

### 1) Add typed visual-load and bridge contracts

Why first:
- they are the cleanest missing host-owned contracts

### 2) Extend current ChattyCog validator

Why second:
- we already have one broad module contract check
- extend it instead of inventing a parallel validation lane

### 3) Split the current family into explicit hosting-mode families

Why third:
- once contracts exist, family honesty becomes much simpler

### 4) Add workspace-module deterministic output

Why fourth:
- this is the missing non-hosted-ui ChattyCog lane

### 5) Surface hosting-mode state into host/UI/CLI

Why fifth:
- after the contracts and families exist, the result surfaces should explain them

## Acceptance For This Milestone

This milestone is complete when:

1. ChattyFactory can deterministically produce:
- a ChattyCog webview module
- a ChattyCog native-window module
- a ChattyCog workspace module

2. The host can validate each honestly against typed contracts.

3. The result surfaces can explain:
- hosting mode
- bridge support
- who owns the real UI

4. Unsupported hosting requests stop cleanly with structured fallback.

## What Should Wait Until After This

Do not treat this milestone as a reason to immediately explode family count again.

After this milestone, the strongest next steps would be:
- one more honest native-desktop family
- one more helper/service-aware family
- or stronger deterministic runtime/helper foundations

But first the ChattyCog module lane should match the real ecosystem contract more closely than it does today.
