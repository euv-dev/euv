/// Portable WGSL SDF instanced UI quad shader for WebGPU and wgpu-compatible Vulkan pipelines.
pub const WGSL_UI_SHADER: &str = r#"
struct Uniforms {
    viewport: vec2<f32>,
};
struct QuadInstance {
    rect: vec4<f32>,
    clip: vec4<f32>,
    color: vec4<f32>,
    border_color: vec4<f32>,
    radii: vec4<f32>,
    stroke_or_blur: f32,
    angle_rad: f32,
    kind: u32,
    reserved: u32,
};
struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) local_pos: vec2<f32>,
    @location(1) half_size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) border_color: vec4<f32>,
    @location(4) radii: vec4<f32>,
    @location(5) stroke_or_blur: f32,
    @location(6) angle_or_pad: f32,
    @location(7) @interpolate(flat) kind: u32,
};
@group(0) @binding(0) var<uniform> u_frame: Uniforms;
@group(0) @binding(1) var<storage, read> instances: array<QuadInstance>;
@vertex
fn vs_main(@builtin(vertex_index) vid: u32, @builtin(instance_index) iid: u32) -> VertexOut {
    var corners = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 0.0), vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0), vec2<f32>(1.0, 1.0), vec2<f32>(0.0, 1.0)
    );
    let inst = instances[iid];
    let uv = corners[vid % 6u];
    let vp = max(u_frame.viewport, vec2<f32>(1.0, 1.0));
    let px = inst.rect.xy + uv * inst.rect.zw;
    let ndc = vec2<f32>((px.x / vp.x) * 2.0 - 1.0, 1.0 - (px.y / vp.y) * 2.0);
    var out: VertexOut;
    out.pos = vec4<f32>(ndc, 0.0, 1.0);
    out.local_pos = (uv - vec2<f32>(0.5, 0.5)) * inst.rect.zw;
    out.half_size = inst.rect.zw * 0.5;
    out.color = inst.color;
    out.border_color = inst.border_color;
    out.radii = inst.radii;
    out.stroke_or_blur = inst.stroke_or_blur;
    out.angle_or_pad = inst.angle_rad;
    out.kind = inst.kind;
    return out;
}
fn sdf_box(p: vec2<f32>, half_sz: vec2<f32>, r: f32) -> f32 {
    let cr = min(r, min(half_sz.x, half_sz.y));
    let q = abs(p) - half_sz + vec2<f32>(cr, cr);
    return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - cr;
}
fn unit_cov(dist: f32) -> f32 {
    return clamp(0.5 - dist, 0.0, 1.0);
}
fn ign_dither(pos_xy: vec2<f32>) -> f32 {
    return fract(52.982918 * fract(dot(pos_xy, vec2<f32>(0.06711056, 0.00583715)))) - 0.5;
}
fn gaussian_falloff(dist: f32, sigma: f32) -> f32 {
    let s = max(sigma, 0.5);
    let x = dist / (s * 1.4142135);
    let ax = abs(x);
    let t = 1.0 / (1.0 + 0.3275911 * ax);
    let poly = t * (0.2548296 + t * (-0.28449672 + t * (1.4214137 + t * (-1.453152 + t * 1.0614054))));
    let erfc_abs = poly * exp(-ax * ax);
    return clamp(select(1.0 - 0.5 * erfc_abs, 0.5 * erfc_abs, x >= 0.0), 0.0, 1.0);
}
@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let r_top = select(in.radii.y, in.radii.x, in.local_pos.x < 0.0);
    let r_bot = select(in.radii.z, in.radii.w, in.local_pos.x < 0.0);
    let r_quad = select(r_bot, r_top, in.local_pos.y < 0.0);
    if (in.kind == 2u) {
        let caster_half = max(in.half_size - vec2<f32>(in.angle_or_pad, in.angle_or_pad), vec2<f32>(0.5, 0.5));
        let sdist = sdf_box(in.local_pos, caster_half, r_quad);
        let sa = in.color.a * gaussian_falloff(sdist, in.stroke_or_blur);
        let d = ign_dither(in.pos.xy) * (1.0 / 255.0);
        return vec4<f32>(clamp(in.color.rgb + vec3<f32>(d, d, d), vec3<f32>(0.0), vec3<f32>(1.0)), clamp(sa + d * 0.5, 0.0, 1.0));
    }
    let r = min(r_quad, min(in.half_size.x, in.half_size.y));
    let dist = sdf_box(in.local_pos, in.half_size, r);
    let outer_cov = unit_cov(dist);
    let bw = in.stroke_or_blur;
    let inner_half = max(in.half_size - vec2<f32>(bw, bw), vec2<f32>(0.0, 0.0));
    let inner_r = max(r - bw, 0.0);
    let inner_valid = inner_half.x > 0.0 && inner_half.y > 0.0;
    let inner_cov = select(outer_cov, select(0.0, unit_cov(sdf_box(in.local_pos, inner_half, inner_r)), inner_valid), bw > 0.0);
    let border_cov = clamp(outer_cov - inner_cov, 0.0, 1.0);
    let fill_a = in.color.a * inner_cov;
    let border_a = in.border_color.a * border_cov;
    let total_a = clamp(fill_a + border_a, 0.0, 1.0);
    let safe_a = max(total_a, 0.0001);
    let lin_rgb = (in.color.rgb * in.color.rgb * fill_a + in.border_color.rgb * in.border_color.rgb * border_a) / safe_a;
    let rgb = sqrt(clamp(lin_rgb, vec3<f32>(0.0), vec3<f32>(1.0)));
    return vec4<f32>(rgb, total_a);
}
"#;

