use super::*;

/// Four-sided edge dimensions for margin, padding, and safe-area insets.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct Edges {
    /// Top edge inset in logical pixels.
    #[get(type(copy))]
    pub(crate) top: f32,
    /// Right edge inset in logical pixels.
    #[get(type(copy))]
    pub(crate) right: f32,
    /// Bottom edge inset in logical pixels.
    #[get(type(copy))]
    pub(crate) bottom: f32,
    /// Left edge inset in logical pixels.
    #[get(type(copy))]
    pub(crate) left: f32,
}

/// Per-corner border radius specification in logical pixels.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct CornerRadius {
    /// Top-left corner radius.
    #[get(type(copy))]
    pub(crate) top_left: f32,
    /// Top-right corner radius.
    #[get(type(copy))]
    pub(crate) top_right: f32,
    /// Bottom-right corner radius.
    #[get(type(copy))]
    pub(crate) bottom_right: f32,
    /// Bottom-left corner radius.
    #[get(type(copy))]
    pub(crate) bottom_left: f32,
}

/// 2D point in logical pixels.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct Point {
    /// Horizontal coordinate.
    #[get(type(copy))]
    pub(crate) x: f32,
    /// Vertical coordinate.
    #[get(type(copy))]
    pub(crate) y: f32,
}

/// 2D size in logical pixels.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct Size {
    /// Horizontal width.
    #[get(type(copy))]
    pub(crate) width: f32,
    /// Vertical height.
    #[get(type(copy))]
    pub(crate) height: f32,
}

/// Axis-aligned rectangle in logical pixels.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct Rect {
    /// Left X coordinate.
    #[get(type(copy))]
    pub(crate) x: f32,
    /// Top Y coordinate.
    #[get(type(copy))]
    pub(crate) y: f32,
    /// Width of the rectangle.
    #[get(type(copy))]
    pub(crate) width: f32,
    /// Height of the rectangle.
    #[get(type(copy))]
    pub(crate) height: f32,
}
