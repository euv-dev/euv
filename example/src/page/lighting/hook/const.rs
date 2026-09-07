/// The HTML `id` attribute value for the standalone Lighting demo canvas element.
pub(crate) const LIGHTING_CANVAS_ID: &str = "lighting-canvas";

/// The CSS selector used to query the Lighting demo canvas element from the DOM.
pub(crate) const LIGHTING_CANVAS_SELECTOR: &str = "#lighting-canvas";

/// The HTML `id` attribute value for the Lighting Canvas 2D loading overlay canvas.
///
/// Mirrors `LIGHTING_WEBGL_LOADING_CANVAS_ID` /
/// `LIGHTING_WEBGPU_LOADING_CANVAS_ID` so the Canvas 2D tab shares the same
/// `c_game_loading_overlay` UX as the two GPU-backed tabs. The overlay paints
/// a centered "Initializing..." line on top of the raytrace canvas during the
/// SSAA acquire + first warmup frame the CPU tab spends before its render
/// loop produces the first paint.
pub(crate) const LIGHTING_LOADING_CANVAS_ID: &str = "lighting-loading-canvas";

/// The CSS selector for the Lighting Canvas 2D loading overlay canvas.
pub(crate) const LIGHTING_LOADING_CANVAS_SELECTOR: &str = "#lighting-loading-canvas";

/// The Canvas 2D context type identifier passed to `HTMLCanvasElement::get_context`.
pub(crate) const LIGHTING_CONTEXT_TYPE: &str = "2d";

/// Logical width of the Lighting page's scene coordinate space.
///
/// The scene (sphere centres, radii, ground row, lamp position) is
/// authored in this fixed 320x240 logical space; the Canvas 2D backing
/// buffer is sized `320 * scale` by `240 * scale` and samples the
/// logical scene, so adaptive resolution changes nothing but the
/// sampling density. The CSS box scales the buffer to fit the visible
/// canvas via the `c_game_3d_canvas` style.
pub(crate) const LIGHTING_WIDTH: f64 = 320.0;

/// Logical height of the Lighting page's scene coordinate space.
pub(crate) const LIGHTING_HEIGHT: f64 = 240.0;

/// Delay in milliseconds before the lighting loop's first `requestAnimationFrame`
/// callback is scheduled, allowing the canvas element to mount before the
/// first frame attempts to acquire a 2D context.
pub(crate) const LIGHTING_LOOP_START_DELAY_MILLIS: i32 = 360;

/// Z position of the eye used as the view direction for the Phong
/// specular term in the Lighting demo.
///
/// The page renders onto a 2D canvas, so we synthesise a fixed
/// "out-of-screen" eye at this Z to keep the specular highlight stable
/// across frames.
pub(crate) const LIGHTING_EYE_Z: f64 = 2.0;

/// The render-scale ladder for the Canvas 2D adaptive-resolution path.
///
/// The backing buffer is sized `320 * scale` by `240 * scale`, so every
/// step keeps the exact 4:3 aspect ratio required by the
/// `c_raytrace_canvas_fullscreen` `object-fit: contain` letterbox
/// contract. All steps produce integer dimensions: 1280x960, 960x720,
/// 800x600, 640x480, 560x420, 480x360, 400x300, 320x240, 240x180,
/// 160x120, 120x90, 80x60. The loop starts at index 7 (scale 1.0) so
/// weak clients never start heavy; the controller climbs toward 4.0
/// only when the frame-time budget allows, so the backing buffer can
/// approach the physical canvas size on strong hardware instead of
/// relying on the browser's smooth upscale.
pub(crate) const LIGHTING_RENDER_SCALES: [f64; 12] = [
    4.0, 3.0, 2.5, 2.0, 1.75, 1.5, 1.25, 1.0, 0.75, 0.5, 0.375, 0.25,
];

/// Exponential-moving-average blend factor for the per-frame CPU render
/// time measurement that drives adaptive resolution.
pub(crate) const LIGHTING_ADAPT_EMA_ALPHA: f64 = 0.1;

