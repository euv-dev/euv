use super::*;

/// Renders the same application chrome and slot ordering for every host.
///
/// The host owns breakpoint observation, theme, routing and overlay history.
/// This component supplies no CSS, focus trap or global listeners. Switching
/// responsive mode remounts the frame; drawer open/close never unmounts it.
///
/// # Arguments
///
/// - `VirtualNode<EuvLayoutProps>` - Controlled shell state and content factories.
///
/// # Returns
///
/// - `VirtualNode` - An unstyled responsive application shell.
#[component]
pub fn euv_layout(node: VirtualNode<EuvLayoutProps>) -> VirtualNode {
    let props: EuvLayoutProps = node.try_get_props().unwrap_or_default();
    let mobile: Signal<bool> = props.mobile;
    let root_class: Signal<String> = props.root_class;
    let root_extra: String = props.classes.root_extra.clone();
    let drawer_id: Signal<String> = use_id("euv-layout-drawer").get_value();
    let settings: Signal<LayoutSettings> = Signal::create(LayoutSettings {
        props: Rc::new(props),
    });
    html! {
        div {
            "data-slot": "layout"
            class: root_class
            class: root_extra
            if { mobile.get() } {
                {
                    layout_frame(layout_settings(settings), EuvLayoutMode::Mobile, drawer_id.get())
                }
            } else {
                {
                    layout_frame(layout_settings(settings), EuvLayoutMode::Desktop, drawer_id.get())
                }
            }
        }
    }
}

/// Renders the common region tree for one responsive mode.
///
/// # Arguments
///
/// - `EuvLayoutProps` - Controlled state and caller-provided regions.
/// - `EuvLayoutMode` - Mode selected by the shared shell.
/// - `String` - Stable drawer identifier belonging to this shell instance.
///
/// # Returns
///
/// - `VirtualNode` - The header, body and optional mounted drawer.
pub(crate) fn layout_frame(
    props: EuvLayoutProps,
    mode: EuvLayoutMode,
    drawer_id: String,
) -> VirtualNode {
    let settings: Signal<LayoutSettings> = Signal::create(LayoutSettings {
        props: Rc::new(props.clone()),
    });
    let EuvLayoutProps {
        drawer_open,
        slots,
        menu_label,
        close_label,
        ..
    }: EuvLayoutProps = props;
    let brand: VirtualNode = layout_slot(slots.brand.clone(), mode);
    let header_navigation: VirtualNode = layout_slot(slots.header_navigation, mode);
    let selector: VirtualNode = layout_slot(slots.selector, mode);
    let theme_control: VirtualNode = layout_slot(slots.theme_control, mode);
    let content: VirtualNode = layout_slot(slots.content, mode);
    let extra: VirtualNode = layout_slot(slots.extra, mode);
    let menu_id: Signal<String> = Signal::create(drawer_id);
    let menu: VirtualNode = if mode == EuvLayoutMode::Mobile {
        html! {
            button {
                "data-slot": "layout-menu"
                type: "button"
                "aria-label": menu_label
                "aria-controls": menu_id.get()
                "aria-expanded": if { drawer_open.get() } {
                    "true"
                } else {
                    "false"
                }
                class: layout_settings(settings).classes.menu_button
                class: if { drawer_open.get() } {
                    layout_settings(settings).classes.menu_button_active
                } else {
                    String::new()
                }
                onclick: UseEuvLayout::use_drawer_toggle(drawer_open)
                "☰"
            }
        }
    } else {
        html! {}
    };
    let navigation_content: VirtualNode = layout_slot(slots.navigation, mode);
    let footer_content: VirtualNode = layout_slot(slots.navigation_footer, mode);
    let navigation: VirtualNode = html! {
        nav {
            "data-slot": "layout-navigation" class: layout_settings(settings).classes.sidebar_navigation navigation_content
        }
        div {
            "data-slot": "layout-navigation-footer" class: layout_settings(settings).classes.sidebar_footer footer_content
        }
    };
    let (sidebar, drawer): (VirtualNode, VirtualNode) = if mode == EuvLayoutMode::Desktop {
        (
            html! {
                aside {
                    "data-slot": "layout-sidebar" class: layout_settings(settings).classes.sidebar navigation
                }
            },
            html! {},
        )
    } else {
        let drawer_brand: VirtualNode = layout_slot(slots.brand, mode);
        (
            html! {},
            html! {
                div {
                    "data-slot": "layout-overlay"
                    "aria-hidden": "true"
                    hidden: !drawer_open.get()
                    class: layout_settings(settings).classes.overlay
                    class: if { drawer_open.get() } {
                        String::new()
                    } else {
                        layout_settings(settings).classes.overlay_hidden
                    }
                    onclick: layout_close(drawer_open)
                }
                aside {
                    "data-slot": "layout-drawer"
                    id: menu_id.get()
                    "aria-label": menu_label
                    "aria-hidden": if { drawer_open.get() } {
                        "false"
                    } else {
                        "true"
                    }
                    hidden: !drawer_open.get()
                    inert: !drawer_open.get()
                    class: layout_settings(settings).classes.drawer
                    class: if { drawer_open.get() } {
                        String::new()
                    } else {
                        layout_settings(settings).classes.drawer_closed
                    }
                    div {
                        "data-slot": "layout-drawer-header"
                        class: layout_settings(settings).classes.drawer_header
                        drawer_brand
                        button {
                            "data-slot": "layout-drawer-close"
                            type: "button"
                            "aria-label": close_label
                            class: layout_settings(settings).classes.drawer_close
                            onclick: layout_close(drawer_open)
                            "×"
                        }
                    }
                    navigation
                }
            },
        )
    };
    html! {
        header {
            "data-slot": "layout-header"
            class: layout_settings(settings).classes.header
            div {
                "data-slot": "layout-header-left"
                class: layout_settings(settings).classes.header_left
                menu
                div {
                    "data-slot": "layout-brand" class: layout_settings(settings).classes.brand brand
                }
            }
            div {
                "data-slot": "layout-header-navigation" class: layout_settings(settings).classes.header_navigation header_navigation
            }
            div {
                "data-slot": "layout-header-actions"
                class: layout_settings(settings).classes.header_actions
                div {
                    "data-slot": "layout-selector" selector
                }
                div {
                    "data-slot": "layout-theme-control" theme_control
                }
            }
        }
        div {
            "data-slot": "layout-body"
            class: layout_settings(settings).classes.body
            sidebar
            main {
                "data-slot": "layout-main" class: layout_settings(settings).classes.main content
            }
        }
        drawer
        div {
            "data-slot": "layout-extra" extra
        }
    }
}

