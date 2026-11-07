/// Props of the [`docs_code_block`] component.
#[derive(Clone, Default)]
pub(crate) struct DocsCodeBlockProps {
    /// The fence language of the block, shown in the header.
    pub(crate) lang: &'static str,
    /// The raw code of the block.
    pub(crate) code: &'static str,
}