/// CPU frame time in milliseconds above which the adaptive-resolution
/// controller steps the render scale down (115% of the 60 FPS budget).
pub(crate) const LIGHTING_ADAPT_SLOW_FRAME_MILLIS: f64 = 16.67 * 1.15;

/// CPU frame time in milliseconds below which the adaptive-resolution
/// controller steps the render scale up one rung (75% of the 60 FPS
/// budget).
pub(crate) const LIGHTING_ADAPT_FAST_FRAME_MILLIS: f64 = 16.67 * 0.75;

/// CPU frame time in milliseconds below which the adaptive-resolution
/// controller steps the render scale up two rungs at once (45% of the
/// 60 FPS budget), skipping intermediate rungs when the headroom is
/// obvious.
pub(crate) const LIGHTING_ADAPT_VERY_FAST_FRAME_MILLIS: f64 = 16.67 * 0.45;

/// Number of consecutive slow frames required before stepping the render
/// scale down one notch.
pub(crate) const LIGHTING_ADAPT_SLOW_FRAMES: u32 = 30;

/// Number of consecutive fast frames required before stepping the render
/// scale up (one notch, or two notches when the frame time also stayed
/// below [`LIGHTING_ADAPT_VERY_FAST_FRAME_MILLIS`] for the same span).
pub(crate) const LIGHTING_ADAPT_FAST_FRAMES: u32 = 45;

/// The HTML `id` attribute value for the Lighting WebGL canvas element.
pub(crate) const LIGHTING_WEBGL_CANVAS_ID: &str = "lighting-webgl-canvas";

/// The CSS selector used to query the Lighting WebGL canvas element.
pub(crate) const LIGHTING_WEBGL_CANVAS_SELECTOR: &str = "#lighting-webgl-canvas";

/// The HTML `id` attribute value for the Lighting WebGL loading overlay canvas.
pub(crate) const LIGHTING_WEBGL_LOADING_CANVAS_ID: &str = "lighting-webgl-loading-canvas";

/// The CSS selector for the Lighting WebGL loading overlay canvas.
pub(crate) const LIGHTING_WEBGL_LOADING_CANVAS_SELECTOR: &str = "#lighting-webgl-loading-canvas";

/// The HTML `id` attribute value for the Lighting WebGPU canvas element.
pub(crate) const LIGHTING_WEBGPU_CANVAS_ID: &str = "lighting-webgpu-canvas";

/// The CSS selector used to query the Lighting WebGPU canvas element.
pub(crate) const LIGHTING_WEBGPU_CANVAS_SELECTOR: &str = "#lighting-webgpu-canvas";

/// The HTML `id` attribute value for the Lighting WebGPU loading overlay canvas.
pub(crate) const LIGHTING_WEBGPU_LOADING_CANVAS_ID: &str = "lighting-webgpu-loading-canvas";

/// The CSS selector for the Lighting WebGPU loading overlay canvas.
pub(crate) const LIGHTING_WEBGPU_LOADING_CANVAS_SELECTOR: &str = "#lighting-webgpu-loading-canvas";

/// The number of `vec4` slots in the GPU uniform block shared by the
/// WebGL and WebGPU lighting shaders: canvas resolution and background
/// color.
pub(crate) const LIGHTING_GPU_UNIFORM_VEC4_COUNT: usize = 2;

/// The GLSL ES 3.00 vertex shader source for the Lighting WebGL demo.
///
/// Attribute-less fullscreen triangle generated from `gl_VertexID`, the
/// same pattern the 3D game page uses for its WebGL programs.
pub(crate) const LIGHTING_WEBGL_VERTEX_SHADER: &str = r#"#version 300 es

void main() {
    vec2 positions[3] = vec2[3](
        vec2(-1.0, -1.0),
        vec2(3.0, -1.0),
        vec2(-1.0, 3.0)
    );
    gl_Position = vec4(positions[gl_VertexID], 0.0, 1.0);
}
"#;

