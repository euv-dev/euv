use super::*;

/// Implementation of route configuration construction.
impl EuvRouteConfig {
    /// Creates a new route configuration.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The route path.
    /// - `F: Fn() -> VirtualNode + 'static` - The component function.
    ///
    /// # Returns
    ///
    /// - `EuvRouteConfig` - The route configuration.
    pub fn new<F>(path: &'static str, component: F) -> Self
    where
        F: Fn() -> VirtualNode + 'static,
    {
        Self {
            path,
            component: Rc::new(component),
        }
    }
}

/// Implementation of router navigation and viewport utilities.
impl Router {
    /// Reads the current hash-based route from the browser URL.
    ///
    /// # Returns
    ///
    /// - `String` - The hash fragment without the leading `#`, or `DEFAULT_ROUTE_PATH` if empty.
    pub fn current_route() -> String {
        let Some(window) = window() else {
            return String::new();
        };
        let hash: String = window.location().hash().unwrap_or_default();
        let route: String = hash
            .strip_prefix(ROUTE_HASH_PREFIX)
            .unwrap_or(&hash)
            .to_string();
        if route.is_empty() {
            DEFAULT_ROUTE_PATH.to_string()
        } else {
            route
        }
    }

    /// Navigates to a new hash-based route.
    ///
    /// Always defers the actual `location.set_hash()` call to `queueMicrotask`
    /// to prevent synchronous `hashchange` dispatch while any caller frame is still
    /// on the stack. This avoids wasm_bindgen's `"closure invoked recursively
    /// or after being dropped"` error which occurs when `set_hash()` fires
    /// `hashchange` synchronously and the handler (or the reactive update chain
    /// it triggers) calls `navigate()` again before the original dispatch finishes.
    ///
    /// When the target route matches the current route, the call is a no-op —
    /// the route signal subscriber ignores equal values (`Signal::set` short-
    /// circuits on `PartialEq`), but the underlying `set_hash` would still push
    /// a duplicate history entry, fire `hashchange`, and re-run the route
    /// subscriber's re-render path. Skipping the call avoids all of that.
    ///
    /// Multiple rapid `navigate()` calls before the microtask fires are coalesced:
    /// only the **last** target route wins, as earlier routes were superseded by
    ///
    /// # Arguments
    ///
    /// - `R: AsRef<str>` - The target route path.
    pub fn navigate<R>(route: R)
    where
        R: AsRef<str>,
    {
        let route_string: String = route.as_ref().to_string();
        // Cheap early-return: a navigate() to the route the user is already on
        // would otherwise queue a microtask, mutate `location.hash`, dispatch
        // `hashchange`, and run the route subscriber's patch path — for no
        // observable effect. Strips the optional `#anchor` suffix so callers
        // can compare against the URL hash form without normalizing themselves.
        let current: String = Self::current_route();
        let target_path: &str = match route_string.find('#') {
            Some(idx) => &route_string[..idx],
            None => &route_string,
        };
        if current == target_path {
            return;
        }
        DEFERRED_NAVIGATION.with(|cell: &Cell<Option<String>>| cell.set(Some(route_string)));
        let deferred_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let target_route: Option<String> =
                DEFERRED_NAVIGATION.with(|cell: &Cell<Option<String>>| cell.take());
            if let Some(route_value) = target_route {
                let Some(nav_window) = web_sys::window() else {
                    return;
                };
                let nav_location: Location = nav_window.location();
                let nav_new_hash: String = format!("{ROUTE_HASH_PREFIX}{route_value}");
                let _: Result<(), JsValue> = nav_location.set_hash(&nav_new_hash);
            }
        }));
        let Some(window) = window() else {
            return;
        };
        window.queue_microtask(deferred_closure.as_ref().unchecked_ref::<Function>());
        deferred_closure.forget();
    }

    /// Creates a link click handler that navigates to the given route.
    ///
    /// Calls `event.prevent_default()` to prevent the `<a>` element's
    /// default hash navigation, then programmatically navigates via
    /// `navigate()`. Without `preventDefault`, both the `<a href>` default
    /// behavior and `navigate()` would fire, potentially creating duplicate
    /// history entries and causing incorrect browser back/forward behavior.
    ///
    /// # Arguments
    ///
    /// - `R: AsRef<str>` - The target route path.
    ///
    /// # Returns
    ///
    /// - `NativeEventHandler` - An event handler for click events.
    pub fn link_handler<R>(route: R) -> NativeEventHandler
    where
        R: AsRef<str>,
    {
        let route_string: String = route.as_ref().to_string();
        NativeEventHandler::create("click", move |event: Event| {
            event.prevent_default();
            Self::navigate(&route_string);
        })
    }

    /// Checks whether the current viewport width qualifies as a mobile device.
    ///
    /// Uses `MOBILE_BREAKPOINT` (768px) as the threshold.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the viewport width is less than the mobile breakpoint.
    pub fn is_mobile() -> bool {
        let Some(window) = window() else {
            return false;
        };
        let width: f64 = window
            .inner_width()
            .ok()
            .map(|value: JsValue| Number::from(value).value_of())
            .unwrap_or_default();
        width < MOBILE_BREAKPOINT as f64
    }
}
