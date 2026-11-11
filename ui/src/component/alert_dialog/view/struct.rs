use super::*;

/// Props of the [`euv_alert_dialog`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAlertDialogProps {
    /// Whether the dialog is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_alert_dialog_trigger`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAlertDialogTriggerProps {
    /// Whether the dialog is open.
    pub open: Signal<bool>,
    /// The id of the content controlled by this trigger.
    pub content_id: &'static str,
    /// The label shown inside the trigger.
    pub label: &'static str,
}

/// Props of the [`euv_alert_dialog_overlay`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAlertDialogOverlayProps {
    /// Whether the dialog is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_alert_dialog_content`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAlertDialogContentProps {
    /// Whether the dialog is open.
    pub open: Signal<bool>,
    /// The id referenced by the trigger's `aria-controls`.
    pub id: &'static str,
    /// The id of the title that labels the dialog.
    pub title_id: &'static str,
    /// The id of the description that explains the dialog.
    pub description_id: &'static str,
}

/// Props of the [`euv_alert_dialog_media`] component.
///
/// The media is the glyph the dialog leads with.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvAlertDialogMediaProps {
    /// The glyph shown inside the media.
    pub glyph: &'static str,
}

/// Props of the [`euv_alert_dialog_header`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvAlertDialogHeaderProps {
    /// The accessible name of the header.
    pub label: &'static str,
}

/// Props of the [`euv_alert_dialog_footer`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvAlertDialogFooterProps {
    /// The accessible name of the footer.
    pub label: &'static str,
}

/// Props of the [`euv_alert_dialog_title`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvAlertDialogTitleProps {
    /// The id referenced by the dialog content's accessible name.
    pub id: &'static str,
    /// The title text.
    pub text: &'static str,
}

/// Props of the [`euv_alert_dialog_description`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvAlertDialogDescriptionProps {
    /// The id referenced by the dialog content's accessible description.
    pub id: &'static str,
    /// The description text.
    pub text: &'static str,
}

/// Props of the [`euv_alert_dialog_action`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAlertDialogActionProps {
    /// The label of the button.
    pub label: &'static str,
    /// The visual button variant, defaulting to the primary treatment.
    pub variant: Option<EuvButtonVariant>,
    /// The shared open state that the action closes.
    pub open: Signal<bool>,
    /// Optional action callback invoked before the dialog closes.
    #[debug(skip)]
    pub onclick: Option<Rc<dyn Fn(Event)>>,
}

/// Props of the [`euv_alert_dialog_cancel`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAlertDialogCancelProps {
    /// The label of the button.
    pub label: &'static str,
    /// The shared open state that the cancel button closes.
    pub open: Signal<bool>,
    /// Optional cancel callback invoked before the dialog closes.
    #[debug(skip)]
    pub onclick: Option<Rc<dyn Fn(Event)>>,
}
