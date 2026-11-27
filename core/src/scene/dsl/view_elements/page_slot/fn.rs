/// Schema anchor for built-in `view!` element `page_slot` — Surgically updatable page-level dynamic OTA/WASM route slot.
pub const fn element() {}

/// Sets the target route path string for dynamic page replacement.
pub const fn route() {}

/// Overrides the target `DynamicUpdateHub` instance.
pub const fn hub() {}

/// Sets an explicit fallback builder closure `Fn() -> NodeId + 'static`.
pub const fn fallback() {}
