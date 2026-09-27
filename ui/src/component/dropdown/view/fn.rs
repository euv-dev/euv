use super::*;

/// A generic dropdown menu aligned with common component libraries.
///
/// The trigger element is supplied as children and toggles the caller-owned
/// `open` signal; the menu lists `items` and closes after a selection. The
/// menu is always mounted and toggled via reactive classes.
///
/// # Arguments
///
/// - `VirtualNode<EuvDropdownProps>` - The props node containing open, items and on_select.
///
/// # Returns
///
/// - `VirtualNode` - The dropdown virtual DOM tree.
#[component]
pub fn euv_dropdown(node: VirtualNode<EuvDropdownProps>) -> VirtualNode {
    let EuvDropdownProps {
        open,
        items,
        on_select,
    }: EuvDropdownProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let menu_items: Vec<VirtualNode> = items
        .into_iter()
        .map(|item: EuvDropdownItem| {
            let item_class: fn() -> &'static Css = if item.active {
                c_euv_dropdown_item_active
            } else {
                c_euv_dropdown_item
            };
            html! {
                button {
                    class: item_class()
                    onclick: select_item(open, on_select.clone(), item.value)
                    {
                        item.label
                    }
                }
            }
        })
        .collect();
    html! {
        div {
            class: c_euv_dropdown()
            children
            div {
                class: c_euv_dropdown_menu()
                class: if { open } {
                    c_euv_dropdown_menu_open()
                } else {
                    c_euv_dropdown_menu_closed()
                }
                menu_items
            }
        }
    }
}

/// Emits the chosen value and closes the menu.
///
/// # Arguments
///
/// - `Signal<bool>` - The open state signal.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select callback.
/// - `&'static str` - The chosen item value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn select_item(
    open: Signal<bool>,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
    value: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_| {
        if let Some(callback) = &on_select {
            callback(value);
        }
        open.set(false);
    }))
}
