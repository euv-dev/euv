/// Chart visualization kinds supported by `EuvChart`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvChartKind {
    /// Vertical bar chart with theme primary/accent series bars.
    #[default]
    Bar,
    /// Connected line plot with circular data vertices.
    Line,
    /// Filled translucent area chart with crisp top stroke.
    Area,
}

/// Text reading direction mode for `EuvDirection`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EuvTextDirection {
    /// Left-to-right layout flow.
    #[default]
    Ltr,
    /// Right-to-left layout flow.
    Rtl,
}
