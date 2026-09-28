use super::*;

/// A generic accordion aligned with common component libraries.
///
/// Renders one header button plus one body per entry. Sections are toggled
/// through the caller-owned `open_keys` signal: clicking a header adds or
/// removes its key, and when `allow_multiple` is `false` the list is
/// narrowed so at most one section stays open.
///
/// The bodies are always mounted and their open/closed state is expressed
/// with reactive classes. A reactive `if` around the section list would be
/// the obvious alternative, but the reactive branch borrows the item list
/// for the duration of the closure while `VirtualNode` is not `Copy`, so
/// the class toggle is the shape that stays inside the borrow rules.
///
/// # Arguments
///
/// - `VirtualNode<EuvCollapseProps>` - The props node containing items,
///   open_keys and allow_multiple.
///
/// # Returns
///
/// - `VirtualNode` - The collapse virtual DOM tree.
#[component]
pub fn euv_collapse(node: VirtualNode<EuvCollapseProps>) -> VirtualNode {
    let EuvCollapseProps {
        items,
        open_keys,
        allow_multiple,
    }: EuvCollapseProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let mut sections: Vec<VirtualNode> = Vec::with_capacity(items.len() + 1);
    sections.extend(
        items
            .into_iter()
            .map(|item: EuvCollapseItem| collapse_section(open_keys, allow_multiple, item)),
    );
    sections.push(children);
    sections.into()
}

/// Renders one accordion section: a header button and its body.
///
/// The open state is read inline inside both reactive `class:` conditionals
/// rather than hoisted into a local, so each class subscribes to
/// `open_keys` on its own and header and body never disagree.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The open-section keys signal.
/// - `bool` - Whether more than one section may stay open.
/// - `EuvCollapseItem` - The section entry to render.
///
/// # Returns
///
/// - `VirtualNode` - The section virtual DOM tree.
fn collapse_section(
    open_keys: Signal<Vec<String>>,
    allow_multiple: bool,
    item: EuvCollapseItem,
) -> VirtualNode {
    let key: &'static str = item.key;
    html! {
        div {
            key: key
            class: c_euv_collapse_item()
            button {
                class: c_euv_collapse_header()
                class: if { open_keys.get().iter().any(|entry: &String| entry == key) } {
                    c_euv_collapse_header_active()
                } else {
                    c_euv_collapse_header()
                }
                onclick: on_collapse_toggle(open_keys, key, allow_multiple)
                {
                    item.title
                }
            }
            div {
                class: c_euv_collapse_body()
                class: if { open_keys.get().iter().any(|entry: &String| entry == key) } {
                    c_euv_collapse_body_open()
                } else {
                    c_euv_collapse_body_closed()
                }
            }
        }
    }
}

/// Toggles the clicked section, honouring the `allow_multiple` mode.
///
/// When `allow_multiple` is `false` the list is narrowed to at most one
/// key: an already-open section collapses to the empty list, and a closed
/// one replaces the whole list. Otherwise the key is added or removed in
/// place. The list is read and written back through the signal rather than
/// captured by the handler, so no `Vec` borrow ever escapes into a render
/// closure.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The open-section keys signal.
/// - `&'static str` - The clicked section key.
/// - `bool` - Whether more than one section may stay open.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn on_collapse_toggle(
    open_keys: Signal<Vec<String>>,
    key: &'static str,
    allow_multiple: bool,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let mut keys: Vec<String> = open_keys.get();
        let is_open: bool = keys.iter().any(|entry: &String| entry == key);
        if allow_multiple {
            if let Some(index) = keys.iter().position(|entry: &String| entry == key) {
                keys.remove(index);
            } else {
                keys.push(key.to_string());
            }
        } else {
            keys = if is_open {
                Vec::new()
            } else {
                vec![key.to_string()]
            };
        }
        open_keys.set(keys);
    }))
}
