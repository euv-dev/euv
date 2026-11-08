use super::*;

/// The surface a bubble is drawn on.
///
/// The reference publishes seven: the plain bubble, the quiet one, the
/// muted one, the tinted one, the outlined one, the bare one and the
/// destructive one.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvBubbleVariant {
    /// The bubble sits on the page surface.
    #[default]
    Default,
    /// The bubble sits on the secondary surface.
    Secondary,
    /// The bubble sits on the muted surface.
    Muted,
    /// The bubble is tinted with the brand colour.
    Tinted,
    /// The bubble carries an outline and no fill.
    Outline,
    /// The bubble carries no fill and no outline.
    Ghost,
    /// The bubble is drawn in the destructive colour.
    Destructive,
}

/// The side of the row a bubble sits on.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvBubbleAlign {
    /// The bubble sits at the start of the row.
    #[default]
    Start,
    /// The bubble sits at the end of the row.
    End,
}

/// The side of a bubble its reactions sit on.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvBubbleSide {
    /// The reactions sit above the bubble.
    Top,
    /// The reactions sit below the bubble.
    #[default]
    Bottom,
}
