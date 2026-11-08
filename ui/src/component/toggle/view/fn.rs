use super::*;

/// A two-state toggle button.
///
/// The control is a `<button>` that reports its pressed state through
/// `aria-pressed`, so assistive technology reads it as a toggle rather
/// than a plain button. The pressed state lives in a caller-owned signal:
/// the component writes the new value into it and reports it to the
/// change callback.
///
/// A disabled toggle carries no click handler at all, so it can never
/// write into the signal.
///
/// # Arguments
///
/// - `VirtualNode<EuvToggleProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The toggle virtual DOM tree.
#[component]
pub fn euv_toggle(node: VirtualNode<EuvToggleProps>) -> VirtualNode {
    let EuvToggleProps {
        pressed,
        variant,
        size,
        label,
        disabled,
        on_change,
    }: EuvToggleProps = node.try_get_props().unwrap_or_default();
    let is_pressed: bool = pressed.get();
    let is_disabled: bool = disabled.get();
    let pressed_attr: &str = if is_pressed {
        TOGGLE_ATTRIBUTE_ON
    } else {
        TOGGLE_ATTRIBUTE_OFF
    };
    let disabled_attr: &str = if is_disabled {
        TOGGLE_ATTRIBUTE_ON
    } else {
        TOGGLE_ATTRIBUTE_EMPTY
    };
    let handler: Option<Rc<dyn Fn(Event)>> = if is_disabled {
        None
    } else {
        toggle_press(pressed, on_change)
    };
    html! {
        button {
            type: "button"
            class: toggle_class(variant, size, is_pressed)
            aria-pressed: pressed_attr
            disabled: disabled_attr
            onclick: handler
            label
        }
    }
}

/// Builds the class list of a toggle.
///
/// The registry splits the control into a base class, a variant class, a
/// size class and a pressed class, so the list is assembled by hand
/// rather than through four `class:` attributes — one attribute would
/// repeat the base name.
///
/// # Arguments
///
/// - `EuvToggleVariant` - The visual variant.
/// - `EuvToggleSize` - The control size.
/// - `bool` - Whether the toggle is pressed.
///
/// # Returns
///
/// - `String` - The class list.
pub(crate) fn toggle_class(
    variant: EuvToggleVariant,
    size: EuvToggleSize,
    is_pressed: bool,
) -> String {
    let mut class_name: String = String::with_capacity(96);
    class_name.push_str(c_euv_toggle().get_name());
    if variant == EuvToggleVariant::Outline {
        class_name.push(' ');
        class_name.push_str(c_euv_toggle_outline().get_name());
    }
    let size_class: &str = match size {
        EuvToggleSize::Default => c_euv_toggle_size_default().get_name(),
        EuvToggleSize::Small => c_euv_toggle_size_small().get_name(),
        EuvToggleSize::Large => c_euv_toggle_size_large().get_name(),
    };
    class_name.push(' ');
    class_name.push_str(size_class);
    if is_pressed {
        class_name.push(' ');
        class_name.push_str(c_euv_toggle_pressed().get_name());
    }
    class_name
}

/// Builds the click handler that flips the pressed state.
///
/// # Arguments
///
/// - `Signal<bool>` - The pressed-state signal owned by the caller.
/// - `Option<Rc<dyn Fn(bool)>>` - The change callback.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The press click handler.
fn toggle_press(
    pressed: Signal<bool>,
    on_change: Option<Rc<dyn Fn(bool)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let next: bool = !pressed.get();
        pressed.set(next);
        if let Some(callback) = on_change.as_ref() {
            callback(next);
        }
    }))
}
