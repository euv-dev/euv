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

/// Props for the [`euv_breadcrumb_list`] component.
///
/// The list holds the crumbs of the trail, so it takes no settings of
/// its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvBreadcrumbListProps {}

/// Props for the [`euv_breadcrumb_item`] component.
///
/// The item holds one crumb, so it takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvBreadcrumbItemProps {}

/// Props for the [`euv_breadcrumb_link`] component.
///
/// The link is an ancestor crumb: it carries the target it opens and the
/// text it shows.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvBreadcrumbLinkProps {
    /// The text shown inside the link.
    pub label: &'static str,
    /// The link target.
    pub href: &'static str,
}

/// Props for the [`euv_breadcrumb_page`] component.
///
/// The page is the current crumb: it names where the reader is, so it
/// carries the text it shows.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvBreadcrumbPageProps {
    /// The text shown inside the page crumb.
    pub label: &'static str,
}

/// Props for the [`euv_breadcrumb_separator`] component.
///
/// The separator divides two crumbs, so it carries the glyph it draws.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvBreadcrumbSeparatorProps {
    /// The separator text.
    pub text: &'static str,
}

/// Props for the [`euv_breadcrumb_ellipsis`] component.
///
/// The ellipsis stands in for the crumbs a collapsed trail hides, so it
/// takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvBreadcrumbEllipsisProps {}
