/// The HTML `id` attribute value for the 2D game canvas element.
pub(crate) const GAME_2D_CANVAS_ID: &str = "game-2d-canvas";

/// The CSS selector used to query the 2D game canvas element from the DOM.
pub(crate) const GAME_2D_CANVAS_SELECTOR: &str = "#game-2d-canvas";

/// The default canvas width in CSS pixels.
pub(crate) const GAME_2D_CANVAS_WIDTH: f64 = 600.0;

/// The default canvas height in CSS pixels.
pub(crate) const GAME_2D_CANVAS_HEIGHT: f64 = 400.0;

/// The gravitational acceleration in pixels per second squared.
pub(crate) const GAME_2D_GRAVITY: f64 = 600.0;

/// The minimum radius of a ball in CSS pixels (relative to a 600px canvas;
/// the runtime scales by `canvas_width / GAME_2D_CANVAS_WIDTH`).
///
/// The Canvas 2D tab renders through a 2x SSAA backing store and then
/// downscales, so a radius of `N` CSS units anti-aliases over `2 * N`
/// physical pixels and ends up visually close to `N` CSS pixels. The
/// WebGL / WebGPU tabs render directly in the physical backing pixel
/// space (their shaders see `ball.radius` as a CSS-unit number but draw
/// it in clip space without DPR scaling), so at DPR=2 a ball there ends
/// up half the visual size of the Canvas 2D path. The shader-time
/// `dpr` multiplier in `game_2d_ball_gpu_record` corrects that mismatch.
///
/// The lower bound is intentionally small so 100 balls can stack
/// comfortably in the 600x400 inline canvas without triggering the
/// `GAME_2D_MAX_BALL_AREA_RATIO` jam path.
pub(crate) const GAME_2D_BALL_MIN_RADIUS: f64 = 4.0;

/// The maximum radius of a ball in CSS pixels (relative to a 600px
/// canvas; the runtime scales by `canvas_width / GAME_2D_CANVAS_WIDTH`).
///
/// See [`GAME_2D_BALL_MIN_RADIUS`] for the SSAA / DPR sizing rationale.
/// The upper bound is chosen so that 100 balls of the maximum radius
/// still fit in the 600x400 inline canvas with room to spare: with a
/// 14 CSS-pixel max radius, even a single-column stack needs only
/// `28 * 4 ~= 112 px` of vertical room.
pub(crate) const GAME_2D_BALL_MAX_RADIUS: f64 = 14.0;

/// The restitution (bounciness) coefficient for wall and ball collisions.
pub(crate) const GAME_2D_RESTITUTION: f64 = 0.85;

/// The linear damping coefficient applied per second to simulate air resistance.
pub(crate) const GAME_2D_LINEAR_DAMPING: f64 = 0.1;

/// The initial upward velocity magnitude when spawning a ball.
pub(crate) const GAME_2D_SPAWN_VELOCITY: f64 = 200.0;

/// The fixed timestep for the 2D game loop in seconds (60 FPS).
pub(crate) const GAME_2D_FIXED_TIMESTEP: f64 = 1.0 / 60.0;

/// The maximum number of balls allowed simultaneously.
pub(crate) const GAME_2D_MAX_BALLS: usize = 100;

/// The debounce interval in milliseconds for the resize event handler.
pub(crate) const GAME_2D_RESIZE_DEBOUNCE_MILLIS: i32 = 100;

/// The delay in milliseconds before starting the 2D game loop after page mount.
///
/// Defers the heavy `requestAnimationFrame` rendering loop to avoid competing
/// with the mobile drawer close animation for main thread time, preventing
/// sidebar animation stutter on page transitions.
pub(crate) const GAME_2D_LOOP_START_DELAY_MILLIS: i32 = 360;

/// The JavaScript property name for the canvas fill style.
pub(crate) const GAME_2D_PROPERTY_FILL_STYLE: &str = "fillStyle";

/// The CSS property name for the computed background colour, used to fill
/// the loading overlay so the scene does not bleed through.
pub(crate) const GAME_2D_PROPERTY_BACKGROUND_COLOR: &str = "background-color";

/// The loading text displayed on the canvas before the game loop starts.
pub(crate) const GAME_2D_LOADING_TEXT: &str = "Loading...";

/// The CSS font family used for the loading text on the canvas.
pub(crate) const GAME_2D_LOADING_FONT_FAMILY: &str = "sans-serif";

/// The ratio of the loading font size to the canvas height.
pub(crate) const GAME_2D_LOADING_FONT_SIZE_RATIO: f64 = 0.04;

