use super::*;

class! {
    // Functional demonstrations own their timing and geometry, including unstyled builds.
    pub(crate) c_animation_stage {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        // The demonstration keeps an intrinsic stage independent of control heights.
        min-height: format!("calc({} * 2 + {})", var!(space-3xl), var!(space-lg));
    }

    pub(crate) c_animation_fade {
        opacity: "1";
        visibility: "visible";
        pointer-events: "auto";
        transition: "none !important";
        @media ((prefers-reduced-motion: no-preference)) {
            transition: "opacity 500ms ease-in-out, visibility 500ms !important";
        }
        @media ((prefers-reduced-motion: reduce)) {
            transition: "none !important";
        }
    }

    pub(crate) c_animation_fade_hidden {
        opacity: "0";
        visibility: "hidden";
        pointer-events: "none";
    }

    pub(crate) c_animation_symbol {
        display: "inline-block";
        font-size: var!(font-5xl);
        line-height: "1";
        animation: "none !important";
    }

    pub(crate) c_animation_spin {
        @media ((prefers-reduced-motion: no-preference)) {
            animation: "example-animation-spin 1500ms linear infinite !important";
        }
        @media ((prefers-reduced-motion: reduce)) {
            animation: "none !important";
        }
    }

    pub(crate) c_animation_pulse {
        @media ((prefers-reduced-motion: no-preference)) {
            animation: "example-animation-pulse 1500ms ease-in-out infinite !important";
        }
        @media ((prefers-reduced-motion: reduce)) {
            animation: "none !important";
        }
    }

    pub(crate) c_animation_progress_track {
        display: "block";
        width: "100%";
        height: var!(space-lg);
        overflow: "hidden";
        border: format!("{} solid currentColor", var!(border-width));
        border-radius: var!(radius-sm);
        box-sizing: "border-box";
    }

    pub(crate) c_animation_progress_fill {
        display: "block";
        width: "100%";
        height: "100%";
        background: "currentColor";
        transform-origin: "left center";
        transform: "scaleX(0)";
        animation: "none !important";
    }

    pub(crate) c_animation_progress_running {
        transform: "scaleX(1)";
        @media ((prefers-reduced-motion: no-preference)) {
            animation: "example-animation-progress 1600ms ease-in-out both !important";
        }
        @media ((prefers-reduced-motion: reduce)) {
            animation: "none !important";
            transform: "scaleX(1)";
        }
    }

    pub(crate) c_animation_progress_replay {
        c_animation_progress_running();
        @media ((prefers-reduced-motion: no-preference)) {
            animation: "example-animation-progress-replay 1600ms ease-in-out both !important";
        }
        @media ((prefers-reduced-motion: reduce)) {
            animation: "none !important";
            transform: "scaleX(1)";
        }
    }

    pub(crate) c_animation_scale {
        display: "block";
        width: "100%";
        height: format!("calc({} * 2 + {})", var!(space-3xl), var!(space-lg));
        background: "currentColor";
        border-radius: var!(radius-sm);
        transform: "scale(1)";
        transition: "none !important";
        @media ((prefers-reduced-motion: no-preference)) {
            transition: "transform 500ms ease-in-out !important";
        }
        @media ((prefers-reduced-motion: reduce)) {
            transition: "none !important";
        }
    }

    pub(crate) c_animation_scale_shrink {
        transform: "scale(0.65)";
    }
}
