use super::*;

/// Single labeled value plotted inside `EuvChart`.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct EuvChartPoint {
    /// Short horizontal axis label (e.g. `"Mon"`).
    pub(crate) label: String,
    /// Numeric data point value.
    #[get(type(copy))]
    pub(crate) value: f32,
}

/// Pure-Rust vector chart widget (`ui.shadcn.com/docs/components/chart`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvChart {
    /// Chart title heading.
    pub(crate) title: String,
    /// Chart style (`Bar`, `Line`, `Area`).
    #[get(type(copy))]
    pub(crate) kind: EuvChartKind,
    /// Ordered data points.
    pub(crate) data: Vec<EuvChartPoint>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Searchable dropdown combobox (`ui.shadcn.com/docs/components/combobox`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvCombobox {
    /// Optional placeholder shown when search input is empty.
    #[new(skip)]
    #[prop(custom)]
    pub(crate) placeholder: String,
    /// Selectable option labels.
    pub(crate) options: Vec<String>,
    /// Optional controlled search query signal (creates internal state when omitted).
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) query: Option<Signal<String>>,
    /// Controlled selected option index signal.
    #[get(type(copy))]
    pub(crate) selected: Signal<usize>,
    /// Controlled popover open state signal.
    #[get(type(copy))]
    pub(crate) open: Signal<bool>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Production sortable, filterable, selectable data table (`ui.shadcn.com/docs/components/data-table`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvDataTable {
    /// Column header titles.
    pub(crate) headers: Vec<String>,
    /// Row cell matrices.
    pub(crate) rows: Vec<Vec<String>>,
    /// Optional controlled search filter signal (creates internal state when omitted).
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) filter: Option<Signal<String>>,
    /// Optional controlled ascending/descending sort toggle signal.
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) sort_ascending: Option<Signal<bool>>,
    /// Optional controlled selected row index signal.
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) selected_row: Option<Signal<Option<usize>>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Trigger button + popover calendar date picker (`ui.shadcn.com/docs/components/date-picker`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvDatePicker {
    /// Month and year display label (e.g. `"November 2026"`).
    pub(crate) month_label: String,
    /// Controlled selected day of month (`1..=31`).
    #[get(type(copy))]
    pub(crate) selected_day: Signal<u32>,
    /// Controlled popover open state.
    #[get(type(copy))]
    pub(crate) open: Signal<bool>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Input with leading/trailing visual addons (`ui.shadcn.com/docs/components/input-group`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvInputGroup {
    /// Optional leading prefix addon text (e.g. `"https://"`).
    #[new(skip)]
    #[prop(custom)]
    pub(crate) prefix: String,
    /// Controlled input text signal.
    #[get(type(copy))]
    pub(crate) value: Signal<String>,
    /// Optional placeholder text inside the input.
    #[new(skip)]
    #[prop(custom)]
    pub(crate) placeholder: String,
    /// Optional trailing suffix badge or unit text (e.g. `".dev"`).
    #[new(skip)]
    #[prop(custom)]
    pub(crate) suffix: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Rich list item row (`ui.shadcn.com/docs/components/item`).
#[derive(Clone, CustomDebug, Data, New, Props)]
pub struct EuvItem {
    /// Optional leading short tag or avatar monogram.
    #[new(skip)]
    #[prop(custom)]
    pub(crate) leading: String,
    /// Primary item title.
    pub(crate) title: String,
    /// Optional secondary muted description.
    #[new(skip)]
    #[prop(custom)]
    pub(crate) description: String,
    /// Optional trailing metadata badge text.
    #[new(skip)]
    #[prop(custom)]
    pub(crate) trailing: String,
    /// Optional click handler.
    #[new(skip)]
    #[debug(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) on_click: Option<UiEventHandler>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Compact native-styled inline selector (`ui.shadcn.com/docs/components/native-select`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvNativeSelect {
    /// Ordered selectable option labels.
    pub(crate) options: Vec<String>,
    /// Controlled selected index signal.
    #[get(type(copy))]
    pub(crate) selected: Signal<usize>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Top-level horizontal navigation menu (`ui.shadcn.com/docs/components/navigation-menu`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvNavigationMenu {
    /// Ordered menu items `(label, detail_summary)`.
    pub(crate) items: Vec<(String, String)>,
    /// Controlled active expanded menu index.
    #[get(type(copy))]
    pub(crate) active: Signal<Option<usize>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Draggable two-pane resizable split panel group (`ui.shadcn.com/docs/components/resizable`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvResizable {
    /// Left panel node handle.
    #[get(type(copy))]
    pub(crate) left: NodeId,
    /// Right panel node handle.
    #[get(type(copy))]
    pub(crate) right: NodeId,
    /// Controlled split ratio in `[0.25, 0.75]`.
    #[get(type(copy))]
    pub(crate) split_ratio: Signal<f32>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Hairline separator with optional centered caption (`ui.shadcn.com/docs/components/separator`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvSeparator {
    /// Optional inline label centered across the separator line.
    pub(crate) label: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Stacking rich Sonner toast deck with dismiss action (`ui.shadcn.com/docs/components/sonner`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvSonner {
    /// Reactive stack of `(title, description)` toast entries.
    #[get(type(copy))]
    pub(crate) messages: Signal<Vec<(String, String)>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Direction-aware container switching horizontal alignment cleanly between LTR and RTL (`ui.shadcn.com/docs/components/direction`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvDirection {
    /// Active text direction mode.
    #[get(type(copy))]
    pub(crate) direction: EuvTextDirection,
    /// Child content node handle.
    #[get(type(copy))]
    pub(crate) child: NodeId,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Single execution span entry displayed inside `EuvTrace`.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct EuvTraceSpan {
    /// Span operation label (e.g. `"bvh.raytrace"`).
    pub(crate) name: String,
    /// Elapsed span duration in milliseconds.
    #[get(type(copy))]
    pub(crate) duration_ms: f32,
    /// Relative start offset in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) offset_frac: f32,
    /// Relative width fraction in `[0.05, 1.0]`.
    #[get(type(copy))]
    pub(crate) width_frac: f32,
    /// Semantic tone indicating healthy/warn/error status.
    #[get(type(copy))]
    pub(crate) tone: EuvTone,
}

/// Waterfall execution trace timeline (`ui.shadcn.com/docs/components/trace`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvTrace {
    /// Trace header title.
    pub(crate) title: String,
    /// Ordered execution spans.
    pub(crate) spans: Vec<EuvTraceSpan>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// File or media attachment chip with size badge and optional remove callback (`ui.shadcn.com/docs/components/attachment`).
#[derive(Clone, CustomDebug, Data, New, Props)]
pub struct EuvAttachment {
    /// Attachment file name.
    pub(crate) filename: String,
    /// Human-readable file size or MIME caption.
    pub(crate) meta: String,
    /// Reactive visibility signal set to `false` when removed.
    #[get(type(copy))]
    pub(crate) visible: Signal<bool>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Chat or assistant message bubble (`ui.shadcn.com/docs/components/bubble` / `message`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvBubble {
    /// Sender display name.
    pub(crate) author: String,
    /// Message text body.
    pub(crate) content: String,
    /// Optional timestamp caption.
    #[new(skip)]
    #[prop(custom)]
    pub(crate) timestamp: String,
    /// When `true`, renders right-aligned primary outgoing bubble; otherwise left-aligned incoming surface.
    #[new(skip)]
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) outgoing: bool,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Interactive status or annotation marker badge (`ui.shadcn.com/docs/components/marker`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvMarker {
    /// Short marker index or code (e.g. `"01"`).
    pub(crate) code: String,
    /// Descriptive marker label.
    pub(crate) label: String,
    /// Semantic status tone.
    #[get(type(copy))]
    pub(crate) tone: EuvTone,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Interactive single-choice questionnaire card (`ui.shadcn.com/docs/components/questionnaire`).
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvQuestionnaire {
    /// Question prompt title.
    pub(crate) prompt: String,
    /// Selectable answer choices.
    pub(crate) options: Vec<String>,
    /// Controlled selected choice index signal.
    #[get(type(copy))]
    pub(crate) selected: Signal<Option<usize>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Cross-platform context action menu (`ui.shadcn.com/docs/components/context-menu`) activated via click, secondary pointer button, or touch long-press.
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvContextMenu {
    /// Trigger surface label displayed in the interactive target zone.
    pub(crate) trigger_label: String,
    /// Ordered context menu action labels.
    pub(crate) items: Vec<String>,
    /// Optional controlled menu open signal (creates internal state when omitted).
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) open: Option<Signal<bool>>,
    /// Optional controlled last-selected action index signal.
    #[new(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) selected: Option<Signal<Option<usize>>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
