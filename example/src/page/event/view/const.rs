/// The HTML id for the keyboard event input element.
pub(crate) const EVENT_KEYBOARD_ID: &str = "event-keyboard";

/// The HTML id for the focus event input element.
pub(crate) const EVENT_FOCUS_ID: &str = "event-focus";

/// The HTML id for the clipboard event input element.
pub(crate) const EVENT_CLIPBOARD_ID: &str = "event-clipboard";

/// The HTML id for the form input element in the event demo.
pub(crate) const EVENT_FORM_INPUT_ID: &str = "event-form-input";

/// The HTML id for the form checkbox element in the event demo.
pub(crate) const EVENT_FORM_CHECKBOX_ID: &str = "event-form-checkbox";

/// The HTML id for the form select element in the event demo.
pub(crate) const EVENT_FORM_SELECT_ID: &str = "event-form-select";

/// The HTML id for the video event demo element.
pub(crate) const EVENT_VIDEO_ID: &str = "event-video";

/// The HTML id for the image event demo element.
pub(crate) const EVENT_IMAGE_ID: &str = "event-image";

/// The HTML name attribute for the keyboard event input element.
pub(crate) const EVENT_KEYBOARD_NAME: &str = "keyboard";

/// The HTML name attribute for the focus event input element.
pub(crate) const EVENT_FOCUS_NAME: &str = "focus";

/// The HTML name attribute for the clipboard event input element.
pub(crate) const EVENT_CLIPBOARD_NAME: &str = "clipboard";

/// The HTML name attribute for the form input element.
pub(crate) const EVENT_FORM_INPUT_NAME: &str = "euv_input";

/// The HTML name attribute for the form checkbox element.
pub(crate) const EVENT_FORM_CHECKBOX_NAME: &str = "form_checkbox";

/// The HTML name attribute for the form select element.
pub(crate) const EVENT_FORM_SELECT_NAME: &str = "form_select";

/// The HTML input type for text.
pub(crate) const EVENT_TEXT_TYPE: &str = "text";

/// The HTML input type for checkbox.
pub(crate) const EVENT_CHECKBOX_TYPE: &str = "checkbox";

/// The HTML autocomplete attribute value for off.
pub(crate) const EVENT_AUTOCOMPLETE_OFF: &str = "off";

/// The HTML placeholder for the keyboard event input element.
pub(crate) const EVENT_KEYBOARD_PLACEHOLDER: &str = "Type here to capture key events...";

/// The HTML placeholder for the focus event input element.
pub(crate) const EVENT_FOCUS_PLACEHOLDER: &str = "Click to focus, click outside to blur...";

/// The HTML placeholder for the clipboard event input element.
pub(crate) const EVENT_CLIPBOARD_PLACEHOLDER: &str = "Try copy, cut, or paste here...";

/// The HTML placeholder for the form input element.
pub(crate) const EVENT_FORM_INPUT_PLACEHOLDER: &str = "Type to trigger input/change events...";

/// The HTML draggable attribute value for true.
pub(crate) const EVENT_DRAGGABLE_TRUE: &str = "true";

/// The HTML controls attribute value for true.
pub(crate) const EVENT_CONTROLS_TRUE: &str = "true";

/// The HTML preload attribute value for metadata.
pub(crate) const EVENT_PRELOAD_METADATA: &str = "metadata";

/// The HTML alt text for the image event demo.
pub(crate) const EVENT_IMAGE_ALT: &str = "Event Demo Image";

/// The video source URL for the video event demo.
pub(crate) const EVENT_VIDEO_SRC: &str =
    "https://ltpp.vip/github/pages/docs-pages/pages/video/ship.mp4";

/// The audio source URL for the audio demo.
pub(crate) const EVENT_AUDIO_SRC: &str =
    "https://ltpp.vip/github/pages/docs-pages/pages/audio/time_boils_the_rain.mp3";

/// The `ctrl` keyboard modifier prefix on the event demo page.
pub(crate) const EVENT_MODIFIER_CTRL: &str = "Ctrl+";

/// The `shift` keyboard modifier prefix on the event demo page.
pub(crate) const EVENT_MODIFIER_SHIFT: &str = "Shift+";

