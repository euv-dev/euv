use super::*;

/// A page demonstrating the async-state primitives
/// ([`UseAsyncHandle`], [`LazyComponent`], [`SuspenseHandle`]).
///
/// The three rows share a single browser timer so the page can
/// drive transitions without spinning up an HTTP server.
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
                title: "Hooks — Async"
                subtitle: "AsyncState (use_async), lazy factory (use_lazy_component), and suspense phases (use_suspense)."
            }
            euv_card {
                title: "use_async"
                p {
                    class: c_render_count_text()
                    "The handle's state exposes an AsyncState machine. The Loading arm carries () by default; the Ok arm carries the resolved value; the Err arm the failure message."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Refetch"
                        onclick: hooks_async_refetch(async_handle)
                    }
                }
                p {
                    class: c_render_count_text()
                    "state: "
                    span {
                        class: c_counter_value()
                        hooks_async_state_label(async_handle)
                    }
                }
            }
            euv_card {
                title: "use_lazy_component"
                p {
                    class: c_render_count_text()
                    "The factory is only invoked on first access. Click Load to run it once; Reset returns the component to the pending state so the next read invokes the factory again."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Load"
                        onclick: hooks_async_lazy_on_load(lazy_value.clone())
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Reset"
                        onclick: hooks_async_lazy_on_reset(lazy_value.clone())
                    }
                }
                div {
                    class: c_counter_row()
                    div {
                        "loaded:"
                        span {
                            class: c_counter_value()
                            hooks_async_lazy_loaded_label(&lazy_value)
                        }
                    }
                    div {
                        "is_pending:"
                        span {
                            class: c_counter_value()
                            hooks_async_lazy_is_pending(&lazy_value)
                        }
                    }
                }
            }
            euv_card {
                title: "use_suspense"
                p {
                    class: c_render_count_text()
                    "resolve_sync and fail flip the phase signal — the rendering code branches on the resulting Pending / Resolved / Failed variant."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { hooks_async_suspense_is_resolved(&suspense) } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: "Resolve"
                        onclick: hooks_async_resolve(suspense, String::from(HOOKS_ASYNC_RESOLVED_VALUE))
                    }
                    euv_button {
                        variant: if { hooks_async_suspense_is_failed(&suspense) } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: "Fail"
                        onclick: hooks_async_fail(suspense, String::from(HOOKS_ASYNC_FAIL_MESSAGE))
                    }
                    euv_button {
                        variant: if { hooks_async_suspense_is_pending(&suspense) } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: "Reset"
                        onclick: hooks_async_reset(suspense)
                    }
                }
                p {
                    class: c_render_count_text()
                    "phase: "
                    span {
                        class: c_counter_value()
                        hooks_async_suspense_phase_label(&suspense)
                    }
                }
            }
        }
    }
}
