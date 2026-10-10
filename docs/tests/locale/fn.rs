use super::*;

#[test]
fn the_gate_translates_every_key_in_every_locale() {
    for tag in supported_locale_tags() {
        let table: &[(&str, &str)] = table_for(tag);
        assert_eq!(
            table.len(),
            gate_translation_keys().len(),
            "{tag} must carry every gate string"
        );
        for key in gate_translation_keys() {
            let message: Option<String> = lookup_table(tag, key);
            assert!(message.is_some(), "{tag} has no entry for {key}");
            assert!(
                !message.unwrap_or_default().is_empty(),
                "{tag} has an empty entry for {key}"
            );
        }
    }
}

#[test]
fn the_four_site_locales_are_covered() {
    assert_eq!(supported_locale_tags().len(), 4);
}

#[test]
fn every_table_declares_the_same_keys_in_the_same_order() {
    let reference: Vec<&str> = table_for(gate_fallback_tag())
        .iter()
        .map(|(key, _): &(&str, &str)| *key)
        .collect();
    for tag in supported_locale_tags() {
        let listed: Vec<&str> = table_for(tag)
            .iter()
            .map(|(key, _): &(&str, &str)| *key)
            .collect();
        assert_eq!(listed, reference, "{tag} declares a different key set");
    }
}

#[test]
fn every_locale_distinguishes_itself_from_the_fallback() {
    let fallback: Option<String> = lookup_table(gate_fallback_tag(), gate_key_hint());
    for tag in supported_locale_tags() {
        if *tag == gate_fallback_tag() {
            continue;
        }
        assert_ne!(
            lookup_table(tag, gate_key_hint()),
            fallback,
            "{tag} renders the fallback hint, so its locale is not wired"
        );
    }
}

#[test]
fn an_unknown_tag_has_no_table() {
    assert!(table_for("fr-FR").is_empty());
    assert_eq!(lookup_table("fr-FR", gate_key_hint()), None);
}

#[test]
fn an_unknown_key_falls_through_to_the_key_itself() {
    assert_eq!(
        lookup_table(gate_fallback_tag(), "docs.password_gate.nope"),
        None
    );
}

#[test]
fn the_chinese_table_keeps_the_pre_localisation_wording() {
    let hint: Option<String> = lookup_table(supported_locale_tags()[0], gate_key_hint());
    assert_eq!(
        hint,
        Some("本文受密码保护，输入密码后即可查看内容。".to_string())
    );
    let keys: &'static [&'static str] = gate_translation_keys();
    let idle: Option<String> = lookup_table(supported_locale_tags()[0], keys[1]);
    assert_eq!(idle, Some("解锁".to_string()));
    let error: Option<String> = lookup_table(supported_locale_tags()[0], keys[3]);
    assert_eq!(error, Some("密码错误，请重试。".to_string()));
}

#[test]
fn every_locale_tag_is_distinct() {
    let mut seen: HashSet<&str> = HashSet::new();
    for tag in supported_locale_tags() {
        assert!(seen.insert(*tag), "{tag} is registered twice");
    }
    assert_eq!(seen.len(), supported_locale_tags().len());
}

#[test]
fn the_label_table_covers_every_supported_tag() {
    let tagged: Vec<&str> = locale_tag_bindings()
        .iter()
        .map(|(_, tag): &(&str, &str)| *tag)
        .collect();
    for tag in supported_locale_tags() {
        assert!(tagged.contains(tag), "{tag} has no label binding");
    }
}

#[test]
fn every_label_binding_resolves_to_a_table() {
    for (label, tag) in locale_tag_bindings() {
        assert!(!label.is_empty(), "a locale label must not be empty");
        assert!(
            !table_for(tag).is_empty(),
            "{label} resolves to {tag}, which has no table"
        );
    }
}
