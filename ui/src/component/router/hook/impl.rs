use super::*;

/// Implementation of router functionality.
///
/// Provides methods for managing browser history, overlays, navigation,
/// and scroll behavior.
impl Router {
    /// Watches the route signal and scrolls the `<main>` content container
    /// back to the top whenever the route changes.
    ///
    /// On each route change, queries the document for the first `<main>`
    /// element and resets its `scrollTop` to zero. The sidebar scroll
    /// position is preserved natively since the `<nav>` element is never
    /// destroyed during route transitions.
    ///
    /// # Arguments
    ///
    /// - `Signal<String>` - The reactive signal holding the current route path.
    pub fn use_scroll_to_top(route_signal: Signal<String>) {
        watch!(route_signal, |_: String| {
            let Some(window_value) = window() else {
                return;
            };
            let Some(document_value) = window_value.document() else {
                return;
            };
            if let Some(main_element) = document_value.query_selector("main").ok().flatten() {
                let html_element: HtmlElement = main_element.unchecked_into();
                html_element.set_scroll_top(0);
            }
        });
    }

    /// Subscribes to browser `hashchange` events and updates the given signal.
    ///
    /// Registers a global event listener on `window` that reads the current
    /// route on every hash change and writes it into the provided signal.
    /// The listener is automatically removed when the hook context is cleared.
    ///
    /// Increments `WINDOW_EVENT_DEPTH` before dispatching and decrements it
    /// after, so that any code that checks re-entrancy can detect that it is
    /// running within a window event handler context.
    ///
    /// Note: `navigate()` always defers `set_hash()` to a microtask, so by the
    /// time the `hashchange` fires, all caller frames have already unwound and
    /// there is no risk of recursive Closure invocation. The handler only needs
    /// to update the route signal with the current URL hash value.
    ///
    /// # Arguments
    ///
    /// - `Signal<String>` - The reactive signal that holds the current route and will be updated on each hash change.
    pub fn use_hash_change(route_signal: Signal<String>) {
        App::use_window_event("hashchange", move || {
            WINDOW_EVENT_DEPTH.with(|depth: &Cell<usize>| depth.set(depth.get() + 1));
            route_signal.set(Self::current_route());
            WINDOW_EVENT_DEPTH.with(|depth: &Cell<usize>| depth.set(depth.get() - 1));
        });
    }

    /// Manages browser history for all overlays (modals, panels, drawers) so that
    /// the back button closes the most recently opened overlay instead of navigating away.
    ///
    /// Uses a unified `OVERLAY_STACK` that records every overlay in the order it was opened.
    /// A `popstate` listener pops the topmost entry and invokes its close callback, so
    /// overlays close in reverse opening order regardless of type.
    ///
    /// Before consulting the overlay stack, the listener iterates over all registered
    /// `popstate` guards (see [`register_popstate_guard`]). The first guard that returns
    /// `true` consumes the event, preventing the overlay stack and normal navigation
    /// from processing it.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The reactive signal controlling the nav drawer visibility.
    /// - `Signal<bool>` - The reactive signal tracking whether the viewport is mobile-sized.
    pub fn use_overlay_history(drawer_open: Signal<bool>, mobile_signal: Signal<bool>) {
        let was_drawer_open: Signal<bool> = App::use_signal(|| false);
        watch!(drawer_open, |is_open: bool| {
            let previous: bool = was_drawer_open.get();
            if is_open && !previous && mobile_signal.get() {
                let closer: Rc<dyn Fn()> = Rc::new(move || {
                    drawer_open.set(false);
                });
                Self::overlay_stack_push(closer);
            }
            was_drawer_open.set(is_open);
        });
        App::use_window_event("popstate", move || {
            WINDOW_EVENT_DEPTH.with(|depth: &Cell<usize>| depth.set(depth.get() + 1));
            let consumed: bool = POPSTATE_GUARDS.with(|guards: &PopstateGuardList| {
                guards
                    .borrow()
                    .iter()
                    .any(|entry: &PopstateGuardEntry| entry.1())
            });
            if consumed {
                WINDOW_EVENT_DEPTH.with(|depth: &Cell<usize>| depth.set(depth.get() - 1));
                return;
            }
            if BACK_PENDING.with(|flag: &Cell<bool>| flag.get()) {
                BACK_PENDING.with(|flag: &Cell<bool>| flag.set(false));
                let pending_route: Option<String> =
                    NAVIGATE_AFTER_BACK.with(|cell: &Cell<Option<String>>| cell.take());
                if let Some(closer) = Self::overlay_stack_pop() {
                    closer();
                }
                if let Some(route) = pending_route {
                    Self::navigate(&route);
                }
                WINDOW_EVENT_DEPTH.with(|depth: &Cell<usize>| depth.set(depth.get() - 1));
                return;
            }
            if let Some(closer) = Self::overlay_stack_pop() {
                closer();
                WINDOW_EVENT_DEPTH.with(|depth: &Cell<usize>| depth.set(depth.get() - 1));
                return;
            }
            WINDOW_EVENT_DEPTH.with(|depth: &Cell<usize>| depth.set(depth.get() - 1));
        });
    }

