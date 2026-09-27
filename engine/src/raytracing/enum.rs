use super::*;

/// Identifies the geometric shape of a ray-tracing [`Occluder`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum OccluderKind {
    /// A sphere centered at `center` with radius stored in `extent.x`.
    #[default]
    Sphere,
    /// An axis-aligned bounding box with min corner `center` and max corner
    /// `extent`.
    Aabb,
    /// A triangle whose three vertices are stored in the occluder's
    /// `vertices` array; `center` and `extent` are unused.
    Triangle,
}
