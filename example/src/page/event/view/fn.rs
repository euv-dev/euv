use super::*;

/// An event handling demo page showcasing all supported browser event types.
///
/// # Returns
///
/// - `VirtualNode` - The event demo page virtual DOM tree.
#[component]
pub(crate) fn page_event(node: VirtualNode<PageEventProps>) -> VirtualNode {
    let PageEventProps: PageEventProps = node.try_get_props().unwrap_or_default();
    let keyboard: UseKeyboardEvent = use_keyboard_event();
    let mouse: UseMouseEvent = use_mouse_event();
    let focus: UseFocusEvent = use_focus_event();
    let drag: UseDragEvent = use_drag_event();
    let wheel: UseWheelEvent = use_wheel_event();
    let clipboard: UseClipboardEvent = use_clipboard_event();
    let touch: UseTouchEvent = use_touch_event();
    let form: UseFormEvent = use_form_event();
    let media: UseMediaEvent = use_media_event();
    let video: UseVideoEvent = use_video_event();
    let image: UseImageEvent = use_image_event();
    let current_url: String = current_url_without_params();
    let qr_code_data_url: String = generate_qr_code_data_url(&current_url);
    let on_key_down = move |event: Event| {
        if let Some(keyboard_event) = event.dyn_ref::<KeyboardEvent>() {
            let key_name: String = keyboard_event.key();
            keyboard.get_last_key().set(key_name);
            let code_name: String = keyboard_event.code();
            keyboard.get_last_key_code().set(code_name);
            let is_repeat: bool = keyboard_event.repeat();
            keyboard.get_key_repeat().set(is_repeat);
            let mut modifier: String = String::new();
            if keyboard_event.ctrl_key() {
                modifier.push_str("Ctrl+");
            }
            if keyboard_event.shift_key() {
                modifier.push_str("Shift+");
            }
            if keyboard_event.alt_key() {
                modifier.push_str("Alt+");
            }
            if keyboard_event.meta_key() {
                modifier.push_str("Meta+");
            }
            if modifier.is_empty() {
                modifier = "None".to_string();
            }
            keyboard.get_modifier().set(modifier);
            Console::log(format!(
                "KeyDown: {} (code: {})",
                keyboard.get_last_key().get(),
                keyboard.get_last_key_code().get()
            ));
        }
    };
    let on_key_up = move |event: Event| {
        if let Some(keyboard_event) = event.dyn_ref::<KeyboardEvent>() {
            let key_name: String = keyboard_event.key();
            keyboard.get_last_key_up().set(key_name.clone());
            Console::log(format!("KeyUp: {key_name}"));
        }
    };
    let on_mouse_click = move |event: Event| {
        if let Some(mouse_event) = event.dyn_ref::<MouseEvent>() {
            let pos: String = format!("({}, {})", mouse_event.client_x(), mouse_event.client_y());
            mouse.get_mouse_pos().set(pos);
            let screen: String =
                format!("({}, {})", mouse_event.screen_x(), mouse_event.screen_y());
            mouse.get_mouse_screen_pos().set(screen);
            let current: i32 = mouse.get_click_count().get();
            mouse.get_click_count().set(current + 1);
            Console::log(format!(
                "Click: {} at ({}, {})",
                current + 1,
                mouse_event.client_x(),
                mouse_event.client_y()
            ));
        }
    };
    let on_double_click = move |_: Event| {
        let current: i32 = mouse.get_double_click_count().get();
        mouse.get_double_click_count().set(current + 1);
        Console::log(format!("DblClick: #{}", current + 1));
    };
    let on_mouse_down = move |event: Event| {
        if let Some(mouse_event) = event.dyn_ref::<MouseEvent>() {
            let button_name: String = match mouse_event.button() {
                0 => "Left".to_string(),
                1 => "Middle".to_string(),
                2 => "Right".to_string(),
                _ => format!("Button {}", mouse_event.button()),
            };
            mouse.get_mouse_button().set(button_name);
            let current: i32 = mouse.get_mouse_down_count().get();
            mouse.get_mouse_down_count().set(current + 1);
        }
    };
    let on_mouse_up = move |_: Event| {
        let current: i32 = mouse.get_mouse_up_count().get();
        mouse.get_mouse_up_count().set(current + 1);
    };
    let on_mouse_move = move |event: Event| {
        if let Some(mouse_event) = event.dyn_ref::<MouseEvent>() {
            let pos: String = format!("({}, {})", mouse_event.client_x(), mouse_event.client_y());
            mouse.get_mouse_pos().set(pos);
            let buttons_mask: String = format!("{}", mouse_event.buttons());
            mouse.get_mouse_buttons().set(buttons_mask);
        }
    };
    let on_mouse_enter = move |_: Event| {
        let current: i32 = mouse.get_mouse_enter_count().get();
        mouse.get_mouse_enter_count().set(current + 1);
    };
    let on_mouse_leave = move |_: Event| {
        let current: i32 = mouse.get_mouse_leave_count().get();
        mouse.get_mouse_leave_count().set(current + 1);
    };
    let on_context_menu = move |event: Event| {
        if event.dyn_ref::<MouseEvent>().is_some() {
            Console::log("ContextMenu: right-click detected");
        }
    };
    let on_mouse_over = move |_: Event| {
        let current: i32 = mouse.get_mouse_over_count().get();
        mouse.get_mouse_over_count().set(current + 1);
    };
    let on_mouse_out = move |_: Event| {
        let current: i32 = mouse.get_mouse_out_count().get();
        mouse.get_mouse_out_count().set(current + 1);
    };
    let on_focus = move |_: Event| {
        focus.get_focus_status().set("Focused".to_string());
        let current: i32 = focus.get_focus_in_count().get();
        focus.get_focus_in_count().set(current + 1);
        Console::log("Focus: input gained focus");
    };
    let on_blur = move |_: Event| {
        focus.get_focus_status().set("Not focused".to_string());
        let current: i32 = focus.get_focus_out_count().get();
        focus.get_focus_out_count().set(current + 1);
        Console::log("Blur: input lost focus");
    };
    let on_focus_in = move |_: Event| {
        Console::log("FocusIn: focus entered");
    };
    let on_focus_out = move |_: Event| {
        Console::log("FocusOut: focus left");
    };
    let on_drag_start = move |_: Event| {
        drag.get_drag_status().set("Dragging".to_string());
        drag.get_drag_enter_counter().set(1);
        Console::log("DragStart: drag started");
    };
    let on_drag = move |event: Event| {
        if drag.get_drag_enter_counter().get() > 0
            && let Some(drag_event) = event.dyn_ref::<DragEvent>()
        {
            let pos: String = format!("({}, {})", drag_event.client_x(), drag_event.client_y());
            drag.get_drag_pending_pos().set(pos);
            if drag.get_drag_raf_id().get() == -1 {
                let pos_signal: Signal<String> = drag.get_drag_pos();
                let pending_signal: Signal<String> = drag.get_drag_pending_pos();
                let raf_id_signal: Signal<i32> = drag.get_drag_raf_id();
                let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                    pos_signal.set(pending_signal.get());
                    raf_id_signal.set(-1);
                }));
                let Some(window): Option<Window> = window() else {
                    return;
                };
                let id: i32 = window
                    .request_animation_frame(closure.as_ref().unchecked_ref())
                    .unwrap_or(-1);
                drag.get_drag_raf_id().set(id);
                closure.forget();
            }
        }
    };
    let on_drag_end = move |_: Event| {
        let raf_id: i32 = drag.get_drag_raf_id().get();
        if raf_id != -1 {
            let Some(window): Option<Window> = window() else {
                return;
            };
            let _: Result<(), JsValue> = window.cancel_animation_frame(raf_id);
            drag.get_drag_raf_id().set(-1);
        }
        if !drag.get_drag_pending_pos().get().is_empty() {
            drag.get_drag_pos().set(drag.get_drag_pending_pos().get());
        }
        drag.get_drag_status().set("Ended".to_string());
        drag.get_drag_enter_counter().set(0);
        Console::log("DragEnd: drag ended");
    };
    let on_drag_over = move |event: Event| {
        event.prevent_default();
    };
    let on_drag_enter = move |_: Event| {
        let counter: i32 = drag.get_drag_enter_counter().get();
        drag.get_drag_enter_counter().set(counter + 1);
        if counter == 0 {
            drag.get_drag_status().set("Dragging".to_string());
            Console::log("DragEnter: entered drop zone");
        }
    };
    let on_drag_leave = move |_: Event| {
        let counter: i32 = drag.get_drag_enter_counter().get();
        drag.get_drag_enter_counter().set(counter - 1);
        if counter <= 1 {
            drag.get_drag_status().set("Outside".to_string());
            Console::log("DragLeave: left drop zone");
        }
    };
    let on_drop = move |event: Event| {
        if let Some(drag_event) = event.dyn_ref::<DragEvent>() {
            let types_str: String = drag_event
                .data_transfer()
                .map(|data_transfer: DataTransfer| {
                    let type_count: u32 = data_transfer.types().length();
                    (0..type_count)
                        .filter_map(|index: u32| data_transfer.types().get(index).as_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                })
                .unwrap_or_default();
            if types_str.is_empty() {
                drag.get_drag_types().set("None".to_string());
            } else {
                drag.get_drag_types().set(types_str);
            }
        }
        drag.get_drag_status().set("Dropped".to_string());
        Console::log("Drop: item dropped");
    };
    let file_drag_over: Signal<bool> = App::use_signal(|| false);
    let on_file_drag_over = move |event: Event| {
        event.prevent_default();
        file_drag_over.set(true);
    };
    let on_file_drag_enter = move |_: Event| {
        file_drag_over.set(true);
        drag.get_drag_status().set("File over zone".to_string());
        Console::log("DragEnter: file entered drop zone");
    };
    let on_file_drag_leave = move |_: Event| {
        file_drag_over.set(false);
        drag.get_drag_status().set("Outside".to_string());
        Console::log("DragLeave: file left drop zone");
    };
    let on_file_drop = move |event: Event| {
        event.prevent_default();
        file_drag_over.set(false);
        if let Some(drag_event) = event.dyn_ref::<DragEvent>() {
            let file_names: String = drag_event
                .data_transfer()
                .and_then(|data_transfer: DataTransfer| data_transfer.files())
                .map(|file_list: FileList| {
                    let count: u32 = file_list.length();
                    (0..count)
                        .filter_map(|index: u32| file_list.get(index).map(|file: File| file.name()))
                        .collect::<Vec<String>>()
                        .join(", ")
                })
                .unwrap_or_default();
            if file_names.is_empty() {
                drag.get_drag_types().set("No files".to_string());
            } else {
                drag.get_drag_types().set(file_names);
            }
        }
        drag.get_drag_status().set("Dropped".to_string());
        Console::log("Drop: files dropped");
    };
    let on_wheel = move |event: Event| {
        if let Some(wheel_event) = event.dyn_ref::<WheelEvent>() {
            let delta: String = format!(
                "({:.1}, {:.1})",
                wheel_event.delta_x(),
                wheel_event.delta_y()
            );
            wheel.get_wheel_delta().set(delta);
            let current: f64 = wheel.get_wheel_total().get();
            wheel.get_wheel_total().set(current + wheel_event.delta_y());
            let mode_name: String = match wheel_event.delta_mode() {
                0 => "pixel".to_string(),
                1 => "line".to_string(),
                2 => "page".to_string(),
                _ => "unknown".to_string(),
            };
            Console::log(format!(
                "Wheel: dx={:.1}, dy={:.1}, mode={}",
                wheel_event.delta_x(),
                wheel_event.delta_y(),
                mode_name
            ));
        }
    };
    let on_copy = move |event: Event| {
        clipboard.get_clipboard_event_type().set("Copy".to_string());
        if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
            let data: Option<String> = clipboard_event
                .clipboard_data()
                .and_then(|cd: DataTransfer| cd.get_data("text").ok());
            clipboard
                .get_clipboard_data()
                .set(data.unwrap_or_else(|| "No data".to_string()));
        }
        Console::log("Copy: text copied");
    };
    let on_cut = move |event: Event| {
        clipboard.get_clipboard_event_type().set("Cut".to_string());
        if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
            let data: Option<String> = clipboard_event
                .clipboard_data()
                .and_then(|cd: DataTransfer| cd.get_data("text").ok());
            clipboard
                .get_clipboard_data()
                .set(data.unwrap_or_else(|| "No data".to_string()));
        }
        Console::log("Cut: text cut");
    };
    let on_paste = move |event: Event| {
        clipboard
            .get_clipboard_event_type()
            .set("Paste".to_string());
        if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
            let data: Option<String> = clipboard_event
                .clipboard_data()
                .and_then(|cd: DataTransfer| cd.get_data("text").ok());
            clipboard
                .get_clipboard_data()
                .set(data.unwrap_or_else(|| "No data".to_string()));
        }
        Console::log("Paste: text pasted");
    };
    let on_touch_start = move |event: Event| {
        let points: Vec<NativeTouchPoint> = NativeTouchPoint::extract_all(&event);
        let detail: String = points
            .iter()
            .map(|point: &NativeTouchPoint| {
                format!(
                    "#{}({}, {})",
                    point.get_identifier(),
                    point.get_client_x(),
                    point.get_client_y()
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let info: String = format!("Start: {} touches [{}]", points.len(), detail);
        touch.get_touch_info().set(info);
        Console::log(format!("TouchStart: {} touches", points.len()));
    };
    let on_touch_move = move |event: Event| {
        let points: Vec<NativeTouchPoint> = NativeTouchPoint::extract_all(&event);
        let detail: String = points
            .iter()
            .map(|point: &NativeTouchPoint| {
                format!(
                    "#{}({}, {})",
                    point.get_identifier(),
                    point.get_client_x(),
                    point.get_client_y()
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let info: String = format!("Move: {} touches [{}]", points.len(), detail);
        touch.get_touch_info().set(info);
    };
    let on_touch_end = move |event: Event| {
        let remaining: Vec<NativeTouchPoint> = NativeTouchPoint::extract_all(&event);
        let changed: Vec<NativeTouchPoint> = NativeTouchPoint::extract_changed(&event);
        let detail: String = changed
            .iter()
            .map(|point: &NativeTouchPoint| {
                format!(
                    "#{}({}, {})",
                    point.get_identifier(),
                    point.get_client_x(),
                    point.get_client_y()
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let info: String = format!("End: {} remaining, lifted [{}]", remaining.len(), detail);
        touch.get_touch_info().set(info);
        Console::log(format!("TouchEnd: {} remaining", remaining.len()));
    };
    let on_touch_cancel = move |event: Event| {
        let changed: Vec<NativeTouchPoint> = NativeTouchPoint::extract_changed(&event);
        let detail: String = changed
            .iter()
            .map(|point: &NativeTouchPoint| {
                format!(
                    "#{}({}, {})",
                    point.get_identifier(),
                    point.get_client_x(),
                    point.get_client_y()
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let info: String = format!("Cancel: [{detail}]");
        touch.get_touch_info().set(info);
        Console::log(format!("TouchCancel: [{detail}]"));
    };
    let on_form_submit = move |event: Event| {
        event.prevent_default();
        let current: i32 = form.get_submit_count().get();
        form.get_submit_count().set(current + 1);
        Console::log(format!("Form submitted #{}", current + 1));
    };
    let on_euv_input = move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            form.get_euv_input_value().set(input.value());
        }
    };
    let on_form_change = move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            form.get_form_change_value().set(input.value());
        }
    };
    let on_checkbox_change = move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            form.get_form_checkbox().set(input.checked());
        }
    };
    let on_select_change = move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>()
        {
            form.get_form_select_value().set(select.value());
        }
    };
    let on_audio_play = move |_: Event| {
        media.get_media_status().set("Playing".to_string());
        media.get_media_event_log().set("Play".to_string());
        Console::log("Play: audio started");
    };
    let on_audio_pause = move |_: Event| {
        media.get_media_status().set("Paused".to_string());
        media.get_media_event_log().set("Pause".to_string());
        Console::log("Pause: audio paused");
    };
    let on_audio_ended = move |_: Event| {
        media.get_media_status().set("Ended".to_string());
        media.get_media_event_log().set("Ended".to_string());
        Console::log("Ended: audio ended");
    };
    let on_audio_loaded_data = move |_: Event| {
        media.get_media_status().set("Loaded".to_string());
        media.get_media_event_log().set("LoadedData".to_string());
    };
    let on_audio_can_play = move |_: Event| {
        media.get_media_event_log().set("CanPlay".to_string());
    };
    let on_audio_volume_change = move |_: Event| {
        media.get_media_event_log().set("VolumeChange".to_string());
        Console::log("VolumeChange: volume changed");
    };
    let on_audio_time_update = move |_: Event| {
        media.get_media_event_log().set("TimeUpdate".to_string());
    };
    let on_video_play = move |_: Event| {
        video.get_video_status().set("Playing".to_string());
        video.get_video_event_log().set("Play".to_string());
        Console::log("Video Play: video started");
    };
    let on_video_pause = move |_: Event| {
        video.get_video_status().set("Paused".to_string());
        video.get_video_event_log().set("Pause".to_string());
        Console::log("Video Pause: video paused");
    };
    let on_video_ended = move |_: Event| {
        video.get_video_status().set("Ended".to_string());
        video.get_video_event_log().set("Ended".to_string());
        Console::log("Video Ended: video ended");
    };
    let on_video_loaded_data = move |_: Event| {
        video.get_video_status().set("Data Loaded".to_string());
        video.get_video_event_log().set("LoadedData".to_string());
        Console::log("Video LoadedData: data loaded");
    };
    let on_video_loaded_metadata = move |_: Event| {
        video.get_video_status().set("Meta Loaded".to_string());
        video
            .get_video_event_log()
            .set("LoadedMetadata".to_string());
        Console::log("Video LoadedMetadata: metadata loaded");
    };
    let on_video_can_play = move |_: Event| {
        video.get_video_event_log().set("CanPlay".to_string());
        Console::log("Video CanPlay: can play");
    };
    let on_video_can_play_through = move |_: Event| {
        video
            .get_video_event_log()
            .set("CanPlayThrough".to_string());
        Console::log("Video CanPlayThrough: can play through");
    };
    let on_video_waiting = move |_: Event| {
        video.get_video_status().set("Waiting".to_string());
        video.get_video_event_log().set("Waiting".to_string());
        Console::log("Video Waiting: buffering");
    };
    let on_video_playing = move |_: Event| {
        video.get_video_status().set("Playing".to_string());
        video.get_video_event_log().set("Playing".to_string());
        Console::log("Video Playing: playback resumed");
    };
    let on_video_time_update = move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(video_el) = target.clone().dyn_into::<HtmlMediaElement>()
        {
            let current: String = format!("{:.2}", video_el.current_time());
            video.get_video_current_time().set(current);
        }
        video.get_video_event_log().set("TimeUpdate".to_string());
    };
    let on_video_duration_change = move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(video_el) = target.clone().dyn_into::<HtmlMediaElement>()
        {
            let dur: String = format!("{:.2}", video_el.duration());
            video.get_video_duration().set(dur);
        }
        video
            .get_video_event_log()
            .set("DurationChange".to_string());
        Console::log("Video DurationChange: duration changed");
    };
    let on_video_progress = move |_: Event| {
        video.get_video_event_log().set("Progress".to_string());
    };
    let on_video_seeking = move |_: Event| {
        video.get_video_event_log().set("Seeking".to_string());
        Console::log("Video Seeking: seeking started");
    };
    let on_video_seeked = move |_: Event| {
        video.get_video_event_log().set("Seeked".to_string());
        Console::log("Video Seeked: seek completed");
    };
    let on_video_volume_change = move |_: Event| {
        video.get_video_event_log().set("VolumeChange".to_string());
        Console::log("Video VolumeChange: volume changed");
    };
    let on_video_rate_change = move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(video_el) = target.clone().dyn_into::<HtmlMediaElement>()
        {
            let rate: String = format!("{}", video_el.playback_rate());
            video.get_video_playback_rate().set(rate);
        }
        video.get_video_event_log().set("RateChange".to_string());
        Console::log("Video RateChange: playback rate changed");
    };
    let on_video_emptied = move |_: Event| {
        video.get_video_event_log().set("Emptied".to_string());
        Console::log("Video Emptied: media emptied");
    };
    let on_video_stalled = move |_: Event| {
        video.get_video_event_log().set("Stalled".to_string());
        Console::log("Video Stalled: data transfer stalled");
    };
    let on_video_suspend = move |_: Event| {
        video.get_video_event_log().set("Suspend".to_string());
        Console::log("Video Suspend: data transfer suspended");
    };
    let on_video_load_start = move |_: Event| {
        video.get_video_event_log().set("LoadStart".to_string());
        Console::log("Video LoadStart: loading started");
    };
    let on_video_error = move |_: Event| {
        video.get_video_status().set("Error".to_string());
        video.get_video_event_log().set("Error".to_string());
        Console::log("Video Error: error occurred");
    };
    let on_image_load = move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(img_el) = target.clone().dyn_into::<HtmlImageElement>()
        {
            let size: String = format!("{}x{}", img_el.natural_width(), img_el.natural_height());
            image.get_image_natural_size().set(size);
        }
        image.get_image_status().set("Loaded".to_string());
        image.get_image_event_log().set("Load".to_string());
        Console::log("Image Load: image loaded successfully");
    };
    let on_image_error = move |_: Event| {
        image.get_image_status().set("Error".to_string());
        image.get_image_event_log().set("Error".to_string());
        Console::log("Image Error: failed to load image");
    };
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🎯"
                title: "Event Handling"
                subtitle: "Complete browser event demo covering keyboard, mouse, focus, drag-and-drop, wheel, clipboard, touch, form, media, video, and image events."
            }
            euv_card {
                title: "Keyboard Events"
                input {
                    id: EVENT_KEYBOARD_ID
                    name: EVENT_KEYBOARD_NAME
                    type: EVENT_TEXT_TYPE
                    autocomplete: EVENT_AUTOCOMPLETE_OFF
                    placeholder: EVENT_KEYBOARD_PLACEHOLDER
                    class: c_euv_input()
                    onkeydown: on_key_down
                    onkeyup: on_key_up
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "KeyDown:"
                        }
                        span {
                            class: c_event_info_value()
                            keyboard.get_last_key()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "KeyCode:"
                        }
                        span {
                            class: c_event_info_value()
                            keyboard.get_last_key_code()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "KeyUp:"
                        }
                        span {
                            class: c_event_info_value()
                            keyboard.get_last_key_up()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Repeat:"
                        }
                        span {
                            class: c_event_info_value()
                            keyboard.get_key_repeat()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Modifiers:"
                        }
                        span {
                            class: c_event_info_value()
                            keyboard.get_modifier()
                        }
                    }
                }
            }
            euv_card {
                title: "Mouse Events"
                div {
                    class: c_event_mouse_area()
                    onclick: on_mouse_click
                    ondblclick: on_double_click
                    onmousedown: on_mouse_down
                    onmouseup: on_mouse_up
                    onmousemove: on_mouse_move
                    onmouseenter: on_mouse_enter
                    onmouseleave: on_mouse_leave
                    oncontextmenu: on_context_menu
                    p {
                        class: c_demo_text()
                        "Click, double-click, right-click, or move your mouse within this area to track mouse events."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Tracks click, dblclick, mousedown, mouseup, mousemove, mouseenter, mouseleave, and contextmenu events."
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Clicks:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_click_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "DblClicks:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_double_click_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "MouseDown:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_down_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "MouseUp:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_up_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Client:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_pos()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Screen:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_screen_pos()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Button:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_button()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Buttons:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_buttons()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Enter:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_enter_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Leave:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_leave_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Over:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_over_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Out:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_out_count()
                        }
                    }
                }
            }
            euv_card {
                title: "Mouse Over/Out Events"
                div {
                    class: c_switcher()
                    div {
                        class: c_event_drag_zone()
                        class: c_switcher_col()
                        onmouseover: on_mouse_over
                        p {
                            class: c_demo_text()
                            "Mouse Over zone"
                        }
                        p {
                            class: c_demo_text_muted()
                            "Move mouse over this area"
                        }
                    }
                    div {
                        class: c_event_drag_zone_active()
                        class: c_switcher_col()
                        onmouseout: on_mouse_out
                        p {
                            class: c_demo_text()
                            "Mouse Out zone"
                        }
                        p {
                            class: c_demo_text_muted()
                            "Move mouse out of this area"
                        }
                    }
                }
            }
            euv_card {
                title: "Focus Events"
                input {
                    id: EVENT_FOCUS_ID
                    name: EVENT_FOCUS_NAME
                    type: EVENT_TEXT_TYPE
                    autocomplete: EVENT_AUTOCOMPLETE_OFF
                    placeholder: EVENT_FOCUS_PLACEHOLDER
                    class: c_euv_input()
                    onfocus: on_focus
                    onblur: on_blur
                    onfocusin: on_focus_in
                    onfocusout: on_focus_out
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Status:"
                        }
                        span {
                            class: c_event_info_value()
                            focus.get_focus_status()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "FocusIn:"
                        }
                        span {
                            class: c_event_info_value()
                            focus.get_focus_in_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "FocusOut:"
                        }
                        span {
                            class: c_event_info_value()
                            focus.get_focus_out_count()
                        }
                    }
                }
            }
            euv_card {
                title: "Drag Events"
                div {
                    class: c_event_drag_zone()
                    ondragstart: on_drag_start
                    ondrag: on_drag
                    ondragend: on_drag_end
                    ondragover: on_drag_over
                    ondragenter: on_drag_enter
                    ondragleave: on_drag_leave
                    ondrop: on_drop
                    div {
                        class: c_event_drag_item()
                        draggable: EVENT_DRAGGABLE_TRUE
                        "Drag Me"
                    }
                    p {
                        class: c_demo_text_muted()
                        "dragstart, drag, dragend, dragover, dragenter, dragleave, drop"
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Status:"
                        }
                        span {
                            class: c_event_info_value()
                            drag.get_drag_status()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Position:"
                        }
                        span {
                            class: c_event_info_value()
                            drag.get_drag_pos()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Types:"
                        }
                        span {
                            class: c_event_info_value()
                            drag.get_drag_types()
                        }
                    }
                }
            }
            euv_card {
                title: "File Drag & Drop"
                div {
                    class: if { file_drag_over } {
                        c_event_drop_zone_active()
                    } else {
                        c_event_drop_zone()
                    }
                    ondragover: on_file_drag_over
                    ondragenter: on_file_drag_enter
                    ondragleave: on_file_drag_leave
                    ondrop: on_file_drop
                    span {
                        class: c_event_drop_icon()
                        "📁"
                    }
                    p {
                        class: c_event_drop_text()
                        "Drag & drop files here"
                    }
                    p {
                        class: c_event_drop_hint()
                        "dragover, dragenter, dragleave, drop"
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Status:"
                        }
                        span {
                            class: c_event_info_value()
                            drag.get_drag_status()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Files:"
                        }
                        span {
                            class: c_event_info_value()
                            drag.get_drag_types()
                        }
                    }
                }
            }
            euv_card {
                title: "Wheel Event"
                div {
                    class: c_event_wheel_zone()
                    onwheel: on_wheel
                    p {
                        class: c_demo_text()
                        "Scroll the mouse wheel within this area to track wheel deltas and scroll mode."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Tracks wheel delta (deltaX, deltaY) and delta mode (pixel, line, or page)."
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Delta:"
                        }
                        span {
                            class: c_event_info_value()
                            wheel.get_wheel_delta()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Total Y:"
                        }
                        span {
                            class: c_event_info_value()
                            wheel.get_wheel_total()
                        }
                    }
                }
            }
            euv_card {
                title: "Clipboard Events"
                div {
                    class: c_event_clipboard_area()
                    input {
                        id: EVENT_CLIPBOARD_ID
                        name: EVENT_CLIPBOARD_NAME
                        type: EVENT_TEXT_TYPE
                        autocomplete: EVENT_AUTOCOMPLETE_OFF
                        placeholder: EVENT_CLIPBOARD_PLACEHOLDER
                        class: c_euv_input()
                        value: "Sample text for clipboard"
                        oncopy: on_copy
                        oncut: on_cut
                        onpaste: on_paste
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Event:"
                        }
                        span {
                            class: c_event_info_value()
                            clipboard.get_clipboard_event_type()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Data:"
                        }
                        span {
                            class: c_event_info_value()
                            clipboard.get_clipboard_data()
                        }
                    }
                }
            }
            euv_card {
                title: "Touch Events"
                div {
                    class: c_event_touch_zone()
                    ontouchstart: on_touch_start
                    ontouchmove: on_touch_move
                    ontouchend: on_touch_end
                    ontouchcancel: on_touch_cancel
                    p {
                        class: c_demo_text()
                        "Touch this area on a mobile device or touchscreen to track touch events."
                    }
                    p {
                        class: c_demo_text_muted()
                        "Tracks touchstart, touchmove, touchend, and touchcancel events with touch point details."
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Touch:"
                        }
                        span {
                            class: c_event_info_value()
                            touch.get_touch_info()
                        }
                    }
                }
            }
            euv_card {
                title: "Form Events"
                div {
                    class: c_event_form_area()
                    form {
                        onsubmit: on_form_submit
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                for: EVENT_FORM_INPUT_ID
                                class: c_form_label()
                                "Input (oninput & onchange)"
                            }
                            input {
                                type: EVENT_TEXT_TYPE
                                id: EVENT_FORM_INPUT_ID
                                name: EVENT_FORM_INPUT_NAME
                                autocomplete: EVENT_AUTOCOMPLETE_OFF
                                placeholder: EVENT_FORM_INPUT_PLACEHOLDER
                                class: c_euv_input()
                                oninput: on_euv_input
                                onchange: on_form_change
                            }
                        }
                        div {
                            class: c_form_checkbox_row()
                            input {
                                id: EVENT_FORM_CHECKBOX_ID
                                name: EVENT_FORM_CHECKBOX_NAME
                                type: EVENT_CHECKBOX_TYPE
                                autocomplete: EVENT_AUTOCOMPLETE_OFF
                                class: c_form_checkbox()
                                onchange: on_checkbox_change
                            }
                            label {
                                for: EVENT_FORM_CHECKBOX_ID
                                class: c_form_checkbox_label()
                                "Checkbox (onchange)"
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                for: EVENT_FORM_SELECT_ID
                                class: c_form_label()
                                "Select (onchange)"
                            }
                            select {
                                id: EVENT_FORM_SELECT_ID
                                name: EVENT_FORM_SELECT_NAME
                                autocomplete: EVENT_AUTOCOMPLETE_OFF
                                class: c_select_input()
                                onchange: on_select_change
                                option {
                                    value: ""
                                    "-- Choose --"
                                }
                                option {
                                    value: "alpha"
                                    "Alpha"
                                }
                                option {
                                    value: "beta"
                                    "Beta"
                                }
                                option {
                                    value: "gamma"
                                    "Gamma"
                                }
                            }
                        }
                        div {
                            class: c_button_controls()
                            euv_button {
                                variant: EuvButtonVariant::Primary
                                label: "Submit"
                            }
                        }
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Input:"
                        }
                        span {
                            class: c_event_info_value()
                            form.get_euv_input_value()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Change:"
                        }
                        span {
                            class: c_event_info_value()
                            form.get_form_change_value()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Checked:"
                        }
                        span {
                            class: c_event_info_value()
                            form.get_form_checkbox()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Select:"
                        }
                        span {
                            class: c_event_info_value()
                            form.get_form_select_value()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Submits:"
                        }
                        span {
                            class: c_event_info_value()
                            form.get_submit_count()
                        }
                    }
                }
            }
            euv_card {
                title: "Audio Media Events"
                div {
                    class: c_event_media_area()
                    audio {
                        class: c_event_audio()
                        controls: EVENT_CONTROLS_TRUE
                        src: EVENT_AUDIO_SRC
                        onplay: on_audio_play
                        onpause: on_audio_pause
                        onended: on_audio_ended
                        onloadeddata: on_audio_loaded_data
                        oncanplay: on_audio_can_play
                        onvolumechange: on_audio_volume_change
                        ontimeupdate: on_audio_time_update
                        p {
                            class: c_demo_text_muted()
                            "Audio player with play, pause, ended, loadeddata, canplay, volumechange, timeupdate events"
                        }
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Status:"
                        }
                        span {
                            class: c_event_info_value()
                            media.get_media_status()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Last Event:"
                        }
                        span {
                            class: c_event_info_value()
                            media.get_media_event_log()
                        }
                    }
                }
            }
            euv_card {
                title: "Video Events"
                div {
                    class: c_event_video_area()
                    video {
                        id: EVENT_VIDEO_ID
                        class: c_event_video()
                        controls: EVENT_CONTROLS_TRUE
                        preload: EVENT_PRELOAD_METADATA
                        src: EVENT_VIDEO_SRC
                        onplay: on_video_play
                        onpause: on_video_pause
                        onended: on_video_ended
                        onloadeddata: on_video_loaded_data
                        onloadedmetadata: on_video_loaded_metadata
                        oncanplay: on_video_can_play
                        oncanplaythrough: on_video_can_play_through
                        onwaiting: on_video_waiting
                        onplaying: on_video_playing
                        ontimeupdate: on_video_time_update
                        ondurationchange: on_video_duration_change
                        onprogress: on_video_progress
                        onseeking: on_video_seeking
                        onseeked: on_video_seeked
                        onvolumechange: on_video_volume_change
                        onratechange: on_video_rate_change
                        onemptied: on_video_emptied
                        onstalled: on_video_stalled
                        onsuspend: on_video_suspend
                        onloadstart: on_video_load_start
                        onerror: on_video_error
                        p {
                            class: c_demo_text_muted()
                            "Video player with play, pause, ended, loadeddata, loadedmetadata, canplay, canplaythrough, waiting, playing, timeupdate, durationchange, progress, seeking, seeked, volumechange, ratechange, emptied, stalled, suspend, loadstart, error events"
                        }
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Status:"
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_status()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Last Event:"
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_event_log()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Current Time:"
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_current_time()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Duration:"
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_duration()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Playback Rate:"
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_playback_rate()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Buffered:"
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_buffered()
                        }
                    }
                }
            }
            euv_card {
                title: "Image Events"
                div {
                    class: c_event_image_area()
                    img {
                        id: EVENT_IMAGE_ID
                        class: c_event_image()
                        src: qr_code_data_url
                        alt: EVENT_IMAGE_ALT
                        onload: on_image_load
                        onerror: on_image_error
                    }
                    p {
                        class: c_event_url_text()
                        current_url
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Status:"
                        }
                        span {
                            class: c_event_info_value()
                            image.get_image_status()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Last Event:"
                        }
                        span {
                            class: c_event_info_value()
                            image.get_image_event_log()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Natural Size:"
                        }
                        span {
                            class: c_event_info_value()
                            image.get_image_natural_size()
                        }
                    }
                }
            }
        }
    }
}
