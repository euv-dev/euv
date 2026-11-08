use super::*;

/// The size of a native select.
///
/// The reference publishes two: the control that stands at the same
/// height as the default button, and the smaller one that sits beside
/// it.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvNativeSelectSize {
    /// The control stands at the default control height.
    #[default]
    Default,
    /// The control stands at the small control height.
    Sm,
}
