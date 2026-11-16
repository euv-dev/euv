use super::*;

/// Props for the `euv_badge` component.
///
/// Defines the strongly-typed interface for the badge status indicator.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvBadgeProps {
    /// Whether to render in outline style instead of solid fill.
    #[get(type(copy))]
    pub outline: bool,
    /// The visual variant of the badge.
    #[get(type(copy))]
    pub variant: EuvBadgeVariant,
    /// The badge text content.
    pub text: ::euv::CowText,
    /// Optional click event handler.
    #[debug(skip)]
    pub on_click: Option<Rc<dyn Fn(Event)>>,
    /// An optional application class composed with the design-system badge classes.
    pub class: Css,
}
