use super::*;

/// Props for the `euv_button` component.
///
/// Defines the strongly-typed interface for a versatile button.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvButtonProps {
    /// The visual variant determining colour scheme.
    pub variant: EuvButtonVariant,
    /// The size of the control.
    pub size: EuvButtonSize,
    /// The button label text, used as fallback when no children are provided.
    pub label: &'static str,
    /// Optional click event handler.
    #[debug(skip)]
    pub onclick: Option<Rc<dyn Fn(Event)>>,
    /// Whether the button is disabled.
    pub disabled: Signal<bool>,
}
