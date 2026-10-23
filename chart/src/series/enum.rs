/// Where a stepped line puts its step.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum StepPosition {
    /// The step falls at the beginning of the interval.
    Start,
    /// The step falls in the middle of the interval.
    #[default]
    Middle,
    /// The step falls at the end of the interval.
    End,
}

/// The shape drawn for a data point.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SymbolShape {
    /// A circle.
    #[default]
    Circle,
    /// A rectangle.
    Rect,
    /// A rounded rectangle.
    RoundRect,
    /// A triangle.
    Triangle,
    /// A diamond.
    Diamond,
    /// A pin.
    Pin,
    /// An arrow.
    Arrow,
    /// No symbol is drawn.
    None,
}
