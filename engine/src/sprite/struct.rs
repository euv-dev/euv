use super::*;

/// Defines a single frame in a sprite animation.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct SpriteFrame {
    /// The source rectangle within the sprite sheet image.
    #[get(type(copy))]
    pub(crate) source: Rect,
    /// The duration this frame should be displayed, in seconds.
    #[get(type(copy))]
    pub(crate) duration: f64,
}

/// Defines a sprite sheet with uniform frame grid dimensions.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct SpriteSheet {
    /// The source image element loaded from an asset.
    pub(crate) image: HtmlImageElement,
    /// The width of each individual frame in pixels.
    #[get(type(copy))]
    pub(crate) frame_width: f64,
    /// The height of each individual frame in pixels.
    #[get(type(copy))]
    pub(crate) frame_height: f64,
    /// The number of columns in the sprite sheet grid.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) columns: u32,
    /// The number of rows in the sprite sheet grid.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) rows: u32,
}

/// A named sequence of frames that form an animation.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct SpriteAnimation {
    /// The name identifying this animation (e.g., `"idle"`, `"walk"`).
    pub(crate) name: String,
    /// The ordered list of frames in this animation.
    pub(crate) frames: Vec<SpriteFrame>,
    /// The playback mode (loop, once, ping-pong).
    #[get(type(copy))]
    pub(crate) mode: AnimationMode,
}

/// Manages the playback state of sprite animations.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct Animator {
    /// The currently active animation, if any.
    #[get(type(clone))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) current_animation: Option<SpriteAnimation>,
    /// The index of the current frame being displayed.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) current_frame_index: usize,
    /// The elapsed time within the current frame, in seconds.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) elapsed_time: f64,
    /// The current playback state.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) state: AnimationState,
    /// The direction of playback (1 = forward, -1 = backward) for ping-pong mode.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) direction: i32,
    /// Whether to flip the sprite horizontally when rendering.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) flip_x: bool,
    /// Whether to flip the sprite vertically when rendering.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) flip_y: bool,
}
