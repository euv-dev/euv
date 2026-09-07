/// The HTML `id` attribute value for the RayTrace demo canvas element.
pub(crate) const RAYTRACE_CANVAS_ID: &str = "raytrace-canvas";

/// The CSS selector used to query the RayTrace demo canvas element from the DOM.
pub(crate) const RAYTRACE_CANVAS_SELECTOR: &str = "#raytrace-canvas";

/// The HTML `id` attribute value for the RayTrace Canvas 2D loading overlay canvas.
///
/// Mirrors `RAYTRACE_WEBGL_LOADING_CANVAS_ID` / `RAYTRACE_WEBGPU_LOADING_CANVAS_ID`
/// so the three RayTrace tabs share the same `c_game_loading_overlay` UX. The
/// overlay paints a centered "Initializing..." line on top of the raytrace
/// canvas during the 200-400 ms warmup window the Canvas 2D tab spends
/// acquiring the SSAA wrapper and tracing its first per-pixel frame.
pub(crate) const RAYTRACE_LOADING_CANVAS_ID: &str = "raytrace-loading-canvas";

/// The CSS selector for the RayTrace Canvas 2D loading overlay canvas.
pub(crate) const RAYTRACE_LOADING_CANVAS_SELECTOR: &str = "#raytrace-loading-canvas";

/// The Canvas 2D context type identifier passed to `HTMLCanvasElement::get_context`.
pub(crate) const RAYTRACE_CONTEXT_TYPE: &str = "2d";

/// Minimum visible duration in milliseconds for the RayTrace Canvas 2D
/// tab's loading overlay.
///
/// Mirrors `GAME_3D_LOADING_MIN_MILLIS` / `raytrace_set_loaded_delayed`
/// (used by the WebGL / WebGPU tabs) so the user always sees the
/// "Initializing..." text for at least one paint even when the SSAA
/// acquire + first warmup ray pass finishes faster than a single
/// frame. Without this floor the overlay would mount and unmount
/// inside the same `requestAnimationFrame` tick, which most browsers
/// collapse into a single paint and the user never sees the loading
/// state at all.
pub(crate) const RAYTRACE_CANVAS_2D_LOADING_MIN_MILLIS: i32 = 400;

/// Logical width of the RayTrace page's offscreen render buffer at full
/// render scale.
///
/// The buffer is sized so a full per-pixel software ray pass finishes
/// well under 16ms per frame on a mid-range laptop. The CSS box scales
/// the buffer to fit the visible canvas via the `c_game_3d_canvas`
/// style.
pub(crate) const RAYTRACE_WIDTH: f64 = 320.0;

/// Logical height of the RayTrace page's offscreen render buffer at full
/// render scale.
pub(crate) const RAYTRACE_HEIGHT: f64 = 240.0;

/// The orbit yaw speed in radians per second for auto-rotation.
///
/// Mirrors the same constant in the 3D game page so the two demos feel
/// visually consistent when both are visible in the sidebar.
pub(crate) const RAYTRACE_AUTO_YAW_SPEED: f64 = 0.5;

/// The minimum angle in radians between the camera pitch and +/- pi/2.
///
/// Prevents the orbit camera from looking straight up or down, which
/// would collapse the `forward x up` cross product and zero the view
/// matrix.
pub(crate) const RAYTRACE_PITCH_CLAMP: f64 = 0.01;

/// The sensitivity multiplier applied to pointer drag deltas before
/// they are folded into orbit angles.
///
/// Matches the value used by the 3D game page's pointer handlers so the
/// two demos feel identical in drag responsiveness.
pub(crate) const RAYTRACE_DRAG_SENSITIVITY: f64 = 0.01;

/// The radius of the orbit sphere on which the camera sits.
///
/// Mirrors `GAME_3D_CAMERA_DISTANCE` so the user can compare the two
/// orbit-camera demos at equivalent zoom levels.
pub(crate) const RAYTRACE_CAMERA_DISTANCE: f64 = 8.0;

/// The y-coordinate of the orbit sphere's centre (the scene's
/// look-at target vertical position).
pub(crate) const RAYTRACE_CAMERA_LOOK_AT_Y: f64 = 0.4;

/// The z-coordinate of the orbit sphere's centre (the scene's
/// look-at target depth).
pub(crate) const RAYTRACE_CAMERA_LOOK_AT_Z: f64 = 0.0;

/// The JavaScript property name for the touch list `touches` on a
/// `TouchEvent`.
pub(crate) const RAYTRACE_EVENT_PROPERTY_TOUCHES: &str = "touches";

/// The JavaScript property name for the client X coordinate on a
/// `Touch` object.
pub(crate) const RAYTRACE_EVENT_PROPERTY_CLIENT_X: &str = "clientX";

/// The JavaScript property name for the client Y coordinate on a
/// `Touch` object.
pub(crate) const RAYTRACE_EVENT_PROPERTY_CLIENT_Y: &str = "clientY";

