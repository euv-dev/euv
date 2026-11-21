use super::*;

/// Renders the root application shell.
///
/// Mirrors the official euv example: one reactive root reading the mobile
/// signal, delegating to a desktop shell (left nav column + internally
/// scrolling main) or a mobile shell (top header + drawer). The wrapper key
/// carries the locale and responsive mode so changing shells tears down
/// their distinct DOM listeners instead of reusing incompatible hook slots.
///
/// # Returns
///
/// - `VirtualNode` - The root virtual DOM tree.
pub(crate) fn app() -> VirtualNode {
    let route_signal: Signal<String> = App::use_signal(Router::current_route);
    let drawer_open: Signal<bool> = App::use_signal(|| false);
    let locale_menu_open: Signal<bool> = App::use_signal(|| false);
    let desktop_locale_menu_open: Signal<bool> = App::use_signal(|| false);
    let collapsed: Signal<Vec<String>> = App::use_signal(all_sidebar_group_keys);
    let search_query: Signal<String> = App::use_signal(String::new);
    let mobile_signal: Signal<bool> = UseEuvLayout::use_resize();
    UseEuvLayout::use_safe_area_fix();
    let theme_state: ThemeState = ThemeState::use_theme_state(mobile_signal);
    let theme_signal: Signal<String> = theme_state.get_theme();
    let root_class_signal: Signal<String> = theme_state.get_root_class();
    Router::use_hash_change(route_signal);
    Router::use_overlay_history(drawer_open, mobile_signal);
    use_anchor_scroll(route_signal);
    use_sidebar_reveal(route_signal, collapsed, drawer_open);
    use_docs_drawer_modal(drawer_open, locale_menu_open, mobile_signal);
    // Keep app hooks outside the reactive shell: their initialization reads
    // theme/mobile state and must not make the shell rebuild its bindings.
    VirtualNode::create_dynamic(move |hook_context: &mut euv::HookContext| {
        let mobile: bool = mobile_signal.get();
        hook_context.switch_arm(usize::from(mobile));
        let key: String = format!(
            "{}-{mobile}",
            locale_of(&parse_route(&route_signal.get()).0).prefix
        );
        let shell: VirtualNode = if mobile {
            html! {
                docs_mobile_shell {
                    route_signal
                    theme_signal
                    root_class_signal
                    mobile_signal
                    drawer_open
                    locale_menu_open
                    collapsed
                    search_query
                }
            }
        } else {
            html! {
                docs_desktop_shell {
                    route_signal
                    theme_signal
                    root_class_signal
                    mobile_signal
                    drawer_open
                    locale_menu_open: desktop_locale_menu_open
                    collapsed
                    search_query
                }
            }
        };
        html! {
            div {
                key: key
                class: c_euv_display_contents()
                shell
            }
        }
    })
}

/// Seeds the collapsed-keys signal with every group in the route's own
/// locale, so the sidebar mounts folded to its section titles.
///
/// The tree is per-locale, so the default has to be derived from the
/// locale actually being rendered — seeding from a fixed locale would
/// fold a tree whose keys the current bundle never emits, leaving the
/// reader's own sections expanded while dead keys sit in the signal.
///
/// The initialiser runs through the hook index once, so a route change
/// does not re-fold a tree the reader has since opened; only the reveal
/// pass (`expand_active_groups`) touches the signal afterwards, and it
/// opens exactly the active page's ancestors.
///
/// # Returns
///
/// - `Vec<String>` - Every group key of the boot route's sidebar tree.
fn all_sidebar_group_keys() -> Vec<String> {
    let (path, _anchor) = parse_route(&Router::current_route());
    all_group_keys(locale_of(&path).sidebar)
}

