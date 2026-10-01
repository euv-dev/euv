use super::*;

/// An animation demo page showcasing CSS animations and transitions.
///
/// # Arguments
///
/// - `VirtualNode<PageAnimationProps>` - The page component node carrying the
///   page props.
///
/// # Returns
///
/// - `VirtualNode` - The animation demo page virtual DOM tree.
#[component]
pub(crate) fn page_animation(node: VirtualNode<PageAnimationProps>) -> VirtualNode {
    let PageAnimationProps: PageAnimationProps = node.try_get_props().unwrap_or_default();
    let box_visible: Signal<bool> = App::use_signal(|| false);
    let spin_active: Signal<bool> = App::use_signal(|| false);
    let pulse_active: Signal<bool> = App::use_signal(|| false);
    let progress: UseProgress = use_progress();
    let scale_active: Signal<bool> = App::use_signal(|| false);
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🎬"
                title: "Animation"
                subtitle: "CSS transitions, keyframe animations, and reactive style changes. Toggle each demo to see how euv binds Signal state to CSS classes and inline styles for smooth, declarative animations."
            }
            euv_card {
                title: "Fade In / Out"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { box_visible } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: if { box_visible } {
                            "Hide Element"
                        } else {
                            "Show Element"
                        }
                        onclick: UseEuvInput::use_toggle(box_visible)
                    }
                }
                if { box_visible } {
                    div {
                        class: c_anim_fade_in()
                        "This element fades in and out with a smooth CSS transition. The visibility is controlled by a Signal — when it becomes true, the node is inserted into the Virtual DOM with a fade-in animation."
                    }
                }
            }
            euv_card {
                title: "CSS Keyframe Spin"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { spin_active } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: if { spin_active } {
                            "Stop Spin"
                        } else {
                            "Start Spin"
                        }
                        onclick: UseEuvInput::use_toggle(spin_active)
                    }
                }
                div {
                    class: c_anim_spin_container()
                    div {
                        class: if { spin_active } {
                            c_anim_spin()
                        } else {
                            c_anim_spin_stopped()
                        }
                        "⟳"
                    }
                }
            }
            euv_card {
                title: "CSS Keyframe Pulse"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { pulse_active } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: if { pulse_active } {
                            "Stop Pulse"
                        } else {
                            "Start Pulse"
                        }
                        onclick: UseEuvInput::use_toggle(pulse_active)
                    }
                }
                div {
                    class: c_anim_pulse_container()
                    div {
                        class: if { pulse_active } {
                            c_anim_pulse()
                        } else {
                            c_anim_pulse_stopped()
                        }
                        "♥"
                    }
                }
            }
            euv_card {
                title: "Animated Progress Bar"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Start"
                        onclick: progress_on_start(progress)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Reset"
                        onclick: progress_on_reset(progress)
                    }
                }
                div {
                    class: c_progress_container()
                    div {
                        class: if { progress.get_running().get() } {
                            c_progress_bar_running()
                        } else {
                            c_progress_bar_stopped()
                        }
                    }
                }
            }
            euv_card {
                title: "Reactive Inline Style (Scale Transform)"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { scale_active } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: if { scale_active } {
                            "Restore"
                        } else {
                            "Shrink"
                        }
                        onclick: UseEuvInput::use_toggle(scale_active)
                    }
                }
                div {
                    class: c_anim_scale_box()
                    class: if { scale_active } {
                        c_anim_scale_shrink()
                    } else {
                        c_anim_scale_normal()
                    }
                }
            }
        }
    }
}
