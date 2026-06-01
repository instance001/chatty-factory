# Architecture Checkpoint 17

This checkpoint closes the next `chatty_factory_ui` extraction wave.

## What Changed

The UI split is no longer centered only on governance presentation.

We now have separate modules for:

- shared governance rendering
- catalog governance panels
- governed extension detail
- extension workbench detail
- proof run controls
- proof history and proof exports
- request and action controls

That means `main.rs` is increasingly acting like a true composition root instead of a mixed rendering file.

## Architectural Read

This is a stronger state than the previous checkpoint:

- governance presentation is modular
- proof orchestration presentation is modular
- request/build/patch controls are modular

The important shift is not line count reduction by itself. It is that the extracted boundaries now map to honest UI subsystems.

## Result

The current `chatty_factory_ui` split is now broad enough that future extractions can be chosen more deliberately:

- registry list rendering
- remaining action rows
- remaining command/status surfaces

rather than continuing to peel features out opportunistically.

## Next Likely Move

The next clean move is to keep `main.rs` shrinking around one of the remaining cross-cutting surfaces:

1. registry-list row rendering
2. command output / task result surface
3. remaining lane action rows outside the extracted panels

At this point, the UI module structure is mature enough that each new extraction can be judged against a clearer subsystem map.
