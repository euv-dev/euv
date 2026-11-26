use super::*;

/// Default identity `Transform3D`.
impl Default for Transform3D {
    /// Returns identity transform at origin with unit scale.
    ///
    /// # Returns
    ///
    /// - `Self` - Identity `Transform3D`.
    fn default() -> Self {
        Self {
            position: Vec3::zero(),
            yaw_rad: 0.0,
            pitch_rad: 0.0,
            roll_rad: 0.0,
            scale: Vec3::splat(1.0),
        }
    }
}

/// Matrix and point transformation helpers for `Transform3D`.
impl Transform3D {
    /// Constructs a `Transform3D` from position, Y-axis yaw, and non-uniform scale.
    ///
    /// # Arguments
    ///
    /// - `Vec3` - World-space translation.
    /// - `f32` - Horizontal Y-axis yaw rotation in radians.
    /// - `Vec3` - Non-uniform scale.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed 3D transform.
    #[must_use]
    pub const fn new(position: Vec3, yaw_rad: f32, scale: Vec3) -> Self {
        Self {
            position,
            yaw_rad,
            pitch_rad: 0.0,
            roll_rad: 0.0,
            scale,
        }
    }

    /// Returns a copy of this transform with full yaw, pitch, and roll Euler rotation in radians.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source transform.
    /// - `f32` - Y-axis yaw in radians.
    /// - `f32` - X-axis pitch in radians.
    /// - `f32` - Z-axis roll in radians.
    ///
    /// # Returns
    ///
    /// - `Self` - Rotated 3D transform.
    #[must_use]
    pub fn with_euler(mut self, yaw_rad: f32, pitch_rad: f32, roll_rad: f32) -> Self {
        self.set_yaw_rad(yaw_rad);
        self.set_pitch_rad(pitch_rad);
        self.set_roll_rad(roll_rad);
        self
    }

    /// Linearly interpolates position, Euler angles, and scale between `self` and `other`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Start transform.
    /// - `Self` - End transform.
    /// - `f32` - Interpolation factor in `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Interpolated transform.
    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let k: f32 = t.clamp(0.0, 1.0);
        Self {
            position: self.get_position().lerp(other.get_position(), k),
            yaw_rad: self.get_yaw_rad() + (other.get_yaw_rad() - self.get_yaw_rad()) * k,
            pitch_rad: self.get_pitch_rad() + (other.get_pitch_rad() - self.get_pitch_rad()) * k,
            roll_rad: self.get_roll_rad() + (other.get_roll_rad() - self.get_roll_rad()) * k,
            scale: self.get_scale().lerp(other.get_scale(), k),
        }
    }

    /// Converts this TRS transform into a column-major `Mat4`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Transform reference.
    ///
    /// # Returns
    ///
    /// - `Mat4` - Model matrix.
    #[must_use]
    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_trs_euler(
            self.get_position(),
            self.get_yaw_rad(),
            self.get_pitch_rad(),
            self.get_roll_rad(),
            self.get_scale(),
        )
    }

    /// Rotates vector `v` by this transform's `(yaw, pitch, roll)` Euler orientation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Transform reference.
    /// - `Vec3` - Input vector.
    ///
    /// # Returns
    ///
    /// - `Vec3` - Rotated vector.
    #[must_use]
    fn rotate_vec(&self, v: Vec3) -> Vec3 {
        let pitch: f32 = self.get_pitch_rad();
        let roll: f32 = self.get_roll_rad();
        let yaw: f32 = self.get_yaw_rad();
        let (cy, sy): (f32, f32) = (yaw.cos(), yaw.sin());
        if pitch.abs() < 1e-6 && roll.abs() < 1e-6 {
            return Vec3::new(
                v.get_x() * cy + v.get_z() * sy,
                v.get_y(),
                -v.get_x() * sy + v.get_z() * cy,
            );
        }
        let (cr, sr): (f32, f32) = (roll.cos(), roll.sin());
        let x1: f32 = v.get_x() * cr - v.get_y() * sr;
        let y1: f32 = v.get_x() * sr + v.get_y() * cr;
        let z1: f32 = v.get_z();
        let (cp, sp): (f32, f32) = (pitch.cos(), pitch.sin());
        let x2: f32 = x1;
        let y2: f32 = y1 * cp - z1 * sp;
        let z2: f32 = y1 * sp + z1 * cp;
        Vec3::new(x2 * cy + z2 * sy, y2, -x2 * sy + z2 * cy)
    }

    /// Transforms a local-space 3D point into world space.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Transform reference.
    /// - `Vec3` - Local-space point.
    ///
    /// # Returns
    ///
    /// - `Vec3` - World-space point.
    #[must_use]
    pub fn transform_point(&self, p: Vec3) -> Vec3 {
        let sc: Vec3 = self.get_scale();
        let scaled: Vec3 = Vec3::new(
            p.get_x() * sc.get_x(),
            p.get_y() * sc.get_y(),
            p.get_z() * sc.get_z(),
        );
        self.rotate_vec(scaled).add(self.get_position())
    }

    /// Rotates a local-space surface normal into world space.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Transform reference.
    /// - `Vec3` - Local normal.
    ///
    /// # Returns
    ///
    /// - `Vec3` - World-space unit normal.
    #[must_use]
    pub fn transform_normal(&self, n: Vec3) -> Vec3 {
        self.rotate_vec(n).normalize()
    }

    /// Transforms a local `Aabb3D` into an enclosing world-space `Aabb3D`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Transform reference.
    /// - `Aabb3D` - Local bounding box.
    ///
    /// # Returns
    ///
    /// - `Aabb3D` - Enclosing world-space bounding box.
    #[must_use]
    pub fn transform_aabb(&self, local: Aabb3D) -> Aabb3D {
        let mn: Vec3 = local.get_min();
        let mx: Vec3 = local.get_max();
        let corners: [Vec3; 8] = [
            Vec3::new(mn.get_x(), mn.get_y(), mn.get_z()),
            Vec3::new(mx.get_x(), mn.get_y(), mn.get_z()),
            Vec3::new(mn.get_x(), mx.get_y(), mn.get_z()),
            Vec3::new(mx.get_x(), mx.get_y(), mn.get_z()),
            Vec3::new(mn.get_x(), mn.get_y(), mx.get_z()),
            Vec3::new(mx.get_x(), mn.get_y(), mx.get_z()),
            Vec3::new(mn.get_x(), mx.get_y(), mx.get_z()),
            Vec3::new(mx.get_x(), mx.get_y(), mx.get_z()),
        ];
        let mut out: Aabb3D = Aabb3D::empty();
        for c in corners.into_iter() {
            out.expand_point(self.transform_point(c));
        }
        out
    }
}

