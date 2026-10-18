use super::*;

/// One supersample position inside one pixel of a frame.
///
/// Names the pixel coordinates and sub-pixel offsets used by `pixel_direction`.
/// All four values are scalars, with no per-sample heap allocation.
#[derive(Clone, Copy, Debug, Getter)]
pub(crate) struct PixelSample {
    /// The pixel's x coordinate, counted from the left column.
    #[get(pub(crate), type(copy))]
    pub(crate) x: f64,
    /// The pixel's y coordinate, counted from the top row.
    #[get(pub(crate), type(copy))]
    pub(crate) y: f64,
    /// The sub-sample's x offset within the pixel, in `[0, 1)`.
    #[get(pub(crate), type(copy))]
    pub(crate) offset_x: f64,
    /// The sub-sample's y offset within the pixel, in `[0, 1)`.
    #[get(pub(crate), type(copy))]
    pub(crate) offset_y: f64,
}
