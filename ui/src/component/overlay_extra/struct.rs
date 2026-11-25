use super::*;

/// Side slide-over drawer portal controlled by `Signal<bool>`.
#[derive(Clone, Data)]
pub struct EuvDrawer {
    /// Reactive open state.
    #[get(type(copy))]
    pub(crate) open: Signal<bool>,
    /// Drawer header title.
    pub(crate) title: String,
    /// Mounted drawer body node.
    #[get(type(copy))]
    pub(crate) body: NodeId,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Bottom sheet portal ideal for iOS / Android / iPadOS action flows.
#[derive(Clone, Data)]
pub struct EuvSheet {
    /// Reactive open state.
    #[get(type(copy))]
    pub(crate) open: Signal<bool>,
    /// Sheet header title.
    pub(crate) title: String,
    /// Mounted sheet body node.
    #[get(type(copy))]
    pub(crate) body: NodeId,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Anchored popover surface toggled by `Signal<bool>`.
#[derive(Clone, Data)]
pub struct EuvPopover {
    /// Reactive open state.
    #[get(type(copy))]
    pub(crate) open: Signal<bool>,
    /// Trigger child node.
    #[get(type(copy))]
    pub(crate) trigger: NodeId,
    /// Popover content node.
    #[get(type(copy))]
    pub(crate) content: NodeId,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Compact floating tooltip label attached to a trigger node.
#[derive(Clone, Data)]
pub struct EuvTooltip {
    /// Tooltip help text.
    pub(crate) text: String,
    /// Trigger child node.
    #[get(type(copy))]
    pub(crate) trigger: NodeId,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Click-triggered dropdown action menu.
#[derive(Clone, Data)]
pub struct EuvDropdown {
    /// Trigger button label.
    pub(crate) label: String,
    /// Action item labels.
    pub(crate) items: Vec<String>,
    /// Reactive open flag.
    #[get(type(copy))]
    pub(crate) open: Signal<bool>,
    /// Last chosen item index signal.
    #[get(type(copy))]
    pub(crate) last_selected: Signal<Option<usize>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Rich corner notification card with title, message, and semantic tone.
#[derive(Clone, Data)]
pub struct EuvNotification {
    /// Reactive visibility signal.
    #[get(type(copy))]
    pub(crate) visible: Signal<bool>,
    /// Notification heading.
    pub(crate) title: String,
    /// Notification detail message.
    pub(crate) message: String,
    /// Semantic tone.
    #[get(type(copy))]
    pub(crate) tone: EuvTone,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Full-surface loading indicator overlay with spinner and caption.
#[derive(Clone, Data)]
pub struct EuvLoading {
    /// Reactive active loading state.
    #[get(type(copy))]
    pub(crate) active: Signal<bool>,
    /// Status message.
    pub(crate) label: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
