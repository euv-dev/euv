use super::*;

/// A conditional rendering demo page.
/// # Arguments
///
/// - `VirtualNode<PageConditionalProps>` - The component props node.
///
/// # Returns
///
/// - `VirtualNode` - The conditional demo page virtual DOM tree.
///
/// # Arguments
///
/// - `VirtualNode<PageConditionalProps>` - The `node` argument.
///
#[component]
pub(crate) fn page_conditional(node: VirtualNode<PageConditionalProps>) -> VirtualNode {
    let PageConditionalProps: PageConditionalProps = node.try_get_props().unwrap_or_default();
    let show_details: Signal<bool> = App::use_signal(|| false);
    let user_type: Signal<ConditionalUserType> = App::use_signal(ConditionalUserType::default);
    let tab: Signal<ConditionalTab> = App::use_signal(ConditionalTab::default);
    let display_name: Signal<String> = App::use_signal(|| "".to_string());
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🔀"
                title: "Conditional Rendering"
                subtitle: "Toggle element visibility with if expressions, render different content based on role with match, and implement tab switching — all driven by Signal state."
            }
            euv_card {
                title: "Toggle Content"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { show_details } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: "Toggle"
                        onclick: UseEuvInput::use_toggle(show_details)
                    }
                }
                if { show_details } {
                    div {
                        class: c_toggle_content()
                        h4 {
                            class: c_toggle_title()
                            "Hidden Details"
                        }
                        p {
                            class: c_demo_text()
                            "This content is conditionally rendered using an if expression bound to a Signal. When the Signal is false, the node is removed from the Virtual DOM entirely."
                        }
                        p {
                            class: c_demo_text_muted()
                            "Use this pattern for modals, accordions, feature flags, permission-based UI, and any scenario where content should appear or disappear reactively."
                        }
                    }
                }
            }
            euv_card {
                title: "Role-Based Rendering"
                div {
                    class: c_role_button_row()
                    euv_button {
                        variant: if { user_type.get() == ConditionalUserType::Guest } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: ConditionalUserType::Guest.label()
                        onclick: user_type_on_select(user_type, ConditionalUserType::Guest)
                    }
                    euv_button {
                        variant: if { user_type.get() == ConditionalUserType::User } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: ConditionalUserType::User.label()
                        onclick: user_type_on_select(user_type, ConditionalUserType::User)
                    }
                    euv_button {
                        variant: if { user_type.get() == ConditionalUserType::Admin } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: ConditionalUserType::Admin.label()
                        onclick: user_type_on_select(user_type, ConditionalUserType::Admin)
                    }
                }
                match { user_type } {
                    ConditionalUserType::Guest => {
                        div {
                            p {
                                class: c_role_guest_text()
                                "Welcome, guest! Please sign in to access more features."
                            }
                        }
                    }
                    ConditionalUserType::User => {
                        div {
                            p {
                                class: c_role_user_text()
                                "Hello, user! You have standard access."
                            }
                        }
                    }
                    _ => {
                        div {
                            p {
                                class: c_role_admin_text()
                                "Welcome, administrator! You have full access."
                            }
                        }
                    }
                }
            }
            euv_card {
                title: "Tab Switching"
                div {
                    class: c_tab_bar()
                    div {
                        class: if { tab.get() == ConditionalTab::Info } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: tab_on_select(tab, ConditionalTab::Info)
                        ConditionalTab::Info.to_string()
                    }
                    div {
                        class: if { tab.get() == ConditionalTab::Settings } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: tab_on_select(tab, ConditionalTab::Settings)
                        ConditionalTab::Settings.to_string()
                    }
                    div {
                        class: if { tab.get() == ConditionalTab::About } {
                            c_tab_item_active()
                        } else {
                            c_tab_item_inactive()
                        }
                        onclick: tab_on_select(tab, ConditionalTab::About)
                        ConditionalTab::About.to_string()
                    }
                }
                match { tab } {
                    ConditionalTab::Info => {
                        div {
                            class: c_tab_content()
                            p {
                                class: c_tab_text()
                                "This is the information tab."
                            }
                            p {
                                class: c_tab_text_muted()
                                "Here you can find general details about the application."
                            }
                        }
                    }
                    ConditionalTab::Settings => {
                        div {
                            class: c_tab_content()
                            p {
                                class: c_tab_text_input()
                                "This is the settings tab."
                            }
                            euv_input {
                                id: "settings-display-name"
                                label: "Display Name"
                                placeholder: "Enter your name"
                                value: display_name
                                autocomplete: "name"
                            }
                        }
                    }
                    _ => {
                        div {
                            class: c_tab_content()
                            p {
                                class: c_tab_text()
                                "This is the about tab."
                            }
                            p {
                                class: c_tab_text_muted()
                                "euv is a declarative UI framework for Rust."
                            }
                        }
                    }
                }
            }
        }
    }
}
