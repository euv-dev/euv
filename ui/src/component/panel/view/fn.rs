use super::*;

/// A bordered section container wrapping children with a titled header.
///
/// Renders a `<section>` carrying the base `c_euv_panel` class plus
/// `c_euv_panel_bordered` or `c_euv_panel_dashed` for the matching variant.
/// The header holds an `h3` title and an optional subtitle (skipped when
/// `subtitle` is empty); the children become the body.
///
/// # Arguments
///
/// - `VirtualNode<EuvPanelProps>` - The props node carrying the component configuration.
///
/// # Returns
///
/// - `VirtualNode` - The component virtual DOM tree.
#[component]
pub fn euv_panel(node: VirtualNode<EuvPanelProps>) -> VirtualNode {
    let EuvPanelProps {
        title,
        subtitle,
        variant,
    }: EuvPanelProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let variant_class: &str = match variant {
        EuvPanelVariant::Plain => c_euv_panel_plain().get_name(),
        EuvPanelVariant::Bordered => c_euv_panel_bordered().get_name(),
        EuvPanelVariant::Dashed => c_euv_panel_dashed().get_name(),
    };
    let mut class_name: String =
        String::with_capacity(c_euv_panel().get_name().len() + 1 + variant_class.len());
    class_name.push_str(c_euv_panel().get_name());
    class_name.push(' ');
    class_name.push_str(variant_class);
    html! {
        section {
            class: class_name
            div {
                class: c_euv_panel_header()
                h3 {
                    class: c_euv_panel_title()
                    {
                        title
                    }
                }
                if { !subtitle.is_empty() } {
                    p {
                        class: c_euv_panel_subtitle()
                        {
                            subtitle
                        }
                    }
                }
            }
            div {
                class: c_euv_panel_body()
                children
            }
        }
    }
}
