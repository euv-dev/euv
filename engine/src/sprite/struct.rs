use super::*;

/// Defines a single frame in a sprite animation.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct SpriteFrame {
    /// The source rectangle within the sprite sheet image.
    #[get(type(copy))]
    pub(crate) source: Rect,
    /// The duration this frame should be displayed, in seconds.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) duration: f64,
}

/// Defines a sprite sheet with uniform frame grid dimensions.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct SpriteSheet {
    /// The source image element loaded from an asset.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) image: HtmlImageElement,
    /// The width of each individual frame in pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) frame_width: f64,
    /// The height of each individual frame in pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) frame_height: f64,
    /// The number of columns in the sprite sheet grid.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) columns: u32,
    /// The number of rows in the sprite sheet grid.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) rows: u32,
}

/// A named sequence of frames that form an animation.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct SpriteAnimation {
    /// The name identifying this animation (e.g., `"idle"`, `"walk"`).
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: String,
    /// The ordered list of frames in this animation.
    pub(crate) frames: Vec<SpriteFrame>,
    /// The playback mode (loop, once, ping-pong).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) mode: AnimationMode,
}

/// Manages the playback state of sprite animations.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct Animator {
    /// The currently active animation, if any.
    #[get(type(clone))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) current_animation: Option<SpriteAnimation>,
    /// The index of the current frame being displayed.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) current_frame_index: usize,
    /// The elapsed time within the current frame, in seconds.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    #[get(pub(crate))]
    pub(crate) elapsed_time: f64,
    /// The current playback state.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) state: AnimationState,
    /// The direction of playback (1 = forward, -1 = backward) for ping-pong mode.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) direction: i32,
    /// Whether to flip the sprite horizontally when rendering.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) flip_x: bool,
    /// Whether to flip the sprite vertically when rendering.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) flip_y: bool,
}

/// The nine source or destination sub-rectangles produced by a nine-slice split.
///
/// Stored as a three-by-three grid in reading order — row 0 is the top
/// (`TOP_LEFT`, `TOP`, `TOP_RIGHT`), row 1 the middle (`LEFT`, `CENTER`,
/// `RIGHT`), row 2 the bottom (`BOTTOM_LEFT`, `BOTTOM`, `BOTTOM_RIGHT`).
/// The named accessors read through those indices, so callers get
/// self-documenting access while the storage stays a single array that
/// can be iterated directly when emitting the nine `drawImage` calls.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct NineSliceRects {
    /// The nine sub-rectangles in reading order, row-major.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) grid: [[Rect; 3]; 3],
}

/// The four per-edge border insets that carve an image into nine patches.
///
/// This is the pure geometry half of a nine-slice: it holds no image
/// handle, so the split can be computed and tested without a DOM. Insets
/// are clamped so `left + right` never exceeds the source width and
/// `top + bottom` never exceeds the source height; a degenerate center
/// patch collapses to zero size rather than inverting.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct NineSliceInsets {
    /// The left border inset in source pixels.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) left: f64,
    /// The right border inset in source pixels.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) right: f64,
    /// The top border inset in source pixels.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) top: f64,
    /// The bottom border inset in source pixels.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) bottom: f64,
}

/// A source image paired with the insets that split it into nine patches.
///
/// Corners keep their natural size, the four edge patches stretch along
/// one axis only, and the center patch stretches along both — so a
/// bordered panel can be resized to any destination size without
/// distorting its corners. The geometry itself lives on
/// [`NineSliceInsets`]; this type only carries the image handle so it
/// can be stored inside a deferred `DrawCommand`.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct NineSlice {
    /// The source image element containing the nine patches.
    #[get(type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) image: HtmlImageElement,
    /// The per-edge border insets defining the split.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) insets: NineSliceInsets,
}

/// Normalized texture coordinates for one sub-rectangle of an atlas image.
///
/// Values are fractions of the atlas dimensions in the range 0.0 to 1.0,
/// with `(0.0, 0.0)` at the image's top-left corner. The V axis runs
/// downward to match the top-left origin used by `Rect`.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct UvRect {
    /// The normalized left edge (U at the rectangle's left).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) u0: f64,
    /// The normalized top edge (V at the rectangle's top).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) v0: f64,
    /// The normalized right edge (U at the rectangle's right).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) u1: f64,
    /// The normalized bottom edge (V at the rectangle's bottom).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) v1: f64,
}

/// The name-to-rectangle index of a sprite atlas, without the image handle.
///
/// Separating the index from the image keeps the lookup half of an atlas
/// free of any DOM dependency, so region bookkeeping can be exercised
/// without a live `HtmlImageElement`.
#[derive(Clone, Data, Debug, Default, New, PartialEq)]
pub struct AtlasRegions {
    /// The named source rectangles keyed by sprite name.
    #[get_mut(pub(crate))]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) regions: HashMap<String, Rect>,
}

/// A named collection of sprite regions packed into one shared image.
///
/// Packing many sprites into a single texture keeps the number of canvas
/// draw calls (and image decode stalls) low. Each entry stores a source
/// rectangle in pixels; [`SpriteAtlas::uv`] converts it to normalized
/// coordinates for GPU samplers, and [`SpriteAtlas::draw`] blits it
/// directly through the 2D canvas API.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct SpriteAtlas {
    /// The shared image element holding every packed sprite.
    #[get(type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) image: HtmlImageElement,
    /// The named source rectangles keyed by sprite name.
    #[get(pub(crate), type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) regions: AtlasRegions,
}
