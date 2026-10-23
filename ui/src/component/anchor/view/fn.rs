use super::*;

/// An anchor list, for jumping between sections of a long page.
///
/// Renders a `c_euv_anchor` column holding the entries in order. The
/// entry whose value matches the `active` signal carries
/// `c_euv_anchor_link_active`; the rest carry `c_euv_anchor_link`.
///
/// # Arguments
///
/// - `VirtualNode<EuvAnchorProps>` - The props node containing the
///   anchor configuration.
///
/// # Returns
///
/// - `VirtualNode` - The anchor list virtual DOM tree.
#[component]
pub fn euv_anchor(node: VirtualNode<EuvAnchorProps>) -> VirtualNode {
    let EuvAnchorProps {
        items,
        title,
        active,
        on_select,
    }: EuvAnchorProps = node.try_get_props().unwrap_or_default();
    let current: String = active.get();
    let entries: Vec<VirtualNode> = items
        .into_iter()
        .map(|item: EuvAnchorItem| {
            let is_active: bool = current == item.value;
            html! {
                a {
                    class: if is_active {
                        c_euv_anchor_link_active()
                    } else {
                        c_euv_anchor_link()
                    }
                    href: "#"
                    onclick: on_anchor_click(on_select.clone(), item.value)
                    item.label
                }
            }
        })
        .collect();
    html! {
        nav {
            class: c_euv_anchor()
            if !title.is_empty() {
                div {
                    class: c_euv_anchor_title()
                    title
                }
            }
            entries
        }
    }
}

/// Builds the handler that reports the chosen anchor.
///
/// # Arguments
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select callback.
/// - `&'static str` - The value belonging to the entry that owns the
///   handler.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn on_anchor_click(
    on_select: Option<Rc<dyn Fn(&'static str)>>,
    value: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        event.prevent_default();
        if let Some(callback) = &on_select {
            callback(value);
        }
    }))
}
