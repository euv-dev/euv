use super::*;

/// The CSS selector used to query the 3D game canvas element from the DOM.
pub(crate) const GAME_3D_CANVAS_SELECTOR: &str = "#game-3d-canvas";

/// The default canvas width in CSS pixels.
pub(crate) const GAME_3D_CANVAS_WIDTH: f64 = 600.0;

/// The default canvas height in CSS pixels.
pub(crate) const GAME_3D_CANVAS_HEIGHT: f64 = 400.0;

/// The fixed timestep for the game loop in seconds (60 FPS).
pub(crate) const GAME_3D_FIXED_TIMESTEP: f64 = 1.0 / 60.0;

/// The half-size of a cube edge, used to define cube vertices relative to center.
pub(crate) const GAME_3D_CUBE_HALF_SIZE: f64 = 1.0;

/// The distance of the camera from the origin.
pub(crate) const GAME_3D_CAMERA_DISTANCE: f64 = 8.0;

/// The orbit yaw speed in radians per second for auto-rotation.
pub(crate) const GAME_3D_AUTO_YAW_SPEED: f64 = 0.5;

/// The minimum angle in radians between the camera pitch and ±π/2.
///
/// This prevents the orbit camera from looking straight up or down, which
/// would make the `forward` vector parallel to the `up` vector and cause
/// the `right = forward × up` cross product to degenerate, producing a
/// zero vector after normalization and collapsing the view matrix.
pub(crate) const GAME_3D_PITCH_CLAMP: f64 = 0.01;

/// The debounce interval in milliseconds for the resize event handler.
pub(crate) const GAME_3D_RESIZE_DEBOUNCE_MILLIS: i32 = 100;

/// The delay in milliseconds before starting the 3D game loop after page mount.
///
/// Defers the heavy `requestAnimationFrame` rendering loop to avoid competing
/// with the mobile drawer close animation for main thread time, preventing
/// sidebar animation stutter on page transitions.
pub(crate) const GAME_3D_LOOP_START_DELAY_MILLIS: i32 = 360;

/// The JavaScript property name for the canvas fill style.
pub(crate) const GAME_3D_PROPERTY_FILL_STYLE: &str = "fillStyle";

/// The CSS property name for the computed background colour, used to fill
/// the loading overlay so the scene does not bleed through.
pub(crate) const GAME_3D_PROPERTY_BACKGROUND_COLOR: &str = "background-color";

/// The loading text displayed on the canvas before the game loop starts.
pub(crate) const GAME_3D_LOADING_TEXT: &str = "Loading...";

/// The CSS font family used for the loading text on the canvas.
pub(crate) const GAME_3D_LOADING_FONT_FAMILY: &str = "sans-serif";

/// The ratio of the loading font size to the canvas height.
pub(crate) const GAME_3D_LOADING_FONT_SIZE_RATIO: f64 = 0.04;

/// The CSS variable name for the loading text color on the canvas.
///
/// Uses `--text-on-accent` because the canvas background is `var!(accent)`,
/// and `text-on-accent` is the theme variable that contrasts with the accent
/// color (foreground/background equal accent in this monochrome design).
pub(crate) const GAME_3D_LOADING_COLOR_VAR: &str = "--text-on-accent";

/// The minimum time in milliseconds the loading overlay stays visible.
///
/// Fast init paths (notably synchronous WebGL init) would otherwise add and
/// remove the overlay canvas within a single frame, so the browser never
/// paints the loading state on tab switches.
pub(crate) const GAME_3D_LOADING_MIN_MILLIS: i32 = 400;

/// The JavaScript property name for the canvas stroke style.
pub(crate) const GAME_3D_PROPERTY_STROKE_STYLE: &str = "strokeStyle";

/// The CSS color used for cube faces.
pub(crate) const GAME_3D_CUBE_FACE_COLOR: &str = "#16c79a";

/// The CSS color used for cube edges.
pub(crate) const GAME_3D_CUBE_EDGE_COLOR: &str = "#e94560";

/// The JavaScript property name for the touch list `touches` on a `TouchEvent`.
pub(crate) const GAME_3D_EVENT_PROPERTY_TOUCHES: &str = "touches";

/// The JavaScript property name for the client X coordinate on a `Touch` object.
pub(crate) const GAME_3D_EVENT_PROPERTY_CLIENT_X: &str = "clientX";

/// The JavaScript property name for the client Y coordinate on a `Touch` object.
pub(crate) const GAME_3D_EVENT_PROPERTY_CLIENT_Y: &str = "clientY";

/// The JavaScript event name for the wheel event, used to register a
/// non-passive listener directly on the canvas element to prevent page
/// scrolling when the mouse wheel is scrolled over the canvas.
pub(crate) const GAME_3D_EVENT_WHEEL: &str = "wheel";