/// The GLSL ES 3.00 fragment shader source for the Lighting WebGL demo.
///
/// Shades the exact same analytic 2D scene as the Canvas 2D software
/// path: five circles plus the ground row, authored in the fixed
/// 320x240 logical space, lit by one directional sun and one point
/// lamp. The per-pixel math mirrors `LightingUniforms::shade` term for
/// term (including the directional-light-no-shadow and
/// eye-distance-falloff quirks) and applies the same `1/2.2` gamma
/// curve. Antialiasing is a physical-resolution 2x2 SSAA: each fragment
/// takes four sub-samples at physical-pixel offsets, converts each one
/// to logical scene coordinates through the letterbox transform, and
/// evaluates the scene analytically at that exact point (painter's
/// order per sub-sample: background -> ground band -> spheres
/// back-to-front), so edges stay smooth at any canvas backing
/// resolution. The scene is letterboxed into the canvas with a uniform
/// scale (never stretched); out-of-scene sub-samples show the canvas
/// background color uploaded in `u_params[1]`, matching the
/// transparent-cleared Canvas 2D tab.
pub(crate) const LIGHTING_WEBGL_FRAGMENT_SHADER: &str = r#"#version 300 es

precision highp float;

uniform vec4 u_params[2];

out vec4 out_color;

// Mirrors the engine's math EPSILON.
const float EPS = 1e-6;
// Logical scene dimensions (the CPU scene is authored in 320x240).
const float SCENE_W = 320.0;
const float SCENE_H = 240.0;
// Ground row: logical y in [187, 188), matching `(240 * 0.78) as i32`.
const float GROUND_Y = 187.0;
const vec3 AMBIENT = vec3(0.08, 0.08, 0.10);
const vec3 EYE = vec3(0.0, 0.0, 2.0);
const vec3 SUN_DIR_RAW = vec3(-0.45, -0.55, -0.70);
const vec3 SUN_COLOR = vec3(1.00, 0.95, 0.85);
// Lamp moved on-screen (top-left, slightly forward in Z) so the
// ray overlay below has a visible ray origin. Must match the lamp
// position in `build_lighting_scene` (example/src/page/lighting/hook/
// lighting_fn.rs) and the WGSL `LAMP_POS` constant.
const vec3 LAMP_POS = vec3(25.6, 43.2, 0.5);
const vec3 LAMP_COLOR = vec3(0.40, 0.70, 1.00);
const float LAMP_INTENSITY = 1.4;
const float LAMP_FALLOFF = 1.0;
// Mirrors the engine's LIGHTING_POINT_LIGHT_MIN_DISTANCE.
const float POINT_MIN_DIST = 0.001;

// Ray overlay constants. The 5 sphere centres are duplicated here
// so the fragment shader can draw a yellow-tinted line segment from
// the lamp to each centre, matching the Bresenham pass on the
// Canvas 2D tab. Logical pixels (SCENE_W x SCENE_H space); the
// blend threshold is `1.6` logical pixels so adaptive resolution
// scales the rays proportionally.
const float RAY_BLEND_DIST = 1.6;
const float RAY_ALPHA = 0.38;
// Sun-disk marker anchor (top-right corner): the directional sun
// has no position, so we render a small bright disk at a fixed
// visible spot to match the Canvas 2D tab.
const vec2 SUN_DISK_POS = vec2(297.6, 28.8);
const float SUN_DISK_RADIUS = 4.0;

// Sphere records: (center x, center y, radius) in logical pixels.
vec3 sphere_record(int index) {
    if (index == 0) { return vec3(SCENE_W * 0.22, SCENE_H * 0.42, 24.0); }
    if (index == 1) { return vec3(SCENE_W * 0.42, SCENE_H * 0.55, 18.0); }
    if (index == 2) { return vec3(SCENE_W * 0.62, SCENE_H * 0.40, 22.0); }
    if (index == 3) { return vec3(SCENE_W * 0.78, SCENE_H * 0.62, 16.0); }
    return vec3(SCENE_W * 0.50, SCENE_H * 0.20, 12.0);
}

