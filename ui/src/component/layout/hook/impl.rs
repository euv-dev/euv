use super::*;

/// Implementation of layout functionality.
///
/// Provides methods for managing viewport resize, drawer toggle, and safe area.
impl UseEuvLayout {
    /// Creates a reactive signal that tracks whether the viewport is in mobile mode
    /// and subscribes to browser `resize` events to keep it updated.
    ///
    /// The resize handler is debounced by `RESIZE_DEBOUNCE_MILLIS` (16ms) to avoid
    /// excessive recomputation during continuous resize operations.
    /// The listener is automatically removed when the hook context is cleared.
    ///
    /// # Returns
    ///
    /// - `Signal<bool>` - A reactive signal that is `true` when the viewport is mobile-sized.
    pub fn use_resize() -> Signal<bool> {
        let mobile_signal: Signal<bool> = App::use_signal(Router::is_mobile);
        let timer_signal: Signal<Option<i32>> = App::use_signal(|| None);
        let debounce_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let mobile: bool = Router::is_mobile();
            mobile_signal.set(mobile);
        }));
        let debounce_callback: Function = debounce_closure
            .as_ref()
            .unchecked_ref::<Function>()
            .clone();
        debounce_closure.forget();
        let Some(timeout_window) = window() else {
            return mobile_signal;
        };
        App::use_window_event(WINDOW_EVENT_RESIZE, move || {
            let old_timer: Option<i32> = timer_signal.get();
            if let Some(timer_id) = old_timer {
                timeout_window.clear_timeout_with_handle(timer_id);
            }
            let new_timer: i32 = timeout_window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    &debounce_callback,
                    RESIZE_DEBOUNCE_MILLIS,
                )
                .unwrap_or_default();
            timer_signal.set(Some(new_timer));
        });
        mobile_signal
    }

    /// Creates a click event handler that toggles the mobile nav drawer signal
    /// with proper browser history management.
    ///
    /// When toggling from open to closed, calls `overlay_back` to remove the
    /// extra history entry that was pushed when the drawer opened. When toggling
    /// from closed to open, the `use_overlay_history` hook handles the
    /// `pushState` call automatically.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The boolean signal controlling the drawer visibility.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click event handler that toggles the drawer.
    pub fn use_drawer_toggle(drawer_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let is_open: bool = drawer_open.get();
            if is_open {
                Router::overlay_stack_close();
            }
            drawer_open.set(!is_open);
        }))
    }

    /// Registers global event listeners that preserve `env(safe-area-inset-*)`
    /// values after exiting any type of fullscreen on Android, and ensures that
    /// the system back button exits native fullscreen instead of navigating away.
    ///
    /// On initialisation, reads the current `env(safe-area-inset-*)` pixel values
    /// through a sentinel `<div>` and caches them in thread-local storage.
    /// When a `fullscreenchange` or `resize` event fires, the cached values are
    /// written directly as inline CSS custom properties on the real app root
    /// element so that layout never depends on the potentially stale `env()`
    /// function result.
    ///
    /// When a native (browser) fullscreen is entered — for example the user taps
    /// the fullscreen button on a `<video controls>` element — a browser history
    /// entry is added via `overlay_push_state` so that the system back gesture
    /// will fire `popstate`. A `popstate` guard registered via
    /// [`Router::register_popstate_guard`] then calls `document.exitFullscreen()` to leave
    /// fullscreen, consuming the history entry without navigating to the previous
    /// route. When the native fullscreen is exited through other means (e.g. the
    /// browser's own exit button), the `fullscreenchange` handler consumes the
    /// extra history entry via `overlay_back`.
    ///
    /// This hook should be called once during app initialization and covers:
    /// - Native video fullscreen → exit via system back button
    /// - CSS simulated fullscreen → exit (canvas drawing mode)
    /// - Any future fullscreen scenarios
    pub fn use_safe_area_fix() {
        Self::cache_safe_area_insets();
        Self::init_safe_area_contract();
        App::use_window_event(WINDOW_EVENT_FULLSCREEN_CHANGE, || {
            let Some(window_value) = window() else {
                return;
            };
            let Some(document_value) = window_value.document() else {
                return;
            };
            let is_fullscreen: bool = document_value.fullscreen_element().is_some();
            if is_fullscreen {
                NATIVE_FULLSCREEN_ACTIVE.with(|flag: &Cell<bool>| flag.set(true));
                Router::overlay_push_state();
            } else {
                let was_active: bool =
                    NATIVE_FULLSCREEN_ACTIVE.with(|flag: &Cell<bool>| flag.get());
                if was_active {
                    NATIVE_FULLSCREEN_ACTIVE.with(|flag: &Cell<bool>| flag.set(false));
                    let exit_by_popstate: bool =
                        NATIVE_FULLSCREEN_EXIT_BY_POPSTATE.with(|flag: &Cell<bool>| flag.get());
                    if exit_by_popstate {
                        NATIVE_FULLSCREEN_EXIT_BY_POPSTATE
                            .with(|flag: &Cell<bool>| flag.set(false));
                    } else {
                        Router::overlay_back(None);
                    }
                }
                Self::apply_cached_insets();
            }
        });
        App::use_window_event(WINDOW_EVENT_WEBKIT_FULLSCREEN_CHANGE, || {
            Self::apply_cached_insets();
        });
        App::use_window_event(WINDOW_EVENT_RESIZE, || {
            Self::apply_cached_insets();
        });
        Router::register_popstate_guard(Rc::new(|| {
            if !NATIVE_FULLSCREEN_ACTIVE.with(|flag: &Cell<bool>| flag.get()) {
                return false;
            }
            NATIVE_FULLSCREEN_EXIT_BY_POPSTATE.with(|flag: &Cell<bool>| flag.set(true));
            let Some(window_value) = window() else {
                return false;
            };
            let Some(document_value) = window_value.document() else {
                return false;
            };
            document_value.exit_fullscreen();
            true
        }));
    }

    /// Publishes the measured safe-area insets as CSS contract variables when
    /// the host genuinely lays out edge-to-edge.
    ///
    /// The measured `env(safe-area-inset-*)` pixels are written to
    /// `--euv-mobile-safe-top` and `--euv-safe-{right,bottom,left}` on `<html>`,
    /// which `c_mobile_header`, `c_mobile_nav_drawer`, `c_app_root` and the
    /// edge-anchored overlays consume as `var(--euv-safe-*, 0px)`. Every one of
    /// those defaults to `0px`, so a browser host that letterboxes the page yet
    /// reports non-zero insets renders flush against the screen edge instead of
    /// reserving a blank band.
    ///
    /// Trust is decided by `is_edge_to_edge_viewport`, which combines the
    /// explicit immersive declaration with the runtime
    /// `screen.height - window.innerHeight` measurement. No platform check
    /// applies here: the top inset is injected by the euv app for an immersive
    /// Android WebView, so a host that declared itself is taken at its word.
    fn init_safe_area_contract() {
        if !is_edge_to_edge_viewport(Self::is_immersive_declared(), measure_viewport_screen_gap()) {
            return;
        }
        let top_value: String =
            SAFE_AREA_INSET_TOP.with(|cell: &RefCell<String>| cell.borrow().clone());
        if top_value.is_empty() {
            return;
        }
        Self::apply_safe_area_contract(&top_value);
    }

    /// Writes the supplied measured top inset and the cached side insets onto
    /// `<html>` as the safe-area contract variables.
    ///
    /// # Arguments
    ///
    /// - `&str` - The measured top safe-area pixel value.
    fn apply_safe_area_contract(top_value: &str) {
        let Some(window_value) = window() else {
            return;
        };
        let Some(document_value) = window_value.document() else {
            return;
        };
        let Some(root) = document_value.document_element() else {
            return;
        };
        let root_element: HtmlElement = root.unchecked_into();
        let right_value: String = safe_area_contract_value(
            true,
            &SAFE_AREA_INSET_RIGHT.with(|cell: &RefCell<String>| cell.borrow().clone()),
        );
        let bottom_value: String = safe_area_contract_value(
            true,
            &SAFE_AREA_INSET_BOTTOM.with(|cell: &RefCell<String>| cell.borrow().clone()),
        );
        let left_value: String = safe_area_contract_value(
            true,
            &SAFE_AREA_INSET_LEFT.with(|cell: &RefCell<String>| cell.borrow().clone()),
        );
        let _: Result<(), JsValue> = root_element
            .style()
            .set_property(IMMERSIVE_SAFE_TOP_PROPERTY, top_value);
        let _: Result<(), JsValue> = root_element
            .style()
            .set_property(SAFE_AREA_RIGHT_PROPERTY, &right_value);
        let _: Result<(), JsValue> = root_element
            .style()
            .set_property(SAFE_AREA_BOTTOM_PROPERTY, &bottom_value);
        let _: Result<(), JsValue> = root_element
            .style()
            .set_property(SAFE_AREA_LEFT_PROPERTY, &left_value);
    }

    /// Returns whether the host environment declares immersive (edge-to-edge)
    /// mode via `window.__EUV_IMMERSIVE__` or a
    /// `<meta name="euv-immersive" content="true">` tag.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when immersive mode is declared by the host.
    pub(crate) fn is_immersive_declared() -> bool {
        let Some(window_value) = window() else {
            return false;
        };
        let global_flag: bool = js_sys::Reflect::get(
            &window_value,
            &JsValue::from_str(IMMERSIVE_WINDOW_FLAG_PROPERTY),
        )
        .ok()
        .and_then(|value: JsValue| value.as_bool())
        .unwrap_or(false);
        if global_flag {
            return true;
        }
        window_value
            .document()
            .and_then(|document_value: Document| {
                document_value
                    .query_selector(IMMERSIVE_META_SELECTOR)
                    .ok()
                    .flatten()
            })
            .and_then(|meta: Element| meta.get_attribute(IMMERSIVE_META_CONTENT_ATTRIBUTE))
            .map(|content: String| content == IMMERSIVE_META_CONTENT_ENABLED)
            .unwrap_or(false)
    }

    /// Reads the current `env(safe-area-inset-*)` pixel values via a temporary
    /// sentinel element and persists them in thread-local storage.
    ///
    /// The sentinel `<div>` is created with `padding-top: env(safe-area-inset-top)`
    /// (and similarly for the other three sides). After forcing a layout
    /// calculation, `getComputedStyle` yields the resolved pixel value, which is
    /// then stored in `SAFE_AREA_INSET_*` thread-local cells.
    ///
    /// If the top inset is empty or `0px` (i.e. no safe area on this device or
    /// immersive mode not active), the values are not cached and no override is
    /// applied.
    fn cache_safe_area_insets() {
        let top_cached: String =
            SAFE_AREA_INSET_TOP.with(|cell: &RefCell<String>| cell.borrow().clone());
        if !top_cached.is_empty() {
            return;
        }
        let Some(win) = window() else {
            return;
        };
        let Some(document_value) = win.document() else {
            return;
        };
        let Some(body) = document_value.body() else {
            return;
        };
        let Ok(created_element) = document_value.create_element("div") else {
            return;
        };
        let sentinel: HtmlElement = created_element.unchecked_into();
        let _: Result<(), JsValue> = sentinel.style().set_property(
            SENTINEL_STYLE_POSITION_PROPERTY,
            SENTINEL_STYLE_POSITION_ABSOLUTE,
        );
        let _: Result<(), JsValue> = sentinel.style().set_property(
            SENTINEL_STYLE_VISIBILITY_PROPERTY,
            SENTINEL_STYLE_VISIBILITY_HIDDEN,
        );
        let _: Result<(), JsValue> = sentinel.style().set_property(
            SENTINEL_STYLE_POINTER_EVENTS_PROPERTY,
            SENTINEL_STYLE_POINTER_EVENTS_NONE,
        );
        let _: Result<(), JsValue> = sentinel
            .style()
            .set_property(SAFE_AREA_PADDING_TOP_PROPERTY, SAFE_AREA_PADDING_TOP_VALUE);
        let _: Result<(), JsValue> = sentinel.style().set_property(
            SAFE_AREA_PADDING_RIGHT_PROPERTY,
            SAFE_AREA_PADDING_RIGHT_VALUE,
        );
        let _: Result<(), JsValue> = sentinel.style().set_property(
            SAFE_AREA_PADDING_BOTTOM_PROPERTY,
            SAFE_AREA_PADDING_BOTTOM_VALUE,
        );
        let _: Result<(), JsValue> = sentinel.style().set_property(
            SAFE_AREA_PADDING_LEFT_PROPERTY,
            SAFE_AREA_PADDING_LEFT_VALUE,
        );
        let _: Result<Node, JsValue> = body.append_child(&sentinel);
        let Some(computed) = win.get_computed_style(&sentinel).ok().flatten() else {
            let _: Result<Node, JsValue> = body.remove_child(&sentinel);
            return;
        };
        let top_value: String = computed
            .get_property_value(SAFE_AREA_PADDING_TOP_PROPERTY)
            .unwrap_or_default();
        let right_value: String = computed
            .get_property_value(SAFE_AREA_PADDING_RIGHT_PROPERTY)
            .unwrap_or_default();
        let bottom_value: String = computed
            .get_property_value(SAFE_AREA_PADDING_BOTTOM_PROPERTY)
            .unwrap_or_default();
        let left_value: String = computed
            .get_property_value(SAFE_AREA_PADDING_LEFT_PROPERTY)
            .unwrap_or_default();
        let _: Result<Node, JsValue> = body.remove_child(&sentinel);
        if top_value.is_empty() || top_value == "0px" {
            return;
        }
        SAFE_AREA_INSET_TOP.with(|cell: &RefCell<String>| *cell.borrow_mut() = top_value);
        SAFE_AREA_INSET_RIGHT.with(|cell: &RefCell<String>| *cell.borrow_mut() = right_value);
        SAFE_AREA_INSET_BOTTOM.with(|cell: &RefCell<String>| *cell.borrow_mut() = bottom_value);
        SAFE_AREA_INSET_LEFT.with(|cell: &RefCell<String>| *cell.borrow_mut() = left_value);
    }

    /// Writes the cached safe-area inset values as inline CSS custom properties
    /// on the real app root element and any fullscreen overlay containers.
    ///
    /// Class rules such as `c_mobile_app_root`, `c_app_nav`, `c_app_main`,
    /// `c_mobile_nav_drawer`, `c_canvas_container_fullscreen`, and
    /// `c_game_container_fullscreen` consume `var(--safe-area-inset-top)`
    /// in their `padding` declarations. By overriding these CSS custom
    /// properties with inline style (which has higher specificity than the
    /// stylesheet rule from `vars!`), all `var()` references resolve to the
    /// cached pixel values, bypassing the stale `env()` function after a
    /// fullscreen exit.
    ///
    /// The canvas and game fullscreen containers are `position: fixed`
    /// and outside the app root subtree, so they do not inherit the inline
    /// overrides — they must be patched separately.
    pub fn apply_cached_insets() {
        let raw_top: String =
            SAFE_AREA_INSET_TOP.with(|cell: &RefCell<String>| cell.borrow().clone());
        let raw_right: String =
            SAFE_AREA_INSET_RIGHT.with(|cell: &RefCell<String>| cell.borrow().clone());
        let raw_bottom: String =
            SAFE_AREA_INSET_BOTTOM.with(|cell: &RefCell<String>| cell.borrow().clone());
        let raw_left: String =
            SAFE_AREA_INSET_LEFT.with(|cell: &RefCell<String>| cell.borrow().clone());
        if raw_top.is_empty()
            && raw_right.is_empty()
            && raw_bottom.is_empty()
            && raw_left.is_empty()
        {
            return;
        }
        let is_trusted: bool =
            is_edge_to_edge_viewport(Self::is_immersive_declared(), measure_viewport_screen_gap());
        let top_value: String = safe_area_contract_value(is_trusted, &raw_top);
        let right_value: String = safe_area_contract_value(is_trusted, &raw_right);
        let bottom_value: String = safe_area_contract_value(is_trusted, &raw_bottom);
        let left_value: String = safe_area_contract_value(is_trusted, &raw_left);
        let Some(window_value) = window() else {
            return;
        };
        let Some(document_value) = window_value.document() else {
            return;
        };
        let apply_to: &dyn Fn(&HtmlElement) = &|element: &HtmlElement| {
            let _: Result<(), JsValue> = element
                .style()
                .set_property(SAFE_AREA_INSET_TOP_PROPERTY, &top_value);
            let _: Result<(), JsValue> = element
                .style()
                .set_property(SAFE_AREA_INSET_RIGHT_PROPERTY, &right_value);
            let _: Result<(), JsValue> = element
                .style()
                .set_property(SAFE_AREA_INSET_BOTTOM_PROPERTY, &bottom_value);
            let _: Result<(), JsValue> = element
                .style()
                .set_property(SAFE_AREA_INSET_LEFT_PROPERTY, &left_value);
            let _: Result<(), JsValue> = element
                .style()
                .set_property(IMMERSIVE_SAFE_TOP_PROPERTY, &top_value);
            let _: Result<(), JsValue> = element
                .style()
                .set_property(SAFE_AREA_RIGHT_PROPERTY, &right_value);
            let _: Result<(), JsValue> = element
                .style()
                .set_property(SAFE_AREA_BOTTOM_PROPERTY, &bottom_value);
            let _: Result<(), JsValue> = element
                .style()
                .set_property(SAFE_AREA_LEFT_PROPERTY, &left_value);
        };
        if let Some(app_root) = document_value
            .query_selector(APP_ROOT_MOBILE_SELECTOR)
            .ok()
            .flatten()
            .map(|element: Element| element.unchecked_into::<HtmlElement>())
            .or_else(|| {
                document_value
                    .query_selector(APP_ROOT_DESKTOP_SELECTOR)
                    .ok()
                    .flatten()
                    .map(|element: Element| element.unchecked_into::<HtmlElement>())
            })
        {
            apply_to(&app_root);
        }
        if let Some(canvas_fullscreen) = document_value
            .query_selector(FULLSCREEN_CANVAS_CONTAINER_SELECTOR)
            .ok()
            .flatten()
            .map(|element: Element| element.unchecked_into::<HtmlElement>())
        {
            apply_to(&canvas_fullscreen);
        }
        if let Some(game_fullscreen) = document_value
            .query_selector(FULLSCREEN_GAME_CONTAINER_SELECTOR)
            .ok()
            .flatten()
            .map(|element: Element| element.unchecked_into::<HtmlElement>())
        {
            apply_to(&game_fullscreen);
        }
    }
}
