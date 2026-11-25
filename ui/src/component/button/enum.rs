/// Visual emphasis variant for `EuvButton`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvButtonVariant {
    /// Filled brand primary button (default).
    #[default]
    Primary,
    /// Subtle secondary surface button.
    Secondary,
    /// High-contrast destructive action button.
    Destructive,
    /// Hairline-bordered outline button.
    Outline,
    /// Borderless transparent ghost button.
    Ghost,
    /// Text link button.
    Link,
}

/// Control height and padding preset for `EuvButton`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvButtonSize {
    /// Compact control height.
    Small,
    /// Standard control height.
    #[default]
    Default,
    /// Prominent large control height.
    Large,
    /// Square icon button.
    Icon,
}
