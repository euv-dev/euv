/// The CSS selector used to query the RayTrace Canvas 2D canvas element
/// from the DOM.
pub(crate) const RAYTRACE_CANVAS_SELECTOR: &str = "#raytrace-canvas";

/// The CSS selector for the RayTrace Canvas 2D loading overlay canvas.
pub(crate) const RAYTRACE_LOADING_CANVAS_SELECTOR: &str = "#raytrace-loading-canvas";

/// The CSS selector used to query the RayTrace WebGL canvas element.
pub(crate) const RAYTRACE_WEBGL_CANVAS_SELECTOR: &str = "#raytrace-webgl-canvas";

/// The CSS selector for the RayTrace WebGL loading overlay canvas.
pub(crate) const RAYTRACE_WEBGL_LOADING_CANVAS_SELECTOR: &str = "#raytrace-webgl-loading-canvas";

/// The CSS selector used to query the RayTrace WebGPU canvas element.
pub(crate) const RAYTRACE_WEBGPU_CANVAS_SELECTOR: &str = "#raytrace-webgpu-canvas";

/// The CSS selector for the RayTrace WebGPU loading overlay canvas.
pub(crate) const RAYTRACE_WEBGPU_LOADING_CANVAS_SELECTOR: &str = "#raytrace-webgpu-loading-canvas";

/// The stable error code recorded when
/// [`RayTraceRenderer::init`](euv_engine::RayTraceRenderer) rejects the
/// requested backend.
///
/// Distinct from [`RAYTRACE_ERROR_CODE_RENDER`] so the status banner can
/// tell "this browser cannot host the backend at all" apart from "the
/// backend was acquired and then failed mid-run".
pub(crate) const RAYTRACE_ERROR_CODE_INIT: &str = "RAYTRACE_INIT";

/// The stable error code recorded when
/// [`RayTraceRenderer::render`](euv_engine::RayTraceRenderer) rejects a
/// frame.
pub(crate) const RAYTRACE_ERROR_CODE_RENDER: &str = "RAYTRACE_RENDER";

/// The stable error code recorded when
/// [`RayTraceRenderer::resize`](euv_engine::RayTraceRenderer) rejects new
/// dimensions.
pub(crate) const RAYTRACE_ERROR_CODE_RESIZE: &str = "RAYTRACE_RESIZE";

/// The fallback detail text used when a rejected engine call carries a
/// `JsValue` that is not a string (a thrown object, a number, ...).
pub(crate) const RAYTRACE_ERROR_DETAIL_UNKNOWN: &str = "engine returned a non-string error";

/// The log prefix every RayTrace engine diagnostic is written with.
pub(crate) const RAYTRACE_LOG_PREFIX: &str = "[euv-engine][raytrace]";

/// The message written to the console when a backend finishes
/// initializing.
pub(crate) const RAYTRACE_LOG_BACKEND_READY: &str = "backend ready";

/// Minimum visible duration in milliseconds for a RayTrace tab's loading
/// overlay.
///
/// A backend can finish `init` well inside a single
/// `requestAnimationFrame` tick, and most browsers collapse a mount +
/// unmount that happens inside one tick into a single paint. The overlay
/// would then never be seen at all, so the `loaded` flag is flipped on a
/// timer instead of immediately, which guarantees the user sees the
/// "Initializing..." line for every tab.
pub(crate) const RAYTRACE_LOADING_MIN_MILLIS: i32 = 400;

/// Logical width of the RayTrace Canvas 2D tab's internal framebuffer at
/// render scale 1.0.
///
/// Every ladder step is a multiple of this pair, so the backing store
/// keeps the exact 4:3 ratio the `c_raytrace_canvas_fullscreen`
/// `object-fit: contain` letterbox contract expects.
pub(crate) const RAYTRACE_WIDTH: f64 = 320.0;

/// Logical height of the RayTrace Canvas 2D tab's internal framebuffer at
/// render scale 1.0.
pub(crate) const RAYTRACE_HEIGHT: f64 = 240.0;

/// The render-scale ladder for the Canvas 2D adaptive-resolution path.
///
/// The backing buffer is sized `320 * scale` by `240 * scale`, so every
/// step keeps a 4:3 frame: 1280x960, 960x720, 800x600, 640x480, 560x420,
/// 480x360, 400x300, 320x240, 240x180, 160x120, 120x90, 80x60. The loop
/// starts at index 7 (scale 1.0) so weak clients never start heavy, and
/// the controller only climbs toward 4.0 after sustained fast frames prove
/// the budget allows it.
pub(crate) const RAYTRACE_RENDER_SCALES: [f64; 12] = [
    4.0, 3.0, 2.5, 2.0, 1.75, 1.5, 1.25, 1.0, 0.75, 0.5, 0.375, 0.25,
];

/// The index into [`RAYTRACE_RENDER_SCALES`] the Canvas 2D loop starts
/// at.
pub(crate) const RAYTRACE_RENDER_SCALE_START_INDEX: usize = 7;

/// Exponential-moving-average blend factor for the measured per-frame
/// render time that drives adaptive resolution.
pub(crate) const RAYTRACE_ADAPT_EMA_ALPHA: f64 = 0.1;

/// Measured render time in milliseconds above which the adaptive
/// controller steps the render scale down (115% of the 60 FPS budget).
pub(crate) const RAYTRACE_ADAPT_SLOW_FRAME_MILLIS: f64 = 16.67 * 1.15;

/// Measured render time in milliseconds below which the adaptive
/// controller steps the render scale up one rung (75% of the 60 FPS
/// budget).
pub(crate) const RAYTRACE_ADAPT_FAST_FRAME_MILLIS: f64 = 16.67 * 0.75;

