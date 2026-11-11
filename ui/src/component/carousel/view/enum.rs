use super::*;

/// The direction the slides of a carousel travel in.
///
/// The reference derives the padding of the track and of every slide from
/// the orientation, so the two are carried by the props of the parts that
/// need them.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvCarouselOrientation {
    /// The slides sit side by side and travel left and right.
    #[default]
    Horizontal,
    /// The slides sit one above the other and travel up and down.
    Vertical,
}
