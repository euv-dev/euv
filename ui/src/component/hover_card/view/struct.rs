use super::*;

/// Props of the [`euv_hover_card`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvHoverCardProps {
    /// Whether the card is showing.
    pub open: Signal<bool>,
}

/// Props of the [`euv_hover_card_trigger`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvHoverCardTriggerProps {
    /// Whether the card is showing.
    pub open: Signal<bool>,
    /// The label shown inside the trigger.
    pub label: &'static str,
}

/// Props of the [`euv_hover_card_content`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvHoverCardContentProps {
    /// Whether the card is showing.
    pub open: Signal<bool>,
}
