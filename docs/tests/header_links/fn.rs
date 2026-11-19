use super::*;

fn value(css: &Css, property: &str) -> Option<String> {
    css.get_style()
        .split(';')
        .filter_map(|part: &str| part.split_once(':'))
        .filter(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim().to_string())
        .next_back()
}

#[test]
fn active_header_links_have_a_persistent_theme_background_and_contrasting_text() {
    let active: &Css = c_docs_header_link_active();
    assert_eq!(active.get_name(), "c_docs_header_link_active");
    assert_eq!(
        value(active, "background").as_deref(),
        Some("var(--accent)")
    );
    assert_eq!(
        value(active, "color").as_deref(),
        Some("var(--accent-foreground)")
    );
    assert!(active.get_pseudo_rules().is_empty());
    let normal: &Css = c_docs_header_link();
    assert_eq!(normal.get_name(), "c_docs_header_link");
    assert_eq!(value(normal, "background"), None);
    assert_eq!(value(normal, "color").as_deref(), Some("var(--foreground)"));
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