/// Conversion helpers for `CameraMode3D`.
impl CameraMode3D {
    /// Converts a numeric mode index (`0..=5`) into a typed `CameraMode3D`.
    ///
    /// # Arguments
    ///
    /// - `u8` - Numeric camera mode index.
    ///
    /// # Returns
    ///
    /// - `Self` - Corresponding `CameraMode3D`.
    #[must_use]
    pub const fn from_u8(index: u8) -> Self {
        match index {
            1 => Self::Hood,
            2 => Self::Orbit,
            3 => Self::Aerial,
            4 => Self::FreeFly,
            5 => Self::CinematicDolly,
            _ => Self::Chase,
        }
    }

    /// Converts this `CameraMode3D` into its stable numeric index (`0..=5`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Camera mode variant.
    ///
    /// # Returns
    ///
    /// - `u8` - Numeric mode index.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Chase => 0,
            Self::Hood => 1,
            Self::Orbit => 2,
            Self::Aerial => 3,
            Self::FreeFly => 4,
            Self::CinematicDolly => 5,
        }
    }
}

/// Fluent builder helpers for `SpawnObjectDesc`.
impl SpawnObjectDesc {
    /// Creates an object spawn descriptor with identity transform, default PBR material, and BVH ray tracing enabled.
    ///
    /// # Arguments
    ///
    /// - `&str` - Human-readable object label.
    /// - `usize` - Registered mesh identifier.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready spawn descriptor.
    #[must_use]
    pub fn new(name: &str, mesh_id: usize) -> Self {
        Self {
            name: name.to_string(),
            mesh_id,
            transform: Transform3D::default(),
            material: PbrMaterial::default(),
            ray_trace_caster: true,
            spin_speed: 0.0,
        }
    }

    /// Sets the world-space transform for the spawned object.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base descriptor.
    /// - `Transform3D` - World-space transform.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated descriptor.
    #[must_use]
    pub fn with_transform(mut self, transform: Transform3D) -> Self {
        self.set_transform(transform);
        self
    }

    /// Sets the PBR surface material override for the spawned object.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base descriptor.
    /// - `PbrMaterial` - PBR material.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated descriptor.
    #[must_use]
    pub fn with_material(mut self, material: PbrMaterial) -> Self {
        self.set_material(material);
        self
    }

