use super::*;

/// One or more pipeline stages a bind group layout slot is visible to.
///
/// WebGPU's `visibility` is a `GPUShaderStage` bitmask, not a single
/// stage: a uniform block that both the vertex and fragment stages read
/// is the most common binding there is, and naming one stage produces a
/// layout the other stage cannot see. [`ShaderStage`] alone cannot carry
/// that, so this newtype holds the combined mask while keeping the
/// variants spelled out rather than written as a raw `0x1 | 0x2`.
///
/// Build one with `|`, or from a single stage (which is what the
/// `Into` impl is for):
///
/// ```
/// use euv_engine::{ShaderStage, ShaderStages};
///
/// let both: ShaderStages = ShaderStage::Vertex | ShaderStage::Fragment;
/// let one: ShaderStages = ShaderStage::Compute.into();
/// assert!(both.contains(ShaderStage::Vertex));
/// assert_eq!(one.bits(), 0x4);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ShaderStages(pub(crate) u32);
