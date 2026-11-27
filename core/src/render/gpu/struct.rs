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
    /// Persistent grow-only Metal staging `id<MTLBuffer>` handle reused across frames (`0` when unallocated).
    #[get(skip)]
    pub(crate) staging_buffer_handle: usize,
    /// Cached CPU-writable `contents()` address of the active shared-mode staging `MTLBuffer` (`0` when unallocated).
    #[get(skip)]
    pub(crate) staging_contents_ptr: usize,
    /// Byte capacity of the retained staging `MTLBuffer`.
    #[get(skip)]
    pub(crate) staging_buffer_capacity: usize,
    /// Total GPU frames submitted through this driver.
    #[get(type(copy))]
    pub(crate) submitted_frames: u64,
    /// Total instanced quad uploads submitted to the GPU.
    #[get(type(copy))]
    pub(crate) uploaded_instances: u64,
    /// Cumulative instance payload bytes uploaded to the GPU across all frames.
    #[get(type(copy))]
    pub(crate) uploaded_bytes: u64,
    /// Instance payload bytes uploaded during the most recent submitted frame.
    #[get(type(copy))]
    pub(crate) last_upload_bytes: usize,
    /// Number of draw batches submitted during the most recent frame.
    #[get(type(copy))]
    pub(crate) last_submitted_batches: usize,
    /// Number of persistent staging buffer allocations or grows across the driver's lifetime.
    #[get(type(copy))]
    pub(crate) staging_reallocations: u64,
    /// Fallback command recorder active on headless hosts.
    pub(crate) fallback: RecordingGpuDriver,
}

/// Process-wide cached Objective-C selector pointers for Apple Metal hot-path submission.
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[derive(Clone, Copy, Debug)]
pub(crate) struct MetalSelectors {
    /// Cached `newBufferWithBytes:length:options:` selector address.
    pub(crate) new_buffer_with_bytes: usize,
    /// Cached `newBufferWithLength:options:` selector address.
    pub(crate) new_buffer_with_length: usize,
    /// Cached `contents` selector address.
    pub(crate) contents: usize,
    /// Cached `commandBuffer` selector address.
    pub(crate) command_buffer: usize,
    /// Cached `commit` selector address.
    pub(crate) commit: usize,
    /// Cached `release` selector address.
    pub(crate) release: usize,
}
