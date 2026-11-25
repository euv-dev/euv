/// Portable WGSL SDF instanced UI quad shader for WebGPU and wgpu-compatible Vulkan pipelines.
pub const WGSL_UI_SHADER: &str = r#"
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
    @location(4) radius: f32,
    @location(5) border_w: f32,
};
@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let q = abs(in.local_pos) - in.half_size + vec2<f32>(in.radius, in.radius);
    let dist = length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - in.radius;
    let alpha = clamp(0.5 - dist, 0.0, 1.0);
    let inner = clamp(0.5 - (dist + in.border_w), 0.0, 1.0);
    let border_mix = clamp(alpha - inner, 0.0, 1.0);
    return vec4<f32>(mix(in.color.rgb, in.border_color.rgb, border_mix), in.color.a * alpha);
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
    float radius;
    float border_w;
};
vertex QuadIn euv_ui_vertex(
    uint vid [[vertex_id]],
    uint iid [[instance_id]],
    const device QuadInstance* instances [[buffer(0)]]
) {
    float2 corners[6] = {
        float2(0.0, 0.0), float2(1.0, 0.0), float2(0.0, 1.0),
        float2(1.0, 0.0), float2(1.0, 1.0), float2(0.0, 1.0)
    };
    QuadInstance inst = instances[iid];
    float2 uv = corners[vid % 6];
    float2 half_sz = inst.rect.zw * 0.5;
    QuadIn out;
    out.position = float4(uv * 2.0 - 1.0, 0.0, 1.0);
    out.local_pos = (uv - 0.5) * inst.rect.zw;
    out.half_size = half_sz;
    out.color = inst.color;
    out.border_color = inst.border_color;
    out.radius = inst.radii.x;
    out.border_w = inst.params.x;
    return out;
}
fragment float4 euv_ui_fragment(QuadIn in [[stage_in]]) {
    float2 q = abs(in.local_pos) - in.half_size + float2(in.radius);
    float dist = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - in.radius;
    float alpha = clamp(0.5 - dist, 0.0, 1.0);
    float inner = clamp(0.5 - (dist + in.border_w), 0.0, 1.0);
    float border_mix = clamp(alpha - inner, 0.0, 1.0);
    return float4(mix(in.color.rgb, in.border_color.rgb, border_mix), in.color.a * alpha);
}
"#;

/// OpenGL ES 3.0 / WebGL 2 vertex shader paired with `GLSL_ES3_UI_FRAGMENT_SHADER`.
pub const GLSL_ES3_UI_VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;
layout(location = 0) in vec2 a_uv;
layout(location = 1) in vec4 a_rect;
layout(location = 2) in vec4 a_color;
layout(location = 3) in vec4 a_border_color;
layout(location = 4) in vec2 a_params;
out vec2 v_local_pos;
out vec2 v_half_size;
out vec4 v_color;
out vec4 v_border_color;
out float v_radius;
out float v_border_w;
void main() {
    v_half_size = a_rect.zw * 0.5;
    v_local_pos = (a_uv - 0.5) * a_rect.zw;
    v_color = a_color;
    v_border_color = a_border_color;
    v_radius = a_params.x;
    v_border_w = a_params.y;
    gl_Position = vec4(a_uv * 2.0 - 1.0, 0.0, 1.0);
}
"#;

/// OpenGL ES 3.0 / WebGL 2 fragment shader matching the exact analytical SDF equation.
pub const GLSL_ES3_UI_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;
in vec2 v_local_pos;
in vec2 v_half_size;
in vec4 v_color;
in vec4 v_border_color;
in float v_radius;
in float v_border_w;
out vec4 frag_color;
void main() {
    vec2 q = abs(v_local_pos) - v_half_size + vec2(v_radius);
    float dist = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - v_radius;
    float alpha = clamp(0.5 - dist, 0.0, 1.0);
    float inner = clamp(0.5 - (dist + v_border_w), 0.0, 1.0);
    float border_mix = clamp(alpha - inner, 0.0, 1.0);
    frag_color = vec4(mix(v_color.rgb, v_border_color.rgb, border_mix), v_color.a * alpha);
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
    float radius : TEXCOORD2;
    float border_w : TEXCOORD3;
};
float4 PSMain(PSInput input) : SV_TARGET {
    float2 q = abs(input.local_pos) - input.half_size + float2(input.radius, input.radius);
    float dist = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - input.radius;
    float alpha = saturate(0.5 - dist);
    float inner = saturate(0.5 - (dist + input.border_w));
    float border_mix = saturate(alpha - inner);
    return float4(lerp(input.color.rgb, input.border_color.rgb, border_mix), input.color.a * alpha);
}
"#;

/// Maximum instanced quads packed into a single GPU vertex-instance draw batch.
pub const MAX_QUADS_PER_BATCH: usize = 4096;
