use super::*;

#[test]
fn tabs_keyboard_direction_reverses_only_horizontal_arrows() {
    for length in [3, 4, 5] {
        for current in 0..length {
            for rtl in [false, true] {
                let forward: usize = (current + 1) % length;
                let backward: usize = (current + length - 1) % length;
                assert_eq!(
                    tab_index_for_key(current, "ArrowRight", length, rtl),
                    Some(if rtl { backward } else { forward })
                );
                assert_eq!(
                    tab_index_for_key(current, "ArrowLeft", length, rtl),
                    Some(if rtl { forward } else { backward })
                );
                assert_eq!(tab_index_for_key(current, "Home", length, rtl), Some(0));
                assert_eq!(
                    tab_index_for_key(current, "End", length, rtl),
                    Some(length - 1)
                );
            }
        }
    }
}

#[test]
fn tabs_keyboard_single_empty_and_unsupported_keys_are_safe() {
    for rtl in [false, true] {
        for key in ["ArrowLeft", "ArrowRight", "Home", "End"] {
            assert_eq!(tab_index_for_key(0, key, 0, rtl), None);
            assert_eq!(tab_index_for_key(0, key, 1, rtl), Some(0));
        }
        for key in ["ArrowUp", "ArrowDown", "Enter", " ", "Tab", "Escape"] {
            assert_eq!(tab_index_for_key(1, key, 3, rtl), None);
        }
    }
}

#[test]
fn tabs_keyboard_rtl_four_entry_focus_mapping_matches_logical_order() {
    let mut current: usize = 2;
    for (key, expected) in [
        ("Home", 0),
        ("ArrowLeft", 1),
        ("ArrowRight", 0),
        ("End", 3),
        ("ArrowLeft", 0),
    ] {
        current = tab_index_for_key(current, key, 4, true).expect("navigation index");
        assert_eq!(current, expected);
    }
}

#[test]
fn tabs_keyboard_resolves_live_owning_list_style_before_selection() {
    let source: &str = include_str!("../../src/component/tabs/view/fn.rs");
    let handler: &str = source
        .split("fn select_tab_by_keyboard(")
        .nth(1)
        .expect("handler");
    assert!(handler.contains("target.closest(TAB_SELECTOR)"));
    assert!(handler.contains("current_tab.closest(TAB_LIST_SELECTOR)"));
    assert!(handler.contains("window.get_computed_style(&tab_list)"));
    assert!(handler.contains("style.get_property_value(TAB_DIRECTION_PROPERTY)"));
    assert!(handler.contains("tab_list.query_selector_all(TAB_SELECTOR)"));
    assert!(
        handler.find("get_computed_style").expect("live style")
            < handler.find("tab_index_for_key").expect("mapping")
    );
    assert!(
        handler.find("tab_index_for_key").expect("mapping")
            < handler
                .find("active.set(next_item.key.to_string())")
                .expect("automatic selection")
    );
    assert!(!handler.contains("document_element"));
    assert!(!handler.contains("get_attribute(\"dir\")"));
}
