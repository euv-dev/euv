use super::*;

fn props<T>(value: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(value)),
    }
}

fn children(node: &VirtualNode) -> &[VirtualNode] {
    match node {
        VirtualNode::Element { children, .. } | VirtualNode::Fragment(children) => children,
        _ => &[],
    }
}

fn class_node<'a>(node: &'a VirtualNode, class: &Css) -> Option<&'a VirtualNode> {
    if matches!(node, VirtualNode::Element { attributes, .. } if attributes.iter().any(|entry: &AttributeEntry| entry.class_names().iter().any(|name: &String| name == class.get_name())))
    {
        return Some(node);
    }
    children(node)
        .iter()
        .find_map(|child: &VirtualNode| class_node(child, class))
}

fn count_tag(node: &VirtualNode, name: &str) -> usize {
    let own: usize = usize::from(
        matches!(node, VirtualNode::Element { tag: Tag::Element(tag), .. } if tag == name),
    );
    own + children(node)
        .iter()
        .map(|child: &VirtualNode| count_tag(child, name))
        .sum::<usize>()
}

fn dynamic_count(node: &VirtualNode) -> usize {
    usize::from(matches!(node, VirtualNode::Dynamic(_)))
        + children(node).iter().map(dynamic_count).sum::<usize>()
}

fn declaration<'a>(style: &'a str, property: &str) -> Option<&'a str> {
    style
        .split(';')
        .filter_map(|part: &str| part.split_once(':'))
        .find(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim())
}

fn natural_footer_style(style: &str) -> bool {
    ["position", "bottom", "left", "right", "z-index", "height"]
        .iter()
        .all(|property: &&str| declaration(style, property).is_none())
}

fn shell_has_no_footer(source: &str) -> bool {
    !source.contains("docs_site_footer") && !source.contains("euv_footer")
}

fn app_declaration(selector: &str, property: &str) -> Option<&'static str> {
    APP_GLOBAL_CSS
        .split('}')
        .filter_map(|rule: &str| rule.rsplit_once('{'))
        .filter(|(selectors, _): &(&str, &str)| {
            selectors
                .split(',')
                .any(|candidate: &str| candidate.trim() == selector)
        })
        .filter_map(|(_, style): (&str, &str)| declaration(style, property))
        .next_back()
}

#[test]
fn document_footer_slot_is_inside_the_middle_content_tail() {
    let layout: VirtualNode = euv_doc_layout(props(EuvDocLayoutProps {
        footer: "License",
        footer_link: "https://example.com/license",
        toc_items: &[EuvTocItem {
            text: "Heading",
            href: "#heading",
            level: 2,
        }],
        ..EuvDocLayoutProps::default()
    }));
    let content: &VirtualNode =
        class_node(&layout, c_euv_doc_content()).expect("middle content column");
    let tail: &VirtualNode =
        class_node(content, c_euv_doc_tail()).expect("tail inside middle content");
    assert_eq!(
        children(tail)
            .iter()
            .filter(|node: &&VirtualNode| matches!(node, VirtualNode::Dynamic(_)))
            .count(),
        1
    );
    assert!(matches!(
        children(&layout).last(),
        Some(VirtualNode::Dynamic(_))
    ));
    let shared: &str = include_str!("../../../ui/src/component/doc_layout/view/fn.rs");
    let tail_source: &str = shared
        .split("class: c_euv_doc_tail()")
        .nth(1)
        .expect("tail source")
        .split("class: c_euv_doc_toc()")
        .next()
        .expect("TOC follows content");
    assert_eq!(tail_source.matches("euv_footer {").count(), 1);
    assert!(tail_source.contains("text: footer"));
    assert!(tail_source.contains("link: footer_link"));
    assert!(tail_source.contains("items: footer_items"));
    let source: &str = include_str!("../../src/component/doc_page/view/fn.rs");
    assert!(source.contains("docs_footer_props(locale, Some(page), &crate::generated::SITE)"));
    for forwarding in [
        "footer: footer.text",
        "footer_link: footer.link",
        "footer_items: footer.items",
    ] {
        assert!(source.contains(forwarding));
    }
    assert!(!source.contains("docs_site_footer {"));
}

