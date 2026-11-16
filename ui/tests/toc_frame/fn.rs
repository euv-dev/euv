use super::*;

fn classes() -> [&'static Css; 6] {
    [
        c_euv_toc(),
        c_euv_toc_title(),
        c_euv_toc_link(),
        c_euv_toc_link_active(),
        c_euv_toc_link_nested(),
        c_euv_toc_link_nested_active(),
    ]
}

#[cfg(not(feature = "theme-custom"))]
fn value(css: &Css, property: &str) -> Option<String> {
    css.get_style()
        .split(';')
        .filter_map(|part: &str| part.split_once(':'))
        .filter(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim().to_string())
        .next_back()
}

#[cfg(not(feature = "theme-custom"))]
#[test]
fn toc_frame_has_no_outer_or_title_frame() {
    for css in classes() {
        let is_title: bool = css.get_name() == c_euv_toc_title().get_name();
        for edge in ["border-top", "border-right", "border-bottom", "border-left"] {
            assert_eq!(
                value(css, edge).as_deref(),
                None,
                "{}: {edge}",
                css.get_name()
            );
        }
        if !is_title {
            assert_eq!(
                value(css, "border").as_deref(),
                Some("none"),
                "{}",
                css.get_name()
            );
        }
        let pseudos: String = format!("{:?}", css.get_pseudo_rules());
        assert!(!pseudos.contains("border"), "{}: {pseudos}", css.get_name());
        assert!(
            !pseudos.contains("box-shadow"),
            "{}: {pseudos}",
            css.get_name()
        );
    }
    assert_eq!(value(c_euv_toc(), "padding-left"), None);
}

#[cfg(not(feature = "theme-custom"))]
#[test]
fn toc_frame_preserves_nested_indentation_and_active_weight() {
    assert_eq!(
        value(c_euv_toc_link(), "font-weight").as_deref(),
        Some("var(--font-weight-normal)")
    );
    for css in [c_euv_toc_link_active(), c_euv_toc_link_nested_active()] {
        assert_eq!(
            value(css, "font-weight").as_deref(),
            Some("var(--font-weight-bold)")
        );
        assert_eq!(value(css, "color").as_deref(), Some("var(--primary)"));
    }
    for css in [c_euv_toc_link_nested(), c_euv_toc_link_nested_active()] {
        assert_eq!(
            value(css, "padding-left").as_deref(),
            Some("var(--space-lg)")
        );
    }
    for css in &classes()[2..] {
        assert_eq!(value(css, "box-shadow").as_deref(), Some("none"));
        assert_eq!(value(css, "border-radius").as_deref(), Some("0"));
        assert_eq!(
            value(css, "outline"),
            None,
            "native focus must remain visible"
        );
    }
}

#[cfg(feature = "theme-custom")]
#[test]
fn toc_frame_leaves_custom_classes_empty() {
    for css in classes().into_iter().chain([c_euv_toc_link_separator()]) {
        assert!(css.get_style().is_empty(), "{}", css.get_name());
        assert!(css.get_pseudo_rules().is_empty(), "{}", css.get_name());
    }
}

#[cfg(not(feature = "theme-custom"))]
#[test]
fn toc_frame_separator_adds_only_a_straight_horizontal_rule() {
    let separator: &Css = c_euv_toc_link_separator();
    assert!(separator.get_style().is_empty());
    assert_eq!(
        separator.get_pseudo_rules(),
        &vec![PseudoRule::new(
            ":is(.c_euv_toc_link_separator)".to_string(),
            "border-bottom: var(--border-width) solid var(--border); padding-bottom: var(--space-xs); ".to_string(),
        )]
    );
}

fn items() -> &'static [EuvTocItem] {
    &[
        EuvTocItem {
            level: 2,
            text: "First",
            href: "#first",
        },
        EuvTocItem {
            level: 3,
            text: "Nested",
            href: "#nested",
        },
        EuvTocItem {
            level: 2,
            text: "Next",
            href: "#next",
        },
        EuvTocItem {
            level: 4,
            text: "Last",
            href: "#last",
        },
    ]
}

fn render(items: &'static [EuvTocItem], active: &str) -> VirtualNode {
    euv_toc(VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvTocProps {
            title: "On this page",
            items,
            active: Signal::create(active.to_string()),
        })),
    })
}

fn anchor_classes(node: &VirtualNode, result: &mut Vec<String>) {
    if let VirtualNode::Element { attributes, .. } = node
        && node.try_get_tag_name().as_deref() == Some("a")
    {
        let names: Vec<String> = attributes
            .iter()
            .filter(|entry: &&AttributeEntry| entry.get_name() == "class")
            .filter_map(AttributeEntry::text_value)
            .collect();
        result.push(names.join(" "));
    }
    for child in node.get_children() {
        anchor_classes(child, result);
    }
}

#[test]
fn toc_frame_places_exactly_one_separator_between_items_and_none_after_the_last() {
    for count in 0..=items().len() {
        for active in ["", "#first", "#nested", "#next", "#last", "#missing"] {
            let rendered: VirtualNode = render(&items()[..count], active);
            let mut classes: Vec<String> = Vec::new();
            anchor_classes(&rendered, &mut classes);
            assert_eq!(classes.len(), count, "count={count}, active={active}");
            for (index, names) in classes.iter().enumerate() {
                let separated: bool = names
                    .split_whitespace()
                    .any(|name: &str| name == c_euv_toc_link_separator().get_name());
                assert_eq!(
                    separated,
                    index + 1 < count,
                    "{index}/{count}, active={active}: {names}"
                );
            }
        }
    }
}
