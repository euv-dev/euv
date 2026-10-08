use super::*;

fn locale_of(init: &str) -> I18n {
    use_i18n(init)
}

#[test]
fn an_i18n_handle_starts_on_the_locale_it_was_given() {
    let handle: I18n = locale_of("fr");
    assert_eq!(
        handle.get_locale().get(),
        "fr",
        "the initial locale is what the caller asked for"
    );
}

#[test]
fn an_empty_locale_falls_back_to_english() {
    let handle: I18n = locale_of("");
    assert_eq!(
        handle.get_locale().get(),
        "en",
        "an empty string is a configuration mistake, not a locale, so it must \
         resolve to the documented default rather than a blank key"
    );
}

#[test]
fn a_locale_can_be_switched_after_the_fact() {
    let handle: I18n = locale_of("en");
    handle.change_locale("ja");
    assert_eq!(handle.get_locale().get(), "ja");
}

#[test]
fn a_fallback_locale_can_be_switched_too() {
    let handle: I18n = locale_of("fr");
    handle.change_fallback_locale("pt");
    assert_eq!(handle.get_fallback_locale().get(), "pt");
}

#[test]
fn registering_a_translation_makes_it_readable() {
    let handle: I18n = locale_of("de");
    i18n_register(
        handle,
        "de",
        &[("k.de.greet", "Hallo"), ("k.de.bye", "Tschuess")],
    );
    assert_eq!(handle.t("k.de.greet"), "Hallo");
    assert_eq!(handle.t("k.de.bye"), "Tschuess");
}

#[test]
fn two_locales_may_translate_differently_and_switching_finds_each() {
    let handle: I18n = locale_of("en");
    i18n_register(handle, "en", &[("k.two.greet", "Hello")]);
    i18n_register(handle, "es", &[("k.two.greet", "Hola")]);
    handle.change_locale("en");
    assert_eq!(handle.t("k.two.greet"), "Hello");
    handle.change_locale("es");
    assert_eq!(handle.t("k.two.greet"), "Hola");
    handle.change_locale("en");
    assert_eq!(
        handle.t("k.two.greet"),
        "Hello",
        "switching back must find the first table intact"
    );
}

#[test]
fn an_untranslated_key_is_reported_rather_than_invented() {
    let handle: I18n = locale_of("qq");
    let observed: String = handle.t("k.never.registered");
    assert!(
        observed.contains("k.never.registered"),
        "a missing translation must surface the key so the gap is visible, got {observed}"
    );
}

#[test]
fn a_fallback_locale_covers_a_gap_in_the_active_one() {
    let handle: I18n = locale_of("qq");
    handle.change_fallback_locale("en");
    i18n_register(handle, "en", &[("k.fb.greet", "Hello")]);
    assert_eq!(
        handle.t("k.fb.greet"),
        "Hello",
        "an unregistered active locale must fall through to the fallback"
    );
}

#[test]
fn a_locale_can_be_removed_wholesale() {
    let handle: I18n = locale_of("es");
    i18n_register(handle, "es", &[("k.rm.greet", "Hola")]);
    assert_eq!(handle.t("k.rm.greet"), "Hola");
    handle.remove_locale("es");
    assert_ne!(
        handle.t("k.rm.greet"),
        "Hola",
        "a removed locale must stop resolving, not linger in the table"
    );
}

#[test]
fn a_single_message_can_be_removed() {
    let handle: I18n = locale_of("en");
    i18n_register(handle, "en", &[("k.one.a", "A")]);
    assert_eq!(handle.t("k.one.a"), "A");
    handle.remove_message("en", "k.one.a");
    assert_ne!(
        handle.t("k.one.a"),
        "A",
        "removing one key must not take the rest of the locale with it"
    );
}

#[test]
fn a_templated_message_substitutes_its_variables() {
    let handle: I18n = locale_of("en");
    i18n_register(handle, "en", &[("k.tpl.hello", "Hello {name}")]);
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("name", "Ada");
    assert_eq!(
        handle.t_with("k.tpl.hello", &vars),
        "Hello Ada",
        "a placeholder left unsubstituted ships a broken sentence to the user"
    );
}

#[test]
fn a_profiler_starts_with_no_entries() {
    let profiler: ProfilerHandle = use_profiler();
    assert!(
        profiler.get_entries().get().is_empty(),
        "a fresh profiler must record nothing before it is asked to"
    );
}

#[test]
fn a_measured_block_records_its_label_and_returns_the_body_result() {
    let profiler: ProfilerHandle = use_profiler();
    let result: u32 = profiler.measure("work", || 21 * 2);
    assert_eq!(
        result, 42,
        "the measurement must be transparent to the body"
    );
    let entries: Vec<ProfileEntry> = profiler.get_entries().get();
    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries[0].label, "work",
        "the label is how a reader tells entries apart"
    );
}

#[test]
fn a_profiler_accumulates_one_entry_per_measurement_in_order() {
    let profiler: ProfilerHandle = use_profiler();
    profiler.measure("first", || ());
    profiler.measure("second", || ());
    let entries: Vec<ProfileEntry> = profiler.get_entries().get();
    assert_eq!(
        entries.len(),
        2,
        "each measurement appends exactly one entry"
    );
    assert_eq!(entries[0].label, "first");
    assert_eq!(
        entries[1].label, "second",
        "entries must keep the order they were taken in"
    );
}

#[test]
fn a_profiler_can_be_cleared() {
    let profiler: ProfilerHandle = use_profiler();
    profiler.measure("a", || ());
    assert!(!profiler.get_entries().get().is_empty());
    profiler.clear();
    assert!(
        profiler.get_entries().get().is_empty(),
        "a cleared profile must not report stale timings"
    );
}
