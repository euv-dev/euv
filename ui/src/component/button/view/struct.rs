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
    /// An optional semantic slot name, defaulting to `button`.
    pub data_slot: Option<&'static str>,
    /// Optional click event handler.
    #[debug(skip)]
    pub onclick: Option<Rc<dyn Fn(Event)>>,
    /// Whether the button is disabled.
    pub disabled: Signal<bool>,
    /// Whether the button submits the form it stands in.
    ///
    /// A button inside a form submits it by default, which is almost never
    /// what a control that only performs an action wants. The button is
    /// therefore inert unless the caller asks for the submission.
    pub submit: bool,
    /// The accessible name of the button.
    ///
    /// The name is the label the button shows unless the caller names one
    /// of its own, which a button that carries a glyph rather than a word
    /// needs.
    pub aria_label: Option<String>,
    /// Whether the button is busy.
    ///
    /// A busy button is one whose work has not finished yet. The state is
    /// announced to the reader, so a control that is working says so.
    pub busy: Signal<bool>,
    /// A stable identifier for application integration tests.
    pub data_testid: &'static str,
}

/// Props for the `euv_button_grouped` component.
///
/// A joined button is a button that sits inside a group of buttons, so
/// it carries the same settings a plain button carries.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvButtonGroupedProps {
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

/// Props for the [`euv_button_link`] component.
///
/// A link that looks like a button: the anchor carries the appearance of
/// the button, because a destination the reader follows is not an action
/// the reader performs and the two want different affordances.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvButtonLinkProps {
    /// The visual variant determining colour scheme.
    pub variant: EuvButtonVariant,
    /// The size of the control.
    pub size: EuvButtonSize,
    /// The label of the link.
    pub label: ::euv::CowText,
    /// The destination the link points at.
    pub href: ::euv::CowText,
    /// Whether the link opens in a new tab.
    #[get(type(copy))]
    pub new_tab: bool,
    /// The accessible name of the link.
    ///
    /// The name is the label the link shows unless the caller names one of
    /// its own, which a link that carries a glyph rather than a word needs.
    pub aria_label: Option<::euv::CowText>,
}
