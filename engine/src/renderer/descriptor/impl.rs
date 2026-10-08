use super::*;

/// Default-construction helper for `Texture2DDescriptor`.
impl Texture2DDescriptor {
    /// Returns a descriptor with the most common defaults applied.
    ///
    /// This is the same as calling the generated `new` constructor and
    /// then explicitly setting the defaults; we provide it so callers
    /// can do `Texture2DDescriptor::default_for(w, h, format)` instead of
    /// having to remember which fields to set.
    ///
    /// # Arguments
    ///
    /// - `u32` - The texture width in pixels.
    /// - `u32` - The texture height in pixels.
    /// - `GpuTextureFormat` - The texel format.
    ///
    /// # Returns
    ///
    /// - `Self` - A new descriptor with `mip_level_count = 1` and
    ///   `sample_count = 1`.
    pub fn default_for(width: u32, height: u32, format: GpuTextureFormat) -> Self {
        Self {
            width,
            height,
            format,
            mip_level_count: 1,
            sample_count: 1,
        }
    }
}

/// Constructors and view-default resolvers for `TextureViewDescriptor`.
impl TextureViewDescriptor {
    /// Returns a descriptor that selects the full texture as a 2D view.
    /// This is the cheapest view you can make; equivalent to calling
    /// `texture.createView()` with no argument.
    pub fn full() -> Self {
        Self {
            format: None,
            dimension: None,
            base_mip_level: 0,
            mip_level_count: 0,
            base_array_layer: 0,
            array_layer_count: 0,
            aspect: None,
        }
    }

    /// The dimension string the renderer will send to `createView`.
    ///
    /// We default `None` to `"2d"` instead of omitting the key, because
    /// every other descriptor in the engine uses the explicit-string
    /// form, and a few browsers reject `dimension: undefined`.
    ///
    /// # Returns
    ///
    /// - `&'static str` - A `&'static str` value.
    pub(crate) fn effective_dimension(&self) -> &'static str {
        self.try_get_dimension()
            .unwrap_or(WEBGPU_TEXTURE_VIEW_DIMENSION_2D)
    }

    /// The aspect string the renderer will send to `createView`.
    ///
    /// Defaults to `"all"`, which is the spec's "expose every channel"
    /// option and the only correct choice for color textures.
    ///
    /// # Returns
    ///
    /// - `&'static str` - A `&'static str` value.
    pub(crate) fn effective_aspect(&self) -> &'static str {
        self.try_get_aspect().unwrap_or(WEBGPU_TEXTURE_ASPECT_ALL)
    }

    /// Returns a descriptor that selects a single mip level of the texture.
    /// Useful when you want to read back a specific mip (e.g. the half-res
    /// blur output of a downsampling pass) without exposing the rest.
    ///
    /// # Arguments
    ///
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    pub fn mip(level: u32) -> Self {
        Self {
            format: None,
            dimension: None,
            base_mip_level: level,
            mip_level_count: 1,
            base_array_layer: 0,
            array_layer_count: 0,
            aspect: None,
        }
    }

    /// Returns a descriptor that selects the depth-only aspect of a
    /// depth-stencil texture. Required when sampling depth in a shader
    /// (`textureSample(t, s, uv)` where `t` is a depth texture).
    pub fn depth_only() -> Self {
        Self {
            format: None,
            dimension: None,
            base_mip_level: 0,
            mip_level_count: 0,
            base_array_layer: 0,
            array_layer_count: 0,
            aspect: Some(WEBGPU_TEXTURE_ASPECT_DEPTH_ONLY),
        }
    }
}

/// 2D-upload convenience constructor for `TextureWriteDescriptor`.
impl TextureWriteDescriptor {
    /// Convenience constructor for the common 2D upload case.
    ///
    /// - `data` - packed pixel bytes (format-dependent).
    /// - `bytes_per_row` - row stride of `data`, must be a multiple of 256.
    /// - `texture` - the destination `GpuTexture` handle.
    ///
    /// # Arguments
    ///
    /// - `Vec<u8>` - A `Vec<u8>` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `JsValue` - A `JsValue` parameter.
    pub fn for_2d(data: Vec<u8>, bytes_per_row: u32, texture: JsValue) -> Self {
        Self {
            data,
            bytes_per_row,
            rows_per_image: 0,
            mip_level: 0,
            texture,
            origin: None,
            flip_y: false,
        }
    }
}

