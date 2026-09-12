use super::*;

/// The aggregate i18n state.
///
/// Constructed once per app via `App::use_i18n()`, threaded
/// through any code that needs to render translated text.
/// Cheap to `Clone` (the internal signal is
/// `Copy`-by-pointer).
///
/// # Storage
///
/// Messages are stored in a process-wide
/// [`I18N_MESSAGES`] `OnceLock<RwLock<HashMap<...>>>` rather
/// than a per-handle `Signal`. The translation table is
/// not reactive on its own — only the `locale` field is —
/// so wrapping the table in a signal forced every `t()`
/// call to clone the entire `HashMap<String, HashMap<String,
/// String>>`. Moving the table behind a `OnceLock` means:
///
/// - one allocation for the whole process (no per-handle
///   `Signal::create(HashMap::new())`),
/// - reads (`t`, `locale_count`, `active_message_count`)
///   borrow through the read guard with zero clone,
/// - writes (`add_messages`, `remove_locale`,
///   `remove_message`) take the write guard once.
///
/// Process-wide translation table storage.
///
/// Backed by [`std::sync::OnceLock`] so the table is
/// allocated lazily on first write/read and never torn
/// down. Wrapped in a [`std::sync::RwLock`] because runtime
/// mutation is supported — `add_messages`,
/// `remove_locale`, and `remove_message` all write.
///
/// On WASM this is single-threaded so the lock is
/// uncontended; on the native test target it serialises
/// the rare concurrent test against itself without
/// affecting functional correctness.
///
/// OPT 22 (tail): replaces the previous
/// `Signal<HashMap<...>>` field on `I18n`. Every `t()` call
/// used to clone the entire translation table; now the
/// table lives behind this lock and `t()` borrows through
/// the read guard.
pub(crate) static I18N_MESSAGES: OnceLock<RwLock<HashMap<String, HashMap<String, String>>>> =
    OnceLock::new();

#[derive(Clone, Data, New)]
pub struct I18n {
    /// The currently-active locale tag. Setting this
    /// via `set_locale` triggers a reactive update that
    /// re-evaluates any reactive `t(...)` read.
    pub(crate) locale: Signal<String>,
    /// The locale to fall back to when a key is missing
    /// in the active locale. Defaults to `"en"`.
    pub(crate) fallback_locale: Signal<String>,
}
