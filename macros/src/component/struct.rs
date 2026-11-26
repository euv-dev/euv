use super::*;

/// Parsed arguments passed to `#[component(...)]` (e.g. `#[component(retention = Freeze)]`).
pub(crate) struct ComponentAttrArgs {
    /// Optional `PageRetention` variant identifier (`Destroy`, `Freeze`, or `Alive`).
    pub(crate) retention: Option<Ident>,
}
