/// Top-level rendering strategy selectable at startup or dynamically at runtime.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RenderMode {
    /// Automatically chooses Hardware GPU when available, falling back to Hybrid then Software.
    #[default]
    Auto,
    /// Pure CPU tile/dirty-rect analytical SDF software rasterizer with zero GPU requirement.
    Software,
    /// Full hardware GPU instanced quad/SDF/glyph batch rendering.
    Hardware,
    /// CPU-rasterized dirty texture tiles composited via GPU textured quads.
    Hybrid,
}

/// Concrete low-level graphics backend driving frame presentation.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GraphicsBackend {
    /// Portable SIMD-friendly CPU software rasterizer.
    #[default]
    SoftwareCpu,
    /// Apple Metal (macOS, iOS, iPadOS).
    Metal,
    /// Khronos Vulkan (Windows, Linux, Android).
    Vulkan,
    /// Microsoft Direct3D 12 (Windows).
    Direct3D12,
    /// Microsoft Direct3D 11 (Windows fallback).
    Direct3D11,
    /// OpenGL ES 3.0 (Android / embedded Linux fallback).
    OpenGlEs,
    /// Browser WebGPU API.
    WebGpu,
    /// Browser WebGL 2 API.
    WebGl2,
    /// Browser HTML5 Canvas 2D software surface blit.
    Canvas2d,
    /// Deterministic offscreen headless backend for automated pixel tests.
    Headless,
}

/// Swapchain presentation synchronization mode.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PresentMode {
    /// Synchronized to display VBlank (tear-free, energy efficient).
    #[default]
    VSync,
    /// Immediate unsynchronized presentation for lowest latency.
    Immediate,
    /// Triple-buffered mailbox presentation for tear-free low latency.
    Mailbox,
}

/// Anti-aliasing quality policy for shapes, borders, and rounded corners.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AntialiasingMode {
    /// Hard pixel edges for maximum throughput.
    None,
    /// Analytical signed-distance-field coverage anti-aliasing (default, crisp and fast).
    #[default]
    AnalyticalSdf,
    /// 4x hardware multi-sample anti-aliasing.
    Msaa4x,
    /// 2x supersampled anti-aliasing.
    Ssaa2x,
}

/// Output framebuffer color gamut.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ColorSpace {
    /// Standard sRGB gamut.
    #[default]
    Srgb,
    /// Wide-gamut Apple Display P3.
    DisplayP3,
    /// Linear HDR-ready color space.
    Linear,
}
