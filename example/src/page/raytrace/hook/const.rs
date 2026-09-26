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
/// up, sun direction, sun color, ambient, resolution, sun screen
/// position, and lamp anchor (the floor-point under the sun where the
/// lit pool converges).
pub(crate) const RAYTRACE_GPU_UNIFORM_VEC4_COUNT: usize = 10;

/// The y-coordinate of the floor's top surface.
///
/// The lamp-anchor uniform copies the sun's x/z onto this constant so
/// the shader knows where on the AABB the lit pool converges. Mirrors
/// the `GROUND_MAX` `y` value baked into the WebGL and WebGPU shaders.
pub(crate) const GROUND_Y_TOP_FLOOR_LAMP: f64 = -0.5;

/// Distance from the scene origin at which the visible sun sphere is
/// placed along the sun direction.
///
/// The sun sphere's world position is always
/// `raytrace_sun_direction(yaw) * RAYTRACE_SUN_DISTANCE`, so the glowing
/// disk the user sees and the vector the shading math integrates share a
/// single source of truth and can never drift apart. All three backends
/// derive the sphere centre from the same sun-direction value: the CPU
/// path multiplies it in `build_raytrace_scene`, and the two shaders
/// multiply the `sun_dir` uniform by `SUN_DISTANCE`.
pub(crate) const RAYTRACE_SUN_DISTANCE: f64 = 8.0;

/// Radius of the visible sun sphere in world units.
///
/// Mirrors `SUN_RADIUS` in the WebGL and WebGPU shaders.
pub(crate) const RAYTRACE_SUN_RADIUS: f64 = 0.5;

/// The number of precomputed occluder bounding spheres uploaded each
/// frame to the GPU shaders (matches the engine's `shadow_points`).
pub(crate) const RAYTRACE_GPU_SPHERE_PACK_COUNT: usize = 4;

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

uniform vec4 u_params[10];
uniform vec4 u_sphere_packs[4];

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
// Sun sphere: positioned along the sun direction at distance
// SUN_DISTANCE so the visible disk and the floor's lit pool share a
// single source of truth. Replaces the previous yaw=0-only placement,
// which kept the sphere pinned to one corner regardless of the
// orbiting sun direction.
const float SUN_DISTANCE = 8.0;
const float SUN_RADIUS = 0.5;

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
    if (index == 3) { return vec3(1.00, 0.95, 0.85); }
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

// Mirrors engine `closest_hit_indexed` over the four scene occluders:
// 0 = ground AABB, 1 = mirror sphere, 2 = emissive sphere, 3 = sun
// sphere. Returns -1 on miss. Ties keep the earliest occluder,
// matching the engine.
int closest_hit_index(
    vec3 origin,
    vec3 dir,
    float t_min,
    float t_max,
    vec3 sun_position,
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
    t = sphere_t(origin, dir, sun_position, SUN_RADIUS, candidate_normal);
    if (t >= t_min && t <= t_max && (best_index < 0 || t < best_t)) {
        best_index = 3;
        best_t = t;
        best_pos = origin + dir * t;
        best_normal = candidate_normal;
    }
    return best_index;
}

// Mirrors engine `LightingUniforms::shade` for the positional sun:
// ambient + Lambert diffuse + Phong specular + emissive. The sun is
// point at `sun_position` so the engine's `soft_shadow_factor` is used
// in `trace` to evaluate occlusion; shadows attenuate the diffuse and
// specular contributions. The falloff is 0 (set by `build_raytrace_lighting`)
// so the sun reads as a distant source and the floor stays uniformly
// lit wherever the shadow rays reach it.
vec3 shade(vec3 position, vec3 normal, int index, vec3 sun_position, vec3 sun_color, vec3 ambient, float shadow) {
    vec3 to_eye = SHADE_EYE - position;
    float view_dist = length(to_eye);
    vec3 view_dir = vec3(0.0);
    if (view_dist > EPS) {
        view_dir = to_eye / view_dist;
    }
    vec3 to_light = sun_position - position;
    float light_dist = length(to_light);
    vec3 light_dir = vec3(0.0);
    if (light_dist > EPS) {
        light_dir = to_light / light_dist;
    }
    vec3 albedo = material_albedo(index);
    float cos_term = max(dot(normal, light_dir), 0.0);
    vec3 diffuse = sun_color * (cos_term * shadow) * albedo;
    float specular = material_specular(index);
    vec3 spec = vec3(0.0);
    if (specular > 0.0) {
        vec3 reflect_dir = normalize(light_dir - normal * (2.0 * dot(light_dir, normal)));
        float spec_factor = pow(max(dot(reflect_dir, view_dir), 0.0), material_shininess(index));
        spec = sun_color * (spec_factor * specular * shadow);
    }
    return ambient + diffuse + spec + material_emissive(index);
}

