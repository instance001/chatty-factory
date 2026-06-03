# ChattyCog Module Skeleton Integration Milestone

## Purpose

This milestone defines the first real bridge between ChattyCog and
ChattyFactory.

The goal is not yet a full end-to-end checkbox pipeline.

The goal is to make ChattyFactory able to deterministically build a
**standalone Rust GUI dashboard skeleton with prefilled ChattyCog compatibility
plug files**.

That means the emitted project should be:

- a real standalone native Rust dashboard by default
- runnable outside ChattyCog on its own
- loadable inside ChattyCog as a drop-in module when the plug files are present

If those ChattyCog plug files are removed, the tool should lose ChattyCog
compatibility but still remain a standalone dashboard build.

That skeleton becomes the stable substrate for later:

- deterministic feature patching
- negative constraint learning
- ChattyCog sandbox plan handoff
- eventual checkbox-style module assembly and handback

## Correct Product Posture

This milestone is intentionally **not** about building a ChattyCog-only module
first.

It is about building:

- a standalone Rust GUI dashboard

and then layering in:

- ChattyCog discovery
- ChattyCog hosting
- ChattyCog handshake
- ChattyCog bridge/context sharing

So the emitted project should be understood as:

- `standalone_rust_dashboard`
- plus
- `chattycog_compatibility_plugs`

That is the right substrate for the future loop where ChattyCog orchestrates
deterministic build plans and ChattyFactory executes them.

## Why This Comes First

Right now, richer ChattyCog outputs are useful, but they are still more
specialized than ideal as the base substrate.

A canonical standalone-native skeleton gives us:

- a predictable patch target
- a stable verification target
- a known-good module contract
- a cleaner separation between:
  - “this dashboard shell exists”
  - and
  - “this dashboard now does X”

That matters because the future product loop we want is:

1. ChattyCog sandbox derives a deterministic implementation plan
2. ChattyFactory receives that plan
3. ChattyFactory executes the checked deterministic steps
4. ChattyFactory emits a ready-made standalone dashboard with ChattyCog module
   compatibility
5. ChattyCog can accept that output back into its module folder and host it in
   a tab

That loop is much easier if the shell itself is canonical.

## Reference Basis

This milestone is grounded in the reference-only `chattycog/` addition now
present in the repo root:

- `chattycog/chattycog_gui/docs/MODULES.md`
- `chattycog/chattycog_gui/docs/MODULE_HANDSHAKE_TEMPLATE.md`
- `chattycog/chattycog_gui/docs/MODULE_VISUAL_LOAD.md`
- `chattycog/chattycog_gui/docs/MODULE_BRIDGE.md`
- `chattycog/chattycog_gui/module_templates/template_module/`
- `chattycog/chattycog_gui/module_templates/template_native_rust_module/`

The important architectural read from those references is:

- a ChattyCog module is not magic
- it is a normal tool plus a known module-enablement contract
- that contract can be emitted deterministically
- for our target case, that normal tool should be a native Rust dashboard

## Immediate Delivery Goal

Add a new deterministic ChattyFactory build target for a **blank standalone
Rust GUI dashboard skeleton with ChattyCog plug files prefilled**.

This should not yet try to be a full special-purpose module.

It should be:

- a real standalone native Rust dashboard by default
- ChattyCog-loadable when the compatibility files are present
- based on a native hosted path, not a webview-first posture

The compatibility layer should provide:

- discovery contract
- visual load contract
- handshake contract
- bridge contract
- context-sharing starter files

## Suggested First Shape

The first emitted skeleton should target the native hosted path:

- `chattycog_native_window_module`

But it should be understood more honestly as:

- `standalone_rust_dashboard_with_chattycog_compatibility`

That gives us:

- a real native Rust GUI dashboard as the base product
- a known hosted-native ChattyCog path
- a removable compatibility layer
- easier eventual module-folder packaging

Webview can remain a later or alternate substrate, but it should not be the
primary first target if the actual design intent is standalone native tools
that can also be loaded as ChattyCog modules.

## Required Emitted File Set

The first canonical standalone-native skeleton should emit at least:

- `Cargo.toml`
- `src/main.rs`
- `manifest.json`
- `visual_load.json`
- `HANDSHAKE.md`
- `STATE_TEMPLATE.md`
- `ChattyCogModuleSpec.json`
- `ProjectSpec.json`
- `AcceptancePlan.json`
- `bridge/status.json`

Recommended optional starter files:

- `bridge/shared_room_state.json`
- `bridge/shared_room_events.json`
- `bridge/outgoing_room_events.json`
- `bridge/log_sources.json`
- `bridge/incoming_assets/module_assets/.keep`
- `network_capabilities.json`
- `README.md`

These are not abstract “nice to have” additions. They correspond directly to
the ChattyCog discovery/hosting/bridge model visible in the reference docs.

## Intended Contract Layers

### 1. Standalone App Layer

Required for the project to be a real standalone tool:

- `Cargo.toml`
- `src/main.rs`

The project must remain runnable as a normal Rust dashboard outside ChattyCog.

### 2. Discovery Layer

Required for ChattyCog to see the module:

- `manifest.json`

Minimum fields should align with the reference model:

- `module_id`
- `display_name`
- `icon`
- `description`

### 3. Hosted Native Visual Surface

Required for ChattyCog to host the real native UI:

- `visual_load.json`

First milestone default:

- `kind = native_window`
- `auto_launch = true`
- `launch.program` points at the built Rust dashboard executable
- `window_title_contains` matches the real native app title