/// The JavaScript event name for the touchstart event, used to register a
/// non-passive listener directly on the canvas element to prevent page
/// scrolling when a finger touches the canvas on mobile devices.
pub(crate) const GAME_3D_EVENT_TOUCH_START: &str = "touchstart";

/// The JavaScript event name for the touchmove event, used to register a
/// non-passive listener directly on the canvas element to prevent page
/// scrolling when a finger drags across the canvas on mobile devices.
pub(crate) const GAME_3D_EVENT_TOUCH_MOVE: &str = "touchmove";

/// The JavaScript event name for the window resize event.
pub(crate) const GAME_3D_EVENT_RESIZE: &str = "resize";

/// The JavaScript property name for the device pixel ratio on `Window`.
pub(crate) const GAME_3D_EVENT_PROPERTY_DEVICE_PIXEL_RATIO: &str = "devicePixelRatio";

/// The cube vertex offsets relative to center, defining the 8 corners of a unit cube.
pub(crate) const GAME_3D_CUBE_VERTICES: [(f64, f64, f64); 8] = [
    (-1.0, -1.0, -1.0),
    (1.0, -1.0, -1.0),
    (1.0, 1.0, -1.0),
    (-1.0, 1.0, -1.0),
    (-1.0, -1.0, 1.0),
    (1.0, -1.0, 1.0),
    (1.0, 1.0, 1.0),
    (-1.0, 1.0, 1.0),
];

/// The cube face indices, each defining a quad by referencing 4 vertex indices.
/// Winding order is counter-clockwise when viewed from outside the cube,
/// ensuring that face normals point outward for correct back-face culling.
pub(crate) const GAME_3D_CUBE_FACES: [(usize, usize, usize, usize); 6] = [
    (0, 3, 2, 1),
    (4, 5, 6, 7),
    (0, 1, 5, 4),
    (2, 3, 7, 6),
    (0, 4, 7, 3),
    (1, 2, 6, 5),
];

/// The 12 unique edges of a unit cube, defined by pairs of vertex indices.
///
/// Used to draw the cube wireframe without duplicating the shared edge
/// between two adjacent visible faces — without deduplication the three
/// edges meeting at the front-most vertex are each stroked twice (once per
/// face), which shows up as visible "extra lines" near the cube's inner
/// corner after SSAA downscaling.
pub(crate) const GAME_3D_CUBE_EDGES: [(usize, usize); 12] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 4),
    (0, 4),
    (1, 5),
    (2, 6),
    (3, 7),
];

/// The CSS selector used to query the 3D WebGPU canvas element from the DOM.
pub(crate) const GAME_3D_WEBGPU_CANVAS_SELECTOR: &str = "#game-3d-webgpu-canvas";

/// The CSS selector for the 3D WebGPU loading overlay canvas.
pub(crate) const GAME_3D_WEBGPU_LOADING_CANVAS_SELECTOR: &str = "#game-3d-webgpu-loading-canvas";

/// The maximum number of cubes the GPU shaders can draw in one scene.
///
/// The uniform layouts size their cube arrays to this value; the demo
/// scene spawns 4 cubes and never adds more, so 8 leaves headroom
/// without wasting buffer space.
pub(crate) const GAME_3D_GPU_MAX_CUBES: usize = 8;

/// The WGSL shader source for the 3D WebGPU cubes demo.
///
/// Renders the same scene as the Canvas 2D tab: every cube is drawn as
/// 12 triangles generated procedurally from `@builtin(vertex_index)`
/// (attribute-less rendering). A uniform buffer at
/// `@group(0) @binding(0)` carries the view-projection matrix, the
/// camera position, and one `CubeData` record (rotation quaternion,
/// position + scale, face color, edge color) per cube. Back faces are
/// discarded in the fragment shader and a screen-space distance-to-edge
/// overlay draws the wireframe, matching the Canvas 2D stroke.
pub(crate) const GAME_3D_WEBGPU_SHADER: &str = r#"
struct CubeData {
    rotation: vec4<f32>,
    pos_scale: vec4<f32>,
    face_color: vec4<f32>,
    edge_color: vec4<f32>,
};

struct SceneUniforms {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
    cubes: array<CubeData, 8>,
};

@group(0) @binding(0) var<uniform> u_scene: SceneUniforms;

const CUBE_CORNERS = array<vec3<f32>, 8>(
    vec3<f32>(-1.0, -1.0, -1.0),
    vec3<f32>(1.0, -1.0, -1.0),
    vec3<f32>(1.0, 1.0, -1.0),
    vec3<f32>(-1.0, 1.0, -1.0),
    vec3<f32>(-1.0, -1.0, 1.0),
    vec3<f32>(1.0, -1.0, 1.0),
    vec3<f32>(1.0, 1.0, 1.0),
    vec3<f32>(-1.0, 1.0, 1.0),
);

