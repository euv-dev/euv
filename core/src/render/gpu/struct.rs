use super::*;

/// Probed physical GPU adapter and native shader compilation telemetry.
#[derive(Clone, Debug, Eq, Getter, New, PartialEq)]
pub struct GpuAdapterInfo {
    /// Active graphics backend identifier.
    #[get(type(copy))]
    pub(crate) backend: GraphicsBackend,
    /// Detected physical GPU adapter name (for example `"Apple M3 Max"`).
    pub(crate) device_name: String,
    /// True when a real physical OS GPU device/runtime was successfully initialized.
    #[get(type(copy))]
    pub(crate) hardware_verified: bool,
    /// True when the native UI shader (`MSL`/`HLSL`/`GLSL`/`WGSL`) compiled on the host GPU.
    #[get(type(copy))]
    pub(crate) shader_compiled: bool,
}

/// Physical OS GPU driver wrapping Metal, D3D12, Vulkan, or WebGL2 with automatic recording fallback.
#[derive(Debug, Getter)]
pub struct NativeGpuDriver {
    /// Probed physical adapter and shader status.
    pub(crate) adapter: GpuAdapterInfo,
    /// Opaque OS GPU device handle (`id<MTLDevice>` on Apple, `0` when fallback).
    #[get(type(copy))]
    pub(crate) device_handle: usize,
    /// Opaque OS GPU command queue handle (`id<MTLCommandQueue>` on Apple, `0` when fallback).
    #[get(type(copy))]
    pub(crate) queue_handle: usize,
    /// Opaque compiled shader library handle (`id<MTLLibrary>` on Apple, `0` when fallback).
    #[get(type(copy))]
    pub(crate) library_handle: usize,
    /// Total GPU frames submitted through this driver.
    #[get(type(copy))]
    pub(crate) submitted_frames: u64,
    /// Total instanced quad uploads submitted to the GPU.
    #[get(type(copy))]
    pub(crate) uploaded_instances: u64,
    /// Fallback command recorder active on headless hosts.
    pub(crate) fallback: RecordingGpuDriver,
}