#[test]
fn home_footer_is_inside_the_content_after_the_body() {
    let page: &DocsPage = crate::generated::SITE
        .pages
        .iter()
        .find(|page: &&DocsPage| page.home)
        .expect("generated home");
    let home: VirtualNode = docs_home_page(props(DocsPageProps {
        route_signal: Signal::create(page.route.to_string()),
    }));
    let content: &VirtualNode = class_node(&home, c_home_content()).expect("home content");
    let body: &VirtualNode = class_node(content, c_home_body()).expect("home body");
    let tail: &VirtualNode = class_node(content, c_home_tail()).expect("home tail");
    assert_eq!(count_tag(body, "footer"), 0);
    assert_eq!(count_tag(tail, "footer"), 1);
    assert_eq!(count_tag(&home, "footer"), 1);
    assert!(
        class_node(
            children(content).last().expect("last content region"),
            c_home_tail()
        )
        .is_some()
    );
}

#[test]
fn missing_page_has_one_footer_inside_the_centered_container() {
    let page: VirtualNode = docs_not_found(props(DocsPageProps {
        route_signal: Signal::create("/missing-footer-regression.html".to_string()),
    }));
    let content: &VirtualNode = class_node(&page, c_page_container()).expect("centered page");
    assert_eq!(count_tag(content, "footer"), 1);
    assert_eq!(count_tag(&page, "footer"), 1);
}

#[test]
fn locked_page_has_one_centered_footer_and_no_document_body() {
    let gate: VirtualNode = HookContext::with(HookContext::current(), || {
        docs_password_gate(props(DocsPasswordGateProps {
            route: "/private-footer-regression.html",
            expected_hash: "hash",
            title: "Protected document",
        }))
    });
    let content: &VirtualNode = class_node(&gate, c_page_container()).expect("centered gate page");
    let form: &VirtualNode = class_node(content, c_pw_gate_wrapper()).expect("gate form region");
    assert_eq!(count_tag(form, "footer"), 0);
    assert_eq!(count_tag(content, "footer"), 1);
    assert_eq!(count_tag(&gate, "footer"), 1);
    assert_eq!(count_tag(&gate, "article"), 0);
    let source: &str = include_str!("../../src/component/password_gate/view/fn.rs");
    let view: &str = source
        .split("pub(crate) fn docs_password_gate")
        .nth(1)
        .expect("gate view")
        .split("/// Builds the submit handler")
        .next()
        .expect("gate view end");
    assert_eq!(view.matches("docs_site_footer {").count(), 1);
    assert!(view.contains("route: route.to_string()"));
    for private_content in ["docs_markdown", "page.blocks", "docs_doc_page"] {
        assert!(!view.contains(private_content));
    }
    let route: &str = include_str!("../../src/component/doc_page/view/fn.rs");
    assert!(route.contains("Some(page) if page.private && !is_unlocked(&path)"));
    assert_eq!(route.matches("docs_password_gate {").count(), 1);
}

#[test]
fn neither_shell_owns_a_footer() {
    let shell: &str = include_str!("../../src/component/layout/view/fn.rs");
    assert!(shell_has_no_footer(shell));
    assert!(!shell_has_no_footer(
        "div { main { docs_main { route_signal } } docs_site_footer { route_signal } }"
    ));
}

#[test]
fn page_override_and_locale_fallback_share_one_resolver() {
    let locale: DocsLocale = DocsLocale {
        footer: "Localized license",
        ..crate::generated::SITE.locales[0]
    };
    let page: DocsPage = DocsPage {
        footer: "Page license",
        ..crate::generated::SITE.pages[0]
    };
    let site: &DocsSite = &crate::generated::SITE;
    assert_eq!(
        docs_footer_props(&locale, Some(&page), site).text,
        "Page license"
    );
    assert_eq!(
        docs_footer_props(&locale, Some(&DocsPage { footer: "", ..page }), site).text,
        "Localized license"
    );
    assert_eq!(
        docs_footer_props(&locale, None, site).text,
        "Localized license"
    );
    let other: DocsLocale = DocsLocale {
        footer: "Other locale license",
        ..locale
    };
    assert_eq!(
        docs_footer_props(&other, None, site).text,
        "Other locale license"
    );
    for page in site.pages {
        let footer: VirtualNode = docs_site_footer(props(DocsSiteFooterProps {
            route: page.route.to_string(),
        }));
        assert_eq!(count_tag(&footer, "footer"), 1);
    }
}

#[test]
fn notices_links_and_icons_are_forwarded_without_loss() {
    let site: DocsSite = DocsSite {
        footer_link: "https://example.com/license",
        footer_items: &[EuvFooterItem {
            text: "Notice",
            link: "https://example.com/notice",
            icon: "/badge.svg",
            alt: "Official badge",
        }],
        ..crate::generated::SITE
    };
    let footer: EuvFooterProps = docs_footer_props(&site.locales[0], None, &site);
    assert_eq!(footer.link, site.footer_link);
    assert_eq!(footer.items.len(), 1);
    assert_eq!(footer.items[0].text, "Notice");
    assert_eq!(footer.items[0].link, "https://example.com/notice");
    assert_eq!(footer.items[0].icon, "/badge.svg");
    assert_eq!(footer.items[0].alt, "Official badge");
    let rendered: VirtualNode = euv_footer(props(footer));
    assert_eq!(count_tag(&rendered, "footer"), 1);
    assert_eq!(dynamic_count(&rendered), 2);
    let wrapper: &str = include_str!("../../src/component/site_footer/view/fn.rs");
    for forwarding in [
        "text: footer.text",
        "link: footer.link",
        "items: footer.items",
    ] {
        assert!(wrapper.contains(forwarding));
    }
    let shared: &str = include_str!("../../../ui/src/component/footer/view/fn.rs");
    for retained in [
        "href: link",
        "href: item.link",
        "target: LINK_TARGET_BLANK",
        "rel: LINK_REL_NOOPENER",
        "footer_item_icon(item)",
        "src: item.icon",
        "alt: item.alt",
    ] {
        assert!(shared.contains(retained));
    }
}

#[test]
fn footer_styles_are_natural_wrapping_and_do_not_reserve_a_bar() {
    assert!(natural_footer_style(c_euv_footer().get_style()));
    let old_fixed_style: &str = "position: fixed; left: 0px; right: 0px; bottom: var(--edge-gutter-bottom); z-index: 90; display: flex; height: var(--docs-site-footer-height); padding: 0px 24px;";
    assert!(!natural_footer_style(old_fixed_style));
    for property in ["position", "bottom", "left", "right", "z-index", "height"] {
        assert_eq!(declaration(c_euv_footer().get_style(), property), None);
        assert_eq!(app_declaration("#app .c_euv_footer", property), None);
    }
    assert_eq!(
        app_declaration("#app .c_euv_footer", "flex-wrap"),
        Some("wrap")
    );
    assert_eq!(
        app_declaration("#app .c_euv_footer_item_link", "white-space"),
        Some("normal")
    );
    assert_eq!(
        app_declaration("#app .c_euv_footer_item_link", "overflow-wrap"),
        Some("anywhere")
    );
    for selector in ["#app .c_app_main", "#app .c_mobile_main"] {
        assert_eq!(app_declaration(selector, "display"), Some("flex"));
        assert_eq!(app_declaration(selector, "min-height"), Some("0"));
        assert_eq!(app_declaration(selector, "overflow-y"), Some("auto"));
    }
    assert_eq!(
        app_declaration("#app .c_app_main", "padding-bottom"),
        Some("var(--edge-gutter-bottom, 20px)")
    );
    assert_eq!(app_declaration("#app .c_mobile_main", "padding-top"), None);
    assert_eq!(
        app_declaration("#app .c_mobile_main", "padding-bottom"),
        None
    );
    assert_eq!(app_declaration("#app .c_app_nav", "padding-bottom"), None);
    assert!(!APP_GLOBAL_CSS.contains("docs-site-footer-height"));
    assert!(!include_str!("../../src/lib.rs").contains("SITE_FOOTER_CSS"));
    assert!(!include_str!("../../src/component/site_footer/view/mod.rs").contains("mod r#const"));
    for style in [c_euv_doc_content(), c_home_content()] {
        let expected: Option<&str> = if cfg!(feature = "theme-custom") {
            assert!(style.get_style().is_empty());
            None
        } else {
            Some("space-between")
        };
        assert_eq!(declaration(style.get_style(), "justify-content"), expected);
    }
}
