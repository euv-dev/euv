use super::*;

/// The step size used when the caller asks for a step of zero.
///
/// A zero step would leave the value where it is, which makes the steppers
/// look broken, so it is normalised to one.
const FALLBACK_STEP: i64 = 1;

/// The glyph drawn on the increment stepper.
const STEP_UP_GLYPH: &str = "+";

/// The glyph drawn on the decrement stepper.
const STEP_DOWN_GLYPH: &str = "\u{2212}";

/// Returns the value a stepper button would write, or `None` when the
/// stepper is inactive.
///
/// A stepper is inactive when the step would carry the value past `min` or
/// `max`; an unbounded direction never blocks a step. The returned value is
/// clamped into the bounds, so a step that lands exactly on a bound is
/// allowed while one that overshoots it is refused.
///
/// # Arguments
///
/// - `i64` - The current value of the field.
/// - `Option<i64>` - The inclusive lower bound, or `None` for no bound.
///
/// # Returns
///
/// - `Option<i64>` - The clamped value the stepper writes, or `None` when
///   the stepper must stay inert.
pub fn stepped_value(current: i64, step: i64, min: Option<i64>, max: Option<i64>) -> Option<i64> {
    let step: i64 = if step == 0 { FALLBACK_STEP } else { step };
    let candidate: i64 = current.saturating_add(step);
    let blocked: bool = if step > 0 {
        max.is_some_and(|max_value: i64| candidate > max_value)
    } else {
        min.is_some_and(|min_value: i64| candidate < min_value)
    };
    if blocked {
        return None;
    }
    let lowered: i64 = match min {
        Some(min_value) => candidate.max(min_value),
        None => candidate,
    };
    Some(match max {
        Some(max_value) => lowered.min(max_value),
        None => lowered,
    })
}

/// Returns whether a stepper button has to be rendered as inert.
///
/// The stepper is inert when the step would carry the value past one of its
/// bounds, which is exactly the case where [`stepped_value`] yields `None`.
///
/// # Arguments
///
/// - `i64` - The current value of the field.
/// - `Option<i64>` - The inclusive lower bound, or `None` for no bound.
///
/// # Returns
///
/// - `bool` - `true` when the stepper cannot move the value.
pub fn stepper_disabled(current: i64, step: i64, min: Option<i64>, max: Option<i64>) -> bool {
    stepped_value(current, step, min, max).is_none()
}

/// Builds the click handler of a stepper button.
///
/// The handler reads the current value from the signal, so it always steps
/// from the value the user sees, and it writes the clamped result back into
/// the same signal before notifying the caller.
///
/// # Arguments
///
/// - `Signal<i64>` - The signal holding the current value.
/// - `i64` - The signed step size.
/// - `Option<i64>` - The inclusive lower bound, or `None` for no bound.
/// - `Option<Rc<dyn Fn(i64)>>` - The callback receiving the new value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler of the stepper.
fn on_stepper_click(
    value: Signal<i64>,
    step: i64,
    min: Option<i64>,
    max: Option<i64>,
    on_change: Option<Rc<dyn Fn(i64)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        let current: i64 = value.get();
        let Some(next) = stepped_value(current, step, min, max) else {
            return;
        };
        value.set(next);
        if let Some(callback) = &on_change {
            callback(next);
        }
    }))
}

/// Builds the change handler of the number field.
///
/// The handler reads the value off the input element rather than trusting a
/// captured value, so a browser that normalises the typed text still lands
/// on the number the user entered. Text that does not parse as an integer is
/// ignored, which leaves the previous value in place.
///
/// # Arguments
///
/// - `Signal<i64>` - The signal holding the current value.
/// - `Option<Rc<dyn Fn(i64)>>` - The callback receiving the new value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The change handler of the field.
fn on_number_change(
    value: Signal<i64>,
    on_change: Option<Rc<dyn Fn(i64)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let reported: Option<String> = event.target().and_then(|target: EventTarget| {
            if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                return Some(input.value());
            }
            None
        });
        let Some(reported) = reported else {
            return;
        };
        let Ok(parsed) = reported.trim().parse::<i64>() else {
            return;
        };
        value.set(parsed);
        if let Some(callback) = &on_change {
            callback(parsed);
        }
    }))
}

/// A numeric field with a pair of stepper buttons.
///
/// The component renders a `c_euv_number_input` wrapper holding a real
/// `<input type="number">` and the two trailing steppers. The native spinner
/// is hidden by the stylesheet, so the steppers are the only way to move the
/// value by mouse; the field itself still accepts typed numbers.
///
/// The value lives in a caller-owned [`Signal`], so the component takes part
/// in two-way binding: a stepper click and a typed number both write into
/// the signal and notify `on_change`. A stepper that would carry the value
/// past `min` or `max` carries `c_euv_number_input_stepper_disabled` and
/// does nothing when clicked.
///
/// # Arguments
///
/// - `VirtualNode<EuvNumberInputProps>` - The props node carrying the value,
///   the bounds, the step size and the change callback.
///
/// # Returns
///
/// - `VirtualNode` - The number field with its two steppers.
#[component]
pub fn euv_number_input(node: VirtualNode<EuvNumberInputProps>) -> VirtualNode {
    let EuvNumberInputProps {
        value,
        min,
        max,
        step,
        label: label_text,
        disabled,
        on_change,
    }: EuvNumberInputProps = node.try_get_props().unwrap_or_default();
    let step_size: i64 = if step == 0 { FALLBACK_STEP } else { step };
    let increment: i64 = step_size;
    let decrement: i64 = -step_size;
    html! {
        div {
            class: c_euv_input_wrapper()
            if !label_text.is_empty() {
                label {
                    class: c_form_label()
                    label_text
                }
            }
            div {
                class: c_euv_number_input()
                input {
                    type: "number"
                    value: value.get().to_string()
                    disabled: disabled
                    class: c_euv_number_input_field()
                    onchange: on_number_change(value, on_change.clone())
                }
                button {
                    type: "button"
                    class: if { stepper_disabled(value.get(), increment, min, max) || disabled } {
                        c_euv_number_input_stepper_disabled()
                    } else {
                        c_euv_number_input_stepper()
                    }
                    onclick: on_stepper_click(value, increment, min, max, on_change.clone())
                    STEP_UP_GLYPH
                }
                button {
                    type: "button"
                    class: if { stepper_disabled(value.get(), decrement, min, max) || disabled } {
                        c_euv_number_input_stepper_disabled()
                    } else {
                        c_euv_number_input_stepper()
                    }
                    onclick: on_stepper_click(value, decrement, min, max, on_change.clone())
                    STEP_DOWN_GLYPH
                }
            }
        }
    }
}