vec3 sphere_albedo(int index) {
    if (index == 0) { return vec3(0.85, 0.20, 0.20); }
    if (index == 1) { return vec3(0.20, 0.80, 0.30); }
    if (index == 2) { return vec3(0.25, 0.45, 0.95); }
    if (index == 3) { return vec3(0.95, 0.85, 0.20); }
    return vec3(0.85, 0.25, 0.75);
}

float sphere_specular(int index) {
    if (index == 0) { return 0.5; }
    if (index == 1) { return 0.6; }
    if (index == 2) { return 0.4; }
    if (index == 3) { return 0.7; }
    return 0.0;
}

float sphere_shininess(int index) {
    if (index == 0) { return 24.0; }
    if (index == 1) { return 32.0; }
    if (index == 2) { return 18.0; }
    if (index == 3) { return 48.0; }
    return 32.0;
}

// Returns the perpendicular distance from point `p` to the line
// segment [a, b] in 2D. The clamped projection keeps the segment
// finite so the lamp-to-sphere rays stop at the sphere centre rather
// than extending infinitely.
float distance_to_segment(vec2 p, vec2 a, vec2 b) {
    vec2 ab = b - a;
    vec2 ap = p - a;
    float t = clamp(dot(ap, ab) / max(dot(ab, ab), 1e-6), 0.0, 1.0);
    vec2 proj = a + ab * t;
    return length(p - proj);
}

// Mirrors engine `LightingUniforms::shade` for the lighting scene's two
// lights: directional sun (shadow unconditionally 1.0) and point lamp
// (shadow 1.0 because the scene passes an empty occluder list). The
// specular intensity of both lights is scaled by
// `apply_falloff(view_dist, falloff)` with the distance to the EYE,
// matching the engine quirk; the sun's falloff is 0.0 (no-op) and the
// lamp's is 1.0.
vec3 shade(vec3 position, vec3 normal, vec3 albedo, float specular, float shininess) {
    vec3 color = AMBIENT;
    vec3 to_eye = EYE - position;
    float view_dist = length(to_eye);
    vec3 view_dir = vec3(0.0);
    if (view_dist > EPS) {
        view_dir = to_eye / view_dist;
    }
    {
        vec3 l = normalize(SUN_DIR_RAW);
        float cos_term = max(dot(normal, l), 0.0);
        vec3 diffuse = SUN_COLOR * cos_term * albedo;
        vec3 spec = vec3(0.0);
        if (specular > 0.0) {
            vec3 reflect_dir = normalize(l - normal * (2.0 * dot(l, normal)));
            float spec_factor = pow(max(dot(reflect_dir, view_dir), 0.0), shininess);
            spec = SUN_COLOR * (spec_factor * specular);
        }
        color += diffuse + spec;
    }
    {
        vec3 to_light = LAMP_POS - position;
        float dist = max(length(to_light), POINT_MIN_DIST);
        vec3 l = to_light / dist;
        float cos_term = max(dot(normal, l), 0.0);
        vec3 diffuse = LAMP_COLOR * (LAMP_INTENSITY * cos_term) * albedo;
        vec3 spec = vec3(0.0);
        if (specular > 0.0) {
            float spec_intensity = LAMP_INTENSITY / (1.0 + LAMP_FALLOFF * view_dist * view_dist);
            vec3 reflect_dir = normalize(l - normal * (2.0 * dot(l, normal)));
            float spec_factor = pow(max(dot(reflect_dir, view_dir), 0.0), shininess);
            spec = LAMP_COLOR * (spec_intensity * spec_factor * specular);
        }
        color += diffuse + spec;
    }
    return color;
}

