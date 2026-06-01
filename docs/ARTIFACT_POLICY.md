# Artifact Policy

This document defines which rebuild artifacts are meant to be durable product output and which are only local execution byproducts.

The goal is to stop generated projects from becoming a fuzzy mix of:
- real shipped files
- fixtures used for acceptance
- local compile caches
- runtime receipts

## Primary Rule

`output/<project>/` is for durable project-facing artifacts.

`runtime/` is for factory-facing receipts, logs, plans, and supervision state.

Local compiler caches and temporary build byproducts should not be treated as durable product output unless a family explicitly says otherwise.

## Durable Output

These belong in `output/<project>/`:

- source files that define the built tool
- deterministic wrapper files
- `ProjectSpec.json`
- `AcceptancePlan.json`
- family-owned fixtures that are part of the definition of done
- module bridge stubs that are part of the portable module contract

Examples:
- `index.html`
- `app.js`
- `styles.css`
- `main.py`
- `Cargo.toml`
- `src/main.rs`
- `manifest.json`
- `visual_load.json`
- `HANDSHAKE.md`
- `bridge/status.json`

## Factory Runtime State

These belong in `runtime/`:

- requests
- plans
- routes
- scaffold inputs
- execution policies
- execution receipts
- project snapshots
- context bundles
- snapshot gates
- acceptance results
- planner handoffs
- planner responses
- fallback receipts

These are supervision artifacts, not part of the generated project itself.

## Allowed Fixture Content

Fixtures are allowed in output projects when they are part of deterministic verification.

Examples:
- `fixtures/input/`
- `fixtures/output/`

The rule is:
- fixtures may live beside the built project
- but they should be clearly separated from the tool's real source and wrapper files

## Disposable Build Byproducts

These should be treated as disposable unless a family explicitly opts in:

- Rust `target/`
- Python `__pycache__/`
- temporary logs written only for local checks
- one-off local server artifacts

They are useful for local execution, but they are not canonical product artifacts.

## Family Guidance

Families should follow this default:

- emit only durable project files and deterministic fixtures into `output/<project>/`
- keep execution receipts in `runtime/`
- avoid treating local compile caches as part of the built result

If a family must generate a local cache or build folder for acceptance, that should be considered tolerated local clutter, not part of the product contract.

## Near-Term Cleanup Direction

The rebuild should gradually move toward:

- excluding `target/` and similar caches from snapshots where appropriate
- documenting tolerated byproducts per family
- optionally adding cleanup helpers for disposable compile output

The important line is:
- product files stay in output
- factory state stays in runtime
- disposable build clutter is not mistaken for either