/// The CSS variable name for the loading text color on the canvas.
///
/// Uses `--text-on-accent` because the canvas background is `var!(accent)`,
/// and `text-on-accent` is the theme variable that contrasts with the accent
/// color (foreground/background equal accent in this monochrome design).
pub(crate) const GAME_2D_LOADING_COLOR_VAR: &str = "--text-on-accent";

/// The minimum time in milliseconds the loading overlay stays visible.
///
/// Fast init paths (notably synchronous WebGL init) would otherwise add and
/// remove the overlay canvas within a single frame, so the browser never
/// paints the loading state on tab switches.
pub(crate) const GAME_2D_LOADING_MIN_MILLIS: i32 = 400;

/// The palette of ball colors used for random color assignment.
pub(crate) const GAME_2D_BALL_COLORS: &[&str] = &[
    "#e94560", "#0f3460", "#16c79a", "#f5b461", "#ec524b", "#41b883", "#6c5ce7", "#fd79a8",
    "#00cec9", "#fab1a0",
];

/// The number of physics substeps performed per fixed timestep.
///
/// Splits the fixed `1/60s` timestep into smaller slices so a fast ball never
/// moves more than its own radius between collision checks, preventing the
/// "tunneling" effect where one ball passes through another in a single step.
pub(crate) const GAME_2D_PHYSICS_SUBSTEPS: usize = 4;

/// The number of ball-to-ball collision resolution passes per substep.
///
/// A single pass is insufficient when a ball is squeezed between two or more
/// other balls: resolving the overlap with ball A can leave the ball still
/// overlapping with ball C. Repeating the pass lets the correction propagate
/// through the contact graph until every overlap is cleared (or a stable
/// stacking configuration is reached).
pub(crate) const GAME_2D_COLLISION_ITERATIONS: usize = 4;

/// The maximum fraction of canvas area that the combined ball cross-sections
/// may occupy at any given canvas size.
///
/// When a fullscreen -> inline transition leaves the ball list unchanged,
/// the total ball area (`pi * sum(radius_i^2)`) is recomputed against the
/// smaller canvas area (`width * height`); if the ratio exceeds this
/// threshold, the oldest balls are trimmed from the front of the list
/// until the ratio is at or below the limit. Without this trim, balls
/// pile up against the floor of the smaller canvas because the impulse
/// solver cannot fit more than ~`area_ratio / pi_avg_ball_radius^2`
/// balls into a `width * height` rectangle while gravity keeps pushing
/// them down.
///
/// 70% is the empirical upper bound at which the impulse + projection
/// solver can still separate the balls into a stable stacking
/// configuration within `GAME_2D_COLLISION_ITERATIONS`. Going above this
/// re-introduces the "infinite collision jitter" regression (gravity
/// overpowers the impulse-driven separation, balls oscillate vertically,
/// the main thread pegs at 100%).
pub(crate) const GAME_2D_MAX_BALL_AREA_RATIO: f64 = 0.70;

/// The fraction by which a jammed ball's radius is scaled when, after all
/// `GAME_2D_COLLISION_ITERATIONS` resolution passes, persistent overlap
/// remains because the impulse solver could not fit the ball into the
/// available space.
///
/// Applied multiplicatively to the *effective collision radius only* (the
/// `overlap` check uses the original radius; the projection along the
/// contact normal uses `radius * GAME_2D_STUCK_RADIUS_SHRINK` so the
/// shrunk ball slips past its neighbours on the next substep). This is a
/// last-resort convergence tool — under normal density the solver
/// converges without ever reaching this code path. The value `0.97` keeps
/// the visual radius essentially unchanged (3% shrink is below
/// perceptual noise for a `>=8px` ball) but lets the jammed ball find a
/// gap when nothing else works.
pub(crate) const GAME_2D_STUCK_RADIUS_SHRINK: f64 = 0.97;

/// The minimum overlap distance (in pixels) at which the jammed-ball
/// radius-shrink fallback is invoked. Below this distance the impulse
/// solver has effectively converged and shrinking would only introduce
/// instability.
pub(crate) const GAME_2D_STUCK_MIN_OVERLAP: f64 = 0.5;

/// The HTML `id` attribute value for the 2D WebGPU canvas element.
pub(crate) const GAME_2D_WEBGPU_CANVAS_ID: &str = "game-2d-webgpu-canvas";

/// The CSS selector used to query the 2D WebGPU canvas element from the DOM.
pub(crate) const GAME_2D_WEBGPU_CANVAS_SELECTOR: &str = "#game-2d-webgpu-canvas";

/// The HTML `id` attribute value for the 2D WebGPU loading overlay canvas.
/// Renders "Loading..." via a 2D context while the GPU renderer initializes.
pub(crate) const GAME_2D_WEBGPU_LOADING_CANVAS_ID: &str = "game-2d-webgpu-loading-canvas";

