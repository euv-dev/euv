use super::*;

/// The frame that holds a hover card.
///
/// The frame owns the open state, so the trigger and the panel always
/// agree on whether the card is showing.
///
/// # Arguments
///
/// - `VirtualNode<EuvHoverCardProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv hover card virtual DOM tree.
#[component]
pub fn euv_hover_card(node: VirtualNode<EuvHoverCardProps>) -> VirtualNode {
    let EuvHoverCardProps { open: _open }: EuvHoverCardProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_hover_card()
            children
        }
    }
}

/// The text the card is anchored to.
///
/// # Arguments
///
/// - `VirtualNode<EuvHoverCardTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv hover card trigger virtual DOM tree.
#[component]
pub fn euv_hover_card_trigger(node: VirtualNode<EuvHoverCardTriggerProps>) -> VirtualNode {
    let EuvHoverCardTriggerProps { open, label }: EuvHoverCardTriggerProps =
        node.try_get_props().unwrap_or_default();
    let on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        open.set(!open.get());
    }));
    html! {
        span {
            class: c_euv_hover_card_trigger()
            onclick: on_click
            {
                label
            }
        }
    }
}

/// The panel of the card.
///
/// The panel is the surface the reader reads while the pointer rests on
/// the trigger, so it carries the popover colours of the design system.
///
/// # Arguments
///
/// - `VirtualNode<EuvHoverCardContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv hover card content virtual DOM tree.
#[component]
pub fn euv_hover_card_content(node: VirtualNode<EuvHoverCardContentProps>) -> VirtualNode {
    let EuvHoverCardContentProps { open }: EuvHoverCardContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "dialog"
            class: c_euv_hover_card_content()
            class: if { open } {
                c_euv_hover_card_content_open()
            } else {
                c_euv_hover_card_content_closed()
            }
            children
        }
    }
}
