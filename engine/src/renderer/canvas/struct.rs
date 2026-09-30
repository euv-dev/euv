use super::*;

/// A 2D camera that defines the viewport into the game world.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Camera2D {
    /// The world-space position of the camera center.
    #[get(type(copy))]
    pub(crate) position: Vector2D,
    /// The zoom factor (1.0 = no zoom, 2.0 = 2x magnification).
    #[get(type(copy))]
    pub(crate) zoom: f64,
    /// The rotation angle in radians.
    #[get(type(copy))]
    pub(crate) rotation: f64,
    /// The viewport width in screen pixels.
    #[get(type(copy))]
    pub(crate) viewport_width: f64,
    /// The viewport height in screen pixels.
    #[get(type(copy))]
    pub(crate) viewport_height: f64,
}

/// A 3D camera that defines the viewport into a 3D world using perspective
/// or orthographic projection.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Camera3D {
    /// The world-space position of the camera (eye).
    #[get(type(copy))]
    pub(crate) position: Vector3D,
    /// The point the camera is looking at (target).
    #[get(type(copy))]
    pub(crate) target: Vector3D,
    /// The up direction for the camera.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) up: Vector3D,
    /// The vertical field of view in radians.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) fov: f64,
    /// The near clipping plane distance.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) near: f64,
    /// The far clipping plane distance.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) far: f64,
    /// The viewport width in pixels.
    #[get(type(copy))]
    pub(crate) viewport_width: f64,
    /// The viewport height in pixels.
    #[get(type(copy))]
    pub(crate) viewport_height: f64,
}

/// A wrapper around `CanvasRenderingContext2d` providing convenience
/// drawing methods and camera management for the game engine.
#[derive(Clone, Data, New)]
pub struct CanvasRenderer {
    /// The underlying canvas 2D rendering context.
    pub(crate) context: CanvasRenderingContext2d,
    /// The active camera controlling the viewport.
    #[get(type(copy))]
    pub(crate) camera: Camera2D,
    /// The active rendering quality preset.
    ///
    /// Controls `imageSmoothingEnabled`, `imageSmoothingQuality`, and
    /// `textRendering` on the underlying context. Defaults to `Medium`.
    #[get(type(copy))]
    pub(crate) quality: RenderQuality,
}

/// A linear gradient defined by two endpoints and a list of color stops.
///
/// Used to create smooth color transitions along a straight line
/// for fill or stroke operations on the canvas.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct LinearGradient {
    /// The starting point of the gradient in world space.
    #[get(type(copy))]
    pub(crate) start: Vector2D,
    /// The ending point of the gradient in world space.
    #[get(type(copy))]
    pub(crate) end: Vector2D,
    /// The ordered list of color stops, each containing a position (0.0 to 1.0) and a CSS color string.
    pub(crate) stops: Vec<(f64, String)>,
}

/// A radial gradient defined by inner and outer circles and a list of color stops.
///
/// Used to create smooth color transitions radiating outward from a center point
/// for fill or stroke operations on the canvas.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct RadialGradient {
    /// The center of the inner circle of the gradient.
    #[get(type(copy))]
    pub(crate) inner_center: Vector2D,
    /// The radius of the inner circle.
    #[get(type(copy))]
    pub(crate) inner_radius: f64,
    /// The center of the outer circle of the gradient.
    #[get(type(copy))]
    pub(crate) outer_center: Vector2D,
    /// The radius of the outer circle.
    #[get(type(copy))]
    pub(crate) outer_radius: f64,
    /// The ordered list of color stops, each containing a position (0.0 to 1.0) and a CSS color string.
    pub(crate) stops: Vec<(f64, String)>,
}

/// Shadow rendering configuration for drop shadow effects on canvas primitives.
///
/// When applied, all subsequent fill, stroke, and draw operations will cast
/// a shadow with the specified color, blur radius, and offset.
#[derive(Clone, Data, Debug, New, PartialEq, PartialOrd)]
pub struct ShadowConfig {
    /// The CSS color string of the shadow (e.g., `"rgba(0,0,0,0.5)"`).
    #[get(type(clone))]
    pub(crate) color: String,
    /// The blur radius of the shadow in pixels.
    #[get(type(copy))]
    pub(crate) blur: f64,
    /// The horizontal offset of the shadow in pixels.
    #[get(type(copy))]
    pub(crate) offset_x: f64,
    /// The vertical offset of the shadow in pixels.
    #[get(type(copy))]
    pub(crate) offset_y: f64,
}

/// Represents the rendering priority layer for draw call ordering.
///
/// Higher z-index values are drawn on top of lower values,
/// enabling correct visual layering of game objects.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct RenderLayer {
    /// The z-index determining draw order. Higher values draw later (on top).
    #[get(type(copy))]
    pub(crate) z_index: i32,
    /// Whether objects in this layer should be rendered.
    #[get(type(copy))]
    pub(crate) visible: bool,
}

/// An ordered buffer of deferred draw commands recorded during a frame.
///
/// Scenes and components push `DrawCommand`s into the list during `on_render`
/// instead of drawing immediately. The engine then replays the whole list once
/// per frame via `CanvasRenderer::replay`, which batches consecutive same-style
/// shapes into a single path and skips redundant canvas state changes. The
/// backing `Vec` is reused across frames via `clear()` to avoid reallocation.
#[derive(Clone, Data, Debug, Default, New)]
pub struct DrawList {
    /// The recorded draw commands for the current frame.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commands: Vec<DrawCommand>,
}

/// A supersampling anti-aliasing (SSAA) canvas wrapper that renders at a higher
/// resolution on an offscreen canvas and downscales to the display canvas for
/// smoother polygon edges in software-rendered 3D scenes.
///
/// The offscreen context is scaled by `scale_factor` so that all drawing
/// code can use logical pixel coordinates without modification. After
/// rendering, call `present()` to draw the high-resolution buffer onto the
/// visible canvas with high-quality image smoothing.
#[derive(Clone, Data, New)]
pub struct SsaaCanvas {
    /// The display canvas element visible to the user.
    pub(crate) display_canvas: HtmlCanvasElement,
    /// The 2D rendering context of the display canvas used for final presentation.
    pub(crate) display_context: CanvasRenderingContext2d,
    /// The offscreen canvas used for high-resolution rendering.
    pub(crate) offscreen_canvas: HtmlCanvasElement,
    /// The 2D rendering context of the offscreen canvas, pre-scaled by `scale_factor`.
    pub(crate) offscreen_context: CanvasRenderingContext2d,
    /// The supersampling scale factor (e.g., 2.0 means 4x SSAA).
    #[get(type(copy))]
    pub(crate) scale_factor: f64,
    /// The rendering quality preset for the downscaling present step.
    ///
    /// Controls the smoothing strategy when the offscreen buffer is
    /// downscaled onto the display canvas. Defaults to `Medium`.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) quality: RenderQuality,
    /// The logical display width in CSS pixels.
    #[get(type(copy))]
    pub(crate) width: f64,
    /// The logical display height in CSS pixels.
    #[get(type(copy))]
    pub(crate) height: f64,
}
