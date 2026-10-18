/// The GLSL ES 3.00 vertex shader for the WebGL2 ray-tracing backend.
///
/// It emits a single attribute-less fullscreen triangle from
/// `gl_VertexID`, so the program needs no vertex buffer and no vertex
/// array object contents. The rasterized triangle covers the whole viewport
/// and the fragment shader derives its own pixel coordinates.
pub(crate) const RAYTRACE_GLSL_VERTEX: &str = r#"#version 300 es

void main() {
    vec2 positions[3] = vec2[3](
        vec2(-1.0, -1.0),
        vec2(3.0, -1.0),
        vec2(-1.0, 3.0)
    );
    gl_Position = vec4(positions[gl_VertexID], 0.0, 1.0);
}
"#;

/// The GLSL ES 3.00 fragment shader for the WebGL2 ray-tracing backend.
///
/// It reads the same packed scene and packed frame buffers the WGSL
/// backend and the CPU reference renderer read, uploaded as two
/// `RGBA32F` textures with `NEAREST` filtering and `CLAMP_TO_EDGE`
/// addressing. Every fetch is an unfiltered `texelFetch`, so no float
/// filtering extension is required. The packed array is addressed as
/// `vec4` slots: slot `i` lives at texel `(i % width, i / width)`.
///
/// Nothing about the scene is baked into the source: the node count, the
/// primitive count, the light count, the sample grid, the bounce limit and
/// the camera all arrive through the two data textures, so one program
/// serves any scene.
pub(crate) const RAYTRACE_GLSL_FRAGMENT: &str = r#"#version 300 es

precision highp float;
precision highp int;
precision highp sampler2D;

uniform sampler2D u_scene;
uniform sampler2D u_frame;

// The vec4 slot count in the packed scene buffer.
const int SCENE_SLOTS = 1;
// The vec4 slot count in one packed BVH node.
const int NODE_SLOTS = 3;
// The vec4 slot count in one packed primitive record.
const int PRIMITIVE_SLOTS = 8;
// The vec4 slot count in the packed frame header.
const int FRAME_HEADER_SLOTS = 8;
// The vec4 slot count in one packed light record.
const int LIGHT_SLOTS = 4;

const float EPS = 1e-6;
const float T_MIN_SECONDARY = 0.001;
const float T_MAX_SECONDARY = 1000.0;
const float SURFACE_OFFSET = 1e-3;
const float PBR_BASE = 0.04;
const float GAMMA = 2.2;

out vec4 out_color;

vec4 fetch_scene(int slot) {
    ivec2 size = textureSize(u_scene, 0);
    int width = size.x;
    return texelFetch(u_scene, ivec2(slot - (slot / width) * width, slot / width), 0);
}

vec4 fetch_frame(int slot) {
    ivec2 size = textureSize(u_frame, 0);
    int width = size.x;
    return texelFetch(u_frame, ivec2(slot - (slot / width) * width, slot / width), 0);
}

vec3 safe_normalize(vec3 v) {
    float len = length(v);
    if (len <= EPS) {
        return vec3(0.0);
    }
    return v / len;
}

// Mirrors the engine slab test: a ray parallel to a slab and outside it
// rejects the node, and one inside it imposes no constraint.
bool slab_interval(float o, float d, float lo, float hi, out float enter, out float exit) {
    if (abs(d) < EPS) {
        if (o < lo || o > hi) {
            return false;
        }
        enter = -1.0e30;
        exit = 1.0e30;
        return true;
    }
    float t0 = (lo - o) / d;
    float t1 = (hi - o) / d;
    enter = min(t0, t1);
    exit = max(t0, t1);
    return true;
}

