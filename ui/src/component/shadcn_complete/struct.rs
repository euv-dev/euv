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
#[derive(Clone, Data, Debug, New)]
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
#[derive(Clone, Data, Debug, New)]
pub struct EuvCombobox {
    /// Placeholder shown when search input is empty.
    pub(crate) placeholder: String,
    /// Selectable option labels.
    pub(crate) options: Vec<String>,
    /// Controlled search query signal.
    #[get(type(copy))]
    pub(crate) query: Signal<String>,
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
#[derive(Clone, Data, Debug, New)]
pub struct EuvDataTable {
    /// Column header titles.
    pub(crate) headers: Vec<String>,
    /// Row cell matrices.
    pub(crate) rows: Vec<Vec<String>>,
    /// Controlled search filter signal.
    #[get(type(copy))]
    pub(crate) filter: Signal<String>,
    /// Controlled ascending/descending sort toggle signal.
    #[get(type(copy))]
    pub(crate) sort_ascending: Signal<bool>,
    /// Controlled selected row index signal.
    #[get(type(copy))]
    pub(crate) selected_row: Signal<Option<usize>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Trigger button + popover calendar date picker (`ui.shadcn.com/docs/components/date-picker`).
#[derive(Clone, Data, Debug, New)]
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
#[derive(Clone, Data, Debug, New)]
pub struct EuvInputGroup {
    /// Leading prefix addon text (e.g. `"https://"`).
    pub(crate) prefix: String,
    /// Controlled input text signal.
    #[get(type(copy))]
    pub(crate) value: Signal<String>,
    /// Placeholder text inside the input.
    pub(crate) placeholder: String,
    /// Trailing suffix badge or unit text (e.g. `".dev"`).
    pub(crate) suffix: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Rich list item row (`ui.shadcn.com/docs/components/item`).
#[derive(Clone, CustomDebug, Data, New)]
pub struct EuvItem {
    /// Leading short tag or avatar monogram.
    pub(crate) leading: String,
    /// Primary item title.
    pub(crate) title: String,
    /// Secondary muted description.
    pub(crate) description: String,
    /// Trailing metadata badge text.
    pub(crate) trailing: String,
    /// Optional click handler.
    #[new(skip)]
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_click: Option<UiEventHandler>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Compact native-styled inline selector (`ui.shadcn.com/docs/components/native-select`).
#[derive(Clone, Data, Debug, New)]
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
#[derive(Clone, Data, Debug, New)]
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
#[derive(Clone, Data, Debug, New)]
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
#[derive(Clone, Data, Debug, New)]
pub struct EuvSeparator {
    /// Optional inline label centered across the separator line.
    pub(crate) label: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Stacking rich Sonner toast deck with dismiss action (`ui.shadcn.com/docs/components/sonner`).
#[derive(Clone, Data, Debug, New)]
pub struct EuvSonner {
    /// Reactive stack of `(title, description)` toast entries.
    #[get(type(copy))]
    pub(crate) messages: Signal<Vec<(String, String)>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Direction-aware container switching horizontal alignment cleanly between LTR and RTL (`ui.shadcn.com/docs/components/direction`).
#[derive(Clone, Data, Debug, New)]
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
