# Initial GitHub Release Plan

## Recommended Version

- `v0.1.0`

This is the right first public cut because the system now has a coherent
product story rather than a loose collection of experiments.

## License Posture

The initial GitHub upload should be explicitly framed as:

- GNU Affero General Public License v3.0 only
- SPDX identifier: `AGPL-3.0-only`

Release prep should keep that visible in:

- repo `README.md`
- `LICENSE`
- workspace `Cargo.toml`
- release notes

## Why This Is a Good Release Point

The current build is the first one that honestly supports the intended product
shape:

- plain-language request in
- real build artifacts out
- plain-language follow-up patching against existing projects
- host-owned safety and interpretability around patch surgery

The release is especially meaningful because the patch path is no longer a
blind edit pipeline. It now includes:

1. diagnosis
2. intent freeze
3. preflight structural guard checks
4. postcheck
5. project-level patchability governance

## Proposed Release Theme

`ChattyFactory v0.1.0: deterministic build and diagnosis-aware patching`

## Recommended Scope

The initial GitHub release should emphasize:

- deterministic multi-family build pipeline
- deterministic patch pipeline
- governed extension and proof infrastructure
- patch X-ray artifacts and UI
- project patchability governance
- modern replacement guidance for intentionally historical lanes

It should not overclaim:

- arbitrary codebase support
- universal safe patching across unknown projects
- full autonomous software engineering beyond the current family/contract model

## Release Artifacts To Include

- source snapshot
- release notes
- optional screenshots of:
  - project browser patchability badges
  - patch X-ray panel
  - extension/governance surfaces

## Pre-Release Checklist

1. Confirm the intended git repository boundary.
2. Review `cargo check` from the current workspace.
3. Decide whether to tag the current state as `v0.1.0` or `v0.1.0-alpha.1`.
4. Add a short screenshot set if the GitHub release page should be more
   legible to first-time readers.
5. Publish release notes from `RELEASE_NOTES_v0.1.0.md`.
6. Verify the GitHub repo description and release page both mention AGPLv3.

## Current Constraint

The `chatty-factory` workspace is not currently inside an initialized `.git`
repository from this execution context, so branch/tag inspection could not be
performed during release prep.

That does not block note drafting, but it does mean the final shipping steps
should begin with a quick repository-boundary sanity check before tagging.
