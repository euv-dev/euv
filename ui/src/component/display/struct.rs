use super::*;

/// Compact status pill badge component.
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvBadge {
    /// Badge text label.
    pub(crate) label: String,
    /// Semantic tone.
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) tone: EuvTone,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Semantic alert banner displaying title and message.
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvAlert {
    /// Alert heading.
    pub(crate) title: String,
    /// Supporting description.
    pub(crate) description: String,
    /// Semantic tone.
    #[get(type(copy))]
    pub(crate) tone: EuvTone,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Linear progress bar displaying normalized completion in `[0.0, 1.0]`.
#[derive(Clone, Copy, Data, Debug, New, Props)]
pub struct EuvProgress {
    /// Progress fraction clamped to `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) value: f32,
    /// Bar height in logical pixels (`0.0` uses default `8.0`).
    #[new(skip)]
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) height: f32,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// High-performance reactive virtualized scrollable list rendering only the visible window of `total_items`.
#[derive(Clone, CustomDebug, Data, New)]
pub struct EuvVirtualList {
    /// Total number of logical rows.
    #[get(type(copy))]
    pub(crate) total_items: usize,
    /// Fixed height of each row in logical pixels.
    #[get(type(copy))]
    pub(crate) item_height: f32,
    /// Viewport height in logical pixels (`0.0` defaults to `320.0`).
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) viewport_height: f32,
    /// Reactive scroll Y offset signal in logical pixels.
    #[get(type(copy))]
    pub(crate) scroll_y: Signal<f32>,
    /// Extra overscan rows mounted above and below the viewport (`0` defaults to `3`).
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) overscan: usize,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Simple structured cross-platform data table.
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvTable {
    /// Ordered column header labels.
    pub(crate) headers: Vec<String>,
    /// Matrix of row cell strings.
    pub(crate) rows: Vec<Vec<String>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
