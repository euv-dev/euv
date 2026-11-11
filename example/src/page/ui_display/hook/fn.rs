use super::*;

/// Builds the reactive state the display demo page drives.
///
/// The back-to-top button follows the page's scroll offset, so its
/// visibility is derived from the scroll signal with a `watch!` rather
/// than tracked by hand.
///
/// # Returns
///
/// - `DisplayDemoState` - The signals the page drives.
pub(crate) fn use_display_demo_state() -> DisplayDemoState {
    let scroll: UseScroll = use_scroll();
    let carousel_index: Signal<u32> = App::use_signal(|| 0);
    let anchor_active: Signal<String> = App::use_signal(|| String::from(DISPLAY_ANCHOR_FIRST));
    let split_size: Signal<f64> = App::use_signal(|| DISPLAY_SPLIT_DEFAULT_SIZE);
    let split_size_secondary: Signal<f64> = App::use_signal(|| 100.0 - DISPLAY_SPLIT_DEFAULT_SIZE);
    let back_top_visible: Signal<bool> = App::use_signal(|| false);
    let spinner_caption: Signal<bool> = App::use_signal(|| false);
    let reference_tab: Signal<String> = App::use_signal(|| String::from(DISPLAY_REFERENCE_ACCOUNT));
    let reference_value: Signal<f64> = App::use_signal(|| 50.0);
    let reference_checked: Signal<bool> = App::use_signal(|| false);
    let reference_disabled: Signal<bool> = App::use_signal(|| true);
    let reference_dialog_open: Signal<bool> = App::use_signal(|| false);
    watch!(scroll.get_y(), |offset: f64| {
        back_top_visible.set(offset > DISPLAY_BACK_TOP_THRESHOLD);
    });
    watch!(split_size, |size: f64| {
        split_size_secondary.set(100.0 - size);
    });
    DisplayDemoState {
        carousel_index,
        anchor_active,
        split_size,
        split_size_secondary,
        back_top_visible,
        spinner_caption,
        reference_tab,
        reference_value,
        reference_checked,
        reference_disabled,
        reference_dialog_open,
    }
}

/// Builds the handler that scrolls the page back to the top.
///
/// # Arguments
///
/// - `UseScroll` - The scroll state the page reads.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
pub(crate) fn display_on_back_to_top(scroll: UseScroll) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        scroll.scroll_to_top();
    }))
}

/// Builds the handler that marks an anchor entry as the active one.
///
/// The anchor list does not own the active entry: it reports the entry that
/// was picked and the page decides which one is active.
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the active anchor value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The handler of the anchor entries.
pub(crate) fn display_on_anchor_select(active: Signal<String>) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |value: &'static str| {
        active.set(String::from(value));
    }))
}

/// Builds the handler that reports a click on a link.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler of the link.
pub(crate) fn display_on_link_click() -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        Console::log(DISPLAY_LINK_LOG);
    }))
}

/// Builds the handler that reports the slide on screen.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(u32)>>` - The change handler of the carousel.
pub(crate) fn display_on_carousel_change() -> Option<Rc<dyn Fn(u32)>> {
    Some(Rc::new(move |index: u32| {
        Console::log(DISPLAY_CAROUSEL_LOG.replace("{index}", &index.to_string()));
    }))
}

/// Builds the handler that reports the empty state's action.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The action handler of the empty state.
pub(crate) fn display_on_empty_action() -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        Console::log(DISPLAY_EMPTY_LOG);
    }))
}

/// Builds the handler that opens the reference dialog.
///
/// # Arguments
///
/// - `Signal<bool>` - The open-state signal owned by the display page.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The dialog trigger handler.
pub(crate) fn display_on_dialog_open(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        open.set(true);
    }))
}
