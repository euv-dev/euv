use super::*;

/// Interpolates `{name}`-style placeholders in `template` from the
/// supplied `vars` map.
///
/// Walks the template once and replaces every `{key}` segment whose
/// key is present in `vars`. Placeholders whose key is missing are
/// left as the literal `{key}` token — matching the i18next
/// default behaviour. No escaping is performed: callers that need
/// it must escape themselves.
///
/// Returns the interpolated `String`. Empty / no-placeholder
/// templates round-trip unchanged.
///
/// # Arguments
///
/// - `&str` - The template containing `{key}` placeholders.
/// - `&HashMap<&'static str, &'static str>` - The variable map. A
///   static-borrowed key list is enough because the typical
///   consumer (`I18n::t_with`) builds the map inline.
///
/// # Returns
///
/// - `String` - The interpolated string.
pub(crate) fn interpolate(template: &str, vars: &HashMap<&'static str, &'static str>) -> String {
    let mut result: String = String::new();
    let bytes: &[u8] = template.as_bytes();
    let mut cursor: usize = 0_usize;
    while cursor < bytes.len() {
        if bytes[cursor] == b'{'
            && cursor + 1_usize < bytes.len()
            && bytes[cursor + 1_usize] != b'{'
        {
            // Look for the matching `}`.
            if let Some(close_rel) = template[cursor + 1_usize..].find('}') {
                let close: usize = cursor + 1_usize + close_rel;
                let key: &str = &template[cursor + 1_usize..close];
                if let Some(value) = vars.get(key) {
                    result.push_str(value);
                } else {
                    // Preserve the original `{key}` placeholder.
                    result.push('{');
                    result.push_str(key);
                    result.push('}');
                }
                cursor = close + 1_usize;
                continue;
            }
        }
        // Append the current char and move on. Works for ASCII;
        // multi-byte UTF-8 falls through as-is because `result.push`
        // re-encodes the byte at the cursor position.
        if let Some(ch) = template[cursor..].chars().next() {
            result.push(ch);
            cursor += ch.len_utf8();
        } else {
            break;
        }
    }
    result
}

/// Obtains the i18n handle registered against the current hook context slot.
///
/// Behaves like `HookContext::use_hook`: the same `I18n` is returned
/// on every render at the same hook index, preserving the locale,
/// fallback locale, and message table across renders.
///
/// The initial `locale` defaults to `"en"`; pass `init_locale` to
/// change it on the first render. Call [`I18n::add_messages`] to
/// register translations under each locale tag.
///
/// # Arguments
///
/// - `&str` - The initial locale tag. Defaults to `"en"` if the
///   empty string is supplied (so a wrapper can pass `""` to adopt
///   the default).
///
/// # Returns
///
/// - `I18n` - The i18n handle.
///   Returns the factory result directly when no hook context is
///   active (e.g. when called outside a render cycle).
pub fn use_i18n(init_locale: &str) -> I18n {
    let locale: &str = if init_locale.is_empty() {
        "en"
    } else {
        init_locale
    };
    HookContext::use_hook(move || {
        I18n::new(
            Signal::create(locale.to_string()),
            Signal::create(String::from("en")),
        )
    })
}

/// Registers a translation `key -> message` under `locale` on the
/// supplied i18n handle.
///
/// Pair with [`use_i18n`]: the typical pattern is to call this in
/// an `App::use_cleanup`-style mount phase so the same translations
/// stay registered across renders.
///
/// The `&'static` lifetime bound matches the internal
/// `MessageEntry` type alias (`(&'static str, &'static str)`);
/// in practice the call site passes an array literal that the
/// compiler naturally satisfies from string-literal promotion.
///
/// # Arguments
///
/// - `I18n` - The i18n handle obtained from `use_i18n()`.
/// - `&str` - The locale tag (e.g. `"en"`, `"zh-CN"`).
/// - `&[(&'static str, &'static str)]` - The `(key, message)` pairs.
///   Internally forwarded as `&[MessageEntry]`.
///
/// # Panics
///
/// This function does not panic.
pub fn i18n_register(handle: I18n, locale: &str, entries: &[(&'static str, &'static str)]) {
    handle.add_messages(locale, entries);
}

/// Clears every registered translation from the process-wide
/// [`I18N_MESSAGES`] storage.
///
/// Intended for integration tests in `ui/tests/i18n/` that
/// rely on a clean table at the start of each case. Production
/// callers must never use this — translation tables are
/// expected to live for the lifetime of the app.
pub fn i18n_reset_for_tests() {
    if let Some(lock) = I18N_MESSAGES.get() {
        let mut guard: std::sync::RwLockWriteGuard<
            'static,
            HashMap<String, HashMap<String, String>>,
        > = lock.write().unwrap_or_else(|e| e.into_inner());
        guard.clear();
    }
}
/// Returns the process-wide messages lock, initialising it
/// on first call.
///
/// All i18n reads and writes route through this helper so
/// the lazy-init logic stays in one place.
pub(crate) fn messages_lock() -> &'static RwLock<HashMap<String, HashMap<String, String>>> {
    I18N_MESSAGES.get_or_init(|| RwLock::new(HashMap::new()))
}
