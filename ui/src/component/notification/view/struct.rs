use super::*;

/// One card in the [`euv_notification`] stack.
///
/// A notification carries a headline and a body line, plus an optional
/// close button. The component is purely presentational: it renders the
/// item it is given and reports the clicks, so the caller keeps
/// ownership of the queue and of any auto-dismiss timing.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNotificationItem {
    /// The notification headline, rendered in `c_euv_notification_title`.
    pub title: &'static str,
    /// The notification body, rendered in `c_euv_notification_text`.
    pub text: &'static str,
    /// The callback fired when the close button is clicked.
    #[debug(skip)]
    pub on_close: Option<Rc<dyn Fn()>>,
    /// Whether the notification renders a close button.
    ///
    /// Defaults to `false`, but supplying an `on_close` callback implies a
    /// dismissible notification, so the close button is rendered either way.
    pub dismissible: bool,
}

/// Props for the [`euv_notification`] component.
///
/// The stack is always mounted, so a caller can push notifications into
/// it without remounting the component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNotificationProps {
    /// The notifications rendered in the stack, top to bottom.
    pub items: Vec<EuvNotificationItem>,
}

/// Props of the [`euv_toast`] component.
///
/// A toast is a short-lived card: it carries a title, a description, an
/// optional action and an optional close button. The component is
/// presentational — it renders what it is given and reports the clicks,
/// so the caller keeps ownership of the queue and of the dismissal.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvToastProps {
    /// The visual variant of the toast.
    #[get(type(copy))]
    pub variant: EuvToastVariant,
    /// The headline of the toast.
    #[get(type(copy))]
    pub title: &'static str,
    /// The body line of the toast.
    #[get(type(copy))]
    pub description: &'static str,
    /// The label of the action button.
    #[get(type(copy))]
    pub action_label: &'static str,
    /// Whether the toast renders a close button.
    #[get(type(copy))]
    pub dismissible: bool,
    /// The callback fired when the action button is clicked.
    #[debug(skip)]
    pub on_action: Option<Rc<dyn Fn()>>,
    /// The callback fired when the close button is clicked.
    #[debug(skip)]
    pub on_close: Option<Rc<dyn Fn()>>,
}

/// Props of the [`euv_toast_title`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvToastTitleProps {
    /// The headline text.
    #[get(type(copy))]
    pub text: &'static str,
}

/// Props of the [`euv_toast_description`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvToastDescriptionProps {
    /// The body text.
    #[get(type(copy))]
    pub text: &'static str,
}

/// Props of the [`euv_toast_action`] component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvToastActionProps {
    /// The label of the button.
    #[get(type(copy))]
    pub label: &'static str,
    /// The callback fired when the button is clicked.
    #[debug(skip)]
    pub on_click: Option<Rc<dyn Fn()>>,
}
