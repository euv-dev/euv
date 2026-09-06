/// The HTML `id` attribute value for the standalone Lighting demo canvas element.
pub(crate) const LIGHTING_CANVAS_ID: &str = "lighting-canvas";

/// The CSS selector used to query the Lighting demo canvas element from the DOM.
pub(crate) const LIGHTING_CANVAS_SELECTOR: &str = "#lighting-canvas";

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
/// contract. All steps produce integer dimensions: 320x240, 240x180,
/// 160x120, 120x90, 80x60.
pub(crate) const LIGHTING_RENDER_SCALES: [f64; 5] = [1.0, 0.75, 0.5, 0.375, 0.25];

/// Exponential-moving-average blend factor for the per-frame CPU render
/// time measurement that drives adaptive resolution.
pub(crate) const LIGHTING_ADAPT_EMA_ALPHA: f64 = 0.1;

/// CPU frame time in milliseconds above which the adaptive-resolution
/// controller steps the render scale down (115% of the 60 FPS budget).
pub(crate) const LIGHTING_ADAPT_SLOW_FRAME_MILLIS: f64 = 16.67 * 1.15;

/// CPU frame time in milliseconds below which the adaptive-resolution
/// controller steps the render scale up (70% of the 60 FPS budget).
pub(crate) const LIGHTING_ADAPT_FAST_FRAME_MILLIS: f64 = 16.67 * 0.7;

/// Number of consecutive slow frames required before stepping the render
/// scale down one notch.
pub(crate) const LIGHTING_ADAPT_SLOW_FRAMES: u32 = 30;

/// Number of consecutive fast frames required before stepping the render
/// scale up one notch.
pub(crate) const LIGHTING_ADAPT_FAST_FRAMES: u32 = 120;

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
/// eye-distance-falloff quirks), averages the same 2x2 sub-samples per
/// logical pixel, and applies the same `1/2.2` gamma curve. The scene
/// is letterboxed into the canvas with a uniform scale (never
/// stretched); out-of-scene fragments show the canvas background color
/// uploaded in `u_params[1]`, matching the transparent-cleared Canvas
/// 2D tab.
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
const vec3 LAMP_POS = vec3(160.0, -10.0, 1.2);
const vec3 LAMP_COLOR = vec3(0.40, 0.70, 1.00);
const float LAMP_INTENSITY = 1.4;
const float LAMP_FALLOFF = 1.0;
// Mirrors the engine's LIGHTING_POINT_LIGHT_MIN_DISTANCE.
const float POINT_MIN_DIST = 0.001;

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

void main() {
    vec2 resolution = u_params[0].xy;
    vec3 background = u_params[1].rgb;
    // Letterbox the fixed 4:3 logical scene into the canvas with a
    // uniform scale so the circles never stretch.
    float viewport_scale = min(resolution.x / SCENE_W, resolution.y / SCENE_H);
    vec2 origin_px = (resolution - vec2(SCENE_W, SCENE_H) * viewport_scale) * 0.5;
    // gl_FragCoord is bottom-up; the logical scene is top-down.
    vec2 frag = vec2(gl_FragCoord.x, resolution.y - gl_FragCoord.y);
    vec2 logical = (frag - origin_px) / viewport_scale;
    vec3 linear = vec3(0.0);
    bool lit = false;
    if (logical.x >= 0.0 && logical.x < SCENE_W && logical.y >= 0.0 && logical.y < SCENE_H) {
        float lx = floor(logical.x);
        float ly = floor(logical.y);
        if (ly >= GROUND_Y && ly < GROUND_Y + 1.0) {
            linear = shade(
                vec3(lx, GROUND_Y, 0.0),
                vec3(0.0, -1.0, 0.0),
                vec3(0.55, 0.55, 0.60),
                0.15,
                12.0
            );
            lit = true;
        }
        for (int i = 0; i < 5; i++) {
            vec3 record = sphere_record(i);
            float radius = record.z;
            float r2 = radius * radius;
            vec3 acc = vec3(0.0);
            int inside = 0;
            for (int sy = 0; sy < 2; sy++) {
                for (int sx = 0; sx < 2; sx++) {
                    float sample_x = lx + 0.25 + float(sx) * 0.5;
                    float sample_y = ly + 0.25 + float(sy) * 0.5;
                    vec2 d = vec2(sample_x, sample_y) - record.xy;
                    float d2 = dot(d, d);
                    if (d2 > r2) { continue; }
                    inside += 1;
                    float dz = sqrt(max(r2 - d2, 0.0));
                    vec3 normal = vec3(d.x / radius, d.y / radius, dz / radius);
                    vec3 position = vec3(sample_x, sample_y, dz / radius);
                    acc += shade(
                        position,
                        normal,
                        sphere_albedo(i),
                        sphere_specular(i),
                        sphere_shininess(i)
                    );
                }
            }
            if (inside > 0) {
                linear = acc / float(inside);
                lit = true;
            }
        }
    }
    if (!lit) {
        out_color = vec4(background, 1.0);
        return;
    }
    vec3 gamma = pow(clamp(linear, vec3(0.0), vec3(1.0)), vec3(1.0 / 2.2));
    out_color = vec4(gamma, 1.0);
}
"#;

