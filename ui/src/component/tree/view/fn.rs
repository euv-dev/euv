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
/// Tree items use the ARIA tree pattern and a single visible roving tab stop;
/// arrow keys navigate and expand or collapse branches.
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
    let visible_values: Vec<String> = visible_tree_node_values(nodes.as_slice(), &expanded_values);
    let roving_value: String = if visible_values
        .iter()
        .any(|value: &String| value == &selected_value)
    {
        selected_value.clone()
    } else {
        visible_values.first().cloned().unwrap_or_default()
    };
    let context: TreeViewContext<'_> = TreeViewContext {
        selected,
        selected_value: &selected_value,
        roving_value: &roving_value,
        expanded,
        expanded_values: &expanded_values,
        on_select,
    };
    let tree_nodes: Vec<VirtualNode> = nodes
        .into_iter()
        .map(|tree_node: EuvTreeNode| tree_node_view(tree_node, &context, 1, true))
        .collect();
    html! {
        div {
            class: c_euv_tree()
            "data-slot": TREE_SLOT
            role: "tree"
            "aria-label": TREE_ARIA_LABEL
            "aria-multiselectable": "false"
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

/// Collects values of the nodes visible with the current expanded set.
///
/// # Arguments
///
/// - `&[EuvTreeNode]` - The root nodes to traverse.
/// - `&[String]` - The values of expanded branches.
///
/// # Returns
///
/// - `Vec<String>` - The visible node values in depth-first order.
fn visible_tree_node_values(nodes: &[EuvTreeNode], expanded_values: &[String]) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();
    collect_visible_tree_node_values(nodes, expanded_values, &mut values);
    values
}

/// Appends visible node values to an accumulator.
///
/// # Arguments
///
/// - `&[EuvTreeNode]` - The nodes to traverse.
/// - `&[String]` - The values of expanded branches.
/// - `&mut Vec<String>` - The accumulator for visible node values.
fn collect_visible_tree_node_values(
    nodes: &[EuvTreeNode],
    expanded_values: &[String],
    values: &mut Vec<String>,
) {
    for node in nodes {
        values.push(node.value.to_string());
        let is_expanded: bool = expanded_values
            .iter()
            .any(|expanded_value: &String| expanded_value.as_str() == node.value);
        if is_expanded {
            collect_visible_tree_node_values(&node.children, expanded_values, values);
        }
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
/// - `&TreeViewContext<'_>` - The shared state read once for this render.
/// - `usize` - The one-based nesting level.
/// - `bool` - Whether the node is visible through its expanded ancestors.
///
/// # Returns
///
/// - `VirtualNode` - The tree node virtual DOM tree.
fn tree_node_view(
    node: EuvTreeNode,
    context: &TreeViewContext<'_>,
    level: usize,
    is_visible: bool,
) -> VirtualNode {
    let value: &'static str = node.value;
    let label: &'static str = node.label;
    let has_children: bool = !node.children.is_empty();
    let is_selected: bool = value == context.selected_value;
    let is_expanded: bool = has_children
        && context
            .expanded_values
            .iter()
            .any(|entry: &String| entry == value);
    let is_roving: bool = value == context.roving_value;
    let aria_level: String = level.to_string();
    let tabindex: &str = if is_roving { "0" } else { "-1" };
    let aria_selected: &str = if is_selected {
        TREE_ATTR_TRUE
    } else {
        TREE_ATTR_FALSE
    };
    let aria_expanded: &str = if is_expanded {
        TREE_ATTR_TRUE
    } else {
        TREE_ATTR_FALSE
    };
    let on_keydown: Option<Rc<dyn Fn(Event)>> = tree_item_keydown(
        value,
        level,
        has_children,
        is_expanded,
        context.selected,
        context.expanded,
        context.on_select.clone(),
    );
    let child_nodes: Vec<VirtualNode> = node
        .children
        .into_iter()
        .map(|child: EuvTreeNode| {
            tree_node_view(child, context, level + 1, is_visible && is_expanded)
        })
        .collect();
    html! {
        div {
            key: value
            "data-slot": TREE_ITEM_SLOT
            "data-value": value
            "data-visible": if is_visible {
                TREE_ATTR_TRUE
            } else {
                TREE_ATTR_FALSE
            }
            role: "treeitem"
            "aria-label": label
            "aria-level": aria_level
            "aria-selected": aria_selected
            "aria-expanded": aria_expanded
            tabindex: tabindex
            class: c_euv_tree_node()
            onkeydown: on_keydown
            div {
                "data-slot": TREE_ROW_SLOT
                class: if is_selected {
                    c_euv_tree_node_row_active()
                } else {
                    c_euv_tree_node_row()
                }
                onclick: on_tree_select(context.selected, context.on_select.clone(), value)
                span {
                    "data-slot": TREE_TOGGLE_SLOT
                    "aria-hidden": "true"
                    class: if !has_children {
                        c_euv_tree_node_toggle_leaf()
                    } else if is_expanded {
                        c_euv_tree_node_toggle_open()
                    } else {
                        c_euv_tree_node_toggle()
                    }
                    onclick: on_tree_toggle(context.expanded, value)
                    "▸"
                }
                span {
                    class: c_euv_tree_node_label()
                    label
                }
            }
            if has_children {
                div {
                    "data-slot": TREE_GROUP_SLOT
                    role: TREE_GROUP_ROLE
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

/// Builds the keyboard handler for a focused tree item.
///
/// Arrow keys move through visible items and expand or collapse branches;
/// Home and End move to the visible bounds, while Enter and Space select.
///
/// # Arguments
///
/// - `&'static str` - The item's stable value.
/// - `usize` - The item's one-based nesting level.
/// - `bool` - Whether the item has children.
/// - `bool` - Whether the item is expanded.
/// - `Signal<String>` - The selected-value signal.
/// - `Signal<Vec<String>>` - The expanded-values signal.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select callback.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The keyboard handler.
fn tree_item_keydown(
    value: &'static str,
    level: usize,
    has_children: bool,
    is_expanded: bool,
    selected: Signal<String>,
    expanded: Signal<Vec<String>>,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let key: String = keyboard_event.key();
        let Some(target) = keyboard_event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        let Some(root) = target.closest(TREE_ROOT_SELECTOR).ok().flatten() else {
            return;
        };
        let Some(current) = target
            .closest(TREE_ITEM_SELECTOR)
            .ok()
            .flatten()
            .and_then(|item: Element| item.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };
        let Ok(nodes) = root.query_selector_all(TREE_ITEM_SELECTOR) else {
            return;
        };
        let mut items: Vec<HtmlElement> = Vec::new();
        for index in 0..nodes.length() {
            let Some(node) = nodes.item(index) else {
                continue;
            };
            let Ok(item) = node.dyn_into::<HtmlElement>() else {
                continue;
            };
            if item.get_attribute(TREE_VISIBLE_ATTRIBUTE).as_deref() == Some(TREE_VISIBLE_VALUE) {
                items.push(item);
            }
        }
        let Some(current_index) = items.iter().position(|item: &HtmlElement| item == &current)
        else {
            return;
        };
        if key == TREE_KEY_ARROW_DOWN
            || key == TREE_KEY_ARROW_UP
            || key == TREE_KEY_HOME
            || key == TREE_KEY_END
        {
            let next_index: usize = if key == TREE_KEY_ARROW_DOWN {
                (current_index + 1).min(items.len() - 1)
            } else if key == TREE_KEY_ARROW_UP {
                current_index.saturating_sub(1)
            } else if key == TREE_KEY_HOME {
                0
            } else {
                items.len() - 1
            };
            keyboard_event.prevent_default();
            if next_index != current_index
                && let Some(item) = items.get(next_index)
            {
                focus_tree_item(item);
            }
            return;
        }
        if key == TREE_KEY_ENTER || key == TREE_KEY_SPACE {
            keyboard_event.prevent_default();
            select_tree_node(selected, on_select.clone(), value);
            return;
        }
        if key == TREE_KEY_ARROW_RIGHT && has_children {
            keyboard_event.prevent_default();
            if !is_expanded {
                let mut values: Vec<String> = expanded.get();
                toggle_expanded_value(&mut values, value);
                expanded.set(values);
                return;
            }
            if let Some(item) = items.get(current_index + 1) {
                let next_level: Option<usize> = item
                    .get_attribute("aria-level")
                    .and_then(|value: String| value.parse::<usize>().ok());
                if next_level == Some(level + 1) {
                    focus_tree_item(item);
                }
            }
            return;
        }
        if key == TREE_KEY_ARROW_LEFT {
            if has_children && is_expanded {
                keyboard_event.prevent_default();
                let mut values: Vec<String> = expanded.get();
                toggle_expanded_value(&mut values, value);
                expanded.set(values);
                return;
            }
            let Some(group) = current.closest(TREE_GROUP_SELECTOR).ok().flatten() else {
                return;
            };
            let Some(parent) = group
                .parent_element()
                .and_then(|item: Element| item.dyn_into::<HtmlElement>().ok())
            else {
                return;
            };
            keyboard_event.prevent_default();
            focus_tree_item(&parent);
        }
    }))
}

/// Moves focus to a tree item and selects it through its rendered row.
///
/// # Arguments
///
/// - `&HtmlElement` - The tree item to focus and select.
fn focus_tree_item(item: &HtmlElement) {
    let _: Result<(), JsValue> = item.focus();
    if let Ok(Some(row)) = item.query_selector(TREE_ROW_SELECTOR)
        && let Ok(row) = row.dyn_into::<HtmlElement>()
    {
        row.click();
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
