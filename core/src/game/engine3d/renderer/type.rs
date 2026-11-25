use super::*;

/// Projected clip-space coordinates paired with world-space vertex position for perspective-correct near-plane clipping.
pub(crate) type ClipVertex3D = ([f32; 4], Vec3);

/// Single near-plane clipped 3D triangle carrying three `(clip_pos, world_pos)` vertices.
pub(crate) type ClippedTriangle3D = [ClipVertex3D; 3];
