/// Props of the [`docs_mermaid`] component.
#[derive(Clone, Default)]
pub(crate) struct DocsMermaidProps {
    /// The inline SVG markup of the diagram.
    pub(crate) svg: &'static str,
}