const FACE_INDICES = array<u32, 24>(
    0u, 3u, 2u, 1u,
    4u, 5u, 6u, 7u,
    0u, 1u, 5u, 4u,
    2u, 3u, 7u, 6u,
    0u, 4u, 7u, 3u,
    1u, 2u, 6u, 5u,
);

const FACE_NORMALS = array<vec3<f32>, 6>(
    vec3<f32>(0.0, 0.0, -1.0),
    vec3<f32>(0.0, 0.0, 1.0),
    vec3<f32>(0.0, -1.0, 0.0),
    vec3<f32>(0.0, 1.0, 0.0),
    vec3<f32>(-1.0, 0.0, 0.0),
    vec3<f32>(1.0, 0.0, 0.0),
);

const CORNER_MAP = array<u32, 6>(0u, 1u, 2u, 0u, 2u, 3u);

const CORNER_UVS = array<vec2<f32>, 6>(
    vec2<f32>(0.0, 0.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(0.0, 0.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(0.0, 1.0),
);

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_pos: vec3<f32>,
    @location(3) face_color: vec3<f32>,
    @location(4) edge_color: vec3<f32>,
};

/// Rotates vector `v` by unit quaternion `q`.
/// Helper body of the `quat_rotate` free function.
///
/// # Arguments
///
/// - `vec4<f32>` - A `vec4<f32>` parameter.
/// - `vec3<f32>` - A `vec3<f32>` parameter.
///
/// # Returns
///
/// - `vec3<f32>` - A `vec3<f32>` value.
fn quat_rotate(q: vec4<f32>, v: vec3<f32>) -> vec3<f32> {
    return v + 2.0 * cross(q.xyz, cross(q.xyz, v) + q.w * v);
}

@vertex
/// WGSL vertex shader entry point.
/// Helper body of the `vs_main` free function.
///
/// # Arguments
///
/// - `u32` - A 32-bit unsigned integer (`u32`).
///
/// # Returns
///
/// - `VertexOutput` - A `VertexOutput` value.
fn vs_main(@builtin(vertex_index) vi: u32) -> VertexOutput {
    let cube = u_scene.cubes[vi / 36u];
    let face_index = (vi % 36u) / 6u;
    let corner_index = vi % 6u;
    let local = CUBE_CORNERS[FACE_INDICES[face_index * 4u + CORNER_MAP[corner_index]]]
        * cube.pos_scale.w;
    let world = quat_rotate(cube.rotation, local) + cube.pos_scale.xyz;
    let clip = u_scene.view_proj * vec4<f32>(world, 1.0);
    var out: VertexOutput;
    // The host view-projection matrix produces OpenGL-style clip z in
    // [-1, 1]; remap to WebGPU's [0, 1] range.
    out.position = vec4<f32>(clip.x, clip.y, clip.z * 0.5 + clip.w * 0.5, clip.w);
    out.uv = CORNER_UVS[corner_index];
    out.normal = quat_rotate(cube.rotation, FACE_NORMALS[face_index]);
    out.world_pos = world;
    out.face_color = cube.face_color.rgb;
    out.edge_color = cube.edge_color.rgb;
    return out;
}

@fragment
/// WGSL fragment shader entry point.
/// Helper body of the `fs_main` free function.
///
/// # Arguments
///
/// - `VertexOutput` - A `VertexOutput` parameter.
///
/// # Returns
///
/// - `@location(0) vec4<f32>` - A `@location(0) vec4<f32>` value.
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let view_dir = normalize(in.world_pos - u_scene.camera_pos.xyz);
    if dot(in.normal, view_dir) >= 0.0 {
        discard;
    }
    // Edge overlay: distance to the quad border in screen pixels, so the
    // wireframe tracks the Canvas 2D stroke width at any zoom level.
    let fw = fwidth(in.uv);
    let dist_x = min(in.uv.x, 1.0 - in.uv.x) / max(fw.x, 1e-6);
    let dist_y = min(in.uv.y, 1.0 - in.uv.y) / max(fw.y, 1e-6);
    if min(dist_x, dist_y) < 0.75 {
        return vec4<f32>(in.edge_color, 1.0);
    }
    return vec4<f32>(in.face_color, 1.0);
}
"#;

/// The CSS selector used to query the 3D WebGL canvas element from the DOM.
pub(crate) const GAME_3D_WEBGL_CANVAS_SELECTOR: &str = "#game-3d-webgl-canvas";

/// The CSS selector for the 3D WebGL loading overlay canvas.
pub(crate) const GAME_3D_WEBGL_LOADING_CANVAS_SELECTOR: &str = "#game-3d-webgl-loading-canvas";

