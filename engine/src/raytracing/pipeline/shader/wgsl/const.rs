/// The WGSL shader for the WebGPU ray-tracing backend.
///
/// It reads the same packed scene and packed frame buffers the GLSL backend
/// and the CPU reference renderer read, uploaded as two read-only storage
/// buffers. The packed array is addressed as `vec4` slots by index.
///
/// Nothing about the scene is baked into the source: the node count, the
/// primitive count, the light count, the sample grid, the bounce limit and
/// the camera all arrive through the two bindings, so one pipeline serves
/// any scene. Every buffer read is range-checked against
/// `arrayLength`, so a short or malformed upload cannot produce an
/// out-of-bounds access.
///
/// This is a software BVH traversal executed in a fragment shader, not
/// hardware ray tracing: the WebGPU standard exposes no hardware
/// ray-tracing API.
pub(crate) const RAYTRACE_WGSL: &str = r#"
@group(0) @binding(0) var<storage, read> scene_data: array<vec4<f32>>;
@group(0) @binding(1) var<storage, read> frame_data: array<vec4<f32>>;

const SCENE_SLOTS: u32 = 1u;
const NODE_SLOTS: u32 = 3u;
const PRIMITIVE_SLOTS: u32 = 8u;
const FRAME_HEADER_SLOTS: u32 = 8u;
const LIGHT_SLOTS: u32 = 4u;

const EPS: f32 = 1e-6;
const T_MIN_SECONDARY: f32 = 0.001;
const T_MAX_SECONDARY: f32 = 1000.0;
const SURFACE_OFFSET: f32 = 1e-3;
const PBR_BASE: f32 = 0.04;
const GAMMA: f32 = 2.2;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
}

struct Hit {
    found: bool,
    t: f32,
    position: vec3<f32>,
    normal: vec3<f32>,
    specular: f32,
    shininess: f32,
    albedo: vec3<f32>,
    emissive: vec3<f32>,
    material_kind: u32,
    original_index: f32,
}

struct Slab {
    hit: bool,
    enter: f32,
    exit: f32,
}

fn scene_slot(slot: u32) -> vec4<f32> {
    if (slot >= arrayLength(&scene_data)) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    return scene_data[slot];
}

fn frame_slot(slot: u32) -> vec4<f32> {
    if (slot >= arrayLength(&frame_data)) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    return frame_data[slot];
}

