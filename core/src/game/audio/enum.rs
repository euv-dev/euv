use super::*;

/// Built-in and custom sound cue categories dispatched by `AudioBus`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SoundCue {
    /// Weapon fire or laser pulse.
    Laser,
    /// Impact explosion or enemy destroyed.
    Explosion,
    /// Collectible pickup or score increment.
    Pickup,
    /// Elastic physics bounce or wall hit.
    Bounce,
    /// Player jump or dash impulse.
    Dash,
    /// UI interaction click inside game HUD.
    UiClick,
    /// Application-defined custom sound identifier.
    Custom(u32),
}
