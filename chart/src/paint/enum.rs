use super::*;

/// The alignment of a text label.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TextAlign {
    /// The text starts at the anchor point and runs to the right.
    #[default]
    Left,
    /// The text is centred on the anchor point.
    Center,
    /// The text ends at the anchor point and runs to the left.
    Right,
}

/// The vertical alignment of a text label.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TextVerticalAlign {
    /// The text's top edge sits on the anchor point.
    Top,
    /// The text is vertically centred on the anchor point.
    #[default]
    Middle,
    /// The text's bottom edge sits on the anchor point.
    Bottom,
    /// The text's baseline sits on the anchor point.
    Baseline,
}

/// What happens to text that does not fit inside its box.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TextOverflow {
    /// The text is truncated with an ellipsis.
    #[default]
    Truncate,
    /// The text is hidden entirely.
    None,
    /// The text overflows its box without being cut.
    Break,
}

/// How a shape is filled.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Fill {
    /// The shape is filled with a single colour.
    Solid(Color),
    /// The shape is filled with a linear gradient.
    Linear(LinearGradient),
    /// The shape is filled with a radial gradient.
    Radial(RadialGradient),
    /// The shape is not filled.
    #[default]
    None,
}

/// How a shape is stroked.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Stroke {
    /// The shape is stroked with a single colour.
    Solid(Color),
    /// The shape is stroked with a linear gradient.
    Linear(LinearGradient),
    /// The shape is stroked with a radial gradient.
    Radial(RadialGradient),
    /// The shape is not stroked.
    #[default]
    None,
}

/// The dash pattern of a line, expressed in ECharts' `lineStyle.type`
/// vocabulary.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LineDashType {
    /// A solid line.
    #[default]
    Solid,
    /// Dashed.
    Dashed,
    /// Dotted.
    Dotted,
}