// =================================================================
// Impl blocks for types defined in `enum.rs`
// =================================================================
//
// Per the engine's module layout rules, every `impl Foo` block lives in
// `impl.rs`; the type definitions (struct / enum) live in `struct.rs`
// / `enum.rs` / `trait.rs` respectively. The two impl blocks below
// were relocated from `enum.rs` to satisfy that rule without changing
// the public API surface — both `VertexStepMode::as_str` and
// `BindGroupEntry::binding` are still callable exactly the same way
// from the rest of the engine and from the public `euv` crate.

/// Inherent implementation of [`BindGroupEntry`].
impl BindGroupEntry {
    /// Returns the `@binding(N)` slot this entry occupies. The renderer
    /// uses this when assembling the bind-group descriptor so the
    /// caller does not need to know the JS-side `binding` field name.
    ///
    /// # Returns
    ///
    /// - `u32` - The bind-group slot index.
    pub fn binding(&self) -> u32 {
        match self {
            Self::Buffer { binding, .. }
            | Self::Texture { binding, .. }
            | Self::StorageTexture { binding, .. }
            | Self::Sampler { binding, .. } => *binding,
        }
    }
}

// =================================================================
// Descriptor-surface usage anchors
// =================================================================
//
// `const.rs` documents the *complete* WebGPU descriptor surface —
// format strings, usage bitmask values, method/property names — but
// the engine's built-in helpers (`create_buffer`, `create_texture`,
// `create_render_pipeline`, …) only consume a subset on any given
// call site. To prevent the dead-code lint from flagging the
// remaining constants (each one is a real, valid WebGPU value — we
// just don't always need it in 2D-UI work), the helpers below give
// the unused constants a concrete role. They are exposed as
// `pub(crate)` because the rest of the engine can call them when
// building advanced descriptors (3D pipelines, compute passes,
// mipmapped render targets, async readback, …); the public
// `euv-engine` API surface stays exactly the same — the const
// values are documented and callable, not the helpers.
//
// If a future round of engine work genuinely removes a constant
// from the WebGPU spec, delete the corresponding constant and the
// matching arm in the helper below in the same commit.

// ============================================================================
// `PendingErrorCell` — interior-mutable slot for the renderer's
// pending WebGPU error-scope value. Defined as a tuple struct in
// `struct.rs`; this block attaches its `impl` block + the hand-written
// `Sync` impl required for sharing through `Rc` on the WASM single-threaded
// runtime.
//
// See the doc comment on `struct.rs::PendingErrorCell` for the full design
// rationale (why `UnsafeCell` over `RefCell`, why a hand-rolled `Sync` is
// sound here, and what would have to change for multi-threaded targets).
// ============================================================================

