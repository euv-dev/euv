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
    html! {
        div {
            class: root_class_signal
            class: c_example_theme()
            nav {
                class: c_app_nav()
                a {
                    href: "https://github.com/euv-dev/euv"
                    target: "_blank"
                    onclick: Router::external_link_handler("https://github.com/euv-dev/euv")
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
                    "Pages"
                }
                build_desktop_nav_items {
                    route_signal: route_signal
                }
                div {
                    class: c_nav_theme_toggle()
                    button {
                        class: c_nav_theme_button()
                        onclick: ThemeState::toggle(theme_signal)
                        div {
                            class: if { theme_signal.get() == THEME_DARK } {
                                c_theme_icon_sun()
                            } else {
                                c_theme_icon_moon()
                            }
                        }
                    }
                }
                a {
                    href: "https://github.com/euv-dev/euv"
                    target: "_blank"
                    onclick: Router::external_link_handler("https://github.com/euv-dev/euv")
                    class: c_nav_footer()
                    div {
                        class: c_nav_footer_divider()
                    }
                    span {
                        class: c_nav_footer_text()
                        "Built with "
                        span {
                            class: c_nav_footer_brand()
                            "Euv & Wasm"
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
            header {
                class: c_mobile_header()
                div {
                    class: c_mobile_header_left()
                    button {
                        class: if { drawer_open } {
                            c_mobile_menu_button_active()
                        } else {
                            c_mobile_menu_button()
                        }
                        onclick: UseEuvLayout::use_drawer_toggle(drawer_open)
                        "☰"
                    }
                    a {
                        href: "https://github.com/euv-dev/euv"
                        target: "_blank"
                        onclick: Router::external_link_handler("https://github.com/euv-dev/euv")
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
                    class: c_mobile_theme_button()
                    onclick: ThemeState::toggle(theme_signal)
                    div {
                        class: if { theme_signal.get() == THEME_DARK } {
                            c_theme_icon_sun()
                        } else {
                            c_theme_icon_moon()
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
                class: if { drawer_open } {
                    c_mobile_overlay().to_string()
                } else {
                    format!("{} {}", c_mobile_overlay().get_name(), c_mobile_overlay_hidden().get_name())
                }
                onclick: on_overlay_click
            }
            nav {
                class: if { drawer_open } {
                    c_mobile_nav_drawer().to_string()
                } else {
                    format!("{} {}", c_mobile_nav_drawer().get_name(), c_mobile_nav_drawer_closed().get_name())
                }
                div {
                    class: c_mobile_nav_drawer_header()
                    div {
                        class: c_mobile_header_logo()
                        a {
                            href: "https://github.com/euv-dev/euv"
                            target: "_blank"
                            onclick: Router::external_link_handler("https://github.com/euv-dev/euv")
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
                    "Pages"
                }
                build_mobile_nav_items {
                    route_signal: route_signal
                    drawer_open: drawer_open
                }
                a {
                    href: "https://github.com/euv-dev/euv"
                    target: "_blank"
                    onclick: Router::external_link_handler("https://github.com/euv-dev/euv")
                    class: c_nav_footer()
                    div {
                        class: c_nav_footer_divider()
                    }
                    span {
                        class: c_nav_footer_text()
                        "Built with "
                        span {
                            class: c_nav_footer_brand()
                            "Euv & Wasm"
                        }
                    }
                }
            }
        }
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
