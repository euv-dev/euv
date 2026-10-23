use super::*;

/// A block that sticks to the top of its scroll container.
///
/// Renders a `c_euv_affix` block holding the caller's content. The class
/// carries the sticky positioning; the distance from the top of the scroll
/// container is written inline as `top: {offset}px;`, so an offset of zero
/// sticks the block to the very top of its container.
///
/// # Arguments
///
/// - `VirtualNode<EuvAffixProps>` - The props node containing the offset.
///
/// # Returns
///
/// - `VirtualNode` - The affix virtual DOM tree.
#[component]
pub fn euv_affix(node: VirtualNode<EuvAffixProps>) -> VirtualNode {
    let EuvAffixProps { offset }: EuvAffixProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let style: String = format!("top: {offset}px;");
    let style_ref: &str = style.as_str();
    html! {
        div {
            class: c_euv_affix()
            style: style_ref
            children
        }
    }
}
