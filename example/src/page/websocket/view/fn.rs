use super::*;

/// A WebSocket chat page with auto-generated UUID and Ping keep-alive.
///
/// Renders a header, a connection card, a send message card,
/// and a messages card that displays all received WebSocket messages
/// as raw text.
///
/// # Returns
///
/// - `VirtualNode` - The WebSocket chat page virtual DOM tree.
///
/// # Arguments
///
/// - `VirtualNode<PageWebsocketProps>` - The `node` argument.
///
#[component]
pub(crate) fn page_websocket(node: VirtualNode<PageWebsocketProps>) -> VirtualNode {
    let PageWebsocketProps: PageWebsocketProps = node.try_get_props().unwrap_or_default();
    let state: UseWebSocket = use_websocket();
    ws_cleanup(state);
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🔌"
                title: "WebSocket Chat"
                subtitle: "Connect to a WebSocket chat server with automatic UUID assignment and Ping keep-alive. Send and receive messages in real time."
            }
            euv_card {
                title: "Connection"
                p {
                    class: c_demo_text()
                    "A random UUID is generated for each session. Click Connect to establish a real-time bidirectional connection. Ping messages are sent automatically to keep the connection alive."
                }
                div {
                    class: c_button_controls()
                    if { state.get_connecting().get() } {
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Wait"
                            disabled: state.get_connecting()
                        }
                    } else if { state.get_connected().get() } {
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Close"
                            onclick: websocket_on_disconnect(state)
                        }
                    } else {
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Connect"
                            onclick: websocket_on_connect(state)
                        }
                    }
                }
                if { !state.get_error().get().is_empty() } {
                    div {
                        class: c_error_box()
                        state.get_error()
                    }
                }
            }
            if { state.get_connected().get() } {
                euv_card {
                    title: "Send Message"
                    div {
                        class: c_inline_input_row()
                        euv_input {
                            placeholder: "Enter message to send"
                            value: state.get_message_input()
                            oninput: UseEuvInput::on_input_value(state.get_message_input())
                            class: c_ws_message_input().clone()
                        }
                        div {
                            class: c_inline_input_button_wrap()
                            euv_button {
                                variant: EuvButtonVariant::Primary
                                label: "Send"
                                onclick: websocket_on_send(state)
                            }
                        }
                    }
                }
            }
            euv_card {
                title: "Messages"
                if { state.get_messages().get().is_empty() } {
                    div {
                        class: c_net_messages_empty()
                        "No messages yet. Connect to start receiving."
                    }
                } else {
                    div {
                        class: c_net_messages_list()
                        for (index, message) in { state.get_messages().get().iter().enumerate() } {
                            div {
                                key: index.to_string()
                                class: c_net_message_item()
                                span {
                                    class: c_net_message_index()
                                    format!("#{}", index + 1)
                                }
                                span {
                                    class: c_net_message_data()
                                    message.data.clone()
                                }
                                if !message.time.is_empty() {
                                    span {
                                        class: c_ws_message_time()
                                        message.time.clone()
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
