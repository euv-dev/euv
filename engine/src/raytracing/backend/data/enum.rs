use super::*;

/// The rendering backend a [`RayTraceRenderer`] drives.
///
/// The backend is chosen by the caller at construction and is never
/// substituted afterwards: a renderer that was asked for WebGPU reports
/// an error when the GPU path fails instead of quietly drawing through
/// Canvas 2D, because a silent substitution turns a diagnosable
/// validation error into a silently slower frame with different pixels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum RayTraceBackend {
    /// The CPU reference path: the frame is traced on the CPU and blitted
    /// through an `ImageData` upload.
    #[default]
    Canvas2D,
    /// The WebGL 2 path: a fullscreen triangle samples two RGBA32F data
    /// textures and writes the default framebuffer.
    WebGl2,
    /// The WebGPU path: a fullscreen triangle reads two storage buffers
    /// through a bind group and writes the swap-chain texture.
    WebGpu,
}
