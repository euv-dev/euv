use super::*;

/// Props for the `euv_alert` component.
///
/// Defines the strongly-typed interface for an alert with variant styling.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvAlertProps {
    /// The visual variant of the alert (success or error).
    #[get(type(copy))]
    pub variant: AlertVariant,
}

/// Props of the [`euv_alert_icon`] component.
///
/// The icon is the glyph the alert leads with.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAlertIconProps {
    /// The glyph shown inside the icon.
    pub glyph: ::euv::CowText,
}

/// Props for the `euv_alert_title` component.
///
/// The title names the alert; the text is the fallback used when the
/// title is given no children.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvAlertTitleProps {
    /// The title text.
    pub text: ::euv::CowText,
}

/// Props for the `euv_alert_description` component.
///
/// The description explains the alert; the text is the fallback used when
/// the description is given no children.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvAlertDescriptionProps {
    /// The description text.
    pub text: ::euv::CowText,
}

/// Props for the `euv_alert_action` component.
///
/// The action holds the control that acts on the alert, so it takes no
/// settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvAlertActionProps {}
