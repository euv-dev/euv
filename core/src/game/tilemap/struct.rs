use super::*;

/// Visual and physical definition for a single tile ID inside a `TileMap`.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct TilePaletteEntry {
    /// Tile fill color.
    #[get(type(copy))]
    pub(crate) color: Color,
    /// Optional inner bevel or accent color.
    #[get(type(copy))]
    pub(crate) border_color: Color,
    /// Whether this tile blocks movement and collision queries.
    #[get(type(copy))]
    pub(crate) solid: bool,
}

/// Grid-based 2D tilemap supporting O(visible) frustum-culled rendering and tile solidity queries.
#[derive(Clone, Data, Debug)]
pub struct TileMap {
    /// Grid column count.
    #[get(type(copy))]
    pub(crate) columns: usize,
    /// Grid row count.
    #[get(type(copy))]
    pub(crate) rows: usize,
    /// Square tile edge size in world units.
    #[get(type(copy))]
    pub(crate) tile_size: f32,
    /// Flat row-major tile identifier grid (`0` = empty/air).
    pub(crate) cells: Vec<u16>,
    /// Palette mapping tile IDs to colors and collision flags.
    pub(crate) palette: Vec<TilePaletteEntry>,
}

/// Sprite sheet slicing descriptor over an RGBA8 `ImageBuffer`.
#[derive(Clone, Data, Debug)]
pub struct SpriteSheet {
    /// Source RGBA8 sprite sheet texture.
    pub(crate) texture: ImageBuffer,
    /// Individual frame width in pixels.
    #[get(type(copy))]
    pub(crate) frame_width: u32,
    /// Individual frame height in pixels.
    #[get(type(copy))]
    pub(crate) frame_height: u32,
    /// Number of frame columns in the sheet.
    #[get(type(copy))]
    pub(crate) columns: u32,
}

/// Frame-rate-driven sprite animation clip state.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct SpriteAnimator {
    /// Starting frame index in the sprite sheet.
    #[get(type(copy))]
    pub(crate) start_frame: u32,
    /// Total frame count in this clip.
    #[get(type(copy))]
    pub(crate) frame_count: u32,
    /// Playback cadence in frames per second.
    #[get(type(copy))]
    pub(crate) fps: f32,
    /// Whether playback wraps continuously.
    #[get(type(copy))]
    pub(crate) looping: bool,
    /// Elapsed clip playback time in seconds.
    #[get(type(copy))]
    pub(crate) elapsed_secs: f32,
}
