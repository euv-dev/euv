use super::*;

/// Renders the root application shell.
///
/// Mirrors the official euv example: one reactive root reading the mobile
/// signal, delegating to a desktop shell (left nav column + internally
/// scrolling main) or a mobile shell (top header + drawer). The wrapper key
/// carries the locale prefix so switching languages remounts the shell with
/// fresh localized labels.
///
/// # Returns
///
/// - `VirtualNode` - The root virtual DOM tree.
pub(crate) fn app() -> VirtualNode {
    let route_signal: Signal<String> = App::use_signal(Router::current_route);
    let drawer_open: Signal<bool> = App::use_signal(|| false);
    let locale_menu_open: Signal<bool> = App::use_signal(|| false);
    let collapsed: Signal<Vec<String>> = App::use_signal(Vec::new);
    let mobile_signal: Signal<bool> = UseEuvLayout::use_resize();
    UseEuvLayout::use_safe_area_fix();
    let theme_state: ThemeState = ThemeState::use_theme_state(mobile_signal);
    let theme_signal: Signal<String> = theme_state.get_theme();
    let root_class_signal: Signal<String> = theme_state.get_root_class();
    Router::use_hash_change(route_signal);
    Router::use_overlay_history(drawer_open, mobile_signal);
    use_anchor_scroll(route_signal);
    html! {
        div {
            key: locale_of(&parse_route(&route_signal.get()).0).prefix
            style: "display: contents"
            if { mobile_signal.get() } {
                docs_mobile_shell {
                    route_signal
                    theme_signal
                    root_class_signal
                    drawer_open
                    locale_menu_open
                    collapsed
                }
            } else {
                docs_desktop_shell {
                    route_signal
                    theme_signal
                    root_class_signal
                    drawer_open
                    locale_menu_open
                    collapsed
                }
            }
        }
    }
}

/// Renders the desktop shell: the example's left nav column (brand header,
/// locale row, section label, scrollable sidebar tree, theme toggle, footer)
/// plus an internally scrolling main area.
///
/// # Arguments
///
/// - `DocsShellProps` - The typed props containing the shell signals.
///
/// # Returns
///
/// - `VirtualNode` - The desktop shell virtual DOM tree.
#[component]
pub(crate) fn docs_desktop_shell(node: VirtualNode<DocsShellProps>) -> VirtualNode {
    let DocsShellProps {
        route_signal,
        theme_signal,
        root_class_signal,
        drawer_open: _,
        locale_menu_open,
        collapsed,
    }: DocsShellProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    html! {
        div {
            class: root_class_signal
            nav {
                class: c_app_nav()
                brand_header(locale)
                locale_row_node(route_signal, locale_menu_open)
                p {
                    class: c_nav_section_label()
                    {
                        section_label(locale)
                    }
                }
                div {
                    class: c_nav_items_scroll()
                    euv_sidebar {
                        route_signal
                        collapsed
                        items: locale.sidebar
                    }
                }
                div {
                    class: c_nav_theme_toggle()
                    button {
                        class: c_nav_theme_button()
                        title: "Toggle theme"
                        onclick: ThemeState::toggle(theme_signal)
                        theme_icon_node(theme_signal)
                    }
                }
                nav_footer_node(github_link(locale))
            }
            main {
                class: c_app_main()
                style: "user-select: text"
                docs_main {
                    route_signal
                }
            }
        }
    }
}

/// Renders the mobile shell: the example's sticky top header (menu button,
/// brand, theme button), an internally scrolling main area, and the
/// slide-out navigation drawer with overlay-stack close behaviour.
///
/// # Arguments
///
/// - `DocsShellProps` - The typed props containing the shell signals.
///
/// # Returns
///
/// - `VirtualNode` - The mobile shell virtual DOM tree.
#[component]
pub(crate) fn docs_mobile_shell(node: VirtualNode<DocsShellProps>) -> VirtualNode {
    let DocsShellProps {
        route_signal,
        theme_signal,
        root_class_signal,
        drawer_open,
        locale_menu_open,
        collapsed,
    }: DocsShellProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    html! {
        div {
            class: root_class_signal
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
                    brand_logo_row(locale)
                }
                button {
                    class: c_mobile_theme_button()
                    title: "Toggle theme"
                    onclick: ThemeState::toggle(theme_signal)
                    theme_icon_node(theme_signal)
                }
            }
            main {
                class: c_mobile_main()
                style: "user-select: text"
                docs_main {
                    route_signal
                }
            }
            div {
                class: if { drawer_open } {
                    c_mobile_overlay().to_string()
                } else {
                    format!("{} {}", c_mobile_overlay().get_name(), c_mobile_overlay_hidden().get_name())
                }
                onclick: close_drawer(drawer_open)
            }
            nav {
                class: if { drawer_open } {
                    c_mobile_nav_drawer().to_string()
                } else {
                    format!("{} {}", c_mobile_nav_drawer().get_name(), c_mobile_nav_drawer_closed().get_name())
                }
                div {
                    class: c_mobile_nav_drawer_header()
                    brand_logo_row(locale)
                    button {
                        class: c_mobile_drawer_close_button()
                        onclick: close_drawer(drawer_open)
                        "✕"
                    }
                }
                locale_row_node(route_signal, locale_menu_open)
                p {
                    class: c_nav_section_label()
                    {
                        section_label(locale)
                    }
                }
                div {
                    class: c_nav_items_scroll()
                    euv_sidebar {
                        route_signal
                        collapsed
                        items: locale.sidebar
                        on_navigate: drawer_navigate()
                    }
                }
                nav_footer_node(github_link(locale))
            }
        }
    }
}

