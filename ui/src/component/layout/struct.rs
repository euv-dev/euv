use super::*;

/// Adaptive cross-platform application shell (Desktop sidebar + top context bar, iPadOS rail, Mobile header + drawer).
#[derive(Clone, Data, Debug, New, Props)]
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
    /// Optional action controls rendered on the right side of the persistent top header bar.
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) top_bar_actions: Option<NodeId>,
    /// Optional compact inline actions rendered beside the mobile header Menu button.
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) mobile_header_actions: Option<NodeId>,
    /// Optional footer widget rendered at the bottom of the navigation sidebar / mobile drawer.
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) sidebar_footer: Option<NodeId>,
    /// Optional live breadcrumb / page context label rendered in the desktop top header bar.
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) breadcrumb: Option<Signal<String>>,
    /// Optional persistent bottom navigation bar activating Twitter/X-style bottom-tab shell layout.
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) bottom_nav: Option<NodeId>,
    /// Ordered route paths cycled directly by horizontal left/right swipes across the viewport.
    #[new(skip)]
    #[prop(custom)]
    pub(crate) swipe_routes: Vec<String>,
    /// Whether the top navigation bar renders a live real-time FPS telemetry pill.
    #[new(skip)]
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) show_fps: bool,
    /// Optional reactive theme controller enabling zero-teardown in-place shell restyling.
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) theme: Option<UiThemeState>,
    /// Optional reactive per-page padding signal synchronized with an embedded `EuvRouterOutlet`.
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) page_padding: Option<Signal<Edges>>,
}

/// Hairline horizontal or vertical structural divider.
#[derive(Clone, Copy, Data, Debug, New, Props)]
pub struct EuvDivider {
    /// True for vertical divider; false for horizontal.
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) vertical: bool,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Zero-boilerplate standardized page wrapper combining `EuvHeader` and uniform vertical section spacing.
#[derive(Clone, Data, Debug, New, Props)]
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
#[derive(Clone, Data, Debug, Default, New, Props)]
pub struct EuvActionRow {
    /// Pre-mounted child controls inside the action row.
    #[new(skip)]
    pub(crate) children: Vec<NodeId>,
}

/// Responsive 1/2/3-column KPI stat grid synchronized with the active viewport breakpoint.
#[derive(Clone, Data, Debug, Default, New, Props)]
pub struct EuvStatGrid {
    /// Pre-mounted stat card children.
    #[new(skip)]
    pub(crate) cards: Vec<NodeId>,
}
