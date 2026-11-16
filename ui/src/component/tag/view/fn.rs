use super::*;

/// A tag component for displaying semantic status indicators.
///
/// Supports solid and outline variants across black and white colour types.
///
/// # Arguments
///
/// - `VirtualNode<EuvTagProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - A styled span tag element.
#[component]
pub fn euv_tag(node: VirtualNode<EuvTagProps>) -> VirtualNode {
    let EuvTagProps {
        color,
        variant,
        text,
        on_click,
    }: EuvTagProps = node.try_get_props().unwrap_or_default();
    match variant {
        EuvTagVariant::Solid => match color {
            EuvTagColor::Black => html! {
                span {
                    class: c_euv_tag_solid_black()
                    onclick: on_click
                    text.clone()
                }
            },
            EuvTagColor::White => html! {
                span {
                    class: c_euv_tag_solid_white()
                    onclick: on_click
                    text.clone()
                }
            },
        },
        EuvTagVariant::Outline => match color {
            EuvTagColor::Black => html! {
                span {
                    class: c_euv_tag_outline_black()
                    onclick: on_click
                    text.clone()
                }
            },
            EuvTagColor::White => html! {
                span {
                    class: c_euv_tag_outline_white()
                    onclick: on_click
                    text.clone()
                }
            },
        },
    }
}
