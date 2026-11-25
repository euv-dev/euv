/// Box display mode determining how a node lays out its children.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Display {
    /// Flexbox container layout (default for cross-platform consistency).
    #[default]
    Flex,
    /// Vertical block flow layout.
    Block,
    /// Multi-column CSS Grid-like layout.
    Grid,
    /// Node is omitted from layout and painting.
    None,
}

/// Positioning strategy relative to parent or viewport.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Position {
    /// Normal flow participation with optional offset.
    #[default]
    Relative,
    /// Positioned relative to the nearest non-static ancestor.
    Absolute,
    /// Positioned relative to the root viewport.
    Fixed,
}

/// Main-axis direction for flex containers.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FlexDirection {
    /// Left-to-right horizontal row.
    #[default]
    Row,
    /// Top-to-bottom vertical column.
    Column,
    /// Right-to-left horizontal row.
    RowReverse,
    /// Bottom-to-top vertical column.
    ColumnReverse,
}

/// Line wrapping behavior for flex containers.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FlexWrap {
    /// Keep all children on a single line.
    #[default]
    NoWrap,
    /// Wrap overflowing children onto subsequent lines.
    Wrap,
}

/// Main-axis alignment inside a flex or grid container.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum JustifyContent {
    /// Pack items at the start of the main axis.
    #[default]
    Start,
    /// Center items along the main axis.
    Center,
    /// Pack items at the end of the main axis.
    End,
    /// Distribute space evenly between items.
    SpaceBetween,
    /// Distribute space around each item.
    SpaceAround,
    /// Distribute equal space between items and edges.
    SpaceEvenly,
}

/// Cross-axis alignment inside a flex or grid container.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AlignItems {
    /// Stretch items to fill cross-axis extent.
    #[default]
    Stretch,
    /// Align items at cross-axis start.
    Start,
    /// Center items along cross axis.
    Center,
    /// Align items at cross-axis end.
    End,
}

/// Per-item cross-axis alignment override.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AlignSelf {
    /// Inherit alignment from parent container.
    #[default]
    Auto,
    /// Stretch across the cross axis.
    Stretch,
    /// Align at cross-axis start.
    Start,
    /// Center on the cross axis.
    Center,
    /// Align at cross-axis end.
    End,
}

/// Overflow clipping and scrolling behavior.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Overflow {
    /// Content may render outside box bounds.
    #[default]
    Visible,
    /// Content outside box bounds is clipped.
    Hidden,
    /// Content outside box bounds is clipped and scrollable.
    Scroll,
}

/// Responsive layout breakpoint class unified across all 7 platforms.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Breakpoint {
    /// Compact viewport (`< 768px`, phones and narrow split-views).
    Compact,
    /// Medium viewport (`768px..1024px`, tablets / iPadOS portrait).
    Medium,
    /// Expanded viewport (`>= 1024px`, desktops and iPadOS landscape).
    #[default]
    Expanded,
}
