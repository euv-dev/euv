/// The title text for header on the ray-tracing page.
pub(crate) const RAYTRACE_HEADER_TITLE: &str = "RayTrace";

/// The subtitle text for header on the ray-tracing page.
pub(crate) const RAYTRACE_HEADER_SUBTITLE: &str = "One ray tracer, three rendering backends. Every tab renders the identical scene through euv-engine's shared RayTraceRenderer: a Canvas 2D software path with adaptive internal resolution, a WebGL 2 GLSL fragment-shader path, and a WebGPU WGSL path. The scene mixes all three primitive kinds the engine traces — a ground AABB, a mirror sphere, an emissive sphere, and a reflective triangle — lit by one directional sun, one point fill, and one spot rim, with 2x2 supersampling and up to 4 reflection bounces. This is shader BVH traversal, not native hardware ray tracing. Drag the canvas to orbit the camera; every tab reports an honest wall-clock FPS. Click Enter Fullscreen for a larger view.";

/// The `tab title canvas 2d` text used on the ray-tracing page.
pub(crate) const RAYTRACE_TAB_TITLE_CANVAS_2D: &str = "RayTrace Demo (2D)";

/// The `tab title webgl` text used on the ray-tracing page.
pub(crate) const RAYTRACE_TAB_TITLE_WEBGL: &str = "RayTrace Demo (GL)";

/// The `tab title webgpu` text used on the ray-tracing page.
pub(crate) const RAYTRACE_TAB_TITLE_WEBGPU: &str = "RayTrace Demo (GPU)";

/// The section heading for the backends card on the ray-tracing page.
pub(crate) const RAYTRACE_BACKENDS_CARD_TITLE: &str = "RayTrace Backends";

/// The long-form description of canvas on the ray-tracing page.
pub(crate) const RAYTRACE_CANVAS_DESCRIPTION: &str = "The Canvas 2D tab runs euv-engine's raytracing module on the CPU. Every frame goes through RayTraceRenderer::render, which traces the scene's bounding-volume hierarchy per pixel with 2x2 supersampling, shades each hit through LightingUniforms::shade_with_visibility using exact BVH shadow rays, averages the samples, applies a 1/2.2 gamma curve, and uploads the result with a single put_image_data. The scene and the light set are built once and never rebuilt: nothing in them moves with the camera, so no frame re-packs or re-clones the scene. An EMA of the measured render time steps the internal resolution through a 640x480 .. 80x60 ladder (always 4:3, so the fullscreen letterbox holds) to protect the frame rate, starting at 320x240 and climbing only when the budget allows. The FPS counter measures unclamped wall-clock time.";

/// The long-form description of webgl on the ray-tracing page.
pub(crate) const RAYTRACE_WEBGL_DESCRIPTION: &str = "The WebGL tab renders the same scene through the engine's own GLSL ES 3.00 program on an attribute-less fullscreen triangle driven by gl_VertexID. The scene arrives as a single RGBA32F data texture holding the packed BVH nodes and primitive records, and the frame arrives as a second one; the fragment shader reads both with texelFetch and traverses the hierarchy itself, so no geometry, light count, or node count is baked into the shader. The per-frame work is a camera and lighting update against persistent textures and program objects: the static scene is uploaded exactly once, at initialization, and never re-sent. Shadows are real shadow rays through the same hierarchy, not a bounding-sphere approximation. Aspect comes from the resolution the frame is packed with, so the scene never stretches. Works in every modern browser with WebGL 2 support.";

/// The long-form description of webgpu on the ray-tracing page.
pub(crate) const RAYTRACE_WEBGPU_DESCRIPTION: &str = "The WebGPU tab runs the same traversal expressed in WGSL: a fullscreen triangle generated from @builtin(vertex_index) and a fragment stage that traces the packed hierarchy per pixel with 2x2 supersampling and up to 4 bounces. The engine owns the pipeline, the bind group, and the two storage buffers, and it awaits shader compilation and device validation during initialization rather than discovering a broken pipeline on the first frame. After that, a steady frame reuses persistent buffers and does not re-encode the scene. Requires a WebGPU-capable browser (Chrome 113+, Edge 113+).";

/// The label text for pause on the ray-tracing page.
pub(crate) const RAYTRACE_PAUSE_LABEL: &str = "Pause";

/// The label text for resume on the ray-tracing page.
pub(crate) const RAYTRACE_RESUME_LABEL: &str = "Resume";

/// The label text for auto rotate on on the ray-tracing page.
pub(crate) const RAYTRACE_AUTO_ROTATE_ON_LABEL: &str = "Auto: On";

/// The label text for auto rotate off on the ray-tracing page.
pub(crate) const RAYTRACE_AUTO_ROTATE_OFF_LABEL: &str = "Auto: Off";

/// The text printed before the fps readout on the ray-tracing page.
pub(crate) const RAYTRACE_FPS_PREFIX: &str = "FPS: ";

/// The fixed summary line for scene on the ray-tracing page.
pub(crate) const RAYTRACE_SCENE_SUMMARY: &str = "Scene: AABB + 2 spheres + triangle";

/// The text printed before the scale readout on the ray-tracing page.
pub(crate) const RAYTRACE_SCALE_PREFIX: &str = "Scale: ";

/// The button label for exit on the ray-tracing page.
pub(crate) const RAYTRACE_EXIT_BUTTON_LABEL: &str = "Exit";

/// The button label for reset camera on the ray-tracing page.
pub(crate) const RAYTRACE_RESET_CAMERA_BUTTON_LABEL: &str = "Reset Camera";

/// The button label for enter fullscreen on the ray-tracing page.
pub(crate) const RAYTRACE_ENTER_FULLSCREEN_BUTTON_LABEL: &str = "Enter Fullscreen";

/// The status text shown for initializing on the ray-tracing page.
pub(crate) const RAYTRACE_INITIALIZING_STATUS: &str = "Initializing...";

/// The status text shown for webgl active on the ray-tracing page.
pub(crate) const RAYTRACE_WEBGL_ACTIVE_STATUS: &str = "WebGL Active";

/// The status text shown for webgl not supported on the ray-tracing page.
pub(crate) const RAYTRACE_WEBGL_NOT_SUPPORTED_STATUS: &str = "WebGL not supported";

/// The status text shown for webgl init failed on the ray-tracing page.
pub(crate) const RAYTRACE_WEBGL_INIT_FAILED_STATUS: &str = "WebGL init failed";

/// The text printed before the status readout on the ray-tracing page.
pub(crate) const RAYTRACE_STATUS_PREFIX: &str = "Status: ";

/// The separator placed between a status code and its detail text.
pub(crate) const RAYTRACE_STATUS_DETAIL_SEPARATOR: &str = " — ";