/// Delay in milliseconds before the raytrace loop's first `requestAnimationFrame`
/// callback is scheduled, allowing the canvas element to mount before the
/// first frame attempts to acquire a 2D context.
pub(crate) const RAYTRACE_LOOP_START_DELAY_MILLIS: i32 = 360;

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
pub(crate) const RAYTRACE_RENDER_SCALES: [f64; 12] = [
    4.0, 3.0, 2.5, 2.0, 1.75, 1.5, 1.25, 1.0, 0.75, 0.5, 0.375, 0.25,
];

/// Exponential-moving-average blend factor for the per-frame CPU render
/// time measurement that drives adaptive resolution.
pub(crate) const RAYTRACE_ADAPT_EMA_ALPHA: f64 = 0.1;

/// CPU frame time in milliseconds above which the adaptive-resolution
/// controller steps the render scale down (115% of the 60 FPS budget).
pub(crate) const RAYTRACE_ADAPT_SLOW_FRAME_MILLIS: f64 = 16.67 * 1.15;

/// CPU frame time in milliseconds below which the adaptive-resolution
/// controller steps the render scale up one rung (75% of the 60 FPS
/// budget).
pub(crate) const RAYTRACE_ADAPT_FAST_FRAME_MILLIS: f64 = 16.67 * 0.75;

/// CPU frame time in milliseconds below which the adaptive-resolution
/// controller steps the render scale up two rungs at once (45% of the
/// 60 FPS budget), skipping intermediate rungs when the headroom is
/// obvious.
pub(crate) const RAYTRACE_ADAPT_VERY_FAST_FRAME_MILLIS: f64 = 16.67 * 0.45;

/// Number of consecutive slow frames required before stepping the render
/// scale down one notch.
pub(crate) const RAYTRACE_ADAPT_SLOW_FRAMES: u32 = 30;

/// Number of consecutive fast frames required before stepping the render
/// scale up (one notch, or two notches when the frame time also stayed
/// below [`RAYTRACE_ADAPT_VERY_FAST_FRAME_MILLIS`] for the same span).
pub(crate) const RAYTRACE_ADAPT_FAST_FRAMES: u32 = 45;

/// The HTML `id` attribute value for the RayTrace WebGL canvas element.
pub(crate) const RAYTRACE_WEBGL_CANVAS_ID: &str = "raytrace-webgl-canvas";

/// The CSS selector used to query the RayTrace WebGL canvas element.
pub(crate) const RAYTRACE_WEBGL_CANVAS_SELECTOR: &str = "#raytrace-webgl-canvas";

/// The HTML `id` attribute value for the RayTrace WebGL loading overlay canvas.
pub(crate) const RAYTRACE_WEBGL_LOADING_CANVAS_ID: &str = "raytrace-webgl-loading-canvas";

/// The CSS selector for the RayTrace WebGL loading overlay canvas.
pub(crate) const RAYTRACE_WEBGL_LOADING_CANVAS_SELECTOR: &str = "#raytrace-webgl-loading-canvas";

/// The HTML `id` attribute value for the RayTrace WebGPU canvas element.
pub(crate) const RAYTRACE_WEBGPU_CANVAS_ID: &str = "raytrace-webgpu-canvas";

/// The CSS selector used to query the RayTrace WebGPU canvas element.
pub(crate) const RAYTRACE_WEBGPU_CANVAS_SELECTOR: &str = "#raytrace-webgpu-canvas";

/// The HTML `id` attribute value for the RayTrace WebGPU loading overlay canvas.
pub(crate) const RAYTRACE_WEBGPU_LOADING_CANVAS_ID: &str = "raytrace-webgpu-loading-canvas";

/// The CSS selector for the RayTrace WebGPU loading overlay canvas.
pub(crate) const RAYTRACE_WEBGPU_LOADING_CANVAS_SELECTOR: &str = "#raytrace-webgpu-loading-canvas";

/// The number of `vec4` slots in the GPU uniform block shared by the
/// WebGL and WebGPU raytrace shaders: orbit eye, camera forward, right,
/// up, sun direction, sun color, ambient, and resolution.
pub(crate) const RAYTRACE_GPU_UNIFORM_VEC4_COUNT: usize = 12;

/// The GLSL ES 3.00 vertex shader source for the RayTrace WebGL demo.
///
/// Attribute-less fullscreen triangle generated from `gl_VertexID`, the
/// same pattern the 3D game page uses for its WebGL programs.
pub(crate) const RAYTRACE_WEBGL_VERTEX_SHADER: &str = r#"#version 300 es

void main() {
    vec2 positions[3] = vec2[3](
        vec2(-1.0, -1.0),
        vec2(3.0, -1.0),
        vec2(-1.0, 3.0)
    );
    gl_Position = vec4(positions[gl_VertexID], 0.0, 1.0);
}
"#;

