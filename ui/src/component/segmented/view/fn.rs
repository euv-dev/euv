use super::*;

/// A segmented control for switching between a few exclusive options.
///
/// Renders a `c_euv_segmented` container holding one
/// `c_euv_segmented_item` button per option. The option matching the
/// selection signal carries `c_euv_segmented_item_active` as well, so the
/// picked segment is filled with the accent colour.
///
/// A disabled segment is marked `disabled` on the button and carries no
/// click handler at all, so it can never write into the selection signal.
///
/// # Arguments
///
/// - `VirtualNode<EuvSegmentedProps>` - The props node holding the options, the selection signal and the change callback.
///
/// # Returns
///
/// - `VirtualNode` - The segmented control virtual DOM tree.
#[component]
pub fn euv_segmented(node: VirtualNode<EuvSegmentedProps>) -> VirtualNode {
    let EuvSegmentedProps {
        options,
        value,
        on_change,
    }: EuvSegmentedProps = node.try_get_props().unwrap_or_default();
    let selected: String = value.get();
    let items: Vec<VirtualNode> = options
        .into_iter()
        .map(|option: EuvSegmentedOption| {
            let is_active: bool = selected == option.value;
            segmented_item(option, is_active, value, on_change.clone())
        })
        .collect();
    html! {
        div {
            class: c_euv_segmented()
            items
        }
    }
}

/// Renders one segment of the control.
///
/// # Arguments
///
/// - `EuvSegmentedOption` - The option this segment renders.
/// - `bool` - Whether the option is the selected one.
/// - `Signal<String>` - The selection signal owned by the caller.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The change callback.
///
/// # Returns
///
/// - `VirtualNode` - The segment virtual DOM tree.
fn segmented_item(
    option: EuvSegmentedOption,
    is_active: bool,
    value: Signal<String>,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
) -> VirtualNode {
    let disabled: bool = option.disabled;
    let handler: Option<Rc<dyn Fn(Event)>> = if disabled {
        None
    } else {
        segmented_pick(value, on_change, option.value)
    };
    html! {
        button {
            type: "button"
            class: segmented_item_class(is_active)
            disabled: boolean_attr(disabled)
            onclick: handler
            option.label
        }
    }
}

/// Builds the class list of one segment.
///
/// The picked segment carries `c_euv_segmented_item_active` on top of
/// `c_euv_segmented_item`, so the class list is assembled by hand rather
/// than through two `class:` attributes — the registry has no separate
/// "inactive" class, and a second attribute would repeat the base name.
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `String` - A `String` value.
fn segmented_item_class(is_active: bool) -> String {
    let base: &str = c_euv_segmented_item().get_name();
    if !is_active {
        return String::from(base);
    }
    let active: &str = c_euv_segmented_item_active().get_name();
    let mut class_name: String = String::with_capacity(base.len() + 1 + active.len());
    class_name.push_str(base);
    class_name.push(' ');
    class_name.push_str(active);
    class_name
}

/// Builds the click handler that picks one segment.
///
/// The handler writes the option value into the selection signal and
/// reports it to the change callback.
///
/// # Arguments
///
/// - `Signal<String>` - The selection signal owned by the caller.
/// - `Option<Rc<dyn Fn(&'static str)>>` - The change callback.
/// - `&'static str` - The value of the picked option.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The pick click handler.
fn segmented_pick(
    value: Signal<String>,
    on_change: Option<Rc<dyn Fn(&'static str)>>,
    option_value: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        value.set(String::from(option_value));
        if let Some(callback) = &on_change {
            callback(option_value);
        }
    }))
}

/// Maps a boolean flag onto the attribute value the renderer expects.
///
/// The renderer writes the boolean DOM attributes (`disabled`, `checked`)
/// through `set_attribute_or_property`, which compares the value against
/// `"true"`, so the flag has to travel as that literal rather than as a
/// Rust `bool` — a plain `bool` attribute value never reaches the DOM.
///
/// # Arguments
///
/// - `bool` - The flag to render.
///
/// # Returns
///
/// - `&'static str` - The attribute value for the flag.
fn boolean_attr(flag: bool) -> &'static str {
    if flag { "true" } else { "" }
}