// Evaluates the analytic scene at one logical-space point, following
// the CPU path's painter order exactly: background first, then the
// ground band, then the spheres back-to-front (index 0..4, each
// overwriting whatever came before when the point falls inside it).
vec3 scene_color(vec2 logical, vec3 background) {
    if (logical.x < 0.0 || logical.x >= SCENE_W || logical.y < 0.0 || logical.y >= SCENE_H) {
        return background;
    }
    vec3 color = background;
    if (logical.y >= GROUND_Y && logical.y < GROUND_Y + 1.0) {
        color = shade(
            vec3(floor(logical.x), GROUND_Y, 0.0),
            vec3(0.0, -1.0, 0.0),
            vec3(0.55, 0.55, 0.60),
            0.15,
            12.0
        );
    }
    for (int i = 0; i < 5; i++) {
        vec3 record = sphere_record(i);
        float radius = record.z;
        float r2 = radius * radius;
        vec2 d = logical - record.xy;
        float d2 = dot(d, d);
        if (d2 > r2) { continue; }
        float dz = sqrt(max(r2 - d2, 0.0));
        vec3 normal = vec3(d.x / radius, d.y / radius, dz / radius);
        vec3 position = vec3(logical, dz / radius);
        color = shade(
            position,
            normal,
            sphere_albedo(i),
            sphere_specular(i),
            sphere_shininess(i)
        );
    }
    return color;
}

void main() {
    vec2 resolution = u_params[0].xy;
    vec3 background = u_params[1].rgb;
    // Letterbox the fixed 4:3 logical scene into the canvas with a
    // uniform scale so the circles never stretch.
    float viewport_scale = min(resolution.x / SCENE_W, resolution.y / SCENE_H);
    vec2 origin_px = (resolution - vec2(SCENE_W, SCENE_H) * viewport_scale) * 0.5;
    // gl_FragCoord is bottom-up; the logical scene is top-down.
    vec2 frag = vec2(gl_FragCoord.x, resolution.y - gl_FragCoord.y);
    // 2x2 super-sampling at physical fragment resolution: each
    // sub-sample offset is taken in physical pixels and converted to
    // logical scene coordinates individually, so edges anti-alias at
    // the canvas backing resolution on any DPI instead of snapping to
    // the 320x240 logical grid.
    vec2 base = floor(frag);
    vec3 acc = vec3(0.0);
    for (int sy = 0; sy < 2; sy++) {
        for (int sx = 0; sx < 2; sx++) {
            vec2 sample_px = base + vec2(0.25 + float(sx) * 0.5, 0.25 + float(sy) * 0.5);
            vec2 logical = (sample_px - origin_px) / viewport_scale;
            vec3 sample_color = scene_color(logical, background);
            // Ray overlay per sub-sample: each ray is a line segment
            // from the lamp's top-left anchor to one of the 5 sphere
            // centres. When a sub-sample falls within RAY_BLEND_DIST
            // logical pixels of any ray, blend the lamp's color over
            // the shaded scene at RAY_ALPHA strength so the sphere
            // still reads through. The sun-disk marker is a separate
            // filled disk at the top-right anchor.
            vec2 lamp_anchor = LAMP_POS.xy;
            for (int i = 0; i < 5; i++) {
                vec3 record = sphere_record(i);
                float d = distance_to_segment(logical, lamp_anchor, record.xy);
                if (d < RAY_BLEND_DIST) {
                    float a = RAY_ALPHA * (1.0 - d / RAY_BLEND_DIST);
                    sample_color = mix(sample_color, LAMP_COLOR, a);
                }
            }
            float sun_d = length(logical - SUN_DISK_POS);
            if (sun_d < SUN_DISK_RADIUS) {
                sample_color = mix(sample_color, vec3(0.95, 0.95, 1.0), 0.85);
            }
            acc += sample_color;
        }
    }
    vec3 linear = acc * 0.25;
    vec3 gamma = pow(clamp(linear, vec3(0.0), vec3(1.0)), vec3(1.0 / 2.2));
    out_color = vec4(gamma, 1.0);
}
"#;