// Mirrors engine `soft_shadow_factor` - returns 1.0 if no occluder
// blocks the path from `origin` toward `light_pos`, otherwise 0.0.
// The bounding spheres `(center, radius)` are precomputed once per
// frame in `RayTraceScene::new` and packed into the `u_sphere_packs`
// uniform array below; this stays binary (no penumbra sampling) to
// mirror the engine exactly.
float occluder_shadow_sphere(vec3 origin, vec3 light_pos, vec3 center, float radius, float dist_to_light) {
    vec3 to_light = light_pos - origin;
    vec3 dir = vec3(0.0);
    if (dist_to_light > EPS) {
        dir = to_light / dist_to_light;
    }
    vec3 oc = origin - center;
    float b = dot(oc, dir);
    float c = dot(oc, oc) - radius * radius;
    float disc = b * b - c;
    if (disc < 0.0) { return 1.0; }
    float sq = sqrt(disc);
    float t1 = -b - sq;
    float t2 = -b + sq;
    float t = t1;
    if (t1 < 0.0) {
        t = t2;
    }
    if (t < 0.0 || t >= dist_to_light - EPS) { return 1.0; }
    return 0.0;
}

// Four precomputed occluder bounding spheres packed as `vec4(center.xyz,
// radius)`. Matches the engine's `shadow_points` exactly. Declared once
// at the top of the shader alongside `u_params`.

float soft_shadow_factor(vec3 origin, vec3 light_pos) {
    float dist_to_light = length(light_pos - origin);
    float shadow = 1.0;
    shadow *= occluder_shadow_sphere(origin, light_pos, u_sphere_packs[0].xyz, u_sphere_packs[0].w, dist_to_light);
    shadow *= occluder_shadow_sphere(origin, light_pos, u_sphere_packs[1].xyz, u_sphere_packs[1].w, dist_to_light);
    shadow *= occluder_shadow_sphere(origin, light_pos, u_sphere_packs[2].xyz, u_sphere_packs[2].w, dist_to_light);
    shadow *= occluder_shadow_sphere(origin, light_pos, u_sphere_packs[3].xyz, u_sphere_packs[3].w, dist_to_light);
    return shadow;
}

