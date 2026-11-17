use super::*;

/// A badge component for displaying status indicators.
///
/// The six variants cover filled, outlined, ghost and link-styled badges. The
/// historical `outline` flag still works: when it is set the badge renders the
/// outline variant whatever the `variant` prop says.
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
        class: custom_class,
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
        EuvBadgeVariant::Ghost => c_euv_badge_ghost,
        EuvBadgeVariant::Link => c_euv_badge_link,
    };
    let data_variant: &str = match effective {
        EuvBadgeVariant::Default => BADGE_VARIANT_DEFAULT,
        EuvBadgeVariant::Secondary => BADGE_VARIANT_SECONDARY,
        EuvBadgeVariant::Destructive => BADGE_VARIANT_DESTRUCTIVE,
        EuvBadgeVariant::Outline => BADGE_VARIANT_OUTLINE,
        EuvBadgeVariant::Ghost => BADGE_VARIANT_GHOST,
        EuvBadgeVariant::Link => BADGE_VARIANT_LINK,
    };
    let base_class: &Css = c_euv_badge();
    let variant_style: &Css = variant_class();
    match on_click {
        Some(click_handler) => html! {
            button {
                type: "button"
                "data-slot": "badge"
                "data-variant": data_variant
                class: base_class
                class: variant_style
                class: c_euv_badge_clickable()
                class: custom_class
                onclick: Some(click_handler)
                text.clone()
            }
        },
        None => html! {
            span {
                "data-slot": "badge"
                "data-variant": data_variant
                class: base_class
                class: variant_style
                class: custom_class
                text.clone()
            }
        },
    }
}
