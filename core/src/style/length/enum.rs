/// CSS-compatible deterministic layout length unit.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Length {
    /// Automatic size determined by content or parent constraints.
    #[default]
    Auto,
    /// Device-independent logical pixels.
    Px(f32),
    /// Percentage of the parent container's dimension in `[0.0, 100.0]`.
    Percent(f32),
    /// Fraction of remaining free space for grid tracks (`fr`).
    Fraction(f32),
    /// Percentage of the root viewport width (`vw`).
    ViewportWidth(f32),
    /// Percentage of the root viewport height (`vh`).
    ViewportHeight(f32),
}
