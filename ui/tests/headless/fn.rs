use super::*;

fn registry_classes() -> [&'static Css; 7] {
    [
        c_virtual_list_status(),
        c_card_title(),
        c_euv_input_wrapper(),
        c_euv_user_select_text(),
        c_euv_feature_grid(),
        c_camera_video_container(),
        c_app_root(),
    ]
}

fn assert_style_mode(class: &Css) {
    {
        assert!(
            !class.get_style().is_empty(),
            "{} must retain its default theme",
            class.get_name()
        );
    }
}

fn hidden_values(node: &VirtualNode, values: &mut Vec<String>) {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            for attribute in attributes {
                if attribute.get_name() == "hidden" {
                    values.push(attribute.text_value().unwrap_or_default());
                }
            }
            for child in children {
                hidden_values(child, values);
            }
        }
        VirtualNode::Fragment(children) => {
            for child in children {
                hidden_values(child, values);
            }
        }
        _ => {}
    }
}

fn drawer(open: bool) -> VirtualNode {
    euv_drawer(VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(EuvDrawerProps {
            open: Signal::create(open),
        })),
    })
}

#[test]
fn every_registry_group_respects_the_configured_style_mode() {
    for class in registry_classes() {
        assert_style_mode(class);
    }
}

#[test]
fn parameterized_registry_classes_respect_the_same_style_mode() {
    let class: Css = c_loading_overlay("transparent");
    assert_style_mode(&class);
    assert!(class.get_name().starts_with("c_loading_overlay-"));
}

#[test]
fn disabling_default_styles_keeps_public_class_names() {
    assert_eq!(c_app_root().get_name(), "c_app_root");
    assert_eq!(c_euv_input_wrapper().get_name(), "c_euv_input_wrapper");
}

#[test]
fn markdown_styles_remain_available_in_both_feature_modes() {
    assert!(euv_md_css().contains(".md-body"));
    inject_app_global_css();
}

#[test]
fn drawer_visibility_does_not_add_native_hidden_attributes() {
    for open in [false, true] {
        let mut values: Vec<String> = Vec::new();
        hidden_values(&drawer(open), &mut values);
        assert!(values.is_empty());
    }
}

class! {
    c_consumer_custom_style {
        color: "inherit";
    }
}

#[test]
fn consumer_class_rules_remain_enabled() {
    assert!(c_consumer_custom_style().get_style().contains("inherit"));
}

#[test]
fn theme_factories_remain_available_in_both_feature_modes() {
    assert_eq!(c_theme_light().get_name(), "c_theme_light");
    assert_eq!(c_theme_dark().get_name(), "c_theme_dark");
}