    /// Sets whether this object casts BVH shadows and appears in reflections.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base descriptor.
    /// - `bool` - Ray-trace caster toggle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated descriptor.
    #[must_use]
    pub fn with_ray_trace(mut self, ray_trace_caster: bool) -> Self {
        self.set_ray_trace_caster(ray_trace_caster);
        self
    }

    /// Sets continuous Y-axis spin speed in radians per second.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base descriptor.
    /// - `f32` - Spin speed in radians/sec.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated descriptor.
    #[must_use]
    pub fn with_spin(mut self, spin_speed: f32) -> Self {
        self.set_spin_speed(spin_speed);
        self
    }
}

/// Fluent builder helpers for `ImportModelDesc`.
impl ImportModelDesc {
    /// Creates an external model import descriptor with identity transform and default PBR fallback material.
    ///
    /// # Arguments
    ///
    /// - `ModelFormat` - Interchange format (`Obj` or `GltfJson`).
    /// - `&str` - Raw model source text.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready import descriptor.
    #[must_use]
    pub fn new(format: ModelFormat, source: &str) -> Self {
        Self {
            format,
            source: source.to_string(),
            transform: Transform3D::default(),
            fallback_material: PbrMaterial::default(),
            spin_speed: 0.0,
        }
    }

    /// Sets the placement transform for the imported model.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base descriptor.
    /// - `Transform3D` - Placement transform.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated descriptor.
    #[must_use]
    pub fn with_transform(mut self, transform: Transform3D) -> Self {
        self.set_transform(transform);
        self
    }

    /// Sets the fallback PBR material when source omits material metadata.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base descriptor.
    /// - `PbrMaterial` - Fallback PBR material.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated descriptor.
    #[must_use]
    pub fn with_material(mut self, fallback_material: PbrMaterial) -> Self {
        self.set_fallback_material(fallback_material);
        self
    }

    /// Sets continuous Y-axis turntable spin speed in radians per second.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base descriptor.
    /// - `f32` - Spin speed in radians/sec.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated descriptor.
    #[must_use]
    pub fn with_spin(mut self, spin_speed: f32) -> Self {
        self.set_spin_speed(spin_speed);
        self
    }
}

/// Dynamic Time-of-Day sun and atmospheric sky lighting for `DirectionalLight3D`.
impl DirectionalLight3D {
    /// Computes physically inspired sun direction, sun color, and sky colors for `hour` in `[0.0, 24.0]`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Hour of day (`6.0` dawn, `12.0` noon, `17.5` golden sunset, `22.0` neon night).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated atmospheric directional light.
    #[must_use]
    pub fn from_time_of_day(hour: f32) -> Self {
        let h: f32 = hour.rem_euclid(24.0);
        let elev_angle: f32 = ((h - 6.0) / 12.0 * PI).clamp(-0.35, 3.49);
        let sun_y: f32 = elev_angle.sin();
        let sun_x: f32 = elev_angle.cos();
        let dir: Vec3 = Vec3::new(sun_x * 0.85, sun_y.max(0.18), 0.52).normalize();
        if (7.5..=16.5).contains(&h) {
            Self {
                direction: dir,
                color: Vec3::new(2.2, 2.08, 1.92),
                sky_zenith: Vec3::new(0.14, 0.34, 0.72),
                sky_horizon: Vec3::new(0.62, 0.78, 0.94),
                time_of_day: h,
            }
        } else if (5.5..7.5).contains(&h) || (16.5..19.5).contains(&h) {
            Self {
                direction: dir,
                color: Vec3::new(2.4, 1.35, 0.72),
                sky_zenith: Vec3::new(0.15, 0.14, 0.38),
                sky_horizon: Vec3::new(0.92, 0.45, 0.28),
                time_of_day: h,
            }
        } else {
            Self {
                direction: Vec3::new(-0.4, 0.65, 0.55).normalize(),
                color: Vec3::new(0.55, 0.68, 1.15),
                sky_zenith: Vec3::new(0.02, 0.03, 0.09),
                sky_horizon: Vec3::new(0.08, 0.11, 0.24),
                time_of_day: h,
            }
        }
    }
}

/// Default golden-sunset directional light.
impl Default for DirectionalLight3D {
    /// Creates a golden-hour sunset sun configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Sunset `DirectionalLight3D`.
    fn default() -> Self {
        Self::from_time_of_day(17.2)
    }
}

/// Default empty 3D open world.
impl Default for World3D {
    /// Creates an empty `World3D` with 64m chunk streaming enabled.
    ///
    /// # Returns
    ///
    /// - `Self` - Empty `World3D`.
    fn default() -> Self {
        Self {
            mesh_library: Vec::new(),
            objects: Vec::new(),
            camera: Camera3D::default(),
            sun: DirectionalLight3D::default(),
            bvh: Bvh3D::default(),
            bvh_dirty: true,
            chunk_size: 64.0,
            player_vehicle_index: None,
            imported_model_count: 0,
            camera_mode: 0,
            camera_orbit_rad: 0.4,
        }
    }
}

