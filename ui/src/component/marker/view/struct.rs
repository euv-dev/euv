use super::*;

/// Props for the [`euv_marker`] component.
///
/// The marker carries the variant and nothing else: the icon and the
/// content beside it are passed as children.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMarkerProps {
    /// The way the marker separates the content.
    pub variant: EuvMarkerVariant,
}

/// Props for the [`euv_marker_icon`] component.
///
/// The icon is decorative and takes no settings: it stands for the
/// marker and the marker's own text says what the marker is.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMarkerIconProps {}

/// Props for the [`euv_marker_content`] component.
///
/// The content is the text of the marker and takes no settings.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMarkerContentProps {}
