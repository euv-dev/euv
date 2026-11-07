use super::*;

/// The surface of an item.
///
/// The reference publishes three: a transparent row, a bordered row and
/// a row on the muted surface.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvItemVariant {
    /// A transparent row that only shows its border on focus.
    #[default]
    Default,
    /// A row with a visible border.
    Outline,
    /// A row on the muted surface.
    Muted,
}

/// The density of an item.
///
/// The reference publishes two: the roomy row and the compact one.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvItemSize {
    /// The roomy row.
    #[default]
    Default,
    /// The compact row.
    Sm,
}

/// The kind of media shown beside an item.
///
/// The reference publishes three: nothing at all, an icon and a picture.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvItemMediaVariant {
    /// No frame around the media.
    #[default]
    Default,
    /// The media is an icon on a muted tile.
    Icon,
    /// The media is a picture in a clipped frame.
    Image,
}
