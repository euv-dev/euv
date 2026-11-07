use super::*;

/// The visual variants of a toast.
///
/// The reference publishes two: the neutral one and the destructive one,
/// which paints the card with the destructive surface and its border.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvToastVariant {
    /// The neutral toast.
    #[default]
    Default,
    /// The toast that reports a failure.
    Destructive,
}
