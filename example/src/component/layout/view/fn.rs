use super::*;

/// Renders the desktop layout with a persistent left sidebar navigation.
///
/// # Arguments
///
/// - `VirtualNode<DesktopLayoutProps>` - The typed props containing route, theme, root class, and panel signals.
///
/// # Returns
///
/// - `VirtualNode` - The desktop application shell virtual DOM tree.
#[component]
pub(crate) fn desktop_layout(node: VirtualNode<DesktopLayoutProps>) -> VirtualNode {
    let DesktopLayoutProps {
        route_signal,
        theme_signal,
        root_class_signal,
        panel_open,
    }: DesktopLayoutProps = node.try_get_props().unwrap_or_default();
    let theme_feature_signal: Signal<String> =
        App::use_signal(|| current_theme_feature().to_string());
    html! {
        div {
            class: root_class_signal
            class: c_example_theme()
            class: c_example_motion_policy()
            nav {
                class: c_app_nav()
                a {
                    href: GITHUB_URL
                    target: LINK_TARGET_BLANK
                    onclick: Router::external_link_handler(GITHUB_URL)
                    class: c_nav_header()
                    euv_logo {
                        variant: LogoButtonVariant::Nav
                    }
                    span {
                        class: c_nav_brand_title()
                        "Euv"
                    }
                }
                p {
                    class: c_nav_section_label()
                    NAV_SECTION_LABEL_PAGES
                }
                build_desktop_nav_items {
                    route_signal: route_signal
                }
                theme_feature_selector {
                    value: theme_feature_signal
                }
                div {
                    class: c_nav_theme_toggle()
                    span {
                        class: c_nav_scheme_label()
                        scheme_name()
                    }
                    button {
                        class: c_nav_theme_button()
                        "aria-label": APP_THEME_TOGGLE_LABEL
                        onclick: ThemeState::toggle(theme_signal)
                        div {
                            "aria-hidden": "true"
                            class: if { theme_signal.get() == THEME_DARK } {
                                c_theme_icon_sun()
                            } else {
                                c_theme_icon_moon()
                            }
                        }
                    }
                }
            }
            main {
                class: c_app_main()
                page_router {
                    route_signal
                }
            }
            euv_vconsole_panel {
                panel_open
            }
        }
    }
}

