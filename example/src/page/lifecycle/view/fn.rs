use super::*;

/// A lifecycle demo page showing mount and update tracking.
/// # Arguments
///
/// - `VirtualNode<PageLifecycleProps>` - The component props node.
///
/// # Returns
///
/// - `VirtualNode` - The lifecycle demo page virtual DOM tree.
#[component]
pub(crate) fn page_lifecycle(node: VirtualNode<PageLifecycleProps>) -> VirtualNode {
    let PageLifecycleProps: PageLifecycleProps = node.try_get_props().unwrap_or_default();
    let state: UseLifecycle = use_lifecycle();
    let render_count: Signal<i32> = state.get_render_count();
    let logs: Signal<Vec<String>> = state.get_logs();
    watch!(render_count, |render_count_value: i32| {
        Console::log(format!("watch! render count changed: {render_count_value}"));
    });
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "♻️"
                title: LIFECYCLE_HEADER_TITLE
                subtitle: LIFECYCLE_HEADER_SUBTITLE
            }
            euv_card {
                title: LIFECYCLE_RENDER_COUNTER_CARD_TITLE
                p {
                    class: c_render_count_text()
                    {
                        LIFECYCLE_RENDERED_PREFIX
                    }
                    span {
                        class: c_counter_value()
                        render_count
                    }
                    {
                        LIFECYCLE_RENDERED_SUFFIX
                    }
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: LIFECYCLE_BUTTON_LABEL_UPDATE
                        onclick: lifecycle_on_trigger(state)
                    }
                }
            }
            euv_card {
                title: LIFECYCLE_EVENT_LOG_CARD_TITLE
                div {
                    class: c_log_container()
                    for (index, log) in { logs.get().iter().enumerate() } {
                        div {
                            key: index.to_string()
                            class: c_log_item()
                            log.clone()
                        }
                    }
                }
            }
        }
    }
}
