# Family Spec: `static_web_dashboard`

- `family_id`: `static_web_dashboard`
- `status`: planned
- `priority`: tier_1
- `primary_substrate`: static_web
- `supports_chattycog_wrapper`: yes
- `supports_standalone`: yes

## Purpose

Build a standalone dashboard-style browser tool from deterministic scaffolds and operator bundles, with the host owning structure, copy slots, bounded behavior hooks, contracts, and acceptance.

This is one of the core proving grounds for the rebuild because it keeps the strongest part of the prototype while removing as much model-authored implementation as possible.

## Best-Fit Requests

- "build me a simple dashboard"
- "make a browser dashboard for tracking X"
- "make a small analytics panel"
- "build a local dashboard with cards and a table"
- "start me a basic dashboard app"

## Not-A-Fit Requests

- requests that explicitly require non-web output
- requests needing real local-native capability without a helper bridge
- requests that are really better expressed as a CLI
- advanced custom front-end work beyond the bounded operator set

## Required Inputs

- project name
- title
- dashboard summary
- requested feature tokens
- exoskeleton target

## Optional Inputs

- preferred copy tone
- preferred operator bundle
- bounded behavior kind
- color/style preset

## Generated Outputs

- `index.html`
- `app.js`
- `styles.css`
- `ProjectSpec.json`
- `COCKPIT_PROTOCOL.md`
- route receipt artifacts

Optional:
- wrapper files if ChattyCog compatibility is selected

## Host-Owned Machinery

- scaffold rendering from templates
- operator bundle selection from native ids
- DOM-safe structural assembly
- deterministic ids/classes/data hooks
- bounded behavior insertion
- `ProjectSpec.json` emission
- acceptance pack assembly

## LLM Responsibilities

- clarify ambiguous requests
- choose between competing dashboard/operator interpretations
- optionally rank plausible operator bundles
- review output when checks are mixed or user-facing quality is uncertain

## Acceptance Pack

- required files exist
- `index.html` references expected JS/CSS assets
- selected operator markers are present
- bounded behavior markers are present when chosen
- `ProjectSpec.json` validates against schema
- route receipt exists

## Common Failure Classes

- missing scaffold output
- operator composition mismatch
- DOM assembly mismatch
- missing expected markers
- schema failure
- unsupported requested capability

## Repair Lanes

- re-render scaffold
- re-apply operator bundle
- re-emit bounded behavior block
- re-patch contract/metadata
- escalate to foreman review if operator set was likely wrong

## Route Notes

This is the strongest early family because it is already closest to the prototype's proven path.

It is important that this family stay:
- strong
- inspectable
- clearly bounded

without becoming the identity of the whole rebuild.