/// The `alt` keyboard modifier prefix on the event demo page.
pub(crate) const EVENT_MODIFIER_ALT: &str = "Alt+";

/// The `meta` keyboard modifier prefix on the event demo page.
pub(crate) const EVENT_MODIFIER_META: &str = "Meta+";

/// The `none` keyboard modifier prefix on the event demo page.
pub(crate) const EVENT_MODIFIER_NONE: &str = "None";

/// The name for mouse button `left` on the event demo page.
pub(crate) const EVENT_MOUSE_BUTTON_LEFT: &str = "Left";

/// The name for mouse button `middle` on the event demo page.
pub(crate) const EVENT_MOUSE_BUTTON_MIDDLE: &str = "Middle";

/// The name for mouse button `right` on the event demo page.
pub(crate) const EVENT_MOUSE_BUTTON_RIGHT: &str = "Right";

/// The console log line emitted for the context menu handler on the event demo page.
pub(crate) const EVENT_LOG_CONTEXT_MENU: &str = "ContextMenu: right-click detected";

/// The `focus state focused` text used on the event demo page.
pub(crate) const EVENT_FOCUS_STATE_FOCUSED: &str = "Focused";

/// The console log line emitted for the focus handler on the event demo page.
pub(crate) const EVENT_LOG_FOCUS: &str = "Focus: input gained focus";

/// The `focus state not focused` text used on the event demo page.
pub(crate) const EVENT_FOCUS_STATE_NOT_FOCUSED: &str = "Not focused";

/// The console log line emitted for the blur handler on the event demo page.
pub(crate) const EVENT_LOG_BLUR: &str = "Blur: input lost focus";

/// The console log line emitted for the focus in handler on the event demo page.
pub(crate) const EVENT_LOG_FOCUS_IN: &str = "FocusIn: focus entered";

/// The console log line emitted for the focus out handler on the event demo page.
pub(crate) const EVENT_LOG_FOCUS_OUT: &str = "FocusOut: focus left";

/// The `drag state dragging` text used on the event demo page.
pub(crate) const EVENT_DRAG_STATE_DRAGGING: &str = "Dragging";

/// The console log line emitted for the drag start handler on the event demo page.
pub(crate) const EVENT_LOG_DRAG_START: &str = "DragStart: drag started";

/// The `drag state ended` text used on the event demo page.
pub(crate) const EVENT_DRAG_STATE_ENDED: &str = "Ended";

/// The console log line emitted for the drag end handler on the event demo page.
pub(crate) const EVENT_LOG_DRAG_END: &str = "DragEnd: drag ended";

/// The console log line emitted for the drag enter handler on the event demo page.
pub(crate) const EVENT_LOG_DRAG_ENTER: &str = "DragEnter: entered drop zone";

/// The `drag state outside` text used on the event demo page.
pub(crate) const EVENT_DRAG_STATE_OUTSIDE: &str = "Outside";

/// The console log line emitted for the drag leave handler on the event demo page.
pub(crate) const EVENT_LOG_DRAG_LEAVE: &str = "DragLeave: left drop zone";

/// The `drag state dropped` text used on the event demo page.
pub(crate) const EVENT_DRAG_STATE_DROPPED: &str = "Dropped";

/// The console log line emitted for the drop handler on the event demo page.
pub(crate) const EVENT_LOG_DROP: &str = "Drop: item dropped";

/// The `file drag state over` text used on the event demo page.
pub(crate) const EVENT_FILE_DRAG_STATE_OVER: &str = "File over zone";

/// The console log line emitted for the file drag enter handler on the event demo page.
pub(crate) const EVENT_LOG_FILE_DRAG_ENTER: &str = "DragEnter: file entered drop zone";

/// The console log line emitted for the file drag leave handler on the event demo page.
pub(crate) const EVENT_LOG_FILE_DRAG_LEAVE: &str = "DragLeave: file left drop zone";

/// The `file drop state empty` text used on the event demo page.
pub(crate) const EVENT_FILE_DROP_STATE_EMPTY: &str = "No files";

/// The console log line emitted for the file drop handler on the event demo page.
pub(crate) const EVENT_LOG_FILE_DROP: &str = "Drop: files dropped";