fn safe_normalize(v: vec3<f32>) -> vec3<f32> {
    let len: f32 = length(v);
    if (len <= EPS) {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    return v / len;
}

// Mirrors the engine slab test: a ray parallel to a slab and outside it
// rejects, and one inside it imposes no constraint.
fn slab(o: f32, d: f32, lo: f32, hi: f32) -> Slab {
    var out: Slab;
    if (abs(d) < EPS) {
        if (o < lo || o > hi) {
            out.hit = false;
            out.enter = 0.0;
            out.exit = 0.0;
            return out;
        }
        out.hit = true;
        out.enter = -1.0e30;
        out.exit = 1.0e30;
        return out;
    }
    let t0: f32 = (lo - o) / d;
    let t1: f32 = (hi - o) / d;
    out.hit = true;
    out.enter = min(t0, t1);
    out.exit = max(t0, t1);
    return out;
}

fn node_overlaps(
    origin: vec3<f32>,
    dir: vec3<f32>,
    t_min: f32,
    t_max: f32,
    lower: vec3<f32>,
    upper: vec3<f32>
) -> bool {
    let sx: Slab = slab(origin.x, dir.x, lower.x, upper.x);
    if (!sx.hit) {
        return false;
    }
    let sy: Slab = slab(origin.y, dir.y, lower.y, upper.y);
    if (!sy.hit) {
        return false;
    }
    let sz: Slab = slab(origin.z, dir.z, lower.z, upper.z);
    if (!sz.hit) {
        return false;
    }
    let t_enter: f32 = max(max(max(sx.enter, sy.enter), sz.enter), t_min);
    let t_exit: f32 = min(min(min(sx.exit, sy.exit), sz.exit), t_max);
    return t_enter <= t_exit;
}

// Two-sided Moller-Trumbore, matching the engine intersect_triangle.
fn triangle_hit(
    origin: vec3<f32>,
    dir: vec3<f32>,
    v0: vec3<f32>,
    v1: vec3<f32>,
    v2: vec3<f32>,
    t_min: f32,
    t_max: f32
) -> Hit {
    var out: Hit;
    out.found = false;
    out.t = 0.0;
    out.position = vec3<f32>(0.0, 0.0, 0.0);
    out.normal = vec3<f32>(0.0, 1.0, 0.0);
    out.specular = 0.0;
    out.shininess = 0.0;
    out.albedo = vec3<f32>(0.0, 0.0, 0.0);
    out.emissive = vec3<f32>(0.0, 0.0, 0.0);
    out.material_kind = 0u;
    out.original_index = 0.0;
    let edge1: vec3<f32> = v1 - v0;
    let edge2: vec3<f32> = v2 - v0;
    let pvec: vec3<f32> = cross(dir, edge2);
    let det: f32 = dot(edge1, pvec);
    if (abs(det) < EPS) {
        return out;
    }
    let inverse_det: f32 = 1.0 / det;
    let tvec: vec3<f32> = origin - v0;
    let u: f32 = dot(tvec, pvec) * inverse_det;
    if (u < 0.0 || u > 1.0) {
        return out;
    }
    let qvec: vec3<f32> = cross(tvec, edge1);
    let v: f32 = dot(dir, qvec) * inverse_det;
    if (v < 0.0 || u + v > 1.0) {
        return out;
    }
    let t: f32 = dot(edge2, qvec) * inverse_det;
    if (t < t_min || t > t_max) {
        return out;
    }
    let geometric: vec3<f32> = safe_normalize(cross(edge1, edge2));
    let normal: vec3<f32> = select(geometric, -geometric, dot(geometric, dir) > 0.0);
    out.found = true;
    out.t = t;
    out.position = origin + dir * t;
    out.normal = normal;
    out.specular = 0.0;
    out.shininess = 0.0;
    out.albedo = vec3<f32>(0.0, 0.0, 0.0);
    out.emissive = vec3<f32>(0.0, 0.0, 0.0);
    out.material_kind = 0u;
    out.original_index = 0.0;
    return out;
}

// The quadratic form resolves both roots, so a ray starting inside the
// solid exits rather than reporting a behind-the-origin miss. The
// direction need not be unit length: the `a` term keeps the roots correct.
fn primitive_hit(
    slot: u32,
    origin: vec3<f32>,
    dir: vec3<f32>,
    t_min: f32,
    t_max: f32
) -> Hit {
    var out: Hit;
    out.found = false;
    out.t = 0.0;
    out.position = vec3<f32>(0.0, 0.0, 0.0);
    out.normal = vec3<f32>(0.0, 1.0, 0.0);
    out.specular = 0.0;
    out.shininess = 0.0;
    out.albedo = vec3<f32>(0.0, 0.0, 0.0);
    out.emissive = vec3<f32>(0.0, 0.0, 0.0);
    out.material_kind = 0u;
    out.original_index = 0.0;
    if (slot + PRIMITIVE_SLOTS > arrayLength(&scene_data)) {
        return out;
    }
    let shape: vec4<f32> = scene_data[slot];
    let extent: vec4<f32> = scene_data[slot + 1u];
    let vertex0: vec4<f32> = scene_data[slot + 2u];
    let vertex1: vec4<f32> = scene_data[slot + 3u];
    let vertex2: vec4<f32> = scene_data[slot + 4u];
    let albedo: vec4<f32> = scene_data[slot + 5u];
    let emissive: vec4<f32> = scene_data[slot + 6u];
    let kind: u32 = u32(shape.w + 0.5);
    if (kind == 2u) {
        var tri: Hit = triangle_hit(origin, dir, vertex0.xyz, vertex1.xyz, vertex2.xyz, t_min, t_max);
        tri.albedo = albedo.xyz;
        tri.specular = albedo.w;
        tri.emissive = emissive.xyz;
        tri.shininess = emissive.w;
        tri.material_kind = u32(extent.w + 0.5);
        tri.original_index = vertex0.w;
        return tri;
    }
    if (kind == 1u) {
        let sx: Slab = slab(origin.x, dir.x, shape.x, extent.x);
        if (!sx.hit) {
            return out;
        }
        let sy: Slab = slab(origin.y, dir.y, shape.y, extent.y);
        if (!sy.hit) {
            return out;
        }
        let sz: Slab = slab(origin.z, dir.z, shape.z, extent.z);
        if (!sz.hit) {
            return out;
        }
        let t_near: f32 = max(max(sx.enter, sy.enter), sz.enter);
        let t_far: f32 = min(min(sx.exit, sy.exit), sz.exit);
        if (t_near > t_far || t_far < t_min) {
            return out;
        }
        var hit_t: f32 = t_near;
        if (hit_t < t_min) {
            hit_t = t_far;
        }
        if (hit_t < t_min || hit_t > t_max) {
            return out;
        }
        let hit: vec3<f32> = origin + dir * hit_t;
        let center: vec3<f32> = (shape.xyz + extent.xyz) * 0.5;
        let delta: vec3<f32> = hit - center;
        let half_extent: vec3<f32> = (extent.xyz - shape.xyz) * 0.5;
        let scaled: vec3<f32> = abs(delta) / max(half_extent, vec3<f32>(EPS));
        var normal: vec3<f32> = vec3<f32>(0.0, 0.0, sign(delta.z));
        if (scaled.y >= scaled.z) {
            normal = vec3<f32>(0.0, sign(delta.y), 0.0);
        }
        if (scaled.x >= scaled.y && scaled.x >= scaled.z) {
            normal = vec3<f32>(sign(delta.x), 0.0, 0.0);
        }
        out.found = true;
        out.t = hit_t;
        out.position = hit;
        out.normal = normal;
        out.albedo = albedo.xyz;
        out.specular = albedo.w;
        out.emissive = emissive.xyz;
        out.shininess = emissive.w;
        out.material_kind = u32(extent.w + 0.5);
        out.original_index = vertex0.w;
        return out;
    }
    let oc: vec3<f32> = origin - shape.xyz;
    let a: f32 = dot(dir, dir);
    if (a <= EPS) {
        return out;
    }
    let b: f32 = dot(oc, dir);
    let c: f32 = dot(oc, oc) - extent.x * extent.x;
    let disc: f32 = b * b - a * c;
    if (disc < 0.0) {
        return out;
    }
    let sq: f32 = sqrt(disc);
    let near_t: f32 = (-b - sq) / a;
    let far_t: f32 = (-b + sq) / a;
    var hit_t: f32 = near_t;
    if (hit_t < t_min) {
        hit_t = far_t;
    }
    if (hit_t < t_min || hit_t > t_max) {
        return out;
    }
    out.found = true;
    out.t = hit_t;
    out.position = origin + dir * hit_t;
    out.normal = safe_normalize(origin + dir * hit_t - shape.xyz);
    out.albedo = albedo.xyz;
    out.specular = albedo.w;
    out.emissive = emissive.xyz;
    out.shininess = emissive.w;
    out.material_kind = u32(extent.w + 0.5);
    out.original_index = vertex0.w;
    return out;
}

// Ordered traversal over the packed preorder node array: an internal-node
// hit visits index + 1, a miss or a leaf jumps to escape. The iteration is
// bounded by node_count so a malformed buffer cannot hang the shader, and
// ties on t resolve to the lowest original primitive index.
fn closest_hit(
    origin: vec3<f32>,
    dir: vec3<f32>,
    t_min: f32,
    t_max: f32
) -> Hit {
    var best: Hit;
    best.found = false;
    best.t = 0.0;
    best.position = vec3<f32>(0.0, 0.0, 0.0);
    best.normal = vec3<f32>(0.0, 1.0, 0.0);
    best.specular = 0.0;
    best.shininess = 0.0;
    best.albedo = vec3<f32>(0.0, 0.0, 0.0);
    best.emissive = vec3<f32>(0.0, 0.0, 0.0);
    best.material_kind = 0u;
    best.original_index = 0.0;
    let header: vec4<f32> = scene_slot(0u);
    let node_count: u32 = u32(header.x + 0.5);
    let primitive_count: u32 = u32(header.y + 0.5);
    let primitive_offset: u32 = u32(header.z + 0.5);
    if (node_count == 0u || primitive_count == 0u) {
        return best;
    }
    var index: u32 = 0u;
    let budget: u32 = node_count * 2u + 2u;
    var step: u32 = 0u;
    loop {
        if (step >= budget) {
            break;
        }
        step = step + 1u;
        if (index >= node_count) {
            break;
        }
        let node_slot: u32 = SCENE_SLOTS + index * NODE_SLOTS;
        if (node_slot + NODE_SLOTS > arrayLength(&scene_data)) {
            break;
        }
        let lower_bound: vec4<f32> = scene_data[node_slot];
        let upper_bound: vec4<f32> = scene_data[node_slot + 1u];
        let escape: u32 = u32(scene_data[node_slot + 2u].x + 0.5);
        let count: u32 = u32(upper_bound.w + 0.5);
        let overlap: bool = node_overlaps(
            origin,
            dir,
            t_min,
            t_max,
            lower_bound.xyz,
            upper_bound.xyz
        );
        if (!overlap) {
            index = escape;
            continue;
        }
        if (count == 0u) {
            index = index + 1u;
            continue;
        }
        let start: u32 = u32(lower_bound.w + 0.5);
        for (var offset: u32 = 0u; offset < count; offset = offset + 1u) {
            let slot: u32 = primitive_offset + (start + offset) * PRIMITIVE_SLOTS;
            let candidate: Hit = primitive_hit(slot, origin, dir, t_min, t_max);
            if (!candidate.found) {
                continue;
            }
            if (best.found) {
                if (best.t < candidate.t) {
                    continue;
                }
                if (best.t == candidate.t && best.original_index <= candidate.original_index) {
                    continue;
                }
            }
            best = candidate;
        }
        index = escape;
    }
    return best;
}

// The shadow ray starts offset along the surface normal and stops short of
// the light, so neither the surface it starts on nor the light itself can
// register as an occluder.
fn occluded(
    origin: vec3<f32>,
    normal: vec3<f32>,
    dir: vec3<f32>,
    max_distance: f32
) -> bool {
    let start: vec3<f32> = origin + normal * SURFACE_OFFSET;
    let clipped: f32 = max(max_distance - SURFACE_OFFSET, T_MIN_SECONDARY);
    let shadow: Hit = closest_hit(start, dir, T_MIN_SECONDARY, clipped);
    return shadow.found;
}

fn lambert_term(
    light_color: vec3<f32>,
    intensity: f32,
    light_dir: vec3<f32>,
    normal: vec3<f32>,
    albedo: vec3<f32>
) -> vec3<f32> {
    let cos_term: f32 = max(dot(normal, light_dir), 0.0);
    let k: f32 = intensity * cos_term;
    return vec3<f32>(
        light_color.x * albedo.x * k,
        light_color.y * albedo.y * k,
        light_color.z * albedo.z * k
    );
}

// The Phong lobe reflects the *incident* ray -L about N, not the Blinn
// half-vector. Reflecting L instead points back toward the light source, so
// the highlight would land on the unlit side.
fn phong_factor(
    light_dir: vec3<f32>,
    normal: vec3<f32>,
    view_dir: vec3<f32>,
    shininess: f32
) -> f32 {
    if (dot(normal, light_dir) <= 0.0) {
        return 0.0;
    }
    let reflected: vec3<f32> = safe_normalize(normal * (2.0 * dot(normal, light_dir)) - light_dir);
    return pow(max(dot(reflected, view_dir), 0.0), shininess);
}

fn schlick_fresnel(light_dir: vec3<f32>, view_dir: vec3<f32>, normal: vec3<f32>) -> f32 {
    let half_vector: vec3<f32> = light_dir + view_dir;
    let half_len: f32 = length(half_vector);
    var cos_term: f32 = 1.0;
    if (half_len > EPS) {
        cos_term = clamp(abs(dot(normal, half_vector / half_len)), 0.0, 1.0);
    }
    return PBR_BASE + (1.0 - PBR_BASE) * pow(1.0 - cos_term, 5.0);
}

fn shade_hit(
    hit: Hit,
    lighting: vec4<f32>,
    ambient: vec4<f32>,
    light_count: u32
) -> vec3<f32> {
    let position: vec3<f32> = hit.position;
    let normal: vec3<f32> = hit.normal;
    let view_dir: vec3<f32> = safe_normalize(lighting.xyz - position);
    var color: vec3<f32> = ambient.xyz;
    for (var i: u32 = 0u; i < light_count; i = i + 1u) {
        let base: u32 = FRAME_HEADER_SLOTS + i * LIGHT_SLOTS;
        if (base + LIGHT_SLOTS > arrayLength(&frame_data)) {
            break;
        }
        let head: vec4<f32> = frame_data[base];
        let direction: vec4<f32> = frame_data[base + 1u];
        let color_record: vec4<f32> = frame_data[base + 2u];
        let cone: vec4<f32> = frame_data[base + 3u];
        let kind: u32 = u32(head.w + 0.5);
        let light_color: vec3<f32> = color_record.xyz;
        let raw_intensity: f32 = direction.w;
        if (raw_intensity <= 0.0) {
            continue;
        }
        var light_dir: vec3<f32> = direction.xyz;
        var distance: f32 = 0.0;
        if (kind != 0u) {
            let to_light: vec3<f32> = head.xyz - position;
            distance = length(to_light);
            if (distance > EPS) {
                light_dir = to_light / distance;
            } else {
                light_dir = vec3<f32>(0.0, 0.0, 0.0);
            }
        }
        if (kind == 2u) {
            let degenerate: bool = light_dir.x == 0.0 && light_dir.y == 0.0 && light_dir.z == 0.0;
            if (!degenerate && dot(light_dir, direction.xyz) > -cone.x) {
                continue;
            }
        }
        if (dot(normal, light_dir) <= 0.0) {
            continue;
        }
        var falloff_factor: f32 = 1.0;
        var shadow_distance: f32 = T_MAX_SECONDARY;
        if (kind != 0u) {
            // Falloff uses the clamped distance so a surface sitting on the
            // lamp cannot divide by zero, but the shadow ray must reach the
            // ACTUAL lamp, so it keeps the unclamped distance.
            let dist: f32 = max(distance, T_MIN_SECONDARY);
            let denominator: f32 = 1.0 + color_record.w * dist * dist;
            falloff_factor = max(1.0 / denominator, 0.0);
            shadow_distance = distance;
        }
        var shadow: f32 = 1.0;
        if (occluded(position, normal, light_dir, shadow_distance)) {
            shadow = 0.0;
        }
        if (shadow <= 0.0) {
            continue;
        }
        let intensity: f32 = raw_intensity * falloff_factor;
        let diffuse: vec3<f32> = lambert_term(light_color, intensity, light_dir, normal, hit.albedo);
        var contribution: vec3<f32> = diffuse;
        if (hit.material_kind == 2u) {
            let fresnel: f32 = schlick_fresnel(light_dir, view_dir, normal);
            contribution = diffuse * (PBR_BASE + (1.0 - PBR_BASE) * fresnel);
        } else if (hit.material_kind == 1u) {
            let factor: f32 = phong_factor(light_dir, normal, view_dir, hit.shininess);
            let k: f32 = intensity * factor * hit.specular;
            contribution = diffuse + light_color * k;
        }
        color = color + contribution * shadow;
    }
    return color + hit.emissive;
}

fn trace(
    origin: vec3<f32>,
    dir: vec3<f32>,
    lighting: vec4<f32>,
    ambient: vec4<f32>,
    light_count: u32,
    max_bounces: u32,
    near_t: f32,
    far_t: f32
) -> vec3<f32> {
    var color: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var throughput: f32 = 1.0;
    var ray_origin: vec3<f32> = origin;
    var ray_dir: vec3<f32> = dir;
    var t_min: f32 = near_t;
    var t_max: f32 = far_t;
    var depth: u32 = 0u;
    loop {
        let hit: Hit = closest_hit(ray_origin, ray_dir, t_min, t_max);
        if (!hit.found) {
            return color + ambient.xyz * throughput;
        }
        color = color + shade_hit(hit, lighting, ambient, light_count) * throughput;
        if (depth >= max_bounces || hit.specular <= EPS) {
            return color;
        }
        throughput = throughput * hit.specular;
        ray_dir = safe_normalize(ray_dir - hit.normal * (2.0 * dot(ray_dir, hit.normal)));
        ray_origin = hit.position;
        t_min = T_MIN_SECONDARY;
        t_max = T_MAX_SECONDARY;
        depth = depth + 1u;
        if (depth > 64u) {
            return color;
        }
    }
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    var positions: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0)
    );
    let position: vec2<f32> = positions[vertex_index];
    out.position = vec4<f32>(position.x, position.y, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let eye_record: vec4<f32> = frame_slot(0u);
    let projection: vec4<f32> = frame_slot(1u);
    let right_record: vec4<f32> = frame_slot(2u);
    let up_record: vec4<f32> = frame_slot(3u);
    let ambient: vec4<f32> = frame_slot(4u);
    let lighting: vec4<f32> = frame_slot(5u);
    let range: vec4<f32> = frame_slot(6u);
    let light_count: u32 = u32(ambient.w + 0.5);
    let max_bounces: u32 = u32(lighting.w + 0.5);
    let samples: f32 = max(range.z, 1.0);
    let sample_count: u32 = u32(samples + 0.5);
    let aspect: f32 = projection.w;
    let resolution: vec2<f32> = vec2<f32>(right_record.w, up_record.w);
    let pixel: vec2<f32> = input.position.xy;
    var accumulated: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    for (var sy: u32 = 0u; sy < sample_count; sy = sy + 1u) {
        for (var sx: u32 = 0u; sx < sample_count; sx = sx + 1u) {
            // The fragment position is top-down, matching the CPU reference
            // renderer's row order directly.
            let px: f32 = floor(pixel.x) + (f32(sx) + 0.5) / samples;
            let py: f32 = floor(pixel.y) + (f32(sy) + 0.5) / samples;
            let ndc_x: f32 = (px / resolution.x) * 2.0 - 1.0;
            let ndc_y: f32 = 1.0 - (py / resolution.y) * 2.0;
            // tan(fov/2) is what makes the vertical field of view exactly
            // `fov`; omitting it widens the image past the camera's ask.
            let tan_half: f32 = eye_record.w;
            let dir: vec3<f32> = safe_normalize(
                projection.xyz
                    + right_record.xyz * (ndc_x * aspect * tan_half)
                    + up_record.xyz * (ndc_y * tan_half)
            );
            accumulated = accumulated + trace(
                eye_record.xyz,
                dir,
                lighting,
                ambient,
                light_count,
                max_bounces,
                range.x,
                range.y
            );
        }
    }
    let linear: vec3<f32> = accumulated / (samples * samples);
    let gamma: vec3<f32> = pow(clamp(linear, vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(1.0, 1.0, 1.0)), vec3<f32>(1.0 / GAMMA));
    return vec4<f32>(gamma, 1.0);
}
"#;
