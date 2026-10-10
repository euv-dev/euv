use super::*;

/// Returns the BCP-47 tag for the locale that owns `route`.
///
/// The tag is matched against the locale's own display label, which
/// `build.rs` already emits from the content repository's
/// `[[locales]]` frontmatter. That is deliberate: the crate has no
/// second copy of which prefix means which language, and inventing one
/// here is how a route ends up rendering Korean text in the Japanese
/// shell. A locale whose label is not one of the four known ones falls
/// back to [`LOCALE_TAG_DEFAULT`], because the English table is the
/// complete one and a missing tag must never leave the gate blank.
///
/// # Arguments
///
/// - `&str` - The page route path.
///
/// # Returns
///
/// - `&'static str` - The locale tag for the route's locale.
pub fn locale_tag_for(route: &str) -> &'static str {
    let label: &str = locale_of(route).label;
    for (entry_label, entry_tag) in LOCALE_TAGS {
        if entry_label == label {
            return entry_tag;
        }
    }
    LOCALE_TAG_DEFAULT
}

/// Translates `key` in the locale that owns `route`.
///
/// Resolution walks the active locale's table and then the English one,
/// so a key missing from a locale degrades to readable English rather
/// than to the raw key or an empty string — the gate never shows a blank
/// label. A key present in neither table returns `key` itself, which
/// makes an unregistered key visible in the UI instead of silently
/// invisible.
///
/// # Arguments
///
/// - `&str` - The page route path, whose locale selects the table.
/// - `&str` - The translation key.
///
/// # Returns
///
/// - `String` - The translated message, or `key` when no table has it.
pub fn locale_text(route: &str, key: &str) -> String {
    let tag: &str = locale_tag_for(route);
    lookup_table(tag, key)
        .or_else(|| lookup_table(LOCALE_TAG_DEFAULT, key))
        .unwrap_or_else(|| key.to_string())
}

/// Returns the message registered for `key` under `tag`, if any.
///
/// # Arguments
///
/// - `&str` - The locale tag to read.
/// - `&str` - The translation key to read.
///
/// # Returns
///
/// - `Option<String>` - The message, or `None` when the tag or the key
///   is not registered.
pub fn lookup_table(tag: &str, key: &str) -> Option<String> {
    table_for(tag)
        .iter()
        .find(|(entry_key, _): &&(&str, &str)| *entry_key == key)
        .map(|(_, message): &(&str, &str)| (*message).to_string())
}

/// Returns the translation table registered under `tag`.
///
/// # Arguments
///
/// - `&str` - The locale tag to read.
///
/// # Returns
///
/// - `&'static [(&'static str, &'static str)]` - The table, empty when
///   the tag is not one of the four supported locales.
pub fn table_for(tag: &str) -> &'static [(&'static str, &'static str)] {
    if tag == PW_GATE_LOCALE_TAG_ZH {
        return &PW_GATE_MESSAGES_ZH;
    }
    if tag == PW_GATE_LOCALE_TAG_EN {
        return &PW_GATE_MESSAGES_EN;
    }
    if tag == PW_GATE_LOCALE_TAG_JA {
        return &PW_GATE_MESSAGES_JA;
    }
    if tag == PW_GATE_LOCALE_TAG_KO {
        return &PW_GATE_MESSAGES_KO;
    }
    &[]
}

/// Returns every locale tag the gate carries a table for.
///
/// # Returns
///
/// - `&'static [&'static str]` - The supported locale tags, in the
///   order the tables are declared.
pub fn supported_locale_tags() -> &'static [&'static str] {
    &[
        PW_GATE_LOCALE_TAG_ZH,
        PW_GATE_LOCALE_TAG_EN,
        PW_GATE_LOCALE_TAG_JA,
        PW_GATE_LOCALE_TAG_KO,
    ]
}

/// Returns every translation key the password gate registers.
///
/// Exposed as a function rather than as a `pub const` because the key
/// list is exactly the thing a completeness test needs to enumerate,
/// and a function is the visibility the project allows an external
/// reader to reach for.
///
/// # Returns
///
/// - `&'static [&'static str]` - Every key, in table order.
pub fn gate_translation_keys() -> &'static [&'static str] {
    &[
        PW_GATE_KEY_HINT,
        PW_GATE_KEY_IDLE,
        PW_GATE_KEY_BUSY,
        PW_GATE_KEY_ERROR,
        PW_GATE_KEY_LABEL,
        PW_GATE_KEY_PLACEHOLDER,
    ]
}

/// Returns the translation key for the locked-page explanation.
///
/// # Returns
///
/// - `&'static str` - The key the hint is registered under.
pub fn gate_key_hint() -> &'static str {
    PW_GATE_KEY_HINT
}

/// Returns the tag the fallback translation table is registered under.
///
/// The fallback is English: it is the only table guaranteed to carry
/// every key, so it is what a partially translated locale degrades to.
///
/// # Returns
///
/// - `&'static str` - The fallback locale tag.
pub fn gate_fallback_tag() -> &'static str {
    LOCALE_TAG_DEFAULT
}

/// Returns every locale tag bound to a content-repository label.
///
/// # Returns
///
/// - `&'static [(&'static str, &'static str)]` - `(label, tag)` pairs.
pub fn locale_tag_bindings() -> &'static [(&'static str, &'static str)] {
    &LOCALE_TAGS
}
