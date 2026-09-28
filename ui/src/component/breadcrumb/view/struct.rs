use super::*;

/// One entry of the [`euv_breadcrumb`] trail.
///
/// Crumbs render in slice order from the root to the current page; the
/// caller is responsible for ordering them that way.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvBreadcrumbItem {
    /// The display text of this crumb.
    #[get(type(copy))]
    pub label: &'static str,
    /// The link target of this crumb (ignored for the last entry, which
    /// renders as the current page).
    #[get(type(copy))]
    pub href: &'static str,
}

/// Props for the [`euv_breadcrumb`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvBreadcrumbProps {
    /// The trail entries, ordered from the root to the current page.
    pub items: Vec<EuvBreadcrumbItem>,
    /// The separator rendered between two crumbs.
    pub separator: &'static str,
}
