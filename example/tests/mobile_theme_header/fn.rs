use super::*;

fn mobile_node() -> VirtualNode {
    mobile_layout(VirtualNode::Element {
        tag: Tag::Element("div".into()),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(MobileLayoutProps {
            route_signal: Signal::create("/header-test".to_string()),
            theme_signal: Signal::create("light".to_string()),
            root_class_signal: Signal::create(String::new()),
            panel_open: Signal::create(false),
            drawer_open: Signal::create(false),
        })),
    })
}

fn elements(node: &VirtualNode) -> Vec<&VirtualNode> {
    let mut result: Vec<&VirtualNode> = vec![node];
    for child in node.get_children() {
        result.extend(elements(child));
    }
    result
}

fn attribute(node: &VirtualNode, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element { attributes, .. } => attributes
            .iter()
            .find(|entry: &&AttributeEntry| entry.get_name() == name)
            .and_then(|entry: &AttributeEntry| entry.text_value()),
        _ => None,
    }
}

fn has_class(node: &VirtualNode, name: &str) -> bool {
    match node {
        VirtualNode::Element { attributes, .. } => {
            attributes.iter().any(|entry: &AttributeEntry| {
                entry.get_name() == "class"
                    && entry
                        .class_names()
                        .iter()
                        .any(|value: &String| value == name)
            })
        }
        _ => false,
    }
}

#[test]
fn mobile_theme_header_contains_the_only_selector_and_keeps_all_builds() {
    let node: VirtualNode = mobile_node();
    let all: Vec<&VirtualNode> = elements(&node);
    let header: &VirtualNode = all
        .iter()
        .copied()
        .find(|entry: &&VirtualNode| entry.try_get_tag_name().as_deref() == Some("header"))
        .expect("mobile header");
    let header_nodes: Vec<&VirtualNode> = elements(header);
    let selector: &VirtualNode = header_nodes
        .iter()
        .copied()
        .find(|entry: &&VirtualNode| {
            attribute(entry, "id").as_deref() == Some(THEME_FEATURE_SELECTOR_ID)
        })
        .expect("theme selector in the header");
    assert_eq!(
        all.iter()
            .filter(|entry: &&&VirtualNode| attribute(entry, "id").as_deref()
                == Some(THEME_FEATURE_SELECTOR_ID))
            .count(),
        1
    );
    assert_eq!(
        attribute(selector, "aria-label").as_deref(),
        Some(THEME_FEATURE_SELECTOR_LABEL)
    );
    let values: Vec<String> = elements(selector)
        .iter()
        .filter(|entry: &&&VirtualNode| entry.try_get_tag_name().as_deref() == Some("option"))
        .map(|entry: &&VirtualNode| attribute(entry, "value").expect("option value"))
        .collect();
    assert_eq!(
        values,
        THEME_FEATURE_OPTIONS
            .iter()
            .map(|(value, _): &(&str, &str)| value.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(values.len(), THEME_FEATURE_OPTIONS.len());
    assert!(
        header_nodes
            .iter()
            .any(|entry: &&VirtualNode| has_class(entry, "c_mobile_theme_button"))
    );
    assert!(
        header_nodes
            .iter()
            .any(|entry: &&VirtualNode| has_class(entry, "c_mobile_header_actions"))
    );
    assert!(
        !all.iter()
            .any(|entry: &&VirtualNode| has_class(entry, "c_nav_scheme_label"))
    );
    let nav: &VirtualNode = all
        .iter()
        .copied()
        .find(|entry: &&VirtualNode| entry.try_get_tag_name().as_deref() == Some("nav"))
        .expect("drawer");
    assert!(
        !elements(nav)
            .iter()
            .any(|entry: &&VirtualNode| attribute(entry, "id").as_deref()
                == Some(THEME_FEATURE_SELECTOR_ID))
    );
    let selected: String = attribute(selector, "value").expect("selected compiled feature");
    assert!(values.contains(&selected));
    {
        assert!(
            selected == THEME_FEATURE_DEFAULT
                || selected == format!("{}{}", THEME_FEATURE_PREFIX, scheme_name())
        );
    }
    if let VirtualNode::Element { attributes, .. } = selector {
        assert!(
            attributes
                .iter()
                .any(|entry: &AttributeEntry| entry.get_name() == "onchange")
        );
    }
}

#[test]
fn mobile_theme_header_styles_bound_the_native_control_without_clipping_it() {
    let selector: String = format!(
        "{} {:?}",
        c_mobile_theme_feature_selector().get_style(),
        c_mobile_theme_feature_selector().get_pseudo_rules()
    )
    .replace(' ', "");
    let actions: String = format!(
        "{} {:?}",
        c_mobile_header_actions().get_style(),
        c_mobile_header_actions().get_pseudo_rules()
    )
    .replace(' ', "");
    assert!(selector.contains("min-width:0"));
    assert!(selector.contains("width:100%"));
    assert!(selector.contains("min-height:var(--space-4xl)"));
    assert!(!selector.contains("overflow:hidden"));
    assert!(actions.contains("min-width:0"));
    assert!(actions.contains("flex-shrink:0"));
}

#[test]
fn desktop_keeps_its_sidebar_selector_and_standalone_scheme_label() {
    let node: VirtualNode = desktop_layout(VirtualNode::Element {
        tag: Tag::Element("div".into()),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(DesktopLayoutProps {
            route_signal: Signal::create("/header-test".to_string()),
            theme_signal: Signal::create("light".to_string()),
            root_class_signal: Signal::create(String::new()),
            panel_open: Signal::create(false),
        })),
    });
    let all: Vec<&VirtualNode> = elements(&node);
    assert!(
        all.iter()
            .any(|entry: &&VirtualNode| has_class(entry, "c_nav_scheme_label"))
    );
    assert!(
        all.iter()
            .any(|entry: &&VirtualNode| has_class(entry, "c_nav_theme_feature_selector"))
    );
    assert!(
        !all.iter()
            .any(|entry: &&VirtualNode| has_class(entry, "c_mobile_header_actions"))
    );
}
