use super::*;

/// Sets the direction the content inside it is laid out in.
///
/// The reference provides the direction through a locale context, so the
/// provider adds no box of its own. The wrapper here keeps the element
/// out of the layout with `display: contents` while the `dir` attribute
/// still reaches every child through inheritance, which is what the
/// context does in the reference.
///
/// # Arguments
///
/// - `VirtualNode<EuvDirectionProps>` - The props node carrying the
///   direction the content is laid out in.
///
/// # Returns
///
/// - `VirtualNode` - The euv direction virtual DOM tree.
#[component]
pub fn euv_direction(node: VirtualNode<EuvDirectionProps>) -> VirtualNode {
    let EuvDirectionProps { direction }: EuvDirectionProps =
        node.try_get_props().unwrap_or_default();
    let direction_attr: &str = match direction {
        EuvDirection::Ltr => "ltr",
        EuvDirection::Rtl => "rtl",
    };
    let direction_class: fn() -> &'static Css = match direction {
        EuvDirection::Ltr => c_euv_direction_ltr,
        EuvDirection::Rtl => c_euv_direction_rtl,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "dir": direction_attr
            class: c_euv_direction()
            class: direction_class()
            children
        }
    }
}