bool node_overlaps(
    vec3 origin,
    vec3 dir,
    float t_min,
    float t_max,
    vec3 lower,
    vec3 upper
) {
    vec3 o = origin;
    vec3 d = dir;
    vec3 lo = lower;
    vec3 hi = upper;
    float enter;
    float exit;
    if (!slab_interval(o.x, d.x, lo.x, hi.x, enter, exit)) {
        return false;
    }
    float t_enter = enter;
    float t_exit = exit;
    if (!slab_interval(o.y, d.y, lo.y, hi.y, enter, exit)) {
        return false;
    }
    t_enter = max(t_enter, enter);
    t_exit = min(t_exit, exit);
    if (!slab_interval(o.z, d.z, lo.z, hi.z, enter, exit)) {
        return false;
    }
    t_enter = max(t_enter, enter);
    t_exit = min(t_exit, exit);
    t_enter = max(t_enter, t_min);
    t_exit = min(t_exit, t_max);
    return t_enter <= t_exit;
}

// Two-sided Moller-Trumbore, matching the engine intersect_triangle.
bool triangle_hit(
    vec3 origin,
    vec3 dir,
    vec3 v0,
    vec3 v1,
    vec3 v2,
    float t_min,
    float t_max,
    out float t,
    out vec3 normal
) {
    vec3 edge1 = v1 - v0;
    vec3 edge2 = v2 - v0;
    vec3 pvec = cross(dir, edge2);
    float det = dot(edge1, pvec);
    if (abs(det) < EPS) {
        return false;
    }
    float inverse_det = 1.0 / det;
    vec3 tvec = origin - v0;
    float u = dot(tvec, pvec) * inverse_det;
    if (u < 0.0 || u > 1.0) {
        return false;
    }
    vec3 qvec = cross(tvec, edge1);
    float v = dot(dir, qvec) * inverse_det;
    if (v < 0.0 || u + v > 1.0) {
        return false;
    }
    t = dot(edge2, qvec) * inverse_det;
    if (t < t_min || t > t_max) {
        return false;
    }
    vec3 geometric = safe_normalize(cross(edge1, edge2));
    normal = dot(geometric, dir) > 0.0 ? -geometric : geometric;
    return true;
}

// The quadratic form resolves both roots, so a ray starting inside the
// solid exits rather than reporting a behind-the-origin miss. The
// direction need not be unit length: the `a` term keeps the roots correct.
bool primitive_hit(
    int slot,
    vec3 origin,
    vec3 dir,
    float t_min,
    float t_max,
    out float t,
    out vec3 normal
) {
    vec4 shape = fetch_scene(slot);
    vec4 extent = fetch_scene(slot + 1);
    vec4 vertex0 = fetch_scene(slot + 2);
    vec4 vertex1 = fetch_scene(slot + 3);
    vec4 vertex2 = fetch_scene(slot + 4);
    int kind = int(shape.w + 0.5);
    if (kind == 2) {
        return triangle_hit(
            origin,
            dir,
            vertex0.xyz,
            vertex1.xyz,
            vertex2.xyz,
            t_min,
            t_max,
            t,
            normal
        );
    }
    if (kind == 1) {
        vec3 lo = shape.xyz;
        vec3 hi = extent.xyz;
        vec3 o = origin;
        vec3 d = dir;
        float enter;
        float exit;
        if (!slab_interval(o.x, d.x, lo.x, hi.x, enter, exit)) {
            return false;
        }
        float t_near = enter;
        float t_far = exit;
        if (!slab_interval(o.y, d.y, lo.y, hi.y, enter, exit)) {
            return false;
        }
        t_near = max(t_near, enter);
        t_far = min(t_far, exit);
        if (!slab_interval(o.z, d.z, lo.z, hi.z, enter, exit)) {
            return false;
        }
        t_near = max(t_near, enter);
        t_far = min(t_far, exit);
        if (t_near > t_far || t_far < t_min) {
            return false;
        }
        float hit_t = t_near < t_min ? t_far : t_near;
        if (hit_t < t_min || hit_t > t_max) {
            return false;
        }
        vec3 hit = origin + dir * hit_t;
        vec3 center = (lo + hi) * 0.5;
        vec3 delta = hit - center;
        vec3 half_extent = (hi - lo) * 0.5;
        vec3 scaled = abs(delta) / max(half_extent, vec3(EPS));
        if (scaled.x >= scaled.y && scaled.x >= scaled.z) {
            normal = vec3(sign(delta.x), 0.0, 0.0);
        } else if (scaled.y >= scaled.z) {
            normal = vec3(0.0, sign(delta.y), 0.0);
        } else {
            normal = vec3(0.0, 0.0, sign(delta.z));
        }
        t = hit_t;
        return true;
    }
    vec3 center = shape.xyz;
    float radius = extent.x;
    vec3 oc = origin - center;
    float a = dot(dir, dir);
    if (a <= EPS) {
        return false;
    }
    float b = dot(oc, dir);
    float c = dot(oc, oc) - radius * radius;
    float disc = b * b - a * c;
    if (disc < 0.0) {
        return false;
    }
    float sq = sqrt(disc);
    float near_t = (-b - sq) / a;
    float far_t = (-b + sq) / a;
    float hit_t = near_t < t_min ? far_t : near_t;
    if (hit_t < t_min || hit_t > t_max) {
        return false;
    }
    vec3 hit = origin + dir * hit_t;
    t = hit_t;
    normal = safe_normalize(hit - center);
    return true;
}

