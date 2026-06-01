# Family Spec: `python_cli_tool`

- `family_id`: `python_cli_tool`
- `status`: planned
- `priority`: tier_2
- `primary_substrate`: python_cli
- `supports_chattycog_wrapper`: no
- `supports_standalone`: yes

## Purpose

Build a deterministic Python CLI tool from templates, structured inputs, mechanical file emission, and family-specific acceptance.

This family is one of the main ways the rebuild moves back toward honest agnostic builds instead of flattening everything into web.

## Best-Fit Requests

- "build a python CLI that does X"
- "make a small local utility"
- "make a file sorter"
- "make a converter tool"
- "make a script I can run with arguments"

## Not-A-Fit Requests

- browser-first interactive tools
- requests needing long-lived GUI surface
- requests that explicitly want Rust or Node instead

## Required Inputs

- project name
- tool summary
- command purpose
- input/output expectations if relevant
- explicit Python request or strong CLI fit

## Optional Inputs

- argument pattern
- fixture preferences
- package naming
- README/cockpit detail level

## Generated Outputs

- `main.py`
- optional supporting Python modules from family rules
- `README.md`
- `ProjectSpec.json`
- `COCKPIT_PROTOCOL.md`
- fixture files when deterministic acceptance needs them
- route receipt artifacts

## Host-Owned Machinery

- template-backed CLI scaffold
- argument skeletons for common request classes
- fixture generation
- acceptance command generation
- syntax smoke checks
- contract emission

## LLM Responsibilities

- clarify ambiguous behavior requirements
- choose between multiple CLI sub-shapes if needed
- review final behavior when acceptance is too weak to prove semantics completely

## Acceptance Pack

- required files exist
- `main.py` passes syntax check
- deterministic fixture inputs are present if needed
- CLI run exits successfully
- expected output artifact or output token exists
- `ProjectSpec.json` validates

## Common Failure Classes

- syntax failure
- fixture mismatch
- missing output artifact
- argument contract mismatch
- unsupported capability hidden inside a "simple script" ask

## Repair Lanes

- re-emit entrypoint from family template
- regenerate fixtures
- patch argument wiring deterministically
- regenerate acceptance commands
- escalate to foreman review if request shape was misclassified

## Route Notes

This family is a major test of whether the rebuild can reclaim non-web builds through host-owned machinery.

If this family still requires broad model-written code for common utilities, the rebuild has not pulled enough machinery down into the host.