/// The WGSL shader source for the Lighting WebGPU demo.
///
/// Mirrors [`LIGHTING_WEBGL_FRAGMENT_SHADER`]: the same analytic 2D
/// scene (five circles + ground row, sun + point lamp), the same
/// `LightingUniforms::shade` math, the same physical-resolution 2x2
/// SSAA (each sub-sample offset is taken in physical pixels and
/// converted to logical scene coordinates individually) and `1/2.2`
/// gamma, and the same uniform-scale letterbox into the canvas. The
/// fullscreen triangle is generated from `@builtin(vertex_index)` and
/// the per-frame resolution / background data arrives in a 2-`vec4`
/// uniform buffer at `@group(0) @binding(0)`. WebGPU fragment positions
/// are top-left origin, so no y-flip is needed (unlike the WebGL
/// variant).
pub(crate) const LIGHTING_WEBGPU_SHADER: &str = r#"
struct SceneUniforms {
    resolution: vec4<f32>,
    background: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u_scene: SceneUniforms;

// Mirrors the engine's math EPSILON.
const EPS: f32 = 1e-6;
// Logical scene dimensions (the CPU scene is authored in 320x240).
const SCENE_W: f32 = 320.0;
const SCENE_H: f32 = 240.0;
// Ground row: logical y in [187, 188), matching `(240 * 0.78) as i32`.
const GROUND_Y: f32 = 187.0;
const AMBIENT = vec3<f32>(0.08, 0.08, 0.10);
const EYE = vec3<f32>(0.0, 0.0, 2.0);
const SUN_DIR_RAW = vec3<f32>(-0.45, -0.55, -0.70);
const SUN_COLOR = vec3<f32>(1.00, 0.95, 0.85);
// Lamp moved on-screen (top-left, slightly forward in Z) so the
// ray overlay below has a visible ray origin. Must match the lamp
// position in `build_lighting_scene` (example/src/page/lighting/hook/
// lighting_fn.rs) and the GLSL `LAMP_POS` constant.
const LAMP_POS = vec3<f32>(25.6, 43.2, 0.5);
const LAMP_COLOR = vec3<f32>(0.40, 0.70, 1.00);
const LAMP_INTENSITY: f32 = 1.4;
const LAMP_FALLOFF: f32 = 1.0;
// Mirrors the engine's LIGHTING_POINT_LIGHT_MIN_DISTANCE.
const POINT_MIN_DIST: f32 = 0.001;

// Ray overlay constants. The 5 sphere centres are duplicated here
// so the fragment shader can draw a yellow-tinted line segment from
// the lamp to each centre, matching the Bresenham pass on the
// Canvas 2D tab and the GLSL distance-to-segment pass. Logical
// pixels (SCENE_W x SCENE_H space); the blend threshold is `1.6`
// logical pixels so adaptive resolution scales the rays
// proportionally.
const RAY_BLEND_DIST: f32 = 1.6;
const RAY_ALPHA: f32 = 0.38;
// Sun-disk marker anchor (top-right corner): the directional sun
// has no position, so we render a small bright disk at a fixed
// visible spot to match the Canvas 2D / WebGL tabs.
const SUN_DISK_POS = vec2<f32>(297.6, 28.8);
const SUN_DISK_RADIUS: f32 = 4.0;

// Sphere records: (center x, center y, radius) in logical pixels.
fn sphere_record(index: i32) -> vec3<f32> {
    if index == 0 { return vec3<f32>(SCENE_W * 0.22, SCENE_H * 0.42, 24.0); }
    if index == 1 { return vec3<f32>(SCENE_W * 0.42, SCENE_H * 0.55, 18.0); }
    if index == 2 { return vec3<f32>(SCENE_W * 0.62, SCENE_H * 0.40, 22.0); }
    if index == 3 { return vec3<f32>(SCENE_W * 0.78, SCENE_H * 0.62, 16.0); }
    return vec3<f32>(SCENE_W * 0.50, SCENE_H * 0.20, 12.0);
}

fn sphere_albedo(index: i32) -> vec3<f32> {
    if index == 0 { return vec3<f32>(0.85, 0.20, 0.20); }
    if index == 1 { return vec3<f32>(0.20, 0.80, 0.30); }
    if index == 2 { return vec3<f32>(0.25, 0.45, 0.95); }
    if index == 3 { return vec3<f32>(0.95, 0.85, 0.20); }
    return vec3<f32>(0.85, 0.25, 0.75);
}

fn sphere_specular(index: i32) -> f32 {
    if index == 0 { return 0.5; }
    if index == 1 { return 0.6; }
    if index == 2 { return 0.4; }
    if index == 3 { return 0.7; }
    return 0.0;
}

fn sphere_shininess(index: i32) -> f32 {
    if index == 0 { return 24.0; }
    if index == 1 { return 32.0; }
    if index == 2 { return 18.0; }
    if index == 3 { return 48.0; }
    return 32.0;
}

// Returns the perpendicular distance from point `p` to the line
// segment [a, b] in 2D. The clamped projection keeps the segment
// finite so the lamp-to-sphere rays stop at the sphere centre rather
// than extending infinitely.
fn distance_to_segment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let t = clamp(dot(ap, ab) / max(dot(ab, ab), 1e-6), 0.0, 1.0);
    let proj = a + ab * t;
    return length(p - proj);
}

