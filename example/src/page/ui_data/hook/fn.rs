use super::*;

/// Builds the reactive state the data demo page drives.
///
/// # Returns
///
/// - `DataDemoState` - The signals the page drives.
pub(crate) fn use_data_demo_state() -> DataDemoState {
    DataDemoState {
        selected: App::use_signal(String::new),
        message_count: App::use_signal(|| 0),
        notification_count: App::use_signal(|| 0),
    }
}

/// Builds the handler that pushes a toast onto the stack.
///
/// # Arguments
///
/// - `Signal<usize>` - The count the handler raises.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
pub(crate) fn data_push_message(count: Signal<usize>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        let current: usize = count.get();
        count.set(current + 1);
    }))
}

/// Builds the handler that removes the last toast from the stack.
///
/// # Arguments
///
/// - `Signal<usize>` - The count the handler lowers.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
pub(crate) fn data_pop_message(count: Signal<usize>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        let current: usize = count.get();
        count.set(current.saturating_sub(1));
    }))
}

/// Builds the handler that pushes a notification onto the stack.
///
/// # Arguments
///
/// - `Signal<usize>` - The count the handler raises.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
pub(crate) fn data_push_notification(count: Signal<usize>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        let current: usize = count.get();
        count.set(current + 1);
    }))
}

/// Builds the handler that removes the last notification from the stack.
///
/// # Arguments
///
/// - `Signal<usize>` - The count the handler lowers.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
pub(crate) fn data_pop_notification(count: Signal<usize>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        let current: usize = count.get();
        count.set(current.saturating_sub(1));
    }))
}

/// Builds the handler that reports the chosen tree node.
///
/// # Arguments
///
/// - `Signal<String>` - The signal the chosen value is written into.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select handler.
pub(crate) fn data_on_tree_select(selected: Signal<String>) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |value: &'static str| {
        selected.set(String::from(value));
    }))
}

/// Builds the handler that reports a confirmed popconfirm.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn()>>` - The confirm handler of the popconfirm.
pub(crate) fn data_on_popconfirm_ok() -> Option<Rc<dyn Fn()>> {
    Some(Rc::new(move || {
        Console::log(DATA_POPCONFIRM_OK_LOG);
    }))
}

/// Builds the handler that reports a cancelled popconfirm.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn()>>` - The cancel handler of the popconfirm.
pub(crate) fn data_on_popconfirm_cancel() -> Option<Rc<dyn Fn()>> {
    Some(Rc::new(move || {
        Console::log(DATA_POPCONFIRM_CANCEL_LOG);
    }))
}

/// Builds the handler that reports the action of a message.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn()>>` - The action handler of the message.
pub(crate) fn data_on_message_action() -> Option<Rc<dyn Fn()>> {
    Some(Rc::new(move || {
        Console::log(DATA_MESSAGE_ACTION_LOG);
    }))
}

/// Builds the handler that closes the topmost message.
///
/// # Arguments
///
/// - `Signal<usize>` - The count of the messages on the stack.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn()>>` - The close handler of the message.
pub(crate) fn data_on_message_close(count: Signal<usize>) -> Option<Rc<dyn Fn()>> {
    Some(Rc::new(move || {
        let current: usize = count.get();
        count.set(current.saturating_sub(1));
        Console::log(DATA_MESSAGE_CLOSE_LOG);
    }))
}

/// Builds the handler that closes the topmost notification.
///
/// # Arguments
///
/// - `Signal<usize>` - The count of the notifications on the stack.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn()>>` - The close handler of the notification.
pub(crate) fn data_on_notification_close(count: Signal<usize>) -> Option<Rc<dyn Fn()>> {
    Some(Rc::new(move || {
        let current: usize = count.get();
        count.set(current.saturating_sub(1));
        Console::log(DATA_NOTIFICATION_CLOSE_LOG);
    }))
}
