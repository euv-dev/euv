use super::*;

/// A Server-Sent Events (SSE) demo page showcasing real-time streaming from an SSE endpoint.
///
/// Renders a header, a URL input card for connecting to an SSE server,
/// and a messages display card showing real-time event data.
///
/// # Returns
///
/// - `VirtualNode` - The SSE demo page virtual DOM tree.
///
/// # Arguments
///
/// - `VirtualNode<PageSseProps>` - The `node` argument.
///
#[component]
pub(crate) fn page_sse(node: VirtualNode<PageSseProps>) -> VirtualNode {
    let PageSseProps: PageSseProps = node.try_get_props().unwrap_or_default();
    let state: UseSse = use_sse();
    sse_cleanup(state);
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "📡"
                title: "Server-Sent Events"
                subtitle: "Connect to an SSE endpoint and receive real-time streaming events from the server. Events appear in the list below as they arrive."
            }
            euv_card {
                title: "Connection"
                p {
                    class: c_demo_text()
                    "Enter the SSE endpoint URL and click Connect to start receiving server-sent events."
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
                            onclick: sse_on_disconnect(state)
                        }
                    } else {
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Connect"
                            onclick: sse_on_connect(state)
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
            euv_card {
                title: "Messages"
                if { state.get_messages().get().is_empty() } {
                    div {
                        class: c_net_messages_empty()
                        "No messages received yet. Connect to an SSE endpoint to start receiving events."
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
                                    message.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
