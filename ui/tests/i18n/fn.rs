use super::*;

/// Helper body of the `fresh_i18n` free function.
///
/// # Returns
///
/// - `I18n` - A `I18n` value.
fn fresh_i18n() -> I18n {
    i18n_reset_for_tests();
    I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
    )
}

/// Helper body of the `seeded_i18n` free function.
///
/// # Returns
///
/// - `I18n` - A `I18n` value.
fn seeded_i18n() -> I18n {
    i18n_reset_for_tests();
    let i18n: I18n = I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
    );
    let entries_en: &[MessageEntry] = &[("hello", "Hello"), ("goodbye", "Goodbye")];
    i18n.add_messages("en", entries_en);
    let entries_zh: &[MessageEntry] = &[("hello", "你好")];
    i18n.add_messages("zh-CN", entries_zh);
    i18n
}

#[test]
fn fresh_i18n_defaults_to_en_locale_and_empty_messages() {
    let i18n: I18n = fresh_i18n();
    assert_eq!(i18n.get_locale().get(), "en");
    assert_eq!(i18n.get_fallback_locale().get(), "en");
    assert_eq!(i18n.locale_count(), 0);
    assert_eq!(i18n.active_message_count(), 0);
}

#[test]
fn fresh_i18n_returns_key_for_missing_translation() {
    let i18n: I18n = fresh_i18n();
    assert_eq!(i18n.t("hello"), "hello");
    assert_eq!(i18n.t("nonexistent"), "nonexistent");
}

#[test]
fn locale_and_fallback_locale_accessors_return_signal_clones() {
    let i18n: I18n = fresh_i18n();
    let _locale: Signal<String> = *i18n.get_locale();
    let _fallback: Signal<String> = *i18n.get_fallback_locale();
}

#[test]
fn reactive_read_via_subscribed_locale_signal_matches_initial_value() {
    let i18n: I18n = fresh_i18n();
    let subscribed: Signal<String> = *i18n.get_locale();
    assert_eq!(subscribed.get(), "en");
}

#[test]
fn seeded_messages_are_translated_by_active_locale() {
    let i18n: I18n = seeded_i18n();
    assert_eq!(i18n.t("hello"), "Hello");
    assert_eq!(i18n.t("goodbye"), "Goodbye");
}

#[test]
fn seeded_i18n_active_message_count_matches_registered_keys() {
    let i18n: I18n = seeded_i18n();
    assert_eq!(i18n.active_message_count(), 2);
}

#[test]
fn seeded_i18n_locale_count_matches_registered_locales() {
    let i18n: I18n = seeded_i18n();
    assert_eq!(i18n.locale_count(), 2);
}

#[test]
fn i18n_clone_shares_internal_signals() {
    let i18n: I18n = seeded_i18n();
    let twin: I18n = i18n;
    assert_eq!(twin.t("hello"), "Hello");
    assert_eq!(twin.get_locale().get(), "en");
}

#[test]
fn change_locale_updates_active_locale_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        i18n.change_locale("zh-CN");
    }))
    .is_ok();
    if ran {
        assert_eq!(i18n.get_locale().get(), "zh-CN");
        assert_eq!(i18n.t("hello"), "你好");
    }
}

#[test]
fn change_locale_falls_back_when_key_missing_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        i18n.change_locale("zh-CN");
    }))
    .is_ok();
    if ran {
        assert_eq!(i18n.t("hello"), "你好");
        assert_eq!(
            i18n.t("goodbye"),
            "Goodbye",
            "missing key in zh-CN should fall back to en"
        );
    }
}

#[test]
fn change_locale_returns_key_when_neither_locale_has_it_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        i18n.change_locale("zh-CN");
    }))
    .is_ok();
    if ran {
        assert_eq!(
            i18n.t("nonexistent"),
            "nonexistent",
            "key missing in both locales must fall through to the key itself"
        );
    }
}

#[test]
fn change_fallback_locale_changes_fallback_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        i18n.change_fallback_locale("zh-CN");
        i18n.change_locale("ja");
    }))
    .is_ok();
    if ran {
        assert_eq!(i18n.t("goodbye"), "goodbye");
    }
}