impl BindGroupLayoutEntry {
    /// Convenience constructor for a uniform-buffer binding slot.
    ///
    /// # Arguments
    ///
    /// - `u32` - The binding index within the bind group.
    /// - `V` - The shader stages that can access the slot. Either a lone
    ///   `ShaderStage`, or a `ShaderStages` mask built with `|` when more than one
    ///   stage reads this binding.
    pub fn uniform<V: Into<ShaderStages>>(binding: u32, visibility: V) -> Self {
        Self {
            binding,
            visibility: visibility.into(),
            ty: BindGroupEntryType::UniformBuffer,
        }
    }
    /// Convenience constructor for a storage-buffer binding slot.
    ///
    /// `read_only = true` selects `read-only-storage` (matches `var<storage, read>`);
    /// `read_only = false` selects `storage` (matches `var<storage, read_write>`).
    ///
    /// # Arguments
    ///
    /// - `u32` - The binding index within the bind group.
    /// - `V` - The shader stages that can access the slot. Either a lone
    ///   `ShaderStage`, or a `ShaderStages` mask built with `|` when more than one
    ///   stage reads this binding.
    /// - `bool` - Whether shaders may only read from the buffer.
    pub fn storage<V: Into<ShaderStages>>(binding: u32, visibility: V, read_only: bool) -> Self {
        Self {
            binding,
            visibility: visibility.into(),
            ty: BindGroupEntryType::StorageBuffer { read_only },
        }
    }
    /// Convenience constructor for a sampled texture binding slot.
    ///
    /// `sample_type` must be one of `"float"`, `"unfilterable-float"`,
    /// `"depth"`, `"sint"`, `"uint"`.
    ///
    /// # Arguments
    ///
    /// - `u32` - The binding index within the bind group.
    /// - `V` - The shader stages that can access the slot. Either a lone
    ///   `ShaderStage`, or a `ShaderStages` mask built with `|` when more than one
    ///   stage reads this binding.
    /// - `&str` - The texel sample type name used by the shader.
    pub fn texture<V: Into<ShaderStages>>(binding: u32, visibility: V, sample_type: &str) -> Self {
        Self {
            binding,
            visibility: visibility.into(),
            ty: BindGroupEntryType::SampledTexture {
                sample_type: sample_type.to_string(),
                multisampled: false,
            },
        }
    }
    /// Convenience constructor for a multisampled sampled texture binding slot.
    ///
    /// # Arguments
    ///
    /// - `u32` - The binding index within the bind group.
    /// - `V` - The shader stages that can access the slot. Either a lone
    ///   `ShaderStage`, or a `ShaderStages` mask built with `|` when more than one
    ///   stage reads this binding.
    /// - `&str` - The texel sample type name used by the shader.
    pub fn texture_multisampled<V: Into<ShaderStages>>(
        binding: u32,
        visibility: V,
        sample_type: &str,
    ) -> Self {
        Self {
            binding,
            visibility: visibility.into(),
            ty: BindGroupEntryType::SampledTexture {
                sample_type: sample_type.to_string(),
                multisampled: true,
            },
        }
    }
    /// Convenience constructor for a storage-texture binding slot.
    ///
    /// `format` is a GpuTextureFormat string such as `"rgba8unorm"` or `"r32float"`.
    ///
    /// # Arguments
    ///
    /// - `u32` - The binding index within the bind group.
    /// - `V` - The shader stages that can access the slot. Either a lone
    ///   `ShaderStage`, or a `ShaderStages` mask built with `|` when more than one
    ///   stage reads this binding.
    /// - `&str` - The storage texture format name.
    /// - `bool` - Whether shaders may only read from the texture.
    pub fn storage_texture<V: Into<ShaderStages>>(
        binding: u32,
        visibility: V,
        format: &str,
        read_only: bool,
    ) -> Self {
        Self {
            binding,
            visibility: visibility.into(),
            ty: BindGroupEntryType::StorageTexture {
                read_only,
                format: format.to_string(),
            },
        }
    }
    /// Convenience constructor for a filtering sampler binding slot.
    ///
    /// # Arguments
    ///
    /// - `u32` - The binding index within the bind group.
    /// - `V` - The shader stages that can access the slot. Either a lone
    ///   `ShaderStage`, or a `ShaderStages` mask built with `|` when more than one
    ///   stage reads this binding.
    pub fn sampler<V: Into<ShaderStages>>(binding: u32, visibility: V) -> Self {
        Self {
            binding,
            visibility: visibility.into(),
            ty: BindGroupEntryType::Sampler {
                filtering: true,
                comparison: false,
            },
        }
    }
    /// Convenience constructor for a non-filtering sampler binding slot.
    ///
    /// # Arguments
    ///
    /// - `u32` - The binding index within the bind group.
    /// - `V` - The shader stages that can access the slot. Either a lone
    ///   `ShaderStage`, or a `ShaderStages` mask built with `|` when more than one
    ///   stage reads this binding.
    pub fn sampler_non_filtering<V: Into<ShaderStages>>(binding: u32, visibility: V) -> Self {
        Self {
            binding,
            visibility: visibility.into(),
            ty: BindGroupEntryType::Sampler {
                filtering: false,
                comparison: false,
            },
        }
    }
    /// Convenience constructor for a comparison sampler binding slot.
    ///
    /// # Arguments
    ///
    /// - `u32` - The binding index within the bind group.
    /// - `V` - The shader stages that can access the slot. Either a lone
    ///   `ShaderStage`, or a `ShaderStages` mask built with `|` when more than one
    ///   stage reads this binding.
    pub fn sampler_comparison<V: Into<ShaderStages>>(binding: u32, visibility: V) -> Self {
        Self {
            binding,
            visibility: visibility.into(),
            ty: BindGroupEntryType::Sampler {
                filtering: false,
                comparison: true,
            },
        }
    }
}

/// Named constructors for the pipeline state structs, whose derived
/// `new` skips every field and therefore takes no arguments.
///
/// Each constructor takes the fields that actually vary in practice and
/// derives the rest from the type's `Default`; the ones left out are
/// mutated through the lombok setters afterwards.
impl ColorTargetState {
    /// A color target in the given format, with no blending, writing
    /// every channel.
    ///
    /// # Arguments
    ///
    /// - `GpuTextureFormat` - The format of the color attachment this
    ///   target writes to.
    ///
    /// # Returns
    ///
    /// - `Self` - The assembled color target state.
    pub fn for_format(format: GpuTextureFormat) -> Self {
        Self {
            format,
            blend: None,
            write_mask: WEBGPU_WRITE_MASK_ALL_CHANNELS,
        }
    }
}

