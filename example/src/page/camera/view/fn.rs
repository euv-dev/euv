use super::*;

/// A camera page component that opens the device camera with QR code
/// scanning on user action. When a QR code containing a valid HTTP or
/// HTTPS URL is detected, the app navigates to that URL.
///
/// For same-origin URLs the hash fragment is extracted and an internal
/// route navigation is performed; for external URLs a full page
/// navigation occurs via `location.href`.
///
/// Renders a header, a card with a video preview area, camera
/// control buttons (open/close and switch), and a QR code scan result
/// display area.
/// # Arguments
///
/// - `VirtualNode<PageCameraProps>` - The component props node.
///
/// # Returns
///
/// - `VirtualNode` - The camera page virtual DOM tree.
///
/// # Arguments
///
/// - `VirtualNode<PageCameraProps>` - The `node` argument.
///
#[component]
pub(crate) fn page_camera(node: VirtualNode<PageCameraProps>) -> VirtualNode {
    let _page_camera_props: PageCameraProps = node.try_get_props().unwrap_or_default();
    let state: UseEuvCamera = UseEuvCamera::use_camera_state();
    state.cleanup(None);
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "📷"
                title: "Camera"
                subtitle: "Open the device camera to scan QR codes. When a valid URL is detected, the app navigates to it automatically."
            }
            euv_card {
                title: "Camera Preview"
                p {
                    class: c_demo_text()
                    "Open the camera to start scanning QR codes. When a valid URL is detected, you will be redirected immediately."
                }
                div {
                    class: c_camera_video_container()
                    video {
                        id: "camera-video"
                        class: if { state.get_camera_open().get() } {
                            c_camera_video_active()
                        } else {
                            c_camera_video_hidden()
                        }
                        autoplay: CAMERA_VIDEO_AUTOPLAY
                        playsinline: CAMERA_VIDEO_PLAYSINLINE
                    }
                    if { state.get_camera_loading().get() } {
                        div {
                            class: c_camera_video_placeholder()
                            div {
                                class: c_camera_placeholder_content()
                                div {
                                    class: c_spinner()
                                }
                                p {
                                    class: c_camera_placeholder_text()
                                    "Opening camera..."
                                }
                            }
                        }
                    } else if { !state.get_camera_open().get() } {
                        div {
                            class: c_camera_video_placeholder()
                            div {
                                class: c_camera_placeholder_content()
                                span {
                                    class: c_camera_placeholder_icon()
                                    "📷"
                                }
                                p {
                                    class: c_camera_placeholder_text()
                                    "Camera preview will appear here"
                                }
                            }
                        }
                    }
                }
                if { !state.get_error_message().get().is_empty() } {
                    div {
                        class: c_camera_error_box()
                        state.get_error_message().get()
                    }
                }
                div {
                    class: c_button_controls()
                    if { state.get_camera_open().get() } {
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Close"
                            onclick: state.on_close(None)
                        }
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Switch"
                            onclick: state.on_switch(None)
                        }
                    } else if { state.get_camera_loading().get() } {
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Opening..."
                            disabled: state.camera_loading
                        }
                    } else {
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Open"
                            onclick: state.on_open(None)
                        }
                    }
                }
                if { !state.get_scan_result().get().is_empty() } {
                    div {
                        class: c_camera_scan_result_box()
                        div {
                            class: c_camera_scan_result_label()
                            "QR Code Result:"
                        }
                        div {
                            class: c_camera_scan_result_value()
                            state.get_scan_result().get()
                        }
                    }
                }
            }
        }
    }
}