/// Renders the desktop nav-column brand header (logo + title, links home).
///
/// # Arguments
///
/// - `&'static DocsLocale` - The current locale.
///
/// # Returns
///
/// - `VirtualNode` - The brand header virtual DOM tree.
fn brand_header(locale: &'static DocsLocale) -> VirtualNode {
    let site: &DocsSite = &crate::generated::SITE;
    let title: &str = if locale.title.is_empty() {
        site.title
    } else {
        locale.title
    };
    html! {
        a {
            class: c_nav_header()
            href: format!("#{}", locale.prefix)
            onclick: Router::link_handler(locale.prefix)
            euv_logo {
                variant: LogoButtonVariant::Nav
            }
            span {
                class: c_nav_brand_title()
                {
                    title
                }
            }
        }
    }
}

/// Renders the mobile brand row (logo + title, links home).
///
/// # Arguments
///
/// - `&'static DocsLocale` - The current locale.
///
/// # Returns
///
/// - `VirtualNode` - The brand row virtual DOM tree.
fn brand_logo_row(locale: &'static DocsLocale) -> VirtualNode {
    let site: &DocsSite = &crate::generated::SITE;
    let title: &str = if locale.title.is_empty() {
        site.title
    } else {
        locale.title
    };
    html! {
        a {
            class: c_mobile_header_logo()
            href: format!("#{}", locale.prefix)
            onclick: Router::link_handler(locale.prefix)
            euv_logo {
                variant: LogoButtonVariant::Nav
            }
            span {
                class: c_nav_brand_title()
                {
                    title
                }
            }
        }
    }
}

/// Renders the theme toggle icon (sun in dark mode, moon in light mode),
/// mirroring the example.
///
/// # Arguments
///
/// - `Signal<String>` - The current theme name signal.
///
/// # Returns
///
/// - `VirtualNode` - The theme icon virtual DOM tree.
fn theme_icon_node(theme_signal: Signal<String>) -> VirtualNode {
    html! {
        div {
            class: if { theme_signal.get() == THEME_DARK } {
                c_theme_icon_sun()
            } else {
                c_theme_icon_moon()
            }
        }
    }
}

