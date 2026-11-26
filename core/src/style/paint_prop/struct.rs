use super::*;

/// Box border stroke specification.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct Border {
    /// Border stroke width in logical pixels.
    #[get(type(copy))]
    pub(crate) width: f32,
    /// Border stroke color.
    #[get(type(copy))]
    pub(crate) color: Color,
}

/// Box drop-shadow specification.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct Shadow {
    /// Horizontal offset in logical pixels.
    #[get(type(copy))]
    pub(crate) offset_x: f32,
    /// Vertical offset in logical pixels.
    #[get(type(copy))]
    pub(crate) offset_y: f32,
    /// Blur radius in logical pixels.
    #[get(type(copy))]
    pub(crate) blur_radius: f32,
    /// Spread radius in logical pixels.
    #[get(type(copy))]
    pub(crate) spread_radius: f32,
    /// Shadow tint color.
    #[get(type(copy))]
    pub(crate) color: Color,
}

/// Two-stop linear gradient specification.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct LinearGradient {
    /// Gradient angle in degrees (`0.0` = top-to-bottom, `90.0` = left-to-right).
    #[get(type(copy))]
    pub(crate) angle_deg: f32,
    /// Start color at stop `0.0`.
    #[get(type(copy))]
    pub(crate) start_color: Color,
    /// End color at stop `1.0`.
    #[get(type(copy))]
    pub(crate) end_color: Color,
}

/// 2D visual transform applied during painting without re-running layout.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct Transform2D {
    /// Horizontal translation in logical pixels.
    #[get(type(copy))]
    pub(crate) translate_x: f32,
    /// Vertical translation in logical pixels.
    #[get(type(copy))]
    pub(crate) translate_y: f32,
    /// Horizontal scale factor (`1.0` = identity).
    #[get(type(copy))]
    pub(crate) scale_x: f32,
    /// Vertical scale factor (`1.0` = identity).
    #[get(type(copy))]
    pub(crate) scale_y: f32,
    /// Clockwise rotation angle in radians.
    #[get(type(copy))]
    pub(crate) rotation_rad: f32,
}

/// Paint-only visual properties decoupled from layout calculation.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct PaintStyle {
    /// Optional solid background fill color.
    #[get(skip)]
    pub(crate) background: Option<Color>,
    /// Optional linear gradient background fill.
    #[get(skip)]
    pub(crate) gradient: Option<LinearGradient>,
    /// Border stroke width and color.
    #[get(type(copy))]
    pub(crate) border: Border,
    /// Per-corner border radii.
    #[get(type(copy))]
    pub(crate) radius: CornerRadius,
    /// Optional drop shadow.
    #[get(skip)]
    pub(crate) shadow: Option<Shadow>,
    /// Optional focus ring border drawn outside the box without altering layout.
    #[get(skip)]
    pub(crate) focus_ring: Option<Border>,
    /// Node opacity in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) opacity: f32,
    /// 2D paint transform.
    #[get(type(copy))]
    pub(crate) transform: Transform2D,
    /// Pointer cursor icon hint.
    #[get(type(copy))]
    pub(crate) cursor: CursorIcon,
    /// Pointer hit-testing policy.
    #[get(type(copy))]
    pub(crate) pointer_events: PointerEvents,
    /// When `true`, prevents scroll deltas from chaining/penetrating through to ancestor scroll containers.
    #[get(type(copy))]
    pub(crate) stop_scroll_propagation: bool,
    /// When `true`, stops pointer/click event bubbling past this node to ancestor handlers.
    #[get(type(copy))]
    pub(crate) stop_event_propagation: bool,
}
