use super::*;

/// One side of the [`euv_pagination`] prev/next footer.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvPaginationItem {
    /// The display text.
    #[get(type(copy))]
    pub text: &'static str,
    /// The link target (hash route path).
    #[get(type(copy))]
    pub link: &'static str,
}

/// Props for the [`euv_pagination`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvPaginationProps {
    /// The label above the previous entry (for example `Previous`).
    #[get(type(copy))]
    pub prev_label: &'static str,
    /// The label above the next entry (for example `Next`).
    #[get(type(copy))]
    pub next_label: &'static str,
    /// The previous entry; `None` renders a spacer.
    #[get(type(copy))]
    pub prev: Option<EuvPaginationItem>,
    /// The next entry; `None` renders a spacer.
    #[get(type(copy))]
    pub next: Option<EuvPaginationItem>,
}

/// Props for the [`euv_pagination_item`] component.
///
/// The item holds one reachable page of the footer: the small label above
/// the entry, the text of the entry and the route it points at.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvPaginationItemProps {
    /// The small uppercase label above the entry.
    #[get(type(copy))]
    pub label: &'static str,
    /// The text of the entry.
    #[get(type(copy))]
    pub text: &'static str,
    /// The route the entry points at.
    #[get(type(copy))]
    pub link: &'static str,
    /// Whether the entry is the right-aligned next side.
    #[get(type(copy))]
    pub next: bool,
}

/// Props for the [`euv_pagination_content`] component.
///
/// The content takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvPaginationContentProps {}

/// Props for the [`euv_pagination_ellipsis`] component.
///
/// The ellipsis takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvPaginationEllipsisProps {}

/// Props for the [`euv_pagination_link`] component.
///
/// The link is one page of a page list. It reports whether it is the page
/// the reader is on, so the current page can be told apart from the rest.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvPaginationLinkProps {
    /// The label of the page link.
    pub label: &'static str,
    /// The route the link points at.
    pub link: &'static str,
    /// Whether the link is the page the reader is on.
    pub active: bool,
}