/// The GLSL ES 3.00 fragment shader source for the RayTrace WebGL demo.
///
/// Traces the exact same scene as the Canvas 2D software path: a ground
/// AABB, a mirror sphere, and an emissive sphere lit by one directional
/// sun. The math mirrors `RayTraceScene::trace` and
/// `LightingUniforms::shade` from euv-engine term for term, including
/// the directional-light-no-shadow and eye-distance-falloff quirks.
/// Per pixel 2x2 sub-samples are averaged (matching the CPU SSAA) and
/// a `1/2.2` gamma curve is applied. The camera basis, sun direction,
/// ambient, and canvas resolution arrive in the `u_params` `vec4`
/// array; the NDC is aspect-corrected from the resolution so the scene
/// never stretches at any canvas size.
pub(crate) const RAYTRACE_WEBGL_FRAGMENT_SHADER: &str = r#"#version 300 es

precision highp float;

uniform vec4 u_params[12];

out vec4 out_color;

// Mirrors the engine's pub(crate) RAYTRACE_DEFAULT_MAX_BOUNCES.
const int MAX_BOUNCES = 4;
// Mirrors the engine's RAYTRACE_DEFAULT_T_MIN / RAYTRACE_DEFAULT_T_MAX.
const float T_MIN = 0.001;
const float T_MAX = 1000.0;
// Mirrors the engine's math EPSILON.
const float EPS = 1e-6;
// The fixed eye used by `LightingUniforms::shade` on the CPU path; the
// orbiting camera only moves the ray origin so specular highlights stay
// stable while orbiting.
const vec3 SHADE_EYE = vec3(0.0, 0.8, 3.5);
const vec3 GROUND_MIN = vec3(-5.0, -0.6, -5.0);
const vec3 GROUND_MAX = vec3(5.0, -0.5, 5.0);
const vec3 MIRROR_CENTER = vec3(0.0, 0.4, 0.0);
const float MIRROR_RADIUS = 0.9;
const vec3 EMISSIVE_CENTER = vec3(1.6, 0.6, -1.4);
const float EMISSIVE_RADIUS = 0.45;
// Sun: positioned at the OPPOSITE direction of the directional sun,
// 8 units out from origin, so the camera always sees the directional
// light source as a tangible object. The position rotates with yaw
// (mirrors `raytrace_sun_direction(yaw) * -8.0` in the Rust side),
// so the sun direction and the visible sun-disk marker always agree.
// The sun is no longer part of the occluder list — it is drawn as a
// screen-space overlay (sun disk + rays to each scene object)
// computed from the projected `SUN_WORLD` constant multiplied by
// `vec3(-sun_dir.xz, -sun_dir.y) * 8.0` on the CPU side and passed
// in via `u_params[8]`. The shadow-free directional light continues
// to provide the actual surface lighting.
const float SUN_DISTANCE = 8.0;
// Sun disk radius in NDC units (independent of canvas resolution).
const float SUN_DISK_RADIUS_NDC = 0.06;
// Ray blend threshold in NDC units. The line segment distance check
// uses this radius around each sun->object ray.
const float RAY_BLEND_NDC = 0.012;
// Number of scene objects the rays are drawn to (1 mirror sphere +
// 1 emissive sphere + 1 ground centre).
const int RAY_TARGET_COUNT = 3;

vec3 material_albedo(int index) {
    if (index == 0) { return vec3(0.30, 0.32, 0.36); }
    if (index == 1) { return vec3(0.05, 0.05, 0.06); }
    return vec3(0.0, 0.0, 0.0);
}

float material_specular(int index) {
    if (index == 0) { return 0.30; }
    if (index == 1) { return 1.0; }
    return 0.0;
}

float material_shininess(int index) {
    if (index == 0) { return 24.0; }
    if (index == 1) { return 64.0; }
    return 0.0;
}

vec3 material_emissive(int index) {
    if (index == 2) { return vec3(1.0, 0.45, 0.10); }
    return vec3(0.0, 0.0, 0.0);
}

// Mirrors engine `ray_sphere_intersect`; returns -1.0 on miss.
float sphere_t(vec3 origin, vec3 dir, vec3 center, float radius, out vec3 normal) {
    vec3 oc = origin - center;
    float b = dot(oc, dir);
    float c = dot(oc, oc) - radius * radius;
    float disc = b * b - c;
    if (disc < 0.0) { return -1.0; }
    float sq = sqrt(disc);
    float t1 = -b - sq;
    float t2 = -b + sq;
    float t = t1;
    if (t1 < 0.0) {
        t = t2;
    }
    if (t < 0.0) { return -1.0; }
    normal = normalize(origin + dir * t - center);
    return t;
}

