use super::*;

/// Creates todo list state signals wrapped in a `UseTodoList` struct.
///
/// # Returns
///
/// - `UseTodoList` - The todo list state.
pub(crate) fn use_todo_list() -> UseTodoList {
    UseTodoList::new(
        App::use_signal(|| {
            let mut items: Vec<String> = Vec::with_capacity(1000);
            for index in 1..=1000 {
                items.push(format!("Item {index}"));
            }
            items
        }),
        App::use_signal(String::new),
        App::use_signal(String::new),
    )
}

/// Validates the new item input and updates the error signal.
///
/// # Arguments
///
/// - `UseTodoList` - The todo list state.
pub(crate) fn validate_todo_new_item(state: UseTodoList) {
    let new_item_value: String = state.get_new_item().get();
    if new_item_value.trim().is_empty() {
        state
            .get_add_error()
            .set(LIST_ERROR_ITEM_NAME_REQUIRED.to_string());
    } else if new_item_value.trim().len() > 50 {
        state
            .get_add_error()
            .set(LIST_ERROR_ITEM_NAME_TOO_LONG.to_string());
    } else {
        state.get_add_error().set(String::new());
    }
}

/// Creates an input event handler that updates the new item and validates it.
///
/// # Arguments
///
/// - `UseTodoList` - The todo list state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
pub(crate) fn todo_list_on_input_new_item(state: UseTodoList) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            state.get_new_item().set(input.value());
        }
        validate_todo_new_item(state);
    }))
}

/// Creates a click event handler that adds a new item to the list.
///
/// # Arguments
///
/// - `UseTodoList` - The todo list state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to add a new item.
pub(crate) fn todo_list_on_add(state: UseTodoList) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        validate_todo_new_item(state);
        let add_error_value: String = state.get_add_error().get();
        if add_error_value.is_empty() {
            let text: String = state.get_new_item().get();
            let mut current: Vec<String> = state.get_items().get();
            current.push(text.trim().to_string());
            state.get_items().set(current);
            state.get_new_item().set(String::new());
        }
    }))
}

/// Creates a click event handler that removes an item at the given index.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The items signal.
/// - `usize` - The index of the item to remove.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to remove the item.
pub(crate) fn todo_list_on_remove(
    items: Signal<Vec<String>>,
    index: usize,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let mut current: Vec<String> = items.get();
        if index < current.len() {
            current.remove(index);
            items.set(current);
        }
    }))
}