/// Reads the immutable shell configuration from its scope-owned handle.
///
/// # Arguments
///
/// - `Signal<LayoutSettings>` - The shell configuration handle.
///
/// # Returns
///
/// - `EuvLayoutProps` - Cloned configuration retaining the caller's state handles.
fn layout_settings(settings: Signal<LayoutSettings>) -> EuvLayoutProps {
    settings.get().props.as_ref().clone()
}

/// Evaluates an optional content factory.
///
/// # Arguments
///
/// - `Option<EuvLayoutSlot>` - The caller's factory, if present.
/// - `EuvLayoutMode` - The selected responsive mode.
///
/// # Returns
///
/// - `VirtualNode` - Slot content, or empty HTML when absent.
fn layout_slot(slot: Option<EuvLayoutSlot>, mode: EuvLayoutMode) -> VirtualNode {
    slot.map_or_else(|| html! {}, |render: EuvLayoutSlot| render(mode))
}

/// Closes the controlled drawer without consuming another overlay when already closed.
///
/// # Arguments
///
/// - `Signal<bool>` - Host-owned drawer state.
/// - `F` - Existing overlay-history close operation.
pub(crate) fn close_layout_drawer<F>(open: Signal<bool>, close_history: F)
where
    F: Fn(),
{
    if open.get() {
        close_history();
        open.set(false);
    }
}

/// Closes only this drawer and consumes its overlay-history entry when open.
///
/// # Arguments
///
/// - `Signal<bool>` - Host-owned drawer state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - Close/backdrop click handler.
fn layout_close(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        close_layout_drawer(open, Router::overlay_stack_close);
    }))
}