/// Open-world construction, model importing, BVH rebuild, and live vehicle/camera simulation for `World3D`.
impl World3D {
    /// Registers a `MeshLodChain` in the instancing library and returns its `mesh_id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable world.
    /// - `MeshLodChain` - Multi-level LOD mesh chain.
    ///
    /// # Returns
    ///
    /// - `usize` - Assigned `mesh_id`.
    pub fn register_mesh(&mut self, chain: MeshLodChain) -> usize {
        let id: usize = self.get_mesh_library().len();
        self.get_mut_mesh_library().push(chain);
        id
    }

    /// Spawns an instanced `SceneObject3D` referencing `mesh_id` into its spatial world chunk.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable world.
    /// - `&str` - Object name.
    /// - `usize` - Registered `mesh_id`.
    /// - `Transform3D` - World transform.
    /// - `PbrMaterial` - Surface PBR material.
    /// - `bool` - Whether this object participates in BVH ray tracing.
    ///
    /// # Returns
    ///
    /// - `usize` - Index of the spawned `SceneObject3D`.
    pub fn spawn_object(
        &mut self,
        name: &str,
        mesh_id: usize,
        transform: Transform3D,
        material: PbrMaterial,
        ray_trace_caster: bool,
    ) -> usize {
        let local_bounds: Aabb3D = self
            .get_mesh_library()
            .get(mesh_id)
            .and_then(|c: &MeshLodChain| c.get_levels().first())
            .map(Mesh3D::get_bounds)
            .unwrap_or_else(|| Aabb3D::new(Vec3::splat(-1.0), Vec3::splat(1.0)));
        let world_bounds: Aabb3D = transform.transform_aabb(local_bounds);
        let cs: f32 = self.get_chunk_size().max(8.0);
        let pos: Vec3 = transform.get_position();
        let chunk_coord: (i32, i32) = (
            (pos.get_x() / cs).floor() as i32,
            (pos.get_z() / cs).floor() as i32,
        );
        let idx: usize = self.get_objects().len();
        self.get_mut_objects().push(SceneObject3D {
            name: name.to_string(),
            mesh_id,
            transform,
            material,
            world_bounds,
            chunk_coord,
            ray_trace_caster,
            spin_speed: 0.0,
        });
        self.set_bvh_dirty(true);
        idx
    }

    /// Imports an external `.obj` or `.gltf` model directly into the live 3D world and returns import telemetry.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable world.
    /// - `ModelFormat` - Source model format.
    /// - `&str` - Raw `.obj` or `.gltf` text content.
    /// - `Transform3D` - Placement transform in the 3D scene.
    /// - `PbrMaterial` - Fallback PBR material.
    /// - `f32` - Optional continuous Y-spin speed in radians/sec.
    ///
    /// # Returns
    ///
    /// - `Result<ImportedModel3D, String>` - Imported model summary or parse error.
    pub fn import_model(
        &mut self,
        format: ModelFormat,
        source: &str,
        transform: Transform3D,
        fallback_material: PbrMaterial,
        spin_speed: f32,
    ) -> Result<ImportedModel3D, String> {
        let imported: ImportedModel3D = ModelImporter3D::import(format, source, fallback_material)?;
        let model_name: String = imported.get_name().clone();
        for (i, chain) in imported.get_meshes().iter().cloned().enumerate() {
            let mat: PbrMaterial = chain
                .get_levels()
                .first()
                .map(Mesh3D::get_material)
                .unwrap_or(fallback_material);
            let mid: usize = self.register_mesh(chain);
            let obj_idx: usize =
                self.spawn_object(&format!("{model_name}_{i}"), mid, transform, mat, true);
            if let Some(obj) = self.get_mut_objects().get_mut(obj_idx) {
                obj.set_spin_speed(spin_speed);
            }
        }
        let next_cnt: usize = self.get_imported_model_count().saturating_add(1);
        self.set_imported_model_count(next_cnt);
        Ok(imported)
    }

