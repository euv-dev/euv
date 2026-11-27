use super::*;

/// Centered responsive page container with constrained maximum width.
#[derive(Clone, Data, New, Props)]
pub struct EuvPageContainer {
    /// Maximum logical content width in pixels (`0.0` defaults to `1160.0`).
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) max_width: f32,
    /// Ordered child node handles.
    #[new(skip)]
    pub(crate) children: Vec<NodeId>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Bordered surface panel for grouped tools or inspector content.
#[derive(Clone, Data, New, Props)]
pub struct EuvPanel {
    /// Optional panel heading.
    pub(crate) title: String,
    /// Ordered child nodes inside the panel.
    #[new(skip)]
    pub(crate) children: Vec<NodeId>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Flex spacing helper arranging child nodes horizontally or vertically with uniform gap.
#[derive(Clone, Data, New, Props)]
pub struct EuvSpace {
    /// When `true`, lays out children in a vertical column; otherwise horizontal row.
    #[get(type(copy))]
    pub(crate) vertical: bool,
    /// Uniform gap in logical pixels.
    #[get(type(copy))]
    pub(crate) gap: f32,
    /// Ordered child node handles.
    #[new(skip)]
    pub(crate) children: Vec<NodeId>,
}

/// Responsive fixed-column CSS-Grid-equivalent container.
#[derive(Clone, Data, New, Props)]
pub struct EuvGrid {
    /// Number of equal-width grid columns.
    #[get(type(copy))]
    pub(crate) columns: u16,
    /// Uniform grid gap in logical pixels.
    #[get(type(copy))]
    pub(crate) gap: f32,
    /// Ordered child node handles.
    #[new(skip)]
    pub(crate) children: Vec<NodeId>,
}

/// Clipped scrollable viewport wrapper with explicit height.
#[derive(Clone, Data, New, Props)]
pub struct EuvScrollArea {
    /// Viewport height in logical pixels.
    #[get(type(copy))]
    pub(crate) height: f32,
    /// Ordered child nodes inside the scroll area.
    #[new(skip)]
    pub(crate) children: Vec<NodeId>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Two-pane split layout with proportional primary ratio.
#[derive(Clone, Data, New, Props)]
pub struct EuvSplitter {
    /// Primary pane percentage in `[15.0, 85.0]`.
    #[get(type(copy))]
    pub(crate) ratio_percent: f32,
    /// Primary pane node handle.
    #[get(type(copy))]
    pub(crate) primary: NodeId,
    /// Secondary pane node handle.
    #[get(type(copy))]
    pub(crate) secondary: NodeId,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Top navigation bar displaying brand title and trailing action nodes.
#[derive(Clone, Data, New, Props)]
pub struct EuvNavbar {
    /// Brand title text.
    pub(crate) brand: String,
    /// Trailing action nodes.
    #[new(skip)]
    pub(crate) actions: Vec<NodeId>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Page footer strip with copyright or metadata text.
#[derive(Clone, Data, New, Props)]
pub struct EuvFooter {
    /// Footer text content.
    pub(crate) text: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Contiguous horizontal group of button nodes.
#[derive(Clone, Data, New, Props)]
pub struct EuvButtonGroup {
    /// Ordered button child handles.
    pub(crate) children: Vec<NodeId>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Pressable two-state toggle pill bound to a reactive `Signal<bool>`.
#[derive(Clone, Data, New, Props)]
pub struct EuvToggle {
    /// Reactive pressed state signal.
    #[get(type(copy))]
    pub(crate) pressed: Signal<bool>,
    /// Toggle label text.
    pub(crate) label: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
