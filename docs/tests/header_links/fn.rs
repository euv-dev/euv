use super::*;

fn value(css: &Css, property: &str) -> Option<String> {
    css.get_style()
        .split(';')
        .filter_map(|part: &str| part.split_once(':'))
        .filter(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim().to_string())
        .next_back()
}

fn pseudo(css: &Css, selector: &str) -> String {
    let rules: String = format!("{:?}", css.get_pseudo_rules());
    let marker: String = format!("selector: \"{selector}\"");
    let mut found: Option<String> = None;
    let mut cursor: usize = 0;
    while let Some(start) = rules[cursor..].find(&marker) {
        let start: usize = cursor + start;
        let tail: &str = &rules[start..];
        let Some(open) = tail.find("style: \"") else {
            break;
        };
        let body: &str = &tail[open + "style: \"".len()..];
        let Some(end) = body.find('"') else {
            break;
        };
        found = Some(body[..end].to_string());
        cursor = start + marker.len();
    }
    found.unwrap_or_default()
}

#[test]
fn the_inactive_header_link_paints_a_lock_in_bar_on_hover_and_keeps_its_background_transparent() {
    let normal: &Css = c_docs_header_link();
    assert_eq!(normal.get_name(), "c_docs_header_link");
    assert_eq!(value(normal, "background"), None);
    assert_eq!(value(normal, "color").as_deref(), Some("var(--foreground)"));
    let hover: String = pseudo(normal, ":hover");
    assert!(
        hover.contains("inset 3px 0 0 0 var(--foreground)"),
        "the hover must paint the same left bar the sidebar tree paints, got {hover}"
    );
    assert!(
        !hover.contains("border"),
        "the hover must not draw a border, or the text would move, got {hover}"
    );
    assert!(
        hover.contains("background: transparent"),
        "the hover must leave the row's own background transparent, got {hover}"
    );
}

#[test]
fn the_active_header_link_paints_the_primary_fill_and_keeps_it_on_hover() {
    let active: &Css = c_docs_header_link_active();
    assert_eq!(active.get_name(), "c_docs_header_link_active");
    assert_eq!(
        value(active, "background").as_deref(),
        Some("var(--primary)"),
        "the active link must be filled with the primary colour, the way the sidebar's active row is"
    );
    assert_eq!(
        value(active, "color").as_deref(),
        Some("var(--primary-foreground)")
    );
    assert_eq!(
        value(active, "font-weight").as_deref(),
        Some("var(--font-weight-semibold)")
    );
    let hover: String = pseudo(active, ":hover");
    assert!(
        hover.contains("background: var(--primary)"),
        "the active link must keep its fill on hover, got {hover}"
    );
    assert!(
        hover.contains("box-shadow: none"),
        "the active link must drop the bar on hover, because the fill already carries the weight, got {hover}"
    );
}

#[test]
fn no_header_link_state_paints_the_accent_colour() {
    for css in [c_docs_header_link(), c_docs_header_link_active()] {
        let style: &str = css.get_style();
        assert!(
            !style.contains("--accent"),
            "{} must not paint the accent, which is the header's own background: {style}",
            css.get_name()
        );
        let pseudo_rules: String = format!("{:?}", css.get_pseudo_rules());
        assert!(
            !pseudo_rules.contains("--accent"),
            "{} must not paint the accent on hover either: {pseudo_rules}",
            css.get_name()
        );
    }
}

#[test]
fn header_layout_does_not_override_the_active_class_text_color() {
    let shared: &str = APP_GLOBAL_CSS
        .split_once("#app .c_docs_header_link, #app .c_docs_header_link_active {")
        .unwrap()
        .1
        .split_once('}')
        .unwrap()
        .0;
    assert!(!shared.contains("color:"));
    assert!(!shared.contains("background:"));
    assert!(!shared.contains("font-weight:"));
    assert!(
        shared.contains("box-shadow 150ms ease"),
        "the bar must fade in with the other transitions, got {shared}"
    );
}

#[test]
fn the_header_layout_owns_no_hover_paint_of_its_own() {
    assert!(
        !APP_GLOBAL_CSS.contains("#app .c_docs_header_link:hover"),
        "the id-scoped hover rule would outrank the class rule and hide the bar"
    );
    assert!(
        !APP_GLOBAL_CSS.contains("c_docs_header_icon_button:hover { background: var(--accent)"),
        "the accent fill is the header's own background, so the theme button's hover showed nothing"
    );
}

