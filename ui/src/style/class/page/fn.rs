use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Page Banner (unified header with emoji icon)
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_page {
        position: "relative";
        text-align: "center";
        box-sizing: "border-box";
    }

    pub(crate) c_page_glow {
        position: "absolute";
        top: "-50%";
        left: "50%";
        transform: "translateX(-50%)";
        width: "400px";
        max-width: "100%";
        height: "400px";
        pointer-events: "none";
    }

    pub(crate) c_page_content {
        position: "relative";
        z-index: "1";
    }

    pub(crate) c_page_icon {
        font-size: "36px";
        padding-bottom: var!(space-md);
        @media ((max-width: 767px)) {
            font-size: "40px";
        }
    }

    pub(crate) c_page_title {
        font-size: var!(font-4xl);
        font-weight: "800";
        letter-spacing: "-0.03em";
        margin: "0px";
        color: var!(foreground);
        margin-bottom: var!(space-sm);
        @media ((max-width: 767px)) {
            font-size: var!(font-3xl);
        }
    }

    pub(crate) c_page_subtitle {
        font-size: var!(font-lg);
        color: var!(muted-foreground);
        margin: "0px auto";
        margin-bottom: var!(space-sm);
        max-width: "560px";
        opacity: "1";
        @media ((max-width: 767px)) {
            font-size: var!(font-base);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Camera Page
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_camera_video_container {
        margin: format!("{} 0", var!(space-lg));
    }

    pub c_camera_video_active {
        width: "100%";
        background: "#000";
        aspect-ratio: "16 / 9";
        object-fit: "cover";
        border: format!("2px solid {}", var!(accent));
        box-sizing: "border-box";
    }

    pub c_camera_video_hidden {
        width: "100%";
        aspect-ratio: "16 / 9";
        display: "none";
    }

    pub c_camera_video_placeholder {
        width: "100%";
        aspect-ratio: "16 / 9";
        border: format!("2px dashed {}", var!(border));
        display: "flex";
        align-items: "center";
        justify-content: "center";
        box-sizing: "border-box";
    }

    pub c_camera_placeholder_content {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        gap: var!(space-md);
    }

    pub c_camera_placeholder_icon {
        font-size: var!(font-4xl);
        opacity: "1";
    }

    pub c_camera_placeholder_text {
        font-size: var!(font-base);
        color: "inherit";
        opacity: "1";
    }

    pub c_camera_error_box {
        margin: format!("{} 0px", var!(gap-component));
        color: var!(foreground);
        font-size: var!(font-base);
        word-break: "break-all";
        overflow-wrap: "break-word";
        box-sizing: "border-box";
    }

    pub c_camera_scan_result_box {
        margin-top: var!(gap-component);
        box-sizing: "border-box";
    }

    pub c_camera_scan_result_label {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
        margin-bottom: var!(space-xs);
    }

    pub c_camera_scan_result_value {
        font-size: var!(font-base);
        color: "inherit";
        word-break: "break-all";
        overflow-wrap: "break-word";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Canvas Drawing Board
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_canvas_preview_container {
        margin: format!("{} 0", var!(space-lg));
        overflow: "hidden";
        border: format!("2px solid {}", var!(border));
        background: "#ffffff";
        aspect-ratio: "9 / 16";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        box-sizing: "border-box";
    }

    pub c_canvas_placeholder {
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        text-align: "center";
        padding: var!(space-lg);
    }

    pub c_canvas_preview_image {
        width: "100%";
        height: "100%";
        object-fit: "cover";
        display: "block";
    }

    pub c_canvas_container_fullscreen {
        width: "100%";
        height: "100%";
        background: var!(background);
        display: "flex";
        flex-direction: "column";
        position: "fixed";
        top: "0";
        left: "0";
        z-index: "10002";
        padding: format!("{} {} {} {}", var!(padding-shell-top), var!(space-lg), var!(padding-shell-bottom), var!(space-lg));
        box-sizing: "border-box";
    }

    pub c_canvas_drawing_fullscreen_wrapper {
        flex: "1";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        overflow: "hidden";
        padding: var!(space-xs);
    }

    pub c_canvas_drawing_fullscreen {
        border: format!("2px solid {}", var!(border));
        display: "block";
        cursor: "crosshair";
        touch-action: "none";
    }

    pub c_canvas_fullscreen_toolbar {
        display: "flex";
        flex-direction: "column";
        align-items: "stretch";
        width: "100%";
        padding: format!("{} {}", var!(space-xs), "0px");
        flex-shrink: "0";
        gap: var!(space-xs);
    }

    pub c_canvas_fullscreen_toolbar_row_top {
        display: "flex";
        align-items: "center";
        height: var!(mobile-header-height);
        width: "100%";
        gap: var!(space-sm);
    }

    pub c_canvas_fullscreen_toolbar_color_wrapper {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex: "1";
        min-width: "0";
    }

    pub c_canvas_color_input_fullscreen {
        width: "100%";
        height: "42px";
        border: format!("1px solid {}", var!(border));
        cursor: "pointer";
        padding: format!("{}", var!(space-xs));
    }

    pub c_canvas_fullscreen_toolbar_row_bottom {
        display: "flex";
        align-items: "center";
        width: "100%";
    }

    pub c_canvas_fullscreen_range_input {
        flex: "1 1 0%";
        min-width: "120px";
        cursor: "pointer";
        -webkit-appearance: "none";
        appearance: "none";
        border: "none";
        outline: "none";
        height: "24px";
        ::-webkit-slider-runnable-track {
            height: "6px";
            background: format!("linear-gradient(to right, {} 0%, {} var(--value,50%), {} var(--value,50%), {} 100%)", var!(foreground), var!(foreground), var!(border), var!(border));
            border-radius: "3px";
            border: "none";
        }
        ::-webkit-slider-thumb {
            width: "20px";
            height: "20px";
            background: var!(background);
            border: format!("2px solid {}", var!(accent));
            border-radius: "50%";
            cursor: "pointer";
            -webkit-appearance: "none";
            margin-top: "-7px";
        }
        ::-moz-range-track {
            height: "6px";
            background: format!("linear-gradient(to right, {} 0%, {} var(--value,50%), {} var(--value,50%), {} 100%)", var!(foreground), var!(foreground), var!(border), var!(border));
            border-radius: "3px";
            border: "none";
        }
        ::-moz-range-thumb {
            width: "20px";
            height: "20px";
            background: var!(background);
            border: format!("2px solid {}", var!(accent));
            border-radius: "50%";
            cursor: "pointer";
        }
    }

    pub c_canvas_fullscreen_toolbar_button {
        flex: "0 0 auto";
        overflow: "hidden";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Game Fullscreen (2D / 3D)
    // ═══════════════════════════════════════════════════════════════════════════

    // Fullscreen overlay container for the 2D and 3D game pages.
    //
    // Mirrors `c_canvas_container_fullscreen` so the safe-area caching
    // pass in `UseEuvLayout::apply_cached_insets` can pick this selector
    // up alongside the canvas one. Differs in that the game overlay holds
    // the *same* `<canvas>` element that the inline page mounts (no
    // canvas re-creation), so the running game loop, balls, cubes, FPS
    // counter and other reactive state survive the fullscreen toggle
    // without reset.
    pub c_game_container_fullscreen {
        width: "100%";
        height: "100%";
        background: var!(background);
        display: "flex";
        flex-direction: "column";
        position: "fixed";
        top: "0";
        left: "0";
        z-index: "10002";
        padding: format!(
        "{} {} {} {}",
        var!(padding-shell-top),
        var!(space-lg),
        var!(padding-shell-bottom),
        var!(space-lg)
        );
        box-sizing: "border-box";
    }

    // Toolbar wrapping the fullscreen exit button on game pages.
    //
    // Game pages only need an `Exit` button here (no color picker / line
    // width control like the canvas drawing board), so the layout is a
    // single horizontal row at the bottom of the overlay.
    pub c_game_fullscreen_toolbar {
        display: "flex";
        align-items: "center";
        justify-content: "flex-end";
        width: "100%";
        padding: format!("{} {} {} {}", var!(space-xs), var!(space-lg), var!(space-xs), var!(space-lg));
        flex-shrink: "0";
        gap: var!(space-sm);
        box-sizing: "border-box";
    }

    // Wrapper that fills the fullscreen game container with the canvas.
    //
    // The game's `<canvas>` backing buffer is 600x400 (3:2). The canvas
    // itself has `width: 100%; height: 100%` so it fills this wrapper
    // exactly, and `object-fit: contain` on the canvas class makes the
    // browser uniformly scale the backing buffer to fit while preserving
    // its 3:2 aspect ratio. Without `object-fit: contain`, the canvas
    // would be stretched to whatever shape the wrapper has, making balls
    // render as horizontally-ovoid ellipses (e.g. on 1280x800 viewports
    // → 1.6:1, slightly squashed vertically). The letterbox bars appear
    // inside the canvas element rather than around an outer wrapper, so
    // the canvas always fills the fullscreen container with the backing
    // buffer uniformly scaled to its native aspect ratio.
    pub c_game_fullscreen_canvas_letterbox {
        position: "relative";
        width: "100%";
        max-width: "100%";
        max-height: "100%";
        height: "100%";
        display: "flex";
        align-items: "center";
        justify-content: "center";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom Attrs Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_custom_attrs_demo {
        // Use the theme foreground token (not `inherit`) so the text inside
        // the dynamic demo block stays readable when the block itself
        // applies a user-supplied background (e.g. `background-color: #000000`)
        // that otherwise paints the inherited text the same colour.
        color: var!(foreground);
        box-sizing: "border-box";
    }

    pub c_demo_text {
        color: var!(foreground);
        margin-bottom: var!(gap-component);
    }

    pub c_demo_text_muted {
        color: var!(foreground);
        opacity: "1";
        font-size: var!(font-base);
        margin-bottom: "0px";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Conditional Rendering Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_toggle_content {
        margin-top: var!(gap-component);
        color: "inherit";
    }

    pub c_toggle_title {
        margin-top: "0px";
        color: var!(accent);
        font-size: var!(font-md);
    }

    pub c_role_button_row {
        display: "flex";
        flex-wrap: "nowrap";
        gap: var!(gap-element);
        margin-bottom: var!(gap-component);
    }

    pub c_role_guest_text {
        color: var!(foreground);
        font-size: var!(font-base);
    }

    pub c_role_user_text {
        color: var!(foreground);
        font-size: var!(font-base);
    }

    pub c_role_admin_text {
        color: var!(foreground);
        font-size: var!(font-base);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // 404 Not Found
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_render_count_text {
        font-size: var!(font-base);
        color: "inherit";
        margin-bottom: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Event Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_event_result {
        font-size: var!(font-base);
        color: "inherit";
        margin-top: var!(gap-element);
    }

    pub c_event_highlight {
        font-weight: "600";
        color: var!(accent);
    }

    pub c_event_info_grid {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: var!(gap-element);
        margin-top: var!(gap-component);
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
            gap: var!(gap-element);
        }
    }

    pub c_event_info_row {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        overflow: "hidden";
    }

    pub c_event_info_label {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(foreground);
        flex-shrink: "0";
    }

    pub c_event_info_value {
        c_text_ellipsis();
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        font-family: "ui-monospace, monospace";
        flex: "1";
    }

    pub c_event_mouse_area {
        min-height: "120px";
        padding: format!("{} 0px", var!(space-xl));
        border: format!("2px dashed {}", var!(border));
        cursor: "crosshair";
        text-align: "center";
        user-select: "none";
        color: "inherit";
    }

    pub c_event_drag_zone {
        min-height: "100px";
        padding: format!("{} 0px", var!(space-xl));
        border: format!("2px dashed {}", var!(border));
        text-align: "center";
        user-select: "none";
        color: var!(foreground);
    }

    pub c_event_drag_zone_active {
        min-height: "100px";
        padding: format!("{} 0px", var!(space-xl));
        border: format!("2px dashed {}", var!(border));
        text-align: "center";
        user-select: "none";
        color: var!(foreground);
    }

    pub c_event_drag_item {
        display: "inline-block";
        padding: format!("{} {}", var!(space-sm), var!(space-xl));
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-base);
        font-weight: "500";
        cursor: "grab";
        margin: var!(space-sm);
    }

    pub c_event_drop_zone {
        border: format!("2px dashed {}", var!(border));
        padding: format!("{} {}", var!(space-4xl), var!(space-xl));
        text-align: "center";
        cursor: "pointer";
    }

    pub c_event_drop_zone_active {
        border: format!("2px dashed {}", var!(accent));
        padding: format!("{} {}", var!(space-4xl), var!(space-xl));
        text-align: "center";
        cursor: "pointer";
        background: var!(accent-muted);
    }

    pub c_event_drop_icon {
        font-size: var!(font-5xl);
        display: "block";
        padding-bottom: var!(space-md);
    }

    pub c_event_drop_text {
        font-size: var!(font-lg);
        font-weight: "500";
        color: "inherit";
        padding-bottom: var!(space-md);
    }

    pub c_event_drop_hint {
        font-size: var!(font-base);
        color: "inherit";
        opacity: "1";
        margin: "0px";
    }

    pub c_event_wheel_zone {
        min-height: "120px";
        padding: format!("{} 0px", var!(space-xl));
        border: format!("2px dashed {}", var!(border));
        text-align: "center";
        overflow: "auto";
        color: var!(foreground);
    }

    pub c_event_clipboard_area {
        color: var!(foreground);
    }

    pub c_event_touch_zone {
        min-height: "120px";
        padding: format!("{} 0px", var!(space-xl));
        border: format!("2px dashed {}", var!(accent));
        text-align: "center";
        touch-action: "none";
        user-select: "none";
        color: var!(accent);
    }

    pub c_event_form_area {
        padding: format!("{} 0px", var!(space-lg));
        color: var!(foreground);
    }

    pub c_event_media_area {
        padding: format!("{} 0px", var!(space-lg));
        color: var!(foreground);
        max-width: "100%";
        overflow: "hidden";
    }

    pub c_event_audio {
        width: "100%";
        max-width: "100%";
    }

    pub c_event_video_area {
        color: var!(foreground);
        width: "100%";
        overflow: "hidden";
    }

    pub c_event_video {
        width: "100%";
        max-width: "100%";
    }

    pub c_event_image_area {
        padding: format!("{} 0px", var!(space-lg));
        color: "inherit";
        width: "100%";
        overflow: "hidden";
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        gap: var!(space-md);
    }

    pub c_event_image {
        width: "200px";
        max-width: "100%";
        object-fit: "contain";
        display: "block";
    }

    pub c_event_url_text {
        text-align: "center";
        font-size: var!(font-sm);
        color: "inherit";
        opacity: "1";
        font-family: "ui-monospace, monospace";
        word-break: "break-all";
        width: "100%";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Timer Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_timer_display {
        text-align: "center";
        padding: var!(space-xl);
        margin: format!("{} 0px", var!(gap-component));
        color: "inherit";
    }

    pub c_timer_value {
        font-size: var!(font-base);
        font-weight: "700";
        color: var!(accent);
        letter-spacing: "0.04em";
        font-family: "ui-monospace, monospace";
    }

    pub c_timer_controls {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(gap-element);
    }

    pub c_timer_done {
        text-align: "center";
        margin-top: var!(gap-component);
        font-size: var!(font-lg);
        font-weight: "600";
        color: var!(foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Browser API Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_browser_api_row {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: var!(gap-component);
        margin-bottom: var!(gap-component);
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
        }
    }

    pub c_browser_api_actions {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(gap-element);
        margin: format!("{} 0px", var!(gap-component));
    }

    pub c_browser_result_box {
        margin-top: var!(gap-component);
        font-size: var!(font-base);
        word-break: "break-all";
        color: "inherit";
    }

    pub c_browser_result_label {
        font-weight: "600";
        color: var!(accent);
    }

    pub c_browser_result_value {
        color: "inherit";
    }

    pub c_browser_info_grid {
        display: "grid";
        gap: var!(gap-component);
        margin-top: var!(gap-component);
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
        }
    }

    pub c_browser_info_item {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
        color: "inherit";
    }

    pub c_browser_info_label {
        font-size: var!(font-sm);
        font-weight: "600";
        color: "inherit";
        opacity: "1";
        text-transform: "uppercase";
        letter-spacing: "0.05em";
    }

    pub c_browser_info_value {
        font-size: var!(font-base);
        color: "inherit";
        word-break: "break-all";
        font-family: "ui-monospace, monospace";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Keep-Alive Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_keep_alive_tab_bar {
        display: "flex";
        border-bottom: format!("1px dashed {}", var!(border));
        margin-bottom: var!(gap-component);
        gap: var!(gap-element);
    }

    pub c_keep_alive_tab_panel {
        padding: format!("{} 0px", var!(gap-element));
    }

    pub c_keep_alive_panel_title {
        margin-top: "0px";
        color: var!(accent);
        font-size: var!(font-md);
        margin-bottom: var!(gap-element);
    }

    pub c_keep_alive_demo_text {
        color: "inherit";
        font-size: var!(font-base);
        margin-bottom: var!(gap-component);
    }

    pub c_keep_alive_counter_display {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        margin: format!("{} 0px", var!(space-xl));
    }

    pub c_keep_alive_counter_value {
        font-size: var!(font-base);
        font-weight: "700";
        font-variant-numeric: "tabular-nums";
        color: var!(accent);
        text-align: "center";
    }

    pub c_keep_alive_counter_controls {
        display: "flex";
        flex-wrap: "wrap";
        gap: format!("{}", var!(space-md));
    }

    pub c_keep_alive_form_group {
        margin-bottom: var!(gap-component);
    }

    pub c_keep_alive_form_preview {
        margin-top: var!(gap-component);
        background: var!(accent-muted);
    }

    pub c_keep_alive_preview_label {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        margin: format!("0px 0px {} 0px", var!(space-xs));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Binding Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_binding_child_box {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-component);
        margin-top: var!(gap-component);
    }

    pub c_binding_child_label {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        text-transform: "uppercase";
        letter-spacing: "0.05em";
    }

    pub c_binding_parent_box {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-component);
        margin-top: var!(gap-component);
    }

    pub c_binding_section_title {
        margin-top: var!(gap-component);
        margin-bottom: var!(gap-element);
        color: var!(accent);
        font-size: var!(font-md);
        font-weight: "600";
    }

    pub c_binding_temp_converter {
        display: "flex";
        align-items: "flex-end";
        gap: var!(gap-component);
        flex-wrap: "wrap";
        margin-top: var!(gap-component);
    }

    pub c_binding_temp_field {
        flex: "1";
        min-width: "120px";
    }

    pub c_binding_temp_arrow {
        font-size: "20px";
        font-weight: "700";
        color: var!(accent);
        padding-bottom: "10px";
    }

    pub c_binding_color_mixer {
        margin-top: var!(gap-component);
    }

    pub c_binding_color_preview {
        width: "100%";
        height: "80px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        margin-bottom: var!(gap-component);
    }

    pub c_binding_color_hex {
        font-family: "ui-monospace, monospace";
        font-size: var!(font-xl);
        font-weight: "700";
        color: var!(text-on-accent);
        letter-spacing: "0.02em";
    }

    pub c_binding_slider_row {
        display: "flex";
        align-items: "center";
        gap: format!("{}", var!(space-md));
        margin-bottom: var!(space-sm);
    }

    pub c_binding_slider_label {
        font-size: var!(font-base);
        font-weight: "700";
        min-width: var!(font-sm);
    }

    pub c_binding_slider {
        flex: "1";
        height: "24px";
        padding: "0px";
        cursor: "pointer";
        -webkit-appearance: "none";
        appearance: "none";
        border: "none";
        outline: "none";
        ::-webkit-slider-runnable-track {
            height: "6px";
            background: format!("linear-gradient(to right, {} 0%, {} var(--value,50%), {} var(--value,50%), {} 100%)", var!(foreground), var!(foreground), var!(border), var!(border));
            border-radius: "3px";
            border: "none";
        }
        ::-webkit-slider-thumb {
            width: "20px";
            height: "20px";
            background: var!(background);
            border: format!("2px solid {}", var!(accent));
            border-radius: "50%";
            cursor: "pointer";
            -webkit-appearance: "none";
            margin-top: "-7px";
        }
        : active::-webkit-slider-thumb {
            transform: "scale(0.92)";
        }
        ::-moz-range-track {
            height: "6px";
            background: format!("linear-gradient(to right, {} 0%, {} var(--value,50%), {} var(--value,50%), {} 100%)", var!(foreground), var!(foreground), var!(border), var!(border));
            border-radius: "3px";
            border: "none";
        }
        ::-moz-range-thumb {
            width: "20px";
            height: "20px";
            background: var!(background);
            border: format!("2px solid {}", var!(accent));
            border-radius: "50%";
            cursor: "pointer";
        }
        : active::-moz-range-thumb {
            transform: "scale(0.92)";
        }
    }

    pub c_binding_slider_value {
        font-size: var!(font-base);
        font-weight: "500";
        color: "inherit";
        min-width: "32px";
        text-align: "right";
        font-family: "ui-monospace, monospace";
    }

    pub c_binding_typed_prop_value {
        font-family: "ui-monospace, monospace";
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        padding: format!("1px {}", var!(space-sm));
    }

    pub c_binding_typed_prop_group {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
    }

    pub c_binding_typed_warning {
        font-size: var!(font-base);
        font-weight: "500";
        color: var!(foreground);
        margin-bottom: "0px";
    }

    pub c_binding_form_label {
        display: "block";
        color: "inherit";
        font-weight: "500";
        font-size: var!(font-base);
    }

    pub c_binding_demo_text {
        color: "inherit";
        margin: "0px"
    }

    pub c_binding_compact_button {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        height: "42px";
        padding: format!("0px {}", var!(space-md));
        background: var!(accent);
        color: var!(text-on-accent);
        border: format!("1px solid {}", var!(accent));
        cursor: "pointer";
        font-size: var!(font-sm);
        font-weight: "600";
        outline: "none";
        white-space: "nowrap";
        user-select: "none";
        -webkit-user-select: "none";
        box-sizing: "border-box";
        :hover {
            background: var!(accent);
        }
        :focus-visible {
            outline: "none";
        }
        :active {
            background: var!(accent);
            color: var!(text-on-accent);
            border-color: var!(accent);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom Attrs - Dynamic Style
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_attrs_dynamic_demo(prop_key: &str, prop_value: &str) {
        {
            prop_key
        }
        : prop_value;
        // No layout defaults: this class lives on a `<p>` whose own margin /
        // spacing is controlled by its parent's typography rules, and pinning
        // any colour here would shadow the user-supplied dynamic property.
        // Users are expected to set `color` / `background` / `font-*` /
        // etc. via the two CSS Property Key / Value inputs above; whatever
        // they type becomes the literal CSS rule applied to the paragraph.
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Dynamic Component Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_dynamic_component_tab_bar {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(gap-element);
        margin-bottom: var!(gap-component);
    }

    pub c_dynamic_component_panel {
        display: "block";
        min-height: var!(min-height-sm);
        margin-top: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Sticky & CSS Effects Demo
    // ═══════════════════════════════════════════════════════════════════════════

    // ═══════════════════════════════════════════════════════════════════════════
    // Home Page — Section
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_home {
        position: "relative";
        text-align: "center";
        margin-bottom: var!(space-xl);
        flex: "1";
        display: "flex";
        flex-direction: "column";
        justify-content: "center";
        @media ((max-width: 767px)) {
            margin-bottom: var!(space-lg);
            flex: "0 0 auto";
            justify-content: "flex-start";
        }
    }

    pub c_home_content {
        position: "relative";
        z-index: "1";
    }

    pub c_home_badge_row {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-sm);
        margin-bottom: var!(space-md);
    }

    pub c_home_badge {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        color: var!(accent);
        font-size: var!(font-xs);
        font-weight: "600";
        letter-spacing: "0.05em";
        border: format!("1px solid {}", var!(accent));
    }

    pub c_home_title {
        font-size: var!(font-5xl);
        font-weight: "800";
        letter-spacing: "-0.03em";
        margin: "0px";
        color: var!(foreground);
        margin-bottom: var!(space-xs);
        @media ((max-width: 767px)) {
            font-size: var!(font-4xl);
        }
    }

    pub c_home_subtitle {
        font-size: var!(font-lg);
        color: var!(foreground);
        margin: "0px auto";
        max-width: "520px";
        margin-bottom: var!(space-lg);
        @media ((max-width: 767px)) {
            font-size: var!(font-base);
        }
    }

    pub c_home_actions {
        display: "flex";
        gap: var!(space-md);
        justify-content: "center";
        flex-wrap: "wrap";
        // Below the breakpoint the actions become equal-width rows: the two
        // labels differ in length, so a content-sized wrap leaves the shorter
        // button visibly narrower and the row reads as misaligned.
        @media ((max-width: 767px)) {
            flex-wrap: "nowrap";
            align-items: "stretch";
        }
    }

    pub c_home_btn_primary {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-2xl));
        background: var!(accent);
        color: var!(text-on-accent);
        font-size: var!(font-base);
        font-weight: "600";
        text-decoration: "none";
        border: "1.5px solid transparent";
        cursor: "pointer";
        letter-spacing: "0.01em";
        vertical-align: "middle";
        min-height: var!(min-height-sm);
        @media ((max-width: 767px)) {
            flex: "1 1 0px";
            min-width: "0px";
        }
        :focus-visible {
            outline: "none";
        }
        :active {
            background: var!(accent);
            color: var!(text-on-accent);
            border-color: "1.5px solid transparent";
        }
    }

    pub c_home_btn_secondary {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-2xl));
        color: var!(accent);
        font-size: var!(font-base);
        font-weight: "600";
        text-decoration: "none";
        border: format!("1.5px solid {}", var!(accent));
        cursor: "pointer";
        letter-spacing: "0.01em";
        vertical-align: "middle";
        min-height: var!(min-height-sm);
        @media ((max-width: 767px)) {
            flex: "1 1 0px";
            min-width: "0px";
        }
        :focus-visible {
            outline: "none";
        }
        :active {
            background: "transparent";
            color: var!(accent);
            border-color: var!(accent);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Home Page — Stats Row
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_home_stats {
        display: "grid";
        grid-template-columns: "repeat(4, 1fr)";
        gap: var!(space-md);
        margin-bottom: var!(space-xl);
        @media ((max-width: 767px)) {
            grid-template-columns: "repeat(2, 1fr)";
            gap: var!(space-sm);
        }
    }

    pub c_home_stat_card {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-xs);
    }

    pub c_home_stat_icon {
        font-size: var!(font-2xl);
    }

    pub c_home_stat_value {
        font-size: var!(font-xl);
        font-weight: "800";
        color: var!(foreground);
        letter-spacing: "-0.01em";
    }

    pub c_home_stat_label {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        font-weight: "500";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Home Page — Section & Feature Cards
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_home_section_title {
        font-size: var!(font-2xl);
        font-weight: "700";
        color: var!(foreground);
        margin: "0px";
        margin: format!("{} 0px", var!(gap-component));
        letter-spacing: "-0.02em";
    }

    pub c_home_section_desc {
        font-size: var!(font-base);
        color: var!(foreground);
        margin: "0px";
        margin-bottom: var!(space-2xl);
    }

    pub c_home_feature_grid {
        display: "grid";
        grid-template-columns: "repeat(2, 1fr)";
        gap: var!(space-md);
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
        }
    }

    pub c_feature_card {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-sm);
        padding: "0px";
        overflow: "hidden";
    }

    pub c_feature_header {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
    }

    pub c_feature_icon {
        font-size: var!(font-2xl);
        display: "flex";
        align-items: "center";
        justify-content: "center";
        line-height: "1";
        flex-shrink: "0";
    }

    pub c_feature_name {
        font-size: var!(font-lg);
        font-weight: "600";
        color: var!(foreground);
        margin: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub c_feature_desc {
        font-size: var!(font-sm);
        color: var!(foreground);
        margin: "0px";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Home Page
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_text_ellipsis {
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub c_info_row {
        display: "flex";
        align-items: "center";
        gap: var!(space-md);
        padding: format!("{} 0", var!(space-sm));
        overflow: "hidden";
    }

    pub c_info_label {
        font-size: var!(font-sm);
        font-weight: "500";
        color: var!(muted-foreground);
        min-width: "72px";
        flex-shrink: "0";
        letter-spacing: "0.02em";
    }

    pub c_info_value {
        c_text_ellipsis();
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(foreground);
        font-family: "ui-monospace, monospace";
        flex: "1";
    }

    pub c_info_link {
        c_text_ellipsis();
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        font-family: "ui-monospace, monospace";
        flex: "1";
        text-decoration: "none";
        transition: format!("opacity {} {}", var!(duration-fast), var!(ease-out));
        :hover {
            opacity: "1";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Async / Loading States
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_loading_container {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-component);
        margin: format!("{} 0px", var!(gap-component));
        box-sizing: "border-box";
    }

    pub c_spinner {
        width: "28px";
        height: "28px";
        border: format!("3px solid {}", var!(accent-muted));
        border-top: format!("3px solid {}", var!(accent));
        border-radius: "50%";
        flex-shrink: "0";
        animation: "euv-spin 0.8s linear infinite";
    }

    pub(crate) c_loading_text_col {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
    }

    pub(crate) c_loading_title {
        color: var!(foreground);
        font-size: var!(font-md);
        font-weight: "500";
    }

    pub(crate) c_loading_subtitle {
        color: var!(muted-foreground);
        font-size: var!(font-base);
    }

    pub c_loading_overlay(background: &str) {
        position: "absolute";
        top: "0";
        left: "0";
        width: "100%";
        height: "100%";
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-component);
        background: background;
        z-index: "1";
        box-sizing: "border-box";
    }

    pub c_error_container {
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
        margin-top: var!(gap-component);
        box-sizing: "border-box";
    }

    pub c_error_icon {
        width: "20px";
        height: "20px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex-shrink: "0";
        color: var!(foreground);
        font-size: var!(font-sm);
        font-weight: "700";
    }

    pub c_error_text {
        color: var!(foreground);
        font-size: var!(font-base);
    }

    pub c_data_box {
        margin: format!("{} 0px", var!(gap-component));
        box-sizing: "border-box";
    }

    pub c_data_pre {
        color: var!(foreground);
        font-size: var!(font-base);
        margin: "0px";
        white-space: "pre-wrap";
        word-break: "break-all";
        font-family: "ui-monospace, monospace";
    }

    pub c_fetch_hint {
        color: "inherit";
        opacity: "1";
        font-size: var!(font-base);
        margin-bottom: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Log / Console Display
    // ═══════════════════════════════════════════════════════════════════════════

    pub c_log_container {
        font-family: "ui-monospace, monospace";
        font-size: var!(font-base);
        margin-top: "0px";
        color: var!(foreground);
    }

    pub c_log_item {
        padding: format!("{} 0px", var!(space-sm));
        font-size: var!(font-base);
        color: var!(foreground);
    }
}