/// Renders the mobile layout with a top header bar and a slide-out navigation drawer.
///
/// # Arguments
///
/// - `VirtualNode<MobileLayoutProps>` - The typed props containing route, theme, root class, panel, and drawer signals.
///
/// # Returns
///
/// - `VirtualNode` - The mobile application shell virtual DOM tree.
#[component]
pub(crate) fn mobile_layout(node: VirtualNode<MobileLayoutProps>) -> VirtualNode {
    let MobileLayoutProps {
        route_signal,
        theme_signal,
        root_class_signal,
        panel_open,
        drawer_open,
    }: MobileLayoutProps = node.try_get_props().unwrap_or_default();
    let theme_feature_signal: Signal<String> =
        App::use_signal(|| current_theme_feature().to_string());
    let feature_signal_for_change: Signal<String> = theme_feature_signal;
    let on_theme_feature_change: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        let selected_feature: Option<String> = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<HtmlSelectElement>().ok())
            .map(|select: HtmlSelectElement| select.value());
        if let Some(feature) = selected_feature {
            feature_signal_for_change.set(feature.clone());
            navigate_to_theme_feature(&feature);
        }
    }));
    let on_overlay_click: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        Router::overlay_stack_close();
        drawer_open.set(false);
    });
    let on_drawer_close_click: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        Router::overlay_stack_close();
        drawer_open.set(false);
    });
    html! {
        div {
            class: root_class_signal
            class: c_example_theme()
            class: c_example_motion_policy()
            header {
                class: c_example_mobile_header()
                div {
                    class: c_example_mobile_header_left()
                    button {
                        class: if { drawer_open.get() } {
                            c_mobile_menu_button_active()
                        } else {
                            c_mobile_menu_button()
                        }
                        onclick: UseEuvLayout::use_drawer_toggle(drawer_open)
                        "☰"
                    }
                    a {
                        href: GITHUB_URL
                        target: LINK_TARGET_BLANK
                        onclick: Router::external_link_handler(GITHUB_URL)
                        class: c_mobile_header_logo()
                        euv_logo {
                            variant: LogoButtonVariant::Nav
                        }
                        span {
                            class: c_nav_brand_title()
                            "Euv"
                        }
                    }
                }
                div {
                    class: c_mobile_header_actions()
                    div {
                        class: c_mobile_theme_feature_selector()
                        euv_native_select {
                            size: EuvNativeSelectSize::Sm
                            id: THEME_FEATURE_SELECTOR_ID
                            aria_label: THEME_FEATURE_SELECTOR_LABEL.to_string()
                            value: theme_feature_signal
                            onchange: on_theme_feature_change
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[0].0
                                label: THEME_FEATURE_OPTIONS[0].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[1].0
                                label: THEME_FEATURE_OPTIONS[1].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[2].0
                                label: THEME_FEATURE_OPTIONS[2].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[3].0
                                label: THEME_FEATURE_OPTIONS[3].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[4].0
                                label: THEME_FEATURE_OPTIONS[4].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[5].0
                                label: THEME_FEATURE_OPTIONS[5].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[6].0
                                label: THEME_FEATURE_OPTIONS[6].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[7].0
                                label: THEME_FEATURE_OPTIONS[7].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[8].0
                                label: THEME_FEATURE_OPTIONS[8].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[9].0
                                label: THEME_FEATURE_OPTIONS[9].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[10].0
                                label: THEME_FEATURE_OPTIONS[10].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[11].0
                                label: THEME_FEATURE_OPTIONS[11].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[12].0
                                label: THEME_FEATURE_OPTIONS[12].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[13].0
                                label: THEME_FEATURE_OPTIONS[13].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[14].0
                                label: THEME_FEATURE_OPTIONS[14].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[15].0
                                label: THEME_FEATURE_OPTIONS[15].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[16].0
                                label: THEME_FEATURE_OPTIONS[16].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[17].0
                                label: THEME_FEATURE_OPTIONS[17].0.trim_start_matches("theme-")
                            }
                            euv_native_select_option {
                                value: THEME_FEATURE_OPTIONS[18].0
                                label: THEME_FEATURE_OPTIONS[18].0.trim_start_matches("theme-")
                            }
                        }
                    }
                    button {
                        class: c_mobile_theme_button()
                        "aria-label": APP_THEME_TOGGLE_LABEL
                        onclick: ThemeState::toggle(theme_signal)
                        div {
                            "aria-hidden": "true"
                            class: if { theme_signal.get() == THEME_DARK } {
                                c_theme_icon_sun()
                            } else {
                                c_theme_icon_moon()
                            }
                        }
                    }
                }
            }
            main {
                class: c_mobile_main()
                page_router {
                    route_signal
                }
            }
            euv_vconsole_panel {
                panel_open
            }
            div {
                class: if { drawer_open.get() } {
                    c_mobile_overlay().to_string()
                } else {
                    format!("{} {}", c_mobile_overlay().get_name(), c_mobile_overlay_hidden().get_name())
                }
                onclick: on_overlay_click
            }
            nav {
                class: if { drawer_open.get() } {
                    c_mobile_nav_drawer().to_string()
                } else {
                    format!("{} {}", c_mobile_nav_drawer().get_name(), c_mobile_nav_drawer_closed().get_name())
                }
                div {
                    class: c_mobile_nav_drawer_header()
                    div {
                        class: c_mobile_header_logo()
                        a {
                            href: GITHUB_URL
                            target: LINK_TARGET_BLANK
                            onclick: Router::external_link_handler(GITHUB_URL)
                            class: c_mobile_header_logo()
                            euv_logo {
                                variant: LogoButtonVariant::Nav
                            }
                            span {
                                class: c_nav_brand_title()
                                "Euv"
                            }
                        }
                    }
                    button {
                        class: c_mobile_drawer_close_button()
                        onclick: on_drawer_close_click
                        "✕"
                    }
                }
                p {
                    class: c_nav_section_label()
                    NAV_SECTION_LABEL_PAGES
                }
                build_mobile_nav_items {
                    route_signal: route_signal
                    drawer_open: drawer_open
                }
            }
        }
    }
}

