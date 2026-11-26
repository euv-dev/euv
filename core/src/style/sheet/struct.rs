use super::*;

/// Complete compiled style value combining layout, paint, and typography with zero heap allocations.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct Style {
    /// Layout geometry properties.
    #[get(type(copy))]
    pub(crate) layout: LayoutStyle,
    /// Visual paint properties.
    #[get(type(copy))]
    pub(crate) paint: PaintStyle,
    /// Typography properties.
    #[get(type(copy))]
    pub(crate) text: TextStyle,
}

/// Responsive style bundle resolving deterministically for compact, medium, and expanded viewports.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct ResponsiveStyle {
    /// Base style applied across all breakpoints.
    #[get(type(copy))]
    pub(crate) base: Style,
    /// Optional style override when viewport is `Breakpoint::Compact`.
    #[new(skip)]
    #[get(skip)]
    pub(crate) compact: Option<Style>,
    /// Optional style override when viewport is `Breakpoint::Medium`.
    #[new(skip)]
    #[get(skip)]
    pub(crate) medium: Option<Style>,
}