struct Hit {
    bool found;
    float t;
    vec3 position;
    vec3 normal;
    float specular;
    float shininess;
    vec3 emissive;
    vec3 albedo;
    int material_kind;
    float original_index;
};

// Ordered traversal over the packed preorder node array: an internal-node
// hit visits index + 1, a miss or a leaf jumps to escape. The iteration is
// bounded by node_count so a malformed buffer cannot hang the shader, and
// ties on t resolve to the lowest original primitive index.
Hit closest_hit(vec3 origin, vec3 dir, float t_min, float t_max) {
    vec4 header = fetch_scene(0);
    int node_count = int(header.x + 0.5);
    int primitive_count = int(header.y + 0.5);
    int primitive_offset = int(header.z + 0.5);
    Hit best;
    best.found = false;
    best.t = 0.0;
    best.position = vec3(0.0);
    best.normal = vec3(0.0, 1.0, 0.0);
    best.specular = 0.0;
    best.shininess = 0.0;
    best.emissive = vec3(0.0);
    best.albedo = vec3(0.0);
    best.material_kind = 0;
    best.original_index = 0.0;
    if (node_count <= 0 || primitive_count <= 0) {
        return best;
    }
    int index = 0;
    int budget = node_count * 2 + 2;
    for (int step = 0; step < budget; step += 1) {
        if (index >= node_count) {
            break;
        }
        int node_slot = SCENE_SLOTS + index * NODE_SLOTS;
        vec4 lower_bound = fetch_scene(node_slot);
        vec4 upper_bound = fetch_scene(node_slot + 1);
        float escape = fetch_scene(node_slot + 2).x;
        int count = int(upper_bound.w + 0.5);
        bool overlap = node_overlaps(
            origin,
            dir,
            t_min,
            t_max,
            lower_bound.xyz,
            upper_bound.xyz
        );
        if (!overlap) {
            index = int(escape + 0.5);
            continue;
        }
        if (count == 0) {
            index += 1;
            continue;
        }
        int start = int(lower_bound.w + 0.5);
        for (int offset = 0; offset < count; offset += 1) {
            int slot = primitive_offset + (start + offset) * PRIMITIVE_SLOTS;
            float hit_t;
            vec3 hit_normal;
            if (!primitive_hit(slot, origin, dir, t_min, t_max, hit_t, hit_normal)) {
                continue;
            }
            float original_index = fetch_scene(slot + 2).w;
            if (best.found) {
                if (best.t < hit_t) {
                    continue;
                }
                if (best.t == hit_t && best.original_index <= original_index) {
                    continue;
                }
            }
            vec4 albedo = fetch_scene(slot + 5);
            vec4 emissive = fetch_scene(slot + 6);
            best.found = true;
            best.t = hit_t;
            best.position = origin + dir * hit_t;
            best.normal = hit_normal;
            best.albedo = albedo.xyz;
            best.specular = albedo.w;
            best.emissive = emissive.xyz;
            best.shininess = emissive.w;
            best.material_kind = int(fetch_scene(slot + 1).w + 0.5);
            best.original_index = original_index;
        }
        index = int(escape + 0.5);
    }
    return best;
}

