use super::*;

fn raw_spacing_dimension(value: &str) -> bool {
    let bytes: &[u8] = value.as_bytes();
    let mut index: usize = 0;
    while index < bytes.len() {
        if !bytes[index].is_ascii_digit() && bytes[index] != b'.' {
            index += 1;
            continue;
        }
        let start: usize = index;
        while index < bytes.len() && (bytes[index].is_ascii_digit() || bytes[index] == b'.') {
            index += 1;
        }
        let number: f64 = value[start..index].parse().unwrap_or(0.0);
        let unit: usize = index;
        while index < bytes.len() && (bytes[index].is_ascii_alphabetic() || bytes[index] == b'%') {
            index += 1;
        }
        if number != 0.0
            && matches!(
                &value[unit..index],
                "px" | "em"
                    | "rem"
                    | "%"
                    | "vw"
                    | "vh"
                    | "vmin"
                    | "vmax"
                    | "ch"
                    | "ex"
                    | "cm"
                    | "mm"
                    | "in"
                    | "pt"
                    | "pc"
                    | "dvh"
                    | "dvw"
            )
        {
            return true;
        }
    }
    false
}

#[test]
fn spacing_scanner_rejects_mixed_literals_and_commented_exemptions() {
    for value in [
        "\"6px\"",
        "format!(\"1px {}\", var!(space-sm))",
        "\"calc(100% - 2rem)\"",
        "\".5em\"",
        "\"-7px\" // optical",
    ] {
        assert!(raw_spacing_dimension(value), "{value}");
    }
    for value in [
        "var!(space-2xs)",
        "format!(\"calc({} / 2)\", var!(space-2xs))",
        "\"0px auto\"",
        "\"var(--euv-mobile-safe-top, 0px)\"",
    ] {
        assert!(!raw_spacing_dimension(value), "{value}");
    }
}

#[test]
fn every_registry_spacing_dimension_is_variable_owned() {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/style/class");
    let mut violations: Vec<String> = Vec::new();
    let mut declarations: usize = 0;
    for entry in fs::read_dir(&root).expect("registry groups").flatten() {
        let path: PathBuf = entry.path().join("fn.rs");
        if !path.is_file() {
            continue;
        }
        let source: String = fs::read_to_string(&path).expect("class registry");
        let mut pending: String = String::new();
        let mut class_name: &str = "";
        for (index, line) in source.lines().enumerate() {
            let code: &str = line.split("//").next().unwrap_or_default().trim();
            if code.starts_with("pub ") || code.starts_with("pub(crate) ") {
                class_name = code.split_whitespace().nth(1).unwrap_or_default();
            }
            if pending.is_empty() {
                let Some((property, _)) = code.split_once(':') else {
                    continue;
                };
                if !(property == "gap"
                    || property == "row-gap"
                    || property == "column-gap"
                    || property == "padding"
                    || property == "margin"
                    || property.starts_with("padding-")
                    || property.starts_with("margin-"))
                {
                    continue;
                }
            }
            pending.push_str(code);
            if !code.contains(';') {
                continue;
            }
            declarations += 1;
            let visually_hidden: bool = path.ends_with("overlay/fn.rs")
                && class_name == "c_file_upload_input_hidden"
                && pending == "margin: \"-1px\";";
            if raw_spacing_dimension(&pending) && !visually_hidden {
                violations.push(format!("{}:{} {pending}", path.display(), index + 1));
            }
            pending.clear();
        }
        assert!(
            pending.is_empty(),
            "unterminated spacing declaration: {}",
            path.display()
        );
    }
    println!(
        "spacing inventory: {declarations} declarations, {} violations",
        violations.len()
    );
    assert!(
        declarations > 1000,
        "audit must cover the complete registry, found {declarations}"
    );
    assert!(
        violations.is_empty(),
        "raw spacing dimensions ({declarations} declarations audited):\n{}",
        violations.join("\n")
    );
}

#[test]

