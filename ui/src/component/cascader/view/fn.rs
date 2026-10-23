use super::*;

/// The separator between the labels of a chosen path.
const PATH_SEPARATOR: &str = " / ";

/// The glyph drawn on the trailing edge of the trigger.
const CARET_GLYPH: &str = "\u{25be}";

/// A multi-level column picker.
///
/// Renders a `c_euv_cascader` wrapper holding the optional group label, a
/// `c_euv_cascader_trigger` button that shows the labels of the chosen path
/// joined with " / " — or the placeholder while the path is empty — next to
/// a trailing `c_euv_cascader_caret` marker, and the `c_euv_cascader_panel`.
///
/// The panel is always mounted: opening and closing it swaps the
/// `c_euv_cascader_panel_open` and `c_euv_cascader_panel_closed` classes
/// rather than removing the columns from the tree, so the columns survive
/// every re-render.
///
/// The panel body is a `c_euv_cascader_columns` row of `c_euv_cascader_column`s.
/// The first column lists the top-level choices, the second the children of
/// the choice the reader opened, and so on: only the columns the reader has
/// actually opened are rendered. A choice on the path the reader is browsing
/// carries `c_euv_cascader_option_active` as well.
///
/// Picking a choice with children opens the next column and leaves the panel
/// open; picking a choice without children writes the whole path into the
/// caller's signal, reports the choice to `on_select` and closes the panel.
///
/// # Arguments
///
/// - `VirtualNode<EuvCascaderProps>` - The props node holding the choices, the chosen path and the callback.
///
/// # Returns
///
/// - `VirtualNode` - The cascader virtual DOM tree.
#[component]
pub fn euv_cascader(node: VirtualNode<EuvCascaderProps>) -> VirtualNode {
    let EuvCascaderProps {
        options,
        value,
        placeholder,
        label: label_text,
        on_select,
    }: EuvCascaderProps = node.try_get_props().unwrap_or_default();
    let chosen: Vec<String> = value.get();
    let open: Signal<bool> = HookContext::use_hook(|| Signal::create(false));
    let browsed: Signal<Vec<String>> = HookContext::use_hook(|| Signal::create(chosen.clone()));
    let browsed_path: Vec<String> = browsed.get();
    let labels: Vec<&'static str> = cascader_path_labels(&options, &chosen);
    let has_path: bool = !labels.is_empty();
    let trigger_text: String = if has_path {
        labels.join(PATH_SEPARATOR)
    } else {
        String::from(placeholder)
    };
    let columns: Vec<&[EuvCascaderOption]> = cascader_columns(&options, &browsed_path);
    let column_nodes: Vec<VirtualNode> = columns
        .iter()
        .enumerate()
        .map(|entry: (usize, &&[EuvCascaderOption])| {
            let depth: usize = entry.0;
            let column: &&[EuvCascaderOption] = entry.1;
            let active_value: Option<&String> = browsed_path.get(depth);
            let rows: Vec<VirtualNode> = column
                .iter()
                .map(|option: &EuvCascaderOption| {
                    let is_active: bool =
                        active_value.is_some_and(|active: &String| active == option.value);
                    cascader_option_row(
                        option,
                        depth,
                        is_active,
                        value,
                        browsed,
                        open,
                        on_select.clone(),
                    )
                })
                .collect();
            html! {
                div {
                    class: c_euv_cascader_column()
                    rows
                }
            }
        })
        .collect();
    html! {
        div {
            class: c_euv_cascader()
            if !label_text.is_empty() {
                label {
                    class: c_form_label()
                    label_text
                }
            }
            button {
                type: "button"
                class: c_euv_cascader_trigger()
                onclick: cascader_trigger_handler(open, browsed, value)
                span {
                    class: if has_path {
                        c_euv_cascader_value()
                    } else {
                        c_euv_cascader_placeholder()
                    }
                    trigger_text
                }
                span {
                    class: c_euv_cascader_caret()
                    CARET_GLYPH
                }
            }
            div {
                class: c_euv_cascader_panel()
                class: if { open } {
                    c_euv_cascader_panel_open()
                } else {
                    c_euv_cascader_panel_closed()
                }
                div {
                    class: c_euv_cascader_columns()
                    column_nodes
                }
            }
        }
    }
}

/// Returns the labels of a chosen path.
///
/// The walk follows the path through the choice tree and collects the label
/// of every choice on it. A path naming a choice the tree does not hold
/// stops the walk, so a stale value can never produce a half-open path.
///
/// # Arguments
///
/// - `&[EuvCascaderOption]` - The top-level choices.
/// - `&[String]` - The values of the path, outermost first.
///
/// # Returns
///
/// - `Vec<&'static str>` - The label of every choice on the path.
pub fn cascader_path_labels(options: &[EuvCascaderOption], path: &[String]) -> Vec<&'static str> {
    let mut labels: Vec<&'static str> = Vec::with_capacity(path.len());
    let mut level: &[EuvCascaderOption] = options;
    for value in path {
        let Some(choice) = level
            .iter()
            .find(|option: &&EuvCascaderOption| option.value == value.as_str())
        else {
            break;
        };
        labels.push(choice.label);
        level = &choice.children;
    }
    labels
}

