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
    params: vec4<f32>,
};
struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) local_pos: vec2<f32>,
    @location(1) half_size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) border_color: vec4<f32>,
    @location(4) radii: vec4<f32>,
    @location(5) border_w: f32,
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
    out.border_w = inst.params.x;
    return out;
}
fn hermite_cov(dist: f32) -> f32 {
    let t = clamp(0.5 - dist / 1.15, 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}
@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let r_top = select(in.radii.y, in.radii.x, in.local_pos.x < 0.0);
    let r_bot = select(in.radii.z, in.radii.w, in.local_pos.x < 0.0);
    let r = min(select(r_bot, r_top, in.local_pos.y < 0.0), min(in.half_size.x, in.half_size.y));
    let q = abs(in.local_pos) - in.half_size + vec2<f32>(r, r);
    let dist = length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - r;
    let outer_cov = hermite_cov(dist);
    let inner_cov = select(outer_cov, hermite_cov(dist + in.border_w), in.border_w > 0.0);
    let border_cov = clamp(outer_cov - inner_cov, 0.0, 1.0);
    let fill_a = in.color.a * inner_cov;
    let border_a = in.border_color.a * border_cov;
    let total_a = clamp(fill_a + border_a, 0.0, 1.0);
    let safe_a = max(total_a, 0.0001);
    let rgb = (in.color.rgb * fill_a + in.border_color.rgb * border_a) / safe_a;
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
    float4 params;
};
struct QuadIn {
    float4 position [[position]];
    float2 local_pos;
    float2 half_size;
    float4 color;
    float4 border_color;
    float4 radii;
    float border_w;
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
    out.border_w = inst.params.x;
    return out;
}
static inline float euv_hermite_cov(float dist) {
    float t = clamp(0.5 - dist / 1.15, 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}
fragment float4 euv_ui_fragment(QuadIn in [[stage_in]]) {
    float r_top = (in.local_pos.x < 0.0) ? in.radii.x : in.radii.y;
    float r_bot = (in.local_pos.x < 0.0) ? in.radii.w : in.radii.z;
    float r = min((in.local_pos.y < 0.0) ? r_top : r_bot, min(in.half_size.x, in.half_size.y));
    float2 q = abs(in.local_pos) - in.half_size + float2(r);
    float dist = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - r;
    float outer_cov = euv_hermite_cov(dist);
    float inner_cov = (in.border_w > 0.0) ? euv_hermite_cov(dist + in.border_w) : outer_cov;
    float border_cov = clamp(outer_cov - inner_cov, 0.0, 1.0);
    float fill_a = in.color.a * inner_cov;
    float border_a = in.border_color.a * border_cov;
    float total_a = clamp(fill_a + border_a, 0.0, 1.0);
    if (total_a <= 0.0001) {
        return float4(0.0);
    }
    float3 rgb = (in.color.rgb * fill_a + in.border_color.rgb * border_a) / total_a;
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
layout(location = 5) in float a_border_w;
out vec2 v_local_pos;
out vec2 v_half_size;
out vec4 v_color;
out vec4 v_border_color;
out vec4 v_radii;
out float v_border_w;
void main() {
    vec2 vp = max(u_viewport, vec2(1.0));
    vec2 px = a_rect.xy + a_uv * a_rect.zw;
    vec2 ndc = vec2((px.x / vp.x) * 2.0 - 1.0, 1.0 - (px.y / vp.y) * 2.0);
    v_half_size = a_rect.zw * 0.5;
    v_local_pos = (a_uv - 0.5) * a_rect.zw;
    v_color = a_color;
    v_border_color = a_border_color;
    v_radii = a_radii;
    v_border_w = a_border_w;
    gl_Position = vec4(ndc, 0.0, 1.0);
}
"#;

/// OpenGL ES 3.0 / WebGL 2 fragment shader matching the exact analytical SDF equation.
pub const GLSL_ES3_UI_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;
in vec2 v_local_pos;
in vec2 v_half_size;
in vec4 v_color;
in vec4 v_border_color;
in vec4 v_radii;
in float v_border_w;
out vec4 frag_color;
float hermite_cov(float dist) {
    float t = clamp(0.5 - dist / 1.15, 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}
void main() {
    float r_top = (v_local_pos.x < 0.0) ? v_radii.x : v_radii.y;
    float r_bot = (v_local_pos.x < 0.0) ? v_radii.w : v_radii.z;
    float r = min((v_local_pos.y < 0.0) ? r_top : r_bot, min(v_half_size.x, v_half_size.y));
    vec2 q = abs(v_local_pos) - v_half_size + vec2(r);
    float dist = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - r;
    float outer_cov = hermite_cov(dist);
    float inner_cov = (v_border_w > 0.0) ? hermite_cov(dist + v_border_w) : outer_cov;
    float border_cov = clamp(outer_cov - inner_cov, 0.0, 1.0);
    float fill_a = v_color.a * inner_cov;
    float border_a = v_border_color.a * border_cov;
    float total_a = clamp(fill_a + border_a, 0.0, 1.0);
    if (total_a <= 0.0001) {
        frag_color = vec4(0.0);
        return;
    }
    vec3 rgb = (v_color.rgb * fill_a + v_border_color.rgb * border_a) / total_a;
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
    float border_w : TEXCOORD3;
};
float HermiteCov(float dist) {
    float t = saturate(0.5 - dist / 1.15);
    return t * t * (3.0 - 2.0 * t);
}
float4 PSMain(PSInput input) : SV_TARGET {
    float r_top = (input.local_pos.x < 0.0) ? input.radii.x : input.radii.y;
    float r_bot = (input.local_pos.x < 0.0) ? input.radii.w : input.radii.z;
    float r = min((input.local_pos.y < 0.0) ? r_top : r_bot, min(input.half_size.x, input.half_size.y));
    float2 q = abs(input.local_pos) - input.half_size + float2(r, r);
    float dist = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - r;
    float outer_cov = HermiteCov(dist);
    float inner_cov = (input.border_w > 0.0) ? HermiteCov(dist + input.border_w) : outer_cov;
    float border_cov = saturate(outer_cov - inner_cov);
    float fill_a = input.color.a * inner_cov;
    float border_a = input.border_color.a * border_cov;
    float total_a = saturate(fill_a + border_a);
    if (total_a <= 0.0001) {
        return float4(0.0, 0.0, 0.0, 0.0);
    }
    float3 rgb = (input.color.rgb * fill_a + input.border_color.rgb * border_a) / total_a;
    return float4(rgb, total_a);
}
"#;

/// Maximum instanced quads packed into a single GPU vertex-instance draw batch.
pub const MAX_QUADS_PER_BATCH: usize = 4096;