/// Renders the desktop shell: the example's left nav column (brand header,
/// locale row, section label, scrollable sidebar tree, theme toggle, footer)
/// plus an internally scrolling main area.
///
/// # Arguments
///
/// - `VirtualNode<DocsShellProps>` - The props node carrying the shell signals.
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
        mobile_signal,
        drawer_open: _,
        locale_menu_open,
        collapsed,
        search_query,
    }: DocsShellProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    html! {
        div {
            class: root_class_signal
            class: c_app_root()
            class: CLASS_DOCS_SHELL
            docs_header(locale, route_signal, theme_signal, search_query, mobile_signal)
            div {
                class: CLASS_DOCS_BODY
                nav {
                    class: c_app_nav()
                    locale_row_node(route_signal, locale_menu_open, mobile_signal, false)
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
                }
                main {
                    class: c_app_main()
                    class: c_euv_user_select_text()
                    docs_main {
                        route_signal
                    }
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
/// - `VirtualNode<DocsShellProps>` - The props node carrying the shell signals.
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
        mobile_signal,
        drawer_open,
        locale_menu_open,
        collapsed,
        search_query: _,
    }: DocsShellProps = node.try_get_props().unwrap_or_default();
    let (path, _anchor) = parse_route(&route_signal.get());
    let locale: &DocsLocale = locale_of(&path);
    html! {
        div {
            // Both shells use tracked class snapshots: the responsive diff
            // reuses this div, so switching from desktop text to a direct
            // signal would not install a new mount-time DOM subscription.
            class: root_class_signal.get()
            header {
                class: c_mobile_header()
                div {
                    class: c_mobile_header_left()
                    button {
                        id: DOCS_DRAWER_TRIGGER_ID
                        type: DOCS_DRAWER_BUTTON_TYPE
                        "aria-label": locale_text(&path, DOCS_KEY_DRAWER_OPEN)
                        title: locale_text(&path, DOCS_KEY_DRAWER_OPEN)
                        "aria-haspopup": DOCS_DRAWER_DIALOG_ROLE
                        "aria-controls": DOCS_DRAWER_ID
                        "aria-expanded": if { drawer_open.get() } {
                            DOCS_DRAWER_ARIA_TRUE
                        } else {
                            DOCS_DRAWER_ARIA_FALSE
                        }
                        class: if { drawer_open } {
                            c_mobile_menu_button_active()
                        } else {
                            c_mobile_menu_button()
                        }
                        onclick: UseEuvLayout::use_drawer_toggle(drawer_open)
                        "☰"
                    }
                    brand_logo_row(locale, false)
                }
                button {
                    type: DOCS_DRAWER_BUTTON_TYPE
                    class: c_mobile_theme_button()
                    title: locale_text(&path, DOCS_KEY_THEME_TOGGLE)
                    "aria-label": locale_text(&path, DOCS_KEY_THEME_TOGGLE)
                    onclick: ThemeState::toggle(theme_signal)
                    theme_icon_node(theme_signal)
                }
            }
            main {
                class: c_mobile_main()
                class: c_euv_user_select_text()
                docs_main {
                    route_signal
                }
            }
            div {
                "aria-hidden": DOCS_DRAWER_ARIA_TRUE
                class: if { drawer_open } {
                    c_mobile_overlay().to_string()
                } else {
                    format!("{} {}", c_mobile_overlay().get_name(), c_mobile_overlay_hidden().get_name())
                }
                onclick: close_drawer(drawer_open)
            }
            nav {
                id: DOCS_DRAWER_ID
                role: DOCS_DRAWER_DIALOG_ROLE
                "aria-modal": DOCS_DRAWER_ARIA_TRUE
                "aria-label": locale_text(&path, DOCS_KEY_DRAWER_TITLE)
                "aria-hidden": if { drawer_open.get() } {
                    DOCS_DRAWER_ARIA_FALSE
                } else {
                    DOCS_DRAWER_ARIA_TRUE
                }
                tabindex: "-1"
                "data-slot": DOCS_DRAWER_CONTENT_SLOT
                class: if { drawer_open } {
                    c_mobile_nav_drawer().to_string()
                } else {
                    format!("{} {}", c_mobile_nav_drawer().get_name(), c_mobile_nav_drawer_closed().get_name())
                }
                div {
                    class: c_mobile_nav_drawer_header()
                    brand_logo_row(locale, true)
                    button {
                        id: DOCS_DRAWER_CLOSE_ID
                        type: DOCS_DRAWER_BUTTON_TYPE
                        "aria-label": locale_text(&path, DOCS_KEY_DRAWER_CLOSE)
                        title: locale_text(&path, DOCS_KEY_DRAWER_CLOSE)
                        class: c_mobile_drawer_close_button()
                        onclick: close_drawer(drawer_open)
                        "✕"
                    }
                }
                locale_row_node(route_signal, locale_menu_open, mobile_signal, true)
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
            }
        }
    }
}

/// Renders the mobile brand row (logo + title, links home).
///
/// # Arguments
///
/// - `&'static DocsLocale` - The current locale.
/// - `bool` - Whether the home link consumes the drawer's overlay entry.
///
/// # Returns
///
/// - `VirtualNode` - The brand row virtual DOM tree.
fn brand_logo_row(locale: &'static DocsLocale, drawer: bool) -> VirtualNode {
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
            onclick: if drawer { drawer_link_handler(locale.prefix) } else { Router::link_handler(locale.prefix) }
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
pub(crate) fn theme_icon_node(theme_signal: Signal<String>) -> VirtualNode {
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

/// Renders the locale switcher row for the nav column / drawer (empty when
/// the site has a single locale).
///
/// Entries come from the generated `SITE_LANGUAGES` table (one per unique
/// content directory), not from `SITE.locales`: per-locale bundling
/// compiles exactly one locale into the wasm, so the full language list no
/// longer exists in `SITE`.
///
/// # Arguments
///
/// - `Signal<String>` - The current route signal.
/// - `Signal<bool>` - The dropdown open signal.
/// - `Signal<bool>` - The responsive mode signal.
/// - `bool` - The responsive mode that owns the row.
///
/// # Returns
///
/// - `VirtualNode` - The locale row virtual DOM tree.
fn locale_row_node(
    route_signal: Signal<String>,
    locale_menu_open: Signal<bool>,
    mobile_signal: Signal<bool>,
    mobile: bool,
) -> VirtualNode {
    VirtualNode::create_dynamic(move |context: &mut HookContext| {
        use_docs_responsive_scope(context.clone(), mobile_signal, mobile);
        render_locale_row(route_signal, locale_menu_open)
    })
}

/// Renders the locale dropdown within its own stable hook lifecycle.
///
/// # Arguments
///
/// - `Signal<String>` - The current route signal.
/// - `Signal<bool>` - The dropdown open signal.
///
/// # Returns
///
/// - `VirtualNode` - The dropdown row.
fn render_locale_row(route_signal: Signal<String>, locale_menu_open: Signal<bool>) -> VirtualNode {
    let languages: &'static [DocsLanguageLink] = crate::generated::SITE_LANGUAGES;
    if languages.len() <= 1 {
        return html! {
            ""
        };
    }
    let current_label: &str = languages
        .iter()
        .find(|language: &&DocsLanguageLink| language.dir == crate::generated::SITE_LOCALE_DIR)
        .map(|language: &DocsLanguageLink| language.label)
        .unwrap_or(languages[0].label);
    let items: Vec<EuvDropdownItem> = languages
        .iter()
        .map(|target: &DocsLanguageLink| EuvDropdownItem {
            label: target.label,
            value: target.dir,
            disabled: false,
            active: target.dir == crate::generated::SITE_LOCALE_DIR,
        })
        .collect();
    let trigger_class: &'static Css = if locale_menu_open.get() {
        c_nav_locale_button_open()
    } else {
        c_nav_locale_button()
    };
    let trigger: VirtualNode = html! {
        euv_dropdown_trigger {
            open: locale_menu_open
            { current_label }
        }
    }
    .extend_attributes([
        AttributeEntry::new(
            LOCALE_TRIGGER_CLASS_ATTRIBUTE.into(),
            AttributeValue::CssRef(trigger_class),
        ),
        AttributeEntry::new(
            LOCALE_TRIGGER_TITLE_ATTRIBUTE.into(),
            AttributeValue::Text(locale_text(&route_signal.get(), DOCS_KEY_LOCALE_TITLE)),
        ),
    ]);
    html! {
        div {
            class: c_nav_locale_row()
            euv_dropdown {
                open: locale_menu_open
                items: items
                on_select: switch_locale(route_signal, locale_menu_open)
                trigger
            }
        }
    }
}

/// Picks the section label shown above the sidebar tree: the first internal,
/// non-home navbar item text (e.g. `"指南"`), falling back to `"文档"`.
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
        .find(|item: &&EuvNavbarItem| {
            !item.link.starts_with(URL_SCHEME_HTTP_PREFIX) && item.link != locale.prefix
        })
        .map(|item: &EuvNavbarItem| item.text)
        .unwrap_or("文档")
}

/// Switches to the language chosen from the dropdown.
///
/// Every locale is its own wasm bundle served from its own directory, so
/// switching languages navigates across directories: the target URL is the
/// site root plus the target bundle's directory plus the current (already
/// prefix-free) in-bundle route. `Location::assign` performs a full load —
/// the standard behaviour of i18n documentation sites — which also swaps
/// the wasm bundle itself.
///
/// # Arguments
///
/// - `Signal<String>` - The route signal.
/// - `Signal<bool>` - The menu-open signal (closed after switching).
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select handler receiving the
///   target bundle directory relative to the site root.
fn switch_locale(
    route_signal: Signal<String>,
    menu_open: Signal<bool>,
) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |dir: &'static str| {
        menu_open.set(false);
        if dir == crate::generated::SITE_LOCALE_DIR {
            return;
        }
        if let Some(window) = window() {
            let route: String = route_signal.get();
            let url: String = format!("{}{dir}#{route}", site_root());
            let _: Result<(), JsValue> = window.location().assign(&url);
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
        dismiss_docs_drawer(drawer_open);
    }))
}