// The shadow ray starts offset along the surface normal and stops short of
// the light, so neither the surface it starts on nor the light itself can
// register as an occluder.
bool occluded(vec3 origin, vec3 normal, vec3 dir, float max_distance) {
    vec3 start = origin + normal * SURFACE_OFFSET;
    float clipped = max(max_distance - SURFACE_OFFSET, T_MIN_SECONDARY);
    Hit shadow = closest_hit(start, dir, T_MIN_SECONDARY, clipped);
    return shadow.found;
}

vec3 lambert_term(
    vec3 light_color,
    float intensity,
    vec3 light_dir,
    vec3 normal,
    vec3 albedo
) {
    float cos_term = max(dot(normal, light_dir), 0.0);
    float k = intensity * cos_term;
    return vec3(
        light_color.x * albedo.x * k,
        light_color.y * albedo.y * k,
        light_color.z * albedo.z * k
    );
}

// The Phong lobe reflects the *incident* ray -L about N, not the Blinn
// half-vector. Reflecting L instead points back toward the light source,
// so the highlight would land on the unlit side.
float phong_factor(vec3 light_dir, vec3 normal, vec3 view_dir, float shininess) {
    if (dot(normal, light_dir) <= 0.0) {
        return 0.0;
    }
    vec3 reflected = safe_normalize(normal * (2.0 * dot(normal, light_dir)) - light_dir);
    return pow(max(dot(reflected, view_dir), 0.0), shininess);
}

float schlick_fresnel(vec3 light_dir, vec3 view_dir, vec3 normal) {
    vec3 half_vector = light_dir + view_dir;
    float half_len = length(half_vector);
    float cos_term = half_len > EPS ? clamp(abs(dot(normal, half_vector / half_len)), 0.0, 1.0) : 1.0;
    return PBR_BASE + (1.0 - PBR_BASE) * pow(1.0 - cos_term, 5.0);
}

vec3 shade_hit(Hit hit, vec4 lighting, vec4 ambient, int light_count) {
    vec3 position = hit.position;
    vec3 normal = hit.normal;
    vec3 view_dir = safe_normalize(lighting.xyz - position);
    vec3 color = ambient.xyz;
    for (int i = 0; i < light_count; i += 1) {
        int base = FRAME_HEADER_SLOTS + i * LIGHT_SLOTS;
        vec4 head = fetch_frame(base);
        vec4 direction = fetch_frame(base + 1);
        vec4 color_record = fetch_frame(base + 2);
        vec4 cone = fetch_frame(base + 3);
        int kind = int(head.w + 0.5);
        vec3 light_color = color_record.xyz;
        float raw_intensity = direction.w;
        if (raw_intensity <= 0.0) {
            continue;
        }
        vec3 light_dir;
        float distance;
        if (kind == 0) {
            light_dir = direction.xyz;
            distance = 0.0;
        } else {
            vec3 to_light = head.xyz - position;
            distance = length(to_light);
            light_dir = distance > EPS ? to_light / distance : vec3(0.0);
        }
        if (kind == 2) {
            if (light_dir != vec3(0.0) && dot(light_dir, direction.xyz) > -cone.x) {
                continue;
            }
        }
        if (dot(normal, light_dir) <= 0.0) {
            continue;
        }
        float falloff_factor = 1.0;
        float shadow_distance = T_MAX_SECONDARY;
        if (kind != 0) {
            // Falloff uses the clamped distance so a surface sitting on the
            // lamp cannot divide by zero, but the shadow ray must reach the
            // ACTUAL lamp, so it keeps the unclamped distance.
            float dist = max(distance, T_MIN_SECONDARY);
            float denominator = 1.0 + color_record.w * dist * dist;
            falloff_factor = max(1.0 / denominator, 0.0);
            shadow_distance = distance;
        }
        float shadow = occluded(position, normal, light_dir, shadow_distance) ? 0.0 : 1.0;
        if (shadow <= 0.0) {
            continue;
        }
        float intensity = raw_intensity * falloff_factor;
        vec3 diffuse = lambert_term(light_color, intensity, light_dir, normal, hit.albedo);
        vec3 contribution = diffuse;
        if (hit.material_kind == 2) {
            float fresnel = schlick_fresnel(light_dir, view_dir, normal);
            contribution = diffuse * (PBR_BASE + (1.0 - PBR_BASE) * fresnel);
        } else if (hit.material_kind == 1) {
            float factor = phong_factor(light_dir, normal, view_dir, hit.shininess);
            float k = intensity * factor * hit.specular;
            contribution = diffuse + light_color * k;
        }
        color += contribution * shadow;
    }
    return color + hit.emissive;
}

