use super::*;

#[derive(Clone)]
pub(super) struct HoverDefinition {
    pub(super) name: String,
    pub(super) origin: String,
    pub(super) body: TokenStream,
}
