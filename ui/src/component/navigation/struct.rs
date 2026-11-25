use super::*;

/// Single sidebar navigation destination descriptor.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct EuvNavItem {
    /// Route path identifier.
    pub(crate) path: String,
    /// Human-readable navigation label.
    pub(crate) label: String,
}

/// Vertical sidebar navigation list highlighting the active route reactively.
#[derive(Clone, Data, Debug)]
pub struct EuvSidebarNav {
    /// Ordered navigation items.
    pub(crate) items: Vec<EuvNavItem>,
    /// Reactive current route signal.
    #[get(type(copy))]
    pub(crate) active_route: Signal<String>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Horizontal tab bar switching active tab index reactively.
#[derive(Clone, Data, Debug)]
pub struct EuvTabs {
    /// Ordered tab labels.
    pub(crate) labels: Vec<String>,
    /// Reactive active tab index signal.
    #[get(type(copy))]
    pub(crate) active: Signal<usize>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
