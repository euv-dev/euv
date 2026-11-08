use super::*;

/// Props for the [`euv_message_scroller`] component.
///
/// The scroller takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageScrollerProps {}

/// Props for the [`euv_message_scroller_viewport`] component.
///
/// The viewport takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageScrollerViewportProps {}

/// Props for the [`euv_message_scroller_content`] component.
///
/// The content takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageScrollerContentProps {}

/// Props for the [`euv_message_scroller_item`] component.
///
/// The item says whether it is the anchor the scroller returns to when
/// the reader jumps back to the end of the list.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageScrollerItemProps {
    /// Whether the item is the anchor the scroller returns to.
    pub scroll_anchor: bool,
}

/// Props for the [`euv_message_scroller_button`] component.
///
/// The button carries the direction it jumps to.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMessageScrollerButtonProps {
    /// The direction the button jumps to.
    pub direction: EuvMessageScrollerDirection,
}
