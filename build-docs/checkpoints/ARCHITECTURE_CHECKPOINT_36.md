# Architecture Checkpoint 36

The negative constraint shelf is now operationally mature enough that it should
stop being the primary focus for a while.

At this checkpoint, the shelf already supports:

- proposal intake from failures
- approval into the live shelf
- consultation during later verification
- activation and deactivation
- bulk low-value deactivation
- archive and restore
- history-preserving retirement
- value-aware active and archived filters
- recent mutation summaries

That means the shelf is no longer the main missing subsystem.

What this checkpoint is really saying:

- the reliability rule container is in strong enough shape
- the higher-value next work is feeding it better evidence

Why this matters:

- if we keep polishing the shelf surface indefinitely, we risk making the rule
  cabinet nicer without improving the factory’s actual learning rate
- the bigger leverage now is:
  - better build verification
  - sharper failure taxonomy
  - better proposed-constraint generation
  - more real failure-to-rule loops

Architectural read:

- the shelf is now a usable governed destination
- the next growth should come from upstream:
  - what failures we classify
  - what proposals we derive
  - what lessons become worth approving

So the clean next pivot is:

- stop centering shelf management
- push back into build verification and proposed-rule generation
- improve the quality of what enters the shelf instead of mostly improving how
  the shelf is displayed
