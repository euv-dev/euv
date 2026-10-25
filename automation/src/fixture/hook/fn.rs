use super::*;

/// Derives a reactive class binding while keeping the modal subtree mounted.
///
/// # Arguments
///
/// - `Signal<bool>` - Modal visibility signal.
///
/// # Returns
///
/// - `Signal<String>` - Class name updated by the framework's computed lifecycle.
pub(crate) fn fixture_modal_class(visible: Signal<bool>) -> Signal<String> {
    computed!(visible, |open: bool| -> String {
        if open {
            c_automation_visible().get_name().to_string()
        } else {
            c_automation_hidden().get_name().to_string()
        }
    })
}

/// Initializes the framework's real theme tokens inside the component lifecycle.
///
/// # Returns
///
/// - `ThemeState` - Theme class and token initialization used by euv controls.
pub(crate) fn fixture_theme() -> ThemeState {
    let mobile: Signal<bool> = App::use_signal(|| false);
    ThemeState::use_theme_state(mobile)
}

/// Creates state inside the euv hook lifecycle.
///
/// # Returns
///
/// - `FixtureState` - Stable signals for the mounted fixture.
pub(crate) fn fixture_state() -> FixtureState {
    FixtureState::new(
        App::use_signal(|| 0),
        App::use_signal(String::new),
        App::use_signal(|| false),
        App::use_signal(|| false),
        App::use_signal(|| false),
        App::use_signal(|| 0),
        App::use_signal(|| 0),
    )
}

/// Updates the counter from a native browser click.
///
/// # Arguments
///
/// - `FixtureState` - Reactive fixture state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - Component click handler.
pub(crate) fn increment_handler(state: FixtureState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_count().set(state.get_count().get() + 1);
        state.get_trusted().set(event.is_trusted());
    }))
}

/// Synchronizes the input value from an actual input event.
///
/// # Arguments
///
/// - `FixtureState` - Reactive fixture state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - Input event handler.
pub(crate) fn input_handler(state: FixtureState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<HtmlInputElement>().ok())
        {
            state.get_text().set(target.value());
            state.get_trusted().set(event.is_trusted());
        }
    }))
}

/// Changes modal visibility without unmounting its contents.
///
/// # Arguments
///
/// - `Signal<bool>` - Visibility signal.
/// - `bool` - Requested visibility.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - Modal action handler.
pub(crate) fn modal_handler(visible: Signal<bool>, next: bool) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| visible.set(next)))
}

/// Observes real keyboard input bubbling through the fixture.
///
/// # Arguments
///
/// - `Signal<i32>` - Enter-event counter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - Keyboard listener.
pub(crate) fn keyboard_handler(keys: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Ok(keyboard) = event.dyn_into::<KeyboardEvent>()
            && keyboard.key() == ENTER_KEY
        {
            keys.set(keys.get() + 1);
        }
    }))
}

/// Publishes the browser's scroll offset through euv reactivity.
///
/// # Arguments
///
/// - `Signal<i32>` - Scroll-position signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - Scroll listener.
pub(crate) fn scroll_handler(position: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<HtmlElement>().ok())
        {
            position.set(target.scroll_top());
        }
    }))
}