/// The GLSL ES 3.00 vertex shader source for the 3D WebGL cubes demo.
///
/// Mirrors [`GAME_3D_WEBGPU_SHADER`]: vertices are generated procedurally
/// from `gl_VertexID` (attribute-less rendering, valid in WebGL 2). The
/// scene is fed through `vec4` uniform arrays: `u_view_proj[4]` is the
/// column-major view-projection matrix, `u_camera_pos` the camera
/// position, and `u_cubes[cube * 4 + {0..3}]` the per-cube rotation
/// quaternion, position + scale, face color, and edge color.
pub(crate) const GAME_3D_WEBGL_VERTEX_SHADER: &str = r#"#version 300 es

uniform vec4 u_view_proj[4];
uniform vec4 u_camera_pos;
uniform vec4 u_cubes[32];

out vec2 v_uv;
out vec3 v_normal;
out vec3 v_world_pos;
out vec3 v_face_color;
out vec3 v_edge_color;

vec3 quat_rotate(vec4 q, vec3 v) {
    return v + 2.0 * cross(q.xyz, cross(q.xyz, v) + q.w * v);
}

void main() {
    vec3 corners[8] = vec3[8](
        vec3(-1.0, -1.0, -1.0),
        vec3(1.0, -1.0, -1.0),
        vec3(1.0, 1.0, -1.0),
        vec3(-1.0, 1.0, -1.0),
        vec3(-1.0, -1.0, 1.0),
        vec3(1.0, -1.0, 1.0),
        vec3(1.0, 1.0, 1.0),
        vec3(-1.0, 1.0, 1.0)
    );
    int face_indices[24] = int[24](
        0, 3, 2, 1,
        4, 5, 6, 7,
        0, 1, 5, 4,
        2, 3, 7, 6,
        0, 4, 7, 3,
        1, 2, 6, 5
    );
    vec3 face_normals[6] = vec3[6](
        vec3(0.0, 0.0, -1.0),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, -1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(-1.0, 0.0, 0.0),
        vec3(1.0, 0.0, 0.0)
    );
    int corner_map[6] = int[6](0, 1, 2, 0, 2, 3);
    vec2 corner_uvs[6] = vec2[6](
        vec2(0.0, 0.0),
        vec2(1.0, 0.0),
        vec2(1.0, 1.0),
        vec2(0.0, 0.0),
        vec2(1.0, 1.0),
        vec2(0.0, 1.0)
    );
    int cube_index = gl_VertexID / 36;
    int face_index = (gl_VertexID % 36) / 6;
    int corner_index = gl_VertexID % 6;
    vec4 rotation = u_cubes[cube_index * 4];
    vec4 pos_scale = u_cubes[cube_index * 4 + 1];
    vec4 face_color = u_cubes[cube_index * 4 + 2];
    vec4 edge_color = u_cubes[cube_index * 4 + 3];
    vec3 local = corners[face_indices[face_index * 4 + corner_map[corner_index]]]
        * pos_scale.w;
    vec3 world = quat_rotate(rotation, local) + pos_scale.xyz;
    mat4 view_proj = mat4(u_view_proj[0], u_view_proj[1], u_view_proj[2], u_view_proj[3]);
    gl_Position = view_proj * vec4(world, 1.0);
    v_uv = corner_uvs[corner_index];
    v_normal = quat_rotate(rotation, face_normals[face_index]);
    v_world_pos = world;
    v_face_color = face_color.rgb;
    v_edge_color = edge_color.rgb;
}
"#;

/// The GLSL ES 3.00 fragment shader source for the 3D WebGL cubes demo.
///
/// Mirrors the WGSL fragment shader: discards back faces and overlays
/// the wireframe via a screen-space distance-to-edge test built on
/// `fwidth` of the per-quad UV coordinates.
pub(crate) const GAME_3D_WEBGL_FRAGMENT_SHADER: &str = r#"#version 300 es

precision highp float;

uniform vec4 u_camera_pos;

in vec2 v_uv;
in vec3 v_normal;
in vec3 v_world_pos;
in vec3 v_face_color;
in vec3 v_edge_color;

out vec4 out_color;

void main() {
    vec3 view_dir = normalize(v_world_pos - u_camera_pos.xyz);
    if (dot(v_normal, view_dir) >= 0.0) {
        discard;
    }
    vec2 fw = fwidth(v_uv);
    float dist_x = min(v_uv.x, 1.0 - v_uv.x) / max(fw.x, 1e-6);
    float dist_y = min(v_uv.y, 1.0 - v_uv.y) / max(fw.y, 1e-6);
    if (min(dist_x, dist_y) < 0.75) {
        out_color = vec4(v_edge_color, 1.0);
        return;
    }
    out_color = vec4(v_face_color, 1.0);
}
"#;

/// Quarter-circle angle in radians, mirroring `FRAC_PI_2`.
pub(crate) const GAME_3D_HALF_PI: f64 = PI * 0.5;
