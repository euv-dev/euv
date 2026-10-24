use super::*;

/// Props for the [`euv_doc_layout`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDocLayoutProps {
    /// The right-column table-of-contents title (e.g. `"On this page"`).
    pub toc_title: &'static str,
    /// The right-column table-of-contents items (column hidden when empty).
    pub toc_items: &'static [EuvTocItem],
    /// The "previous page" link label (e.g. `"Previous"`).
    pub prev_label: &'static str,
    /// The "next page" link label (e.g. `"Next"`).
    pub next_label: &'static str,
    /// The previous page entry (skipped when `None`).
    pub prev: Option<EuvPaginationItem>,
    /// The next page entry (skipped when `None`).
    pub next: Option<EuvPaginationItem>,
    /// The footer text under the pagination (skipped when empty).
    pub footer: &'static str,
    /// The links rendered under the footer text (skipped when empty).
    ///
    /// A filing notice is the canonical use: the number as plain text,
    /// the official badge as an icon, and the destination pointing at the
    /// authority's own query page.
    pub footer_links: &'static [EuvFooterLink],
    /// The `href` of the TOC entry the reader is currently at.
    ///
    /// Passed through to [`euv_toc`] unchanged so a caller that drives
    /// scroll highlighting writes one signal and the whole right column
    /// follows. Empty means no entry is highlighted.
    pub toc_active: Signal<String>,
}
