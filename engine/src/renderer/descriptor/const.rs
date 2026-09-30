/// The sample mask that writes every sample of a multisampled target.
///
/// A 4x MSAA pipeline needs all four bits set (`0xf`); a single-sample
/// pipeline uses the same value, which is what the spec's default mask
/// is for any sample count.
pub(crate) const WEBGPU_MULTISAMPLE_MASK_ALL: u32 = 0xf;

/// Aspect selector that restricts a depth-stencil view to its depth channel.
pub(crate) const WEBGPU_TEXTURE_ASPECT_DEPTH_ONLY: &str = "depth-only";

/// Aspect selector that exposes every channel of a multi-aspect texture.
pub(crate) const WEBGPU_TEXTURE_ASPECT_ALL: &str = "all";

/// The channel write mask that lets the fragment stage write red, green,
/// blue, and alpha.
///
/// Four bits, one per channel; the right value for almost every opaque
/// fragment.
pub(crate) const WEBGPU_WRITE_MASK_ALL_CHANNELS: u32 = 0xf;