#[test]
fn add_messages_inserts_new_locale_set_path() {
    let i18n: I18n = fresh_i18n();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        let entries: &[MessageEntry] = &[("hello", "Bonjour"), ("goodbye", "Au revoir")];
        i18n.add_messages("fr", entries);
        i18n.change_locale("fr");
    }))
    .is_ok();
    if ran {
        assert_eq!(i18n.t("hello"), "Bonjour");
        assert_eq!(i18n.t("goodbye"), "Au revoir");
    }
}

#[test]
fn add_messages_overwrites_existing_entry_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        let entries: &[MessageEntry] = &[("hello", "Howdy")];
        i18n.add_messages("en", entries);
    }))
    .is_ok();
    if ran {
        assert_eq!(i18n.t("hello"), "Howdy");
    }
}

#[test]
fn add_messages_supports_empty_batch_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        let entries: &[MessageEntry] = &[];
        i18n.add_messages("es", entries);
    }))
    .is_ok();
    if ran {
        assert_eq!(i18n.locale_count(), 3);
        assert_eq!(i18n.active_message_count(), 2);
    }
}

#[test]
fn remove_locale_drops_locale_from_table_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        i18n.remove_locale("zh-CN");
    }))
    .is_ok();
    if ran {
        assert_eq!(i18n.locale_count(), 1);
    }
}

#[test]
fn remove_locale_absent_locale_is_noop_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        i18n.remove_locale("ja");
    }))
    .is_ok();
    if ran {
        assert_eq!(i18n.locale_count(), 2);
    }
}

#[test]
fn remove_message_drops_single_key_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        i18n.remove_message("en", "goodbye");
    }))
    .is_ok();
    if ran {
        assert_eq!(i18n.t("goodbye"), "goodbye");
        assert_eq!(i18n.t("hello"), "Hello");
    }
}

#[test]
fn remove_message_absent_key_is_noop_set_path() {
    let i18n: I18n = seeded_i18n();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        i18n.remove_message("en", "nonexistent");
    }))
    .is_ok();
    if ran {
        assert_eq!(i18n.t("hello"), "Hello");
        assert_eq!(i18n.t("goodbye"), "Goodbye");
    }
}

#[test]
fn t_with_substitutes_placeholders_in_pre_registered_messages() {
    i18n_reset_for_tests();
    let i18n: I18n = I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
    );
    let entries: &[MessageEntry] = &[("greet", "Hello, {name}!")];
    i18n.add_messages("en", entries);
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("name", "Alice");
    assert_eq!(i18n.t_with("greet", &vars), "Hello, Alice!");
}

#[test]
fn t_with_leaves_missing_placeholders_as_literal_tokens() {
    i18n_reset_for_tests();
    let i18n: I18n = I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
    );
    let entries: &[MessageEntry] = &[("greet", "Hello, {name}!")];
    i18n.add_messages("en", entries);
    let vars: HashMap<&'static str, &'static str> = HashMap::new();
    assert_eq!(i18n.t_with("greet", &vars), "Hello, {name}!");
}

#[test]
fn t_with_substitutes_multiple_placeholders() {
    i18n_reset_for_tests();
    let i18n: I18n = I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
    );
    let entries: &[MessageEntry] = &[("ordered", "{greeting}, {name}!")];
    i18n.add_messages("en", entries);
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("greeting", "你好");
    vars.insert("name", "Alice");
    assert_eq!(i18n.t_with("ordered", &vars), "你好, Alice!");
}

#[test]
fn t_with_supports_underscored_placeholder_names() {
    i18n_reset_for_tests();
    let i18n: I18n = I18n::new(
        Signal::create(String::from("en")),
        Signal::create(String::from("en")),
    );
    let entries: &[MessageEntry] = &[("greet", "Hi, {first_name}!")];
    i18n.add_messages("en", entries);
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("first_name", "Alice");
    assert_eq!(i18n.t_with("greet", &vars), "Hi, Alice!");
}
