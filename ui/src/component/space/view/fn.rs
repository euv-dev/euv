use super::*;

/// A layout gap primitive separating sibling blocks without a border.
///
/// Renders a `<div>` carrying the base `c_euv_space` class plus an inline
/// style setting either `margin-top` (default) or `margin-left` when the
/// `vertical` signal is true. The gap length is a `var!` reference to the
/// spacing design token selected by `size`, never a hardcoded pixel value.
///
/// # Arguments
///
/// - `VirtualNode<EuvSpaceProps>` - The props node carrying the component configuration.
///
/// # Returns
///
/// - `VirtualNode` - The component virtual DOM tree.
#[component]
pub fn euv_space(node: VirtualNode<EuvSpaceProps>) -> VirtualNode {
    let EuvSpaceProps { size, vertical }: EuvSpaceProps = node.try_get_props().unwrap_or_default();
    let token: &str = match size {
        EuvSpaceSize::Xs => var!("space-xs"),
        EuvSpaceSize::Sm => var!("space-sm"),
        EuvSpaceSize::Md => var!("space-md"),
        EuvSpaceSize::Lg => var!("space-lg"),
        EuvSpaceSize::Xl => var!("space-xl"),
    };
    let style: String = if vertical.get() {
        format!("margin-top: {token};")
    } else {
        format!("margin-left: {token};")
    };
    let style_ref: &str = style.as_str();
    html! {
        div {
            class: c_euv_space()
            style: style_ref
        }
    }
}
