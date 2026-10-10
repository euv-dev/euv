use super::*;

/// The generated documentation site, reduced to what route resolution
/// actually reads.
///
/// `build.rs` emits the site's data as `DocsSite` / `DocsPage` /
/// `DocsLocale` (`data::struct.rs`), and each page carries its whole
/// rendered body — the block AST, its heading TOC, its sidebar. None of
/// that is needed to answer "which page does this route name", and
/// binding the answer to it would make the routing rules untestable:
/// there is no way to construct a `DocsPage` without a browser-produced
/// block AST, so a site with a deliberately colliding page pair — the
/// one shape the normalisation must refuse to merge — could never be
/// described in a test at all.
///
/// These two types are the route layer's own view of the same generated
/// data: the locale prefixes and the route strings, nothing else.
/// `router::site()` projects the generated site onto them once, and every
/// normalisation rule is expressed against this view alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteSite {
    /// Every locale the site serves, in declaration order.
    pub locales: &'static [RouteLocale],
    /// Every page route the site serves.
    pub pages: &'static [RoutePage],
}

/// One locale of a [`RouteSite`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteLocale {
    /// URL prefix (`/` or `/zh/`).
    pub prefix: &'static str,
}

/// One page route of a [`RouteSite`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoutePage {
    /// Full route as `build.rs` generated it.
    pub route: &'static str,
}