#[test]
fn the_header_carries_no_vertical_rule_between_the_search_field_and_the_theme_button() {
    assert!(
        !APP_GLOBAL_CSS.contains("c_docs_header_separator"),
        "the search field now paints its own border, so the hairline is a second divider"
    );
    let view: &str = include_str!("../../src/component/header/view/fn.rs");
    assert!(
        !view.contains("CLASS_DOCS_HEADER_SEPARATOR"),
        "the separator's class, its rule and its constant must all be gone"
    );
    let app: &str = include_str!("../../src/app/const.rs");
    assert!(
        !app.contains("CLASS_DOCS_HEADER_SEPARATOR"),
        "the separator's constant must be gone"
    );
}

fn anchor<'a>(node: &'a VirtualNode, href: &str) -> Option<&'a VirtualNode> {
    if matches!(node, VirtualNode::Element { tag: Tag::Element(tag), attributes, .. }
        if tag == "a" && attributes.iter().any(|entry: &AttributeEntry|
            entry.get_name() == "href" && entry.text_value().as_deref() == Some(href))
            && attributes.iter().any(|entry: &AttributeEntry| entry.class_names().iter().any(|name: &String|
                name == c_docs_header_link().get_name() || name == c_docs_header_link_active().get_name())))
    {
        return Some(node);
    }
    node.get_children()
        .iter()
        .find_map(|child: &VirtualNode| anchor(child, href))
}

fn assert_rendered_link(locale: &'static DocsLocale, link: &str, route: &str, expected: bool) {
    let header: VirtualNode = docs_header(
        locale,
        Signal::create(route.to_string()),
        Signal::create("light".to_string()),
        Signal::create(String::new()),
        Signal::create(false),
    );
    let href: String = if link.starts_with("http") {
        link.to_string()
    } else {
        format!("#{link}")
    };
    let node: &VirtualNode = anchor(&header, &href).expect("actual header anchor");
    let VirtualNode::Element { attributes, .. } = node else {
        panic!("header anchor must be an element");
    };
    let classes: Vec<String> = attributes
        .iter()
        .flat_map(AttributeEntry::class_names)
        .collect();
    assert_eq!(
        classes.contains(&c_docs_header_link_active().get_name().to_string()),
        expected,
        "link {link}, route {route}"
    );
    assert_eq!(
        classes.contains(&c_docs_header_link().get_name().to_string()),
        !expected,
        "link {link}, route {route}"
    );
}

#[test]
fn rendered_header_sections_normalize_slashes_anchors_and_prefix_boundaries() {
    let locale: &'static DocsLocale = Box::leak(Box::new(DocsLocale {
        prefix: "/",
        navbar: &[
            EuvNavbarItem {
                text: "Guide",
                link: "/guide/",
            },
            EuvNavbarItem {
                text: "Guide without slash",
                link: "/guide",
            },
            EuvNavbarItem {
                text: "Anchored guide",
                link: "/guide/#install",
            },
            EuvNavbarItem {
                text: "External",
                link: "https://example.com/guide/",
            },
        ],
        ..crate::generated::SITE.locales[0]
    }));
    for link in ["/guide/", "/guide", "/guide/#install"] {
        for route in [
            "/guide",
            "/guide/",
            "/guide/#hello",
            "/guide/getting-started.html#hello",
            "/guide/nested/page.html",
        ] {
            assert_rendered_link(locale, link, route, true);
        }
        for route in [
            "/",
            "/guides/",
            "/guidebook/page.html",
            "/other/guide/",
            "/zh/guide/",
        ] {
            assert_rendered_link(locale, link, route, false);
        }
    }
    assert_rendered_link(locale, "https://example.com/guide/", "/guide/", false);
}

#[test]
fn rendered_header_site_and_locale_home_links_are_exact_only() {
    let locale: &'static DocsLocale = Box::leak(Box::new(DocsLocale {
        prefix: "/zh/",
        navbar: &[
            EuvNavbarItem {
                text: "Home",
                link: "/",
            },
            EuvNavbarItem {
                text: "Empty home",
                link: "",
            },
            EuvNavbarItem {
                text: "Locale home",
                link: "/zh/",
            },
            EuvNavbarItem {
                text: "Localized guide",
                link: "/zh/guide/",
            },
        ],
        ..crate::generated::SITE.locales[0]
    }));
    for link in ["/", ""] {
        for route in ["", "/", "/#hello"] {
            assert_rendered_link(locale, link, route, true);
        }
        for route in ["/guide/", "/zh/", "/zh/guide/page.html#hello"] {
            assert_rendered_link(locale, link, route, false);
        }
    }
    for route in ["/zh", "/zh/", "/zh/#hello"] {
        assert_rendered_link(locale, "/zh/", route, true);
    }
    for route in ["/", "/zh/guide/", "/zh/guide/page.html#hello", "/zh-other/"] {
        assert_rendered_link(locale, "/zh/", route, false);
    }
    assert_rendered_link(locale, "/zh/guide/", "/zh/guide/page.html#hello", true);
    assert_rendered_link(locale, "/zh/guide/", "/zh/guides/page.html", false);
}
