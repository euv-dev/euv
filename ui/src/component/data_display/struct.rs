use super::*;

/// Compact dismissible or labeled metadata tag.
#[derive(Clone, Data)]
pub struct EuvTag {
    /// Tag text label.
    pub(crate) label: String,
    /// Semantic tone.
    #[get(type(copy))]
    pub(crate) tone: EuvTone,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Circular initials avatar surface.
#[derive(Clone, Data)]
pub struct EuvAvatar {
    /// Display initials (1-2 chars).
    pub(crate) initials: String,
    /// Diameter in logical pixels.
    #[get(type(copy))]
    pub(crate) size: f32,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Crisp vector-rendered symbolic icon badge.
#[derive(Clone, Data)]
pub struct EuvIcon {
    /// Symbolic glyph string.
    pub(crate) symbol: String,
    /// Icon box size in logical pixels.
    #[get(type(copy))]
    pub(crate) size: f32,
    /// Foreground color.
    #[get(type(copy))]
    pub(crate) color: Color,
}

/// Keyboard shortcut keycap indicator (`Cmd`, `Ctrl`, `K`).
#[derive(Clone, Data)]
pub struct EuvKbd {
    /// Keycap text.
    pub(crate) key: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Semantic typography text node across heading, body, and caption scales.
#[derive(Clone, Data)]
pub struct EuvTypography {
    /// Text string.
    pub(crate) text: String,
    /// Scale variant.
    #[get(type(copy))]
    pub(crate) variant: EuvTypographyVariant,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Empty-state placeholder card with title and explanation.
#[derive(Clone, Data)]
pub struct EuvEmpty {
    /// Empty-state heading.
    pub(crate) title: String,
    /// Supporting guidance text.
    pub(crate) description: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Shimmer-style skeleton loading placeholder rectangle.
#[derive(Clone, Data)]
pub struct EuvSkeleton {
    /// Width in logical pixels.
    #[get(type(copy))]
    pub(crate) width: f32,
    /// Height in logical pixels.
    #[get(type(copy))]
    pub(crate) height: f32,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Vector circular activity spinner rendered via `Canvas`.
#[derive(Clone, Data)]
pub struct EuvSpinner {
    /// Diameter in logical pixels.
    #[get(type(copy))]
    pub(crate) size: f32,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Single chronological milestone entry inside `EuvTimeline`.
#[derive(Clone, Debug, Getter, New, PartialEq, Eq)]
pub struct EuvTimelineItem {
    /// Milestone title.
    pub(crate) title: String,
    /// Timestamp or version label.
    pub(crate) timestamp: String,
    /// Event description.
    pub(crate) detail: String,
}

/// Vertical chronological timeline component.
#[derive(Clone, Data)]
pub struct EuvTimeline {
    /// Ordered milestone entries.
    pub(crate) items: Vec<EuvTimelineItem>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Structured vertical list component with dividers.
#[derive(Clone, Data)]
pub struct EuvList {
    /// Ordered row text labels.
    pub(crate) items: Vec<String>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Hierarchical node entry inside `EuvTree`.
#[derive(Clone, Debug, Getter, New, PartialEq, Eq)]
pub struct EuvTreeNode {
    /// Visible node label.
    pub(crate) label: String,
    /// Nested child tree nodes.
    pub(crate) children: Vec<Self>,
}

/// Indented hierarchical tree viewer component.
#[derive(Clone, Data)]
pub struct EuvTree {
    /// Root tree nodes.
    pub(crate) roots: Vec<EuvTreeNode>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
