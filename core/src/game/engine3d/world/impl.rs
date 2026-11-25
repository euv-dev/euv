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
            scale: Vec3::splat(1.0),
        }
    }
}

/// Matrix and point transformation helpers for `Transform3D`.
impl Transform3D {
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
        Mat4::from_trs(self.get_position(), self.get_yaw_rad(), self.get_scale())
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
        let sx: f32 = p.get_x() * sc.get_x();
        let sy: f32 = p.get_y() * sc.get_y();
        let sz: f32 = p.get_z() * sc.get_z();
        let c: f32 = self.get_yaw_rad().cos();
        let s: f32 = self.get_yaw_rad().sin();
        let rx: f32 = sx * c + sz * s;
        let rz: f32 = -sx * s + sz * c;
        Vec3::new(rx, sy, rz).add(self.get_position())
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
        let c: f32 = self.get_yaw_rad().cos();
        let s: f32 = self.get_yaw_rad().sin();
        Vec3::new(
            n.get_x() * c + n.get_z() * s,
            n.get_y(),
            -n.get_x() * s + n.get_z() * c,
        )
        .normalize()
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
        let asphalt_mat: PbrMaterial = PbrMaterial::pbr(Color::hex(0x18202C), 0.25, 0.14, 0.58);
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
            PbrMaterial::pbr(Color::hex(0x1E293B), 0.72, 0.16, 0.65),
            PbrMaterial::pbr(Color::hex(0x334155), 0.55, 0.24, 0.45),
            PbrMaterial::pbr(Color::hex(0x0F172A), 0.85, 0.10, 0.78),
            PbrMaterial::pbr(Color::hex(0x475569), 0.40, 0.35, 0.35),
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
