use super::*;

/// Layout-affecting properties separated from paint properties for surgical dirty-flag tracking.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct LayoutStyle {
    /// Display mode (`Flex`, `Block`, `Grid`, `None`).
    #[get(type(copy))]
    pub(crate) display: Display,
    /// Positioning strategy (`Relative`, `Absolute`, `Fixed`).
    #[get(type(copy))]
    pub(crate) position: Position,
    /// Flex main-axis direction.
    #[get(type(copy))]
    pub(crate) flex_direction: FlexDirection,
    /// Flex line wrapping mode.
    #[get(type(copy))]
    pub(crate) flex_wrap: FlexWrap,
    /// Main-axis justification.
    #[get(type(copy))]
    pub(crate) justify_content: JustifyContent,
    /// Cross-axis alignment for children.
    #[get(type(copy))]
    pub(crate) align_items: AlignItems,
    /// Cross-axis alignment override for this node.
    #[get(type(copy))]
    pub(crate) align_self: AlignSelf,
    /// Overflow clipping/scrolling behavior.
    #[get(type(copy))]
    pub(crate) overflow: Overflow,
    /// Flex grow factor (`>= 0.0`).
    #[get(type(copy))]
    pub(crate) flex_grow: f32,
    /// Flex shrink factor (`>= 0.0`).
    #[get(type(copy))]
    pub(crate) flex_shrink: f32,
    /// Flex basis length.
    #[get(type(copy))]
    pub(crate) flex_basis: Length,
    /// Explicit grid column count when `display == Display::Grid`.
    #[get(type(copy))]
    pub(crate) grid_columns: u16,
    /// Width specification.
    #[get(type(copy))]
    pub(crate) width: Length,
    /// Height specification.
    #[get(type(copy))]
    pub(crate) height: Length,
    /// Minimum width constraint.
    #[get(type(copy))]
    pub(crate) min_width: Length,
    /// Minimum height constraint.
    #[get(type(copy))]
    pub(crate) min_height: Length,
    /// Maximum width constraint.
    #[get(type(copy))]
    pub(crate) max_width: Length,
    /// Maximum height constraint.
    #[get(type(copy))]
    pub(crate) max_height: Length,
    /// Outer margin insets.
    #[get(type(copy))]
    pub(crate) margin: Edges,
    /// Inner padding insets.
    #[get(type(copy))]
    pub(crate) padding: Edges,
    /// Row gap between children in logical pixels.
    #[get(type(copy))]
    pub(crate) row_gap: f32,
    /// Column gap between children in logical pixels.
    #[get(type(copy))]
    pub(crate) column_gap: f32,
    /// Optional explicit top offset for positioned elements.
    #[get(skip)]
    pub(crate) top: Option<f32>,
    /// Optional explicit right offset for positioned elements.
    #[get(skip)]
    pub(crate) right: Option<f32>,
    /// Optional explicit bottom offset for positioned elements.
    #[get(skip)]
    pub(crate) bottom: Option<f32>,
    /// Optional explicit left offset for positioned elements.
    #[get(skip)]
    pub(crate) left: Option<f32>,
    /// Stacking order for painting and hit-testing.
    #[get(type(copy))]
    pub(crate) z_index: i32,
    /// Optional fixed aspect ratio (`width / height`).
    #[get(skip)]
    pub(crate) aspect_ratio: Option<f32>,
}
