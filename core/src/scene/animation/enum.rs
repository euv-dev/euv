/// Mathematical easing curves applied to retained scene node transitions.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Easing {
    /// Constant velocity interpolation.
    Linear,
    /// Smooth deceleration curve ideal for UI entrances and drawers.
    #[default]
    EaseOutCubic,
    /// Symmetric accelerate-then-decelerate curve.
    EaseInOutCubic,
    /// Subtle spring-like overshoot curve for modal and popover entrances.
    EaseOutBack,
}
