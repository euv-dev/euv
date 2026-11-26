use super::*;

/// Projected clip-space coordinates paired with world-space vertex position for perspective-correct near-plane clipping.
pub(crate) type ClipVertex3D = ([f32; 4], Vec3);

/// Single near-plane clipped 3D triangle carrying three `(clip_pos, world_pos)` vertices.
pub(crate) type ClippedTriangle3D = [ClipVertex3D; 3];

/// Read-only color, luma, quarter-res HDR bloom, and TAA history slices consumed by `Renderer3D::post_process_band`.
pub(crate) type PostProcessInputs<'a> = (&'a [u8], &'a [u8], &'a [[f32; 3]], &'a [u8]);
