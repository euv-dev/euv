use super::*;

/// Props of the [`docs_site_footer`] component.
#[derive(Clone, Default)]
pub(crate) struct DocsSiteFooterProps {
    /// The current route signal.
    pub(crate) route_signal: Signal<String>,
}