    /// Spawns an object from a fluent `SpawnObjectDesc`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable world.
    /// - `&SpawnObjectDesc` - Spawn descriptor.
    ///
    /// # Returns
    ///
    /// - `usize` - Spawned object index.
    pub fn spawn_with(&mut self, desc: &SpawnObjectDesc) -> usize {
        let idx: usize = self.spawn_object(
            desc.get_name(),
            desc.get_mesh_id(),
            desc.get_transform(),
            desc.get_material(),
            desc.get_ray_trace_caster(),
        );
        if let Some(obj) = self.get_mut_objects().get_mut(idx) {
            obj.set_spin_speed(desc.get_spin_speed());
        }
        idx
    }

    /// Imports an external `.obj` or `.gltf` model using a fluent `ImportModelDesc`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable world.
    /// - `&ImportModelDesc` - Import descriptor.
    ///
    /// # Returns
    ///
    /// - `Result<ImportedModel3D, String>` - Imported model telemetry or parse error.
    pub fn import_with(&mut self, desc: &ImportModelDesc) -> Result<ImportedModel3D, String> {
        self.import_model(
            desc.get_format(),
            desc.get_source(),
            desc.get_transform(),
            desc.get_fallback_material(),
            desc.get_spin_speed(),
        )
    }

    /// Returns the typed `CameraMode3D` currently active in this world.
    ///
    /// # Arguments
    ///
    /// - `&Self` - World reference.
    ///
    /// # Returns
    ///
    /// - `CameraMode3D` - Active typed camera controller mode.
    #[must_use]
    pub fn active_camera_mode(&self) -> CameraMode3D {
        CameraMode3D::from_u8(self.get_camera_mode())
    }

    /// Sets the active 3D camera controller using a typed `CameraMode3D`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable world.
    /// - `CameraMode3D` - Desired camera controller mode.
    pub fn set_camera_mode_enum(&mut self, mode: CameraMode3D) {
        self.set_camera_mode(mode.as_u8());
    }

    /// Rebuilds the world-space BVH acceleration structure when geometry or transforms are dirty.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable world.
    pub fn ensure_bvh(&mut self) {
        if !self.get_bvh_dirty() {
            return;
        }
        let mut tris: Vec<BvhTriangle> = Vec::new();
        for obj in self.get_objects().iter() {
            if !obj.get_ray_trace_caster() {
                continue;
            }
            let Some(chain) = self.get_mesh_library().get(obj.get_mesh_id()) else {
                continue;
            };
            let Some(mesh) = chain.get_levels().first() else {
                continue;
            };
            let verts: &[Vertex3D] = mesh.get_vertices().as_slice();
            let tr: Transform3D = obj.get_transform();
            let mat: PbrMaterial = obj.get_material();
            for tri in mesh.get_indices().iter() {
                let (i0, i1, i2): (usize, usize, usize) =
                    (tri[0] as usize, tri[1] as usize, tri[2] as usize);
                if i0 < verts.len() && i1 < verts.len() && i2 < verts.len() {
                    let p0: Vec3 = tr.transform_point(verts[i0].get_position());
                    let p1: Vec3 = tr.transform_point(verts[i1].get_position());
                    let p2: Vec3 = tr.transform_point(verts[i2].get_position());
                    tris.push(BvhTriangle::from_points(p0, p1, p2, mat));
                }
            }
        }
        self.set_bvh(Bvh3D::build(tris));
        self.set_bvh_dirty(false);
    }

    /// Returns the optional player vehicle index.
    ///
    /// # Arguments
    ///
    /// - `&Self` - World reference.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - Vehicle index in `objects`.
    #[must_use]
    pub fn vehicle_index(&self) -> Option<usize> {
        let Self {
            player_vehicle_index,
            ..
        } = self;
        *player_vehicle_index
    }

