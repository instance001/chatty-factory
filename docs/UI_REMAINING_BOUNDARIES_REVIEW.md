# UI Remaining Boundaries Review

This review answers one question:

When should we keep shrinking `crates/chatty_factory_ui/src/main.rs`, and when should we stop?

## Current State

The recent extraction wave already moved several honest subsystems out of `main.rs`:

- governance presentation helpers
- catalog governance panels
- governed extension detail
- extension workbench detail
- proof run controls
- proof history and proof export rendering
- request and action controls

That means `main.rs` is no longer a pure rendering dump. It is increasingly acting as a composition root.

## Remaining High-Signal Surfaces In `main.rs`

The remaining large surfaces are:

1. top bar and toast overlay
2. project browser side panel
3. project details panel
4. runtime status plus extension registry dashboard
5. last-result panel
6. command output panel

Not all of those deserve extraction.

## Healthy Next Boundaries

These are still good candidates for extraction because they represent real subsystem boundaries.

### 1. Project Browser Panel

Why it is healthy:

- it is a full left-side navigation surface
- it owns project-list interaction behavior
- it has its own refresh/select/patch-this flow
- it is cohesive and likely to grow independently

Recommended module shape:

- `project_browser_panel.rs`

### 2. Runtime And Registry Dashboard

Why it is healthy:

- the right-column `Runtime Status` + `Extension Registry` area is a real operational dashboard
- it already contains its own counts, refresh actions, policy toggles, freshness summaries, and registry-filter controls
- it is the largest remaining structured subsystem in `main.rs`

Recommended module shape:

- `runtime_registry_dashboard.rs`

Important note:

This is probably the strongest next extraction candidate.

### 3. Last Result / Command Output Surface

Why it is healthy:

- it is a self-contained execution feedback surface
- it groups result summary, fallback visibility, and command log display concerns
- it is likely to keep evolving with task execution behavior

Recommended module shape:

- `execution_feedback_panel.rs`

This is a healthy candidate, but slightly less urgent than the runtime/registry dashboard.

## Borderline Boundaries

These are possible extraction candidates, but only if they grow more.

### Top Bar And Toast Overlay

Why it is borderline:

- it is cohesive
- but still small and highly tied to global app state

Recommendation:

- keep in `main.rs` for now

### Project Details Panel

Why it is borderline:

- it is cohesive
- but still relatively straightforward compared with the registry/runtime dashboard
- it may be worth keeping near the composition root until it grows more behavior

Recommendation:

- defer for now unless it starts adding more project-side actions or previews

## Unhealthy Next Boundaries

These would likely make the UI harder to navigate rather than easier.

Do not extract these just to reduce line count:

- tiny action-row helpers
- one-off widget fragments
- single label groups
- local row-formatting helpers with no independent behavior
- micro-panels that exist only once and have weak identity

In particular, these would be a bad next wave:

- extracting isolated governance refresh rows
- extracting individual project-list row renderers by themselves
- extracting small command-log fragments without the surrounding execution surface

## Recommendation

The next extraction wave should only continue if we take one of these:

1. `runtime_registry_dashboard.rs`
2. `project_browser_panel.rs`
3. `execution_feedback_panel.rs`

If we are not ready to take one of those whole surfaces, we should stop shrinking.

## Preferred Order

Recommended order for any further UI extraction:

1. `runtime_registry_dashboard.rs`
2. `execution_feedback_panel.rs`
3. `project_browser_panel.rs`

Why this order:

- the runtime/registry dashboard is the heaviest remaining subsystem
- execution feedback is the next clearest standalone surface
- the project browser is healthy, but less urgent than the large central operational surfaces

## Bottom Line

We are at a good stopping point now.

Further shrinking is still healthy only if it follows one of the remaining true subsystem boundaries above.

If the next step is not one of those, more extraction would likely become cosmetic and detrimental.