/// Navigates from the drawer's home link after consuming its overlay entry.
///
/// # Arguments
///
/// - `&'static str` - The locale's home route.
///
/// # Returns
///
/// - `NativeEventHandler` - The intercepted link click handler.
fn drawer_link_handler(link: &'static str) -> NativeEventHandler {
    NativeEventHandler::create(DOCS_DRAWER_CLICK_EVENT, move |event: Event| {
        event.prevent_default();
        Router::overlay_back(Some(link.to_string()));
    })
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
/// The scroll follows the route, not the render cycle. The shell
/// re-renders whenever the theme changes — the theme signal is read while
/// the shell renders — and re-entering this hook used to scroll again with
/// the route unchanged, which threw the reader back to the anchor, or to
/// the top of the page, when they merely switched the theme.
///
/// `watch!` runs the body once when the shell mounts, so a deep link still
/// lands on its heading, and afterwards only when the route actually
/// moves. Scrolling is deferred with a timeout so it runs after the
/// reactive DOM update.
///
/// # Arguments
///
/// - `Signal<String>` - The current route signal.
fn use_anchor_scroll(route_signal: Signal<String>) {
    watch!(route_signal, |route: String| {
        let (_path, anchor) = parse_route(&route);
        schedule_scroll(anchor);
    });
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
    let Some(outer_window) = window() else {
        return;
    };
    let callback: Closure<dyn FnMut()> = Closure::once(move || {
        let Some(window) = window() else {
            return;
        };
        let scrolled: bool = anchor
            .and_then(|id: String| {
                window
                    .document()
                    .and_then(|doc: Document| doc.get_element_by_id(&id))
            })
            .map(|element: Element| {
                element.scroll_into_view();
            })
            .is_some();
        if !scrolled {
            window.scroll_to_with_x_and_y(0.0, 0.0);
            if let Some(doc) = window.document() {
                for selector in [
                    MAIN_CONTAINER_SELECTOR_DESKTOP,
                    MAIN_CONTAINER_SELECTOR_MOBILE,
                ] {
                    if let Ok(Some(main)) = doc.query_selector(selector) {
                        main.set_scroll_top(0);
                    }
                }
            }
        }
    });
    let _: Result<i32, JsValue> = outer_window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            100,
        );
    callback.forget();
}

