use super::*;

/// Builds a click handler that switches the supplied i18n
/// handle to the supplied locale.
///
/// # Arguments
///
/// - `I18n` - The i18n handle whose locale is switched on click.
/// - `String` - The locale tag to switch to.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that switches the locale.
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
/// - `I18n` - The i18n handle to translate with.
/// - `&'static str` - The translation key to look up.
///
/// # Returns
///
/// - `String` - The translated message, or the key itself when missing.
pub(crate) fn hooks_i18n_translate(handle: I18n, key: &'static str) -> String {
    handle.t(key)
}