fn standard_composite_parts_do_not_add_outer_spacing_to_container_gaps() {
    let parts: &[(&str, &Css)] = &[
        ("dialog header", c_euv_dialog_header()),
        ("dialog footer", c_euv_dialog_footer()),
        ("dialog title", c_euv_dialog_title()),
        ("dialog description", c_euv_dialog_description()),
        ("sheet header", c_euv_sheet_header()),
        ("sheet footer", c_euv_sheet_footer()),
        ("sheet title", c_euv_sheet_title()),
        ("sheet description", c_euv_sheet_description()),
        ("field content", c_euv_field_content()),
        ("field label", c_euv_field_label()),
        ("field description", c_euv_field_description()),
        ("field error", c_euv_field_error()),
        ("item content", c_euv_item_content()),
        ("item title", c_euv_item_title()),
        ("item description", c_euv_item_description()),
        ("menu group", c_euv_dropdown_menu_group()),
    ];
    for (name, css) in parts {
        for declaration in css.get_style().split(';') {
            let Some((property, value)) = declaration.split_once(':') else {
                continue;
            };
            if property.trim().starts_with("padding") && !name.starts_with("sheet ") {
                assert!(
                    matches!(value.trim(), "0" | "0px"),
                    "{name} duplicates its surface's inset: {declaration}"
                );
            }
            if property.trim().starts_with("margin") {
                let footer_push: bool = *name == "sheet footer"
                    && property.trim() == "margin-top"
                    && value.trim() == "auto";
                assert!(
                    matches!(value.trim(), "0" | "0px") || footer_push,
                    "{name} duplicates its container gap: {declaration}"
                );
            }
        }
    }
}

#[test]
fn card_surfaces_share_a_compact_inset_without_styling_their_content_twice() {
    for css in [c_card(), c_euv_feature_card(), c_virtual_list_card()] {
        assert_eq!(
            class_property(css, "padding").as_deref(),
            Some("var(--card-padding)")
        );
    }
    assert_eq!(
        class_property(c_card(), "gap").as_deref(),
        Some("var(--card-content-gap)")
    );
    for css in [
        c_euv_card_header(),
        c_euv_card_content(),
        c_euv_card_footer(),
    ] {
        assert_eq!(class_property(css, "padding"), None);
    }
    assert_eq!(
        class_property(c_card_stack(), "gap").as_deref(),
        Some("var(--page-block-gap)")
    );
    let stack: String = format!("{:?}", c_card_stack().get_pseudo_rules());
    assert!(stack.contains("data-euv-dynamic-id"));
    assert!(stack.contains("margin-block: 0px"));
    assert!(stack.contains(".c_page_container > .c_card_stack"));
    let mobile: String = format!("{:?}", c_card_stack().get_media_rules());
    assert!(mobile.contains("var(--page-block-gap-mobile)"));
    assert!(mobile.contains("pseudo_rules: [PseudoRule"), "{mobile}");
    let compact: String = mobile.split_whitespace().collect();
    assert!(
        !compact.contains("gap:var(--page-block-gap-mobile);:is("),
        "{mobile}"
    );
}

#[test]
fn feature_navigation_reserves_row_height_and_symmetric_paint_clearance() {
    for css in [c_nav_item_active(), c_nav_item_inactive()] {
        assert_eq!(
            class_property(css, "min-height").as_deref(),
            Some("var(--nav-row-height)")
        );
        assert_eq!(
            class_property(css, "padding").as_deref(),
            Some("var(--nav-row-padding) var(--edge-gutter-nav)")
        );
        let rules: String = format!("{:?}", css.get_pseudo_rules());
        assert!(rules.contains(":focus-visible"));
        assert!(rules.contains("var(--ring)"));
    }
    let scroll: &Css = c_nav_items_scroll();
    assert_eq!(
        class_property(scroll, "padding").as_deref(),
        Some("var(--space-sm) var(--nav-item-inset)")
    );
    assert_eq!(class_property(scroll, "min-height").as_deref(), Some("0px"));
    assert_eq!(
        class_property(scroll, "overflow-y").as_deref(),
        Some("auto")
    );
    assert_eq!(
        class_property(scroll, "scrollbar-gutter").as_deref(),
        Some("stable both-edges")
    );
    assert_eq!(
        class_property(scroll, "scrollbar-width").as_deref(),
        Some("thin")
    );
}
