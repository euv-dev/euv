pub(super) const CONSUMER_SOURCE: &str = r#"use euv::*;
use euv_ui::*;

#[test]
fn dependency_component_is_a_real_button() {
    let node: VirtualNode = html! {
        euv_button {
            size: EuvButtonSize::Icon
            label: "dependency discovery"
        }
    };
    assert!(matches!(node, VirtualNode::Element { tag: Tag::Element(ref tag), .. } if tag == "button"));
}
"#;

pub(super) const DEPENDENCY_CASES: [(&str, &str, bool); 6] = [
    ("normal", "dependencies", false),
    ("development", "dev-dependencies", false),
    (
        "target_normal",
        "target.'cfg(not(target_arch = \"wasm32\"))'.dependencies",
        false,
    ),
    (
        "target_development",
        "target.'cfg(not(target_arch = \"wasm32\"))'.dev-dependencies",
        false,
    ),
    ("inherited", "dependencies", true),
    (
        "target_inherited",
        "target.'cfg(not(target_arch = \"wasm32\"))'.dependencies",
        true,
    ),
];
