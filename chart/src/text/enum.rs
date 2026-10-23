/// The alignment of a text label, as ECharts names it.
///
/// The option layer uses ECharts' own vocabulary so that an existing
/// configuration translates field by field; the paint layer uses the
/// chart crate's own enums.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LabelAlign {
    /// The label starts at its anchor.
    #[default]
    Left,
    /// The label is centred on its anchor.
    Center,
    /// The label ends at its anchor.
    Right,
}

/// The vertical alignment of a text label, as ECharts names it.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LabelVerticalAlign {
    /// The label's top edge sits on its anchor.
    Top,
    /// The label is centred on its anchor.
    #[default]
    Middle,
    /// The label's bottom edge sits on its anchor.
    Bottom,
}
