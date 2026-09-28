use super::*;

/// Props for the `euv_rating` component.
///
/// Defines the strongly-typed interface for the star rating. The value and
/// scale live in caller-owned signals so a parent can drive the rating and
/// receive updates when `readonly` is false.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvRatingProps {
    /// The current rating value, which may be fractional for half stars.
    pub value: Signal<f64>,
    /// The maximum rating value, the number of stars rendered.
    pub max: Signal<f64>,
    /// Whether the rating is display-only, suppressing the fill overlay.
    pub readonly: Signal<bool>,
    /// The star glyph size.
    pub size: EuvRatingSize,
}
