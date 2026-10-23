use super::*;

/// One entry of the display list.
#[derive(Clone, Data, Debug)]
pub struct DisplayItem {
    /// The drawing instruction.
    pub cmd: DrawCmd,
    /// The paint-order key.
    ///
    /// Items are drawn in ascending `z`; items with the same `z` are drawn
    /// in the order they were added.
    #[get(type(copy))]
    pub z: i32,
    /// The identity of the element this item belongs to, used to route
    /// pointer events.
    #[get(type(copy))]
    pub element_id: u32,
    /// The series this item belongs to, if any.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub series_index: Option<usize>,
    /// The data index this item belongs to, if any.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub data_index: Option<usize>,
}

/// The ordered list of drawing instructions produced by the chart model.
///
/// The list is retained between frames: a chart that does not change does
/// not rebuild it, and the GPU backends keep their buffers in sync with
/// it rather than re-uploading every frame. This is the single biggest
/// reason the chart crate is faster than a JavaScript implementation: an
/// unchanged chart costs nothing to draw.
#[derive(Clone, Data, Debug, Default)]
pub struct DisplayList {
    /// The items, in paint order.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) items: Vec<DisplayItem>,
    /// The bounds of everything in the list.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) bounds: BBox,
}
