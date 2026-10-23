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
