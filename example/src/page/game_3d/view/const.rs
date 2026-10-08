/// The title text for header on the 3D game page.
pub(crate) const GAME_3D_HEADER_TITLE: &str = "3D Game Engine";

/// The subtitle text for header on the 3D game page.
pub(crate) const GAME_3D_HEADER_SUBTITLE: &str = "A rotating cubes 3D demo powered by euv-engine's Vector3D, Quaternion, Matrix4x4, and Camera3D. Drag to orbit the camera. Switch tabs to compare Canvas 2D and WebGPU rendering backends.";

/// The section heading for the canvas card on the 3D game page.
pub(crate) const GAME_3D_CANVAS_CARD_TITLE: &str = "3D Rendering Demo";

/// The section heading for the features card on the 3D game page.
pub(crate) const GAME_3D_FEATURES_CARD_TITLE: &str = "3D Engine Features";

/// The long-form description of canvas on the 3D game page.
pub(crate) const GAME_3D_CANVAS_DESCRIPTION: &str = "This demo uses euv-engine's 3D math: Vector3D for positions, Quaternion for rotation, Matrix4x4 for view/projection transforms, Camera3D for orbit camera with perspective projection, and Transform3D for cube transforms. Features include back-face culling, painter's algorithm depth sorting, and quaternion-based angular velocity integration. The WebGPU tab demonstrates GPU-accelerated rendering with a WGSL shader pipeline.";

/// The long-form description of webgpu on the 3D game page.
pub(crate) const GAME_3D_WEBGPU_DESCRIPTION: &str = "This demo uses euv-engine's WebGpuRenderer to initialize a GPU device, create a render pipeline from a WGSL shader, and render the same rotating cubes scene as the Canvas 2D tab: every cube is drawn as 12 shader-generated triangles with per-cube transform and colors uploaded to a uniform buffer each frame via requestAnimationFrame. Drag on the canvas to orbit the camera. Requires a WebGPU-capable browser (Chrome 113+, Edge 113+).";

/// The long-form description of webgl on the 3D game page.
pub(crate) const GAME_3D_WEBGL_DESCRIPTION: &str = "This demo uses euv-engine's WebGl2Backend to acquire a WebGL 2 context, compile a GLSL ES 3.00 program, and render the same rotating cubes scene as the Canvas 2D tab: every cube is drawn as 12 shader-generated triangles with per-cube transform and colors uploaded to vec4 uniform arrays each frame via requestAnimationFrame. Drag on the canvas to orbit the camera. Works in every modern browser with WebGL 2 support.";

/// The label text for pause on the 3D game page.
pub(crate) const GAME_3D_PAUSE_LABEL: &str = "Pause";

/// The label text for resume on the 3D game page.
pub(crate) const GAME_3D_RESUME_LABEL: &str = "Resume";

/// The label text for auto rotate on on the 3D game page.
pub(crate) const GAME_3D_AUTO_ROTATE_ON_LABEL: &str = "Auto: On";

/// The label text for auto rotate off on the 3D game page.
pub(crate) const GAME_3D_AUTO_ROTATE_OFF_LABEL: &str = "Auto: Off";

/// The text printed before the fps readout on the 3D game page.
pub(crate) const GAME_3D_FPS_PREFIX: &str = "FPS: ";

/// The text printed before the cubes readout on the 3D game page.
pub(crate) const GAME_3D_CUBES_PREFIX: &str = "Cubes: ";

/// The button label for exit on the 3D game page.
pub(crate) const GAME_3D_EXIT_BUTTON_LABEL: &str = "Exit";

/// The button label for reset camera on the 3D game page.
pub(crate) const GAME_3D_RESET_CAMERA_BUTTON_LABEL: &str = "Reset Camera";

/// The button label for enter fullscreen on the 3D game page.
pub(crate) const GAME_3D_ENTER_FULLSCREEN_BUTTON_LABEL: &str = "Enter Fullscreen";

/// The text printed before the status readout on the 3D game page.
pub(crate) const GAME_3D_STATUS_PREFIX: &str = "Status: ";

/// The status text shown for initializing on the 3D game page.
pub(crate) const GAME_3D_INITIALIZING_STATUS: &str = "Initializing...";

/// The status text shown for webgl active on the 3D game page.
pub(crate) const GAME_3D_WEBGL_ACTIVE_STATUS: &str = "WebGL Active";

/// The status text shown for webgl not supported on the 3D game page.
pub(crate) const GAME_3D_WEBGL_NOT_SUPPORTED_STATUS: &str = "WebGL not supported";

/// The status text shown for webgl init failed on the 3D game page.
pub(crate) const GAME_3D_WEBGL_INIT_FAILED_STATUS: &str = "WebGL init failed";
