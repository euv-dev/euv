use super::*;

/// Implements CSS composite operation string conversion for `BlendMode`.
impl BlendMode {
    /// Returns the CSS `globalCompositeOperation` string for this blend mode.
    ///
    /// # Returns
    ///
    /// - `&str` - The CSS composite operation string.
    pub fn to_css(&self) -> &str {
        match self {
            BlendMode::Normal => BLEND_MODE_NORMAL,
            BlendMode::Multiply => BLEND_MODE_MULTIPLY,
            BlendMode::Screen => BLEND_MODE_SCREEN,
            BlendMode::Lighter => BLEND_MODE_LIGHTER,
            BlendMode::Overlay => BLEND_MODE_OVERLAY,
            BlendMode::Darken => BLEND_MODE_DARKEN,
            BlendMode::Lighten => BLEND_MODE_LIGHTEN,
            BlendMode::ColorDodge => BLEND_MODE_COLOR_DODGE,
            BlendMode::ColorBurn => BLEND_MODE_COLOR_BURN,
            BlendMode::HardLight => BLEND_MODE_HARD_LIGHT,
            BlendMode::SoftLight => BLEND_MODE_SOFT_LIGHT,
            BlendMode::Difference => BLEND_MODE_DIFFERENCE,
            BlendMode::Exclusion => BLEND_MODE_EXCLUSION,
            BlendMode::Hue => BLEND_MODE_HUE,
            BlendMode::Saturation => BLEND_MODE_SATURATION,
            BlendMode::Color => BLEND_MODE_COLOR,
            BlendMode::Luminosity => BLEND_MODE_LUMINOSITY,
        }
    }
}

/// Inherent implementation of [`VertexStepMode`].
impl VertexStepMode {
    /// Returns the WGSL / WebGPU string representation.
    ///
    /// # Returns
    ///
    /// - `&'static str` - A static `&str` representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertex => WEBGPU_VERTEX_STEP_MODE_VERTEX,
            Self::Instance => WEBGPU_VERTEX_STEP_MODE_INSTANCE,
        }
    }
}

/// Combines two buffer uses into the bitmask a `GpuBufferDescriptor`
/// expects.
///
/// A buffer may be used for several purposes at once, and the WebGPU
/// `usage` field is a bitmask rather than an enum, so combining uses is
/// the normal case rather than the exception. The `|` operator is the
/// only way the engine combines them, which keeps every bit value in
/// exactly one place.
impl BitOr for BufferUsage {
    /// The combined `usage` bitmask.
    type Output = u32;

    /// Combines two buffer uses into their `usage` bitmask.
    ///
    /// # Arguments
    ///
    /// - `Self` - The right-hand use.
    ///
    /// # Returns
    ///
    /// - `Self::Output` - The bitmask carrying both uses.
    fn bitor(self, rhs: Self) -> Self::Output {
        buffer_usage_bit(self) | buffer_usage_bit(rhs)
    }
}

/// Combines two texture uses into the bitmask a `GpuTextureDescriptor`
/// expects.
///
/// The texture counterpart of the `|` operator on [`BufferUsage`], with
/// the same single-source-of-truth rationale.
impl BitOr for TextureUsage {
    /// The combined `usage` bitmask.
    type Output = u32;

    /// Combines two texture uses into their `usage` bitmask.
    ///
    /// # Arguments
    ///
    /// - `Self` - The right-hand use.
    ///
    /// # Returns
    ///
    /// - `Self::Output` - The bitmask carrying both uses.
    fn bitor(self, rhs: Self) -> Self::Output {
        texture_usage_bit(self) | texture_usage_bit(rhs)
    }
}

/// Combines two shader stages into the visibility bitmask a bind group
/// layout entry expects.
///
/// The third of the three bitmask-carrying WebGPU enums; see
/// [`BufferUsage`] for why combining is expressed as an operator rather
/// than a helper call.
impl BitOr for ShaderStage {
    /// The combined visibility bitmask.
    type Output = ShaderStages;

    /// Combines two shader stages into their visibility bitmask.
    ///
    /// # Arguments
    ///
    /// - `Self` - The right-hand stage.
    ///
    /// # Returns
    ///
    /// - `Self::Output` - The mask naming both stages.
    fn bitor(self, rhs: Self) -> Self::Output {
        ShaderStages(shader_stage_bit(self) | shader_stage_bit(rhs))
    }
}

