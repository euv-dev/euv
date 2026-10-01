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