/// Keeps the active sidebar entry visible: expands its collapsed ancestor
/// groups (any depth) and scrolls the sidebar container so the entry is
/// fully revealed.
///
/// Runs once at startup, on every route change, and each time the mobile
/// drawer opens (the desktop aside and the drawer share the `collapsed`
/// signal, so one expansion covers both). A theme or locale switch
/// re-renders the shell and re-enters this hook, but it does not change
/// the route, so it must not move the sidebar.
///
/// # Arguments
///
/// - `Signal<String>` - The current route signal.
/// - `Signal<Vec<String>>` - The collapsed sidebar group keys.
/// - `Signal<bool>` - The mobile drawer open signal.
fn use_sidebar_reveal(
    route_signal: Signal<String>,
    collapsed: Signal<Vec<String>>,
    drawer_open: Signal<bool>,
) {
    reveal_active_sidebar(route_signal, collapsed);
    // The scroll is tied to the route value, not to the subscribe call. A
    // signal fires its subscribers whenever it is re-set, and the shell
    // re-renders on theme and locale changes too, which re-runs this hook
    // and used to throw the reader's scroll position back to the active
    // entry mid-page. Comparing against the route this hook last acted on
    // lets the callback bail when the route did not actually move.
    let seen_route: Signal<String> = App::use_signal(Router::current_route);
    route_signal.subscribe(move || {
        if seen_route.get() == route_signal.get() {
            return;
        }
        seen_route.set(route_signal.get());
        reveal_active_sidebar(route_signal, collapsed);
    });
    drawer_open.subscribe(move || {
        if drawer_open.get() {
            reveal_active_sidebar(route_signal, collapsed);
        }
    });
}

