# Family Spec: `rust_cli_tool`

- `family_id`: `rust_cli_tool`
- `status`: planned
- `priority`: tier_2
- `primary_substrate`: rust_cli
- `supports_chattycog_wrapper`: no
- `supports_standalone`: yes

## Purpose

Build a deterministic Rust CLI tool through family templates, cargo-aware scaffolding, mechanical acceptance, and repair lanes that do not depend on broad freeform code generation.

This family matters because it tests whether the rebuild can support stricter non-web substrates honestly rather than quietly collapsing them back to browser output.

## Best-Fit Requests

- "build a Rust CLI"
- "make a small compiled tool"
- "make a local utility in Rust"
- requests that explicitly value Rust over Python or Node

## Not-A-Fit Requests

- browser-style interactive tools
- rapid exploratory tools where Python is the better honest default
- requests needing native GUI rather than CLI

## Required Inputs

- project name
- tool summary
- explicit Rust preference or strong compiled-tool fit
- input/output expectations if relevant

## Optional Inputs

- crate naming preferences
- argument conventions
- fixture preferences
- edition/tooling defaults if exposed later

## Generated Outputs

- `Cargo.toml`
- `src/main.rs`
- optional supporting modules
- `README.md`
- `ProjectSpec.json`
- `COCKPIT_PROTOCOL.md`
- fixture files when required for acceptance
- route receipt artifacts

## Host-Owned Machinery

- cargo project scaffold templates
- deterministic `Cargo.toml` emission
- entrypoint template emission
- fixture generation
- cargo-aware acceptance generation
- contract emission

## LLM Responsibilities

- clarify behavior if the tool request is underspecified
- choose between competing CLI interpretations
- review result quality when compile success does not fully prove semantic success

## Acceptance Pack

- required files exist
- `Cargo.toml` validates structurally
- `cargo check` or equivalent family smoke check passes
- deterministic run or output check passes when fixtures apply
- `ProjectSpec.json` validates

## Common Failure Classes

- invalid cargo metadata
- compile failure
- fixture mismatch
- runtime output mismatch
- route mismatch where Rust was requested but family support is insufficient

## Repair Lanes

- re-emit cargo scaffold
- re-emit entrypoint/module templates
- regenerate fixtures
- rerun cargo-oriented acceptance
- escalate to foreman review when explicit Rust intent conflicts with current family coverage

## Route Notes

This family should not be added just to claim agnosticism on paper.

It should be added when the host can genuinely own enough of:
- scaffold generation
- metadata correctness
- acceptance
- repairability

that Rust becomes an honest deterministic route rather than a fragile aspiration.
