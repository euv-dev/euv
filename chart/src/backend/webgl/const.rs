//! Constants for the WebGL2 rendering backend.
//!
//! The values here are the ones the chart backend needs and that
//! `web-sys` does not expose as named constants, plus the two shader
//! sources the backend compiles. Keeping the shaders next to the
//! constants makes the whole GPU contract of the backend readable in one
//! place: the vertex layout, the texture units, and the code that turns a
//! vertex into a pixel.

/// The texture unit the glyph atlas — and, one batch at a time, an image
/// — is sampled from.
///
/// The atlas and the images share one unit on purpose: a chart frame
/// never draws the atlas and an image at the same time, so one unit is
/// enough and the shader needs one sampler instead of two.
pub(crate) const GL_TEXTURE_UNIT_PRIMARY: u32 = 0;

/// The texture unit the gradient ramps are sampled from.
///
/// The ramp texture holds one row per distinct gradient, so the shader
/// picks the row it needs from the vertex instead of rebinding a texture
/// per shape.
pub(crate) const GL_TEXTURE_UNIT_RAMP: u32 = 1;

/// The texture unit the gradient parameters are read from.
///
/// The parameters hold the axis of a linear gradient or the centre and
/// radius of a radial one. They live in a texture rather than in uniform
/// arrays because a uniform array would put a hard cap on the number of
/// gradients a chart may use.
pub(crate) const GL_TEXTURE_UNIT_PARAMETERS: u32 = 2;

/// The vertex attribute location of the position, in CSS pixels.
pub(crate) const GL_ATTRIBUTE_POSITION: u32 = 0;

/// The vertex attribute location of the texture coordinate.
pub(crate) const GL_ATTRIBUTE_UV: u32 = 1;

/// The vertex attribute location of the colour.
pub(crate) const GL_ATTRIBUTE_COLOR: u32 = 2;

/// The vertex attribute location of the shading metadata.
///
/// The first component tells the fragment stage how the vertex is shaded
/// and the second names the gradient row the vertex belongs to.
pub(crate) const GL_ATTRIBUTE_META: u32 = 3;

/// The number of floats one vertex holds.
///
/// The layout is a position, a texture coordinate, a colour, and two
/// metadata values — ten floats, forty bytes.
pub(crate) const GL_VERTEX_FLOATS: usize = 10;

/// The size of one vertex, in bytes.
///
/// The value is passed to `vertexAttribPointer` as the stride, so it has
/// to match the layout the backend writes.
pub(crate) const GL_VERTEX_STRIDE: i32 = (GL_VERTEX_FLOATS * 4) as i32;

/// The number of texels the gradient ramp texture is wide.
///
/// A hundred and sixty samples are far more than the eye can resolve in
/// a colour ramp, and the ramp is sampled with linear filtering, so the
/// banding is invisible.
pub(crate) const GL_RAMP_WIDTH: usize = 256;

/// The number of texels the gradient parameter texture is wide.
///
/// The first column holds the axis of a linear gradient or the centre
/// and radius of a radial one; the second column holds the kind.
pub(crate) const GL_PARAMETER_WIDTH: usize = 2;

/// The vertex shader of the chart backend.
///
/// The chart draws in CSS pixels, so the shader is the place where the
/// logical coordinate system meets the drawing buffer: the uniform
/// `u_resolution` carries the size of the canvas in CSS pixels, and the
/// position is mapped into clip space from it. That keeps the geometry
/// independent of the device pixel ratio — a resize never touches a
/// vertex.
pub(crate) const GL_VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_uv;
layout(location = 2) in vec4 a_color;
layout(location = 3) in vec2 a_meta;

uniform vec2 u_resolution;

out vec2 v_uv;
out vec4 v_color;
out vec2 v_position;
flat out int v_kind;
flat out int v_gradient;

void main() {
    vec2 size = max(u_resolution, vec2(1.0));
    vec2 ndc = vec2(
        a_position.x / size.x * 2.0 - 1.0,
        1.0 - a_position.y / size.y * 2.0
    );
    gl_Position = vec4(ndc, 0.0, 1.0);
    v_uv = a_uv;
    v_color = a_color;
    v_position = a_position;
    v_kind = int(a_meta.x + 0.5);
    v_gradient = int(a_meta.y);
}
"#;

/// The fragment shader of the chart backend.
///
/// One shader covers the whole chart. The kind carried by the vertex
/// decides what the fragment does: a flat colour, a glyph read from the
/// atlas, a gradient read from the ramp, or an image. Keeping them in
/// one program is what lets the backend put fills, strokes, labels, and
/// images into a single vertex buffer and still draw them in the order
/// the display list asks for.
///
/// The gradient is evaluated per fragment rather than per vertex, so a
/// radial gradient stays circular instead of collapsing into the
/// straight lines the tessellated triangles would interpolate.
pub(crate) const GL_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

in vec2 v_uv;
in vec4 v_color;
in vec2 v_position;
flat in int v_kind;
flat in int v_gradient;

uniform sampler2D u_atlas;
uniform sampler2D u_ramp;
uniform sampler2D u_parameters;
uniform float u_gradient_count;

out vec4 frag_color;

void main() {
    vec4 color = v_color;

    if (v_kind == 2) {
        int row = v_gradient;
        vec4 axis = texelFetch(u_parameters, ivec2(0, row), 0);
        vec4 shape = texelFetch(u_parameters, ivec2(1, row), 0);
        float t;
        if (shape.x < 0.5) {
            vec2 delta = axis.zw - axis.xy;
            float length_squared = max(dot(delta, delta), 1e-6);
            t = dot(v_position - axis.xy, delta) / length_squared;
        } else {
            float radius = max(axis.z, 1e-6);
            t = length(v_position - axis.xy) / radius;
        }
        t = clamp(t, 0.0, 1.0);
        float row_position = (float(row) + 0.5) / max(u_gradient_count, 1.0);
        vec4 ramp = textureLod(u_ramp, vec2(t, row_position), 0.0);
        color = vec4(ramp.rgb, ramp.a * v_color.a);
    } else if (v_kind == 1) {
        float coverage = textureLod(u_atlas, v_uv, 0.0).a;
        color = vec4(v_color.rgb, v_color.a * coverage);
    } else if (v_kind == 3) {
        vec4 texel = textureLod(u_atlas, v_uv, 0.0);
        color = vec4(texel.rgb * v_color.rgb, texel.a * v_color.a);
    }

    // The drawing buffer is premultiplied, so the fragment carries the
    // colour already multiplied by its own alpha.
    frag_color = vec4(color.rgb * color.a, color.a);
}
"#;
