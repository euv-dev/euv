/// Supported 3D model interchange formats parsed natively by `ModelImporter3D`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ModelFormat {
    /// Wavefront `.obj` ASCII geometry format (`v`, `vn`, `f` triangles/quads/n-gons).
    #[default]
    Obj,
    /// Khronos glTF 2.0 `.gltf` JSON descriptor with inline PBR materials and mesh geometry.
    GltfJson,
}

/// Optional procedural micro-detail shading mode attached to a `PbrMaterial`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SurfaceDetailMode {
    /// Pure physically based Cook-Torrance GGX material without procedural pattern overlays.
    #[default]
    None,
    /// Procedural glass facade grid with emissive office windows for architectural towers.
    ArchitecturalFacade,
    /// Procedural urban road surface with curb/sidewalk borders and asphalt micro-grain.
    RoadSurface,
}