// Mirrors engine `ray_aabb_intersect` (slab method + max-axis normal);
// returns -1.0 on miss.
float aabb_t(vec3 origin, vec3 dir, vec3 bmin, vec3 bmax, out vec3 normal) {
    vec3 inv_dir = 1.0 / dir;
    vec3 t1 = (bmin - origin) * inv_dir;
    vec3 t2 = (bmax - origin) * inv_dir;
    vec3 tmin = min(t1, t2);
    vec3 tmax = max(t1, t2);
    float t_near = max(max(tmin.x, tmin.y), tmin.z);
    float t_far = min(min(tmax.x, tmax.y), tmax.z);
    if (t_near > t_far || t_far < 0.0) { return -1.0; }
    vec3 hit = origin + dir * t_near;
    vec3 center = (bmin + bmax) * 0.5;
    vec3 extent = (bmax - bmin) * 0.5;
    vec3 d = hit - center;
    vec3 a = abs(d) / max(extent, vec3(EPS));
    if (a.x >= a.y && a.x >= a.z) {
        normal = vec3(sign(d.x), 0.0, 0.0);
    } else if (a.y >= a.z) {
        normal = vec3(0.0, sign(d.y), 0.0);
    } else {
        normal = vec3(0.0, 0.0, sign(d.z));
    }
    return t_near;
}

// Mirrors engine `closest_hit_indexed` over the three scene occluders:
// 0 = ground AABB, 1 = mirror sphere, 2 = emissive sphere. The sun
// sphere is no longer part of the trace path; it is drawn as a
// screen-space overlay in `fs_main` so the directional lighting and
// the visible sun-disk marker stay in sync as the camera orbits.
// Returns -1 on miss. Ties keep the earliest occluder, matching the
// engine.
int closest_hit_index(
    vec3 origin,
    vec3 dir,
    float t_min,
    float t_max,
    out float best_t,
    out vec3 best_pos,
    out vec3 best_normal
) {
    int best_index = -1;
    best_t = 0.0;
    best_pos = vec3(0.0);
    best_normal = vec3(0.0, 1.0, 0.0);
    vec3 candidate_normal = vec3(0.0);
    float t = aabb_t(origin, dir, GROUND_MIN, GROUND_MAX, candidate_normal);
    if (t >= t_min && t <= t_max) {
        best_index = 0;
        best_t = t;
        best_pos = origin + dir * t;
        best_normal = candidate_normal;
    }
    t = sphere_t(origin, dir, MIRROR_CENTER, MIRROR_RADIUS, candidate_normal);
    if (t >= t_min && t <= t_max && (best_index < 0 || t < best_t)) {
        best_index = 1;
        best_t = t;
        best_pos = origin + dir * t;
        best_normal = candidate_normal;
    }
    t = sphere_t(origin, dir, EMISSIVE_CENTER, EMISSIVE_RADIUS, candidate_normal);
    if (t >= t_min && t <= t_max && (best_index < 0 || t < best_t)) {
        best_index = 2;
        best_t = t;
        best_pos = origin + dir * t;
        best_normal = candidate_normal;
    }
    return best_index;
}

// Returns the perpendicular distance from point `p` to the line
// segment [a, b] in 2D (used by the sun->object ray overlay in
// `fs_main`). The clamped projection keeps the segment finite so
// rays stop at each object centre.
float distance_to_segment_2d(vec2 p, vec2 a, vec2 b) {
    vec2 ab = b - a;
    vec2 ap = p - a;
    float t = clamp(dot(ap, ab) / max(dot(ab, ab), 1e-6), 0.0, 1.0);
    vec2 proj = a + ab * t;
    return length(p - proj);
}

// Mirrors engine `LightingUniforms::shade` for the single directional
// sun: ambient + Lambert diffuse + Phong specular + emissive, shadow
// unconditionally 1.0 for directional lights, specular intensity
// unchanged because the sun's falloff is 0.0.
vec3 shade(vec3 position, vec3 normal, int index, vec3 sun_dir, vec3 sun_color, vec3 ambient) {
    vec3 to_eye = SHADE_EYE - position;
    float view_dist = length(to_eye);
    vec3 view_dir = vec3(0.0);
    if (view_dist > EPS) {
        view_dir = to_eye / view_dist;
    }
    vec3 albedo = material_albedo(index);
    float cos_term = max(dot(normal, sun_dir), 0.0);
    vec3 diffuse = sun_color * cos_term * albedo;
    float specular = material_specular(index);
    vec3 spec = vec3(0.0);
    if (specular > 0.0) {
        vec3 reflect_dir = normalize(sun_dir - normal * (2.0 * dot(sun_dir, normal)));
        float spec_factor = pow(max(dot(reflect_dir, view_dir), 0.0), material_shininess(index));
        spec = sun_color * (spec_factor * specular);
    }
    return ambient + diffuse + spec + material_emissive(index);
}

