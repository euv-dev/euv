use super::*;

/// Default locale tag for the i18n demo.
pub(crate) const HOOKS_I18N_DEFAULT_LOCALE: &str = "en";

/// Secondary locale tag the demo can switch into.
pub(crate) const HOOKS_I18N_OTHER_LOCALE: &str = "zh-CN";

/// English-language button label.
pub(crate) const HOOKS_I18N_LABEL_EN: &str = "English";

/// "Other" (Chinese) language button label.
pub(crate) const HOOKS_I18N_LABEL_OTHER: &str = "中文";

/// Translation key for the greeting message.
pub(crate) const HOOKS_I18N_KEY_GREETING: &str = "greeting";

/// Translation key for the farewell message.
pub(crate) const HOOKS_I18N_KEY_FAREWELL: &str = "farewell";

/// English (`en`) translation table — `&'static` tuples are the
/// only form `i18n_register` can accept without leaking. The
/// underlying `I18n::add_messages` does the
/// `&str -> String` round-trip on the caller's behalf.
pub(crate) const HOOKS_I18N_EN_MESSAGES: [(&str, &str); 2] = [
    (HOOKS_I18N_KEY_GREETING, "Hello, world!"),
    (HOOKS_I18N_KEY_FAREWELL, "Goodbye, world!"),
];

/// `zh-CN` translation table.
pub(crate) const HOOKS_I18N_ZH_MESSAGES: [(&str, &str); 2] = [
    (HOOKS_I18N_KEY_GREETING, "你好,世界!"),
    (HOOKS_I18N_KEY_FAREWELL, "再见,世界!"),
];

/// Builds a click handler that switches the supplied i18n
/// handle to the supplied locale.
///
/// # Arguments
///
/// - `I18n` - The `handle` argument.
/// - `String` - The `locale` argument.
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The value this function returns.
///
pub(crate) fn hooks_i18n_switch(handle: I18n, locale: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        handle.change_locale(locale.as_str());
    }))
}

/// Returns the translated message for `key` on the supplied
/// handle, falling back to the key itself if missing.
///
/// # Arguments
///
/// - `I18n` - The `handle` argument.
/// - `&'static str` - The `key` argument.
/// # Returns
///
/// - `String` - The value this function returns.
///
pub(crate) fn hooks_i18n_translate(handle: I18n, key: &'static str) -> String {
    handle.t(key)
}
