use super::*;

/// A page demonstrating the "protective" hooks
/// ([`ErrorBoundary`] and [`ProfilerHandle`]).
///
/// # Arguments
///
/// - `VirtualNode<PageHooksProtectProps>` - The page component node carrying
///   the page props.
///
/// # Returns
///
/// - `VirtualNode` - The protective hooks page virtual DOM tree.
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
                title: HOOKS_PROTECT_HEADER_TITLE
                subtitle: HOOKS_PROTECT_HEADER_SUBTITLE
            }
            euv_card {
                title: HOOKS_PROTECT_BOUNDARY_CARD_TITLE
                p {
                    class: c_render_count_text()
                    HOOKS_PROTECT_BOUNDARY_CARD_DESCRIPTION
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { hooks_protect_is_healthy(&boundary) } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: HOOKS_PROTECT_TRY_HEALTHY_LABEL
                        onclick: hooks_protect_try_healthy(boundary)
                    }
                    euv_button {
                        variant: if { hooks_protect_is_caught(&boundary) } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: HOOKS_PROTECT_TRY_PANIC_LABEL
                        onclick: hooks_protect_try_panic(boundary)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: HOOKS_PROTECT_BOUNDARY_RESET_LABEL
                        onclick: hooks_protect_reset(boundary)
                    }
                }
                p {
                    class: c_render_count_text()
                    HOOKS_PROTECT_PHASE_PREFIX
                    span {
                        class: c_counter_value()
                        hooks_protect_phase_label(&boundary)
                    }
                }
            }
            euv_card {
                title: HOOKS_PROTECT_PROFILER_CARD_TITLE
                p {
                    class: c_render_count_text()
                    HOOKS_PROTECT_PROFILER_CARD_DESCRIPTION
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: HOOKS_PROTECT_PROFILER_MEASURE_LABEL
                        onclick: hooks_protect_profile_slow(profiler)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: HOOKS_PROTECT_PROFILER_CLEAR_LABEL
                        onclick: hooks_protect_profile_clear(profiler)
                    }
                }
                p {
                    class: c_render_count_text()
                    HOOKS_PROTECT_ENTRIES_PREFIX
                    span {
                        class: c_counter_value()
                        hooks_protect_entry_count(profiler)
                    }
                }
                p {
                    class: c_render_count_text()
                    HOOKS_PROTECT_TRIGGER_LABEL_PREFIX
                    span {
                        class: c_counter_value()
                        trigger_label
                    }
                }
            }
        }
    }
}