// Mirrors engine `trace_bounces`: throughput-weighted iterative
// reflection with at most MAX_BOUNCES bounces; a miss adds the ambient
// color scaled by the current throughput.
vec3 trace(vec3 origin, vec3 dir, vec3 sun_dir, vec3 sun_color, vec3 ambient) {
    vec3 color = vec3(0.0);
    float throughput = 1.0;
    int depth = 0;
    for (int bounce = 0; bounce <= MAX_BOUNCES; bounce++) {
        float hit_t = 0.0;
        vec3 hit_pos = vec3(0.0);
        vec3 hit_normal = vec3(0.0, 1.0, 0.0);
        int index = closest_hit_index(origin, dir, T_MIN, T_MAX, hit_t, hit_pos, hit_normal);
        if (index < 0) {
            color += ambient * throughput;
            break;
        }
        color += shade(hit_pos, hit_normal, index, sun_dir, sun_color, ambient) * throughput;
        float spec = material_specular(index);
        if (depth >= MAX_BOUNCES || spec <= EPS) { break; }
        throughput *= spec;
        dir = dir - hit_normal * (2.0 * dot(dir, hit_normal));
        origin = hit_pos;
        depth += 1;
    }
    return color;
}

void main() {
    vec3 eye = u_params[0].xyz;
    vec3 forward = u_params[1].xyz;
    vec3 right = u_params[2].xyz;
    vec3 up = u_params[3].xyz;
    vec3 sun_dir = u_params[4].xyz;
    vec3 sun_color = u_params[5].rgb;
    vec3 ambient = u_params[6].rgb;
    vec2 resolution = u_params[7].xy;
    // Sun + scene-object screen positions packed on the CPU side:
    // u_params[8] = sun (ndc_x, ndc_y, depth, _)
    // u_params[9] = mirror sphere centre (ndc_x, ndc_y, depth, _)
    // u_params[10] = emissive sphere centre (ndc_x, ndc_y, depth, _)
    // u_params[11] = ground centre (ndc_x, ndc_y, depth, _)
    // depth < 0 means the world point is behind the camera and the
    // ray to it should be skipped (the projection puts the segment
    // off-screen in arbitrary directions).
    vec2 sun_ndc = u_params[8].xy;
    float sun_depth = u_params[8].z;
    vec2 ray_targets[RAY_TARGET_COUNT];
    ray_targets[0] = u_params[9].xy;
    ray_targets[1] = u_params[10].xy;
    ray_targets[2] = u_params[11].xy;
    float ray_depths[RAY_TARGET_COUNT];
    ray_depths[0] = u_params[9].z;
    ray_depths[1] = u_params[10].z;
    ray_depths[2] = u_params[11].z;
    float aspect = resolution.x / resolution.y;
    float base_x = floor(gl_FragCoord.x);
    // gl_FragCoord is bottom-up; the CPU path scans top-down, which
    // flips ndc_y. Sampling bottom-up directly yields the same set of
    // sub-sample NDC values.
    float base_y = floor(gl_FragCoord.y);
    vec3 acc = vec3(0.0);
    for (int sy = 0; sy < 2; sy++) {
        for (int sx = 0; sx < 2; sx++) {
            float px = base_x + 0.25 + float(sx) * 0.5;
            float py = base_y + 0.25 + float(sy) * 0.5;
            float ndc_x = (px / resolution.x) * 2.0 - 1.0;
            float ndc_y = (py / resolution.y) * 2.0 - 1.0;
            vec3 dir = normalize(forward + right * (ndc_x * aspect) + up * ndc_y);
            vec3 traced = trace(eye, dir, sun_dir, sun_color, ambient);
            // Sun disk + ray overlay. Apply per sub-sample so the
            // overlay anti-aliases alongside the trace pass.
            vec2 frag_ndc = vec2(ndc_x, ndc_y);
            // Sun disk: a circular bright marker at the projected
            // sun position, drawn ONLY when the sun is in front of
            // the camera (sun_depth > 0). The blend is stronger in
            // the centre and tapers at the edge so the disk reads as
            // a soft glow rather than a hard circle.
            if (sun_depth > 0.0) {
                float sun_d = length(frag_ndc - sun_ndc);
                if (sun_d < SUN_DISK_RADIUS_NDC) {
                    float sun_alpha = 1.0 - sun_d / SUN_DISK_RADIUS_NDC;
                    traced = mix(traced, sun_color, sun_alpha * 0.92);
                }
            }
            // Rays from the sun to each scene object centre, drawn
            // only when BOTH endpoints are in front of the camera.
            // Each ray is a thin line segment; we blend the sun's
            // color over the traced color within RAY_BLEND_NDC.
            if (sun_depth > 0.0) {
                for (int i = 0; i < RAY_TARGET_COUNT; i++) {
                    if (ray_depths[i] > 0.0) {
                        float d = distance_to_segment_2d(
                            frag_ndc,
                            sun_ndc,
                            ray_targets[i]
                        );
                        if (d < RAY_BLEND_NDC) {
                            float a = (1.0 - d / RAY_BLEND_NDC) * 0.55;
                            traced = mix(traced, sun_color, a);
                        }
                    }
                }
            }
            acc += traced;
        }
    }
    vec3 linear = acc * 0.25;
    vec3 gamma = pow(clamp(linear, vec3(0.0), vec3(1.0)), vec3(1.0 / 2.2));
    out_color = vec4(gamma, 1.0);
}
"#;