/// Returns the columns the panel renders.
///
/// The first column holds the top-level choices; every further column holds
/// the children of the choice the reader opened at the level above it. The
/// walk stops at the first level the path does not name, so only the
/// columns the reader has actually opened are rendered.
///
/// # Arguments
///
/// - `&'a [EuvCascaderOption]` - The top-level choices.
/// - `&[String]` - The values of the path the reader is browsing, outermost first.
///
/// # Returns
///
/// - `Vec<&'a [EuvCascaderOption]>` - The choices of every open column, outermost first.
pub fn cascader_columns<'a>(
    options: &'a [EuvCascaderOption],
    path: &[String],
) -> Vec<&'a [EuvCascaderOption]> {
    let mut columns: Vec<&'a [EuvCascaderOption]> = vec![options];
    let mut level: &'a [EuvCascaderOption] = options;
    for value in path {
        let Some(choice) = level
            .iter()
            .find(|option: &&EuvCascaderOption| option.value == value.as_str())
        else {
            break;
        };
        if choice.children.is_empty() {
            break;
        }
        level = &choice.children;
        columns.push(level);
    }
    columns
}

/// Renders one choice of a column.
///
/// A choice on the path the reader is browsing carries the active class as
/// well as the plain option class, so the column shows where the reader
/// is. The click handler is built per choice, because every choice needs
/// to know its own value and level.
///
/// # Arguments
///
/// - `&EuvCascaderOption` - The choice this row renders.
/// - `usize` - The level of the choice, counting from the top-level column.
/// - `bool` - Whether the choice is on the path the reader is browsing.
/// - `Signal<Vec<String>>` - The chosen path owned by the caller.
/// - `Signal<bool>` - The panel's open state.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select callback.
///
/// # Returns
///
/// - `VirtualNode` - The choice row virtual DOM tree.
fn cascader_option_row(
    option: &EuvCascaderOption,
    depth: usize,
    is_active: bool,
    value: Signal<Vec<String>>,
    browsed: Signal<Vec<String>>,
    open: Signal<bool>,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
) -> VirtualNode {
    let option_value: &'static str = option.value;
    let option_label: &'static str = option.label;
    let has_children: bool = !option.children.is_empty();
    let handler: Option<Rc<dyn Fn(Event)>> = cascader_option_handler(
        value,
        browsed,
        open,
        on_select,
        option_value,
        depth,
        has_children,
    );
    if is_active {
        html! {
            button {
                type: "button"
                class: c_euv_cascader_option()
                class: c_euv_cascader_option_active()
                onclick: handler
                option_label
            }
        }
    } else {
        html! {
            button {
                type: "button"
                class: c_euv_cascader_option()
                onclick: handler
                option_label
            }
        }
    }
}

/// Builds the click handler of one choice.
///
/// The handler puts the choice on the path the reader is browsing, drops
/// the levels below it and opens the next column. A choice without children
/// ends the walk: the whole path is written into the caller's signal, the
/// choice is reported to the callback and the panel is closed.
///
/// The event type is a parameter so the DOM-free test suite can drive the
/// handler with a stand-in event; the component always instantiates it with
/// [`Event`].
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The chosen path owned by the caller.
/// - `Signal<bool>` - The panel's open state.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select callback.
/// - `&'static str` - The value of the clicked choice.
/// - `usize` - The level of the clicked choice.
/// - `bool` - Whether the choice has children.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(E)>>` - The click handler of the choice.
pub fn cascader_option_handler<E: 'static>(
    value: Signal<Vec<String>>,
    browsed: Signal<Vec<String>>,
    open: Signal<bool>,
    on_select: Option<Rc<dyn Fn(&'static str)>>,
    option_value: &'static str,
    depth: usize,
    has_children: bool,
) -> Option<Rc<dyn Fn(E)>> {
    Some(Rc::new(move |_: E| {
        let mut path: Vec<String> = browsed.get();
        path.truncate(depth);
        path.push(String::from(option_value));
        browsed.set(path.clone());
        if has_children {
            open.set(true);
            return;
        }
        value.set(path);
        if let Some(callback) = &on_select {
            callback(option_value);
        }
        open.set(false);
    }))
}

/// Builds the click handler of the trigger.
///
/// The handler opens and closes the panel. Opening it puts the panel back
/// on the chosen path, so the reader always starts where the last choice
/// left off.
///
/// The event type is a parameter so the DOM-free test suite can drive the
/// handler with a stand-in event; the component always instantiates it with
/// [`Event`].
///
/// # Arguments
///
/// - `Signal<bool>` - The panel's open state.
/// - `Signal<Vec<String>>` - The browsed path owned by the component.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(E)>>` - The trigger click handler.
pub fn cascader_trigger_handler<E: 'static>(
    open: Signal<bool>,
    browsed: Signal<Vec<String>>,
    value: Signal<Vec<String>>,
) -> Option<Rc<dyn Fn(E)>> {
    Some(Rc::new(move |_: E| {
        if open.get() {
            open.set(false);
            return;
        }
        browsed.set(value.get());
        open.set(true);
    }))
}
