use super::*;

const GAP_CARET: &str = "var(--gap-caret)";

const CARET_SIZE: &str = "var(--space-lg)";

fn style_value(class: &Css, property: &str) -> Option<String> {
    class
        .get_style()
        .split(';')
        .filter_map(|part: &str| part.split_once(':'))
        .filter(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim().to_string())
        .next_back()
}

fn theme_source() -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/style/var/theme/fn.rs"))
        .expect("the theme token file")
}

fn form_triggers() -> [(&'static Css, &'static Css, &'static Css, &'static Css); 5] {
    [
        (
            c_euv_select_trigger(),
            c_euv_select_value(),
            c_euv_select_placeholder(),
            c_euv_select_caret(),
        ),
        (
            c_euv_date_picker_trigger(),
            c_euv_date_picker_value(),
            c_euv_date_picker_placeholder(),
            c_euv_date_picker_caret(),
        ),
        (
            c_euv_time_picker_trigger(),
            c_euv_time_picker_value(),
            c_euv_time_picker_placeholder(),
            c_euv_time_picker_caret(),
        ),
        (
            c_euv_cascader_trigger(),
            c_euv_cascader_value(),
            c_euv_cascader_placeholder(),
            c_euv_cascader_caret(),
        ),
        (
            c_euv_tree_select_trigger(),
            c_euv_tree_select_value(),
            c_euv_tree_select_placeholder(),
            c_euv_tree_select_caret(),
        ),
    ]
}

fn carets() -> [&'static Css; 6] {
    [
        c_euv_select_caret(),
        c_euv_date_picker_caret(),
        c_euv_time_picker_caret(),
        c_euv_cascader_caret(),
        c_euv_tree_select_caret(),
        c_euv_dropdown_menu_sub_caret(),
    ]
}

#[test]
fn the_caret_gap_is_one_named_token_declared_for_both_theme_modes() {
    let theme: String = theme_source();
    let declared: usize = theme.matches("gap-caret:").count();
    assert_eq!(
        declared, 2,
        "the gap token must be declared once per theme mode, found {declared}"
    );
    assert!(
        theme.contains("gap-caret: \"8px\";"),
        "the caret gap must be a named token on the shared spacing scale"
    );
    assert!(
        !theme.contains("gap-caret: \"0px\";"),
        "a zero gap token would leave the caret touching the label"
    );
}

#[test]
fn every_form_trigger_separates_its_label_from_its_caret_by_the_shared_gap() {
    for (trigger, _, _, _) in form_triggers() {
        assert_eq!(
            style_value(trigger, "gap").as_deref(),
            Some(GAP_CARET),
            "{} must keep the label and the caret apart by the shared gap token",
            trigger.get_name()
        );
    }
}

#[test]
fn every_caret_reserves_a_stable_box_the_label_cannot_squeeze() {
    for caret in carets() {
        assert_eq!(
            style_value(caret, "flex-shrink").as_deref(),
            Some("0"),
            "{} must not shrink into the label",
            caret.get_name()
        );
        assert_eq!(
            style_value(caret, "width").as_deref(),
            Some(CARET_SIZE),
            "{} must reserve a fixed inline size",
            caret.get_name()
        );
        assert_eq!(
            style_value(caret, "height").as_deref(),
            Some(CARET_SIZE),
            "{} must reserve a fixed block size",
            caret.get_name()
        );
    }
}

#[test]
fn every_form_trigger_ellipsizes_its_label_and_keeps_the_caret_visible() {
    for (trigger, value, placeholder, _) in form_triggers() {
        for label in [value, placeholder] {
            assert_eq!(
                style_value(label, "min-width").as_deref(),
                Some("0px"),
                "{} must be allowed to shrink below its content width",
                label.get_name()
            );
            assert_eq!(
                style_value(label, "overflow").as_deref(),
                Some("hidden"),
                "{} must clip the text it cannot show",
                label.get_name()
            );
            assert_eq!(
                style_value(label, "text-overflow").as_deref(),
                Some("ellipsis"),
                "{} must show an ellipsis instead of a clipped glyph",
                label.get_name()
            );
            assert_eq!(
                style_value(label, "white-space").as_deref(),
                Some("nowrap"),
                "{} must stay on one line so the ellipsis applies",
                label.get_name()
            );
            assert_eq!(
                style_value(label, "flex").as_deref(),
                Some("1"),
                "{} must take the room the caret leaves",
                label.get_name()
            );
        }
        assert_eq!(
            style_value(trigger, "width").as_deref(),
            Some("100%"),
            "{} is a fixed-width control, so the label is what has to ellipsize",
            trigger.get_name()
        );
    }
}

#[test]
fn the_dropdown_sub_menu_row_ellipsizes_its_label_before_the_caret() {
    let label: &Css = c_euv_dropdown_menu_item_label();
    assert_eq!(
        style_value(label, "min-width").as_deref(),
        Some("0px"),
        "the sub-menu label must be allowed to shrink below its content width"
    );
    assert_eq!(
        style_value(label, "overflow").as_deref(),
        Some("hidden"),
        "the sub-menu label must clip the text it cannot show"
    );
    assert_eq!(
        style_value(label, "text-overflow").as_deref(),
        Some("ellipsis"),
        "the sub-menu label must show an ellipsis instead of a clipped glyph"
    );
    assert_eq!(
        style_value(label, "white-space").as_deref(),
        Some("nowrap"),
        "the sub-menu label must stay on one line so the ellipsis applies"
    );
    let trigger: &Css = c_euv_dropdown_menu_sub_trigger();
    assert_eq!(
        style_value(trigger, "gap").as_deref(),
        Some(GAP_CARET),
        "the sub-menu row must keep the label and the caret apart by the shared gap token"
    );
    let caret: &Css = c_euv_dropdown_menu_sub_caret();
    assert_eq!(
        style_value(caret, "margin-inline-start").as_deref(),
        Some("auto"),
        "the sub-menu caret is pushed to the trailing edge, so the gap token is what \
         separates it from the label"
    );
}

#[test]
fn the_caret_gap_token_is_read_by_every_caret_bearing_trigger() {
    let mut readers: usize = 0;
    let registry: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/style/class");
    for entry in fs::read_dir(&registry)
        .expect("the class registry")
        .flatten()
    {
        let path: PathBuf = entry.path().join("fn.rs");
        if !path.is_file() {
            continue;
        }
        let source: String = fs::read_to_string(&path).expect("a class file");
        readers += source.matches("var!(gap-caret)").count();
    }
    assert!(
        readers >= 6,
        "the gap token must be read by every caret-bearing trigger, found {readers} readers"
    );
}