/// Apple Metal Shading Language (MSL) SDF instanced UI vertex and fragment shaders for macOS, iOS, and iPadOS.
pub const MSL_UI_SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;
struct QuadInstance {
    float4 rect;
    float4 clip;
    float4 color;
    float4 border_color;
    float4 radii;
    float stroke_or_blur;
    float angle_rad;
    uint kind;
    uint reserved;
};
struct QuadIn {
    float4 position [[position]];
    float2 local_pos;
    float2 half_size;
    float4 color;
    float4 border_color;
    float4 radii;
    float stroke_or_blur;
    float angle_or_pad;
    uint kind [[flat]];
};
vertex QuadIn euv_ui_vertex(
    uint vid [[vertex_id]],
    uint iid [[instance_id]],
    const device QuadInstance* instances [[buffer(0)]],
    constant float2& viewport_size [[buffer(1)]]
) {
    float2 corners[6] = {
        float2(0.0, 0.0), float2(1.0, 0.0), float2(0.0, 1.0),
        float2(1.0, 0.0), float2(1.0, 1.0), float2(0.0, 1.0)
    };
    QuadInstance inst = instances[iid];
    float2 uv = corners[vid % 6];
    float2 vp = max(viewport_size, float2(1.0, 1.0));
    float2 px = inst.rect.xy + uv * inst.rect.zw;
    float2 ndc = float2((px.x / vp.x) * 2.0 - 1.0, 1.0 - (px.y / vp.y) * 2.0);
    float2 half_sz = inst.rect.zw * 0.5;
    QuadIn out;
    out.position = float4(ndc, 0.0, 1.0);
    out.local_pos = (uv - 0.5) * inst.rect.zw;
    out.half_size = half_sz;
    out.color = inst.color;
    out.border_color = inst.border_color;
    out.radii = inst.radii;
    out.stroke_or_blur = inst.stroke_or_blur;
    out.angle_or_pad = inst.angle_rad;
    out.kind = inst.kind;
    return out;
}
static inline float euv_sdf_box(float2 p, float2 half_sz, float r) {
    float cr = min(r, min(half_sz.x, half_sz.y));
    float2 q = abs(p) - half_sz + float2(cr);
    return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - cr;
}
static inline float euv_unit_cov(float dist) {
    return clamp(0.5 - dist, 0.0, 1.0);
}
static inline float euv_ign_dither(float2 pos_xy) {
    return fract(52.982918 * fract(dot(pos_xy, float2(0.06711056, 0.00583715)))) - 0.5;
}
static inline float euv_gaussian_falloff(float dist, float sigma) {
    float s = max(sigma, 0.5);
    float x = dist / (s * 1.4142135);
    float ax = fabs(x);
    float t = 1.0 / (1.0 + 0.3275911 * ax);
    float poly = t * (0.2548296 + t * (-0.28449672 + t * (1.4214137 + t * (-1.453152 + t * 1.0614054))));
    float erfc_abs = poly * exp(-ax * ax);
    return clamp((x >= 0.0) ? (0.5 * erfc_abs) : (1.0 - 0.5 * erfc_abs), 0.0, 1.0);
}
fragment float4 euv_ui_fragment(QuadIn in [[stage_in]]) {
    float r_top = (in.local_pos.x < 0.0) ? in.radii.x : in.radii.y;
    float r_bot = (in.local_pos.x < 0.0) ? in.radii.w : in.radii.z;
    float r_quad = (in.local_pos.y < 0.0) ? r_top : r_bot;
    if (in.kind == 2u) {
        float2 caster_half = max(in.half_size - float2(in.angle_or_pad), float2(0.5));
        float sdist = euv_sdf_box(in.local_pos, caster_half, r_quad);
        float sa = in.color.a * euv_gaussian_falloff(sdist, in.stroke_or_blur);
        float d = euv_ign_dither(in.position.xy) * (1.0 / 255.0);
        return float4(clamp(in.color.rgb + d, 0.0, 1.0), clamp(sa + d * 0.5, 0.0, 1.0));
    }
    float r = min(r_quad, min(in.half_size.x, in.half_size.y));
    float dist = euv_sdf_box(in.local_pos, in.half_size, r);
    float outer_cov = euv_unit_cov(dist);
    float bw = in.stroke_or_blur;
    float2 inner_half = max(in.half_size - float2(bw), float2(0.0));
    float inner_r = max(r - bw, 0.0);
    float inner_cov = outer_cov;
    if (bw > 0.0) {
        inner_cov = (inner_half.x > 0.0 && inner_half.y > 0.0)
            ? euv_unit_cov(euv_sdf_box(in.local_pos, inner_half, inner_r))
            : 0.0;
    }
    float border_cov = clamp(outer_cov - inner_cov, 0.0, 1.0);
    float fill_a = in.color.a * inner_cov;
    float border_a = in.border_color.a * border_cov;
    float total_a = clamp(fill_a + border_a, 0.0, 1.0);
    if (total_a <= 0.0001) {
        return float4(0.0);
    }
    float3 lin_rgb = (in.color.rgb * in.color.rgb * fill_a + in.border_color.rgb * in.border_color.rgb * border_a) / total_a;
    float3 rgb = sqrt(clamp(lin_rgb, 0.0, 1.0));
    return float4(rgb, total_a);
}
"#;

