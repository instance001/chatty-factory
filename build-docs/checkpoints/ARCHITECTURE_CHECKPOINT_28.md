# Architecture Checkpoint 28

The patch self-review layer can now redirect historical intent into a modern
replacement bundle, not only a single modern lane.

What changed at this checkpoint:

- self-review can recognize when one historical lane maps to multiple declared
  modern replacements
- if those replacements are structurally ready or already present, self-review
  can compose an executable bundle of the still-needed modern lanes
- the host can hand that bundle into bounded composition instead of trying to
  force the original legacy lane through preflight
- the review receipt now records:
  - the full recommended replacement bundle
  - the executable subset that still needed surgery
  - whether that bundle was already present or ready for composition

Why this matters:

- the factory no longer has to choose between:
  - blindly forcing a stale historical lane
  - or simply blocking forever
- it can now revise the plan into the modern capability shape and keep going

Concrete proof:

- request:
  - `patch build_me_a_sigma_helper_backed add helper summary badges`
- old historical intent:
  - `helper_summary_badges`
- modern replacement bundle:
  - `helper_summary_count_delta`
  - `helper_summary_lane_count_chip`
  - `helper_summary_types_chip`
- self-review selected the still-needed executable subset:
  - `helper_summary_count_delta`
- bounded composition then executed that modern lane successfully

This is the first clear step from:

- self-review can patch its own patch plan

to:

- self-review can decompose a stale capability request into the modern bounded
  capability bundle that actually matches the project
