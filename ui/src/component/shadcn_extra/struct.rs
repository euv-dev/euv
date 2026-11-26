use super::*;

/// Modal confirmation dialog aligned with shadcn/ui `AlertDialog`.
#[derive(Clone, Data, Debug, New)]
pub struct EuvAlertDialog {
    /// Reactive dialog open state.
    #[get(type(copy))]
    pub(crate) open: Signal<bool>,
    /// Dialog heading text.
    pub(crate) title: String,
    /// Explanatory description text.
    #[new(skip)]
    pub(crate) description: String,
    /// Cancel button label (`"Cancel"` when omitted).
    #[new(skip)]
    pub(crate) cancel_label: String,
    /// Primary destructive/confirm button label (`"Continue"` when omitted).
    #[new(skip)]
    pub(crate) confirm_label: String,
    /// Reactive flag set to `true` when the user confirms.
    #[get(type(copy))]
    pub(crate) confirmed: Signal<bool>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Interactive collapsible disclosure panel aligned with shadcn/ui `Collapsible`.
#[derive(Clone, Data, Debug, New)]
pub struct EuvCollapsible {
    /// Reactive expanded state.
    #[get(type(copy))]
    pub(crate) open: Signal<bool>,
    /// Header title text.
    pub(crate) title: String,
    /// Mounted body node toggled by `open`.
    #[get(type(copy))]
    pub(crate) content: NodeId,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Wrapping multi-option toggle button group aligned with shadcn/ui `ToggleGroup`.
#[derive(Clone, Data, Debug, New)]
pub struct EuvToggleGroup {
    /// Ordered toggle item labels.
    pub(crate) options: Vec<String>,
    /// Reactive selected item index.
    #[get(type(copy))]
    pub(crate) selected: Signal<usize>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Segmented one-time passcode input aligned with shadcn/ui `InputOTP`.
#[derive(Clone, Data, Debug, New)]
pub struct EuvInputOtp {
    /// Reactive passcode string value.
    #[get(type(copy))]
    pub(crate) value: Signal<String>,
    /// Total number of passcode digit slots (`4..=8`).
    #[get(type(copy))]
    pub(crate) length: usize,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Searchable command palette aligned with shadcn/ui `Command`.
#[derive(Clone, Data, Debug, New)]
pub struct EuvCommand {
    /// Optional controlled reactive search query string (creates internal state when omitted).
    #[new(skip)]
    #[get(skip)]
    pub(crate) query: Option<Signal<String>>,
    /// Input placeholder prompt (`"Type a command or search..."` when empty).
    #[new(skip)]
    pub(crate) placeholder: String,
    /// Available command item labels.
    pub(crate) items: Vec<String>,
    /// Reactive index of the last activated command item.
    #[get(type(copy))]
    pub(crate) selected: Signal<Option<usize>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Interactive preview card aligned with shadcn/ui `HoverCard`.
#[derive(Clone, Data, Debug, New)]
pub struct EuvHoverCard {
    /// Optional controlled reactive preview card visibility flag (creates internal state when omitted).
    #[new(skip)]
    #[get(skip)]
    pub(crate) open: Option<Signal<bool>>,
    /// Trigger link button label.
    pub(crate) trigger_label: String,
    /// Preview card heading.
    pub(crate) title: String,
    /// Optional preview card body text.
    #[new(skip)]
    pub(crate) description: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Interactive 7-column monthly calendar picker aligned with shadcn/ui `Calendar`.
#[derive(Clone, Data, Debug, New)]
pub struct EuvCalendar {
    /// Month and year display heading (e.g. `"November 2026"`).
    pub(crate) month_label: String,
    /// Number of days in the displayed month (`28..=31`).
    #[get(type(copy))]
    pub(crate) days_in_month: u32,
    /// Reactive selected day of month (`1..=days_in_month`).
    #[get(type(copy))]
    pub(crate) selected_day: Signal<u32>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Aspect-ratio constrained media container aligned with shadcn/ui `AspectRatio`.
#[derive(Clone, Copy, Data, Debug, New)]
pub struct EuvAspectRatio {
    /// Target width-to-height ratio (e.g. `16.0 / 9.0`).
    #[get(type(copy))]
    pub(crate) ratio: f32,
    /// Mounted child node displayed inside the aspect-ratio box.
    #[get(type(copy))]
    pub(crate) child: NodeId,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Responsive horizontal menubar aligned with shadcn/ui `Menubar`.
#[derive(Clone, Data, Debug, New)]
pub struct EuvMenubar {
    /// Top-level menu item labels.
    pub(crate) menus: Vec<String>,
    /// Reactive active menu index, or `None` when closed.
    #[get(type(copy))]
    pub(crate) active_menu: Signal<Option<usize>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Responsive slide carousel aligned with shadcn/ui `Carousel`.
#[derive(Clone, Data, Debug, New)]
pub struct EuvCarousel {
    /// Slide content titles or summaries.
    pub(crate) slides: Vec<String>,
    /// Reactive active slide index.
    #[get(type(copy))]
    pub(crate) active_slide: Signal<usize>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