/// OpenGL ES 3.0 / WebGL 2 vertex shader paired with `GLSL_ES3_UI_FRAGMENT_SHADER`.
pub const GLSL_ES3_UI_VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;
uniform vec2 u_viewport;
layout(location = 0) in vec2 a_uv;
layout(location = 1) in vec4 a_rect;
layout(location = 2) in vec4 a_color;
layout(location = 3) in vec4 a_border_color;
layout(location = 4) in vec4 a_radii;
layout(location = 5) in vec4 a_params;
out vec2 v_local_pos;
out vec2 v_half_size;
out vec4 v_color;
out vec4 v_border_color;
out vec4 v_radii;
out vec4 v_params;
void main() {
    vec2 vp = max(u_viewport, vec2(1.0));
    vec2 px = a_rect.xy + a_uv * a_rect.zw;
    vec2 ndc = vec2((px.x / vp.x) * 2.0 - 1.0, 1.0 - (px.y / vp.y) * 2.0);
    v_half_size = a_rect.zw * 0.5;
    v_local_pos = (a_uv - 0.5) * a_rect.zw;
    v_color = a_color;
    v_border_color = a_border_color;
    v_radii = a_radii;
    v_params = a_params;
    gl_Position = vec4(ndc, 0.0, 1.0);
}
"#;

