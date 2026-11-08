use super::*;

/// Props for the [`euv_collapsible`] component.
///
/// The collapsible carries the state of the section and the trigger and
/// the content inside it read that same state.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCollapsibleProps {
    /// Whether the section is open.
    pub open: Signal<bool>,
}

/// Props for the [`euv_collapsible_trigger`] component.
///
/// The trigger carries the state of the section so it can report the
/// state to a reader and toggle the section when it is pressed.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCollapsibleTriggerProps {
    /// Whether the section is open.
    pub open: Signal<bool>,
}

/// Props for the [`euv_collapsible_content`] component.
///
/// The content carries the state of the section so it can hide itself
/// when the section is closed.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCollapsibleContentProps {
    /// Whether the section is open.
    pub open: Signal<bool>,
}