/// Renders the sidebar selector for every independently compiled theme variant.
///
/// # Arguments
///
/// - `VirtualNode<ThemeFeatureSelectorProps>` - The current build's feature value.
///
/// # Returns
///
/// - `VirtualNode` - The labelled selector that reloads into the selected build.
#[component]
pub(crate) fn theme_feature_selector(node: VirtualNode<ThemeFeatureSelectorProps>) -> VirtualNode {
    let ThemeFeatureSelectorProps { value }: ThemeFeatureSelectorProps =
        node.try_get_props().unwrap_or_default();
    let options: Vec<EuvSelectOption> = THEME_FEATURE_OPTIONS
        .iter()
        .copied()
        .map(
            |(value, label): (&'static str, &'static str)| EuvSelectOption {
                value,
                label,
                group: "",
                separator_before: false,
                disabled: false,
            },
        )
        .collect();
    let on_change: Option<Rc<dyn Fn(&'static str)>> = Some(Rc::new(|feature: &'static str| {
        navigate_to_theme_feature(feature)
    }));
    html! {
        div {
            class: c_nav_theme_feature_selector()
            euv_select {
                value: value
                options: options
                size: EuvSelectSize::Sm
                label: THEME_FEATURE_SELECTOR_LABEL
                aria_label: THEME_FEATURE_SELECTOR_LABEL.to_string()
                id: THEME_FEATURE_SELECTOR_ID
                on_change: on_change
            }
        }
    }
}

/// Navigates to the separately built page for a selected theme feature.
///
/// # Arguments
///
/// - `&str` - The `default` selector value or a declared `theme-*` feature.
///
/// # Returns
///
/// - `()` - No value; attempts navigation and reports browser errors.
fn navigate_to_theme_feature(feature: &str) {
    let Some(window_value): Option<Window> = window() else {
        Console::error("Cannot switch theme feature without a browser window.");
        return;
    };
    let location: Location = window_value.location();
    let pathname: String = match location.pathname() {
        Ok(value) => value,
        Err(error) => {
            Console::error(format!(
                "Cannot read the current path while switching theme feature: {}",
                error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())
            ));
            return;
        }
    };
    let base_path: String = theme_feature_base_path(&pathname);
    let hash: String = location.hash().unwrap_or_default();
    let target_url: String = format!(
        "{base_path}/{THEME_FEATURES_DIRECTORY}/{feature}/{THEME_FEATURE_INDEX_FILE}{hash}"
    );
    if let Err(error) = location.set_href(&target_url) {
        Console::error(format!(
            "Failed to switch to theme feature {feature}: {}",
            error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())
        ));
    }
}

/// Returns the feature that was enabled for the current example build.
///
/// # Returns
///
/// - `&'static str` - The current `theme-*` feature, or `default` when none is enabled.
fn current_theme_feature() -> &'static str {
    if cfg!(feature = "theme-custom") {
        return THEME_FEATURE_CUSTOM;
    }
    let selected_scheme: &str = scheme_name();
    let feature: &str = THEME_FEATURE_OPTIONS
        .iter()
        .find_map(|(feature, _): &(&'static str, &'static str)| {
            (feature.strip_prefix(THEME_FEATURE_PREFIX) == Some(selected_scheme))
                .then_some(*feature)
        })
        .unwrap_or(THEME_FEATURE_DEFAULT);
    if feature == THEME_FEATURE_MONOCHROME && !cfg!(feature = "theme-monochrome") {
        return THEME_FEATURE_DEFAULT;
    }
    feature
}

