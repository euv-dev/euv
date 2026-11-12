use super::*;

/// Props for the [`euv_drawer`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDrawerProps {
    /// The open state signal; the drawer slides in while set and clicking the
    /// overlay closes it.
    pub open: Signal<bool>,
}

/// Props for the [`euv_drawer_trigger`] component.
///
/// The trigger carries the state of the drawer so it can report the
/// state to a reader and open the drawer when it is pressed.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDrawerTriggerProps {
    /// Whether the drawer is open.
    pub open: Signal<bool>,
}

/// Props for the [`euv_drawer_content`] component.
///
/// The panel carries the state of the drawer so it can report the state
/// to a reader and hide itself while the drawer is closed.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDrawerContentProps {
    /// Whether the drawer is open.
    pub open: Signal<bool>,
}

/// Props for the [`euv_drawer_close`] component.
///
/// The close button carries the state of the drawer so it can close the
/// drawer when it is pressed.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDrawerCloseProps {
    /// Whether the drawer is open.
    pub open: Signal<bool>,
}

/// Props for the [`euv_drawer_header`] component.
///
/// The header takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDrawerHeaderProps {}

/// Props for the [`euv_drawer_footer`] component.
///
/// The footer takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDrawerFooterProps {}

/// Props for the [`euv_drawer_title`] component.
///
/// The title takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDrawerTitleProps {}

/// Props for the [`euv_drawer_description`] component.
///
/// The description takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDrawerDescriptionProps {}

/// Props for the [`euv_drawer_overlay`] component.
///
/// The overlay dims the page behind the drawer. It carries the open state,
/// so it can fade away together with the panel.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDrawerOverlayProps {
    /// Whether the drawer is open.
    pub open: Signal<bool>,
}
