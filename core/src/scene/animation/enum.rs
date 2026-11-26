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
    /// Emphasized quintic deceleration curve (`1 - (1 - t)^5`) for silky-smooth drawers and sheets.
    EaseOutQuint,
    /// Crisp cubic acceleration curve (`t^3`) for natural overlay and drawer exits.
    EaseInCubic,
    /// Lightly damped physical spring curve for expressive drawers, sheets, and floating cards.
    SpringGentle,
    /// Crisp high-stiffness spring curve for responsive modals, popovers, and pill transitions.
    SpringSnappy,
}