/// The WGSL shader source for the RayTrace WebGPU demo.
///
/// Mirrors [`RAYTRACE_WEBGL_FRAGMENT_SHADER`]: the same hardcoded
/// ground-AABB / mirror-sphere / emissive-sphere scene, the same
/// `trace_bounces` + `LightingUniforms::shade` math, the same 2x2 SSAA
/// and `1/2.2` gamma. The fullscreen triangle is generated from
/// `@builtin(vertex_index)` and the per-frame camera / sun / ambient /
/// resolution data arrives in a single 12-`vec4` uniform buffer at
/// `@group(0) @binding(0)` (slots 8..11 carry the projected sun
/// position + the three scene object centres for the ray overlay).
pub(crate) const RAYTRACE_WEBGPU_SHADER: &str = r#"
struct SceneUniforms {
    camera_eye: vec4<f32>,
    camera_forward: vec4<f32>,
    camera_right: vec4<f32>,
    camera_up: vec4<f32>,
    sun_dir: vec4<f32>,
    sun_color: vec4<f32>,
    ambient: vec4<f32>,
    resolution: vec4<f32>,
    sun_screen: vec4<f32>,
    mirror_screen: vec4<f32>,
    emissive_screen: vec4<f32>,
    ground_screen: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u_scene: SceneUniforms;

// Mirrors the engine's pub(crate) RAYTRACE_DEFAULT_MAX_BOUNCES.
const MAX_BOUNCES: i32 = 4;
// Mirrors the engine's RAYTRACE_DEFAULT_T_MIN / RAYTRACE_DEFAULT_T_MAX.
const T_MIN: f32 = 0.001;
const T_MAX: f32 = 1000.0;
// Mirrors the engine's math EPSILON.
const EPS: f32 = 1e-6;
// The fixed eye used by `LightingUniforms::shade` on the CPU path.
const SHADE_EYE = vec3<f32>(0.0, 0.8, 3.5);
const GROUND_MIN = vec3<f32>(-5.0, -0.6, -5.0);
const GROUND_MAX = vec3<f32>(5.0, -0.5, 5.0);
const MIRROR_CENTER = vec3<f32>(0.0, 0.4, 0.0);
const MIRROR_RADIUS: f32 = 0.9;
const EMISSIVE_CENTER = vec3<f32>(1.6, 0.6, -1.4);
const EMISSIVE_RADIUS: f32 = 0.45;
// Sun: positioned at the OPPOSITE direction of the directional sun,
// 8 units out from origin, so the camera always sees the directional
// light source as a tangible object. The position rotates with yaw
// (mirrors `raytrace_sun_direction(yaw) * -8.0` in the Rust side),
// so the sun direction and the visible sun-disk marker always agree.
// The sun is no longer part of the occluder list — it is drawn as a
// screen-space overlay (sun disk + rays to each scene object)
// computed from the projected `u_scene.sun_screen` uniform filled in
// on the CPU side. The shadow-free directional light continues to
// provide the actual surface lighting.
const SUN_DISTANCE: f32 = 8.0;
// Sun disk radius in NDC units (independent of canvas resolution).
const SUN_DISK_RADIUS_NDC: f32 = 0.06;
// Ray blend threshold in NDC units. The line segment distance check
// uses this radius around each sun->object ray.
const RAY_BLEND_NDC: f32 = 0.012;
// Number of scene objects the rays are drawn to (1 mirror sphere +
// 1 emissive sphere + 1 ground centre).
const RAY_TARGET_COUNT: i32 = 3;

struct HitResult {
    t: f32,
    index: i32,
    position: vec3<f32>,
    normal: vec3<f32>,
};

fn material_albedo(index: i32) -> vec3<f32> {
    if index == 0 { return vec3<f32>(0.30, 0.32, 0.36); }
    if index == 1 { return vec3<f32>(0.05, 0.05, 0.06); }
    return vec3<f32>(0.0, 0.0, 0.0);
}

fn material_specular(index: i32) -> f32 {
    if index == 0 { return 0.30; }
    if index == 1 { return 1.0; }
    return 0.0;
}

fn material_shininess(index: i32) -> f32 {
    if index == 0 { return 24.0; }
    if index == 1 { return 64.0; }
    return 0.0;
}

fn material_emissive(index: i32) -> vec3<f32> {
    if index == 2 { return vec3<f32>(1.0, 0.45, 0.10); }
    return vec3<f32>(0.0, 0.0, 0.0);
}

// Mirrors engine `ray_sphere_intersect`; returns -1.0 on miss.
fn sphere_t(origin: vec3<f32>, dir: vec3<f32>, center: vec3<f32>, radius: f32, normal: ptr<function, vec3<f32>>) -> f32 {
    let oc = origin - center;
    let b = dot(oc, dir);
    let c = dot(oc, oc) - radius * radius;
    let disc = b * b - c;
    if disc < 0.0 { return -1.0; }
    let sq = sqrt(disc);
    let t1 = -b - sq;
    let t2 = -b + sq;
    var t = t1;
    if t1 < 0.0 {
        t = t2;
    }
    if t < 0.0 { return -1.0; }
    *normal = normalize(origin + dir * t - center);
    return t;
}

// Mirrors engine `ray_aabb_intersect` (slab method + max-axis normal);
// returns -1.0 on miss.
fn aabb_t(origin: vec3<f32>, dir: vec3<f32>, bmin: vec3<f32>, bmax: vec3<f32>, normal: ptr<function, vec3<f32>>) -> f32 {
    let inv_dir = 1.0 / dir;
    let t1 = (bmin - origin) * inv_dir;
    let t2 = (bmax - origin) * inv_dir;
    let tmin = min(t1, t2);
    let tmax = max(t1, t2);
    let t_near = max(max(tmin.x, tmin.y), tmin.z);
    let t_far = min(min(tmax.x, tmax.y), tmax.z);
    if t_near > t_far || t_far < 0.0 { return -1.0; }
    let hit = origin + dir * t_near;
    let center = (bmin + bmax) * 0.5;
    let extent = (bmax - bmin) * 0.5;
    let d = hit - center;
    let a = abs(d) / max(extent, vec3<f32>(EPS));
    if a.x >= a.y && a.x >= a.z {
        *normal = vec3<f32>(sign(d.x), 0.0, 0.0);
    } else if a.y >= a.z {
        *normal = vec3<f32>(0.0, sign(d.y), 0.0);
    } else {
        *normal = vec3<f32>(0.0, 0.0, sign(d.z));
    }
    return t_near;
}

// Mirrors engine `closest_hit_indexed` over the three scene occluders:
// 0 = ground AABB, 1 = mirror sphere, 2 = emissive sphere. The sun
// sphere is no longer part of the trace path; it is drawn as a
// screen-space overlay in `fs_main` so the directional lighting and
// the visible sun-disk marker stay in sync as the camera orbits.
// `index` is -1 on miss. Ties keep the earliest occluder, matching
// the engine.
fn closest_hit_index(origin: vec3<f32>, dir: vec3<f32>) -> HitResult {
    var best: HitResult;
    best.t = 0.0;
    best.index = -1;
    best.position = vec3<f32>(0.0);
    best.normal = vec3<f32>(0.0, 1.0, 0.0);
    var candidate_normal = vec3<f32>(0.0);
    var t = aabb_t(origin, dir, GROUND_MIN, GROUND_MAX, &candidate_normal);
    if t >= T_MIN && t <= T_MAX {
        best.index = 0;
        best.t = t;
        best.position = origin + dir * t;
        best.normal = candidate_normal;
    }
    t = sphere_t(origin, dir, MIRROR_CENTER, MIRROR_RADIUS, &candidate_normal);
    if t >= T_MIN && t <= T_MAX && (best.index < 0 || t < best.t) {
        best.index = 1;
        best.t = t;
        best.position = origin + dir * t;
        best.normal = candidate_normal;
    }
    t = sphere_t(origin, dir, EMISSIVE_CENTER, EMISSIVE_RADIUS, &candidate_normal);
    if t >= T_MIN && t <= T_MAX && (best.index < 0 || t < best.t) {
        best.index = 2;
        best.t = t;
        best.position = origin + dir * t;
        best.normal = candidate_normal;
    }
    return best;
}

// Returns the perpendicular distance from point `p` to the line
// segment [a, b] in 2D (used by the sun->object ray overlay in
// `fs_main`). The clamped projection keeps the segment finite so
// rays stop at each object centre.
fn distance_to_segment_2d(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let t = clamp(dot(ap, ab) / max(dot(ab, ab), 1e-6), 0.0, 1.0);
    let proj = a + ab * t;
    return length(p - proj);
}

// Mirrors engine `LightingUniforms::shade` for the single directional
// sun: ambient + Lambert diffuse + Phong specular + emissive, shadow
// unconditionally 1.0 for directional lights, specular intensity
// unchanged because the sun's falloff is 0.0.
fn shade(position: vec3<f32>, normal: vec3<f32>, index: i32) -> vec3<f32> {
    let to_eye = SHADE_EYE - position;
    let view_dist = length(to_eye);
    var view_dir = vec3<f32>(0.0);
    if view_dist > EPS {
        view_dir = to_eye / view_dist;
    }
    let sun_dir = u_scene.sun_dir.xyz;
    let sun_color = u_scene.sun_color.rgb;
    let albedo = material_albedo(index);
    let cos_term = max(dot(normal, sun_dir), 0.0);
    let diffuse = sun_color * (cos_term * albedo);
    let specular = material_specular(index);
    var spec = vec3<f32>(0.0);
    if specular > 0.0 {
        let reflect_dir = normalize(sun_dir - normal * (2.0 * dot(sun_dir, normal)));
        let spec_factor = pow(max(dot(reflect_dir, view_dir), 0.0), material_shininess(index));
        spec = sun_color * (spec_factor * specular);
    }
    return u_scene.ambient.rgb + diffuse + spec + material_emissive(index);
}

// Mirrors engine `trace_bounces`: throughput-weighted iterative
// reflection with at most MAX_BOUNCES bounces; a miss adds the ambient
// color scaled by the current throughput.
fn trace(origin_arg: vec3<f32>, dir_arg: vec3<f32>) -> vec3<f32> {
    var color = vec3<f32>(0.0);
    var throughput = 1.0;
    var origin = origin_arg;
    var dir = dir_arg;
    var depth = 0;
    for (var bounce = 0; bounce <= MAX_BOUNCES; bounce++) {
        let hit = closest_hit_index(origin, dir);
        if hit.index < 0 {
            color += u_scene.ambient.rgb * throughput;
            break;
        }
        color += shade(hit.position, hit.normal, hit.index) * throughput;
        let spec = material_specular(hit.index);
        if depth >= MAX_BOUNCES || spec <= EPS { break; }
        throughput *= spec;
        dir = dir - hit.normal * (2.0 * dot(dir, hit.normal));
        origin = hit.position;
        depth += 1;
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
    let eye = u_scene.camera_eye.xyz;
    let forward = u_scene.camera_forward.xyz;
    let right = u_scene.camera_right.xyz;
    let up = u_scene.camera_up.xyz;
    let resolution = u_scene.resolution.xy;
    // Sun + scene-object screen positions packed on the CPU side:
    // sun_screen.xy = sun NDC, sun_screen.z = sun depth (> 0 when
    // in front of the camera). Same convention for the three scene
    // objects (mirror, emissive, ground centre).
    let sun_ndc = u_scene.sun_screen.xy;
    let sun_depth = u_scene.sun_screen.z;
    let ray_targets = array<vec2<f32>, 3>(
        u_scene.mirror_screen.xy,
        u_scene.emissive_screen.xy,
        u_scene.ground_screen.xy,
    );
    let ray_depths = array<f32, 3>(
        u_scene.mirror_screen.z,
        u_scene.emissive_screen.z,
        u_scene.ground_screen.z,
    );
    let aspect = resolution.x / resolution.y;
    // WebGPU fragment positions are top-left origin, matching the CPU
    // path's top-down scanline order.
    let base_x = floor(frag_pos.x);
    let base_y = floor(frag_pos.y);
    var acc = vec3<f32>(0.0);
    let sun_color = u_scene.sun_color.rgb;
    for (var sy = 0; sy < 2; sy = sy + 1) {
        for (var sx = 0; sx < 2; sx = sx + 1) {
            let px = base_x + 0.25 + f32(sx) * 0.5;
            let py = base_y + 0.25 + f32(sy) * 0.5;
            let ndc_x = (px / resolution.x) * 2.0 - 1.0;
            let ndc_y = 1.0 - (py / resolution.y) * 2.0;
            let dir = normalize(forward + right * (ndc_x * aspect) + up * ndc_y);
            var traced = trace(eye, dir);
            // Sun disk + ray overlay. Apply per sub-sample so the
            // overlay anti-aliases alongside the trace pass.
            let frag_ndc = vec2<f32>(ndc_x, ndc_y);
            // Sun disk: a circular bright marker at the projected
            // sun position, drawn ONLY when the sun is in front of
            // the camera (sun_depth > 0). The blend is stronger in
            // the centre and tapers at the edge so the disk reads as
            // a soft glow rather than a hard circle.
            if sun_depth > 0.0 {
                let sun_d = length(frag_ndc - sun_ndc);
                if sun_d < SUN_DISK_RADIUS_NDC {
                    let sun_alpha = 1.0 - sun_d / SUN_DISK_RADIUS_NDC;
                    traced = mix(traced, sun_color, sun_alpha * 0.92);
                }
            }
            // Rays from the sun to each scene object centre, drawn
            // only when BOTH endpoints are in front of the camera.
            // Each ray is a thin line segment; we blend the sun's
            // color over the traced color within RAY_BLEND_NDC.
            if sun_depth > 0.0 {
                for (var i = 0; i < RAY_TARGET_COUNT; i = i + 1) {
                    if ray_depths[i] > 0.0 {
                        let d = distance_to_segment_2d(
                            frag_ndc,
                            sun_ndc,
                            ray_targets[i],
                        );
                        if d < RAY_BLEND_NDC {
                            let a = (1.0 - d / RAY_BLEND_NDC) * 0.55;
                            traced = mix(traced, sun_color, a);
                        }
                    }
                }
            }
            acc += traced;
        }
    }
    let linear = acc * 0.25;
    let gamma = pow(clamp(linear, vec3<f32>(0.0), vec3<f32>(1.0)), vec3<f32>(1.0 / 2.2));
    return vec4<f32>(gamma, 1.0);
}
"#;