/// Expands the ancestor groups of the active route and schedules the sidebar
/// scroll.
///
/// # Arguments
///
/// - `Signal<String>` - The current route signal.
/// - `Signal<Vec<String>>` - The collapsed sidebar group keys.
fn reveal_active_sidebar(route_signal: Signal<String>, collapsed: Signal<Vec<String>>) {
    let (path, _anchor) = parse_route(&route_signal.get());
    let items: &'static [EuvSidebarItem] = locale_of(&path).sidebar;
    expand_active_groups(items, &path, collapsed);
    schedule_sidebar_scroll();
}

/// Removes the ancestor group keys of `path` from the `collapsed` signal so
/// the active entry renders. Group keys mirror euv-ui's `euv_sidebar_item`
/// recursion (`{prefix}/{text}`, top-level prefix empty).
///
/// # Arguments
///
/// - `&'static [EuvSidebarItem]` - The locale sidebar tree.
/// - `&str` - The active route path.
/// - `Signal<Vec<String>>` - The collapsed sidebar group keys.
fn expand_active_groups(
    items: &'static [EuvSidebarItem],
    path: &str,
    collapsed: Signal<Vec<String>>,
) {
    let mut keys: Vec<String> = Vec::new();
    collect_active_group_keys(items, path, "", &mut keys);
    if keys.is_empty() {
        return;
    }
    let mut current: Vec<String> = collapsed.get();
    let before: usize = current.len();
    current.retain(|key: &String| !keys.contains(key));
    if current.len() != before {
        collapsed.set(current);
    }
}