    /// Builds a GTA-inspired open-world neon downtown city with reflective wet asphalt, glass towers, and a drivable sports car.
    ///
    /// # Returns
    ///
    /// - `Self` - Populated open-world 3D city ready for real-time PBR + Ray Tracing.
    #[must_use]
    pub fn create_open_world_city() -> Self {
        let mut world: Self = Self::default();
        let asphalt_mat: PbrMaterial = PbrMaterial::pbr(Color::hex(0x18202C), 0.25, 0.14, 0.58)
            .with_surface_detail(SurfaceDetailMode::RoadSurface);
        let road_mesh: usize = world.register_mesh(MeshLodChain::from_mesh(Mesh3D::ground_plane(
            "wet_asphalt_boulevard",
            220.0,
            220.0,
            24,
            asphalt_mat,
        )));
        let _: usize = world.spawn_object(
            "city_ground",
            road_mesh,
            Transform3D::default(),
            asphalt_mat,
            false,
        );
        let stripe_mat: PbrMaterial = PbrMaterial::neon(Color::hex(0xFACC15), 1.2);
        let stripe_mesh: usize = world.register_mesh(MeshLodChain::from_mesh(Mesh3D::box_mesh(
            "lane_stripe",
            0.35,
            0.05,
            5.0,
            true,
            stripe_mat,
        )));
        for iz in -7..=7 {
            let _: usize = world.spawn_object(
                &format!("stripe_{iz}"),
                stripe_mesh,
                Transform3D::new(
                    Vec3::new(0.0, 0.02, (iz as f32) * 14.0),
                    0.0,
                    Vec3::splat(1.0),
                ),
                stripe_mat,
                false,
            );
        }
        let tower_unit: usize = world.register_mesh(MeshLodChain::from_mesh(Mesh3D::box_mesh(
            "city_skyscraper",
            1.0,
            1.0,
            1.0,
            true,
            PbrMaterial::default(),
        )));
        let neon_unit: usize = world.register_mesh(MeshLodChain::from_mesh(Mesh3D::box_mesh(
            "neon_sign",
            1.0,
            1.0,
            1.0,
            true,
            PbrMaterial::neon(Color::hex(0x38BDF8), 1.8),
        )));
        let tower_palettes: [PbrMaterial; 4] = [
            PbrMaterial::pbr(Color::hex(0x1E293B), 0.72, 0.16, 0.65)
                .with_surface_detail(SurfaceDetailMode::ArchitecturalFacade),
            PbrMaterial::pbr(Color::hex(0x334155), 0.55, 0.24, 0.45)
                .with_surface_detail(SurfaceDetailMode::ArchitecturalFacade),
            PbrMaterial::pbr(Color::hex(0x0F172A), 0.85, 0.10, 0.78)
                .with_surface_detail(SurfaceDetailMode::ArchitecturalFacade),
            PbrMaterial::pbr(Color::hex(0x475569), 0.40, 0.35, 0.35)
                .with_surface_detail(SurfaceDetailMode::ArchitecturalFacade),
        ];
        let neon_colors: [Color; 4] = [
            Color::hex(0x38BDF8),
            Color::hex(0xF43F5E),
            Color::hex(0xA855F7),
            Color::hex(0x34D399),
        ];
        let mut idx_counter: usize = 0;
        for side in [-1.0_f32, 1.0_f32] {
            for row in -4..=4 {
                let z: f32 = (row as f32) * 20.0 - 12.0;
                let x: f32 = side * (16.5 + ((idx_counter % 3) as f32) * 3.0);
                let h: f32 = 16.0 + ((idx_counter * 7) % 5) as f32 * 8.5;
                let w: f32 = 9.0 + ((idx_counter % 2) as f32) * 3.0;
                let mat: PbrMaterial = tower_palettes[idx_counter % tower_palettes.len()];
                let _: usize = world.spawn_object(
                    &format!("tower_{idx_counter}"),
                    tower_unit,
                    Transform3D::new(Vec3::new(x, 0.0, z), 0.0, Vec3::new(w, h, 11.0)),
                    mat,
                    true,
                );
                let neon_mat: PbrMaterial =
                    PbrMaterial::neon(neon_colors[idx_counter % neon_colors.len()], 1.9);
                let _: usize = world.spawn_object(
                    &format!("crown_{idx_counter}"),
                    neon_unit,
                    Transform3D::new(
                        Vec3::new(x - side * (w * 0.5 + 0.28), h * 0.72, z),
                        0.0,
                        Vec3::new(0.36, 1.4, 8.2),
                    ),
                    neon_mat,
                    false,
                );
                idx_counter += 1;
            }
        }
        let car_mat: PbrMaterial = PbrMaterial::pbr(Color::hex(0xF43F5E), 0.88, 0.12, 0.72);
        let car_mesh: usize = world.register_mesh(MeshLodChain::from_mesh(Mesh3D::sports_car(
            "hero_gt", car_mat,
        )));
        let car_idx: usize = world.spawn_object(
            "player_gt_car",
            car_mesh,
            Transform3D::new(Vec3::new(0.0, 0.0, 6.0), 0.0, Vec3::splat(1.1)),
            car_mat,
            true,
        );
        world.set_player_vehicle_index(Some(car_idx));
        world.set_camera(Camera3D {
            position: Vec3::new(0.0, 6.2, 18.5),
            yaw: -FRAC_PI_2,
            pitch: -0.18,
            fov_deg: 62.0,
            near: 0.3,
            far: 420.0,
        });
        world.ensure_bvh();
        world
    }

