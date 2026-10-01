use super::*;

/// A toggle switch component with two-way binding via a signal.
///
/// Renders a row containing a `<button role="switch">` whose appearance
/// tracks the checked state, plus a paired label. The button carries
/// `aria-checked` so assistive technology announces the on/off state
/// without depending on the visual classes. Clicking the button toggles
/// the caller-owned `checked` signal through the hook-registered
/// [`EuvSwitchState`] handle; a disabled switch ignores clicks entirely.
///
/// # Arguments
///
/// - `VirtualNode<EuvSwitchProps>` - The props node containing switch configuration.
///
/// # Returns
///
/// - `VirtualNode` - A styled labeled switch element.
#[component]
pub fn euv_switch(node: VirtualNode<EuvSwitchProps>) -> VirtualNode {
    let EuvSwitchProps {
        id,
        name,
        checked,
        label: label_text,
        disabled,
    }: EuvSwitchProps = node.try_get_props().unwrap_or_default();
    let state: EuvSwitchState = use_euv_switch_state(checked);
    let handler: Option<Rc<dyn Fn(Event)>> = state.on_toggle(disabled);
    html! {
        div {
            class: c_form_switch_row()
            button {
                id: id
                name: name
                type: "button"
                role: "switch"
                aria-checked: checked
                disabled: disabled
                class: if { checked } {
                    c_form_switch_on()
                } else {
                    c_form_switch()
                }
                onclick: handler
                span {
                    class: c_form_switch_track()
                    span {
                        class: c_form_switch_thumb()
                    }
                }
            }
            label {
                for: id
                class: c_form_switch_label()
                label_text
            }
        }
    }
}

/// Obtains the switch state handle registered against the current hook
/// context slot.
///
/// Behaves like [`HookContext::use_hook`] — the same handle is returned
/// on every render at the same hook index, so the captured signal
/// survives re-renders. The factory is used directly when no hook
/// context is active (e.g. when called outside a render cycle).
///
/// # Arguments
///
/// - `Signal<bool>` - The checked-state signal owned by the caller.
///
/// # Returns
///
/// - `EuvSwitchState` - The switch state handle.
pub fn use_euv_switch_state(checked: Signal<bool>) -> EuvSwitchState {
    HookContext::use_hook(move || EuvSwitchState::new(checked))
}
