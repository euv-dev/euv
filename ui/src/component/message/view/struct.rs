use super::*;

/// One toast in the [`euv_message`] stack.
///
/// A message is a short line of text with an optional action and an
/// optional close button. The component is purely presentational: it
/// renders the item it is given and reports the clicks, so the caller
/// keeps ownership of the queue and of any auto-dismiss timing.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageItem {
    /// The message body, rendered in `c_euv_message_text`.
    pub text: &'static str,
    /// The label of the optional action button.
    ///
    /// An empty label renders no action button at all.
    pub action_label: &'static str,
    /// The callback fired when the action button is clicked.
    #[debug(skip)]
    pub on_action: Option<Rc<dyn Fn()>>,
    /// The callback fired when the close button is clicked.
    #[debug(skip)]
    pub on_close: Option<Rc<dyn Fn()>>,
    /// Whether the message renders a close button.
    ///
    /// Defaults to `false`, but supplying an `on_close` callback implies a
    /// dismissible message, so the close button is rendered either way.
    pub dismissible: bool,
}

/// Props for the [`euv_message`] component.
///
/// The stack is always mounted, so a caller can push messages into it
/// without remounting the component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageProps {
    /// The messages rendered in the stack, top to bottom.
    pub items: Vec<EuvMessageItem>,
}

/// Props for the [`euv_message_group`] component.
///
/// The group takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageGroupProps {}

/// Props for the [`euv_message_header`] component.
///
/// The header takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageHeaderProps {}

/// Props for the [`euv_message_avatar`] component.
///
/// The avatar takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageAvatarProps {}

/// Props for the [`euv_message_content`] component.
///
/// The content takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageContentProps {}

/// Props for the [`euv_message_bubble`] component.
///
/// The bubble takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageBubbleProps {}

/// Props for the [`euv_message_footer`] component.
///
/// The footer takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageFooterProps {}