/// The mask arithmetic behind [`ShaderStages`], kept next to the
/// [`BitOr`] impl for [`ShaderStage`] that produces one.
impl ShaderStages {
    /// The raw `GPUShaderStage` bitmask this mask stands for.
    ///
    /// # Returns
    ///
    /// - `u32` - The combined bits, ready for the wire.
    pub fn bits(self) -> u32 {
        self.0
    }

    /// Whether this mask names no stage at all.
    ///
    /// A bind group layout slot with an empty mask is visible to
    /// nothing, which the driver reports as a validation error rather
    /// than as a silently unreachable binding.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when no stage is named.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Whether one stage is among the stages this mask names.
    ///
    /// # Arguments
    ///
    /// - `ShaderStage` - The stage to test for.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the stage's bit is set.
    pub fn contains(self, stage: ShaderStage) -> bool {
        self.0 & shader_stage_bit(stage) == shader_stage_bit(stage)
    }
}

/// Lets a lone stage stand in where a mask is expected, so every
/// single-stage call site keeps reading as one.
impl From<ShaderStage> for ShaderStages {
    /// Widens one stage into a mask naming only that stage.
    ///
    /// # Arguments
    ///
    /// - `ShaderStage` - The stage the mask should name.
    ///
    /// # Returns
    ///
    /// - `ShaderStages` - A mask with that stage's single bit set.
    fn from(value: ShaderStage) -> Self {
        ShaderStages(shader_stage_bit(value))
    }
}

/// Adding a stage to a mask that already names others.
impl BitOr<ShaderStage> for ShaderStages {
    /// The widened mask.
    type Output = ShaderStages;

    /// Sets one more stage's bit, leaving the existing bits alone.
    ///
    /// # Arguments
    ///
    /// - `ShaderStage` - The stage to add.
    ///
    /// # Returns
    ///
    /// - `Self::Output` - The mask naming both sides.
    fn bitor(self, rhs: ShaderStage) -> Self::Output {
        ShaderStages(self.0 | shader_stage_bit(rhs))
    }
}

/// Combining two masks, which is what a caller reaches for when the two
/// halves of a layout entry are built independently.
impl BitOr for ShaderStages {
    /// The union of the two masks.
    type Output = ShaderStages;

    /// Sets every bit either side names.
    ///
    /// # Arguments
    ///
    /// - `Self` - The right-hand mask.
    ///
    /// # Returns
    ///
    /// - `Self::Output` - The mask naming both sides.
    fn bitor(self, rhs: Self) -> Self::Output {
        ShaderStages(self.0 | rhs.0)
    }
}

/// The `GPUBufferUsage` bit one use occupies.
///
/// The canonical WebGPU values, in one place: a buffer's legal uses
/// are a `GPUTextureUsage`-style bitmask, and getting a bit wrong
/// produces a buffer that validates in some draws and silently loses
/// the access in others. Keeping the table on the enum itself means a
/// call site cannot reference a stale duplicate constant.
impl BufferUsage {
    /// The `GPUBufferUsage` bit this use occupies.
    ///
    /// # Returns
    ///
    /// - `u32` - The bit this use occupies in a `GpuBufferDescriptor`
    ///   `usage`; combine several with `|`.
    pub fn usage_bit(self) -> u32 {
        buffer_usage_bit(self)
    }
}

/// The `GPUTextureUsage` bit one use occupies.
///
/// The texture-side counterpart of [`BufferUsage::usage_bit`]. Note
/// that the buffer and texture families share their variant *names*
/// but not their *values* - `CopySource` is `4` for a buffer and `1`
/// for a texture - which is why the two tables are separate.
impl TextureUsage {
    /// The `GPUTextureUsage` bit this use occupies.
    ///
    /// # Returns
    ///
    /// - `u32` - The bit this use occupies in a `GpuTextureDescriptor`
    ///   `usage`; combine several with `|`.
    pub fn usage_bit(self) -> u32 {
        texture_usage_bit(self)
    }
}

/// The `GPUShaderStage` bit one pipeline stage occupies.
impl ShaderStage {
    /// The `GPUShaderStage` bit this pipeline stage occupies.
    ///
    /// # Returns
    ///
    /// - `u32` - The bit this stage occupies in a
    ///   `BindGroupLayoutEntry` `visibility`; combine several with
    ///   `|`.
    pub fn stage_bit(self) -> u32 {
        shader_stage_bit(self)
    }
}
