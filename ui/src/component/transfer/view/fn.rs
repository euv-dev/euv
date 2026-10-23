use super::*;

/// A two-list shuttle that moves rows between a source and a target list.
///
/// Renders a `c_euv_transfer` row holding a `c_euv_transfer_panel` for
/// each list and a `c_euv_transfer_actions` column in the middle. Every
/// panel carries a `c_euv_transfer_header` — the list's title and the
/// number of its rows — above a `c_euv_transfer_list` of
/// `c_euv_transfer_item` rows.
///
/// A marked row carries `c_euv_transfer_item_active` on top of
/// `c_euv_transfer_item`, and its marker box swaps `c_euv_transfer_box`
/// for `c_euv_transfer_box_active` with a "✓" glyph. Clicking a row
/// toggles its value in the `selected` signal; a disabled row carries no
/// click handler at all, so it never toggles.
///
/// The middle column holds two `euv_button` actions: one moves every
/// marked source row to the target, the other moves every marked target
/// row back. The component never changes the two lists itself — it only
/// reports each move through the callbacks, so the caller stays the owner
/// of both lists.
///
/// # Arguments
///
/// - `VirtualNode<EuvTransferProps>` - The props node holding the two lists, the marked rows and the move callbacks.
///
/// # Returns
///
/// - `VirtualNode` - The transfer virtual DOM tree.
#[component]
pub fn euv_transfer(node: VirtualNode<EuvTransferProps>) -> VirtualNode {
    let EuvTransferProps {
        source,
        target,
        selected,
        source_title,
        target_title,
        on_move_to_target,
        on_move_to_source,
    }: EuvTransferProps = node.try_get_props().unwrap_or_default();
    let marked: Vec<String> = selected.get();
    let source_count: usize = source.len();
    let target_count: usize = target.len();
    let source_values: Vec<&'static str> = source
        .iter()
        .map(|item: &EuvTransferItem| item.value)
        .collect();
    let target_values: Vec<&'static str> = target
        .iter()
        .map(|item: &EuvTransferItem| item.value)
        .collect();
    let source_rows: Vec<VirtualNode> = source
        .into_iter()
        .map(|item: EuvTransferItem| {
            transfer_row(item, transfer_is_marked(&marked, item.value), selected)
        })
        .collect();
    let target_rows: Vec<VirtualNode> = target
        .into_iter()
        .map(|item: EuvTransferItem| {
            transfer_row(item, transfer_is_marked(&marked, item.value), selected)
        })
        .collect();
    let move_to_target: Rc<dyn Fn(Event)> =
        transfer_move_to_target(selected, source_values, on_move_to_target);
    let move_to_source: Rc<dyn Fn(Event)> =
        transfer_move_to_source(selected, target_values, on_move_to_source);
    html! {
        div {
            class: c_euv_transfer()
            div {
                class: c_euv_transfer_panel()
                div {
                    class: c_euv_transfer_header()
                    span {
                        source_title
                    }
                    span {
                        source_count
                    }
                }
                div {
                    class: c_euv_transfer_list()
                    source_rows
                }
            }
            div {
                class: c_euv_transfer_actions()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    label: ">"
                    onclick: Some(move_to_target)
                }
                euv_button {
                    variant: EuvButtonVariant::Outline
                    label: "<"
                    onclick: Some(move_to_source)
                }
            }
            div {
                class: c_euv_transfer_panel()
                div {
                    class: c_euv_transfer_header()
                    span {
                        target_title
                    }
                    span {
                        target_count
                    }
                }
                div {
                    class: c_euv_transfer_list()
                    target_rows
                }
            }
        }
    }
}

/// Renders one row of a transfer list.
///
/// The marker box is `c_euv_transfer_box_active` with a "✓" glyph while
/// the row is marked, and the plain `c_euv_transfer_box` otherwise. A
/// disabled row gets no click handler at all, so it can never write into
/// the selection signal.
///
/// # Arguments
///
/// - `EuvTransferItem` - The row this list renders.
/// - `bool` - Whether the row is currently marked.
/// - `Signal<Vec<String>>` - The marked-rows signal owned by the caller.
///
/// # Returns
///
/// - `VirtualNode` - The row virtual DOM tree.
fn transfer_row(
    item: EuvTransferItem,
    is_marked: bool,
    selected: Signal<Vec<String>>,
) -> VirtualNode {
    let disabled: bool = item.disabled;
    let handler: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        transfer_toggle_handler(selected, item.value)
    };
    html! {
        div {
            class: transfer_row_class(is_marked)
            onclick: handler
            span {
                class: if is_marked {
                    c_euv_transfer_box_active()
                } else {
                    c_euv_transfer_box()
                }
                if is_marked {
                    "✓"
                }
            }
            span {
                item.label
            }
        }
    }
}

