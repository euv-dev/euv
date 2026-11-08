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
    /// The title text.
    pub text: &'static str,
}

/// Props of the [`euv_alert_dialog_description`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvAlertDialogDescriptionProps {
    /// The description text.
    pub text: &'static str,
}

/// Props of the [`euv_alert_dialog_action`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvAlertDialogActionProps {
    /// The label of the button.
    pub label: &'static str,
}

/// Props of the [`euv_alert_dialog_cancel`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvAlertDialogCancelProps {
    /// The label of the button.
    pub label: &'static str,
}
