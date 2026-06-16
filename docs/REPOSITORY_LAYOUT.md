# Repository Layout Proposal

Historical note:
- this is a proposal-era layout document
- use [CURRENT_ARCHITECTURE.md](./CURRENT_ARCHITECTURE.md) for the current implementation shape

This document proposes the initial layout for the `chatty-factory/` rebuild.

The goal is not to perfectly predict the final tree. The goal is to give the rebuild clean architectural seams from the start so we do not drag prototype coupling into the new codebase.

## Layout Goals

- separate product architecture from prototype history
- keep contracts and templates visible
- make family/operator machinery first-class
- keep room for both standalone and ChattyCog wrapper output
- support host-owned deterministic systems without hiding them in one giant crate

## Proposed Top-Level Tree

- `chatty-factory/README.md`
- `chatty-factory/build-docs/plans/REBUILD_PLAN.md`
- `chatty-factory/docs/`
- `chatty-factory/crates/`
- `chatty-factory/contracts/`
- `chatty-factory/families/`
- `chatty-factory/operator_registry/`
- `chatty-factory/templates/`
- `chatty-factory/schemas/`
- `chatty-factory/examples/`
- `chatty-factory/scripts/`

## Proposed Crates

### `crates/chatty_factory_core`

Purpose:
- shared domain types
- request normalization
- route/control types
- family/operator registries
- contract models

Owns:
- typed records
- ids
- enums
- schema helpers
- route receipts

Should not own:
- GUI code
- direct template assets
- prototype-oriented prompt flows

### `crates/chatty_factory_control`

Purpose:
- control plane graph
- route transitions
- retry transitions
- route explanation artifacts

Likely dependencies:
- `petgraph`

Owns:
- node/edge types
- route selection orchestration
- retry policy mapping

### `crates/chatty_factory_families`

Purpose:
- first-class family implementations
- family registry
- scaffold input resolution
- family-specific acceptance builders

Owns:
- `static_web_dashboard`
- `chattycog_basic_dashboard`
- `python_cli_tool`
- `rust_cli_tool`

Should expose:
- family descriptors
- family builders
- family acceptance plans
- family repair lane descriptors

### `crates/chatty_factory_operators`

Purpose:
- mechanical operator execution

Owns:
- DOM operators
- JSON patch operators
- wrapper operators
- bounded behavior emitters
- helper bridge operators

Likely dependencies:
- `dom_query`
- `json-patch`

### `crates/chatty_factory_templates`

Purpose:
- template loading and rendering

Likely dependencies:
- `minijinja`

Owns:
- template environment
- render helpers
- shared context adapters

Should not own:
- route policy
- acceptance logic

### `crates/chatty_factory_verify`

Purpose:
- schema validation
- syntax/build/runtime checks
- acceptance execution
- failure capture

Likely dependencies:
- `jsonschema`
- `regex_automata`

Owns:
- acceptance runners
- validation reports
- failure reports

### `crates/chatty_factory_cli`

Purpose:
- minimal early shell for local development

Why first:
- cheaper than building a full GUI immediately
- enough to test milestone-one family flows

Could later support:
- request input
- dry-run route display
- build execution
- receipt inspection

### `crates/chatty_factory_gui`

Purpose:
- later desktop control surface if desired

This should come after the core/family/control stack is real, unless a GUI becomes immediately necessary for workflow reasons.

## Contracts Folder

`contracts/` should hold human-readable contract definitions and examples.

Suggested contents:
- `request_record.md`
- `route_decision.md`
- `acceptance_plan.md`
- `failure_report.md`
- `project_spec.md`

This folder is for design visibility.
Machine-readable schemas belong in `schemas/`.

## Families Folder

`families/` should hold the family specs and, later, possibly data-driven family manifests.

Suggested contents now:
- planning/spec documents

Possible later contents:
- `family_manifest.json`
- capability descriptors
- repair lane descriptors

## Operator Registry Folder

`operator_registry/` should hold:
- operator definitions
- operator eligibility notes
- operator acceptance contributions

Suggested initial separation:
- `dom/`
- `json/`
- `wrapper/`
- `behavior/`
- `helper/`

## Templates Folder

`templates/` should be explicit, visible, and versionable.

Suggested shape:
- `templates/families/static_web_dashboard/`
- `templates/families/chattycog_basic_dashboard/`
- `templates/families/python_cli_tool/`
- `templates/families/rust_cli_tool/`
- `templates/wrappers/chattycog/`
- `templates/shared/`

## Schemas Folder

`schemas/` should hold JSON Schemas for machine-owned artifacts.

Suggested initial schemas:
- `request_record.schema.json`
- `route_decision.schema.json`
- `acceptance_plan.schema.json`
- `failure_report.schema.json`
- `project_spec.schema.json`
- `manifest.schema.json`
- `visual_load.schema.json`

## Examples Folder

`examples/` should prove the family system with small representative inputs.

Suggested contents:
- one request example per family
- one route decision example
- one acceptance plan example
- one failure report example

## Scripts Folder

`scripts/` should only hold narrow helper utilities for local dev and packaging.

It should not become the new hiding place for core product logic.

## Initial Recommendation

Start with:
- `chatty_factory_core`
- `chatty_factory_control`
- `chatty_factory_families`
- `chatty_factory_templates`
- `chatty_factory_verify`
- `chatty_factory_cli`

Then add:
- `chatty_factory_operators`

as soon as DOM/JSON/wrapper operator work becomes concrete enough to justify the split.

This gives us enough structure to avoid a monolith without over-fragmenting the first milestone.
