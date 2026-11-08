use super::*;

/// Holds a section that can be folded away.
///
/// The collapsible carries the state of the section. The trigger and the
/// content inside it read that same state, so the trigger reports the
/// state to a reader and the content hides itself when the section is
/// closed.
///
/// # Arguments
///
/// - `VirtualNode<EuvCollapsibleProps>` - The props node carrying the
///   state of the section.
///
/// # Returns
///
/// - `VirtualNode` - The euv collapsible virtual DOM tree.
#[component]
pub fn euv_collapsible(node: VirtualNode<EuvCollapsibleProps>) -> VirtualNode {
    let EuvCollapsibleProps { open }: EuvCollapsibleProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "collapsible"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            class: c_euv_collapsible()
            children
        }
    }
}

/// Toggles the section it belongs to.
///
/// The trigger reports the state of the section through `aria-expanded`,
/// so a reader knows whether the content below it is there before they
/// press it.
///
/// # Arguments
///
/// - `VirtualNode<EuvCollapsibleTriggerProps>` - The props node carrying
///   the state of the section.
///
/// # Returns
///
/// - `VirtualNode` - The euv collapsible trigger virtual DOM tree.
#[component]
pub fn euv_collapsible_trigger(node: VirtualNode<EuvCollapsibleTriggerProps>) -> VirtualNode {
    let EuvCollapsibleTriggerProps { open }: EuvCollapsibleTriggerProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            "data-slot": "collapsible-trigger"
            "aria-expanded": if { open.get() } {
                "true"
            } else {
                "false"
            }
            class: c_euv_collapsible_trigger()
            onclick: on_collapsible_toggle(open)
            children
        }
    }
}

/// Holds the body of a collapsible section.
///
/// The body is hidden while the section is closed, so the content it
/// holds is not read out and does not take room on the page.
///
/// # Arguments
///
/// - `VirtualNode<EuvCollapsibleContentProps>` - The props node carrying
///   the state of the section.
///
/// # Returns
///
/// - `VirtualNode` - The euv collapsible content virtual DOM tree.
#[component]
pub fn euv_collapsible_content(node: VirtualNode<EuvCollapsibleContentProps>) -> VirtualNode {
    let EuvCollapsibleContentProps { open }: EuvCollapsibleContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "collapsible-content"
            "data-state": if { open.get() } {
                "open"
            } else {
                "closed"
            }
            class: c_euv_collapsible_content()
            class: if { open.get() } {
                c_euv_collapsible_content_open()
            } else {
                c_euv_collapsible_content_closed()
            }
            children
        }
    }
}

/// Builds the handler that folds the section away or opens it.
///
/// The state is flipped rather than set, so the trigger works in both
/// directions without the caller having to know the current state.
///
/// # Arguments
///
/// - `Signal<bool>` - The state of the section.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The press handler.
fn on_collapsible_toggle(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let next: bool = !open.get();
        open.set(next);
    }))
}