    /// Watches the drawer open signal and scrolls the mobile navigation drawer
    /// to make the currently active navigation item visible when the drawer opens.
    ///
    /// Uses nested `requestAnimationFrame` to defer the scroll until after the
    /// framework has completed its DOM update cycle. The first `requestAnimationFrame`
    /// fires after the framework's own `requestAnimationFrame`-based render pass,
    /// and the second one fires after the browser has laid out the new DOM.
    /// Locates the scrollable `c-nav-items-scroll` container and the active nav
    /// item within the drawer, then sets `scrollTop` so the active item appears
    /// near the vertical center of the container.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The reactive signal controlling the mobile nav drawer visibility.
    pub fn use_scroll_drawer_to_active(drawer_open: Signal<bool>) {
        watch!(drawer_open, |is_open: bool| {
            if !is_open {
                return;
            }
            let Some(window_value) = window() else {
                return;
            };
            let outer_raf: Window = window_value.clone();
            let inner_raf_clone: Window = window_value.clone();
            let inner_doc_clone: Window = window_value.clone();
            let outer_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                let inner_raf: Window = inner_raf_clone.clone();
                let inner_doc: Window = inner_doc_clone.clone();
                let inner_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                    let Some(document_value) = inner_doc.document() else {
                        return;
                    };
                    let Some(drawer_nav) = document_value
                        .query_selector(DRAWER_NAV_SELECTOR)
                        .ok()
                        .flatten()
                    else {
                        return;
                    };
                    let Some(active_element) = drawer_nav
                        .query_selector(ACTIVE_NAV_ITEM_SELECTOR)
                        .ok()
                        .flatten()
                    else {
                        return;
                    };
                    let active_html_element: HtmlElement = active_element.unchecked_into();
                    let Some(scroll_container) = drawer_nav
                        .query_selector(NAV_ITEMS_SCROLL_SELECTOR)
                        .ok()
                        .flatten()
                    else {
                        return;
                    };
                    let scroll_html_element: HtmlElement = scroll_container.unchecked_into();
                    let active_rect: DomRect = active_html_element.get_bounding_client_rect();
                    let container_rect: DomRect = scroll_html_element.get_bounding_client_rect();
                    let offset_from_container_top: f64 = active_rect.top() - container_rect.top();
                    let current_scroll_top: i32 = scroll_html_element.scroll_top();
                    let container_height: f64 = container_rect.height();
                    let active_height: f64 = active_rect.height();
                    let target_scroll_top: f64 = current_scroll_top as f64
                        + offset_from_container_top
                        - (container_height - active_height) / 2.0;
                    scroll_html_element.set_scroll_top(target_scroll_top.max(0.0) as i32);
                }));
                let _: Result<i32, JsValue> =
                    inner_raf.request_animation_frame(inner_closure.as_ref().unchecked_ref());
                inner_closure.forget();
            }));
            let _: Result<i32, JsValue> =
                outer_raf.request_animation_frame(outer_closure.as_ref().unchecked_ref());
            outer_closure.forget();
        });
    }

    /// Registers a `popstate` guard callback that is invoked on every `popstate`
    /// event before the overlay stack is consulted.
    ///
    /// Guards are called in registration order. The first guard that returns `true`
    /// consumes the `popstate` event, preventing the overlay stack and normal
    /// navigation from processing it. This allows external modules (e.g. native
    /// fullscreen, canvas fullscreen) to intercept the system back gesture without
    /// registering their own independent `popstate` listener.
    ///
    /// Returns a guard ID that can be passed to [`Router::unregister_popstate_guard`] to
    /// remove the guard when it is no longer needed.
    ///
    /// # Arguments
    ///
    /// - `Rc<dyn Fn() -> bool>` - The guard callback. Return `true` to consume the
    ///   `popstate` event, `false` to let subsequent guards or the overlay stack
    ///   handle it.
    ///
    /// # Returns
    ///
    /// - `usize` - A unique guard ID for later unregistration.
    pub fn register_popstate_guard(guard: Rc<dyn Fn() -> bool>) -> usize {
        NEXT_POPSTATE_GUARD_ID.with(|counter: &Cell<usize>| {
            let id: usize = counter.get();
            counter.set(id + 1);
            POPSTATE_GUARDS.with(|guards: &PopstateGuardList| {
                guards.borrow_mut().push((id, guard));
            });
            id
        })
    }

    /// Pushes a browser history entry for an overlay that is about to open.
    ///
    /// Call this when an overlay (vconsole panel) opens so that the browser
    /// back button will close the overlay instead of navigating away.
    pub fn overlay_push_state() {
        let Some(window) = window() else {
            return;
        };
        let Ok(history) = window.history() else {
            return;
        };
        let _: Result<(), JsValue> = history.push_state(&JsValue::NULL, "");
    }

    /// Performs a programmatic `history.back()` to consume the overlay's
    /// history entry, optionally scheduling a navigation to run after the
    /// `popstate` event fires.
    ///
    /// # Arguments
    ///
    /// - `Option<String>` - An optional route to navigate to after the back completes.
    pub fn overlay_back(navigate_target: Option<String>) {
        BACK_PENDING.with(|flag: &Cell<bool>| flag.set(true));
        if let Some(ref route) = navigate_target {
            NAVIGATE_AFTER_BACK.with(|cell: &Cell<Option<String>>| cell.set(Some(route.clone())));
        }
        let Some(window) = window() else {
            return;
        };
        let Ok(history) = window.history() else {
            return;
        };
        let _: Result<(), JsValue> = history.back();
    }

    /// Pushes an overlay close callback onto the unified `OVERLAY_STACK` and
    /// pushes a browser history entry so the back button dismisses it.
    ///
    /// Call this whenever any overlay (modal, panel, or drawer) opens.
    ///
    /// # Arguments
    ///
    /// - `Rc<dyn Fn()>` - The callback that closes the overlay (e.g., sets its visibility signal to `false`).
    pub(crate) fn overlay_stack_push(closer: Rc<dyn Fn()>) {
        OVERLAY_STACK.with(|stack: &OverlayStack| {
            stack.borrow_mut().push(OverlayEntry { closer });
        });
        Self::overlay_push_state();
    }

    /// Pops the most recently opened overlay from the unified `OVERLAY_STACK` and
    /// returns its close callback, without invoking it.
    ///
    /// Also synchronizes the `MODAL_STACK` by removing the matching entry if the
    /// popped overlay is a modal.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn()>>` - The topmost overlay's close callback, or `None` if no overlay is open.
    pub(crate) fn overlay_stack_pop() -> Option<Rc<dyn Fn()>> {
        let closer: Option<Rc<dyn Fn()>> = OVERLAY_STACK.with(|stack: &OverlayStack| {
            stack
                .borrow_mut()
                .pop()
                .map(|entry: OverlayEntry| entry.closer)
        });
        if let Some(ref closer_ref) = closer {
            MODAL_STACK.with(|stack: &ModalStack| {
                let mut entries: RefMut<'_, Vec<ModalStackEntry>> = stack.borrow_mut();
                if let Some(index) = entries
                    .iter()
                    .rposition(|(_, closer): &ModalStackEntry| Rc::ptr_eq(closer, closer_ref))
                {
                    entries.remove(index);
                }
            });
        }
        closer
    }

    /// Closes the most recently opened overlay via the UI and consumes one
    /// browser history entry.
    ///
    /// Triggers `overlay_back`, which sets the `BACK_PENDING` flag and calls
    /// `history.back()`. The resulting `popstate` handler invocation pops the
    /// top entry from `OVERLAY_STACK` and runs its close callback, keeping the
    /// history count in sync. Use this when the user dismisses an overlay
    /// through a close button, overlay click, or confirm/cancel action.
    ///
    /// Note: this method does **not** pop `OVERLAY_STACK` itself — the popstate
    /// handler is the single owner of the pop, so UI dismissal and the system
    /// back gesture share one consistent path.
    pub fn overlay_stack_close() {
        Self::overlay_back(None);
    }

    /// Registers an open modal by pushing it onto the global modal stack and
    /// adding a browser history entry, enabling nested modals.
    ///
    /// The stack is ordered with the most recently opened modal on top. When the
    /// user triggers a system back gesture (or presses the browser back button),
    /// the `popstate` handler pops the topmost entry from `OVERLAY_STACK` and
    /// invokes its close callback, so the most recently opened overlay is dismissed
    /// first instead of navigating to the previous page.
    ///
    /// If the given visibility signal is already on the stack, this is a no-op so
    /// that re-opening an already-open modal does not create duplicate stack or
    /// history entries.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The modal's visibility signal, used as a stable identity for later removal.
    /// - `Rc<dyn Fn()>` - The callback that closes the modal (e.g., sets the visibility signal to `false`).
    pub fn modal_push(visible: Signal<bool>, closer: Rc<dyn Fn()>) {
        let already_open: bool = MODAL_STACK.with(|stack: &ModalStack| {
            stack
                .borrow()
                .iter()
                .any(|(signal, _): &ModalStackEntry| *signal == visible)
        });
        if already_open {
            return;
        }
        MODAL_STACK.with(|stack: &ModalStack| stack.borrow_mut().push((visible, closer.clone())));
        Self::overlay_stack_push(closer);
    }

    /// Closes a modal that was opened via [`Router::modal_push`] when the user dismisses
    /// it through the UI (close button, overlay click, confirm/cancel action)
    /// rather than the system back gesture.
    ///
    /// Removes the entry matching the given visibility signal from the global
    /// stack (by identity, not necessarily the top, so nested modals stay
    /// consistent) and consumes one matching browser history entry via
    /// `overlay_stack_close`, keeping the history count in sync so a subsequent back
    /// gesture behaves correctly.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The visibility signal identifying the modal to remove.
    pub fn modal_close_via_ui(visible: Signal<bool>) {
        let removed: bool = MODAL_STACK.with(|stack: &ModalStack| {
            let mut entries: RefMut<'_, Vec<ModalStackEntry>> = stack.borrow_mut();
            if let Some(index) = entries
                .iter()
                .rposition(|(signal, _): &ModalStackEntry| *signal == visible)
            {
                entries.remove(index);
                true
            } else {
                false
            }
        });
        if removed {
            Self::overlay_stack_close();
        }
    }

    /// Opens the given URL in the system default browser using `window.open`
    /// with the `_system` target name.
    ///
    /// In a bridge WebView environment, the `_system` target instructs the
    /// shell opener plugin to delegate the URL to the operating system's
    /// default browser. In a regular browser, `window.open` falls back to
    /// opening a new tab or window as usual.
    ///
    /// # Arguments
    ///
    /// - `U: AsRef<str>` - The URL to open.
    pub fn open_system_browser<U>(url: U)
    where
        U: AsRef<str>,
    {
        let Some(window_value) = window() else {
            return;
        };
        if let Ok(open_fn) = Reflect::get(&window_value, &JsValue::from_str("open"))
            .and_then(|value: JsValue| value.dyn_into::<Function>())
        {
            let _: Result<JsValue, JsValue> = open_fn.call2(
                &window_value,
                &JsValue::from_str(url.as_ref()),
                &JsValue::from_str(SYSTEM_BROWSER_TARGET),
            );
        }
    }

    /// Creates a click event handler for external `<a>` links that opens
    /// the URL in the system default browser.
    ///
    /// Calls `event.prevent_default()` to suppress the `<a>` element's
    /// default navigation (which would open inside the WebView), then
    /// delegates to `open_system_browser` so the URL is handled by the
    /// operating system's default browser.
    ///
    /// # Arguments
    ///
    /// - `U: AsRef<str>` - The external URL to open on click.
    ///
    /// # Returns
    ///
    /// - `NativeEventHandler` - An event handler for click events.
    pub fn external_link_handler<U>(url: U) -> NativeEventHandler
    where
        U: AsRef<str>,
    {
        let url_string: String = url.as_ref().to_string();
        NativeEventHandler::create("click", move |event: Event| {
            event.prevent_default();
            Self::open_system_browser(&url_string);
        })
    }

    /// Helper to close the drawer and navigate.
    ///
    /// Used internally by mobile nav items.
    /// Closes the drawer via overlay back and schedules navigation to the target route
    /// after the popstate event is processed.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The drawer open signal.
    /// - `T: AsRef<str>` - The target route.
    pub fn close_drawer_and_navigate<T>(_drawer_open: Signal<bool>, target: T)
    where
        T: AsRef<str>,
    {
        Self::overlay_back(Some(target.as_ref().to_string()));
    }
}
