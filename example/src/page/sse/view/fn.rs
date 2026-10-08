use super::*;

/// A Server-Sent Events (SSE) demo page showcasing real-time streaming from an SSE endpoint.
///
/// Renders a header, a URL input card for connecting to an SSE server,
/// and a messages display card showing real-time event data.
///
/// # Arguments
///
/// - `VirtualNode<PageSseProps>` - The component node carrying the SSE demo page properties.
///
/// # Returns
///
/// - `VirtualNode` - The SSE demo page virtual DOM tree.
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
                title: SSE_DEMO_TITLE
                subtitle: SSE_DEMO_SUBTITLE
            }
            euv_card {
                title: SSE_CONNECTION_CARD_TITLE
                p {
                    class: c_demo_text()
                    SSE_CONNECTION_DESCRIPTION
                }
                div {
                    class: c_button_controls()
                    if { state.get_connecting().get() } {
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: SSE_CONNECTING_BUTTON_LABEL
                            disabled: state.get_connecting()
                        }
                    } else if { state.get_connected().get() } {
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: SSE_DISCONNECT_BUTTON_LABEL
                            onclick: sse_on_disconnect(state)
                        }
                    } else {
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: SSE_CONNECT_BUTTON_LABEL
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
                title: SSE_MESSAGES_CARD_TITLE
                if { state.get_messages().get().is_empty() } {
                    div {
                        class: c_net_messages_empty()
                        SSE_MESSAGES_EMPTY_TEXT
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
