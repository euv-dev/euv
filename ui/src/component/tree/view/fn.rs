use super::*;

/// A hierarchical tree with selectable rows and collapsible branches.
///
/// Renders a `c_euv_tree` column holding one `c_euv_tree_node` per node.
/// Each node holds a clickable row — `c_euv_tree_node_row`, or
/// `c_euv_tree_node_row_active` for the selected node — carrying a toggle
/// marker and a `c_euv_tree_node_label`. The marker is
/// `c_euv_tree_node_toggle` while collapsed, `c_euv_tree_node_toggle_open`
/// while expanded, and `c_euv_tree_node_toggle_leaf` for a node without
/// children, which renders it invisible so the labels stay aligned.
///
/// The children of a branch stay mounted: the branch container carries
/// `c_euv_tree_node_children` plus `c_euv_tree_node_children_open` or
/// `c_euv_tree_node_children_closed`, so collapsing a branch is a class
/// swap and never a reactive `if` that would unmount the subtree.
///
/// The selection is read once per render and handed down the recursion,
/// and the expansion set is read once per render as well, so a deep tree
/// never re-reads a signal per node.
///
/// # Arguments
///
/// - `VirtualNode<EuvTreeProps>` - The props node containing the nodes, the selection and the expansion seed.
///
/// # Returns
///
/// - `VirtualNode` - The tree virtual DOM tree.
#[component]
pub fn euv_tree(node: VirtualNode<EuvTreeProps>) -> VirtualNode {
    let EuvTreeProps {
        nodes,
        selected,
        on_select,
        default_expanded,
    }: EuvTreeProps = node.try_get_props().unwrap_or_default();
    let expanded: Signal<Vec<String>> = use_euv_tree_expanded(nodes.as_slice(), default_expanded);
    let selected_value: String = selected.get();
    let expanded_values: Vec<String> = expanded.get();
    let tree_nodes: Vec<VirtualNode> = nodes
        .into_iter()
        .map(|tree_node: EuvTreeNode| {
            tree_node_view(
                tree_node,
                selected,
                &selected_value,
                expanded,
                &expanded_values,
                on_select.clone(),
            )
        })
        .collect();
    html! {
        div {
            class: c_euv_tree()
            tree_nodes
        }
    }
}

/// Obtains the expansion state handle registered against the current hook
/// context slot.
///
/// Behaves like [`HookContext::use_hook`] — the same handle is returned on
/// every render at the same hook index, so the expanded set survives
/// re-renders. The seed is built once, when the slot is first written:
/// an empty set when the tree starts collapsed, and the value of every
/// node in the tree when it starts expanded.
///
/// # Arguments
///
/// - `&[EuvTreeNode]` - The root nodes used to seed the expanded set.
/// - `bool` - Whether the tree starts with every node expanded.
///
/// # Returns
///
/// - `Signal<Vec<String>>` - The signal holding the expanded node values.
pub fn use_euv_tree_expanded(nodes: &[EuvTreeNode], default_expanded: bool) -> Signal<Vec<String>> {
    let seed: Vec<String> = if default_expanded {
        tree_node_values(nodes)
    } else {
        Vec::new()
    };
    HookContext::use_hook(move || Signal::create(seed))
}

/// Collects the value of every node in a tree, depth first.
///
/// # Arguments
///
/// - `&[EuvTreeNode]` - The nodes to walk.
///
/// # Returns
///
/// - `Vec<String>` - The value of every node, parents before children.
fn tree_node_values(nodes: &[EuvTreeNode]) -> Vec<String> {
    let mut values: Vec<String> = Vec::with_capacity(nodes.len());
    collect_tree_node_values(nodes, &mut values);
    values
}

/// Appends the value of every node in a tree to an accumulator.
///
/// # Arguments
///
/// - `&[EuvTreeNode]` - The nodes to walk.
/// - `&mut Vec<String>` - The accumulator the node values are appended to.
fn collect_tree_node_values(nodes: &[EuvTreeNode], values: &mut Vec<String>) {
    for node in nodes {
        values.push(node.value.to_string());
        collect_tree_node_values(&node.children, values);
    }
}

