# GitHub Upload Metadata

This file is a ready-to-use metadata pack for the initial GitHub upload.

## Repository Name

- `chatty-factory`

## Visibility Recommendation

- public, if the goal is to publish the AGPLv3-licensed build as an open
  source release

## License

- GNU Affero General Public License v3.0 only
- SPDX: `AGPL-3.0-only`

## GitHub About Description

Deterministic build-and-patch factory for plain-language software requests,
with diagnosis-aware patching and project patchability governance. AGPLv3.

## Short Repo Intro

ChattyFactory is a local deterministic build-and-patch system that turns
plain-language requests into real generated projects, then supports
follow-up patching with host-owned diagnosis, intent freezing, structural
preflight checks, and project-level patchability governance.

## Suggested README Intro Blurb

ChattyFactory is a deterministic build-and-patch factory for plain-language
software requests. It generates real project artifacts within a bounded
family/contract model, then supports follow-up patching against existing
projects with diagnosis-aware safety mechanisms instead of blind text edits.

This repository is licensed under the GNU Affero General Public License v3.0
only (`AGPL-3.0-only`).

## Suggested GitHub Topics

- `agplv3`
- `rust`
- `llm-tools`
- `code-generation`
- `deterministic-systems`
- `software-factory`
- `patching`
- `developer-tools`
- `local-first`
- `desktop-ui`

## Suggested Website Field

If no external website exists yet:

- leave blank

If a documentation or project page is added later:

- point it at the public project overview, not a private planning surface

## Suggested Release Title

- `ChattyFactory v0.1.0`

## Suggested Release Subtitle

- `Deterministic build and diagnosis-aware patching`

## Suggested First Sentence For Release Body

ChattyFactory v0.1.0 is the first public cut where plain-language build
requests, deterministic patching, and diagnosis-aware patch safety form a
coherent end-to-end product story.

## Suggested Pinned Notes

If the repository uses pinned issues or a pinned discussion, the best first
messages would be:

1. what ChattyFactory is
2. what it does today
3. what its current boundaries are
4. that it is licensed under AGPLv3

## Upload Checklist

1. Confirm `LICENSE` is present in the repo root.
2. Confirm `Cargo.toml` still declares `AGPL-3.0-only`.
3. Use the About description above when creating the GitHub repository.
4. Add the suggested topic list.
5. Publish `v0.1.0` using `RELEASE_NOTES_v0.1.0.md`.
