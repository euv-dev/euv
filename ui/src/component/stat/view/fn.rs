use super::*;

/// A single metric tile showing one large value with its label.
///
/// Renders a `c_euv_stat` block: an optional `c_euv_stat_icon` glyph, the
/// `c_euv_stat_value` number in the accent-weighted monospace style, the
/// `c_euv_stat_label` name in muted styling, and the `c_euv_stat_hint`
/// secondary note. The icon and the hint are each skipped when their
/// prop is empty, so a plain label/value tile stays free of empty rows.
///
/// # Arguments
///
/// - `VirtualNode<EuvStatProps>` - The props node containing label, value, hint and icon.
///
/// # Returns
///
/// - `VirtualNode` - The stat tile virtual DOM tree.
#[component]
pub fn euv_stat(node: VirtualNode<EuvStatProps>) -> VirtualNode {
    let EuvStatProps {
        label,
        value,
        hint,
        icon,
    }: EuvStatProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_euv_stat()
            if { !icon.is_empty() } {
                span {
                    class: c_euv_stat_icon()
                    {
                        icon
                    }
                }
            }
            span {
                class: c_euv_stat_value()
                {
                    value
                }
            }
            span {
                class: c_euv_stat_label()
                {
                    label
                }
            }
            if { !hint.is_empty() } {
                span {
                    class: c_euv_stat_hint()
                    {
                        hint
                    }
                }
            }
        }
    }
}
