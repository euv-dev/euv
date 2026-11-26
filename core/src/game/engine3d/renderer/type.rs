use super::*;

/// Projected clip-space coordinates paired with world-space vertex position for perspective-correct near-plane clipping.
pub(crate) type ClipVertex3D = ([f32; 4], Vec3);

/// Single near-plane clipped 3D triangle carrying three `(clip_pos, world_pos)` vertices.
pub(crate) type ClippedTriangle3D = [ClipVertex3D; 3];

/// Read-only color, luma, G-Buffer triangle IDs, quarter-res HDR bloom, and TAA history slices consumed by `Renderer3D::post_process_band`.
pub(crate) type PostProcessInputs<'a> = (&'a [u8], &'a [u8], &'a [u32], &'a [[f32; 3]], &'a [u8]);

/// Type-erased chunk dispatch trampoline signature used by the persistent 3D worker pool.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type Engine3dChunkFn = fn(usize, usize, usize);
