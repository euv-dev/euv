use super::*;

/// Holds a run of bubbles.
///
/// The group is a column that keeps every bubble in the run on one
/// side, so a conversation reads as a single column.
///
/// # Arguments
///
/// - `VirtualNode<EuvBubbleGroupProps>` - The props node carrying the
///   settings of the group.
///
/// # Returns
///
/// - `VirtualNode` - The euv bubble group virtual DOM tree.
#[component]
pub fn euv_bubble_group(node: VirtualNode<EuvBubbleGroupProps>) -> VirtualNode {
    let EuvBubbleGroupProps {}: EuvBubbleGroupProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "bubble-group"
            class: c_euv_bubble_group()
            children
        }
    }
}

/// Holds one message bubble.
///
/// The variant decides the surface the bubble is drawn on, and the align
/// decides the side of the row it sits on.
///
/// # Arguments
///
/// - `VirtualNode<EuvBubbleProps>` - The props node carrying the variant
///   and the alignment of the bubble.
///
/// # Returns
///
/// - `VirtualNode` - The euv bubble virtual DOM tree.
#[component]
pub fn euv_bubble(node: VirtualNode<EuvBubbleProps>) -> VirtualNode {
    let EuvBubbleProps { variant, align }: EuvBubbleProps =
        node.try_get_props().unwrap_or_default();
    let variant_attr: &str = match variant {
        EuvBubbleVariant::Default => "default",
        EuvBubbleVariant::Secondary => "secondary",
        EuvBubbleVariant::Muted => "muted",
        EuvBubbleVariant::Tinted => "tinted",
        EuvBubbleVariant::Outline => "outline",
        EuvBubbleVariant::Ghost => "ghost",
        EuvBubbleVariant::Destructive => "destructive",
    };
    let variant_class: fn() -> &'static Css = match variant {
        EuvBubbleVariant::Default => c_euv_bubble_default,
        EuvBubbleVariant::Secondary => c_euv_bubble_secondary,
        EuvBubbleVariant::Muted => c_euv_bubble_muted,
        EuvBubbleVariant::Tinted => c_euv_bubble_tinted,
        EuvBubbleVariant::Outline => c_euv_bubble_outline,
        EuvBubbleVariant::Ghost => c_euv_bubble_ghost,
        EuvBubbleVariant::Destructive => c_euv_bubble_destructive,
    };
    let align_attr: &str = match align {
        EuvBubbleAlign::Start => BUBBLE_ALIGN_START,
        EuvBubbleAlign::End => BUBBLE_ALIGN_END,
    };
    let align_class: fn() -> &'static Css = match align {
        EuvBubbleAlign::Start => c_euv_bubble_align_start,
        EuvBubbleAlign::End => c_euv_bubble_align_end,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "bubble"
            "data-variant": variant_attr
            "data-align": align_attr
            class: c_euv_bubble()
            class: variant_class()
            class: align_class()
            children
        }
    }
}

/// Holds the text of a bubble.
///
/// The content wraps rather than pushing the bubble wider than the row
/// it sits in.
///
/// # Arguments
///
/// - `VirtualNode<EuvBubbleContentProps>` - The props node carrying the
///   settings of the content.
///
/// # Returns
///
/// - `VirtualNode` - The euv bubble content virtual DOM tree.
#[component]
pub fn euv_bubble_content(node: VirtualNode<EuvBubbleContentProps>) -> VirtualNode {
    let EuvBubbleContentProps {}: EuvBubbleContentProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_bubble_content();

    html! {
            div {
                "data-slot": "bubble-content"
                class: base_class

                children
            }
    }
}

/// Holds the reactions that belong to a bubble.
///
/// The reactions sit beside the bubble, above it or below it, and line
/// up with the side of the row the bubble sits on.
///
/// # Arguments
///
/// - `VirtualNode<EuvBubbleReactionsProps>` - The props node carrying
///   the side of the bubble and the alignment of the reactions.
///
/// # Returns
///
/// - `VirtualNode` - The euv bubble reactions virtual DOM tree.
#[component]
pub fn euv_bubble_reactions(node: VirtualNode<EuvBubbleReactionsProps>) -> VirtualNode {
    let EuvBubbleReactionsProps { side, align }: EuvBubbleReactionsProps =
        node.try_get_props().unwrap_or_default();
    let side_attr: &str = match side {
        EuvBubbleSide::Top => "top",
        EuvBubbleSide::Bottom => "bottom",
    };
    let side_class: fn() -> &'static Css = match side {
        EuvBubbleSide::Top => c_euv_bubble_reactions_top,
        EuvBubbleSide::Bottom => c_euv_bubble_reactions_bottom,
    };
    let align_attr: &str = match align {
        EuvBubbleAlign::Start => BUBBLE_ALIGN_START,
        EuvBubbleAlign::End => BUBBLE_ALIGN_END,
    };
    let align_class: fn() -> &'static Css = match align {
        EuvBubbleAlign::Start => c_euv_bubble_reactions_start,
        EuvBubbleAlign::End => c_euv_bubble_reactions_end,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
            div {
                "data-slot": "bubble-reactions"
                "data-side": side_attr
                "data-align": align_attr
                class: c_euv_bubble_reactions()
                class: side_class()
                class: align_class()
                children
            }
    }
}
