use super::*;

/// Creates props demo state signals.
///
/// # Returns
///
/// - `UsePropsDemo` - The props demo state.
pub(crate) fn use_props_demo() -> UsePropsDemo {
    UsePropsDemo::new(App::use_signal(|| BINDING_PARENT_GREETING_TEXT.to_string()))
}

/// Creates two-way binding demo state signals.
///
/// # Returns
///
/// - `UseTwoWayDemo` - The two-way binding demo state.
pub(crate) fn use_two_way_demo() -> UseTwoWayDemo {
    UseTwoWayDemo::new(
        App::use_signal(|| BINDING_SHARED_TEXT_PLACEHOLDER.to_string()),
        App::use_signal(|| 0),
    )
}

/// Creates a click event handler that increments the shared counter.
///
/// The `html!` macro wraps the returned closure into a `NativeEventHandler`
/// when it is bound to an event attribute.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn two_way_on_increment(counter: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: i32 = counter.get();
        counter.set(current + 1);
    }))
}

/// Creates a click event handler that decrements the shared counter.
///
/// The `html!` macro wraps the returned closure into a `NativeEventHandler`
/// when it is bound to an event attribute.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn two_way_on_decrement(counter: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: i32 = counter.get();
        counter.set(current - 1);
    }))
}

/// Creates cross-component demo state signals with watch! bindings.
///
/// # Returns
///
/// - `UseCrossComponentDemo` - The cross-component demo state.
pub(crate) fn use_cross_component_demo() -> UseCrossComponentDemo {
    let state: UseCrossComponentDemo = UseCrossComponentDemo::new(
        App::use_signal(|| 0.0),
        App::use_signal(|| 32.0),
        App::use_signal(|| 79),
        App::use_signal(|| 70),
        App::use_signal(|| 229),
        App::use_signal(|| BINDING_HEX_COLOR_INITIAL.to_string()),
    );
    let celsius: Signal<f64> = state.get_celsius();
    let fahrenheit: Signal<f64> = state.get_fahrenheit();
    let red: Signal<i32> = state.get_red();
    let green: Signal<i32> = state.get_green();
    let blue: Signal<i32> = state.get_blue();
    let hex_color: Signal<String> = state.get_hex_color();
    watch!(celsius, |celsius_value: f64| {
        let new_fahrenheit: f64 = celsius_value * 9.0 / 5.0 + 32.0;
        let rounded: f64 = (new_fahrenheit * 100.0).round() / 100.0;
        fahrenheit.set(rounded);
    });
    watch!(fahrenheit, |fahrenheit_value: f64| {
        let new_celsius: f64 = (fahrenheit_value - 32.0) * 5.0 / 9.0;
        let rounded: f64 = (new_celsius * 100.0).round() / 100.0;
        celsius.set(rounded);
    });
    watch!(
        red,
        green,
        blue,
        |red_value: i32, green_value: i32, blue_value: i32| {
            let clamped_red: i32 = red_value.clamp(0, 255);
            let clamped_green: i32 = green_value.clamp(0, 255);
            let clamped_blue: i32 = blue_value.clamp(0, 255);
            hex_color.set(format!(
                "#{:02x}{:02x}{:02x}",
                clamped_red, clamped_green, clamped_blue
            ));
        }
    );
    state
}

/// Creates an input event handler that updates the celsius value via
/// `requestAnimationFrame` throttling to ensure at most one signal
/// update per paint frame.
///
/// Instead of updating the signal on every `oninput` event (which can
/// fire many times per frame), stores the pending value and schedules
/// a single `requestAnimationFrame` callback. The callback reads the
/// latest pending value and applies it exactly once per paint frame.
///
/// The `html!` macro wraps the returned closure into a `NativeEventHandler`
/// when it is bound to an event attribute.
///
/// # Arguments
///
/// - `Signal<f64>` - The celsius signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
pub(crate) fn cross_on_input_celsius(signal: Signal<f64>) -> Option<Rc<dyn Fn(Event)>> {
    let pending_value: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            let parsed: f64 = input.value().parse().unwrap_or_default();
            pending_value.set(parsed);
            if raf_id.get().is_some() {
                return;
            }
            let pending_for_raf: Rc<Cell<f64>> = pending_value.clone();
            let raf_id_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
            let Some(window_value): Option<Window> = window() else {
                return;
            };
            let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                raf_id_clone.set(None);
                signal.set(pending_for_raf.get());
            }));
            let id: i32 = window_value
                .request_animation_frame(raf_closure.as_ref().unchecked_ref())
                .unwrap_or_default();
            raf_id.set(Some(id));
            raf_closure.forget();
        }
    }))
}

