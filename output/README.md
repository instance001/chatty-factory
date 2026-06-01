# Output

This folder is reserved for deterministic rebuild outputs.

As family lanes become real, generated projects should land here instead of being mixed into source, docs, or runtime state.

Policy:
- durable project files live here
- deterministic fixtures may live here when they are part of acceptance
- factory receipts do not live here; they live in `runtime/`
- disposable build byproducts like Rust `target/` are tolerated local clutter, not canonical output
