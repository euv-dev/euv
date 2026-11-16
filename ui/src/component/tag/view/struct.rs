use super::*;

/// Props for the `euv_tag` component.
///
/// Defines the strongly-typed interface for a status tag indicator.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvTagProps {
    /// The semantic colour type.
    #[get(type(copy))]
    pub color: EuvTagColor,
    /// The visual variant (solid or outline).
    #[get(type(copy))]
    pub variant: EuvTagVariant,
    /// The tag text content.
    pub text: ::euv::CowText,
    /// Optional click event handler.
    #[debug(skip)]
    pub on_click: Option<Rc<dyn Fn(Event)>>,
}
