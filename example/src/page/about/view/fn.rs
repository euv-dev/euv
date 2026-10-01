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
                        BRAND_NAME
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
                            target: ABOUT_LINK_TARGET_BLANK
                            onclick: Router::external_link_handler(EUV_REPOSITORY)
                            ABOUT_GITHUB_BUTTON_LABEL
                        }
                        a {
                            class: c_home_btn_secondary()
                            href: next_route_for_href
                            onclick: Router::link_handler(next_route)
                            ABOUT_BROWSE_BUTTON_LABEL
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
                        ABOUT_STAT_WASM_VALUE
                    }
                    div {
                        class: c_home_stat_label()
                        ABOUT_STAT_RUNTIME_LABEL
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
                        ABOUT_STAT_RUST_VALUE
                    }
                    div {
                        class: c_home_stat_label()
                        ABOUT_STAT_LANGUAGE_LABEL
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
                        ABOUT_STAT_VDOM_VALUE
                    }
                    div {
                        class: c_home_stat_label()
                        ABOUT_STAT_ARCHITECTURE_LABEL
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
                        ABOUT_STAT_CRATES_LABEL
                    }
                }
            }
            div {
                h2 {
                    class: c_home_section_title()
                    ABOUT_SECTION_FEATURES_TITLE
                }
                p {
                    class: c_home_section_desc()
                    ABOUT_SECTION_FEATURES_DESC
                }
                div {
                    class: c_home_feature_grid()
                    euv_card {
                        title: ABOUT_FEATURE_REACTIVE_SIGNALS_TITLE
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
                                    ABOUT_FEATURE_REACTIVE_SIGNALS_NAME
                                }
                            }
                            p {
                                class: c_feature_desc()
                                ABOUT_FEATURE_REACTIVE_SIGNALS_DESC
                            }
                        }
                    }
                    euv_card {
                        title: ABOUT_FEATURE_VIRTUAL_DOM_TITLE
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
                                    ABOUT_FEATURE_VIRTUAL_DOM_NAME
                                }
                            }
                            p {
                                class: c_feature_desc()
                                ABOUT_FEATURE_VIRTUAL_DOM_DESC
                            }
                        }
                    }
                    euv_card {
                        title: ABOUT_FEATURE_HTML_MACROS_TITLE
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
                                    ABOUT_FEATURE_HTML_MACROS_NAME
                                }
                            }
                            p {
                                class: c_feature_desc()
                                ABOUT_FEATURE_HTML_MACROS_DESC
                            }
                        }
                    }
                    euv_card {
                        title: ABOUT_FEATURE_CROSS_PLATFORM_TITLE
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
                                    ABOUT_FEATURE_CROSS_PLATFORM_NAME
                                }
                            }
                            p {
                                class: c_feature_desc()
                                ABOUT_FEATURE_CROSS_PLATFORM_DESC
                            }
                        }
                    }
                }
            }
            div {
                h2 {
                    class: c_home_section_title()
                    ABOUT_SECTION_PACKAGE_INFO_TITLE
                }
                euv_card {
                    title: ABOUT_CARD_PROJECT_DETAILS_TITLE
                    euv_info {
                        label: ABOUT_INFO_NAME_LABEL
                        EUV_PACKAGE_NAME
                    }
                    euv_info {
                        label: ABOUT_INFO_VERSION_LABEL
                        version.clone()
                    }
                    euv_info {
                        label: ABOUT_INFO_EDITION_LABEL
                        EUV_EDITION
                    }
                    euv_info {
                        label: ABOUT_INFO_LICENSE_LABEL
                        EUV_LICENSE
                    }
                    euv_info {
                        label: ABOUT_INFO_AUTHORS_LABEL
                        EUV_AUTHORS
                    }
                    euv_info {
                        label: ABOUT_INFO_REPOSITORY_LABEL
                        a {
                            class: c_info_link()
                            href: EUV_REPOSITORY
                            target: ABOUT_LINK_TARGET_BLANK
                            onclick: Router::external_link_handler(EUV_REPOSITORY)
                            EUV_REPOSITORY_NAME
                        }
                    }
                }
                euv_card {
                    title: ABOUT_CARD_BUILD_INFORMATION_TITLE
                    euv_info {
                        label: ABOUT_BUILD_DATE_LABEL
                        EUV_BUILD_DATE
                    }
                    euv_info {
                        label: ABOUT_BUILD_TIME_LABEL
                        EUV_BUILD_CLOCK
                    }
                    euv_info {
                        label: ABOUT_BUILD_TIMESTAMP_LABEL
                        EUV_BUILD_TIMESTAMP
                    }
                }
            }
            if { !native_bridge_state.get_loading().get() && native_bridge_state.get_available().get() } {
                div {
                    h2 {
                        class: c_home_section_title()
                        ABOUT_SECTION_NATIVE_BRIDGE_TITLE
                    }
                    euv_card {
                        title: ABOUT_CARD_BRIDGE_INTEGRATION_TITLE
                        euv_info {
                            label: ABOUT_BRIDGE_PERMISSIONS_LABEL
                            native_bridge_state.get_permissions()
                        }
                    }
                }
            }
        }
    }
}