/// Finds the site root shared by the root page and all variant build pages.
///
/// # Arguments
///
/// - `&str` - The current page path from `window.location.pathname`.
///
/// # Returns
///
/// - `String` - The path prefix preceding the variant directory or entry file.
fn theme_feature_base_path(pathname: &str) -> String {
    let variant_root: Option<(&str, &str)> = pathname.split_once(THEME_FEATURES_PATH_SEPARATOR);
    if let Some((root_path, _variant_path)) = variant_root {
        return root_path.to_string();
    }
    match pathname.rsplit_once('/') {
        Some((root_path, _entry_file)) => root_path.to_string(),
        None => String::new(),
    }
}

/// Asynchronously fetches the docs status with retry logic.
///
/// Attempts to fetch the crate status JSON up to `VERSION_FETCH_MAX_RETRY_COUNT` times,
/// waiting `VERSION_FETCH_RETRY_DELAY_MS` milliseconds between each retry on failure.
///
/// # Returns
///
/// - `Result<DocsStatus, ()>` - The parsed docs status on success, or an error indicator on failure.
async fn fetch_docs_status_with_retry() -> Result<DocsStatus, ()> {
    let Some(window_value): Option<Window> = window() else {
        return Err(());
    };
    let mut attempt: u32 = 0;
    loop {
        let promise: Promise = window_value.fetch_with_str(DOCS_STATUS_URL);
        match JsFuture::from(promise).await {
            Ok(response) => {
                let response_value: Response = match response.dyn_into() {
                    Ok(value) => value,
                    Err(error) => {
                        Console::error(format!(
                            "Failed to convert fetch response: {}",
                            error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())
                        ));
                        return Err(());
                    }
                };
                let text_string: String = match response_value.text() {
                    Ok(promise) => match JsFuture::from(promise).await {
                        Ok(value) => value.as_string().unwrap_or_default(),
                        Err(error) => {
                            Console::error(format!(
                                "Failed to read response text: {}",
                                error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())
                            ));
                            return Err(());
                        }
                    },
                    Err(error) => {
                        Console::error(format!(
                            "Failed to get response text promise: {}",
                            error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())
                        ));
                        return Err(());
                    }
                };
                Console::log(&text_string);
                return Ok(serde_json::from_str::<DocsStatus>(&text_string).unwrap_or_default());
            }
            Err(error) => {
                attempt += 1;
                if attempt >= VERSION_FETCH_MAX_RETRY_COUNT {
                    Console::error(format!(
                        "Failed to fetch version status after {VERSION_FETCH_MAX_RETRY_COUNT} attempts: {}",
                        error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())
                    ));
                    return Err(());
                }
                Console::warn(format!(
                    "Failed to fetch version status (attempt {attempt}/{VERSION_FETCH_MAX_RETRY_COUNT}): {}. Retrying in {VERSION_FETCH_RETRY_DELAY_MS}ms...",
                    error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string())
                ));
                sleep_ms(VERSION_FETCH_RETRY_DELAY_MS).await;
            }
        }
    }
}

/// Asynchronously checks for euv documentation updates from docs.rs.
///
/// Fetches the crate status JSON with retry logic, parses the `DocsStatus` payload,
/// and if a newer version is available, invokes the bridge `update_cache` command
/// to synchronize the local cache. The bridge invocation is itself retried up to
/// `VIEW_UPDATE_RETRY_COUNT` times (with `VIEW_UPDATE_RETRY_DELAY_MS` between
/// attempts) when the native side reports a failure or the JS-side invoke rejects.
///
/// # Returns
///
/// - `UpdateResult` - The documentation status, version, and whether an update was triggered.
async fn check_docs_update() -> UpdateResult {
    let parsed: DocsStatus = match fetch_docs_status_with_retry().await {
        Ok(status) => status,
        Err(()) => {
            return UpdateResult {
                doc_status: false,
                version: String::new(),
                updating: false,
                data: String::new(),
                message: MESSAGE_FETCH_DOCS_STATUS_FAILED.to_string(),
            };
        }
    };
    if !matches!(
        CompareVersion::compare_version(parsed.get_version(), EUV_VERSION),
        Ok(VersionLevel::Greater)
    ) {
        Console::log(format!(
            "Current version v{EUV_VERSION} is already the latest version"
        ));
        return UpdateResult {
            doc_status: parsed.get_doc_status(),
            version: parsed.get_version().clone(),
            updating: false,
            data: String::new(),
            message: MESSAGE_ALREADY_LATEST_VERSION.to_string(),
        };
    }
    if !BridgeConfig::is_available(None) {
        return UpdateResult {
            doc_status: parsed.get_doc_status(),
            version: parsed.get_version().clone(),
            updating: false,
            data: String::new(),
            message: MESSAGE_BRIDGE_UNAVAILABLE.to_string(),
        };
    }
    notify_native_with_retry(parsed.get_doc_status(), parsed.get_version().clone()).await
}

