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
    /// Optional procedural surface detail overlay evaluated during deferred shading.
    #[get(type(copy))]
    pub(crate) surface_detail: SurfaceDetailMode,
}

/// Ergonomic descriptor for constructing an axis-aligned box `Mesh3D`.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct BoxMeshDesc {
    /// Human-readable mesh name.
    pub(crate) name: String,
    /// Box extents `(width, height, depth)` along `(X, Y, Z)`.
    #[get(type(copy))]
    pub(crate) size: Vec3,
    /// When `true`, base rests on `y = 0`; otherwise centered at the origin.
    #[get(type(copy))]
    pub(crate) ground_pivot: bool,
    /// Surface PBR material.
    #[get(type(copy))]
    pub(crate) material: PbrMaterial,
}

/// Ergonomic descriptor for constructing a subdivided ground plane `Mesh3D`.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct GroundPlaneDesc {
    /// Human-readable mesh name.
    pub(crate) name: String,
    /// Plane width along X.
    #[get(type(copy))]
    pub(crate) width: f32,
    /// Plane depth along Z.
    #[get(type(copy))]
    pub(crate) depth: f32,
    /// Grid subdivisions per axis (clamped to `1..=16`).
    #[get(type(copy))]
    pub(crate) subdivisions: usize,
    /// Surface PBR material.
    #[get(type(copy))]
    pub(crate) material: PbrMaterial,
}

/// Nanite-style virtualized geometry cluster storing tight bounds, normal cone, and triangle slice.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct Meshlet3D {
    /// First triangle index inside `Mesh3D::indices`.
    #[get(type(copy))]
    pub(crate) first_tri: u32,
    /// Number of triangles belonging to this meshlet cluster (`<= 64`).
    #[get(type(copy))]
    pub(crate) tri_count: u32,
    /// Tight local-space bounding box enclosing all cluster vertices.
    #[get(type(copy))]
    pub(crate) bounds: Aabb3D,
    /// Average outward normal cone axis for backface cluster culling.
    #[get(type(copy))]
    pub(crate) cone_axis: Vec3,
    /// Minimum dot product between member face normals and `cone_axis`.
    #[get(type(copy))]
    pub(crate) cone_cutoff: f32,
    /// Geometric screen-space error metric used for continuous LOD transitions.
    #[get(type(copy))]
    pub(crate) screen_error: f32,
}

/// Indexed triangle mesh with local AABB, Nanite-style meshlet clusters, and default PBR material.
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
    /// Partitioned Nanite-style meshlet clusters for hierarchical culling and LOD streaming.
    pub(crate) meshlets: Vec<Meshlet3D>,
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