/// Renders one node: its row, and the container holding its children.
///
/// The child list is built with a single `.map(...).collect()` before the
/// `html!` block: a `for` loop inside `html!` over a local vector fails
/// the `'static` bound the reactive attribute closure needs.
///
/// # Arguments
///
/// - `EuvTreeNode` - The node to render, consumed.
/// - `Signal<String>` - The selected-value signal.
/// - `&str` - The selected value, read once per render.
/// - `Signal<Vec<String>>` - The expanded-values signal.
/// - `&[String]` - The expanded values, read once per render.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select callback.
///
/// # Returns
///
/// - `VirtualNode` - The tree node virtual DOM tree.
fn tree_node_view(
    node: EuvTreeNode,
    selected: Signal<String>,
    selected_value: &str,
    expanded: Signal<Vec<String>>,
    expanded_values: &[String],
    on_select: Option<Rc<dyn Fn(&'static str)>>,
) -> VirtualNode {
    let value: &'static str = node.value;
    let label: &'static str = node.label;
    let has_children: bool = !node.children.is_empty();
    let is_selected: bool = value == selected_value;
    let is_expanded: bool = expanded_values.iter().any(|entry: &String| entry == value);
    let child_nodes: Vec<VirtualNode> = node
        .children
        .into_iter()
        .map(|child: EuvTreeNode| {
            tree_node_view(
                child,
                selected,
                selected_value,
                expanded,
                expanded_values,
                on_select.clone(),
            )
        })
        .collect();
    html! {
        div {
            key: value
            class: c_euv_tree_node()
            div {
                class: if is_selected {
                    c_euv_tree_node_row_active()
                } else {
                    c_euv_tree_node_row()
                }
                onclick: on_tree_select(selected, on_select.clone(), value)
                span {
                    class: if !has_children {
                        c_euv_tree_node_toggle_leaf()
                    } else if is_expanded {
                        c_euv_tree_node_toggle_open()
                    } else {
                        c_euv_tree_node_toggle()
                    }
                    onclick: on_tree_toggle(expanded, value)
                    "▸"
                }
                span {
                    class: c_euv_tree_node_label()
                    label
                }
            }
            if has_children {
                div {
                    class: c_euv_tree_node_children()
                    class: if is_expanded {
                        c_euv_tree_node_children_open()
                    } else {
                        c_euv_tree_node_children_closed()
                    }
                    child_nodes
                }
            }
        }
    }
}

/// Selects a node and reports the selection to the caller.
///
/// The node's value is written into the caller-owned `selected` signal and
/// the `on_select` callback is called with it, so a caller can react to
/// the selection without subscribing to the signal itself.
///
/// # Arguments
///
/// - `Signal<String>` - The selected-value signal.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select callback.
/// - `&'static str` - The selected node's value.
pub fn select_tree_node(
    selected: Signal<String>,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
    value: &'static str,
) {
    selected.set(value.to_string());
    if let Some(callback) = &on_select {
        callback(value);
    }
}

/// Adds a node's value to the expanded set, or removes it when it is there.
///
/// # Arguments
///
/// - `&mut Vec<String>` - The expanded values, mutated in place.
/// - `&str` - The toggled node's value.
pub fn toggle_expanded_value(values: &mut Vec<String>, value: &str) {
    if let Some(index) = values.iter().position(|entry: &String| entry == value) {
        values.remove(index);
    } else {
        values.push(value.to_string());
    }
}

/// Builds the click handler that selects a node.
///
/// # Arguments
///
/// - `Signal<String>` - The selected-value signal.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select callback.
/// - `&'static str` - The clicked node's value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn on_tree_select(
    selected: Signal<String>,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
    value: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        select_tree_node(selected, on_select.clone(), value);
    }))
}

/// Builds the click handler that expands or collapses a branch.
///
/// The event is stopped so the click does not bubble into the enclosing
/// row, whose handler would otherwise select the node as well: the toggle
/// only ever folds a branch, it never changes the selection.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The expanded-values signal.
/// - `&'static str` - The toggled node's value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn on_tree_toggle(expanded: Signal<Vec<String>>, value: &'static str) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        event.stop_propagation();
        let mut values: Vec<String> = expanded.get();
        toggle_expanded_value(&mut values, value);
        expanded.set(values);
    }))
}
