# Tooling Policy

ChattyFactory rebuild policy: use free/open host-side tools aggressively when they remove fragile model work.

## Why

The rebuild is trying to move the LLM out of:
- exact file surgery
- repetitive scaffolding
- metadata shaping
- mechanical validation
- broad failure diagnosis

and move it back into:
- route choice
- ambiguity handling
- quality review
- bounded fallback judgment

That means external tools are welcome when they let the host perform exact work more reliably.

## Good Reasons To Adopt A Tool

- it removes a repeated failure class
- it widens honest substrate support
- it improves deterministic acceptance
- it replaces brittle string surgery
- it shrinks the fallback surface that still requires freeform planning

## Bad Reasons To Adopt A Tool

- it feels fancy
- it duplicates something the host can already do simply
- it introduces heavy complexity without reducing LLM entropy
- it hides product logic inside an opaque dependency

## Preferred Tool Categories

- template engines
- schema validators
- JSON patching tools
- DOM/HTML manipulation tools
- AST/source editing tools
- workflow/control-plane libraries
- failure classification libraries
- deterministic CLI helpers

## Adoption Rule

When we find a tool that is:
- free/open
- practical to wrap
- deterministic enough
- useful offline or locally

and it can replace fragile LLM machinery, we should seriously consider bringing it in.