/// Renders the nav-column footer: divider plus an external "Built with" link
/// to the site repository (empty when the locale has no external link).
///
/// # Arguments
///
/// - `Option<&'static str>` - The external repository URL.
///
/// # Returns
///
/// - `VirtualNode` - The footer virtual DOM tree.
fn nav_footer_node(github: Option<&'static str>) -> VirtualNode {
    let Some(url) = github else {
        return html! {
            ""
        };
    };
    html! {
        a {
            class: c_nav_footer()
            href: url
            target: "_blank"
            onclick: Router::external_link_handler(url)
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

/// Renders the locale switcher row for the nav column / drawer (empty when
/// the site has a single locale).
///
/// # Arguments
///
/// - `Signal<String>` - The current route signal.
/// - `Signal<bool>` - The dropdown open signal.
///
/// # Returns
///
/// - `VirtualNode` - The locale row virtual DOM tree.
fn locale_row_node(route_signal: Signal<String>, locale_menu_open: Signal<bool>) -> VirtualNode {
    let site: &DocsSite = &crate::generated::SITE;
    if site.locales.len() <= 1 {
        return html! {
            ""
        };
    }
    let (path, _anchor) = parse_route(&route_signal.get());
    let current_label: &str = locale_of(&path).label;
    let items: Vec<EuvDropdownItem> = site
        .locales
        .iter()
        .map(|target: &DocsLocale| EuvDropdownItem {
            label: target.label,
            value: target.prefix,
        })
        .collect();
    html! {
        div {
            class: c_nav_locale_row()
            euv_dropdown {
                open: locale_menu_open
                items: items
                on_select: switch_locale(route_signal, locale_menu_open)
                button {
                    class: c_nav_theme_button()
                    title: "Language"
                    onclick: toggle_menu(locale_menu_open)
                    {
                        current_label
                    }
                }
            }
        }
    }
}

/// Picks the section label shown above the sidebar tree: the first internal,
/// non-home navbar item text (e.g. `"Guide"`), falling back to `"Docs"`.
///
/// # Arguments
///
/// - `&'static DocsLocale` - The current locale.
///
/// # Returns
///
/// - `&'static str` - The section label.
fn section_label(locale: &'static DocsLocale) -> &'static str {
    locale
        .navbar
        .iter()
        .find(|item: &&EuvNavbarItem| !item.link.starts_with("http") && item.link != locale.prefix)
        .map(|item: &EuvNavbarItem| item.text)
        .unwrap_or("Docs")
}

/// Returns the first external (`http`) navbar link, used as the footer
/// repository URL.
///
/// # Arguments
///
/// - `&'static DocsLocale` - The current locale.
///
/// # Returns
///
/// - `Option<&'static str>` - The external URL.
fn github_link(locale: &'static DocsLocale) -> Option<&'static str> {
    locale
        .navbar
        .iter()
        .find(|item: &&EuvNavbarItem| item.link.starts_with("http"))
        .map(|item: &EuvNavbarItem| item.link)
}

/// Toggles the locale dropdown menu.
///
/// # Arguments
///
/// - `Signal<bool>` - The menu-open signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn toggle_menu(menu_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_| menu_open.set(!menu_open.get())))
}

/// Switches the current route into the locale chosen from the dropdown.
///
/// The shell (sidebar tree, labels, brand title) is locale-bound and computed
/// at mount, so after navigating to the new locale's route the page is
/// reloaded — the standard behaviour of i18n documentation sites — to rebuild
/// every localized label deterministically.
///
/// # Arguments
///
/// - `Signal<String>` - The route signal.
/// - `Signal<bool>` - The menu-open signal (closed after switching).
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select handler receiving the
///   target locale prefix.
fn switch_locale(
    route_signal: Signal<String>,
    menu_open: Signal<bool>,
) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |prefix: &'static str| {
        let site: &DocsSite = &crate::generated::SITE;
        let Some(target) = site.locales.iter().find(|locale| locale.prefix == prefix) else {
            return;
        };
        let (path, _anchor) = parse_route(&route_signal.get());
        menu_open.set(false);
        if let Some(window) = web_sys::window() {
            let location: web_sys::Location = window.location();
            // `set_hash` applies synchronously, so the reload below boots the
            // app straight into the target locale's route.
            let route: String = route_in_locale(&path, target);
            let _: Result<(), JsValue> = location.set_hash(&route);
            let _: Result<(), JsValue> = location.reload();
        }
    }))
}

/// Closes the drawer, consuming its overlay history entry (mirrors the
/// example's overlay click / close button handlers).
///
/// # Arguments
///
/// - `Signal<bool>` - The drawer-open signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_drawer(drawer_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_| {
        Router::overlay_stack_close();
        drawer_open.set(false);
    }))
}

/// Drawer navigation handler: consumes the drawer's overlay history entry
/// via `Router::overlay_back` and navigates after the popstate, so the
/// browser history stays consistent (mirrors the euv example's mobile nav).
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The navigation interceptor.
fn drawer_navigate() -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(|link: &'static str| {
        Router::overlay_back(Some(link.to_string()));
    }))
}

/// Scrolls to the in-page anchor after route changes (or to top).
///
/// Subscribes to the route signal; scrolling is deferred with a timeout so
/// it runs after the reactive DOM update. Also runs once on startup so
/// deep links with an anchor work on first load.
///
/// # Arguments
///
/// - `Signal<String>` - The current route signal.
fn use_anchor_scroll(route_signal: Signal<String>) {
    let schedule = move || {
        let raw: String = route_signal.get();
        let (_path, anchor) = parse_route(&raw);
        schedule_scroll(anchor);
    };
    route_signal.subscribe({
        let schedule = schedule.clone();
        move || schedule()
    });
    schedule();
}

/// Defers a scroll until after the reactive re-render.
///
/// The main content scrolls inside the `c_app_main` / `c_mobile_main`
/// containers (viewport-locked shell), so "back to top" resets those
/// containers as well as the window.
///
/// # Arguments
///
/// - `Option<String>` - The anchor slug; `None` scrolls to the page top.
fn schedule_scroll(anchor: Option<String>) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let callback: Closure<dyn FnMut()> = Closure::once(move || {
        let Some(window) = web_sys::window() else {
            return;
        };
        let scrolled: bool = anchor
            .and_then(|id| window.document().and_then(|doc| doc.get_element_by_id(&id)))
            .map(|element| {
                element.scroll_into_view();
            })
            .is_some();
        if !scrolled {
            window.scroll_to_with_x_and_y(0.0, 0.0);
            if let Some(doc) = window.document() {
                for selector in ["[class*=c_app_main]", "[class*=c_mobile_main]"] {
                    if let Ok(Some(main)) = doc.query_selector(selector) {
                        main.set_scroll_top(0);
                    }
                }
            }
        }
    });
    let _: Result<i32, JsValue> = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        100,
    );
    callback.forget();
}