/// Measured render time in milliseconds below which the adaptive
/// controller steps the render scale up two rungs at once (45% of the 60
/// FPS budget), skipping intermediate rungs when the headroom is obvious.
pub(crate) const RAYTRACE_ADAPT_VERY_FAST_FRAME_MILLIS: f64 = 16.67 * 0.45;

/// Number of consecutive slow frames required before stepping the render
/// scale down one notch.
pub(crate) const RAYTRACE_ADAPT_SLOW_FRAMES: u32 = 30;

/// Number of consecutive fast frames required before stepping the render
/// scale up (one notch, or two notches when the frame time also stayed
/// below [`RAYTRACE_ADAPT_VERY_FAST_FRAME_MILLIS`] for the same span).
pub(crate) const RAYTRACE_ADAPT_FAST_FRAMES: u32 = 45;

/// Supersampling grid resolution per axis used by every RayTrace tab.
///
/// `2` traces four rays per pixel, matching the 2x2 supersampling the
/// demo has always used and keeping the three backends on one sampling
/// grid. It sits inside the engine's accepted `1..=4` range, so
/// `RayTraceSettings::validate` accepts it unchanged.
pub(crate) const RAYTRACE_SAMPLES_PER_AXIS: u32 = 2;

/// The secondary bounce limit used by every RayTrace tab.
///
/// `4` matches the engine default and the bounce count the demo has
/// always advertised, and sits inside the engine's accepted `0..=16`
/// range.
pub(crate) const RAYTRACE_MAX_BOUNCES: u32 = 4;

/// The orbit yaw speed in radians per second for auto-rotation.
///
/// Mirrors the 3D game page's constant so the two orbit demos feel the
/// same when both are visible in the sidebar.
pub(crate) const RAYTRACE_AUTO_YAW_SPEED: f64 = 0.5;

/// The minimum angle in radians between the camera pitch and +/- pi/2.
///
/// Wider than a pure gimbal guard on purpose. The engine derives the
/// camera basis as `forward x up`, and a forward vector very close to
/// vertical makes that cross product collapse toward zero; past the
/// point where it reaches zero the engine rejects the frame outright as
/// a degenerate basis. An 0.12 rad floor keeps the cross product's
/// horizontal component at 0.12 or more, which is the smallest value that
/// both stays numerically far from the degenerate case and still lets the
/// user look nearly straight down at the scene.
pub(crate) const RAYTRACE_PITCH_CLAMP: f64 = 0.12;

/// The sensitivity multiplier applied to pointer drag deltas before they
/// are folded into orbit angles.
///
/// Matches the 3D game page's pointer handlers so the two demos feel
/// identical in drag responsiveness.
pub(crate) const RAYTRACE_DRAG_SENSITIVITY: f64 = 0.01;

/// The radius of the orbit sphere the camera sits on.
pub(crate) const RAYTRACE_CAMERA_DISTANCE: f64 = 8.0;

/// The x-coordinate of the orbit sphere's centre (the scene's look-at
/// target).
pub(crate) const RAYTRACE_CAMERA_LOOK_AT_X: f64 = 0.0;

/// The y-coordinate of the orbit sphere's centre (the scene's look-at
/// target).
pub(crate) const RAYTRACE_CAMERA_LOOK_AT_Y: f64 = 0.4;

/// The z-coordinate of the orbit sphere's centre (the scene's look-at
/// target).
pub(crate) const RAYTRACE_CAMERA_LOOK_AT_Z: f64 = 0.0;

/// The fixed viewer position the demo's specular lobes are evaluated
/// against.
///
/// The light set is built once and reused for the lifetime of the loop so
/// no frame allocates a light list, which means the shading eye is fixed
/// as well: the demo traces exactly what it packs, and the specular
/// highlight on the mirror sphere reads as a highlight for this viewer
/// rather than tracking the orbiting eye.
pub(crate) const RAYTRACE_SHADING_EYE_X: f64 = 0.0;

/// The y component of [`RAYTRACE_SHADING_EYE_X`]'s viewer position.
pub(crate) const RAYTRACE_SHADING_EYE_Y: f64 = 1.2;

/// The z component of [`RAYTRACE_SHADING_EYE_X`]'s viewer position.
pub(crate) const RAYTRACE_SHADING_EYE_Z: f64 = 4.2;

/// The JavaScript property name for the touch list `touches` on a
/// `TouchEvent`.
pub(crate) const RAYTRACE_EVENT_PROPERTY_TOUCHES: &str = "touches";

/// The JavaScript property name for the client X coordinate on a `Touch`
/// object.
pub(crate) const RAYTRACE_EVENT_PROPERTY_CLIENT_X: &str = "clientX";

/// The JavaScript property name for the client Y coordinate on a `Touch`
/// object.
pub(crate) const RAYTRACE_EVENT_PROPERTY_CLIENT_Y: &str = "clientY";

/// The JavaScript event name for the window resize event.
pub(crate) const RAYTRACE_EVENT_RESIZE: &str = "resize";

/// The JavaScript property name for the device pixel ratio on `Window`.
pub(crate) const RAYTRACE_EVENT_PROPERTY_DEVICE_PIXEL_RATIO: &str = "devicePixelRatio";

/// Delay in milliseconds before a loop's first `requestAnimationFrame`
/// callback is scheduled, allowing the canvas element to mount before the
/// first frame attempts to acquire a backend.
pub(crate) const RAYTRACE_LOOP_START_DELAY_MILLIS: i32 = 360;

/// Debounce window in milliseconds applied to window resize events before
/// they mark the GPU-backed loops' backing store dirty.
///
/// Mirrors `GAME_3D_RESIZE_DEBOUNCE_MILLIS`.
pub(crate) const RAYTRACE_RESIZE_DEBOUNCE_MILLIS: i32 = 100;