/// Walks the sidebar tree collecting the keys of the ancestor groups of the
/// active entry. The active group's own collapse state is left untouched so
/// its title click keeps toggling; only descendant activation expands a group.
/// Returns whether the subtree contains the active route.
///
/// # Arguments
///
/// - `&'static [EuvSidebarItem]` - The items of the current tree level.
/// - `&str` - The active route path.
/// - `&mut Vec<String>` - The collected group keys.
///
/// # Returns
///
/// - `bool` - `true` when `path` resolves inside this subtree.
fn collect_active_group_keys(
    items: &'static [EuvSidebarItem],
    path: &str,
    prefix: &str,
    keys: &mut Vec<String>,
) -> bool {
    let mut contains: bool = false;
    for item in items {
        if item.children.is_empty() {
            if item.link == Some(path) {
                contains = true;
            }
            continue;
        }
        let key: String = sidebar_group_key(prefix, item.text);
        let mut child_keys: Vec<String> = Vec::new();
        let child_contains: bool =
            collect_active_group_keys(item.children, path, &key, &mut child_keys);
        if child_contains {
            keys.push(key);
            keys.append(&mut child_keys);
        }
        if child_contains || item.link == Some(path) {
            contains = true;
        }
    }
    contains
}

/// Defers the sidebar scroll until after the reactive re-render (the
/// expansion above mutates `collapsed`, which re-renders the tree first).
fn schedule_sidebar_scroll() {
    let Some(window) = window() else {
        return;
    };
    let callback: Closure<dyn FnMut()> = Closure::once(scroll_active_sidebar_item);
    let _: Result<i32, JsValue> = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        100,
    );
    callback.forget();
}

/// Scrolls every visible sidebar container (desktop aside and mobile drawer)
/// so its active entry is fully in view, moving no further than it must.
///
/// If the active entry is already fully visible inside the container's
/// scroll viewport, no scroll is performed (a no-op avoids the jarring
/// jump that would happen on every nav click while the user reads the
/// current page). Otherwise the entry is pulled just past the nearest
/// edge — top or bottom — by a small breathing gap, via the plain DOM
/// `Element.scroll_top` setter, which is an instant jump with no
/// animation, so the sidebar snaps to the new position immediately.
///
/// This deliberately does not centre the entry. Centring moved the tree
/// by up to half the container height on every navigation, which read as
/// a lurch: landing on the last entry threw the whole sidebar to its
/// end. The minimum move that reveals the entry is the only amount that
/// carries information.
fn scroll_active_sidebar_item() {
    let Some(document) = window().and_then(|window: Window| window.document()) else {
        return;
    };
    let Ok(containers) = document.query_selector_all(SIDEBAR_SCROLL_SELECTOR) else {
        return;
    };
    for index in 0..containers.length() {
        let Some(node) = containers.item(index) else {
            continue;
        };
        let container: Element = node.unchecked_into();
        let container_rect: DomRect = container.get_bounding_client_rect();
        if container_rect.height() <= 0.0 {
            continue;
        }
        let Ok(Some(active)) = container.query_selector(SIDEBAR_ACTIVE_SELECTOR) else {
            continue;
        };
        let active_rect: DomRect = active.get_bounding_client_rect();
        // Skip when the entry is already fully inside the container's
        // viewport — same arithmetic the euv example uses to detect
        // "in view" before scrolling.
        if active_rect.top() >= container_rect.top()
            && active_rect.bottom() <= container_rect.bottom()
        {
            continue;
        }
        // Reveal by the smaller of the two distances, then add the gap so
        // the entry does not sit flush against the edge it was pulled out
        // of. Centring would pick the larger one and overshoot.
        let above: f64 = active_rect.top() - container_rect.top();
        let below: f64 = active_rect.bottom() - container_rect.bottom();
        let delta: f64 = if above <= below {
            -(above + SIDEBAR_SCROLL_GAP)
        } else {
            below + SIDEBAR_SCROLL_GAP
        };
        let target: f64 = container.scroll_top() as f64 + delta;
        let max_scroll: f64 = (container.scroll_height() as f64 - container_rect.height()).max(0.0);
        container.set_scroll_top(target.clamp(0.0, max_scroll) as i32);
    }
}
