use super::*;

/// Single 3D mesh vertex storing local position and unit surface normal.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct Vertex3D {
    /// Local-space 3D position.
    #[get(type(copy))]
    pub(crate) position: Vec3,
    /// Local-space unit surface normal.
    #[get(type(copy))]
    pub(crate) normal: Vec3,
}

/// Physically based metallic-roughness material parameters for 3D surfaces.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct PbrMaterial {
    /// Base linear albedo RGB color.
    #[get(type(copy))]
    pub(crate) base_color: Vec3,
    /// Metallic factor in `[0.0, 1.0]` (`1.0` = conductor/mirror/chrome).
    #[get(type(copy))]
    pub(crate) metallic: f32,
    /// Surface roughness in `[0.04, 1.0]` (`0.04` = mirror gloss, `1.0` = matte).
    #[get(type(copy))]
    pub(crate) roughness: f32,
    /// Emissive HDR RGB glow (neon windows, headlights, streetlights).
    #[get(type(copy))]
    pub(crate) emissive: Vec3,
    /// Reflectivity multiplier used for ray-traced reflection blending (`0.0..=1.0`).
    #[get(type(copy))]
    pub(crate) reflectivity: f32,
}

/// Indexed triangle mesh with local AABB and default PBR material.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct Mesh3D {
    /// Human-readable asset name.
    pub(crate) name: String,
    /// Contiguous vertex buffer.
    pub(crate) vertices: Vec<Vertex3D>,
    /// Triangle index triplets `[i0, i1, i2]`.
    pub(crate) indices: Vec<[u32; 3]>,
    /// Precomputed local-space bounding box.
    #[get(type(copy))]
    pub(crate) bounds: Aabb3D,
    /// Default PBR surface material.
    #[get(type(copy))]
    pub(crate) material: PbrMaterial,
}

/// Multi-level Level-of-Detail (LOD) mesh chain (`LOD0` full detail through `LOD2` distant proxy).
#[derive(Clone, Data, Debug, PartialEq)]
pub struct MeshLodChain {
    /// Ordered detail levels from highest (`[0]`) to lowest (`[len - 1]`).
    pub(crate) levels: Vec<Mesh3D>,
}

/// Result returned by `ModelImporter3D` containing imported mesh LOD chains and telemetry.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct ImportedModel3D {
    /// Model name extracted from source or fallback label.
    pub(crate) name: String,
    /// Source interchange format.
    #[get(type(copy))]
    pub(crate) format: ModelFormat,
    /// Imported mesh LOD chains.
    pub(crate) meshes: Vec<MeshLodChain>,
    /// Total imported `LOD0` vertices.
    #[get(type(copy))]
    pub(crate) total_vertices: usize,
    /// Total imported `LOD0` triangles.
    #[get(type(copy))]
    pub(crate) total_triangles: usize,
}

/// Stateless 3D model importer supporting Wavefront `.obj` and glTF 2.0 `.gltf` JSON assets.
#[derive(Clone, Copy, Debug, Default)]
pub struct ModelImporter3D;
