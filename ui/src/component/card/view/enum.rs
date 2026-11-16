use super::*;

/// The heading level a title of the library is rendered at.
///
/// A title is a heading, and the level it takes decides where it sits in the
/// outline of the page. The library cannot know the level a page needs: the
/// same card is the section of one page and the whole panel of another, so
/// the level is a setting of the component rather than a fixed part of it.
///
/// The level each component takes when the caller names none: the title of a
/// card is a part of a section, and the title of a result is the title of
/// the page it stands for. A page that nests the component under a heading
/// of its own names the level it needs, and the outline of the page stays
/// free of a skipped level.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvHeadingLevel {
    /// A first-level heading, for the title of the page.
    H1,
    /// A second-level heading, for the title of a section of the page.
    H2,
    /// A third-level heading, for the title of a part of a section.
    H3,
    /// A fourth-level heading, for the title of a part of a part.
    H4,
    /// A fifth-level heading, for the title of a detail of a part.
    H5,
    /// A sixth-level heading, for the title of the deepest named thing.
    H6,
}