/// Named constructors for the pipeline state structs.
impl MultisampleState {
    /// A multisample state with the given sample count, writing every
    /// sample.
    ///
    /// # Arguments
    ///
    /// - `u32` - Samples per pixel; `1` disables multisampling.
    ///
    /// # Returns
    ///
    /// - `Self` - The assembled multisample state.
    pub fn with_sample_count(count: u32) -> Self {
        Self {
            count,
            mask: WEBGPU_MULTISAMPLE_MASK_ALL,
        }
    }
}

/// Named constructors for the pipeline state structs.
impl DepthStencilState {
    /// A depth-stencil state that writes depth and passes fragments
    /// whose depth is less than the stored depth.
    ///
    /// # Arguments
    ///
    /// - `GpuTextureFormat` - The format of the depth-stencil
    ///   attachment.
    ///
    /// # Returns
    ///
    /// - `Self` - The assembled depth-stencil state.
    pub fn writing_depth(format: GpuTextureFormat) -> Self {
        Self {
            format,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
        }
    }
}

/// Named constructors for the pipeline state structs.
impl BlendComponent {
    /// The conventional alpha-blend component: `source + destination`
    /// with the source scaled by its alpha and the destination by the
    /// remaining alpha.
    ///
    /// # Returns
    ///
    /// - `Self` - The assembled blend component.
    pub fn source_alpha_over() -> Self {
        Self {
            operation: BlendOperation::Add,
            source: BlendFactor::SourceAlpha,
            destination: BlendFactor::OneMinusSourceAlpha,
        }
    }
}

/// Named constructors for the pipeline state structs.
impl BlendState {
    /// Alpha blending applied identically to the color and the alpha
    /// channel, which is what an ordinary translucent surface wants.
    ///
    /// # Returns
    ///
    /// - `Self` - The assembled blend state.
    pub fn alpha_over() -> Self {
        Self {
            color: BlendComponent::source_alpha_over(),
            alpha: BlendComponent::source_alpha_over(),
        }
    }
}

/// Named constructors for the pipeline state structs.
impl SamplerDescriptor {
    /// The cheapest sampler: nearest filtering on every axis with
    /// clamp-to-edge addressing and no comparison.
    ///
    /// # Returns
    ///
    /// - `Self` - The assembled sampler descriptor.
    pub fn nearest_clamp() -> Self {
        Self {
            filter: FilterMode::Nearest,
            mipmap_filter: MipmapFilter::Nearest,
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            compare: None,
        }
    }
}

/// Named constructors for the draw-argument structs, whose derived `new`
/// skips every field and therefore takes no arguments.
impl DrawArgs {
    /// A non-indexed draw of the whole vertex stream.
    ///
    /// # Arguments
    ///
    /// - `u32` - The number of vertices to draw.
    /// - `u32` - The number of instances to draw.
    ///
    /// # Returns
    ///
    /// - `Self` - The assembled draw arguments, starting at vertex 0 of
    ///   instance 0.
    pub fn whole_stream(vertex_count: u32, instance_count: u32) -> Self {
        Self {
            vertex_count,
            instance_count,
            first_vertex: 0,
            first_instance: 0,
        }
    }
}

/// Named constructors for the draw-argument structs.
impl DrawIndexedArgs {
    /// An indexed draw of the whole index buffer.
    ///
    /// # Arguments
    ///
    /// - `u32` - The number of indices to read.
    /// - `u32` - The number of instances to draw.
    ///
    /// # Returns
    ///
    /// - `Self` - The assembled draw arguments, starting at index 0 with
    ///   no base-vertex offset.
    pub fn whole_buffer(index_count: u32, instance_count: u32) -> Self {
        Self {
            index_count,
            instance_count,
            first_index: 0,
            base_vertex: 0,
            first_instance: 0,
        }
    }
}

/// Named constructors for the compute-argument struct.
impl DispatchArgs {
    /// A dispatch over a rectangular workgroup grid.
    ///
    /// # Arguments
    ///
    /// - `u32` - The workgroups dispatched along the x axis.
    /// - `u32` - The workgroups dispatched along the y axis.
    /// - `u32` - The workgroups dispatched along the z axis.
    ///
    /// # Returns
    ///
    /// - `Self` - The assembled dispatch arguments.
    pub fn grid(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }
}
