use super::*;

/// The visual variants of a toggle.
///
/// The default toggle is a borderless control that fills with the accent
/// surface when it is pressed; the outline variant carries a one-pixel
/// border so a row of toggles reads as a set of separate controls.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvToggleVariant {
    /// A borderless control that fills with the accent surface when pressed.
    #[default]
    Default,
    /// A bordered control.
    Outline,
}

/// The sizes of a toggle.
///
/// The reference publishes three: the default box, a smaller one and a
/// larger one.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvToggleSize {
    /// The default control size.
    #[default]
    Default,
    /// The small control size.
    Small,
    /// The large control size.
    Large,
}