    /// Cycles the active 3D camera perspective (`Chase` -> `Hood` -> `Orbit` -> `Aerial`) safely without clipping.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable world.
    ///
    /// # Returns
    ///
    /// - `u8` - New active camera mode index.
    pub fn cycle_camera_mode(&mut self) -> u8 {
        let next: u8 = (self.get_camera_mode() + 1) % 4;
        self.set_camera_mode(next);
        next
    }

    /// Tests whether a candidate horizontal AABB intersects any solid obstacle object excluding `ignore_idx`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - World reference.
    /// - `Aabb3D` - Candidate moving object bounds.
    /// - `usize` - Self object index to skip.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when blocked by a solid building or imported model.
    #[must_use]
    fn collides_with_obstacles(&self, candidate: Aabb3D, ignore_idx: usize) -> bool {
        for (idx, obj) in self.get_objects().iter().enumerate() {
            if idx == ignore_idx || !obj.get_ray_trace_caster() {
                continue;
            }
            let b: Aabb3D = obj.get_world_bounds();
            let overlap_x: bool = candidate.get_min().get_x() < b.get_max().get_x()
                && candidate.get_max().get_x() > b.get_min().get_x();
            let overlap_y: bool = candidate.get_min().get_y() < b.get_max().get_y()
                && candidate.get_max().get_y() > b.get_min().get_y();
            let overlap_z: bool = candidate.get_min().get_z() < b.get_max().get_z()
                && candidate.get_max().get_z() > b.get_min().get_z();
            if overlap_x && overlap_y && overlap_z {
                return true;
            }
        }
        false
    }