/// The CSS selector for the 2D WebGPU loading overlay canvas.
pub(crate) const GAME_2D_WEBGPU_LOADING_CANVAS_SELECTOR: &str = "#game-2d-webgpu-loading-canvas";

/// The WGSL shader source for the 2D WebGPU bouncing balls demo.
///
/// Renders every ball as a camera-facing quad (two triangles per ball,
/// vertices generated procedurally from `@builtin(vertex_index)`) and
/// discards fragments outside the unit circle in the fragment stage.
/// Ball data (center, radius, color) and the logical canvas size are
/// read from the `@group(0) @binding(0)` uniform buffer, which the host
/// refreshes each frame. The draw call issues `ball_count * 6` vertices.
pub(crate) const GAME_2D_WEBGPU_SHADER: &str = r#"
struct BallData {
    pos_radius: vec4<f32>,
    color: vec4<f32>,
};

struct BallsUniforms {
    canvas_size: vec2<f32>,
    _pad: vec2<f32>,
    balls: array<BallData, 100>,
};

@group(0) @binding(0) var<uniform> u_balls: BallsUniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec3<f32>,
};

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
    var corners = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
    );
    let ball = u_balls.balls[vi / 6u];
    let corner = corners[vi % 6u];
    let world = ball.pos_radius.xy + corner * ball.pos_radius.z;
    let clip = vec2<f32>(
        world.x / u_balls.canvas_size.x * 2.0 - 1.0,
        1.0 - world.y / u_balls.canvas_size.y * 2.0,
    );
    var out: VertexOutput;
    out.position = vec4<f32>(clip, 0.0, 1.0);
    out.uv = corner;
    out.color = ball.color.rgb;
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
    if dot(in.uv, in.uv) > 1.0 {
        discard;
    }
    return vec4<f32>(in.color, 1.0);
}
"#;

/// The HTML `id` attribute value for the 2D WebGL canvas element.
pub(crate) const GAME_2D_WEBGL_CANVAS_ID: &str = "game-2d-webgl-canvas";

/// The CSS selector used to query the 2D WebGL canvas element from the DOM.
pub(crate) const GAME_2D_WEBGL_CANVAS_SELECTOR: &str = "#game-2d-webgl-canvas";

/// The HTML `id` attribute value for the 2D WebGL loading overlay canvas.
pub(crate) const GAME_2D_WEBGL_LOADING_CANVAS_ID: &str = "game-2d-webgl-loading-canvas";

/// The CSS selector for the 2D WebGL loading overlay canvas.
pub(crate) const GAME_2D_WEBGL_LOADING_CANVAS_SELECTOR: &str = "#game-2d-webgl-loading-canvas";

/// The GLSL ES 3.00 vertex shader source for the 2D WebGL bouncing balls demo.
///
/// Mirrors the WGSL balls shader: per-ball quads are generated procedurally
/// from `gl_VertexID` (attribute-less rendering, valid in WebGL 2), and the
/// per-frame ball data arrives in `vec4` uniform arrays uploaded via
/// `uniform4fv`. The draw call issues `ball_count * 6` vertices.
pub(crate) const GAME_2D_WEBGL_VERTEX_SHADER: &str = r#"#version 300 es

uniform vec2 u_canvas_size;
uniform vec4 u_ball_pos_radius[100];
uniform vec4 u_ball_color[100];

out vec2 v_uv;
out vec3 v_color;

void main() {
    vec2 corners[6] = vec2[6](
        vec2(-1.0, -1.0),
        vec2(1.0, -1.0),
        vec2(1.0, 1.0),
        vec2(-1.0, -1.0),
        vec2(1.0, 1.0),
        vec2(-1.0, 1.0)
    );
    int ball_index = gl_VertexID / 6;
    vec4 ball = u_ball_pos_radius[ball_index];
    vec2 corner = corners[gl_VertexID % 6];
    vec2 world = ball.xy + corner * ball.z;
    gl_Position = vec4(
        world.x / u_canvas_size.x * 2.0 - 1.0,
        1.0 - world.y / u_canvas_size.y * 2.0,
        0.0,
        1.0
    );
    v_uv = corner;
    v_color = u_ball_color[ball_index].rgb;
}
"#;

/// The GLSL ES 3.00 fragment shader source for the 2D WebGL bouncing balls demo.
///
/// Discards fragments outside the unit circle so each quad renders as a
/// filled ball.
pub(crate) const GAME_2D_WEBGL_FRAGMENT_SHADER: &str = r#"#version 300 es

precision mediump float;

in vec2 v_uv;
in vec3 v_color;

out vec4 out_color;

void main() {
    if (dot(v_uv, v_uv) > 1.0) {
        discard;
    }
    out_color = vec4(v_color, 1.0);
}
"#;