/// The WGSL shader source for the Lighting WebGPU demo.
///
/// Mirrors [`LIGHTING_WEBGL_FRAGMENT_SHADER`]: the same analytic 2D
/// scene (five circles + ground row, sun + point lamp), the same
/// `LightingUniforms::shade` math, the same 2x2 sub-sample averaging
/// and `1/2.2` gamma, and the same uniform-scale letterbox into the
/// canvas. The fullscreen triangle is generated from
/// `@builtin(vertex_index)` and the per-frame resolution / background
/// data arrives in a 2-`vec4` uniform buffer at `@group(0)
/// @binding(0)`. WebGPU fragment positions are top-left origin, so no
/// y-flip is needed (unlike the WebGL variant).
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
const LAMP_POS = vec3<f32>(160.0, -10.0, 1.2);
const LAMP_COLOR = vec3<f32>(0.40, 0.70, 1.00);
const LAMP_INTENSITY: f32 = 1.4;
const LAMP_FALLOFF: f32 = 1.0;
// Mirrors the engine's LIGHTING_POINT_LIGHT_MIN_DISTANCE.
const POINT_MIN_DIST: f32 = 0.001;

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
    let logical = (frag_pos.xy - origin_px) / viewport_scale;
    var linear = vec3<f32>(0.0);
    var lit = false;
    if logical.x >= 0.0 && logical.x < SCENE_W && logical.y >= 0.0 && logical.y < SCENE_H {
        let lx = floor(logical.x);
        let ly = floor(logical.y);
        if ly >= GROUND_Y && ly < GROUND_Y + 1.0 {
            linear = shade(
                vec3<f32>(lx, GROUND_Y, 0.0),
                vec3<f32>(0.0, -1.0, 0.0),
                vec3<f32>(0.55, 0.55, 0.60),
                0.15,
                12.0,
            );
            lit = true;
        }
        for (var i = 0; i < 5; i++) {
            let record = sphere_record(i);
            let radius = record.z;
            let r2 = radius * radius;
            var acc = vec3<f32>(0.0);
            var inside = 0;
            for (var sy = 0; sy < 2; sy++) {
                for (var sx = 0; sx < 2; sx++) {
                    let sample_x = lx + 0.25 + f32(sx) * 0.5;
                    let sample_y = ly + 0.25 + f32(sy) * 0.5;
                    let d = vec2<f32>(sample_x, sample_y) - record.xy;
                    let d2 = dot(d, d);
                    if d2 > r2 { continue; }
                    inside += 1;
                    let dz = sqrt(max(r2 - d2, 0.0));
                    let normal = vec3<f32>(d.x / radius, d.y / radius, dz / radius);
                    let position = vec3<f32>(sample_x, sample_y, dz / radius);
                    acc += shade(
                        position,
                        normal,
                        sphere_albedo(i),
                        sphere_specular(i),
                        sphere_shininess(i),
                    );
                }
            }
            if inside > 0 {
                linear = acc / f32(inside);
                lit = true;
            }
        }
    }
    if !lit {
        return vec4<f32>(background, 1.0);
    }
    let gamma = pow(clamp(linear, vec3<f32>(0.0), vec3<f32>(1.0)), vec3<f32>(1.0 / 2.2));
    return vec4<f32>(gamma, 1.0);
}
"#;
