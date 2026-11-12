use super::*;

/// The visual variant of the `euv_badge` component.
///
/// The variants mirror shadcn/ui's filled, outlined and text-only badge styles.
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
    /// A transparent badge that highlights on hover.
    Ghost,
    /// A text-only badge styled like an inline link.
    Link,
}
