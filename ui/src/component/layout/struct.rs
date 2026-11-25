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
    /// Optional active route signal used to auto-close the mobile drawer on navigation.
    #[new(skip)]
    #[get(skip)]
    pub(crate) route_signal: Option<Signal<String>>,
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

/// Zero-boilerplate standardized page wrapper combining `EuvHeader` and uniform vertical section spacing.
#[derive(Clone, Data, Debug, New)]
pub struct EuvPage {
    /// Page title text.
    pub(crate) title: String,
    /// Page subtitle description.
    pub(crate) subtitle: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
    /// Ordered section children rendered beneath the page header.
    #[new(skip)]
    pub(crate) sections: Vec<NodeId>,
}

/// Standardized wrapping horizontal action bar aligning buttons and controls on one center baseline.
#[derive(Clone, Data, Debug, Default, New)]
pub struct EuvActionRow {
    /// Pre-mounted child controls inside the action row.
    #[new(skip)]
    pub(crate) children: Vec<NodeId>,
}

/// Responsive 1/2/3-column KPI stat grid synchronized with the active viewport breakpoint.
#[derive(Clone, Data, Debug, Default, New)]
pub struct EuvStatGrid {
    /// Pre-mounted stat card children.
    #[new(skip)]
    pub(crate) cards: Vec<NodeId>,
}
