/// The title text for header on the 2D game page.
pub(crate) const GAME_2D_HEADER_TITLE: &str = "2D Game Engine";

/// The subtitle text for header on the 2D game page.
pub(crate) const GAME_2D_HEADER_SUBTITLE: &str = "A bouncing balls physics demo powered by euv-engine. Click on the canvas to spawn balls. Each ball has gravity, wall bouncing with restitution, and impulse-based ball-to-ball collision. Switch tabs to compare Canvas 2D, WebGL, and WebGPU rendering backends.";

/// The section heading for the canvas card on the 2D game page.
pub(crate) const GAME_2D_CANVAS_CARD_TITLE: &str = "2D Rendering Demo";

/// The section heading for the features card on the 2D game page.
pub(crate) const GAME_2D_FEATURES_CARD_TITLE: &str = "2D Engine Features";

/// The long-form description of canvas on the 2D game page.
pub(crate) const GAME_2D_CANVAS_DESCRIPTION: &str = "This demo uses euv-engine's Vector2D for position/velocity math, impulse-based collision resolution with mass proportional to radius squared, wall reflection with configurable restitution, and a fixed-timestep game loop with accumulator pattern for deterministic physics at 60 Hz. The WebGPU tab demonstrates GPU-accelerated rendering with a WGSL shader pipeline.";

/// The long-form description of webgpu on the 2D game page.
pub(crate) const GAME_2D_WEBGPU_DESCRIPTION: &str = "This demo uses euv-engine's WebGpuRenderer to initialize a GPU device, create a render pipeline from a WGSL shader, and render the same bouncing balls scene as the Canvas 2D tab: every ball is drawn as a shader-generated quad with per-ball position, radius, and color uploaded to a uniform buffer each frame. Click or tap to spawn balls; pause and clear work exactly like Canvas 2D. Requires a WebGPU-capable browser (Chrome 113+, Edge 113+).";

/// The long-form description of webgl on the 2D game page.
pub(crate) const GAME_2D_WEBGL_DESCRIPTION: &str = "This demo uses euv-engine's WebGl2Backend to acquire a WebGL 2 context, compile a GLSL ES 3.00 program, and render the same bouncing balls scene as the Canvas 2D tab: every ball is drawn as a shader-generated quad with per-ball position, radius, and color uploaded to vec4 uniform arrays each frame. Click or tap to spawn balls; pause and clear work exactly like Canvas 2D. Works in every modern browser with WebGL 2 support.";

/// The label text for pause on the 2D game page.
pub(crate) const GAME_2D_PAUSE_LABEL: &str = "Pause";

/// The label text for resume on the 2D game page.
pub(crate) const GAME_2D_RESUME_LABEL: &str = "Resume";

/// The text printed before the fps readout on the 2D game page.
pub(crate) const GAME_2D_FPS_PREFIX: &str = "FPS: ";

/// The text printed before the balls readout on the 2D game page.
pub(crate) const GAME_2D_BALLS_PREFIX: &str = "Balls: ";

/// The text printed before the total readout on the 2D game page.
pub(crate) const GAME_2D_TOTAL_PREFIX: &str = "Total: ";

/// The button label for exit on the 2D game page.
pub(crate) const GAME_2D_EXIT_BUTTON_LABEL: &str = "Exit";

/// The button label for clear on the 2D game page.
pub(crate) const GAME_2D_CLEAR_BUTTON_LABEL: &str = "Clear";

/// The button label for enter fullscreen on the 2D game page.
pub(crate) const GAME_2D_ENTER_FULLSCREEN_BUTTON_LABEL: &str = "Enter Fullscreen";

/// The text printed before the status readout on the 2D game page.
pub(crate) const GAME_2D_STATUS_PREFIX: &str = "Status: ";

/// The status text shown for initializing on the 2D game page.
pub(crate) const GAME_2D_INITIALIZING_STATUS: &str = "Initializing...";

/// The status text shown for webgl active on the 2D game page.
pub(crate) const GAME_2D_WEBGL_ACTIVE_STATUS: &str = "WebGL Active";

/// The status text shown for webgl not supported on the 2D game page.
pub(crate) const GAME_2D_WEBGL_NOT_SUPPORTED_STATUS: &str = "WebGL not supported";

/// The status text shown for webgl init failed on the 2D game page.
pub(crate) const GAME_2D_WEBGL_INIT_FAILED_STATUS: &str = "WebGL init failed";
