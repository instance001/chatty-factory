# Architecture Checkpoint 29

Project patchability interpretation has crossed another useful threshold.

At this checkpoint, the browser and project-detail surface no longer treat every
historical blocker the same way.

The system now distinguishes between:

- `patch-risk`
  - projects with genuinely risky blocked lanes
- `patch-historical`
  - projects blocked only by historical lanes with named modern replacements
- `patch-decomposable`
  - projects whose historical blockers are not only replaced in theory, but are
    already decomposable into a modern replacement bundle right now

What changed:

- project patchability receipts now persist:
  - `decomposable_historical_blocker_bundles`
  - `decomposable_historical_blocker_count`
- the project browser now has:
  - `Show decomposable historical only`
  - a summary count for decomposable historical projects
  - a `patch-decomposable` badge
  - a `[decomposable-historical]` row badge
- project sorting now places decomposable historical projects:
  - below true patch risk
  - above plain historical blockers

Why this matters:

- the browser is no longer only a risk dashboard
- it is becoming a modernization dashboard too
- operators can now separate:
  - projects that are unsafe to patch
  - from projects whose legacy patch posture is already understandable and
    auto-modernizable

Concrete reading:

- `build_me_a_sigma_helper_backed`
  - still has historical blockers
  - but those blockers are fully decomposable into modern replacement bundles
  - so it now belongs in the `patch-decomposable` class rather than generic
    `patch-historical`

This is an important step in the factory’s maturity:

- patchability governance is no longer just answering
  - `is surgery risky?`
- it is also answering
  - `is this legacy posture already ready to be rewritten into the modern
    contract model?`
