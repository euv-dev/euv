/// The title text for header on the Phong lighting page.
pub(crate) const LIGHTING_HEADER_TITLE: &str = "Phong Lighting";

/// The subtitle text for header on the Phong lighting page.
pub(crate) const LIGHTING_HEADER_SUBTITLE: &str = "A Phong shading demo rendered three ways: a Canvas 2D software path with an ImageData fast path and adaptive internal resolution, a WebGL 2 GLSL fragment-shader path, and a WebGPU WGSL path. All backends shade the same scene (5 spheres + 1 ground line, 1 directional sun + 1 point lamp) with 2x2 sub-sample coverage, and every tab reports an honest wall-clock FPS. Click Enter Fullscreen for a larger view.";

/// The section heading for the canvas card on the Phong lighting page.
pub(crate) const LIGHTING_CANVAS_CARD_TITLE: &str = "Lighting Demo";

/// The section heading for the backends card on the Phong lighting page.
pub(crate) const LIGHTING_BACKENDS_CARD_TITLE: &str = "Lighting Backends";

/// The long-form description of canvas on the Phong lighting page.
pub(crate) const LIGHTING_CANVAS_DESCRIPTION: &str = "The Canvas 2D tab runs euv-engine's lighting module on the CPU: every frame, every sphere pixel reconstructs the surface normal from its screen-space position (dz = sqrt(r^2 - d^2)) and feeds it to LightingUniforms::shade together with one directional sun and one point lamp, and the ground line is shaded with the same pipeline using a fixed up-pointing normal. Finished frames are packed into a persistent RGBA buffer (gamma 1/2.2, alpha 0 outside the shapes so the theme background shows through) and uploaded with a single put_image_data call, and an EMA of the CPU frame time steps the internal resolution through a 640x480 .. 80x60 ladder (always 4:3, so the fullscreen letterbox holds) to protect the frame rate, starting at 320x240 and climbing only when the budget allows. The FPS counter measures unclamped wall-clock time.";

/// The long-form description of webgl on the Phong lighting page.
pub(crate) const LIGHTING_WEBGL_DESCRIPTION: &str = "The WebGL tab runs the identical scene inside a GLSL ES 3.00 fragment shader drawn on an attribute-less fullscreen triangle (gl_VertexID, no vertex buffers): the five circles, ground row, sun, and point lamp are hardcoded in the shader, and the canvas resolution plus computed background color are uploaded per frame as a vec4 uniform array. The fragment shader mirrors the engine's LightingUniforms::shade term for term (gamma 1/2.2) and anti-aliases at the physical canvas resolution: each fragment takes 2x2 sub-samples in physical pixels and evaluates the analytic scene at each sub-sample's exact logical position (painter's order: background, ground row, spheres back-to-front), so edges stay smooth at any backing resolution. The fixed 4:3 logical scene is letterboxed with a uniform scale so the spheres never stretch at any canvas size. Works in every modern browser with WebGL 2 support.";

/// The long-form description of webgpu on the Phong lighting page.
pub(crate) const LIGHTING_WEBGPU_DESCRIPTION: &str = "The WebGPU tab runs the same shader logic expressed in WGSL: a fullscreen triangle generated from @builtin(vertex_index) and a fragment stage that shades the scene with physical-resolution 2x2 sub-sample coverage. Per-frame data arrives in a single 2-vec4 uniform buffer at @group(0) @binding(0) via WebGpuRenderer's create_render_pipeline / create_uniform_buffer / render_frame_with_bind_group helpers. Requires a WebGPU-capable browser (Chrome 113+, Edge 113+).";

/// The label text for pause on the Phong lighting page.
pub(crate) const LIGHTING_PAUSE_LABEL: &str = "Pause";

/// The label text for resume on the Phong lighting page.
pub(crate) const LIGHTING_RESUME_LABEL: &str = "Resume";

/// The text printed before the fps readout on the Phong lighting page.
pub(crate) const LIGHTING_FPS_PREFIX: &str = "FPS: ";

/// The fixed summary line for lights on the Phong lighting page.
pub(crate) const LIGHTING_LIGHTS_SUMMARY: &str = "Lights: 1 directional + 1 point";

/// The text printed before the scale readout on the Phong lighting page.
pub(crate) const LIGHTING_SCALE_PREFIX: &str = "Scale: ";

/// The text printed before the status readout on the Phong lighting page.
pub(crate) const LIGHTING_STATUS_PREFIX: &str = "Status: ";

/// The button label for exit on the Phong lighting page.
pub(crate) const LIGHTING_EXIT_BUTTON_LABEL: &str = "Exit";

/// The button label for enter fullscreen on the Phong lighting page.
pub(crate) const LIGHTING_ENTER_FULLSCREEN_BUTTON_LABEL: &str = "Enter Fullscreen";

/// The status text shown for canvas 2d active on the Phong lighting page.
pub(crate) const LIGHTING_CANVAS_2D_ACTIVE_STATUS: &str = "Canvas 2D Active";

/// The status text shown for initializing on the Phong lighting page.
pub(crate) const LIGHTING_INITIALIZING_STATUS: &str = "Initializing...";

/// The status text shown for webgl active on the Phong lighting page.
pub(crate) const LIGHTING_WEBGL_ACTIVE_STATUS: &str = "WebGL Active";

/// The status text shown for webgl not supported on the Phong lighting page.
pub(crate) const LIGHTING_WEBGL_NOT_SUPPORTED_STATUS: &str = "WebGL not supported";

/// The status text shown for webgl init failed on the Phong lighting page.
pub(crate) const LIGHTING_WEBGL_INIT_FAILED_STATUS: &str = "WebGL init failed";
