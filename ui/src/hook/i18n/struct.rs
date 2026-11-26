use super::*;

/// Reactive multi-locale translation catalog with zero global mutable state.
#[derive(Clone, Data, Debug)]
pub struct I18nCatalog {
    /// Active locale identifier signal (e.g. `"en"`, `"zh"`).
    #[get(type(copy))]
    pub(crate) locale: Signal<String>,
    /// Fallback locale used when a key is absent from the active locale.
    pub(crate) fallback_locale: String,
    /// Locale-to-key-value translation tables.
    pub(crate) tables: HashMap<String, HashMap<String, String>>,
}