/// The name for wheel delta mode `pixel` on the event demo page.
pub(crate) const EVENT_WHEEL_DELTA_MODE_PIXEL: &str = "pixel";

/// The name for wheel delta mode `line` on the event demo page.
pub(crate) const EVENT_WHEEL_DELTA_MODE_LINE: &str = "line";

/// The name for wheel delta mode `page` on the event demo page.
pub(crate) const EVENT_WHEEL_DELTA_MODE_PAGE: &str = "page";

/// The name for wheel delta mode `unknown` on the event demo page.
pub(crate) const EVENT_WHEEL_DELTA_MODE_UNKNOWN: &str = "unknown";

/// The label text for clipboard copy on the event demo page.
pub(crate) const EVENT_CLIPBOARD_COPY_LABEL: &str = "Copy";

/// The `clipboard content type` text used on the event demo page.
pub(crate) const EVENT_CLIPBOARD_CONTENT_TYPE: &str = "text";

/// The `clipboard state empty` text used on the event demo page.
pub(crate) const EVENT_CLIPBOARD_STATE_EMPTY: &str = "No data";

/// The console log line emitted for the clipboard copy handler on the event demo page.
pub(crate) const EVENT_LOG_CLIPBOARD_COPY: &str = "Copy: text copied";

/// The console log line emitted for the clipboard cut handler on the event demo page.
pub(crate) const EVENT_LOG_CLIPBOARD_CUT: &str = "Cut: text cut";

/// The label text for clipboard paste on the event demo page.
pub(crate) const EVENT_CLIPBOARD_PASTE_LABEL: &str = "Paste";

/// The console log line emitted for the clipboard paste handler on the event demo page.
pub(crate) const EVENT_LOG_CLIPBOARD_PASTE: &str = "Paste: text pasted";

/// The `audio state playing` text used on the event demo page.
pub(crate) const EVENT_AUDIO_STATE_PLAYING: &str = "Playing";

/// The label text for audio play on the event demo page.
pub(crate) const EVENT_AUDIO_PLAY_LABEL: &str = "Play";

/// The console log line emitted for the audio play handler on the event demo page.
pub(crate) const EVENT_LOG_AUDIO_PLAY: &str = "Play: audio started";

/// The `audio state paused` text used on the event demo page.
pub(crate) const EVENT_AUDIO_STATE_PAUSED: &str = "Paused";

/// The label text for audio pause on the event demo page.
pub(crate) const EVENT_AUDIO_PAUSE_LABEL: &str = "Pause";

/// The console log line emitted for the audio pause handler on the event demo page.
pub(crate) const EVENT_LOG_AUDIO_PAUSE: &str = "Pause: audio paused";

/// The console log line emitted for the audio ended handler on the event demo page.
pub(crate) const EVENT_LOG_AUDIO_ENDED: &str = "Ended: audio ended";

/// The `media state loaded` text used on the event demo page.
pub(crate) const EVENT_MEDIA_STATE_LOADED: &str = "Loaded";

/// The `video event loaded data` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_LOADED_DATA: &str = "LoadedData";

/// The `video event can play` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_CAN_PLAY: &str = "CanPlay";

/// The `video event volume change` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_VOLUME_CHANGE: &str = "VolumeChange";

/// The console log line emitted for the video volume change handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_VOLUME_CHANGE: &str = "VolumeChange: volume changed";

/// The `video event time update` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_TIME_UPDATE: &str = "TimeUpdate";

/// The console log line emitted for the video play handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_PLAY: &str = "Video Play: video started";

/// The console log line emitted for the video pause handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_PAUSE: &str = "Video Pause: video paused";

/// The console log line emitted for the video ended handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_ENDED: &str = "Video Ended: video ended";

/// The `video state data loaded` text used on the event demo page.
pub(crate) const EVENT_VIDEO_STATE_DATA_LOADED: &str = "Data Loaded";

/// The console log line emitted for the video loaded data handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_LOADED_DATA: &str = "Video LoadedData: data loaded";

/// The `video state meta loaded` text used on the event demo page.
pub(crate) const EVENT_VIDEO_STATE_META_LOADED: &str = "Meta Loaded";

/// The `video event loaded metadata` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_LOADED_METADATA: &str = "LoadedMetadata";