// Mirrors engine `LightingUniforms::shade` for the lighting scene's two
// lights: directional sun (shadow unconditionally 1.0) and point lamp
// (shadow 1.0 because the scene passes an empty occluder list). The
// specular intensity of both lights is scaled by
// `apply_falloff(view_dist, falloff)` with the distance to the EYE,
// matching the engine quirk; the sun's falloff is 0.0 (no-op) and the
// lamp's is 1.0.
fn shade(position: vec3<f32>, normal: vec3<f32>, albedo: vec3<f32>, specular: f32, shininess: f32) -> vec3<f32> {
    var color = AMBIENT;
    let to_eye = EYE - position;
    let view_dist = length(to_eye);
    var view_dir = vec3<f32>(0.0);
    if view_dist > EPS {
        view_dir = to_eye / view_dist;
    }
    {
        let l = normalize(SUN_DIR_RAW);
        let cos_term = max(dot(normal, l), 0.0);
        let diffuse = SUN_COLOR * (cos_term * albedo);
        var spec = vec3<f32>(0.0);
        if specular > 0.0 {
            let reflect_dir = normalize(l - normal * (2.0 * dot(l, normal)));
            let spec_factor = pow(max(dot(reflect_dir, view_dir), 0.0), shininess);
            spec = SUN_COLOR * (spec_factor * specular);
        }
        color += diffuse + spec;
    }
    {
        let to_light = LAMP_POS - position;
        let dist = max(length(to_light), POINT_MIN_DIST);
        let l = to_light / dist;
        let cos_term = max(dot(normal, l), 0.0);
        let diffuse = LAMP_COLOR * (LAMP_INTENSITY * cos_term) * albedo;
        var spec = vec3<f32>(0.0);
        if specular > 0.0 {
            let spec_intensity = LAMP_INTENSITY / (1.0 + LAMP_FALLOFF * view_dist * view_dist);
            let reflect_dir = normalize(l - normal * (2.0 * dot(l, normal)));
            let spec_factor = pow(max(dot(reflect_dir, view_dir), 0.0), shininess);
            spec = LAMP_COLOR * (spec_intensity * spec_factor * specular);
        }
        color += diffuse + spec;
    }
    return color;
}