    /// Advances drivable vehicle/camera controls, AABB collision resolution, and spinning models by `dt` seconds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable world.
    /// - `f32` - Delta time in seconds.
    /// - `&InputState` - Real-time polled keyboard/pointer input.
    /// - `&ActionMap` - Action bindings for WASD/Arrows movement.
    pub fn step_simulation(&mut self, dt: f32, input: &InputState, actions: &ActionMap) {
        let move_vec: Point = actions.movement_vector(input);
        let steer: f32 = move_vec.get_x();
        let throttle: f32 = -move_vec.get_y();
        let mut any_moved: bool = false;
        let car_opt: Option<usize> = self.vehicle_index();
        if let Some(car_idx) = car_opt
            && let Some((mut tr, mesh_id)) = self
                .get_objects()
                .get(car_idx)
                .map(|car: &SceneObject3D| (car.get_transform(), car.get_mesh_id()))
        {
            let mesh_bounds: Aabb3D = self
                .get_mesh_library()
                .get(mesh_id)
                .and_then(|c: &MeshLodChain| c.get_levels().first())
                .map(Mesh3D::get_bounds)
                .unwrap_or_else(|| Aabb3D::new(Vec3::splat(-2.0), Vec3::splat(2.0)));
            if steer.abs() > 0.05 {
                let mut rotated: Transform3D = tr;
                rotated.set_yaw_rad(tr.get_yaw_rad() - steer * 1.85 * dt);
                if !self.collides_with_obstacles(rotated.transform_aabb(mesh_bounds), car_idx) {
                    tr = rotated;
                    any_moved = true;
                }
            }
            if throttle.abs() > 0.05 {
                let fwd_x: f32 = -tr.get_yaw_rad().sin();
                let fwd_z: f32 = -tr.get_yaw_rad().cos();
                let speed: f32 = 24.0 * throttle * dt;
                let cur: Vec3 = tr.get_position();
                let cand_x: f32 = (cur.get_x() + fwd_x * speed).clamp(-9.2, 9.2);
                let cand_z: f32 = (cur.get_z() + fwd_z * speed).clamp(-85.0, 85.0);
                let mut try_x: Transform3D = tr;
                try_x.set_position(Vec3::new(cand_x, 0.0, cur.get_z()));
                if !self.collides_with_obstacles(try_x.transform_aabb(mesh_bounds), car_idx) {
                    tr.set_position(Vec3::new(cand_x, 0.0, tr.get_position().get_z()));
                    any_moved = true;
                }
                let mut try_z: Transform3D = tr;
                try_z.set_position(Vec3::new(tr.get_position().get_x(), 0.0, cand_z));
                if !self.collides_with_obstacles(try_z.transform_aabb(mesh_bounds), car_idx) {
                    tr.set_position(Vec3::new(tr.get_position().get_x(), 0.0, cand_z));
                    any_moved = true;
                }
            }
            if any_moved {
                let world_b: Aabb3D = tr.transform_aabb(mesh_bounds);
                if let Some(car_mut) = self.get_mut_objects().get_mut(car_idx) {
                    car_mut.set_transform(tr);
                    car_mut.set_world_bounds(world_b);
                }
            }
            let car_tr: Transform3D = self.get_objects()[car_idx].get_transform();
            let car_pos: Vec3 = car_tr.get_position();
            let yaw: f32 = car_tr.get_yaw_rad();
            let cam_mode: u8 = self.get_camera_mode();
            let (mut desired_eye, look_target): (Vec3, Vec3) = match cam_mode {
                1 => {
                    let eye: Vec3 = Vec3::new(
                        car_pos.get_x() - yaw.sin() * 1.4,
                        2.15,
                        car_pos.get_z() - yaw.cos() * 1.4,
                    );
                    let tgt: Vec3 = Vec3::new(
                        car_pos.get_x() - yaw.sin() * 18.0,
                        1.8,
                        car_pos.get_z() - yaw.cos() * 18.0,
                    );
                    (eye, tgt)
                }
                2 => {
                    let orbit: f32 = self.get_camera_orbit_rad() + dt * 0.45;
                    self.set_camera_orbit_rad(orbit);
                    let eye: Vec3 = Vec3::new(
                        car_pos.get_x() + orbit.sin() * 10.5,
                        5.2,
                        car_pos.get_z() + orbit.cos() * 10.5,
                    );
                    (eye, Vec3::new(car_pos.get_x(), 1.6, car_pos.get_z()))
                }
                3 => {
                    let eye: Vec3 = Vec3::new(
                        car_pos.get_x() + yaw.sin() * 18.0,
                        18.5,
                        car_pos.get_z() + yaw.cos() * 18.0,
                    );
                    (eye, Vec3::new(car_pos.get_x(), 1.2, car_pos.get_z()))
                }
                4 => {
                    let cur_cam: Camera3D = self.get_camera();
                    let fwd: Vec3 = cur_cam.forward();
                    let right: Vec3 = cur_cam.right();
                    let moved: Vec3 = cur_cam
                        .get_position()
                        .add(fwd.scale(throttle * 18.0 * dt))
                        .add(right.scale(steer * 14.0 * dt));
                    let eye: Vec3 = Vec3::new(moved.get_x(), moved.get_y().max(1.5), moved.get_z());
                    (eye, eye.add(fwd.scale(16.0)))
                }
                5 => {
                    let orbit: f32 = self.get_camera_orbit_rad() + dt * 0.28;
                    self.set_camera_orbit_rad(orbit);
                    let eye: Vec3 = Vec3::new(
                        car_pos.get_x() + orbit.sin() * 7.8,
                        3.6 + (orbit * 1.5).sin() * 1.4,
                        car_pos.get_z() + orbit.cos() * 13.5,
                    );
                    (eye, Vec3::new(car_pos.get_x(), 1.8, car_pos.get_z() - 3.0))
                }
                _ => {
                    let eye: Vec3 = Vec3::new(
                        car_pos.get_x() + yaw.sin() * 12.5,
                        6.0,
                        car_pos.get_z() + yaw.cos() * 12.5,
                    );
                    let tgt: Vec3 =
                        Vec3::new(car_pos.get_x(), 2.2, car_pos.get_z() - yaw.cos() * 4.0);
                    (eye, tgt)
                }
            };
            desired_eye.set_x(desired_eye.get_x().clamp(-9.5, 9.5));
            let cam_probe: Aabb3D = Aabb3D::new(
                desired_eye.sub(Vec3::splat(0.6)),
                desired_eye.add(Vec3::splat(0.6)),
            );
            if self.collides_with_obstacles(cam_probe, car_idx) {
                desired_eye = Vec3::new(
                    car_pos.get_x(),
                    desired_eye.get_y().max(8.5),
                    car_pos.get_z() + 8.0,
                );
            }
            let mut cam: Camera3D = self.get_camera();
            let smooth: f32 = (dt * 7.0).clamp(0.05, 1.0);
            cam.set_position(cam.get_position().lerp(desired_eye, smooth));
            let look_dir: Vec3 = look_target.sub(cam.get_position()).normalize();
            cam.set_yaw(look_dir.get_z().atan2(look_dir.get_x()));
            cam.set_pitch(look_dir.get_y().clamp(-0.92, 0.92).asin());
            self.set_camera(cam);
        }
        for obj in self.get_mut_objects().iter_mut() {
            let spin: f32 = obj.get_spin_speed();
            if spin.abs() > 0.001 {
                let mut tr: Transform3D = obj.get_transform();
                tr.set_yaw_rad(tr.get_yaw_rad() + spin * dt);
                obj.set_transform(tr);
                any_moved = true;
            }
        }
        if any_moved {
            self.set_bvh_dirty(true);
        }
    }
}
