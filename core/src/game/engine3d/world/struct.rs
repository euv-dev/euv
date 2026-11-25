use super::*;

/// 3D world-space transform (translation, Y-yaw rotation, non-uniform scale).
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct Transform3D {
    /// World-space translation `(x, y, z)`.
    #[get(type(copy))]
    pub(crate) position: Vec3,
    /// Horizontal Y-axis rotation in radians.
    #[get(type(copy))]
    pub(crate) yaw_rad: f32,
    /// Scale multiplier along `(x, y, z)`.
    #[get(type(copy))]
    pub(crate) scale: Vec3,
}

/// Instanced 3D scene object placed inside an open-world chunk.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct SceneObject3D {
    /// Human-readable object label.
    pub(crate) name: String,
    /// Index into `World3D::mesh_library` enabling GPU instancing batching.
    #[get(type(copy))]
    pub(crate) mesh_id: usize,
    /// World-space TRS transform.
    #[get(type(copy))]
    pub(crate) transform: Transform3D,
    /// Material override (or default mesh material).
    #[get(type(copy))]
    pub(crate) material: PbrMaterial,
    /// Cached world-space AABB for O(1) frustum culling and chunk streaming.
    #[get(type(copy))]
    pub(crate) world_bounds: Aabb3D,
    /// Spatial open-world chunk coordinate `(chunk_x, chunk_z)`.
    #[get(type(copy))]
    pub(crate) chunk_coord: (i32, i32),
    /// Whether this object casts ray-traced shadows and appears in BVH reflections.
    #[get(type(copy))]
    pub(crate) ray_trace_caster: bool,
    /// Continuous spin speed in radians/sec for animated hero models or props.
    #[get(type(copy))]
    pub(crate) spin_speed: f32,
}

/// Primary sun/moon directional light and atmospheric sky parameters.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct DirectionalLight3D {
    /// Normalized direction pointing toward the sun.
    #[get(type(copy))]
    pub(crate) direction: Vec3,
    /// Linear HDR sun radiance color.
    #[get(type(copy))]
    pub(crate) color: Vec3,
    /// Ambient sky zenith color.
    #[get(type(copy))]
    pub(crate) sky_zenith: Vec3,
    /// Ambient sky horizon and volumetric fog color.
    #[get(type(copy))]
    pub(crate) sky_horizon: Vec3,
    /// Time of day in `[0.0, 24.0]` hours.
    #[get(type(copy))]
    pub(crate) time_of_day: f32,
}

/// Complete open-world 3D scene graph with mesh instancing library, chunk streaming, and BVH acceleration.
#[derive(Clone, Data, Debug)]
pub struct World3D {
    /// Shared LOD mesh asset library referenced by `SceneObject3D::mesh_id`.
    pub(crate) mesh_library: Vec<MeshLodChain>,
    /// All world objects partitioned into open-world chunks.
    pub(crate) objects: Vec<SceneObject3D>,
    /// Active 3D perspective camera.
    #[get(type(copy))]
    pub(crate) camera: Camera3D,
    /// Active directional sun/sky lighting.
    #[get(type(copy))]
    pub(crate) sun: DirectionalLight3D,
    /// Flat BVH built over world-space triangles for ray-traced shadows, reflections, and GI.
    pub(crate) bvh: Bvh3D,
    /// Dirty flag triggering automatic BVH rebuild on geometry or transform changes.
    #[get(type(copy))]
    pub(crate) bvh_dirty: bool,
    /// Open-world chunk size in meters (`64.0m` per city block).
    #[get(type(copy))]
    pub(crate) chunk_size: f32,
    /// Index of the drivable player vehicle object if spawned.
    #[get(skip)]
    pub(crate) player_vehicle_index: Option<usize>,
    /// Total imported external 3D models (`.obj` / `.gltf`).
    #[get(type(copy))]
    pub(crate) imported_model_count: usize,
    /// Active 3D camera mode (`0 = Chase`, `1 = Hood`, `2 = Orbit`, `3 = Aerial`).
    #[get(type(copy))]
    pub(crate) camera_mode: u8,
    /// Continuous orbit angle in radians used when `camera_mode == 2`.
    #[get(type(copy))]
    pub(crate) camera_orbit_rad: f32,
}
