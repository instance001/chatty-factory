# Chatty-EDU Native Starter Integration Milestone

## Purpose

Add a first-class deterministic starter lane for Chatty-EDU that mirrors the
mechanical selection posture we now use for Chatty-Cog, while respecting the
fact that Chatty-EDU uses a **separate handshake and bridge contract**.

This milestone exists because Chatty-EDU is not just a renamed Chatty-Cog
surface:

- it is a separate ecosystem
- it uses the same broad idea of drop-in module hosting
- but its hosted handoff posture is **not** contract-compatible with Chatty-Cog

So the correct build posture is:

- standalone Rust GUI dashboard first
- Chatty-EDU compatibility plug files second
- no fake cross-ecosystem handshake unification

## Product Goal

The factory should be able to build a native Rust dashboard that:

- runs as a normal standalone desktop tool
- can be dropped into a Chatty-EDU module folder
- can be hosted inside a Chatty-EDU native-window tab
- can write suspend/handoff bridge state through the Chatty-EDU bridge
- can lose Chatty-EDU compatibility cleanly if the plug files are removed

## Architectural Call

Do **not** force Chatty-EDU through the Chatty-Cog contract model.

Instead:

- keep the general starter pattern the same
- keep the build substrate the same class: standalone Rust GUI dashboard
- give Chatty-EDU its own:
  - family id
  - starter selection id
  - manifest/visual-load expectations
  - bridge env var contract
  - module spec artifact
  - acceptance checks

This preserves the good part of the factory:

- one mechanistic starter-selection model

without collapsing ecosystem-specific truth:

- different host ecosystems can require different plug-file contracts

## Implemented First Slice

The first integrated Chatty-EDU lane is:

- `chattyedu_native_window_module`

It is:

- standalone by nature
- native Rust + `eframe`
- mechanically selectable from:
  - UI starter picker
  - CLI `--starter`
- emitted as a deterministic family build

## Emitted Files

The starter currently emits:

- `Cargo.toml`
- `src/main.rs`
- `README.md`
- `STATE_TEMPLATE.md`
- `manifest.json`
- `visual_load.json`
- `HANDSHAKE.md`
- `network_capabilities.json`
- `bridge/.gitkeep`
- `ChattyEduModuleSpec.json`
- `ProjectSpec.json`
- `AcceptancePlan.json`

## Bridge Contract

Unlike Chatty-Cog, the starter uses the Chatty-EDU-native bridge posture:

- `CHATTYEDU_BRIDGE_STATUS`

The standalone app:

- ignores that bridge outside Chatty-EDU
- writes handoff state only when the environment variable exists

That keeps the app portable:

- add the plug -> Chatty-EDU-compatible
- remove the plug -> standalone-only

## Verification Posture

The lane now has a first dedicated contract layer:

- `ChattyEduModuleSpec`
- `chattyedu_module_contract`
- `chattyedu_visual_load_contract`

Current verification checks:

- manifest exists and matches module identity
- handshake exists and contains module identity
- visual load exists and uses `native_window`
- `src/main.rs` includes the `CHATTYEDU_BRIDGE_STATUS` bridge posture
- `network_capabilities.json` exists with a features array
- self-test path still proves the standalone app is runnable

## Mechanical Selection Requirement

This lane is intentionally not left to broad LLM guessing alone.

It should be selectable mechanically through:

- starter picker in the desktop UI
- `cargo run -p chatty_factory_cli -- build --starter chattyedu_native_window_module ...`

The recommendation/routing layer may still suggest it, but operator-controlled
starter selection remains the stronger truth signal.

## Why This Matters

This milestone proves the factory can support **multiple ecosystem-native
starter families** without flattening them into one fake generic module system.

That is strategically important because the future factory likely needs:

- shared build mechanics
- shared governance and verification patterns
- but not shared handshake truth where the ecosystems genuinely differ

## Immediate Next Steps

1. Surface Chatty-EDU starter lifecycle in the same family governance views as the Chatty-Cog native starter.
2. Add a lightweight project/governance note for Chatty-EDU outputs so they read as ecosystem-native builds, not generic desktop tools.
3. Decide whether Chatty-EDU should get:
   - its own additional starter variants later
   - or only this native Rust shell as the primary forward lane
4. Only after that, consider deeper patch-lane work on top of the shell.

## Success Condition

This milestone is successful when:

- Chatty-EDU has a first-class deterministic native starter family
- the lane is mechanically selectable
- the starter emits a real standalone Rust GUI dashboard
- the starter emits Chatty-EDU-specific compatibility files
- the host verifies those files through a dedicated contract path
- the architecture treats Chatty-EDU as a truthful ecosystem peer, not a hacked alias of Chatty-Cog
