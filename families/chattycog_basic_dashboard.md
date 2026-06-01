# Family Spec: `chattycog_basic_dashboard`

- `family_id`: `chattycog_basic_dashboard`
- `status`: planned
- `priority`: tier_1
- `primary_substrate`: static_web_with_wrapper
- `supports_chattycog_wrapper`: yes
- `supports_standalone`: yes

## Purpose

Produce a starter dashboard that works as a standalone web project and also includes the standard files needed to drop it into ChattyCog as a module with minimal manual cleanup.

This family exists because "give me a basic module starter and we will patch it up from there" is a recurring and valuable request shape.

## Best-Fit Requests

- "make me a basic ChattyCog module"
- "start a dashboard module for ChattyCog"
- "make a simple module base"
- "build a basic module starter with a dashboard"

## Not-A-Fit Requests

- requests for non-dashboard module types
- requests that explicitly want native Rust or Python module substrate instead of web
- advanced custom bridge or host integration beyond the starter surface

## Required Inputs

- project/module name
- display title
- summary/description
- requested dashboard feature tokens

## Optional Inputs

- icon hint
- initial operator bundle
- style preset
- starter notes for `HANDSHAKE.md`

## Generated Outputs

- standalone dashboard files:
  - `index.html`
  - `app.js`
  - `styles.css`
- ChattyCog wrapper/integration files:
  - `manifest.json`
  - `visual_load.json`
  - `HANDSHAKE.md`
  - bridge JS file
- `ProjectSpec.json`
- `COCKPIT_PROTOCOL.md`
- route receipt artifacts

## Host-Owned Machinery

- dashboard scaffold rendering
- deterministic ChattyCog wrapper emission
- metadata population
- bridge file wiring
- schema validation for wrapper metadata
- starter acceptance assembly

## LLM Responsibilities

- clarify whether the user really wants a starter module versus a fully custom build
- choose or confirm initial dashboard/operator emphasis
- review output quality if multiple bundles were plausible

## Acceptance Pack

- all standalone files exist
- all ChattyCog wrapper files exist
- `manifest.json` validates
- `visual_load.json` validates
- `index.html` references the bridge file correctly
- route receipt exists

## Common Failure Classes

- missing wrapper file
- invalid wrapper metadata
- broken bridge reference
- operator mismatch
- unsupported request beyond starter scope

## Repair Lanes

- re-emit wrapper metadata
- re-emit bridge file and references
- re-render dashboard scaffold
- route to another module family if the starter family was the wrong fit

## Route Notes

This family should become the default deterministic starting point for quick ChattyCog-compatible module work.

It is also a strategic family because it proves the rebuild can be:
- standalone first
- wrapper-capable second
- mechanically generated in both layers