/// The console log line emitted for the video loaded metadata handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_LOADED_METADATA: &str = "Video LoadedMetadata: metadata loaded";

/// The console log line emitted for the video can play handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_CAN_PLAY: &str = "Video CanPlay: can play";

/// The `video event can play through` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_CAN_PLAY_THROUGH: &str = "CanPlayThrough";

/// The console log line emitted for the video can play through handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_CAN_PLAY_THROUGH: &str = "Video CanPlayThrough: can play through";

/// The `video state waiting` text used on the event demo page.
pub(crate) const EVENT_VIDEO_STATE_WAITING: &str = "Waiting";

/// The console log line emitted for the video waiting handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_WAITING: &str = "Video Waiting: buffering";

/// The console log line emitted for the video playing handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_PLAYING: &str = "Video Playing: playback resumed";

/// The `video event duration change` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_DURATION_CHANGE: &str = "DurationChange";

/// The console log line emitted for the video duration change handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_DURATION_CHANGE: &str = "Video DurationChange: duration changed";

/// The `video event progress` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_PROGRESS: &str = "Progress";

/// The `video event seeking` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_SEEKING: &str = "Seeking";

/// The console log line emitted for the video seeking handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_SEEKING: &str = "Video Seeking: seeking started";

/// The `video event seeked` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_SEEKED: &str = "Seeked";

/// The console log line emitted for the video seeked handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_SEEKED: &str = "Video Seeked: seek completed";

/// The console log line emitted for the video volume change 2 handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_VOLUME_CHANGE_2: &str = "Video VolumeChange: volume changed";

/// The `video event rate change` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_RATE_CHANGE: &str = "RateChange";

/// The console log line emitted for the video rate change handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_RATE_CHANGE: &str = "Video RateChange: playback rate changed";

/// The `video event emptied` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_EMPTIED: &str = "Emptied";

/// The console log line emitted for the video emptied handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_EMPTIED: &str = "Video Emptied: media emptied";

/// The `video event stalled` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_STALLED: &str = "Stalled";

/// The console log line emitted for the video stalled handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_STALLED: &str = "Video Stalled: data transfer stalled";

/// The `video event suspend` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_SUSPEND: &str = "Suspend";

/// The console log line emitted for the video suspend handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_SUSPEND: &str = "Video Suspend: data transfer suspended";

/// The `video event load start` text used on the event demo page.
pub(crate) const EVENT_VIDEO_EVENT_LOAD_START: &str = "LoadStart";

/// The console log line emitted for the video load start handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_LOAD_START: &str = "Video LoadStart: loading started";

/// The `media state error` text used on the event demo page.
pub(crate) const EVENT_MEDIA_STATE_ERROR: &str = "Error";

/// The console log line emitted for the video error handler on the event demo page.
pub(crate) const EVENT_LOG_VIDEO_ERROR: &str = "Video Error: error occurred";

/// The `image state loaded` text used on the event demo page.
pub(crate) const EVENT_IMAGE_STATE_LOADED: &str = "Load";

/// The console log line emitted for the image load handler on the event demo page.
pub(crate) const EVENT_LOG_IMAGE_LOAD: &str = "Image Load: image loaded successfully";

/// The console log line emitted for the image error handler on the event demo page.
pub(crate) const EVENT_LOG_IMAGE_ERROR: &str = "Image Error: failed to load image";

/// The title text for header on the event demo page.
pub(crate) const EVENT_HEADER_TITLE: &str = "Event Handling";

/// The subtitle text for header on the event demo page.
pub(crate) const EVENT_HEADER_SUBTITLE: &str = "Complete browser event demo covering keyboard, mouse, focus, drag-and-drop, wheel, clipboard, touch, form, media, video, and image events.";

/// The section heading for the keyboard card on the event demo page.
pub(crate) const EVENT_KEYBOARD_CARD_TITLE: &str = "Keyboard Events";

/// The `keyboard label key down` text used on the event demo page.
pub(crate) const EVENT_KEYBOARD_LABEL_KEY_DOWN: &str = "KeyDown:";

/// The `keyboard label key code` text used on the event demo page.
pub(crate) const EVENT_KEYBOARD_LABEL_KEY_CODE: &str = "KeyCode:";

