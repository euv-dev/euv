use super::*;

/// Props for the [`euv_popover`] component.
///
/// The open state is owned by the caller so its own trigger (rendered outside
/// the popover) can toggle it through [`on_popover_toggle`]; the panel body
/// is built from the children.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvPopoverProps {
    /// The open state signal driving the panel's reactive class.
    pub open: Signal<bool>,
    /// The panel title rendered in the header row.
    pub title: &'static str,
    /// The side of the trigger the panel is anchored to.
    pub placement: EuvTooltipPlacement,
}
