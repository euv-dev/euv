use super::*;

/// The visual variant of the `euv_badge` component.
///
/// The variants are the ones the registry publishes: the filled default, a
/// quiet secondary, a destructive fill and the bare outline.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvBadgeVariant {
    /// The filled default badge.
    #[default]
    Default,
    /// A badge on the secondary surface.
    Secondary,
    /// A badge that marks a destructive state.
    Destructive,
    /// A badge with a border and no fill.
    Outline,
}
