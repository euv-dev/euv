use super::*;

/// Projected clip-space coordinates paired with world-space vertex position for perspective-correct near-plane clipping.
pub(crate) type ClipVertex3D = ([f32; 4], Vec3);

/// Single near-plane clipped 3D triangle carrying three `(clip_pos, world_pos)` vertices.
pub(crate) type ClippedTriangle3D = [ClipVertex3D; 3];

/// Four precomputed `(dir, inv_dir)` ray tuples for 2x2 soft-shadow or Lumen GI/AO patterns.
pub(crate) type GiRayQuad = [(Vec3, Vec3); 4];

/// Read-only depth, G-Buffer triangle IDs, shading records, precomputed GI rays, and SAH BVH consumed by `Renderer3D::trace_probe_band`.
pub(crate) type ProbeTraceInputs<'a> = (
    &'a [f32],
    &'a [u32],
    &'a [GShadeTri],
    &'a [GiRayQuad],
    &'a Bvh3D,
);

/// Read-only color, luma, G-Buffer triangle IDs, quarter-res HDR bloom, active bloom tile mask, and TAA history slices consumed by `Renderer3D::post_process_band`.
pub(crate) type PostProcessInputs<'a> = (
    &'a [u8],
    &'a [u8],
    &'a [u32],
    &'a [[f32; 3]],
    &'a [u8],
    &'a [u8],
);

/// Type-erased chunk dispatch trampoline signature used by the persistent 3D worker pool.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type Engine3dChunkFn = fn(usize, usize, usize);