/// Creates an input event handler that updates the fahrenheit value via
/// `requestAnimationFrame` throttling to ensure at most one signal
/// update per paint frame.
///
/// Instead of updating the signal on every `oninput` event (which can
/// fire many times per frame), stores the pending value and schedules
/// a single `requestAnimationFrame` callback. The callback reads the
/// latest pending value and applies it exactly once per paint frame.
///
/// The `html!` macro wraps the returned closure into a `NativeEventHandler`
/// when it is bound to an event attribute.
///
/// # Arguments
///
/// - `Signal<f64>` - The fahrenheit signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
pub(crate) fn cross_on_input_fahrenheit(signal: Signal<f64>) -> Option<Rc<dyn Fn(Event)>> {
    let pending_value: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            let parsed: f64 = input.value().parse().unwrap_or_default();
            pending_value.set(parsed);
            if raf_id.get().is_some() {
                return;
            }
            let pending_for_raf: Rc<Cell<f64>> = pending_value.clone();
            let raf_id_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
            let Some(window_value): Option<Window> = window() else {
                return;
            };
            let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                raf_id_clone.set(None);
                signal.set(pending_for_raf.get());
            }));
            let id: i32 = window_value
                .request_animation_frame(raf_closure.as_ref().unchecked_ref())
                .unwrap_or_default();
            raf_id.set(Some(id));
            raf_closure.forget();
        }
    }))
}

/// Creates an input event handler that updates an i32 signal via
/// `requestAnimationFrame` throttling to ensure at most one signal
/// update per paint frame.
///
/// Instead of updating the signal on every `oninput` event (which can
/// fire many times per frame), stores the pending value and schedules
/// a single `requestAnimationFrame` callback. The callback reads the
/// latest pending value and applies it exactly once per paint frame.
///
/// The `html!` macro wraps the returned closure into a `NativeEventHandler`
/// when it is bound to an event attribute.
///
/// # Arguments
///
/// - `Signal<i32>` - The signal to update.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
pub(crate) fn cross_on_input_i32(signal: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    let pending_value: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            let parsed: i32 = input.value().parse().unwrap_or_default();
            let clamped: i32 = parsed.clamp(0, 255);
            let percent: f64 = (clamped as f64 / 255.0 * 100.0).clamp(0.0, 100.0);
            input
                .style()
                .set_property("--value", &format!("{percent}%"))
                .unwrap_or(());
            pending_value.set(clamped);
            if raf_id.get().is_some() {
                return;
            }
            let pending_for_raf: Rc<Cell<i32>> = pending_value.clone();
            let raf_id_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
            let Some(window_value): Option<Window> = window() else {
                return;
            };
            let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                raf_id_clone.set(None);
                signal.set(pending_for_raf.get());
            }));
            let id: i32 = window_value
                .request_animation_frame(raf_closure.as_ref().unchecked_ref())
                .unwrap_or_default();
            raf_id.set(Some(id));
            raf_closure.forget();
        }
    }))
}

/// Creates typed props demo state signals.
///
/// # Returns
///
/// - `UseTypedPropsDemo` - The typed props demo state.
pub(crate) fn use_typed_props_demo() -> UseTypedPropsDemo {
    UseTypedPropsDemo::new(
        App::use_signal(|| false),
        App::use_signal(|| 5),
        App::use_signal(|| 0),
    )
}

/// Creates a click event handler that toggles the disabled signal.
///
/// The `html!` macro wraps the returned closure into a `NativeEventHandler`
/// when it is bound to an event attribute.
///
/// # Arguments
///
/// - `Signal<bool>` - The disabled signal to toggle.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn typed_props_on_toggle_disabled(disabled: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = disabled.get();
        disabled.set(!current);
    }))
}

/// Creates a click event handler that increments the count within the max limit.
///
/// When the disabled signal is true, the handler executes but skips the count update.
///
/// The `html!` macro wraps the returned closure into a `NativeEventHandler`
/// when it is bound to an event attribute.
///
/// # Arguments
///
/// - `Signal<i32>` - The current count signal.
/// - `Signal<i32>` - The max count signal.
/// - `Signal<bool>` - The disabled signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn typed_props_on_increment(
    count: Signal<i32>,
    max_count: Signal<i32>,
    disabled: Signal<bool>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if disabled.get() {
            return;
        }
        let current: i32 = count.get();
        let max: i32 = max_count.get();
        if current < max {
            count.set(current + 1);
        }
    }))
}

/// Creates a click event handler that resets the count to zero.
///
/// When the disabled signal is true, the handler executes but skips the reset.
///
/// The `html!` macro wraps the returned closure into a `NativeEventHandler`
/// when it is bound to an event attribute.
///
/// # Arguments
///
/// - `Signal<i32>` - The current count signal.
/// - `Signal<bool>` - The disabled signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn typed_props_on_reset_count(
    count: Signal<i32>,
    disabled: Signal<bool>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if disabled.get() {
            return;
        }
        count.set(0);
    }))
}
