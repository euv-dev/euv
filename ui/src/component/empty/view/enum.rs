use super::*;

/// The size the picture of an empty state is drawn at.
///
/// The reference publishes two: a picture that sits on the background
/// as it is, and a picture that sits on the muted surface in a rounded
/// box.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvEmptyMediaVariant {
    /// The picture sits on the background as it is.
    #[default]
    Default,
    /// The picture sits on the muted surface in a rounded box.
    Icon,
}
