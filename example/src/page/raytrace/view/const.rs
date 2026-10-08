/// The title text for header on the ray-tracing page.
pub(crate) const RAYTRACE_HEADER_TITLE: &str = "RayTrace";

/// The subtitle text for header on the ray-tracing page.
pub(crate) const RAYTRACE_HEADER_SUBTITLE: &str = "A real-time ray tracer rendered three ways: a Canvas 2D software path with an ImageData fast path and adaptive internal resolution, a WebGL 2 GLSL fragment-shader path, and a WebGPU WGSL path. All backends trace the same scene (1 mirror sphere + 1 emissive sphere + 1 ground AABB) with 2x2 SSAA and up to 4 reflection bounces, and every tab reports an honest wall-clock FPS. Drag the canvas to orbit the camera; the directional sun rotates with the yaw so the lit side of the spheres tracks the orbiting camera. Click Enter Fullscreen for a larger view.";

/// The `tab title canvas 2d` text used on the ray-tracing page.
pub(crate) const RAYTRACE_TAB_TITLE_CANVAS_2D: &str = "RayTrace Demo (2D)";

/// The `tab title webgl` text used on the ray-tracing page.
pub(crate) const RAYTRACE_TAB_TITLE_WEBGL: &str = "RayTrace Demo (GL)";

/// The `tab title webgpu` text used on the ray-tracing page.
pub(crate) const RAYTRACE_TAB_TITLE_WEBGPU: &str = "RayTrace Demo (GPU)";

/// The section heading for the backends card on the ray-tracing page.
pub(crate) const RAYTRACE_BACKENDS_CARD_TITLE: &str = "RayTrace Backends";

/// The long-form description of canvas on the ray-tracing page.
pub(crate) const RAYTRACE_CANVAS_DESCRIPTION: &str = "The Canvas 2D tab runs euv-engine's raytracing module on the CPU: every frame, for every pixel of the internal buffer, the camera fires a primary Ray through the scene using RayTraceScene::trace, which iteratively reflects up to 4 bounces with zero heap allocation per ray. LightingUniforms::shade combines ambient, Lambertian diffuse, and Phong specular per hit. Finished frames are packed into a persistent RGBA buffer (gamma 1/2.2) and uploaded with a single put_image_data call, and an EMA of the CPU frame time steps the internal resolution through a 640x480 .. 80x60 ladder (always 4:3, so the fullscreen letterbox holds) to protect the frame rate, starting at 320x240 and climbing only when the budget allows. The FPS counter measures unclamped wall-clock time.";

/// The long-form description of webgl on the ray-tracing page.
pub(crate) const RAYTRACE_WEBGL_DESCRIPTION: &str = "The WebGL tab runs the identical scene inside a GLSL ES 3.00 fragment shader drawn on an attribute-less fullscreen triangle (gl_VertexID, no vertex buffers): the ground AABB, mirror sphere, and emissive sphere are hardcoded in the shader, and the orbit camera basis, sun direction, ambient, and canvas resolution are uploaded per frame as a vec4 uniform array. The fragment shader mirrors the engine's trace_bounces and LightingUniforms::shade term for term (2x2 SSAA, 4 bounces, gamma 1/2.2), and the NDC is aspect-corrected from the resolution uniform so the scene never stretches at any canvas size. Works in every modern browser with WebGL 2 support.";

/// The long-form description of webgpu on the ray-tracing page.
pub(crate) const RAYTRACE_WEBGPU_DESCRIPTION: &str = "The WebGPU tab runs the same shader logic expressed in WGSL: a fullscreen triangle generated from @builtin(vertex_index) and a fragment stage that ray-traces the scene per pixel with 2x2 SSAA and up to 4 bounces. Per-frame data arrives in a single 8-vec4 uniform buffer at @group(0) @binding(0) via WebGpuRenderer's create_render_pipeline / create_uniform_buffer / render_frame_with_bind_group helpers. Requires a WebGPU-capable browser (Chrome 113+, Edge 113+).";

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
pub(crate) const RAYTRACE_SCENE_SUMMARY: &str = "Scene: 1 mirror + 1 emissive + 1 ground";

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
