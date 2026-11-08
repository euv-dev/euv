use super::*;

/// Props for the `euv_modal` component.
///
/// Defines the strongly-typed interface for the modal dialog.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvModalProps {
    /// The modal title text.
    #[get(type(copy))]
    pub title: &'static str,
    /// Optional close handler triggered by overlay or close button click.
    #[debug(skip)]
    pub onclick: Option<Rc<dyn Fn(Event)>>,
}

/// Props of the [`euv_dialog`] component.
///
/// The dialog holds the open state, so the trigger and the panel always
/// agree on whether the dialog is showing.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDialogProps {
    /// Whether the dialog is open.
    pub open: Signal<bool>,
    /// The title of the dialog.
    pub title: &'static str,
    /// The description of the dialog.
    pub description: &'static str,
}

/// Props of the [`euv_dialog_header`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvDialogHeaderProps {
    /// The accessible name of the header.
    pub label: &'static str,
}

/// Props of the [`euv_dialog_title`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvDialogTitleProps {
    /// The title text.
    pub text: &'static str,
}

/// Props of the [`euv_dialog_description`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvDialogDescriptionProps {
    /// The description text.
    pub text: &'static str,
}

/// Props of the [`euv_dialog_footer`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvDialogFooterProps {
    /// The accessible name of the footer.
    pub label: &'static str,
}

/// Props of the [`euv_dialog_close`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvDialogCloseProps {
    /// The accessible name of the button.
    pub label: &'static str,
}
