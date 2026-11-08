use super::*;

/// A badge component for displaying status indicators.
///
/// The four variants are the ones the registry publishes: the filled
/// default, the quiet secondary, the destructive fill and the bare
/// outline. The historical `outline` flag still works: when it is set the
/// badge renders the outline variant whatever the `variant` prop says.
///
/// # Arguments
///
/// - `VirtualNode<EuvBadgeProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - A styled span badge element.
#[component]
pub fn euv_badge(node: VirtualNode<EuvBadgeProps>) -> VirtualNode {
    let EuvBadgeProps {
        text,
        outline,
        variant,
        on_click,
    }: EuvBadgeProps = node.try_get_props().unwrap_or_default();
    let effective: EuvBadgeVariant = if outline {
        EuvBadgeVariant::Outline
    } else {
        variant
    };
    let variant_class: fn() -> &'static Css = match effective {
        EuvBadgeVariant::Default => c_euv_badge_default,
        EuvBadgeVariant::Secondary => c_euv_badge_secondary,
        EuvBadgeVariant::Destructive => c_euv_badge_destructive,
        EuvBadgeVariant::Outline => c_euv_badge_outline,
    };
    html! {
        span {
            class: c_euv_badge()
            class: variant_class()
            onclick: on_click
            text
        }
    }
}
