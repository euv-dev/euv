use super::*;

/// A select whose option panel is a tree.
///
/// Renders a `c_euv_tree_select` wrapper holding the optional group label,
/// a `c_euv_tree_select_trigger` button that shows the selected node's
/// label in `c_euv_tree_select_value` — or the placeholder in
/// `c_euv_tree_select_placeholder` while no node matches — next to a
/// trailing `c_euv_tree_select_caret` marker, and the
/// `c_euv_tree_select_panel` the tree lives in.
///
/// The panel is always mounted: opening and closing it swaps the
/// `c_euv_tree_select_panel_open` and `c_euv_tree_select_panel_closed`
/// classes rather than removing the tree from the tree, so the nodes
/// survive every re-render.
///
/// The tree itself is the [`euv_tree`] component, so the panel reuses the
/// tree classes — `c_euv_tree`, `c_euv_tree_node`, `c_euv_tree_node_row`,
/// `c_euv_tree_node_row_active`, the toggle markers and the children
/// containers — instead of carrying a second set of them. The children of
/// a node stay mounted; expanding a branch is a class swap.
///
/// Choosing a node writes its value into the `value` signal, reports it to
/// the `on_select` callback and closes the panel. The open state is owned
/// by the component itself through a hook, so the caller never has to
/// wire it up.
///
/// # Arguments
///
/// - `VirtualNode<EuvTreeSelectProps>` - The props node holding the tree, the value signal and the select callback.
///
/// # Returns
///
/// - `VirtualNode` - The tree select virtual DOM tree.
#[component]
pub fn euv_tree_select(node: VirtualNode<EuvTreeSelectProps>) -> VirtualNode {
    let EuvTreeSelectProps {
        nodes,
        value,
        placeholder,
        label: label_text,
        on_select,
    }: EuvTreeSelectProps = node.try_get_props().unwrap_or_default();
    let open: Signal<bool> = HookContext::use_hook(|| Signal::create(false));
    let selected_value: String = value.get();
    let selected_label: Option<&'static str> = tree_select_find_label(&nodes, &selected_value);
    let trigger_text: &'static str = selected_label.unwrap_or(placeholder);
    let has_selection: bool = selected_label.is_some();
    let pick: Rc<dyn Fn(&'static str)> = tree_select_pick_handler(value, on_select, open);
    html! {
        div {
            class: c_euv_tree_select()
            if !label_text.is_empty() {
                div {
                    class: c_form_label()
                    label_text
                }
            }
            button {
                type: "button"
                class: c_euv_tree_select_trigger()
                onclick: tree_select_toggle_handler(open)
                span {
                    class: if has_selection {
                        c_euv_tree_select_value()
                    } else {
                        c_euv_tree_select_placeholder()
                    }
                    trigger_text
                }
                span {
                    class: c_euv_tree_select_caret()
                    "▾"
                }
            }
            div {
                class: c_euv_tree_select_panel()
                class: if { open } {
                    c_euv_tree_select_panel_open()
                } else {
                    c_euv_tree_select_panel_closed()
                }
                euv_tree {
                    nodes: nodes
                    selected: value
                    on_select: Some(pick)
                    default_expanded: true
                }
            }
        }
    }
}

/// Finds the label of the node carrying the given value.
///
/// The search walks the tree depth first, so a nested node is found by
/// its value just like a root one.
///
/// # Arguments
///
/// - `&[EuvTreeNode]` - The nodes to walk.
/// - `&str` - The value of the node to find.
///
/// # Returns
///
/// - `Option<&'static str>` - The label of the matching node, when there is one.
pub fn tree_select_find_label(nodes: &[EuvTreeNode], value: &str) -> Option<&'static str> {
    for node in nodes.iter() {
        if node.value == value {
            return Some(node.label);
        }
        if let Some(found) = tree_select_find_label(&node.children, value) {
            return Some(found);
        }
    }
    None
}

/// Builds the click handler that opens or closes the panel.
///
/// # Arguments
///
/// - `Signal<bool>` - The panel open state owned by the component.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(E)>>` - The toggle click handler.
pub fn tree_select_toggle_handler<E: 'static>(open: Signal<bool>) -> Option<Rc<dyn Fn(E)>> {
    Some(Rc::new(move |_: E| {
        let is_open: bool = open.get();
        open.set(!is_open);
    }))
}

/// Builds the handler that picks a node of the tree.
///
/// # Arguments
///
/// - `Signal<String>` - The value signal owned by the caller.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select callback.
/// - `Signal<bool>` - The panel open state owned by the component.
///
/// # Returns
///
/// - `Rc<dyn Fn(&'static str)>` - The pick handler.
fn tree_select_pick_handler(
    value: Signal<String>,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
    open: Signal<bool>,
) -> Rc<dyn Fn(&'static str)> {
    Rc::new(move |node_value: &'static str| {
        tree_select_pick_node(value, on_select.clone(), open, node_value);
    })
}

/// Picks a node: writes its value, reports it and closes the panel.
///
/// The value lands in the caller's signal first, so the callback and the
/// closing of the panel both see the new selection.
///
/// # Arguments
///
/// - `Signal<String>` - The value signal owned by the caller.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select callback.
/// - `Signal<bool>` - The panel open state owned by the component.
/// - `&'static str` - The value of the picked node.
pub fn tree_select_pick_node(
    value: Signal<String>,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
    open: Signal<bool>,
    node_value: &'static str,
) {
    value.set(String::from(node_value));
    if let Some(callback) = &on_select {
        callback(node_value);
    }
    open.set(false);
}
