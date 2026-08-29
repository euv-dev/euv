use super::*;

/// A page demonstrating the "protective" hooks
/// ([`ErrorBoundary`] and [`ProfilerHandle`]).
#[component]
pub(crate) fn page_hooks_protect(node: VirtualNode<PageHooksProtectProps>) -> VirtualNode {
    let PageHooksProtectProps: PageHooksProtectProps = node.try_get_props().unwrap_or_default();
    let boundary: ErrorBoundary = use_error_boundary();
    let profiler: ProfilerHandle = use_profiler();
    let trigger_label: String = profiler_measure(HOOKS_PROTECT_PROFILER_LABEL_TRIGGER, || {
        String::from(HOOKS_PROTECT_TRIGGER_RENDER_VALUE)
    });
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🛡️"
                title: "Hooks — Protect"
                subtitle: "ErrorBoundary catches panics inside try_with; ProfilerHandle keeps a list of measurements without a global collector."
            }
            euv_card {
                title: "ErrorBoundary"
                p {
                    class: c_render_count_text()
                    "try_with invokes the supplied closure inside a catch_unwind shim. If the closure panics, the boundary transitions to Caught and the caller gets an Err carrying the message."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { hooks_protect_is_healthy(&boundary) } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: "Try a healthy run"
                        onclick: hooks_protect_try_healthy(boundary)
                    }
                    euv_button {
                        variant: if { hooks_protect_is_caught(&boundary) } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: "Try a panic"
                        onclick: hooks_protect_try_panic(boundary)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: "Reset"
                        onclick: hooks_protect_reset(boundary)
                    }
                }
                p {
                    class: c_render_count_text()
                    "phase: "
                    span {
                        class: c_counter_value()
                        hooks_protect_phase_label(&boundary)
                    }
                }
            }
            euv_card {
                title: "Profiler"
                p {
                    class: c_render_count_text()
                    "Each render records a measurement via profiler_measure. Click to push more rows into the entries list."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Measure a slow op"
                        onclick: hooks_protect_profile_slow(profiler)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: "Clear"
                        onclick: hooks_protect_profile_clear(profiler)
                    }
                }
                p {
                    class: c_render_count_text()
                    "entries: "
                    span {
                        class: c_counter_value()
                        hooks_protect_entry_count(profiler)
                    }
                }
                p {
                    class: c_render_count_text()
                    "current render's trigger label: "
                    span {
                        class: c_counter_value()
                        trigger_label
                    }
                }
            }
        }
    }
}