vec3 trace(vec3 origin, vec3 dir, vec4 lighting, vec4 ambient, int light_count, float max_bounces, float near_t, float far_t) {
    vec3 color = vec3(0.0);
    float throughput = 1.0;
    float t_min = near_t;
    float t_max = far_t;
    int depth = 0;
    for (int bounce = 0; bounce <= 64; bounce += 1) {
        Hit hit = closest_hit(origin, dir, t_min, t_max);
        if (!hit.found) {
            return color + ambient.xyz * throughput;
        }
        color += shade_hit(hit, lighting, ambient, light_count) * throughput;
        if (depth >= int(max_bounces + 0.5) || hit.specular <= EPS) {
            return color;
        }
        throughput *= hit.specular;
        dir = normalize(dir - hit.normal * (2.0 * dot(dir, hit.normal)));
        origin = hit.position;
        t_min = T_MIN_SECONDARY;
        t_max = T_MAX_SECONDARY;
        depth += 1;
    }
    return color;
}

void main() {
    vec4 eye_record = fetch_frame(0);
    vec4 projection = fetch_frame(1);
    vec4 right_record = fetch_frame(2);
    vec4 up_record = fetch_frame(3);
    vec4 ambient = fetch_frame(4);
    vec4 lighting = fetch_frame(5);
    vec4 range = fetch_frame(6);
    int light_count = int(ambient.w + 0.5);
    float max_bounces = lighting.w;
    float samples = max(range.z, 1.0);
    float aspect = projection.w;
    vec2 resolution = vec2(right_record.w, up_record.w);
    int sample_count = int(samples + 0.5);
    vec3 accumulated = vec3(0.0);
    for (int sy = 0; sy < sample_count; sy += 1) {
        for (int sx = 0; sx < sample_count; sx += 1) {
            // gl_FragCoord is bottom-up; mapping it straight onto ndc_y
            // keeps the image upright and matches the CPU reference.
            // CPU reference renderer walks.
            float px = floor(gl_FragCoord.x) + (float(sx) + 0.5) / samples;
            float py = floor(gl_FragCoord.y) + (float(sy) + 0.5) / samples;
            float ndc_x = (px / resolution.x) * 2.0 - 1.0;
            float ndc_y = (py / resolution.y) * 2.0 - 1.0;
            // tan(fov/2) is what makes the vertical field of view exactly
            // `fov`; omitting it widens the image past the camera's ask.
            float tan_half = eye_record.w;
            vec3 dir = normalize(
                projection.xyz
                    + right_record.xyz * (ndc_x * aspect * tan_half)
                    + up_record.xyz * (ndc_y * tan_half)
            );
            accumulated += trace(
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
    vec3 linear = accumulated / (samples * samples);
    vec3 gamma = pow(clamp(linear, vec3(0.0), vec3(1.0)), vec3(1.0 / GAMMA));
    out_color = vec4(gamma, 1.0);
}
"#;
