use super::*;

/// Props for the [`euv_tooltip`] component.
///
/// The trigger element is supplied as children; the bubble carries `text`.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTooltipProps {
    /// The hint text rendered inside the bubble.
    pub text: &'static str,
    /// The side of the trigger the bubble is anchored to.
    pub placement: EuvTooltipPlacement,
}