/// The `keyboard label key up` text used on the event demo page.
pub(crate) const EVENT_KEYBOARD_LABEL_KEY_UP: &str = "KeyUp:";

/// The `keyboard label repeat` text used on the event demo page.
pub(crate) const EVENT_KEYBOARD_LABEL_REPEAT: &str = "Repeat:";

/// The `keyboard label modifiers` text used on the event demo page.
pub(crate) const EVENT_KEYBOARD_LABEL_MODIFIERS: &str = "Modifiers:";

/// The section heading for the mouse card on the event demo page.
pub(crate) const EVENT_MOUSE_CARD_TITLE: &str = "Mouse Events";

/// The usage hint for the mouse card on the event demo page.
pub(crate) const EVENT_MOUSE_CARD_HINT: &str =
    "Click, double-click, right-click, or move your mouse within this area to track mouse events.";

/// The explanatory paragraph for the mouse card on the event demo page.
pub(crate) const EVENT_MOUSE_CARD_BODY: &str = "Tracks click, dblclick, mousedown, mouseup, mousemove, mouseenter, mouseleave, and contextmenu events.";

/// The `mouse label clicks` text used on the event demo page.
pub(crate) const EVENT_MOUSE_LABEL_CLICKS: &str = "Clicks:";

/// The `mouse label double clicks` text used on the event demo page.
pub(crate) const EVENT_MOUSE_LABEL_DOUBLE_CLICKS: &str = "DblClicks:";

/// The `mouse label down` text used on the event demo page.
pub(crate) const EVENT_MOUSE_LABEL_DOWN: &str = "MouseDown:";

/// The `mouse label up` text used on the event demo page.
pub(crate) const EVENT_MOUSE_LABEL_UP: &str = "MouseUp:";

/// The `mouse label client` text used on the event demo page.
pub(crate) const EVENT_MOUSE_LABEL_CLIENT: &str = "Client:";

/// The `mouse label screen` text used on the event demo page.
pub(crate) const EVENT_MOUSE_LABEL_SCREEN: &str = "Screen:";

/// The `mouse label button` text used on the event demo page.
pub(crate) const EVENT_MOUSE_LABEL_BUTTON: &str = "Button:";

/// The `mouse label buttons` text used on the event demo page.
pub(crate) const EVENT_MOUSE_LABEL_BUTTONS: &str = "Buttons:";

/// The `mouse label enter` text used on the event demo page.
pub(crate) const EVENT_MOUSE_LABEL_ENTER: &str = "Enter:";

/// The `mouse label leave` text used on the event demo page.
pub(crate) const EVENT_MOUSE_LABEL_LEAVE: &str = "Leave:";

/// The `mouse label over` text used on the event demo page.
pub(crate) const EVENT_MOUSE_LABEL_OVER: &str = "Over:";

/// The `mouse label out` text used on the event demo page.
pub(crate) const EVENT_MOUSE_LABEL_OUT: &str = "Out:";

/// The section heading for the mouse over out card on the event demo page.
pub(crate) const EVENT_MOUSE_OVER_OUT_CARD_TITLE: &str = "Mouse Over/Out Events";

/// The label text for mouse over zone on the event demo page.
pub(crate) const EVENT_MOUSE_OVER_ZONE_LABEL: &str = "Mouse Over zone";

/// The usage hint for mouse over zone on the event demo page.
pub(crate) const EVENT_MOUSE_OVER_ZONE_HINT: &str = "Move mouse over this area";

/// The label text for mouse out zone on the event demo page.
pub(crate) const EVENT_MOUSE_OUT_ZONE_LABEL: &str = "Mouse Out zone";

/// The usage hint for mouse out zone on the event demo page.
pub(crate) const EVENT_MOUSE_OUT_ZONE_HINT: &str = "Move mouse out of this area";

/// The section heading for the focus card on the event demo page.
pub(crate) const EVENT_FOCUS_CARD_TITLE: &str = "Focus Events";

/// The status text shown for focus label on the event demo page.
pub(crate) const EVENT_FOCUS_LABEL_STATUS: &str = "Status:";

/// The `focus label focus in` text used on the event demo page.
pub(crate) const EVENT_FOCUS_LABEL_FOCUS_IN: &str = "FocusIn:";

