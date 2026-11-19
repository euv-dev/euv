use super::*;

fn render_image(
    status: EuvImageStatus,
    width: Option<u32>,
    height: Option<u32>,
    placeholder: &'static str,
) -> VirtualNode {
    let mut context: HookContext = HookContext::default();
    let state: Signal<EuvImageStatus> = HookContext::with(context.clone(), || {
        HookContext::use_hook(|| Signal::create(status))
    });
    state.set(status);
    context.reset_index();
    HookContext::with(context, || {
        euv_image(VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("div")),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: Some(Box::new(EuvImageProps {
                src: "https://example.invalid/placeholder.png",
                alt: "Original accessible image",
                placeholder,
                width,
                height,
            })),
        })
    })
}

fn element_attributes(node: &VirtualNode) -> &[AttributeEntry] {
    match node {
        VirtualNode::Element { attributes, .. } => attributes,
        _ => panic!("expected an element"),
    }
}

fn element_children(node: &VirtualNode) -> &[VirtualNode] {
    match node {
        VirtualNode::Element { children, .. } => children,
        _ => panic!("expected an element"),
    }
}

fn attribute_text(node: &VirtualNode, name: &str) -> Option<String> {
    element_attributes(node)
        .iter()
        .find(|entry: &&AttributeEntry| entry.get_name() == name)
        .and_then(AttributeEntry::text_value)
}

fn has_class(node: &VirtualNode, name: &str) -> bool {
    element_attributes(node)
        .iter()
        .any(|entry: &AttributeEntry| {
            entry
                .class_names()
                .iter()
                .any(|class: &String| class == name)
        })
}

#[test]
fn unavailable_images_keep_native_loading_and_accessibility_without_flow_layout() {
    for status in [EuvImageStatus::Loading, EuvImageStatus::Failed] {
        let node: VirtualNode = render_image(status, None, None, "Historical placeholder");
        let children: &[VirtualNode] = element_children(&node);
        assert_eq!(children.len(), 2);
        assert!(has_class(&node, "c_euv_image"));
        assert!(has_class(
            &node,
            match status {
                EuvImageStatus::Loading => "c_euv_image_placeholder",
                _ => "c_euv_image_failed",
            }
        ));
        assert!(format!("{:?}", children[0]).contains("Historical placeholder"));
        assert_eq!(element_children(&children[0]).len(), 1);
        let picture: &VirtualNode = &children[1];
        assert!(has_class(picture, "c_euv_image_unavailable_picture"));
        assert_eq!(
            attribute_text(picture, "src").as_deref(),
            Some("https://example.invalid/placeholder.png")
        );
        assert_eq!(
            attribute_text(picture, "alt").as_deref(),
            Some("Original accessible image")
        );
        assert_eq!(attribute_text(picture, "loading").as_deref(), Some("lazy"));
        for name in ["onload", "onerror"] {
            assert!(
                element_attributes(picture)
                    .iter()
                    .any(|entry: &AttributeEntry| entry.get_name() == name)
            );
        }
        for name in ["hidden", "aria-hidden", "style"] {
            assert!(attribute_text(picture, name).is_none());
        }
    }
}

#[test]
fn loaded_image_removes_only_unavailable_presentation() {
    let node: VirtualNode =
        render_image(EuvImageStatus::Loaded, None, None, "Historical placeholder");
    let children: &[VirtualNode] = element_children(&node);
    assert_eq!(children.len(), 1);
    assert!(has_class(&node, "c_euv_image"));
    assert!(!has_class(&node, "c_euv_image_placeholder"));
    assert!(!has_class(&node, "c_euv_image_failed"));
    assert!(!has_class(&children[0], "c_euv_image_unavailable_picture"));
    assert_eq!(
        attribute_text(&children[0], "alt").as_deref(),
        Some("Original accessible image")
    );
}

#[test]
fn image_fallback_preserves_every_optional_size_and_empty_placeholder() {
    for status in [
        EuvImageStatus::Loading,
        EuvImageStatus::Failed,
        EuvImageStatus::Loaded,
    ] {
        for (width, height, expected) in [
            (None, None, ""),
            (Some(240), None, "width: 240px;"),
            (None, Some(120), "height: 120px;"),
            (Some(240), Some(120), "width: 240px; height: 120px;"),
        ] {
            let node: VirtualNode = render_image(status, width, height, "");
            assert_eq!(attribute_text(&node, "style").as_deref(), Some(expected));
            assert_eq!(element_children(&node).len(), 1);
        }
    }
}

#[test]
fn image_fallback_css_preserves_geometry_and_positive_lazy_image_box() {
    let loading: String = c_euv_image_placeholder().get_style().replace(' ', "");
    let failed: String = c_euv_image_failed().get_style().replace(' ', "");
    let picture: String = c_euv_image_unavailable_picture()
        .get_style()
        .replace(' ', "");
    assert_eq!(loading, failed);
    for declaration in [
        "position:relative;",
        "display:flex;",
        "align-items:center;",
        "justify-content:center;",
        "width:100%;",
        "box-sizing:border-box;",
    ] {
        assert!(
            loading.contains(declaration),
            "missing {declaration}: {loading}"
        );
    }
    for property in [
        "min-height:",
        "padding:",
        "border:",
        "background:",
        "color:",
        "font-size:",
    ] {
        assert!(loading.contains(property));
    }
    for declaration in [
        "position:absolute;",
        "inset:0px;",
        "width:100%;",
        "height:100%;",
        "opacity:0;",
        "pointer-events:none;",
    ] {
        assert!(
            picture.contains(declaration),
            "missing {declaration}: {picture}"
        );
    }
    for property in [
        "display:",
        "visibility:",
        "animation:",
        "transition:",
        "max-width:",
    ] {
        assert!(!picture.contains(property));
    }
    assert!(!c_euv_image().get_style().contains("position:"));
}
