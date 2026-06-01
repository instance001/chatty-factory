# Family Spec Template

Use this template for first-class build families in the rebuild.

Each family spec should be concrete enough that the host can:
- classify into the family
- gather scaffold inputs
- emit files mechanically
- validate outcomes
- repair common failures without broad planning

## Header

- `family_id`
- `status`
- `priority`
- `primary_substrate`
- `supports_chattycog_wrapper`
- `supports_standalone`

## Purpose

What this family is for.

## Best-Fit Requests

What plain-language asks should map here.

## Not-A-Fit Requests

What should be routed elsewhere.

## Required Inputs

What structured request data the host needs before build.

## Optional Inputs

What can tune output without changing the family.

## Generated Outputs

What files/artifacts the host should emit.

## Host-Owned Machinery

What exact work should be mechanical.

## LLM Responsibilities

What the model is still allowed to do for this family.

## Acceptance Pack

What the host should verify.

## Common Failure Classes

What failures are likely for this family.

## Repair Lanes

What the host should try before escalating.

## Route Notes

Why this family exists and where it sits in the substrate strategy.
