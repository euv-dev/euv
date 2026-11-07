use super::*;

/// A one-time-code field.
///
/// The field is one real input with a slot drawn for every character of
/// the code. The input itself is transparent and covers the slots, so
/// the reader types into the field while the drawn slots show what was
/// typed. The slot the caret sits in is marked, which is the same
/// affordance the reference gives the reader.
///
/// # Arguments
///
/// - `VirtualNode<EuvInputOtpProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv input otp virtual DOM tree.
#[component]
pub fn euv_input_otp(node: VirtualNode<EuvInputOtpProps>) -> VirtualNode {
    let EuvInputOtpProps {
        value,
        length,
        disabled,
        label,
    }: EuvInputOtpProps = node.try_get_props().unwrap_or_default();
    let text: String = value.get();
    let characters: Vec<String> = text.chars().map(|found: char| found.to_string()).collect();
    let mut slots: Vec<VirtualNode> = Vec::new();
    for index in 0..length {
        let character: String = characters.get(index).cloned().unwrap_or_default();
        let active: bool = index == characters.len();
        slots.push(otp_slot(character, active));
    }
    html! {
        div {
            class: c_euv_input_otp()
            "aria-label": label
            input {
                type: "text"
                inputmode: "numeric"
                autocomplete: "one-time-code"
                value: text
                disabled: disabled
                class: c_euv_input_otp_input()
                oninput: on_otp_input(value)
            }
            slots
        }
    }
}

/// Draws one slot of the code.
///
/// The slot is drawn by a helper of its own so the slot's text is owned
/// by the call that draws it, and the field's own text stays untouched.
///
/// # Arguments
///
/// - `String` - The character drawn in the slot.
/// - `bool` - Whether the slot is the one the caret is in.
///
/// # Returns
///
/// - `VirtualNode` - The slot virtual DOM tree.
fn otp_slot(character: String, active: bool) -> VirtualNode {
    let character: &'static str = Box::leak(character.into_boxed_str());
    html! {
        div {
            class: c_euv_input_otp_slot()
            class: if active {
                c_euv_input_otp_slot_active()
            } else {
                c_euv_input_otp_slot_idle()
            }
            {
                character
            }
        }
    }
}

/// Builds the handler that keeps the field's text in the caller's signal.
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the field's text.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn on_otp_input(value: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Some(target) = event.target() else {
            return;
        };
        let Ok(input) = target.dyn_into::<HtmlInputElement>() else {
            return;
        };
        value.set(input.value());
    }))
}

/// A row of slots.
///
/// The group is the row the slots sit in; it draws nothing itself, so a
/// caller can use it to lay a code out in one line.
///
/// # Arguments
///
/// - `VirtualNode<EuvInputOtpGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv input otp group virtual DOM tree.
#[component]
pub fn euv_input_otp_group(node: VirtualNode<EuvInputOtpGroupProps>) -> VirtualNode {
    let EuvInputOtpGroupProps { label }: EuvInputOtpGroupProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "aria-label": label
            class: c_euv_input_otp_group()
            children
        }
    }
}

/// One slot of the code.
///
/// The slot draws the character it stands for. The slot the caret is in
/// carries the active class, which is what marks it on screen.
///
/// # Arguments
///
/// - `VirtualNode<EuvInputOtpSlotProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv input otp slot virtual DOM tree.
#[component]
pub fn euv_input_otp_slot(node: VirtualNode<EuvInputOtpSlotProps>) -> VirtualNode {
    let EuvInputOtpSlotProps {
        char: character,
        active,
    }: EuvInputOtpSlotProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_euv_input_otp_slot()
            class: if active {
                c_euv_input_otp_slot_active()
            } else {
                c_euv_input_otp_slot_idle()
            }
            {
                character
            }
        }
    }
}

/// The rule that divides two groups of slots.
///
/// # Arguments
///
/// - `VirtualNode<EuvInputOtpGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv input otp separator virtual DOM tree.
#[component]
pub fn euv_input_otp_separator(node: VirtualNode<EuvInputOtpGroupProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "separator"
            class: c_euv_input_otp_separator()
            children
        }
    }
}
