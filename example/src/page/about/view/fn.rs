use super::*;

/// Returns the next route path based on the navigation items order.
///
/// Given a current route, finds its position in the navigation items
/// and returns the next item's target path. If the current route is
/// the last one or not found, returns the first navigation item.
///
/// # Arguments
///
/// - `&str` - The current route path to find.
///
/// # Returns
///
/// - `&'static str` - The next route path in the navigation order.
pub(crate) fn get_next_route(current_route: &str) -> &'static str {
    let nav_items: &[(&str, &str, &str)] = NAV_ITEMS;
    let current_index: usize = nav_items
        .iter()
        .position(|item: &(&str, &str, &str)| item.2 == current_route)
        .unwrap_or_default();
    let next_index: usize = (current_index + 1) % nav_items.len();
    nav_items[next_index].2
}

/// A home page component displaying the Euv framework showcase.
///
/// Features a section with animated gradient, feature highlights,
/// package metadata cards, and interactive demos — all with modern
/// glass-morphism design language.
///
/// # Arguments
///
/// - `VirtualNode<PageAboutProps>` - The page component node carrying the
///   page props.
///
/// # Returns
///
/// - `VirtualNode` - The home page virtual DOM tree.
#[component]
pub(crate) fn page_about(node: VirtualNode<PageAboutProps>) -> VirtualNode {
    let PageAboutProps: PageAboutProps = node.try_get_props().unwrap_or_default();
    let native_bridge_state: UseEuvNativeBridge = UseEuvNativeBridge::use_bridge_state();
    native_bridge_state.load_data(None);
    let version: String = format!("v{EUV_VERSION}");
    let current_route: String = Router::current_route();
    let next_route: &'static str = get_next_route(&current_route);
    let next_route_for_href: String = format!("#{next_route}");
    html! {
        div {
            class: c_page_container()
            div {
                class: c_home()
                div {
                    class: c_home_content()
                    h1 {
                        class: c_home_title()
                        "Euv"
                    }
                    div {
                        class: c_home_badge_row()
                        div {
                            class: c_home_badge()
                            version.clone()
                        }
                    }
                    p {
                        class: c_home_subtitle()
                        EUV_DESCRIPTION
                    }
                    div {
                        class: c_home_actions()
                        a {
                            class: c_home_btn_primary()
                            href: EUV_REPOSITORY
                            target: "_blank"
                            onclick: Router::external_link_handler(EUV_REPOSITORY)
                            "GitHub"
                        }
                        a {
                            class: c_home_btn_secondary()
                            href: next_route_for_href
                            onclick: Router::link_handler(next_route)
                            "Browse"
                        }
                    }
                }
            }
            div {
                class: c_home_stats()
                div {
                    class: c_home_stat_card()
                    div {
                        class: c_home_stat_icon()
                        "⚡"
                    }
                    div {
                        class: c_home_stat_value()
                        "WASM"
                    }
                    div {
                        class: c_home_stat_label()
                        "Runtime"
                    }
                }
                div {
                    class: c_home_stat_card()
                    div {
                        class: c_home_stat_icon()
                        "🦀"
                    }
                    div {
                        class: c_home_stat_value()
                        "Rust"
                    }
                    div {
                        class: c_home_stat_label()
                        "Language"
                    }
                }
                div {
                    class: c_home_stat_card()
                    div {
                        class: c_home_stat_icon()
                        "🎨"
                    }
                    div {
                        class: c_home_stat_value()
                        "VDOM"
                    }
                    div {
                        class: c_home_stat_label()
                        "Architecture"
                    }
                }
                div {
                    class: c_home_stat_card()
                    div {
                        class: c_home_stat_icon()
                        "📦"
                    }
                    div {
                        class: c_home_stat_value()
                        "4"
                    }
                    div {
                        class: c_home_stat_label()
                        "Crates"
                    }
                }
            }
            div {
                h2 {
                    class: c_home_section_title()
                    "Features"
                }
                p {
                    class: c_home_section_desc()
                    "Everything you need for declarative cross-platform UI development."
                }
                div {
                    class: c_home_feature_grid()
                    euv_card {
                        title: "Reactive Signals"
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_header()
                                div {
                                    class: c_feature_icon()
                                    "⚡"
                                }
                                h4 {
                                    class: c_feature_name()
                                    "Signal-Based Reactivity"
                                }
                            }
                            p {
                                class: c_feature_desc()
                                "Fine-grained reactive state management with automatic dependency tracking. Signals only notify dependents that read them, avoiding unnecessary re-renders."
                            }
                        }
                    }
                    euv_card {
                        title: "Virtual DOM"
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_header()
                                div {
                                    class: c_feature_icon()
                                    "🌲"
                                }
                                h4 {
                                    class: c_feature_name()
                                    "Efficient Diffing"
                                }
                            }
                            p {
                                class: c_feature_desc()
                                "Virtual DOM with optimized reconciliation for smooth 60fps updates. The differ computes the minimal set of DOM operations needed to sync the UI with the latest state."
                            }
                        }
                    }
                    euv_card {
                        title: "HTML Macros"
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_header()
                                div {
                                    class: c_feature_icon()
                                    "🏗️"
                                }
                                h4 {
                                    class: c_feature_name()
                                    "Declarative Syntax"
                                }
                            }
                            p {
                                class: c_feature_desc()
                                "Write UI with familiar HTML-like macros that compile to efficient Rust at build time. No runtime template engine — just zero-cost abstractions."
                            }
                        }
                    }
                    euv_card {
                        title: "Cross-Platform"
                        div {
                            class: c_feature_card()
                            div {
                                class: c_feature_header()
                                div {
                                    class: c_feature_icon()
                                    "🌐"
                                }
                                h4 {
                                    class: c_feature_name()
                                    "WebAssembly Powered"
                                }
                            }
                            p {
                                class: c_feature_desc()
                                "Run anywhere with WASM — browsers, servers, and native platforms via the bridge. Share the same Rust codebase across all targets."
                            }
                        }
                    }
                }
            }
            div {
                h2 {
                    class: c_home_section_title()
                    "Package Info"
                }
                euv_card {
                    title: "Project Details"
                    euv_info {
                        label: "Name"
                        EUV_PACKAGE_NAME
                    }
                    euv_info {
                        label: "Version"
                        version.clone()
                    }
                    euv_info {
                        label: "Edition"
                        EUV_EDITION
                    }
                    euv_info {
                        label: "License"
                        EUV_LICENSE
                    }
                    euv_info {
                        label: "Authors"
                        EUV_AUTHORS
                    }
                    euv_info {
                        label: "Repository"
                        a {
                            class: c_info_link()
                            href: EUV_REPOSITORY
                            target: "_blank"
                            onclick: Router::external_link_handler(EUV_REPOSITORY)
                            EUV_REPOSITORY_NAME
                        }
                    }
                }
                euv_card {
                    title: "Build Information"
                    euv_info {
                        label: "Date"
                        EUV_BUILD_DATE
                    }
                    euv_info {
                        label: "Time"
                        EUV_BUILD_CLOCK
                    }
                    euv_info {
                        label: "Timestamp"
                        EUV_BUILD_TIMESTAMP
                    }
                }
            }
            if { !native_bridge_state.get_loading().get() && native_bridge_state.get_available().get() } {
                div {
                    h2 {
                        class: c_home_section_title()
                        "Native Bridge"
                    }
                    euv_card {
                        title: "bridge Integration"
                        euv_info {
                            label: "Permissions"
                            native_bridge_state.get_permissions()
                        }
                    }
                }
            }
        }
    }
}