/// Invokes the bridge `update_cache` command and retries on failure.
///
/// A failure is anything other than a native-side `UpdateStatus::Success`:
/// JS-side promise rejection, native-side `UpdateStatus::Failed`, or a
/// payload that cannot be deserialized into `UpdateStatus`. Retries up to
/// `VIEW_UPDATE_RETRY_COUNT` total attempts with `VIEW_UPDATE_RETRY_DELAY_MS`
/// between attempts, then collapses to a final `UpdateResult` with
/// `updating: false` and the last observed native message.
///
/// The webview-derived `doc_status` / `version` travel in as parameters
/// so the eventual `UpdateResult` keeps the docs.rs fetch context even
/// after the bridge call fails.
///
/// # Arguments
///
/// - `bool` - The docs.rs documentation status carried through the bridge call.
/// - `String` - The version string carried through the bridge call.
///
/// # Returns
///
/// - `UpdateResult` - UI-facing snapshot carrying docs.rs context plus
///   the most recent `data` / `message` reported by the native side.
async fn notify_native_with_retry(doc_status: bool, version: String) -> UpdateResult {
    let mut attempt: u32 = 0;
    loop {
        match try_notify_native_once().await {
            Ok((UpdateStatus::Success, payload)) => {
                if attempt > 0 {
                    Console::log(format!(
                        "update_cache bridge returned after {attempt}/{VIEW_UPDATE_RETRY_COUNT} prior failure(s): {payload}"
                    ));
                } else {
                    Console::log(format!("update_cache bridge returned: {payload}"));
                }
                return UpdateResult {
                    doc_status,
                    version,
                    updating: true,
                    data: payload.get_data().clone(),
                    message: payload.get_message().clone(),
                };
            }
            Ok((UpdateStatus::Failed, payload)) => {
                attempt += 1;
                if attempt >= VIEW_UPDATE_RETRY_COUNT {
                    Console::error(format!(
                        "update_cache bridge returned after {attempt}/{VIEW_UPDATE_RETRY_COUNT} attempts ({attempt} failure(s)): {payload}",
                    ));
                    return UpdateResult {
                        doc_status,
                        version,
                        updating: false,
                        data: String::new(),
                        message: format!(
                            "{} (attempt {attempt}/{VIEW_UPDATE_RETRY_COUNT}, {attempt} failure(s))",
                            payload.get_message()
                        ),
                    };
                }
                Console::warn(format!(
                    "update_cache bridge returned (attempt {attempt}/{VIEW_UPDATE_RETRY_COUNT}): {payload}. Retrying in {VIEW_UPDATE_RETRY_DELAY_MS}ms...",
                ));
                sleep_ms(VIEW_UPDATE_RETRY_DELAY_MS).await;
            }
            Err(error) => {
                attempt += 1;
                if attempt >= VIEW_UPDATE_RETRY_COUNT {
                    Console::error(format!(
                        "update_cache bridge failed after {attempt}/{VIEW_UPDATE_RETRY_COUNT} attempts ({attempt} failure(s)): {error} (no native payload)"
                    ));
                    return UpdateResult {
                        doc_status,
                        version,
                        updating: false,
                        data: String::new(),
                        message: format!(
                            "{error} (attempt {attempt}/{VIEW_UPDATE_RETRY_COUNT}, {attempt} failure(s))"
                        ),
                    };
                }
                Console::warn(format!(
                    "update_cache bridge failed (attempt {attempt}/{VIEW_UPDATE_RETRY_COUNT}): {error} (no native payload). Retrying in {VIEW_UPDATE_RETRY_DELAY_MS}ms..."
                ));
                sleep_ms(VIEW_UPDATE_RETRY_DELAY_MS).await;
            }
        }
    }
}