/// OpenGL ES 3.0 / WebGL 2 fragment shader matching the exact analytical SDF, Gaussian shadow, and linear-light compositing equations.
pub const GLSL_ES3_UI_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;
in vec2 v_local_pos;
in vec2 v_half_size;
in vec4 v_color;
in vec4 v_border_color;
in vec4 v_radii;
in vec4 v_params;
out vec4 frag_color;
float sdf_box(vec2 p, vec2 half_sz, float r) {
    float cr = min(r, min(half_sz.x, half_sz.y));
    vec2 q = abs(p) - half_sz + vec2(cr);
    return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - cr;
}
float unit_cov(float dist) {
    return clamp(0.5 - dist, 0.0, 1.0);
}
float ign_dither(vec2 pos_xy) {
    return fract(52.982918 * fract(dot(pos_xy, vec2(0.06711056, 0.00583715)))) - 0.5;
}
float gaussian_falloff(float dist, float sigma) {
    float s = max(sigma, 0.5);
    float x = dist / (s * 1.4142135);
    float ax = abs(x);
    float t = 1.0 / (1.0 + 0.3275911 * ax);
    float poly = t * (0.2548296 + t * (-0.28449672 + t * (1.4214137 + t * (-1.453152 + t * 1.0614054))));
    float erfc_abs = poly * exp(-ax * ax);
    return clamp((x >= 0.0) ? (0.5 * erfc_abs) : (1.0 - 0.5 * erfc_abs), 0.0, 1.0);
}
void main() {
    float r_top = (v_local_pos.x < 0.0) ? v_radii.x : v_radii.y;
    float r_bot = (v_local_pos.x < 0.0) ? v_radii.w : v_radii.z;
    float r_quad = (v_local_pos.y < 0.0) ? r_top : r_bot;
    if (v_params.z > 1.5 && v_params.z < 2.5) {
        vec2 caster_half = max(v_half_size - vec2(v_params.y), vec2(0.5));
        float sdist = sdf_box(v_local_pos, caster_half, r_quad);
        float sa = v_color.a * gaussian_falloff(sdist, v_params.x);
        float d = ign_dither(gl_FragCoord.xy) * (1.0 / 255.0);
        frag_color = vec4(clamp(v_color.rgb + vec3(d), 0.0, 1.0), clamp(sa + d * 0.5, 0.0, 1.0));
        return;
    }
    float r = min(r_quad, min(v_half_size.x, v_half_size.y));
    float dist = sdf_box(v_local_pos, v_half_size, r);
    float outer_cov = unit_cov(dist);
    float bw = v_params.x;
    vec2 inner_half = max(v_half_size - vec2(bw), vec2(0.0));
    float inner_r = max(r - bw, 0.0);
    float inner_cov = outer_cov;
    if (bw > 0.0) {
        inner_cov = (inner_half.x > 0.0 && inner_half.y > 0.0)
            ? unit_cov(sdf_box(v_local_pos, inner_half, inner_r))
            : 0.0;
    }
    float border_cov = clamp(outer_cov - inner_cov, 0.0, 1.0);
    float fill_a = v_color.a * inner_cov;
    float border_a = v_border_color.a * border_cov;
    float total_a = clamp(fill_a + border_a, 0.0, 1.0);
    if (total_a <= 0.0001) {
        frag_color = vec4(0.0);
        return;
    }
    vec3 lin_rgb = (v_color.rgb * v_color.rgb * fill_a + v_border_color.rgb * v_border_color.rgb * border_a) / total_a;
    vec3 rgb = sqrt(clamp(lin_rgb, 0.0, 1.0));
    frag_color = vec4(rgb, total_a);
}
"#;

