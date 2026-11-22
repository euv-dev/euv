/// Props of the [`docs_mermaid`] component.
#[derive(Clone, Default)]
pub(crate) struct DocsMermaidProps {
    /// The mermaid source of the fence.
    pub(crate) source: &'static str,
    /// The ordinal of the diagram on the page.
    ///
    /// The ordinal names the diagram's scoped ids and CSS selectors, so
    /// two diagrams on one page never share a marker id.
    pub(crate) ordinal: usize,
}
