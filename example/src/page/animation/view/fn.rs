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
                title: ANIMATION_HEADER_TITLE
                subtitle: ANIMATION_HEADER_SUBTITLE
            }
            euv_card {
                title: ANIMATION_FADE_CARD_TITLE
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { box_visible } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: if { box_visible } {
                            ANIMATION_FADE_HIDE_LABEL
                        } else {
                            ANIMATION_FADE_SHOW_LABEL
                        }
                        onclick: UseEuvInput::use_toggle(box_visible)
                    }
                }
                div {
                    class: c_animation_fade()
                    class: if { box_visible } {
                        c_animation_fade()
                    } else {
                        c_animation_fade_hidden()
                    }
                    "aria-hidden": if { box_visible } {
                        "false"
                    } else {
                        "true"
                    }
                    ANIMATION_FADE_DEMO_TEXT
                }
            }
            euv_card {
                title: ANIMATION_SPIN_CARD_TITLE
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { spin_active } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: if { spin_active } {
                            ANIMATION_SPIN_STOP_LABEL
                        } else {
                            ANIMATION_SPIN_START_LABEL
                        }
                        onclick: UseEuvInput::use_toggle(spin_active)
                    }
                }
                div {
                    class: c_animation_stage()
                    div {
                        class: c_animation_symbol()
                        class: if { spin_active } {
                            c_animation_spin()
                        } else {
                            c_animation_symbol()
                        }
                        "⟳"
                    }
                }
            }
            euv_card {
                title: ANIMATION_PULSE_CARD_TITLE
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { pulse_active } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: if { pulse_active } {
                            ANIMATION_PULSE_STOP_LABEL
                        } else {
                            ANIMATION_PULSE_START_LABEL
                        }
                        onclick: UseEuvInput::use_toggle(pulse_active)
                    }
                }
                div {
                    class: c_animation_stage()
                    div {
                        class: c_animation_symbol()
                        class: if { pulse_active } {
                            c_animation_pulse()
                        } else {
                            c_animation_symbol()
                        }
                        "♥"
                    }
                }
            }
            euv_card {
                title: ANIMATION_PROGRESS_CARD_TITLE
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: ANIMATION_PROGRESS_START_LABEL
                        onclick: progress_on_start(progress)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: ANIMATION_PROGRESS_RESET_LABEL
                        onclick: progress_on_reset(progress)
                    }
                }
                div {
                    class: c_animation_progress_track()
                    role: "img"
                    "aria-label": ANIMATION_PROGRESS_CARD_TITLE
                    div {
                        class: c_animation_progress_fill()
                        class: if { progress.get_running().get() } {
                            if progress.get_generation().get().is_multiple_of(2) {
                                c_animation_progress_running()
                            } else {
                                c_animation_progress_replay()
                            }
                        } else {
                            c_animation_progress_fill()
                        }
                    }
                }
            }
            euv_card {
                title: ANIMATION_SCALE_CARD_TITLE
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { scale_active } {
                            EuvButtonVariant::Outline
                        } else {
                            EuvButtonVariant::Primary
                        }
                        label: if { scale_active } {
                            ANIMATION_SCALE_RESTORE_LABEL
                        } else {
                            ANIMATION_SCALE_SHRINK_LABEL
                        }
                        onclick: UseEuvInput::use_toggle(scale_active)
                    }
                }
                div {
                    class: c_animation_scale()
                    class: if { scale_active } {
                        c_animation_scale_shrink()
                    } else {
                        c_animation_scale()
                    }
                }
            }
        }
    }
}
