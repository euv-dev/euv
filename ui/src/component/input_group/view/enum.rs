use super::*;

/// Where an input-group addon sits relative to the control.
///
/// The reference publishes four: the two inline positions beside the
/// field and the two block positions above and below it.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvInputGroupAlign {
    /// Before the control, on the same line.
    #[default]
    InlineStart,
    /// After the control, on the same line.
    InlineEnd,
    /// Above the control, on its own line.
    BlockStart,
    /// Below the control, on its own line.
    BlockEnd,
}

/// The size of a button that sits inside an input group.
///
/// The reference publishes four: two text buttons and two square icon
/// buttons.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvInputGroupButtonSize {
    /// The extra-small text button.
    #[default]
    Xs,
    /// The small text button.
    Sm,
    /// The extra-small square icon button.
    IconXs,
    /// The small square icon button.
    IconSm,
}
