use super::*;

/// A page demonstrating the async-state primitives
/// ([`UseAsyncHandle`], [`LazyComponent`], [`SuspenseHandle`]).
///
/// The three rows share a single browser timer so the page can
/// drive transitions without spinning up an HTTP server.
///
/// # Arguments
///
/// - `VirtualNode<PageHooksAsyncProps>` - The props node carrying the page configuration.
///
/// # Returns
///
/// - `VirtualNode` - The rendered async-hooks page element tree.
#[component]
pub(crate) fn page_hooks_async(node: VirtualNode<PageHooksAsyncProps>) -> VirtualNode {
    let PageHooksAsyncProps: PageHooksAsyncProps = node.try_get_props().unwrap_or_default();
    let async_handle: UseAsyncHandle<String, ()> = use_async::<String, ()>();
    let lazy_value: LazyComponent<String> =
        use_lazy_component::<String, _>(|| String::from(HOOKS_ASYNC_LAZY_VALUE));
    let suspense: SuspenseHandle<String> = use_suspense::<String>();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🌐"
                title: HOOKS_ASYNC_HEADER_TITLE
                subtitle: HOOKS_ASYNC_HEADER_SUBTITLE
            }
            euv_card {
                title: HOOKS_ASYNC_CARD_TITLE
                p {
                    class: c_render_count_text()
                    HOOKS_ASYNC_CARD_DESCRIPTION
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: HOOKS_ASYNC_REFETCH_LABEL
                        onclick: hooks_async_refetch(async_handle)
                    }
                }
                p {
                    class: c_render_count_text()
                    HOOKS_ASYNC_STATE_PREFIX
                    span {
                        class: c_counter_value()
                        hooks_async_state_label(async_handle)
                    }
                }
            }
            euv_card {
                title: HOOKS_ASYNC_LAZY_CARD_TITLE
                p {
                    class: c_render_count_text()
                    HOOKS_ASYNC_LAZY_CARD_DESCRIPTION
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: HOOKS_ASYNC_LAZY_LOAD_LABEL
                        onclick: hooks_async_lazy_on_load(lazy_value.clone())
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: HOOKS_ASYNC_RESET_LABEL
                        onclick: hooks_async_lazy_on_reset(lazy_value.clone())
                    }
                }
                div {
                    class: c_counter_row()
                    div {
                        HOOKS_ASYNC_LAZY_LOADED_PREFIX
                        span {
                            class: c_counter_value()
                            hooks_async_lazy_loaded_label(&lazy_value)
                        }
                    }
                    div {
                        HOOKS_ASYNC_LAZY_PENDING_PREFIX
                        span {
                            class: c_counter_value()
                            hooks_async_lazy_is_pending(&lazy_value)
                        }
                    }
                }
            }
            euv_card {
                title: HOOKS_ASYNC_SUSPENSE_CARD_TITLE
                p {
                    class: c_render_count_text()
                    HOOKS_ASYNC_SUSPENSE_CARD_DESCRIPTION
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { hooks_async_suspense_is_resolved(&suspense) } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: HOOKS_ASYNC_SUSPENSE_RESOLVE_LABEL
                        onclick: hooks_async_resolve(suspense, String::from(HOOKS_ASYNC_RESOLVED_VALUE))
                    }
                    euv_button {
                        variant: if { hooks_async_suspense_is_failed(&suspense) } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: HOOKS_ASYNC_SUSPENSE_FAIL_LABEL
                        onclick: hooks_async_fail(suspense, String::from(HOOKS_ASYNC_FAIL_MESSAGE))
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: HOOKS_ASYNC_RESET_LABEL
                        onclick: hooks_async_reset(suspense)
                    }
                }
                p {
                    class: c_render_count_text()
                    HOOKS_ASYNC_PHASE_PREFIX
                    span {
                        class: c_counter_value()
                        hooks_async_suspense_phase_label(&suspense)
                    }
                }
            }
        }
    }
}
