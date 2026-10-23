use super::*;

/// Props for the [`euv_link`] component.
///
/// Defines the strongly-typed interface for a link: the text, the
/// destination it points at, and whether it is currently disabled.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvLinkProps {
    /// The link text.
    pub text: &'static str,
    /// The destination the link points at.
    ///
    /// An empty string renders the link as a plain span with no
    /// destination — the shape a disabled or in-page action takes.
    pub href: &'static str,
    /// Whether the link is disabled.
    pub disabled: bool,
    /// The callback fired when the link is taken.
    #[debug(skip)]
    pub on_click: Option<Rc<dyn Fn(Event)>>,
}
