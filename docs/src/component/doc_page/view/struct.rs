use super::*;

/// Props of the [`docs_main`] and [`docs_doc_page`] components.
#[derive(Clone, Default)]
pub(crate) struct DocsPageProps {
    /// The current route signal.
    pub(crate) route_signal: Signal<String>,
}
