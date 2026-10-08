use super::*;

/// A counter demo page showcasing reactive signal-driven state updates.
/// # Arguments
///
/// - `VirtualNode<PageCounterProps>` - The component props node.
///
/// # Returns
///
/// - `VirtualNode` - The counter demo page virtual DOM tree.
///
/// # Arguments
///
/// - `VirtualNode<PageCounterProps>` - The `node` argument.
///
#[component]
pub(crate) fn page_counter(node: VirtualNode<PageCounterProps>) -> VirtualNode {
    let PageCounterProps: PageCounterProps = node.try_get_props().unwrap_or_default();
    let count: Signal<i32> = App::use_signal(|| 0);
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🔢"
                title: "Counter"
                subtitle: "A reactive counter powered by Signal state. Click Add, Subtract, or Reset to see fine-grained updates — only the changed number re-renders, not the entire page."
            }
            euv_card {
                title: "Counter"
                p {
                    class: c_counter_text()
                    "The current count is "
                    span {
                        id: "counter"
                        class: c_counter_value()
                        count
                    }
                    ". Each click updates the Signal, and the Virtual DOM efficiently patches only the changed text node."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Add"
                        onclick: counter_on_increment(count)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Subtract"
                        onclick: counter_on_decrement(count)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Reset"
                        onclick: counter_on_reset(count)
                    }
                }
            }
        }
    }
}
