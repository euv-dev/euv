use super::*;

/// The side of the trigger the [`euv_tooltip`] bubble is rendered on.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvTooltipPlacement {
    /// Above the trigger, horizontally centred.
    #[default]
    Top,
    /// Below the trigger, horizontally centred.
    Bottom,
    /// Left of the trigger, vertically centred.
    Left,
    /// Right of the trigger, vertically centred.
    Right,
}
