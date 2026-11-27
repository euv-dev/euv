/// Schema anchor for built-in `view!` element `slot` — Surgically updatable component-level dynamic OTA/WASM slot.
pub const fn element() {}

/// Sets the unique slot identifier string.
pub const fn id() {}

/// Overrides the target `DynamicUpdateHub` instance.
pub const fn hub() {}

/// Sets an explicit fallback builder closure `Fn() -> NodeId + 'static`.
pub const fn fallback() {}