// Evaluates the analytic scene at one logical-space point, following
// the CPU path's painter order exactly: background first, then the
// ground band, then the spheres back-to-front (index 0..4, each
// overwriting whatever came before when the point falls inside it).
fn scene_color(logical: vec2<f32>, background: vec3<f32>) -> vec3<f32> {
    if logical.x < 0.0 || logical.x >= SCENE_W || logical.y < 0.0 || logical.y >= SCENE_H {
        return background;
    }
    var color = background;
    if logical.y >= GROUND_Y && logical.y < GROUND_Y + 1.0 {
        color = shade(
            vec3<f32>(floor(logical.x), GROUND_Y, 0.0),
            vec3<f32>(0.0, -1.0, 0.0),
            vec3<f32>(0.55, 0.55, 0.60),
            0.15,
            12.0,
        );
    }
    for (var i = 0; i < 5; i++) {
        let record = sphere_record(i);
        let radius = record.z;
        let r2 = radius * radius;
        let d = logical - record.xy;
        let d2 = dot(d, d);
        if d2 > r2 { continue; }
        let dz = sqrt(max(r2 - d2, 0.0));
        let normal = vec3<f32>(d.x / radius, d.y / radius, dz / radius);
        let position = vec3<f32>(logical, dz / radius);
        color = shade(
            position,
            normal,
            sphere_albedo(i),
            sphere_specular(i),
            sphere_shininess(i),
        );
    }
    return color;
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    return vec4<f32>(positions[vi], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) frag_pos: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution = u_scene.resolution.xy;
    let background = u_scene.background.rgb;
    // Letterbox the fixed 4:3 logical scene into the canvas with a
    // uniform scale so the circles never stretch. WebGPU fragment
    // positions are top-left origin, matching the logical scene's
    // top-down y axis directly.
    let viewport_scale = min(resolution.x / SCENE_W, resolution.y / SCENE_H);
    let origin_px = (resolution - vec2<f32>(SCENE_W, SCENE_H) * viewport_scale) * 0.5;
    // 2x2 super-sampling at physical fragment resolution: each
    // sub-sample offset is taken in physical pixels and converted to
    // logical scene coordinates individually, so edges anti-alias at
    // the canvas backing resolution on any DPI instead of snapping to
    // the 320x240 logical grid.
    let base = floor(frag_pos.xy);
    var acc = vec3<f32>(0.0);
    for (var sy = 0; sy < 2; sy++) {
        for (var sx = 0; sx < 2; sx++) {
            let sample_px = base + vec2<f32>(0.25 + f32(sx) * 0.5, 0.25 + f32(sy) * 0.5);
            let logical = (sample_px - origin_px) / viewport_scale;
            var sample_color = scene_color(logical, background);
            // Ray overlay per sub-sample: each ray is a line segment
            // from the lamp's top-left anchor to one of the 5 sphere
            // centres. When a sub-sample falls within RAY_BLEND_DIST
            // logical pixels of any ray, blend the lamp's color over
            // the shaded scene at RAY_ALPHA strength so the sphere
            // still reads through. The sun-disk marker is a separate
            // filled disk at the top-right anchor.
            let lamp_anchor = LAMP_POS.xy;
            for (var i = 0; i < 5; i = i + 1) {
                let record = sphere_record(i);
                let d = distance_to_segment(logical, lamp_anchor, record.xy);
                if (d < RAY_BLEND_DIST) {
                    let a = RAY_ALPHA * (1.0 - d / RAY_BLEND_DIST);
                    sample_color = mix(sample_color, LAMP_COLOR, a);
                }
            }
            let sun_d = length(logical - SUN_DISK_POS);
            if (sun_d < SUN_DISK_RADIUS) {
                sample_color = mix(sample_color, vec3<f32>(0.95, 0.95, 1.0), 0.85);
            }
            acc += sample_color;
        }
    }
    let linear = acc * 0.25;
    let gamma = pow(clamp(linear, vec3<f32>(0.0), vec3<f32>(1.0)), vec3<f32>(1.0 / 2.2));
    return vec4<f32>(gamma, 1.0);
}
"#;
