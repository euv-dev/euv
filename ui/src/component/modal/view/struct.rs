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

/// Props of the [`euv_dialog_trigger`] component.
///
/// The trigger opens the dialog, so it carries the open state the panel
/// reads.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDialogTriggerProps {
    /// Whether the dialog is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_dialog_close`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDialogCloseProps {
    /// The accessible name of the button.
    pub label: &'static str,
    /// The optional handler that closes the dialog.
    #[debug(skip)]
    pub onclick: Option<Rc<dyn Fn(Event)>>,
}

/// Props for the [`euv_dialog_overlay`] component.
///
/// The overlay dims the page behind a dialog. It carries the open state, so
/// it can fade away together with the panel.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDialogOverlayProps {
    /// Whether the dialog is open.
    pub open: Signal<bool>,
}

/// Props for the [`euv_dialog_content`] component.
///
/// The content is the panel of a dialog. It carries the open state, so the
/// panel can slide away together with the overlay.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDialogContentProps {
    /// Whether the dialog is open.
    pub open: Signal<bool>,
}
