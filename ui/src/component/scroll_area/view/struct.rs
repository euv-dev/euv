use super::*;

/// Props for the [`euv_scroll_area`] component.
///
/// The area is a frame around a scrolling viewport: the frame keeps the
/// content inside its own bounds, and the viewport is what scrolls.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvScrollAreaProps {
    /// The accessible name of the scrolling region.
    pub label: &'static str,
}
