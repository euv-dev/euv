use super::*;

class! {
    pub c_game_stats_bar {
        display: "flex";
        gap: "16px";
        margin-bottom: "12px";
        font-size: "14px";
        font-weight: "600";
    }

    pub c_game_stats_label {
        color: "inherit";
    }

    pub c_game_stats_fps_value {
        color: "inherit";
    }

    pub c_game_stats_count_value {
        color: "inherit";
    }

    pub c_game_stats_total_value {
        color: "inherit";
    }

    pub c_game_description {
        line-height: "1.5";
        color: "inherit";
        margin-bottom: var!(gap-component);
    }

    pub c_game_canvas_wrapper {
        position: "relative";
        width: "100%";
        // Inline-mode canvas wrapper: gives the canvas a stable 3:2 CSS
        // box (matching the 600x400 backing buffer) so the inline
        // dimensions are 820x547 on a 1280-wide page. Without
        // `aspect-ratio`, the inner `c_game_fullscreen_canvas_wrapper`
        // collapses to the canvas's natural inline-block height (a few
        // pixels) on inline layout, producing a 820x210 (or similar)
        // strip instead of the intended 3:2 frame.
        // The two caps stop the ratio from pushing the frame past the
        // visible area on either axis. They are percentages of the wrapper
        // rather than viewport units: the wrapper is a flex item, so a
        // viewport-unit cap would ignore the surrounding page chrome
        // (header/footer height) and let the frame spill under the fold.
        aspect-ratio: "3 / 2";
        max-width: "100%";
        max-height: "100%";
    }

    pub c_game_fullscreen_canvas_wrapper {
        // Fullscreen-mode wrapper: fills the fixed fullscreen
        // container (1248x750 on a 1280x800 viewport) so the canvas
        // packed inside takes the entire viewport minus toolbar
        // padding. The canvas's `width:100%; height:100%` makes its
        // CSS box match this wrapper exactly.
        flex: "1";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        overflow: "hidden";
        min-height: "0";
        width: "100%";
        height: "100%";
    }

    pub c_game_3d_canvas {
        width: "100%";
        height: "100%";
        cursor: "grab";
        display: "block";
        background: var!(accent);
        touch-action: "none";
        object-fit: "contain";
    }

    pub c_game_3d_canvas_fullscreen {
        // Fullscreen canvas: CSS box matches the fullscreen wrapper
        // (1248x750 on 1280x800 viewport, 100% of the column). The
        // backing buffer is resized to match via the fullscreen enter
        // hook (see game_2d hook::fn::enter_game_2d_fullscreen and
        // game_3d hook::fn::enter_game_3d_fullscreen) so the ball /
        // cube physics bounds, click mapping, and clear rect all
        // operate on the full canvas dimensions instead of the
        // 600x400 default. No CSS rotation, no aspect-ratio squeeze,
        // no letterbox - the canvas fills the available space and the
        // game elements are redrawn at the new size.
        width: "100%";
        height: "100%";
        cursor: "grab";
        display: "block";
        background: var!(accent);
        touch-action: "none";
    }

    pub c_raytrace_canvas_fullscreen {
        // RayTrace and Lighting are software-rendered 2D canvases with
        // a fixed 4:3 backing buffer (320x240). Their backing buffer
        // is NOT resized on fullscreen enter (re-tracing 1248x750
        // pixels per frame is not viable in WASM), so the CSS box
        // must NOT stretch the canvas element to the wrapper - that
        // would deform every sphere into a 1.45:1 ellipse on a 1280x800
        // viewport. `object-fit: contain` makes the browser uniformly
        // scale the 4:3 backing buffer to the largest 4:3 fit inside
        // the wrapper, painting letterbox bars where the wrapper
        // extends beyond 4:3.
        width: "100%";
        height: "100%";
        cursor: "grab";
        display: "block";
        background: "#000000";
        touch-action: "none";
        object-fit: "contain";
    }

    pub c_game_2d_canvas {
        width: "100%";
        height: "100%";
        cursor: "pointer";
        display: "block";
        background: var!(accent);
        touch-action: "none";
        object-fit: "contain";
    }

    pub c_game_2d_canvas_fullscreen {
        // Fullscreen canvas - see c_game_3d_canvas_fullscreen for
        // the resize-on-enter rationale.
        width: "100%";
        height: "100%";
        cursor: "pointer";
        display: "block";
        background: var!(accent);
        touch-action: "none";
    }

    pub c_canvas_pixelated {
        image-rendering: "pixelated";
        image-rendering: "crisp-edges";
    }

    pub c_game_loading_overlay {
        position: "absolute";
        top: "0";
        left: "0";
        width: "100%";
        height: "100%";
        pointer-events: "none";
    }

    pub c_keep_alive_tab_visible {
        display: "block";
    }

    pub c_keep_alive_tab_hidden {
        display: "none";
    }

    pub c_binding_slider_label_accent {
        color: var!(accent);
    }

    pub c_binding_color_preview_bg(background: &str) {
        background: {
            background
        };
    }

    pub c_slider_value(value_percent: &str) {
        {
            "--value"
        }
        : {
            value_percent
        };
    }

    pub c_anim_scale_shrink {
        transform: "scale(0.85)";
    }

    pub c_anim_scale_normal {
        transform: "scale(1)";
    }

    // The gesture demo's touch surface: tall enough to drag in, with the
    // browser's own touch and text gestures suppressed so a drag reads as a
    // drag rather than a scroll or a selection.
    pub c_event_gesture_zone {
        min-height: "340px";
        touch-action: "none";
        user-select: "none";
        -webkit-user-select: "none";
    }
}