// Mirrors engine `trace_bounces` - throughput-weighted iterative
// reflection with at most MAX_BOUNCES bounces; a miss adds the ambient
// color scaled by the current throughput.
vec3 trace(vec3 origin, vec3 dir, vec3 sun_position, vec3 sun_color, vec3 ambient) {
    vec3 color = vec3(0.0);
    float throughput = 1.0;
    int depth = 0;
    for (int bounce = 0; bounce <= MAX_BOUNCES; bounce++) {
        float hit_t = 0.0;
        vec3 hit_pos = vec3(0.0);
        vec3 hit_normal = vec3(0.0, 1.0, 0.0);
        int index = closest_hit_index(origin, dir, T_MIN, T_MAX, sun_position, hit_t, hit_pos, hit_normal);
        if (index < 0) {
            color += ambient * throughput;
            break;
        }
        float shadow = soft_shadow_factor(hit_pos, sun_position);
        color += shade(hit_pos, hit_normal, index, sun_position, sun_color, ambient, shadow) * throughput;
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
    float aspect = resolution.x / resolution.y;
    float base_x = floor(gl_FragCoord.x);
    // gl_FragCoord is bottom-up; the CPU path scans top-down, which
    // flips ndc_y. Sampling bottom-up directly yields the same set of
    // sub-sample NDC values.
    float base_y = floor(gl_FragCoord.y);
    // The visible sun sphere and the shadow-ray target both sit at the
    // sun direction MIRRORED ABOUT THE GROUND PLANE (only y is negated),
    // scaled to SUN_DISTANCE. `sun_dir` points down (negative y), so
    // using it unmirrored would bury the sun under the floor while the
    // floor's Phong lobe — whose peak is `reflect(sun_dir, +y)`, i.e.
    // the same y-negation — stayed above it. Mirroring here is what
    // keeps the glowing disk and the floor's lit pool on the same side.
    // Mirrors the CPU path's `raytrace_sun_position`.
    vec3 sun_position = vec3(sun_dir.x, -sun_dir.y, sun_dir.z) * SUN_DISTANCE;
    vec3 acc = vec3(0.0);
    for (int sy = 0; sy < 2; sy++) {
        for (int sx = 0; sx < 2; sx++) {
            float px = base_x + 0.25 + float(sx) * 0.5;
            float py = base_y + 0.25 + float(sy) * 0.5;
            float ndc_x = (px / resolution.x) * 2.0 - 1.0;
            float ndc_y = (py / resolution.y) * 2.0 - 1.0;
            vec3 dir = normalize(forward + right * (ndc_x * aspect) + up * ndc_y);
            acc += trace(eye, dir, sun_position, sun_color, ambient);
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
/// resolution data arrives in a single 8-`vec4` uniform buffer at
/// `@group(0) @binding(0)`.
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
};

struct SpherePack {
    center_radius: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u_scene: SceneUniforms;
@group(0) @binding(1) var<uniform> u_sphere_packs: array<SpherePack, 4>;

fn sphere_pack_center(idx: i32) -> vec3<f32> {
    return u_sphere_packs[idx].center_radius.xyz;
}

fn sphere_pack_radius(idx: i32) -> f32 {
    return u_sphere_packs[idx].center_radius.w;
}

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
// Sun sphere: positioned along the sun direction at SUN_DISTANCE so the
// visible disk and the floor's lit pool share a single source of truth.
// Replaces the previous yaw=0-only placement.
const SUN_DISTANCE: f32 = 8.0;
const SUN_RADIUS: f32 = 0.5;

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
    if index == 3 { return vec3<f32>(1.00, 0.95, 0.85); }
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

// Mirrors engine `closest_hit_indexed` over the four scene occluders:
// 0 = ground AABB, 1 = mirror sphere, 2 = emissive sphere, 3 = sun
// sphere. `index` is -1 on miss. Ties keep the earliest occluder,
// matching the engine.
fn closest_hit_index(origin: vec3<f32>, dir: vec3<f32>, sun_dir: vec3<f32>) -> HitResult {
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
    t = sphere_t(
        origin,
        dir,
        vec3<f32>(sun_dir.x, -sun_dir.y, sun_dir.z) * SUN_DISTANCE,
        SUN_RADIUS,
        &candidate_normal,
    );
    if t >= T_MIN && t <= T_MAX && (best.index < 0 || t < best.t) {
        best.index = 3;
        best.t = t;
        best.position = origin + dir * t;
        best.normal = candidate_normal;
    }
    return best;
}

// Mirrors engine `LightingUniforms::shade` for the positional sun:
// ambient + Lambert diffuse + Phong specular + emissive. The sun is a
// point light at `sun_position` so the engine's `soft_shadow_factor`
// evaluates occlusion and the resulting `shadow` factor attenuates
// both diffuse and specular contributions. Falloff is 0 (set by
// `build_raytrace_lighting`) so the sun reads as a distant source and
// the floor stays uniformly lit wherever the shadow rays reach it.
fn shade(position: vec3<f32>, normal: vec3<f32>, index: i32, sun_position: vec3<f32>, shadow: f32) -> vec3<f32> {
    let to_eye = SHADE_EYE - position;
    let view_dist = length(to_eye);
    var view_dir = vec3<f32>(0.0);
    if view_dist > EPS {
        view_dir = to_eye / view_dist;
    }
    let sun_color = u_scene.sun_color.rgb;
    let to_light = sun_position - position;
    let light_dist = length(to_light);
    var light_dir = vec3<f32>(0.0);
    if light_dist > EPS {
        light_dir = to_light / light_dist;
    }
    let albedo = material_albedo(index);
    let cos_term = max(dot(normal, light_dir), 0.0);
    let diffuse = sun_color * (cos_term * shadow * albedo);
    let specular = material_specular(index);
    var spec = vec3<f32>(0.0);
    if specular > 0.0 {
        let reflect_dir = normalize(light_dir - normal * (2.0 * dot(light_dir, normal)));
        let spec_factor = pow(max(dot(reflect_dir, view_dir), 0.0), material_shininess(index));
        spec = sun_color * (spec_factor * specular * shadow);
    }
    return u_scene.ambient.rgb + diffuse + spec + material_emissive(index);
}

// Mirrors engine `soft_shadow_factor` - 1.0 if no occluder blocks the
// path from `origin` toward `light_pos`, otherwise 0.0. The bounding
// spheres `(center, radius)` are precomputed once per frame in
// `RayTraceScene::new` and packed into `u_sphere_packs`; this stays
// binary (no penumbra sampling) to mirror the engine exactly.
fn occluder_shadow_sphere(origin: vec3<f32>, light_pos: vec3<f32>, center: vec3<f32>, radius: f32, dist_to_light: f32) -> f32 {
    var dir = vec3<f32>(0.0);
    if dist_to_light > EPS {
        dir = (light_pos - origin) / dist_to_light;
    }
    let oc = origin - center;
    let b = dot(oc, dir);
    let c = dot(oc, oc) - radius * radius;
    let disc = b * b - c;
    if disc < 0.0 { return 1.0; }
    let sq = sqrt(disc);
    let t1 = -b - sq;
    let t2 = -b + sq;
    var t = t1;
    if t1 < 0.0 {
        t = t2;
    }
    if t < 0.0 || t >= dist_to_light - EPS { return 1.0; }
    return 0.0;
}

fn soft_shadow_factor(origin: vec3<f32>, light_pos: vec3<f32>) -> f32 {
    let dist_to_light = length(light_pos - origin);
    var shadow: f32 = 1.0;
    shadow *= occluder_shadow_sphere(origin, light_pos, sphere_pack_center(0), sphere_pack_radius(0), dist_to_light);
    shadow *= occluder_shadow_sphere(origin, light_pos, sphere_pack_center(1), sphere_pack_radius(1), dist_to_light);
    shadow *= occluder_shadow_sphere(origin, light_pos, sphere_pack_center(2), sphere_pack_radius(2), dist_to_light);
    shadow *= occluder_shadow_sphere(origin, light_pos, sphere_pack_center(3), sphere_pack_radius(3), dist_to_light);
    return shadow;
}

// Mirrors engine `trace_bounces` - throughput-weighted iterative
// reflection with at most MAX_BOUNCES bounces; a miss adds the ambient
// color scaled by the current throughput.
fn trace(origin_arg: vec3<f32>, dir_arg: vec3<f32>, sun_dir: vec3<f32>) -> vec3<f32> {
    var color = vec3<f32>(0.0);
    var throughput = 1.0;
    var origin = origin_arg;
    var dir = dir_arg;
    var depth = 0;
    let sun_position = vec3<f32>(sun_dir.x, -sun_dir.y, sun_dir.z) * SUN_DISTANCE;
    for (var bounce = 0; bounce <= MAX_BOUNCES; bounce++) {
        let hit = closest_hit_index(origin, dir, sun_dir);
        if hit.index < 0 {
            color += u_scene.ambient.rgb * throughput;
            break;
        }
        let shadow = soft_shadow_factor(hit.position, sun_position);
        color += shade(hit.position, hit.normal, hit.index, sun_position, shadow) * throughput;
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
    var positions = array<vec2<f32>, 3> (
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
    let sun_dir = u_scene.sun_dir.xyz;
    let resolution = u_scene.resolution.xy;
    let aspect = resolution.x / resolution.y;
    // WebGPU fragment positions are top-left origin, matching the CPU
    // path's top-down scanline order.
    let base_x = floor(frag_pos.x);
    let base_y = floor(frag_pos.y);
    var acc = vec3<f32>(0.0);
    for (var sy = 0; sy < 2; sy++) {
        for (var sx = 0; sx < 2; sx++) {
            let px = base_x + 0.25 + f32(sx) * 0.5;
            let py = base_y + 0.25 + f32(sy) * 0.5;
            let ndc_x = (px / resolution.x) * 2.0 - 1.0;
            let ndc_y = 1.0 - (py / resolution.y) * 2.0;
            let dir = normalize(forward + right * (ndc_x * aspect) + up * ndc_y);
            acc += trace(eye, dir, sun_dir);
        }
    }
    let linear = acc * 0.25;
    let gamma = pow(clamp(linear, vec3<f32>(0.0), vec3<f32>(1.0)), vec3(1.0 / 2.2));
    return vec4<f32>(gamma, 1.0);
}
"#;
