use super::*;

/// Creates an `IntersectionObserver` stored on `window.__euv_observer_instance`
/// that observes the container element matching `selector`.
///
/// When the container enters or leaves the viewport, its tag name,
/// intersection state, and child count are logged via `Console::log`.
/// Only the container element itself is observed (not individual children)
/// to avoid O(N) performance overhead with large lists.
///
/// The observer is automatically disconnected when the hook context is cleared
/// (i.e., when the component unmounts or a `match` arm switches).
///
/// # Arguments
///
/// - `&str` - A CSS selector string to identify the elements to observe.
fn bind_observer(selector: &str) {
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let Some(document_value): Option<Document> = window_value.document() else {
        return;
    };
    let observer_key: JsValue = JsValue::from_str("__euv_observer_instance");
    if Reflect::get(&window_value, &observer_key)
        .ok()
        .and_then(|value: JsValue| value.dyn_into::<IntersectionObserver>().ok())
        .is_some()
    {
        return;
    }
    let callback: Closure<dyn FnMut(Array)> = Closure::wrap(Box::new(move |entries: Array| {
        for index in 0..entries.length() {
            let entry: JsValue = entries.get(index);
            let Ok(intersection_entry): Result<IntersectionObserverEntry, JsValue> =
                entry.dyn_into::<IntersectionObserverEntry>()
            else {
                continue;
            };
            if !intersection_entry.is_intersecting() {
                continue;
            }
            let target: Element = intersection_entry.target();
            let tag_name: String = target.tag_name();
            let intersection_ratio: f64 = intersection_entry.intersection_ratio();
            let data_index: Option<String> = target.get_attribute("data_index");
            match data_index {
                Some(index_value) => {
                    Console::log(format!(
                        "[IntersectionObserver] <{}> index={}, intersection_ratio={:.2}",
                        tag_name, index_value, intersection_ratio
                    ));
                }
                None => {
                    let Ok(children): Result<NodeList, JsValue> =
                        target.query_selector_all("[data_index]")
                    else {
                        continue;
                    };
                    let total_count: u32 = children.length();
                    let estimated_visible: u32 =
                        (intersection_ratio * total_count as f64).ceil() as u32;
                    Console::log(format!(
                        "[IntersectionObserver] <{}> intersection_ratio={:.2}, total_items={}, estimated_visible_items={}",
                        tag_name, intersection_ratio, total_count, estimated_visible
                    ));
                }
            }
        }
    }));
    let Ok(observer): Result<IntersectionObserver, JsValue> =
        IntersectionObserver::new(callback.as_ref().unchecked_ref())
    else {
        return;
    };
    let _: Result<bool, JsValue> = Reflect::set(&window_value, &observer_key, observer.as_ref());
    callback.forget();
    if let Some(container_element) = document_value.query_selector(selector).ok().flatten() {
        observer.observe(&container_element);
    }
}

/// Schedules a `bind_observer` call via `queueMicrotask` so that the DOM
/// is fully updated before querying elements.
///
/// Because `use_intersection_observer` is called during the component render
/// callback (inside `dispatch_updates`), the DOM has not yet been patched when
/// this function runs.  `queueMicrotask` fires after the current `dispatch_updates`
/// microtask completes — at that point `render_full_replace` has already inserted
/// the new DOM nodes, so `query_selector` can find the container element.
///
/// Uses a guard flag (`window.__euv_observer_pending`) to debounce
/// multiple schedule requests into a single microtask callback.
///
/// # Arguments
///
/// - `String` - A CSS selector string to identify the elements to observe.
fn schedule_bind_observer(selector: String) {
    let pending_key: JsValue = JsValue::from_str(OBSERVER_PENDING_KEY);
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    if !Reflect::get(&window_value, &pending_key)
        .unwrap_or(JsValue::UNDEFINED)
        .is_undefined()
    {
        return;
    }
    let _: Result<bool, JsValue> = Reflect::set(&window_value, &pending_key, &JsValue::TRUE);
    let microtask_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let Some(window_value): Option<Window> = window() else {
            return;
        };
        let key: JsValue = JsValue::from_str(OBSERVER_PENDING_KEY);
        let _: Result<bool, JsValue> = Reflect::set(&window_value, &key, &JsValue::UNDEFINED);
        bind_observer(&selector);
    }));
    window_value.queue_microtask(microtask_closure.as_ref().unchecked_ref::<Function>());
    microtask_closure.forget();
}

/// Observes the container element matching the given CSS selector for viewport
/// intersection changes.
///
/// On first call, schedules an initial binding via `queueMicrotask` so
/// that the DOM is fully updated before querying elements.
/// Only the container itself is observed (not individual children) to avoid
/// O(N) overhead with large lists.
///
/// A guard flag (`window.__euv_observer_listener`) ensures the initial
/// `queueMicrotask` is only registered once.
///
/// The observer is automatically disconnected when the hook context is cleared
/// (i.e., when the component unmounts or a `match` arm switches).
///
/// # Arguments
///
/// - `&str` - A CSS selector string to identify the container element to observe.
pub(crate) fn use_intersection_observer(selector: &str) {
    let init_selector: String = selector.to_string();
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let listener_key: JsValue = JsValue::from_str("__euv_observer_listener");
    if Reflect::get(&window_value, &listener_key)
        .unwrap_or(JsValue::UNDEFINED)
        .is_undefined()
    {
        let _: Result<bool, JsValue> = Reflect::set(&window_value, &listener_key, &JsValue::TRUE);
        schedule_bind_observer(init_selector);
    }
    App::use_cleanup(move || {
        let window_value: Window = match window() {
            Some(window_instance) => window_instance,
            None => return,
        };
        let observer_key: JsValue = JsValue::from_str("__euv_observer_instance");
        let listener_key: JsValue = JsValue::from_str("__euv_observer_listener");
        if let Some(observer) = Reflect::get(&window_value, &observer_key)
            .ok()
            .and_then(|value: JsValue| value.dyn_into::<IntersectionObserver>().ok())
        {
            observer.disconnect();
        }
        let _: Result<bool, JsValue> =
            Reflect::set(&window_value, &observer_key, &JsValue::UNDEFINED);
        let _: Result<bool, JsValue> =
            Reflect::set(&window_value, &listener_key, &JsValue::UNDEFINED);
        let pending_key: JsValue = JsValue::from_str(OBSERVER_PENDING_KEY);
        let _: Result<bool, JsValue> =
            Reflect::set(&window_value, &pending_key, &JsValue::UNDEFINED);
    });
}
