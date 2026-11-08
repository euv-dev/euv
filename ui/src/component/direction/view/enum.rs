use super::*;

/// The direction a block of content is laid out in.
///
/// The reference derives the direction from the locale. A caller that
/// knows the direction states it, and everything inside the provider
/// follows: text runs the right way, icons mirror, and the writing mode
/// of the whole subtree flips with it.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvDirection {
    /// The content reads from left to right.
    #[default]
    Ltr,
    /// The content reads from right to left.
    Rtl,
}
