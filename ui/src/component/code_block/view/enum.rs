use super::*;

/// The surface a code block is drawn on.
///
/// The reference publishes two: the plain code block and the one that
/// says the snippet does not run.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvCodeBlockVariant {
    /// The code block is drawn on the page surface.
    #[default]
    Default,
    /// The code block says the snippet does not run.
    Destructive,
}

/// The size of a code block.
///
/// The reference publishes two: the default block and the small one.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvCodeBlockSize {
    /// The block stands at the default size.
    #[default]
    Default,
    /// The block stands at the small size.
    Sm,
}

/// The state of one line of a code block.
///
/// The reference publishes four: the plain line, the line that was
/// added, the line that was removed and the line that is called out.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvCodeBlockLineVariant {
    /// The line is an ordinary line of the snippet.
    #[default]
    Default,
    /// The line was added by the change the snippet shows.
    Added,
    /// The line was removed by the change the snippet shows.
    Removed,
    /// The line is called out.
    Highlighted,
}
