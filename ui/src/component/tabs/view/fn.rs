use super::*;

/// A generic tab bar with a single panel, aligned with common site
/// frameworks.
///
/// Renders the tab bar from `items` plus a panel holding the children. The
/// selected tab is owned by the caller through `active`, so the selection
/// survives re-renders and can be shared with page-level state. The panel
/// is always mounted: switching the panel content is left to the caller's
/// own `match` / `if` on the same `active` signal.
///
/// # Arguments
///
/// - `VirtualNode<EuvTabsProps>` - The props node carrying the component configuration.
///
/// # Returns
///
/// - `VirtualNode` - The component virtual DOM tree.
#[component]
pub fn euv_tabs(node: VirtualNode<EuvTabsProps>) -> VirtualNode {
    let EuvTabsProps { items, active }: EuvTabsProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    if items.is_empty() {
        return html! {
            div {
                class: c_euv_tabs_panel()
                children
            }
        };
    }
    let tab_buttons: Vec<VirtualNode> = items
        .into_iter()
        .map(|item: EuvTabItem| tab_button(active, item))
        .collect();
    html! {
        div {
            class: c_euv_tabs()
            div {
                class: c_euv_tabs_bar()
                tab_buttons
            }
            div {
                class: c_euv_tabs_panel()
                children
            }
        }
    }
}

/// Renders one tab button, switching between the active and inactive
/// classes through the caller-owned `active` signal.
///
/// # Arguments
///
/// - `Signal<String>` - The selected tab key signal.
/// - `EuvTabItem` - The tab entry to render.
///
/// # Returns
///
/// - `VirtualNode` - The tab button virtual DOM tree.
fn tab_button(active: Signal<String>, item: EuvTabItem) -> VirtualNode {
    let key: &'static str = item.key;
    html! {
        button {
            key: key
            class: c_euv_tab_item()
            class: if { active.get() == key } {
                c_euv_tab_item_active()
            } else {
                c_euv_tab_item()
            }
            onclick: select_tab(active, key)
            {
                item.label
            }
        }
    }
}

/// Makes the clicked tab the active one.
///
/// # Arguments
///
/// - `Signal<String>` - The selected tab key signal.
/// - `&'static str` - The clicked tab key.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn select_tab(active: Signal<String>, key: &'static str) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        active.set(key.to_string());
    }))
}
