use super::*;

/// The active backend behind a [`RayTraceRenderer`].
///
/// The enum holds exactly one live adapter, so there is never a second
/// surface, program, or device alive at once and no resource is shared
/// between two owners: [`RayTraceBackendAdapter::release`] destroys only
/// what this variant owns, never a caller's device or context.
#[derive(Debug)]
pub(crate) enum RayTraceBackendAdapter {
    /// The CPU reference path.
    Canvas2D(CanvasRayTraceBackend),
    /// The WebGL 2 path.
    WebGl2(WebGlRayTraceBackend),
    /// The WebGPU path.
    WebGpu(WebGpuRayTraceBackend),
}
