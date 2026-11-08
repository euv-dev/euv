use super::*;

/// A box that holds a fixed shape.
///
/// The ratio is handed to the CSS `aspect-ratio` property, so the box
/// grows and shrinks with its column while the content inside keeps the
/// proportions it was given.
///
/// A ratio of zero or less would collapse the box to nothing, so the
/// component falls back to a square.
///
/// # Arguments
///
/// - `VirtualNode<EuvAspectRatioProps>` - The props node carrying the
///   ratio.
///
/// # Returns
///
/// - `VirtualNode` - The aspect-ratio virtual DOM tree.
#[component]
pub fn euv_aspect_ratio(node: VirtualNode<EuvAspectRatioProps>) -> VirtualNode {
    let EuvAspectRatioProps { ratio }: EuvAspectRatioProps =
        node.try_get_props().unwrap_or_default();
    let effective_ratio: f32 = if ratio > 0.0 { ratio } else { 1.0 };
    let style: String = format!("aspect-ratio: {effective_ratio};");
    let style_ref: &str = style.as_str();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_aspect_ratio()
            "data-slot": "aspect-ratio"
            style: style_ref
            children
        }
    }
}