/// Direct3D 11/12 HLSL pixel shader for Windows native hardware rendering.
pub const HLSL_UI_SHADER: &str = r#"
struct PSInput {
    float4 pos : SV_POSITION;
    float2 local_pos : TEXCOORD0;
    float2 half_size : TEXCOORD1;
    float4 color : COLOR0;
    float4 border_color : COLOR1;
    float4 radii : TEXCOORD2;
    float4 params : TEXCOORD3;
};
float SdfBox(float2 p, float2 half_sz, float r) {
    float cr = min(r, min(half_sz.x, half_sz.y));
    float2 q = abs(p) - half_sz + float2(cr, cr);
    return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - cr;
}
float UnitCov(float dist) {
    return saturate(0.5 - dist);
}
float IgnDither(float2 pos_xy) {
    return frac(52.982918 * frac(dot(pos_xy, float2(0.06711056, 0.00583715)))) - 0.5;
}
float GaussianFalloff(float dist, float sigma) {
    float s = max(sigma, 0.5);
    float x = dist / (s * 1.4142135);
    float ax = abs(x);
    float t = 1.0 / (1.0 + 0.3275911 * ax);
    float poly = t * (0.2548296 + t * (-0.28449672 + t * (1.4214137 + t * (-1.453152 + t * 1.0614054))));
    float erfc_abs = poly * exp(-ax * ax);
    return saturate((x >= 0.0) ? (0.5 * erfc_abs) : (1.0 - 0.5 * erfc_abs));
}
float4 PSMain(PSInput input) : SV_TARGET {
    float r_top = (input.local_pos.x < 0.0) ? input.radii.x : input.radii.y;
    float r_bot = (input.local_pos.x < 0.0) ? input.radii.w : input.radii.z;
    float r_quad = (input.local_pos.y < 0.0) ? r_top : r_bot;
    if (input.params.z > 1.5 && input.params.z < 2.5) {
        float2 caster_half = max(input.half_size - float2(input.params.y, input.params.y), float2(0.5, 0.5));
        float sdist = SdfBox(input.local_pos, caster_half, r_quad);
        float sa = input.color.a * GaussianFalloff(sdist, input.params.x);
        float d = IgnDither(input.pos.xy) * (1.0 / 255.0);
        return float4(saturate(input.color.rgb + d), saturate(sa + d * 0.5));
    }
    float r = min(r_quad, min(input.half_size.x, input.half_size.y));
    float dist = SdfBox(input.local_pos, input.half_size, r);
    float outer_cov = UnitCov(dist);
    float bw = input.params.x;
    float2 inner_half = max(input.half_size - float2(bw, bw), float2(0.0, 0.0));
    float inner_r = max(r - bw, 0.0);
    float inner_cov = outer_cov;
    if (bw > 0.0) {
        inner_cov = (inner_half.x > 0.0 && inner_half.y > 0.0)
            ? UnitCov(SdfBox(input.local_pos, inner_half, inner_r))
            : 0.0;
    }
    float border_cov = saturate(outer_cov - inner_cov);
    float fill_a = input.color.a * inner_cov;
    float border_a = input.border_color.a * border_cov;
    float total_a = saturate(fill_a + border_a);
    if (total_a <= 0.0001) {
        return float4(0.0, 0.0, 0.0, 0.0);
    }
    float3 lin_rgb = (input.color.rgb * input.color.rgb * fill_a + input.border_color.rgb * input.border_color.rgb * border_a) / total_a;
    float3 rgb = sqrt(saturate(lin_rgb));
    return float4(rgb, total_a);
}
"#;

/// Maximum instanced quads packed into a single GPU vertex-instance draw batch.
pub const MAX_QUADS_PER_BATCH: usize = 4096;

/// Exact packed byte stride of one `#[repr(C)]` `GpuQuadInstance` (`5 * 16 + 4 * 4 = 96` bytes).
pub const GPU_QUAD_INSTANCE_BYTES: usize = 96;
