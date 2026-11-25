use super::*;

/// Adaptive cross-platform application shell (Desktop sidebar, iPadOS rail, Mobile header + safe-area insets).
#[derive(Clone, Data, Debug, New)]
pub struct EuvAppShell {
    /// Application brand title shown in sidebar or mobile top bar.
    pub(crate) brand: String,
    /// Mounted sidebar navigation content node.
    #[get(type(copy))]
    pub(crate) sidebar: NodeId,
    /// Mounted main page content node.
    #[get(type(copy))]
    pub(crate) main_content: NodeId,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Hairline horizontal or vertical structural divider.
#[derive(Clone, Copy, Data, Debug, New)]
pub struct EuvDivider {
    /// True for vertical divider; false for horizontal.
    #[get(type(copy))]
    pub(crate) vertical: bool,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
