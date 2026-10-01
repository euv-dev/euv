use super::*;

/// A single binding entry inside a `BindGroupDescriptor`.
#[derive(Clone, Debug)]
pub enum BindGroupEntry {
    /// A uniform / storage buffer binding.
    ///
    /// In WGSL terms, the buffer's `usage` must include `UNIFORM` for
    /// `var<uniform>` bindings and `STORAGE` for `var<storage>` bindings.
    Buffer {
        /// The binding slot (matches `@binding(N)` in the shader).
        binding: u32,
        /// The `GpuBuffer` handle.
        buffer: JsValue,
        /// The byte offset into the buffer where the binding starts.
        offset: u64,
        /// The size in bytes of the binding. `None` means "until the end
        /// of the buffer".
        size: Option<u64>,
    },
    /// A read-write storage texture binding.
    ///
    /// The `GpuTexture` must have been created with `STORAGE_BINDING`
    /// in its `usage` flag. Combine with `view` (a `GpuTextureView`)
    /// obtained from `GpuTexture.createView()`.
    StorageTexture {
        /// The binding slot.
        binding: u32,
        /// The `GpuTextureView` handle.
        view: JsValue,
        /// `true` for `texture_storage_2d<format, read>` bindings,
        /// `false` for `texture_storage_2d<format, read_write>` bindings.
        read_only: bool,
    },
    /// A sampled texture binding.
    Texture {
        /// The binding slot.
        binding: u32,
        /// The `GpuTextureView` handle.
        view: JsValue,
    },
    /// A sampler binding.
    Sampler {
        /// The binding slot.
        binding: u32,
        /// The `GpuSampler` handle.
        sampler: JsValue,
    },
}

/// The resource kind bound at a single slot of a `BindGroupLayoutEntry`.
#[derive(Clone, Debug)]
pub enum BindGroupEntryType {
    /// `GpuBufferBindingLayout { type: "uniform" }`.
    UniformBuffer,
    /// `GpuBufferBindingLayout { type: "storage" | "read-only-storage" }`.
    StorageBuffer {
        /// `true` → `"read-only-storage"`, `false` → `"storage"`.
        read_only: bool,
    },
    /// `GpuTextureBindingLayout`.
    SampledTexture {
        /// One of `"float"`, `"unfilterable-float"`, `"depth"`, `"sint"`, `"uint"`.
        sample_type: String,
        /// `true` if the bound texture is multisampled (matches MSAA render-target sampling).
        multisampled: bool,
    },
    /// `GpuStorageTextureBindingLayout`.
    StorageTexture {
        /// `true` → `"read-only"`, `false` → `"read-write"`.
        read_only: bool,
        /// Texture format string (e.g. `"rgba8unorm"`, `"r32float"`).
        format: String,
    },
    /// `GpuSamplerBindingLayout`.
    Sampler {
        /// `true` for filtering samplers (linear interpolation).
        filtering: bool,
        /// `true` for comparison samplers (depth-texture sampling).
        comparison: bool,
    },
}
