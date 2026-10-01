use super::*;

/// Creates stopwatch state signals wrapped in a `UseStopwatch` struct.
///
/// Must be called at the top level of a component function (not inside
/// conditionals or loops) to maintain hook call order stability.
///
/// # Returns
///
/// - `UseStopwatch` - The stopwatch state containing seconds, running, and handle signals.
pub(crate) fn use_stopwatch() -> UseStopwatch {
    UseStopwatch::new(
        App::use_signal(|| 0),
        App::use_signal(|| false),
        App::use_signal(|| None),
    )
}

/// Creates countdown state signals wrapped in a `UseCountdown` struct.
///
/// Must be called at the top level of a component function (not inside
/// conditionals or loops) to maintain hook call order stability.
///
/// # Returns
///
/// - `UseCountdown` - The countdown state containing total, remaining, running, handle, and input signals.
pub(crate) fn use_countdown() -> UseCountdown {
    UseCountdown::new(
        App::use_signal(|| 60),
        App::use_signal(|| 60),
        App::use_signal(|| false),
        App::use_signal(|| None),
        App::use_signal(|| "60".to_string()),
    )
}

/// Creates a click event handler that toggles the stopwatch between
/// running and paused. Dispatches to start on the first click after
/// reset, and to pause on every subsequent click until reset.
///
/// # Arguments
///
/// - `UseStopwatch` - The stopwatch state (only `Copy` signals are read from this).
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that toggles the stopwatch.
pub(crate) fn stopwatch_on_start(state: UseStopwatch) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let was_running: bool = state.get_running().get();
        if was_running {
            stopwatch_on_pause_inner(state);
        } else {
            stopwatch_on_start_inner(state);
        }
    }))
}

/// Performs the stopwatch start logic without re-allocating the event
/// handler that the timer page would otherwise need to rebuild on
/// every render.
///
/// # Arguments
///
/// - `UseStopwatch` - The `state` argument.
///
fn stopwatch_on_start_inner(state: UseStopwatch) {
    let was_running: bool = state.get_running().get();
    if was_running {
        return;
    }
    state.get_running().set(true);
    let handle_opt: Option<IntervalHandle> = state.get_handle().get();
    if let Some(existing_handle) = handle_opt {
        existing_handle.clear();
    }
    let seconds_signal: Signal<i32> = state.get_seconds();
    let handle_signal: Signal<Option<IntervalHandle>> = state.get_handle();
    let new_handle: IntervalHandle = App::use_interval(1000, move || {
        let current: i32 = seconds_signal.get();
        seconds_signal.set(current + 1);
    });
    handle_signal.set(Some(new_handle));
}

/// Clears the active interval, sets running to false.
///
/// # Arguments
///
/// - `UseStopwatch` - The `state` argument.
///
fn stopwatch_on_pause_inner(state: UseStopwatch) {
    let handle_opt: Option<IntervalHandle> = state.get_handle().get();
    if let Some(existing_handle) = handle_opt {
        existing_handle.clear();
    }
    state.get_handle().set(None);
    state.get_running().set(false);
}

/// Creates a click event handler that toggles the countdown between
/// running and paused. Dispatches to start on the first click after
/// reset, and to pause on every subsequent click until reset.
///
/// # Arguments
///
/// - `UseCountdown` - The `state` argument.
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The value this function returns.
///
pub(crate) fn countdown_on_start(state: UseCountdown) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let was_running: bool = state.get_running().get();
        if was_running {
            countdown_on_pause_inner(state);
        } else {
            countdown_on_start_inner(state);
        }
    }))
}

/// Performs the countdown start logic without re-allocating the event
/// handler that the timer page would otherwise need to rebuild on
/// every render.
///
/// # Arguments
///
/// - `UseCountdown` - The `state` argument.
///
fn countdown_on_start_inner(state: UseCountdown) {
    let was_running: bool = state.get_running().get();
    if was_running {
        return;
    }
    let current_remaining: i32 = state.get_remaining().get();
    let current_total: i32 = state.get_total().get();
    let has_paused_state: bool = current_remaining > 0 && current_remaining < current_total;
    if !has_paused_state {
        let input_text: String = state.get_input().get();
        let parsed: i32 = input_text.parse::<i32>().unwrap_or(60);
        let safe_total: i32 = if parsed > 0 { parsed } else { 60 };
        state.get_total().set(safe_total);
        state.get_remaining().set(safe_total);
    }
    state.get_running().set(true);
    let handle_opt: Option<IntervalHandle> = state.get_handle().get();
    if let Some(existing_handle) = handle_opt {
        existing_handle.clear();
    }
    let remaining_signal: Signal<i32> = state.get_remaining();
    let running_signal: Signal<bool> = state.get_running();
    let handle_signal: Signal<Option<IntervalHandle>> = state.get_handle();
    let new_handle: IntervalHandle = App::use_interval(1000, move || {
        if running_signal.get() {
            let current: i32 = remaining_signal.get();
            if current > 0 {
                remaining_signal.set(current - 1);
            } else {
                running_signal.set(false);
                handle_signal.set(None);
            }
        }
    });
    handle_signal.set(Some(new_handle));
}

/// Clears the active countdown interval, sets running to false.
///
/// # Arguments
///
/// - `UseCountdown` - The `state` argument.
///
fn countdown_on_pause_inner(state: UseCountdown) {
    let handle_opt: Option<IntervalHandle> = state.get_handle().get();
    if let Some(existing_handle) = handle_opt {
        existing_handle.clear();
    }
    state.get_handle().set(None);
    state.get_running().set(false);
}

/// Creates a click event handler that resets the stopwatch.
///
/// Immediately clears the interval, resets running state and seconds counter.
///
/// # Arguments
///
/// - `UseStopwatch` - The stopwatch state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to reset the stopwatch.
pub(crate) fn stopwatch_on_reset(state: UseStopwatch) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let handle_opt: Option<IntervalHandle> = state.get_handle().get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        state.get_handle().set(None);
        state.get_running().set(false);
        state.get_seconds().set(0);
    }))
}

/// Creates a click event handler that resets the countdown.
///
/// Immediately clears the interval, resets running state, and restores
/// remaining to the total value.
///
/// # Arguments
///
/// - `UseCountdown` - The countdown state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to reset the countdown.
pub(crate) fn countdown_on_reset(state: UseCountdown) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let handle_opt: Option<IntervalHandle> = state.get_handle().get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        state.get_handle().set(None);
        state.get_running().set(false);
        let current_total: i32 = state.get_total().get();
        state.get_remaining().set(current_total);
    }))
}

/// Creates an input event handler that updates the countdown input signal.
///
/// # Arguments
///
/// - `UseCountdown` - The countdown state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler for the countdown input field.
pub(crate) fn countdown_on_input(state: UseCountdown) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            state.get_input().set(input.value());
        }
    }))
}