/// Performs one bridge invocation and classifies the outcome.
///
/// Deserializes the native `CacheUpdateResult` payload into
/// `UpdateResultPayload` and surfaces every field to the retry loop so it
/// can build a complete `UpdateResult` (success log message, snapshot
/// name, or last error description).
///
/// # Returns
///
/// - `Result<(UpdateStatus, UpdateResultPayload), String>` - `Ok` carries the
///   native-side outcome plus the full payload, so the caller can read `data` /
///   `message` without re-deserializing; `Err` is a transport-level or
///   deserialization failure that justifies a retry.
async fn try_notify_native_once() -> Result<(UpdateStatus, UpdateResultPayload), String> {
    let promise: Promise = BridgeConfig::invoke(INVOKE_UPDATE_CACHE, None, None)?;
    let value: JsValue = JsFuture::from(promise)
        .await
        .map_err(|error: JsValue| error.as_string().unwrap_or(ERROR_NULL_TEXT.to_string()))?;
    let payload: UpdateResultPayload =
        serde_wasm_bindgen::from_value(value).map_err(|error: serde_wasm_bindgen::Error| {
            format!("failed to deserialize update_cache result: {error}")
        })?;
    Ok((payload.result, payload))
}

/// Suspends the current task for `millis` milliseconds via `setTimeout`.
///
/// Centralizes the timer wiring used by retry loops so call sites stay
/// declarative and the timeout closure is a single point to swap for tests.
///
/// # Arguments
///
/// - `u32` - Number of milliseconds to sleep.
async fn sleep_ms(millis: u32) {
    let Some(window): Option<Window> = window() else {
        return;
    };
    let _: Result<JsValue, JsValue> =
        JsFuture::from(Promise::new(&mut |resolve: Function, _reject: Function| {
            let _: Result<i32, JsValue> = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, millis as i32);
        }))
        .await;
}

/// Renders the application shell with navigation and route-based page content.
///
/// Detects viewport size and switches between desktop sidebar layout and
/// mobile header + drawer layout accordingly.
///
/// # Returns
///
/// - `VirtualNode` - The root application virtual DOM tree.
pub(crate) fn app() -> VirtualNode {
    Console::init();
    let route_signal: Signal<String> = App::use_signal(Router::current_route);
    let panel_open: Signal<bool> = App::use_signal(|| false);
    let drawer_open: Signal<bool> = App::use_signal(|| false);
    let mobile_signal: Signal<bool> = UseEuvLayout::use_resize();
    let theme_state: ThemeState = ThemeState::use_theme_state(mobile_signal);
    let theme_signal: Signal<String> = theme_state.get_theme();
    let root_class_signal: Signal<String> = theme_state.get_root_class();
    UseCacheUpdate::use_cache_state().load(check_docs_update);
    Router::use_hash_change(route_signal);
    Router::use_scroll_to_top(route_signal);
    Router::use_overlay_history(drawer_open, mobile_signal);
    Router::use_scroll_drawer_to_active(drawer_open);
    UseEuvLayout::use_safe_area_fix();
    html! {
        if { mobile_signal } {
            mobile_layout {
                route_signal
                theme_signal
                root_class_signal
                panel_open
                drawer_open
            }
        } else {
            desktop_layout {
                route_signal
                theme_signal
                root_class_signal
                panel_open
            }
        }
    }
}