That keeps the base tool truly standalone while still letting ChattyCog dock
the real native window into a tab.

### 4. Human Handoff Layer

Required for module identity and future suspend-rundown continuity:

- `HANDSHAKE.md`

This should be prefilled, not empty, with:

- module identity
- declared purpose
- expected inputs
- expected outputs
- suspend rundown template

### 5. Portable Bridge Layer

Required for ChattyCog compatibility without making the dashboard
ChattyCog-only:

- `bridge/status.json`

Recommended from day one:

- `bridge/shared_room_state.json`
- `bridge/shared_room_events.json`
- `bridge/outgoing_room_events.json`
- `bridge/log_sources.json`

The key principle is:

- removing these files should leave a normal standalone Rust dashboard
- keeping them should make that dashboard ChattyCog-ready

### 6. Context / Room Starter Layer

These files are the future bridge between a simple hosted dashboard and a
room-aware/context-sharing module:

- `bridge/shared_room_state.json`
- `bridge/shared_room_events.json`
- `bridge/outgoing_room_events.json`
- `network_capabilities.json`

Even if the first blank dashboard does not use them deeply, pre-seeding them
gives future patch lanes a stable place to attach richer room-sharing features.

## Acceptance Contract For The Blank Skeleton

The first blank standalone-native skeleton should be considered successful only
if:

1. Standalone app contract passes
   - the Rust dashboard builds
   - the native app entrypoint exists

2. Discovery contract passes
   - `manifest.json` exists
   - required manifest fields are present

3. Visual load contract passes
   - `visual_load.json` exists
   - it points at a real native hosted surface

4. Bridge contract passes
   - `bridge/status.json` exists

5. Handshake contract passes
   - `HANDSHAKE.md` exists
   - module identity fields are prefilled

6. Module spec contract passes
   - `ChattyCogModuleSpec.json` remains aligned with emitted files

7. Standalone-to-hosted compatibility passes
   - removing ChattyCog plug files should remove compatibility
   - but should not destroy the standalone dashboard itself

## Minimal UI For The Blank Shell

The first blank shell should not try to be feature-rich.

It should show:

- module identity
- handshake summary
- bridge status area
- room/context placeholder area
- “ready for patch layering” status copy

This UI should be native Rust GUI, not hosted HTML pretending to be the final
product shape.

The goal is to create a stable patch substrate, not a pretty demo.

## How Future Patches Should Attach

The blank skeleton exists so later deterministic patches can do things like:

- add helper-backed inbox monitoring
- add a bridge activity panel
- add shared-room event views
- add domain-specific dashboards
- add file processing panes
- add module-specific actions

In other words:

- shell first
- behavior second

That keeps ChattyCog-compatible dashboard evolution closer to:

- “patch this dashboard to do X”

instead of:

- “invent a new ChattyCog contract from scratch each time”

## ChattyCog Sandbox Handoff Direction

This milestone should also define the future plan handoff shape, even if the
first implementation does not execute it yet.

Suggested future artifact:

- `ChattyCogModuleBuildPlan`

Suggested fields:

- `plan_id`
- `source_system = "chattycog"`
- `module_posture`
- `target_family_id`
- `requested_hosting_mode`
- `module_identity`
- `selected_deterministic_steps`
- `selected_patch_kinds`
- `selected_helper_ids`
- `selected_bridge_capabilities`
- `selected_acceptance_recipe_ids`
- `notes`
- `created_at`

The important idea is:

- ChattyCog can remain the orchestration/sandbox side
- ChattyFactory can remain the deterministic execution side

## Delivery Order

### Wave 1: Reference Mapping

Use the `chattycog/` reference folder to derive the real minimal module
contract for a native hosted dashboard.

Deliverables:

- this milestone doc
- a mapped file inventory
- explicit first hosting target choice

### Wave 2: Blank Native Rust Skeleton

Add a deterministic `chattycog_native_window_module` blank-shell build posture
in ChattyFactory for a standalone Rust GUI dashboard with ChattyCog plug files.

Deliverables:

- emitted skeleton files
- acceptance contract
- proof build

### Wave 3: Verification Alignment

Make ChattyFactory verify the skeleton against the real ChattyCog-style module
contract, not just generic file existence.

Deliverables:

- stronger ChattyCog module verification
- targeted failure classification for missing handshake/bridge/load surfaces

### Wave 4: Patch-On-Shell

Start treating “module does X” as deterministic patches on top of the blank
native shell.

Deliverables:

- at least one proof patch that upgrades the blank shell into a useful module

### Wave 5: Plan Handoff Contract

Define and prove the first machine-readable handoff from ChattyCog sandbox plan
to ChattyFactory deterministic execution plan.

Deliverables:

- handoff contract
- one sample plan
- one sample host execution against that plan

## First Proof Target

The first proof should be intentionally boring:

- build a blank standalone Rust GUI dashboard with ChattyCog compatibility
  plugs
- prove it emits the real ChattyCog contract files
- prove it verifies cleanly as a ChattyCog-loadable native module
- prove it is still fundamentally a standalone dashboard build

Only then should we patch:

- “module does X”

onto it.

## Intended Outcome

If this milestone succeeds, ChattyFactory gains:

- a canonical standalone-native dashboard substrate with ChattyCog
  compatibility
- a safer patch base for ChattyCog-facing features
- a cleaner future integration point with ChattyCog sandbox planning

And ChattyCog gains:

- a deterministic build engine it can eventually hand checked plans to

This is the right bridge if we want the two systems to cooperate without
collapsing into one blurry tool.
