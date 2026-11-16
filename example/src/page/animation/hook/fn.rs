use super::*;

/// Creates progress bar state with a running signal.
///
/// # Returns
///
/// - `UseProgress` - The progress state containing a running signal.
pub(crate) fn use_progress() -> UseProgress {
    Css::inject_css(ANIMATION_DEMO_KEYFRAMES);
    UseProgress::new(App::use_signal(|| false), App::use_signal(|| 0))
}

/// Creates a click event handler that starts the progress bar animation.
///
/// Alternates the fill animation name to replay it on every start.
///
/// # Arguments
///
/// - `UseProgress` - The progress state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to start the progress bar.
pub(crate) fn progress_on_start(state: UseProgress) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        progress_start(state);
    }))
}

/// Creates a click event handler that resets the progress bar.
///
/// Sets running to false to remove the CSS animation and reset the bar.
///
/// # Arguments
///
/// - `UseProgress` - The progress state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to reset the progress bar.
pub(crate) fn progress_on_reset(state: UseProgress) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        progress_reset(state);
    }))
}

/// Starts a fresh progress animation, including after completion.
///
/// # Arguments
///
/// - `UseProgress` - The progress state.
pub(crate) fn progress_start(state: UseProgress) {
    state
        .get_generation()
        .set(state.get_generation().get().wrapping_add(1));
    state.get_running().set(true);
}

/// Returns the progress fill to its empty state without a transition.
///
/// # Arguments
///
/// - `UseProgress` - The progress state.
pub(crate) fn progress_reset(state: UseProgress) {
    state.get_running().set(false);
}
