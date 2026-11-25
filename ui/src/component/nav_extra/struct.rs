use super::*;

/// Horizontal breadcrumb trail separated by `/`.
#[derive(Clone, Data, New)]
pub struct EuvBreadcrumb {
    /// Ordered trail labels.
    pub(crate) segments: Vec<String>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Interactive page navigation bar bound to `Signal<usize>`.
#[derive(Clone, Data, New)]
pub struct EuvPagination {
    /// Current 1-based page number signal.
    #[get(type(copy))]
    pub(crate) current: Signal<usize>,
    /// Total number of pages.
    #[get(type(copy))]
    pub(crate) total_pages: usize,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Multi-step progress indicator bound to `Signal<usize>`.
#[derive(Clone, Data, New)]
pub struct EuvSteps {
    /// Step titles.
    pub(crate) labels: Vec<String>,
    /// Current 0-based step index signal.
    #[get(type(copy))]
    pub(crate) active: Signal<usize>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Single collapsible item inside `EuvAccordion`.
#[derive(Clone, Debug, Getter, New, PartialEq, Eq)]
pub struct EuvAccordionItem {
    /// Collapsible section title.
    pub(crate) title: String,
    /// Collapsible body text.
    pub(crate) content: String,
}

/// Reactive accordion expanding one active section at a time.
#[derive(Clone, Data, New)]
pub struct EuvAccordion {
    /// Accordion entries.
    pub(crate) items: Vec<EuvAccordionItem>,
    /// Currently expanded index (`None` when all collapsed).
    #[get(type(copy))]
    pub(crate) open_index: Signal<Option<usize>>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Table-of-contents outline panel with active anchor selection.
#[derive(Clone, Data, New)]
pub struct EuvToc {
    /// Ordered section heading labels.
    pub(crate) headings: Vec<String>,
    /// Active heading index signal.
    #[get(type(copy))]
    pub(crate) active: Signal<usize>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
