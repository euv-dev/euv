use super::*;

/// Parsed input for `euv_main!(builder_fn)`.
pub(crate) struct EntryInput {
    /// Function identifier producing a configured `AppConfig` and mounting the root view.
    pub(crate) bootstrap: Ident,
}
