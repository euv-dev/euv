use super::*;

/// Props for the [`euv_bubble`] component.
///
/// The bubble carries the surface it is drawn on and the side of the row
/// it sits on. The content and the reactions inside it are passed as
/// children.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvBubbleProps {
    /// The surface the bubble is drawn on.
    pub variant: EuvBubbleVariant,
    /// The side of the row the bubble sits on.
    pub align: EuvBubbleAlign,
}

/// Props for the [`euv_bubble_group`] component.
///
/// The group takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvBubbleGroupProps {}

/// Props for the [`euv_bubble_content`] component.
///
/// The content takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvBubbleContentProps {}

/// Props for the [`euv_bubble_reactions`] component.
///
/// The reactions carry the side of the bubble they sit on and the side
/// of the row they line up with.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvBubbleReactionsProps {
    /// The side of the bubble the reactions sit on.
    pub side: EuvBubbleSide,
    /// The side of the row the reactions line up with.
    pub align: EuvBubbleAlign,
}
