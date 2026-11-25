/// Supported 3D model interchange formats parsed natively by `ModelImporter3D`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ModelFormat {
    /// Wavefront `.obj` ASCII geometry format (`v`, `vn`, `f` triangles/quads/n-gons).
    #[default]
    Obj,
    /// Khronos glTF 2.0 `.gltf` JSON descriptor with inline PBR materials and mesh geometry.
    GltfJson,
}
