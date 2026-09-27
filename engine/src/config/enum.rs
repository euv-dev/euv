use super::*;

/// The rendering backend type used by the engine.
///
/// Selects between the Canvas 2D immediate-mode API and the WebGPU
/// pipeline-based API. Canvas 2D is universally supported and suitable
/// for 2D games and simple 3D software rendering. WebGPU provides
/// GPU-accelerated rendering for demanding 2D and 3D scenes.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum RenderBackendType {
    /// The Canvas 2D rendering backend (`CanvasRenderingContext2d`).
    ///
    /// Universal browser support, immediate-mode drawing API, suitable
    /// for 2D games and software-rendered 3D.
    #[default]
    Canvas2D,
    /// The WebGPU rendering backend (`GpuDevice` + `GpuCanvasContext`).
    ///
    /// GPU-accelerated rendering with shaders, buffers, and pipelines.
    /// Requires a WebGPU-capable browser.
    WebGpu,
    /// The WebGL 2 rendering backend (`WebGl2RenderingContext`).
    ///
    /// GPU-accelerated rendering with GLSL ES 3.00 shaders. Supported by
    /// every modern browser, making it the GPU fallback for browsers that
    /// do not expose WebGPU.
    WebGl,
}

/// The power preference hint passed to `navigator.gpu.requestAdapter`.
///
/// Maps to the WebGPU `GPUPowerPreference` enum. The browser uses this
/// hint to select a GPU adapter when multiple are available.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GpuPowerPreference {
    /// Prefers a low-power adapter to conserve energy, suitable for
    /// mobile devices and battery-conscious scenarios.
    #[default]
    LowPower,
    /// Prefers a high-performance adapter for maximum rendering throughput,
    /// suitable for desktop gaming and compute-intensive scenes.
    HighPerformance,
}