/// The `focus label focus out` text used on the event demo page.
pub(crate) const EVENT_FOCUS_LABEL_FOCUS_OUT: &str = "FocusOut:";

/// The section heading for the drag card on the event demo page.
pub(crate) const EVENT_DRAG_CARD_TITLE: &str = "Drag Events";

/// The label text for drag source on the event demo page.
pub(crate) const EVENT_DRAG_SOURCE_LABEL: &str = "Drag Me";

/// The explanatory paragraph for the drag card on the event demo page.
pub(crate) const EVENT_DRAG_CARD_BODY: &str =
    "dragstart, drag, dragend, dragover, dragenter, dragleave, drop";

/// The `drag label position` text used on the event demo page.
pub(crate) const EVENT_DRAG_LABEL_POSITION: &str = "Position:";

/// The `drag label types` text used on the event demo page.
pub(crate) const EVENT_DRAG_LABEL_TYPES: &str = "Types:";

/// The section heading for the file drop card on the event demo page.
pub(crate) const EVENT_FILE_DROP_CARD_TITLE: &str = "File Drag & Drop";

/// The usage hint for file drop zone on the event demo page.
pub(crate) const EVENT_FILE_DROP_ZONE_HINT: &str = "Drag & drop files here";

/// The explanatory paragraph for the file drop card on the event demo page.
pub(crate) const EVENT_FILE_DROP_CARD_BODY: &str = "dragover, dragenter, dragleave, drop";

/// The `file drop label files` text used on the event demo page.
pub(crate) const EVENT_FILE_DROP_LABEL_FILES: &str = "Files:";

/// The section heading for the wheel card on the event demo page.
pub(crate) const EVENT_WHEEL_CARD_TITLE: &str = "Wheel Event";

/// The usage hint for the wheel card on the event demo page.
pub(crate) const EVENT_WHEEL_CARD_HINT: &str =
    "Scroll the mouse wheel within this area to track wheel deltas and scroll mode.";

/// The explanatory paragraph for the wheel card on the event demo page.
pub(crate) const EVENT_WHEEL_CARD_BODY: &str =
    "Tracks wheel delta (deltaX, deltaY) and delta mode (pixel, line, or page).";

/// The `wheel label delta` text used on the event demo page.
pub(crate) const EVENT_WHEEL_LABEL_DELTA: &str = "Delta:";

/// The `wheel label total y` text used on the event demo page.
pub(crate) const EVENT_WHEEL_LABEL_TOTAL_Y: &str = "Total Y:";

/// The section heading for the clipboard card on the event demo page.
pub(crate) const EVENT_CLIPBOARD_CARD_TITLE: &str = "Clipboard Events";

/// The `clipboard sample text` text used on the event demo page.
pub(crate) const EVENT_CLIPBOARD_SAMPLE_TEXT: &str = "Sample text for clipboard";

/// The `clipboard label event` text used on the event demo page.
pub(crate) const EVENT_CLIPBOARD_LABEL_EVENT: &str = "Event:";

/// The `clipboard label data` text used on the event demo page.
pub(crate) const EVENT_CLIPBOARD_LABEL_DATA: &str = "Data:";

/// The section heading for the touch card on the event demo page.
pub(crate) const EVENT_TOUCH_CARD_TITLE: &str = "Touch Events";

/// The usage hint for the touch card on the event demo page.
pub(crate) const EVENT_TOUCH_CARD_HINT: &str =
    "Touch this area on a mobile device or touchscreen to track touch events.";

/// The explanatory paragraph for the touch card on the event demo page.
pub(crate) const EVENT_TOUCH_CARD_BODY: &str =
    "Tracks touchstart, touchmove, touchend, and touchcancel events with touch point details.";

/// The `touch label touch` text used on the event demo page.
pub(crate) const EVENT_TOUCH_LABEL_TOUCH: &str = "Touch:";

/// The section heading for the form card on the event demo page.
pub(crate) const EVENT_FORM_CARD_TITLE: &str = "Form Events";

/// The fieldset legend for form input on the event demo page.
pub(crate) const EVENT_FORM_INPUT_LEGEND: &str = "Input (oninput & onchange)";

