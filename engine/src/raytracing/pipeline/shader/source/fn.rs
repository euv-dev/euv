use super::*;

/// Returns the GLSL ES 3.00 vertex shader source for the WebGL2 backend.
///
/// # Returns
///
/// - `&'static str` - The vertex shader source.
pub fn raytrace_glsl_vertex() -> &'static str {
    RAYTRACE_GLSL_VERTEX
}

/// Returns the GLSL ES 3.00 fragment shader source for the WebGL2
/// backend.
///
/// # Returns
///
/// - `&'static str` - The fragment shader source.
pub fn raytrace_glsl_fragment() -> &'static str {
    RAYTRACE_GLSL_FRAGMENT
}

/// Returns the WGSL shader source for the WebGPU backend.
///
/// # Returns
///
/// - `&'static str` - The combined vertex and fragment shader source.
pub fn raytrace_wgsl() -> &'static str {
    RAYTRACE_WGSL
}
