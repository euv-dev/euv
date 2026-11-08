use super::*;

/// Props for the `euv_button_group` component.
///
/// A group joins the buttons it holds into one control, so the inner
/// borders between the buttons are dropped and only the outer frame
/// remains.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvButtonGroupProps {
    /// The direction the group is laid out in.
    #[get(type(copy))]
    pub orientation: EuvButtonGroupOrientation,
}

/// Props for the `euv_button_group_text` component.
///
/// The text sits between two buttons and takes their height, so a group
/// reads as one row even when it mixes a label with its actions.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvButtonGroupTextProps {
    /// The text shown inside the group.
    #[get(type(copy))]
    pub text: &'static str,
}

/// Props for the `euv_button_group_separator` component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvButtonGroupSeparatorProps {
    /// The direction the separator runs in.
    #[get(type(copy))]
    pub orientation: EuvButtonGroupSeparatorOrientation,
}