/// The fieldset legend for form checkbox on the event demo page.
pub(crate) const EVENT_FORM_CHECKBOX_LEGEND: &str = "Checkbox (onchange)";

/// The fieldset legend for form select on the event demo page.
pub(crate) const EVENT_FORM_SELECT_LEGEND: &str = "Select (onchange)";

/// The placeholder text for form select on the event demo page.
pub(crate) const EVENT_FORM_SELECT_PLACEHOLDER: &str = "-- Choose --";

/// The `value` attribute of the `alpha` form option on the event demo page.
pub(crate) const EVENT_FORM_OPTION_VALUE_ALPHA: &str = "alpha";

/// The visible text of the `alpha` form option on the event demo page.
pub(crate) const EVENT_FORM_OPTION_LABEL_ALPHA: &str = "Alpha";

/// The `value` attribute of the `beta` form option on the event demo page.
pub(crate) const EVENT_FORM_OPTION_VALUE_BETA: &str = "beta";

/// The visible text of the `beta` form option on the event demo page.
pub(crate) const EVENT_FORM_OPTION_LABEL_BETA: &str = "Beta";

/// The `value` attribute of the `gamma` form option on the event demo page.
pub(crate) const EVENT_FORM_OPTION_VALUE_GAMMA: &str = "gamma";

/// The visible text of the `gamma` form option on the event demo page.
pub(crate) const EVENT_FORM_OPTION_LABEL_GAMMA: &str = "Gamma";

/// The label text for form submit on the event demo page.
pub(crate) const EVENT_FORM_SUBMIT_LABEL: &str = "Submit";

/// The `form label input` text used on the event demo page.
pub(crate) const EVENT_FORM_LABEL_INPUT: &str = "Input:";

/// The `form label change` text used on the event demo page.
pub(crate) const EVENT_FORM_LABEL_CHANGE: &str = "Change:";

/// The `form label checked` text used on the event demo page.
pub(crate) const EVENT_FORM_LABEL_CHECKED: &str = "Checked:";

/// The `form label select` text used on the event demo page.
pub(crate) const EVENT_FORM_LABEL_SELECT: &str = "Select:";

/// The `form label submits` text used on the event demo page.
pub(crate) const EVENT_FORM_LABEL_SUBMITS: &str = "Submits:";

/// The section heading for the audio card on the event demo page.
pub(crate) const EVENT_AUDIO_CARD_TITLE: &str = "Audio Media Events";

/// The explanatory paragraph for the audio card on the event demo page.
pub(crate) const EVENT_AUDIO_CARD_BODY: &str =
    "Audio player with play, pause, ended, loadeddata, canplay, volumechange, timeupdate events";

/// The `media label last event` text used on the event demo page.
pub(crate) const EVENT_MEDIA_LABEL_LAST_EVENT: &str = "Last Event:";

/// The section heading for the video card on the event demo page.
pub(crate) const EVENT_VIDEO_CARD_TITLE: &str = "Video Events";

/// The explanatory paragraph for the video card on the event demo page.
pub(crate) const EVENT_VIDEO_CARD_BODY: &str = "Video player with play, pause, ended, loadeddata, loadedmetadata, canplay, canplaythrough, waiting, playing, timeupdate, durationchange, progress, seeking, seeked, volumechange, ratechange, emptied, stalled, suspend, loadstart, error events";

/// The `video label current time` text used on the event demo page.
pub(crate) const EVENT_VIDEO_LABEL_CURRENT_TIME: &str = "Current Time:";

/// The `video label duration` text used on the event demo page.
pub(crate) const EVENT_VIDEO_LABEL_DURATION: &str = "Duration:";

/// The `video label playback rate` text used on the event demo page.
pub(crate) const EVENT_VIDEO_LABEL_PLAYBACK_RATE: &str = "Playback Rate:";

/// The `video label buffered` text used on the event demo page.
pub(crate) const EVENT_VIDEO_LABEL_BUFFERED: &str = "Buffered:";

/// The section heading for the image card on the event demo page.
pub(crate) const EVENT_IMAGE_CARD_TITLE: &str = "Image Events";

/// The `image label natural size` text used on the event demo page.
pub(crate) const EVENT_IMAGE_LABEL_NATURAL_SIZE: &str = "Natural Size:";