/// Builds the class list of one transfer row.
///
/// A marked row carries `c_euv_transfer_item_active` on top of
/// `c_euv_transfer_item`, so the class list is assembled by hand rather
/// than through two `class:` attributes — the registry has no separate
/// "unmarked" class, and a second attribute would repeat the base name.
///
/// # Arguments
///
/// - `bool` - Whether the row is currently marked.
///
/// # Returns
///
/// - `String` - The class list of the row.
fn transfer_row_class(is_marked: bool) -> String {
    let base: &str = c_euv_transfer_item().get_name();
    if !is_marked {
        return String::from(base);
    }
    let active: &str = c_euv_transfer_item_active().get_name();
    let mut class_name: String = String::with_capacity(base.len() + 1 + active.len());
    class_name.push_str(base);
    class_name.push(' ');
    class_name.push_str(active);
    class_name
}

/// Whether a row's value is among the marked rows.
///
/// # Arguments
///
/// - `&[String]` - The marked values, read once per render.
/// - `&str` - The value of the row to look for.
///
/// # Returns
///
/// - `bool` - Whether the row is marked.
pub fn transfer_is_marked(marked: &[String], value: &str) -> bool {
    marked.iter().any(|entry: &String| entry.as_str() == value)
}

/// Builds the click handler that marks or unmarks one row.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The marked-rows signal owned by the caller.
/// - `&'static str` - The value of the clicked row.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle click handler.
fn transfer_toggle_handler(
    selected: Signal<Vec<String>>,
    value: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        transfer_toggle_value(selected, value);
    }))
}

/// Marks a row in the marked-rows signal, or unmarks it when it is marked.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The marked-rows signal owned by the caller.
/// - `&'static str` - The value of the toggled row.
pub fn transfer_toggle_value(selected: Signal<Vec<String>>, value: &'static str) {
    let mut marked: Vec<String> = selected.get();
    if let Some(index) = marked.iter().position(|entry: &String| entry == value) {
        marked.remove(index);
    } else {
        marked.push(String::from(value));
    }
    selected.set(marked);
}

/// Builds the click handler that moves the marked source rows to the
/// target.
///
/// The handler reports every marked row of the source list through the
/// callback, in the order the rows are declared, and never touches the
/// two lists itself.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The marked-rows signal owned by the caller.
/// - `Vec<&'static str>` - The values of the source list's rows.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The move callback.
///
/// # Returns
///
/// - `Rc<dyn Fn(E)>` - The move click handler.
pub fn transfer_move_to_target<E: 'static>(
    selected: Signal<Vec<String>>,
    source_values: Vec<&'static str>,
    on_move: Option<Rc<dyn Fn(&'static str)>>,
) -> Rc<dyn Fn(E)> {
    Rc::new(move |_: E| {
        transfer_report_moves(selected, &source_values, &on_move);
    })
}

/// Builds the click handler that moves the marked target rows back to the
/// source.
///
/// The handler reports every marked row of the target list through the
/// callback, in the order the rows are declared, and never touches the
/// two lists itself.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The marked-rows signal owned by the caller.
/// - `Vec<&'static str>` - The values of the target list's rows.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The move callback.
///
/// # Returns
///
/// - `Rc<dyn Fn(E)>` - The move click handler.
pub fn transfer_move_to_source<E: 'static>(
    selected: Signal<Vec<String>>,
    target_values: Vec<&'static str>,
    on_move: Option<Rc<dyn Fn(&'static str)>>,
) -> Rc<dyn Fn(E)> {
    Rc::new(move |_: E| {
        transfer_report_moves(selected, &target_values, &on_move);
    })
}

/// Reports every marked row of one list through the move callback.
///
/// The rows are reported in the order the list declares them, and only
/// the rows that are actually marked are reported.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The marked-rows signal owned by the caller.
/// - `&[&'static str]` - The values of the list's rows.
/// - `&Option<Rc<dyn Fn(&'static str)>>` - The move callback.
pub fn transfer_report_moves(
    selected: Signal<Vec<String>>,
    values: &[&'static str],
    on_move: &Option<Rc<dyn Fn(&'static str)>>,
) {
    let marked: Vec<String> = selected.get();
    for value in values.iter() {
        if transfer_is_marked(&marked, value)
            && let Some(callback) = on_move
        {
            callback(value);
        }
    }
}
