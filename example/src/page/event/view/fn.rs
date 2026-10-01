use super::*;

/// An event handling demo page showcasing all supported browser event types.
///
/// # Returns
///
/// - `VirtualNode` - The event demo page virtual DOM tree.
/// # Arguments
///
/// - `VirtualNode<PageEventProps>` - The VirtualNode<PageEventProps> parameter.
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
    let on_key_down: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        if let Some(keyboard_event) = event.dyn_ref::<KeyboardEvent>() {
            let key_name: String = keyboard_event.key();
            keyboard.get_last_key().set(key_name);
            let code_name: String = keyboard_event.code();
            keyboard.get_last_key_code().set(code_name);
            let is_repeat: bool = keyboard_event.repeat();
            keyboard.get_key_repeat().set(is_repeat);
            let mut modifier: String = String::new();
            if keyboard_event.ctrl_key() {
                modifier.push_str(EVENT_MODIFIER_CTRL);
            }
            if keyboard_event.shift_key() {
                modifier.push_str(EVENT_MODIFIER_SHIFT);
            }
            if keyboard_event.alt_key() {
                modifier.push_str(EVENT_MODIFIER_ALT);
            }
            if keyboard_event.meta_key() {
                modifier.push_str(EVENT_MODIFIER_META);
            }
            if modifier.is_empty() {
                modifier = EVENT_MODIFIER_NONE.to_string();
            }
            keyboard.get_modifier().set(modifier);
            Console::log(format!(
                "KeyDown: {} (code: {})",
                keyboard.get_last_key().get(),
                keyboard.get_last_key_code().get()
            ));
        }
    });
    let on_key_up: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        if let Some(keyboard_event) = event.dyn_ref::<KeyboardEvent>() {
            let key_name: String = keyboard_event.key();
            keyboard.get_last_key_up().set(key_name.clone());
            Console::log(format!("KeyUp: {key_name}"));
        }
    });
    let on_mouse_click: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
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
    });
    let on_double_click: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        let current: i32 = mouse.get_double_click_count().get();
        mouse.get_double_click_count().set(current + 1);
        Console::log(format!("DblClick: #{}", current + 1));
    });
    let on_mouse_down: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        if let Some(mouse_event) = event.dyn_ref::<MouseEvent>() {
            let button_name: String = match mouse_event.button() {
                0 => EVENT_MOUSE_BUTTON_LEFT.to_string(),
                1 => EVENT_MOUSE_BUTTON_MIDDLE.to_string(),
                2 => EVENT_MOUSE_BUTTON_RIGHT.to_string(),
                _ => format!("Button {}", mouse_event.button()),
            };
            mouse.get_mouse_button().set(button_name);
            let current: i32 = mouse.get_mouse_down_count().get();
            mouse.get_mouse_down_count().set(current + 1);
        }
    });
    let on_mouse_up: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        let current: i32 = mouse.get_mouse_up_count().get();
        mouse.get_mouse_up_count().set(current + 1);
    });
    let on_mouse_move: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        if let Some(mouse_event) = event.dyn_ref::<MouseEvent>() {
            let pos: String = format!("({}, {})", mouse_event.client_x(), mouse_event.client_y());
            mouse.get_mouse_pos().set(pos);
            let buttons_mask: String = format!("{}", mouse_event.buttons());
            mouse.get_mouse_buttons().set(buttons_mask);
        }
    });
    let on_mouse_enter: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        let current: i32 = mouse.get_mouse_enter_count().get();
        mouse.get_mouse_enter_count().set(current + 1);
    });
    let on_mouse_leave: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        let current: i32 = mouse.get_mouse_leave_count().get();
        mouse.get_mouse_leave_count().set(current + 1);
    });
    let on_context_menu: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        if event.dyn_ref::<MouseEvent>().is_some() {
            Console::log(EVENT_LOG_CONTEXT_MENU);
        }
    });
    let on_mouse_over: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        let current: i32 = mouse.get_mouse_over_count().get();
        mouse.get_mouse_over_count().set(current + 1);
    });
    let on_mouse_out: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        let current: i32 = mouse.get_mouse_out_count().get();
        mouse.get_mouse_out_count().set(current + 1);
    });
    let on_focus: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        focus
            .get_focus_status()
            .set(EVENT_FOCUS_STATE_FOCUSED.to_string());
        let current: i32 = focus.get_focus_in_count().get();
        focus.get_focus_in_count().set(current + 1);
        Console::log(EVENT_LOG_FOCUS);
    });
    let on_blur: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        focus
            .get_focus_status()
            .set(EVENT_FOCUS_STATE_NOT_FOCUSED.to_string());
        let current: i32 = focus.get_focus_out_count().get();
        focus.get_focus_out_count().set(current + 1);
        Console::log(EVENT_LOG_BLUR);
    });
    let on_focus_in: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        Console::log(EVENT_LOG_FOCUS_IN);
    });
    let on_focus_out: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        Console::log(EVENT_LOG_FOCUS_OUT);
    });
    let on_drag_start: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        drag.get_drag_status()
            .set(EVENT_DRAG_STATE_DRAGGING.to_string());
        drag.get_drag_enter_counter().set(1);
        Console::log(EVENT_LOG_DRAG_START);
    });
    let on_drag: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
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
    });
    let on_drag_end: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
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
        drag.get_drag_status()
            .set(EVENT_DRAG_STATE_ENDED.to_string());
        drag.get_drag_enter_counter().set(0);
        Console::log(EVENT_LOG_DRAG_END);
    });
    let on_drag_over: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        event.prevent_default();
    });
    let on_drag_enter: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        let counter: i32 = drag.get_drag_enter_counter().get();
        drag.get_drag_enter_counter().set(counter + 1);
        if counter == 0 {
            drag.get_drag_status()
                .set(EVENT_DRAG_STATE_DRAGGING.to_string());
            Console::log(EVENT_LOG_DRAG_ENTER);
        }
    });
    let on_drag_leave: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        let counter: i32 = drag.get_drag_enter_counter().get();
        drag.get_drag_enter_counter().set(counter - 1);
        if counter <= 1 {
            drag.get_drag_status()
                .set(EVENT_DRAG_STATE_OUTSIDE.to_string());
            Console::log(EVENT_LOG_DRAG_LEAVE);
        }
    });
    let on_drop: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
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
                drag.get_drag_types().set(EVENT_MODIFIER_NONE.to_string());
            } else {
                drag.get_drag_types().set(types_str);
            }
        }
        drag.get_drag_status()
            .set(EVENT_DRAG_STATE_DROPPED.to_string());
        Console::log(EVENT_LOG_DROP);
    });
    let file_drag_over: Signal<bool> = App::use_signal(|| false);
    let on_file_drag_over: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        event.prevent_default();
        file_drag_over.set(true);
    });
    let on_file_drag_enter: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        file_drag_over.set(true);
        drag.get_drag_status()
            .set(EVENT_FILE_DRAG_STATE_OVER.to_string());
        Console::log(EVENT_LOG_FILE_DRAG_ENTER);
    });
    let on_file_drag_leave: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        file_drag_over.set(false);
        drag.get_drag_status()
            .set(EVENT_DRAG_STATE_OUTSIDE.to_string());
        Console::log(EVENT_LOG_FILE_DRAG_LEAVE);
    });
    let on_file_drop: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
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
                drag.get_drag_types()
                    .set(EVENT_FILE_DROP_STATE_EMPTY.to_string());
            } else {
                drag.get_drag_types().set(file_names);
            }
        }
        drag.get_drag_status()
            .set(EVENT_DRAG_STATE_DROPPED.to_string());
        Console::log(EVENT_LOG_FILE_DROP);
    });
    let on_wheel: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
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
                0 => EVENT_WHEEL_DELTA_MODE_PIXEL.to_string(),
                1 => EVENT_WHEEL_DELTA_MODE_LINE.to_string(),
                2 => EVENT_WHEEL_DELTA_MODE_PAGE.to_string(),
                _ => EVENT_WHEEL_DELTA_MODE_UNKNOWN.to_string(),
            };
            Console::log(format!(
                "Wheel: dx={:.1}, dy={:.1}, mode={}",
                wheel_event.delta_x(),
                wheel_event.delta_y(),
                mode_name
            ));
        }
    });
    let on_copy: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        clipboard
            .get_clipboard_event_type()
            .set(EVENT_CLIPBOARD_COPY_LABEL.to_string());
        if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
            let data: Option<String> = clipboard_event
                .clipboard_data()
                .and_then(|cd: DataTransfer| cd.get_data(EVENT_CLIPBOARD_CONTENT_TYPE).ok());
            clipboard
                .get_clipboard_data()
                .set(data.unwrap_or_else(|| EVENT_CLIPBOARD_STATE_EMPTY.to_string()));
        }
        Console::log(EVENT_LOG_CLIPBOARD_COPY);
    });
    let on_cut: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        clipboard.get_clipboard_event_type().set("Cut".to_string());
        if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
            let data: Option<String> = clipboard_event
                .clipboard_data()
                .and_then(|cd: DataTransfer| cd.get_data(EVENT_CLIPBOARD_CONTENT_TYPE).ok());
            clipboard
                .get_clipboard_data()
                .set(data.unwrap_or_else(|| EVENT_CLIPBOARD_STATE_EMPTY.to_string()));
        }
        Console::log(EVENT_LOG_CLIPBOARD_CUT);
    });
    let on_paste: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        clipboard
            .get_clipboard_event_type()
            .set(EVENT_CLIPBOARD_PASTE_LABEL.to_string());
        if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
            let data: Option<String> = clipboard_event
                .clipboard_data()
                .and_then(|cd: DataTransfer| cd.get_data(EVENT_CLIPBOARD_CONTENT_TYPE).ok());
            clipboard
                .get_clipboard_data()
                .set(data.unwrap_or_else(|| EVENT_CLIPBOARD_STATE_EMPTY.to_string()));
        }
        Console::log(EVENT_LOG_CLIPBOARD_PASTE);
    });
    let on_touch_start: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
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
    });
    let on_touch_move: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
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
    });
    let on_touch_end: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
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
    });
    let on_touch_cancel: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
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
    });
    let on_form_submit: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        event.prevent_default();
        let current: i32 = form.get_submit_count().get();
        form.get_submit_count().set(current + 1);
        Console::log(format!("Form submitted #{}", current + 1));
    });
    let on_euv_input: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            form.get_euv_input_value().set(input.value());
        }
    });
    let on_form_change: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            form.get_form_change_value().set(input.value());
        }
    });
    let on_checkbox_change: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            form.get_form_checkbox().set(input.checked());
        }
    });
    let on_select_change: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>()
        {
            form.get_form_select_value().set(select.value());
        }
    });
    let on_audio_play: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        media
            .get_media_status()
            .set(EVENT_AUDIO_STATE_PLAYING.to_string());
        media
            .get_media_event_log()
            .set(EVENT_AUDIO_PLAY_LABEL.to_string());
        Console::log(EVENT_LOG_AUDIO_PLAY);
    });
    let on_audio_pause: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        media
            .get_media_status()
            .set(EVENT_AUDIO_STATE_PAUSED.to_string());
        media
            .get_media_event_log()
            .set(EVENT_AUDIO_PAUSE_LABEL.to_string());
        Console::log(EVENT_LOG_AUDIO_PAUSE);
    });
    let on_audio_ended: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        media
            .get_media_status()
            .set(EVENT_DRAG_STATE_ENDED.to_string());
        media
            .get_media_event_log()
            .set(EVENT_DRAG_STATE_ENDED.to_string());
        Console::log(EVENT_LOG_AUDIO_ENDED);
    });
    let on_audio_loaded_data: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        media
            .get_media_status()
            .set(EVENT_MEDIA_STATE_LOADED.to_string());
        media
            .get_media_event_log()
            .set(EVENT_VIDEO_EVENT_LOADED_DATA.to_string());
    });
    let on_audio_can_play: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        media
            .get_media_event_log()
            .set(EVENT_VIDEO_EVENT_CAN_PLAY.to_string());
    });
    let on_audio_volume_change = move |_: Event| {
        media
            .get_media_event_log()
            .set(EVENT_VIDEO_EVENT_VOLUME_CHANGE.to_string());
        Console::log(EVENT_LOG_VIDEO_VOLUME_CHANGE);
    };
    let on_audio_time_update: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        media
            .get_media_event_log()
            .set(EVENT_VIDEO_EVENT_TIME_UPDATE.to_string());
    });
    let on_video_play = move |_: Event| {
        video
            .get_video_status()
            .set(EVENT_AUDIO_STATE_PLAYING.to_string());
        video
            .get_video_event_log()
            .set(EVENT_AUDIO_PLAY_LABEL.to_string());
        Console::log(EVENT_LOG_VIDEO_PLAY);
    };
    let on_video_pause: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_status()
            .set(EVENT_AUDIO_STATE_PAUSED.to_string());
        video
            .get_video_event_log()
            .set(EVENT_AUDIO_PAUSE_LABEL.to_string());
        Console::log(EVENT_LOG_VIDEO_PAUSE);
    });
    let on_video_ended: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_status()
            .set(EVENT_DRAG_STATE_ENDED.to_string());
        video
            .get_video_event_log()
            .set(EVENT_DRAG_STATE_ENDED.to_string());
        Console::log(EVENT_LOG_VIDEO_ENDED);
    });
    let on_video_loaded_data: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_status()
            .set(EVENT_VIDEO_STATE_DATA_LOADED.to_string());
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_LOADED_DATA.to_string());
        Console::log(EVENT_LOG_VIDEO_LOADED_DATA);
    });
    let on_video_loaded_metadata: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_status()
            .set(EVENT_VIDEO_STATE_META_LOADED.to_string());
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_LOADED_METADATA.to_string());
        Console::log(EVENT_LOG_VIDEO_LOADED_METADATA);
    });
    let on_video_can_play: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_CAN_PLAY.to_string());
        Console::log(EVENT_LOG_VIDEO_CAN_PLAY);
    });
    let on_video_can_play_through: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_CAN_PLAY_THROUGH.to_string());
        Console::log(EVENT_LOG_VIDEO_CAN_PLAY_THROUGH);
    });
    let on_video_waiting: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_status()
            .set(EVENT_VIDEO_STATE_WAITING.to_string());
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_STATE_WAITING.to_string());
        Console::log(EVENT_LOG_VIDEO_WAITING);
    });
    let on_video_playing: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_status()
            .set(EVENT_AUDIO_STATE_PLAYING.to_string());
        video
            .get_video_event_log()
            .set(EVENT_AUDIO_STATE_PLAYING.to_string());
        Console::log(EVENT_LOG_VIDEO_PLAYING);
    });
    let on_video_time_update: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(video_el) = target.clone().dyn_into::<HtmlMediaElement>()
        {
            let current: String = format!("{:.2}", video_el.current_time());
            video.get_video_current_time().set(current);
        }
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_TIME_UPDATE.to_string());
    });
    let on_video_duration_change = move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(video_el) = target.clone().dyn_into::<HtmlMediaElement>()
        {
            let dur: String = format!("{:.2}", video_el.duration());
            video.get_video_duration().set(dur);
        }
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_DURATION_CHANGE.to_string());
        Console::log(EVENT_LOG_VIDEO_DURATION_CHANGE);
    };
    let on_video_progress: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_PROGRESS.to_string());
    });
    let on_video_seeking = move |_: Event| {
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_SEEKING.to_string());
        Console::log(EVENT_LOG_VIDEO_SEEKING);
    };
    let on_video_seeked: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_SEEKED.to_string());
        Console::log(EVENT_LOG_VIDEO_SEEKED);
    });
    let on_video_volume_change: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_VOLUME_CHANGE.to_string());
        Console::log(EVENT_LOG_VIDEO_VOLUME_CHANGE_2);
    });
    let on_video_rate_change: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(video_el) = target.clone().dyn_into::<HtmlMediaElement>()
        {
            let rate: String = format!("{}", video_el.playback_rate());
            video.get_video_playback_rate().set(rate);
        }
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_RATE_CHANGE.to_string());
        Console::log(EVENT_LOG_VIDEO_RATE_CHANGE);
    });
    let on_video_emptied: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_EMPTIED.to_string());
        Console::log(EVENT_LOG_VIDEO_EMPTIED);
    });
    let on_video_stalled: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_STALLED.to_string());
        Console::log(EVENT_LOG_VIDEO_STALLED);
    });
    let on_video_suspend: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_SUSPEND.to_string());
        Console::log(EVENT_LOG_VIDEO_SUSPEND);
    });
    let on_video_load_start: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_event_log()
            .set(EVENT_VIDEO_EVENT_LOAD_START.to_string());
        Console::log(EVENT_LOG_VIDEO_LOAD_START);
    });
    let on_video_error: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        video
            .get_video_status()
            .set(EVENT_MEDIA_STATE_ERROR.to_string());
        video
            .get_video_event_log()
            .set(EVENT_MEDIA_STATE_ERROR.to_string());
        Console::log(EVENT_LOG_VIDEO_ERROR);
    });
    let on_image_load: Box<dyn FnMut(Event)> = Box::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(img_el) = target.clone().dyn_into::<HtmlImageElement>()
        {
            let size: String = format!("{}x{}", img_el.natural_width(), img_el.natural_height());
            image.get_image_natural_size().set(size);
        }
        image
            .get_image_status()
            .set(EVENT_MEDIA_STATE_LOADED.to_string());
        image
            .get_image_event_log()
            .set(EVENT_IMAGE_STATE_LOADED.to_string());
        Console::log(EVENT_LOG_IMAGE_LOAD);
    });
    let on_image_error: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
        image
            .get_image_status()
            .set(EVENT_MEDIA_STATE_ERROR.to_string());
        image
            .get_image_event_log()
            .set(EVENT_MEDIA_STATE_ERROR.to_string());
        Console::log(EVENT_LOG_IMAGE_ERROR);
    });
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🎯"
                title: EVENT_HEADER_TITLE
                subtitle: EVENT_HEADER_SUBTITLE
            }
            euv_card {
                title: EVENT_KEYBOARD_CARD_TITLE
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
                            EVENT_KEYBOARD_LABEL_KEY_DOWN
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
                            EVENT_KEYBOARD_LABEL_KEY_CODE
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
                            EVENT_KEYBOARD_LABEL_KEY_UP
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
                            EVENT_KEYBOARD_LABEL_REPEAT
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
                            EVENT_KEYBOARD_LABEL_MODIFIERS
                        }
                        span {
                            class: c_event_info_value()
                            keyboard.get_modifier()
                        }
                    }
                }
            }
            euv_card {
                title: EVENT_MOUSE_CARD_TITLE
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
                        EVENT_MOUSE_CARD_HINT
                    }
                    p {
                        class: c_demo_text_muted()
                        EVENT_MOUSE_CARD_BODY
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            EVENT_MOUSE_LABEL_CLICKS
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
                            EVENT_MOUSE_LABEL_DOUBLE_CLICKS
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
                            EVENT_MOUSE_LABEL_DOWN
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
                            EVENT_MOUSE_LABEL_UP
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
                            EVENT_MOUSE_LABEL_CLIENT
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
                            EVENT_MOUSE_LABEL_SCREEN
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
                            EVENT_MOUSE_LABEL_BUTTON
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
                            EVENT_MOUSE_LABEL_BUTTONS
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
                            EVENT_MOUSE_LABEL_ENTER
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
                            EVENT_MOUSE_LABEL_LEAVE
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
                            EVENT_MOUSE_LABEL_OVER
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
                            EVENT_MOUSE_LABEL_OUT
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_out_count()
                        }
                    }
                }
            }
            euv_card {
                title: EVENT_MOUSE_OVER_OUT_CARD_TITLE
                div {
                    class: c_switcher()
                    div {
                        class: c_event_drag_zone()
                        class: c_switcher_col()
                        onmouseover: on_mouse_over
                        p {
                            class: c_demo_text()
                            EVENT_MOUSE_OVER_ZONE_LABEL
                        }
                        p {
                            class: c_demo_text_muted()
                            EVENT_MOUSE_OVER_ZONE_HINT
                        }
                    }
                    div {
                        class: c_event_drag_zone_active()
                        class: c_switcher_col()
                        onmouseout: on_mouse_out
                        p {
                            class: c_demo_text()
                            EVENT_MOUSE_OUT_ZONE_LABEL
                        }
                        p {
                            class: c_demo_text_muted()
                            EVENT_MOUSE_OUT_ZONE_HINT
                        }
                    }
                }
            }
            euv_card {
                title: EVENT_FOCUS_CARD_TITLE
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
                            EVENT_FOCUS_LABEL_STATUS
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
                            EVENT_FOCUS_LABEL_FOCUS_IN
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
                            EVENT_FOCUS_LABEL_FOCUS_OUT
                        }
                        span {
                            class: c_event_info_value()
                            focus.get_focus_out_count()
                        }
                    }
                }
            }
            euv_card {
                title: EVENT_DRAG_CARD_TITLE
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
                        EVENT_DRAG_SOURCE_LABEL
                    }
                    p {
                        class: c_demo_text_muted()
                        EVENT_DRAG_CARD_BODY
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            EVENT_FOCUS_LABEL_STATUS
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
                            EVENT_DRAG_LABEL_POSITION
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
                            EVENT_DRAG_LABEL_TYPES
                        }
                        span {
                            class: c_event_info_value()
                            drag.get_drag_types()
                        }
                    }
                }
            }
            euv_card {
                title: EVENT_FILE_DROP_CARD_TITLE
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
                        EVENT_FILE_DROP_ZONE_HINT
                    }
                    p {
                        class: c_event_drop_hint()
                        EVENT_FILE_DROP_CARD_BODY
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            EVENT_FOCUS_LABEL_STATUS
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
                            EVENT_FILE_DROP_LABEL_FILES
                        }
                        span {
                            class: c_event_info_value()
                            drag.get_drag_types()
                        }
                    }
                }
            }
            euv_card {
                title: EVENT_WHEEL_CARD_TITLE
                div {
                    class: c_event_wheel_zone()
                    onwheel: on_wheel
                    p {
                        class: c_demo_text()
                        EVENT_WHEEL_CARD_HINT
                    }
                    p {
                        class: c_demo_text_muted()
                        EVENT_WHEEL_CARD_BODY
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            EVENT_WHEEL_LABEL_DELTA
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
                            EVENT_WHEEL_LABEL_TOTAL_Y
                        }
                        span {
                            class: c_event_info_value()
                            wheel.get_wheel_total()
                        }
                    }
                }
            }
            euv_card {
                title: EVENT_CLIPBOARD_CARD_TITLE
                div {
                    class: c_event_clipboard_area()
                    input {
                        id: EVENT_CLIPBOARD_ID
                        name: EVENT_CLIPBOARD_NAME
                        type: EVENT_TEXT_TYPE
                        autocomplete: EVENT_AUTOCOMPLETE_OFF
                        placeholder: EVENT_CLIPBOARD_PLACEHOLDER
                        class: c_euv_input()
                        value: EVENT_CLIPBOARD_SAMPLE_TEXT
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
                            EVENT_CLIPBOARD_LABEL_EVENT
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
                            EVENT_CLIPBOARD_LABEL_DATA
                        }
                        span {
                            class: c_event_info_value()
                            clipboard.get_clipboard_data()
                        }
                    }
                }
            }
            euv_card {
                title: EVENT_TOUCH_CARD_TITLE
                div {
                    class: c_event_touch_zone()
                    ontouchstart: on_touch_start
                    ontouchmove: on_touch_move
                    ontouchend: on_touch_end
                    ontouchcancel: on_touch_cancel
                    p {
                        class: c_demo_text()
                        EVENT_TOUCH_CARD_HINT
                    }
                    p {
                        class: c_demo_text_muted()
                        EVENT_TOUCH_CARD_BODY
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            EVENT_TOUCH_LABEL_TOUCH
                        }
                        span {
                            class: c_event_info_value()
                            touch.get_touch_info()
                        }
                    }
                }
            }
            euv_card {
                title: EVENT_FORM_CARD_TITLE
                div {
                    class: c_event_form_area()
                    form {
                        onsubmit: on_form_submit
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                for: EVENT_FORM_INPUT_ID
                                class: c_form_label()
                                EVENT_FORM_INPUT_LEGEND
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
                                EVENT_FORM_CHECKBOX_LEGEND
                            }
                        }
                        div {
                            class: c_euv_input_wrapper()
                            label {
                                for: EVENT_FORM_SELECT_ID
                                class: c_form_label()
                                EVENT_FORM_SELECT_LEGEND
                            }
                            select {
                                id: EVENT_FORM_SELECT_ID
                                name: EVENT_FORM_SELECT_NAME
                                autocomplete: EVENT_AUTOCOMPLETE_OFF
                                class: c_select_input()
                                onchange: on_select_change
                                option {
                                    value: ""
                                    EVENT_FORM_SELECT_PLACEHOLDER
                                }
                                option {
                                    value: EVENT_FORM_OPTION_VALUE_ALPHA
                                    EVENT_FORM_OPTION_LABEL_ALPHA
                                }
                                option {
                                    value: EVENT_FORM_OPTION_VALUE_BETA
                                    EVENT_FORM_OPTION_LABEL_BETA
                                }
                                option {
                                    value: EVENT_FORM_OPTION_VALUE_GAMMA
                                    EVENT_FORM_OPTION_LABEL_GAMMA
                                }
                            }
                        }
                        div {
                            class: c_button_controls()
                            euv_button {
                                variant: EuvButtonVariant::Primary
                                label: EVENT_FORM_SUBMIT_LABEL
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
                            EVENT_FORM_LABEL_INPUT
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
                            EVENT_FORM_LABEL_CHANGE
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
                            EVENT_FORM_LABEL_CHECKED
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
                            EVENT_FORM_LABEL_SELECT
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
                            EVENT_FORM_LABEL_SUBMITS
                        }
                        span {
                            class: c_event_info_value()
                            form.get_submit_count()
                        }
                    }
                }
            }
            euv_card {
                title: EVENT_AUDIO_CARD_TITLE
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
                            EVENT_AUDIO_CARD_BODY
                        }
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            EVENT_FOCUS_LABEL_STATUS
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
                            EVENT_MEDIA_LABEL_LAST_EVENT
                        }
                        span {
                            class: c_event_info_value()
                            media.get_media_event_log()
                        }
                    }
                }
            }
            euv_card {
                title: EVENT_VIDEO_CARD_TITLE
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
                            EVENT_VIDEO_CARD_BODY
                        }
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            EVENT_FOCUS_LABEL_STATUS
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
                            EVENT_MEDIA_LABEL_LAST_EVENT
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
                            EVENT_VIDEO_LABEL_CURRENT_TIME
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
                            EVENT_VIDEO_LABEL_DURATION
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
                            EVENT_VIDEO_LABEL_PLAYBACK_RATE
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
                            EVENT_VIDEO_LABEL_BUFFERED
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_buffered()
                        }
                    }
                }
            }
            euv_card {
                title: EVENT_IMAGE_CARD_TITLE
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
                            EVENT_FOCUS_LABEL_STATUS
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
                            EVENT_MEDIA_LABEL_LAST_EVENT
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
                            EVENT_IMAGE_LABEL_NATURAL_SIZE
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
