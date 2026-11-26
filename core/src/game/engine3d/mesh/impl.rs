use super::*;

/// Default neutral matte PBR material.
impl Default for PbrMaterial {
    /// Returns a mid-gray dielectric PBR material.
    ///
    /// # Returns
    ///
    /// - `Self` - Default `PbrMaterial`.
    fn default() -> Self {
        Self {
            base_color: Vec3::new(0.72, 0.75, 0.80),
            metallic: 0.1,
            roughness: 0.55,
            emissive: Vec3::zero(),
            reflectivity: 0.12,
            surface_detail: SurfaceDetailMode::None,
        }
    }
}

/// Material presets and fluent builders for metallic conductors, dielectrics, architectural glass, and neon signage.
impl PbrMaterial {
    /// Creates a PBR material from an sRGB `Color`, `metallic`, `roughness`, and `reflectivity`.
    ///
    /// # Arguments
    ///
    /// - `Color` - Base albedo color.
    /// - `f32` - Metallic scalar `[0.0, 1.0]`.
    /// - `f32` - Roughness scalar `[0.04, 1.0]`.
    /// - `f32` - Ray-traced reflectivity weight `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured PBR material.
    #[must_use]
    pub fn pbr(color: Color, metallic: f32, roughness: f32, reflectivity: f32) -> Self {
        Self {
            base_color: Vec3::from_color(color),
            metallic: metallic.clamp(0.0, 1.0),
            roughness: roughness.clamp(0.04, 1.0),
            emissive: Vec3::zero(),
            reflectivity: reflectivity.clamp(0.0, 1.0),
            surface_detail: SurfaceDetailMode::None,
        }
    }

    /// Creates a metallic conductor PBR material (chrome, painted car body, brushed steel).
    ///
    /// # Arguments
    ///
    /// - `Color` - Base metallic reflectance color.
    /// - `f32` - Surface roughness in `[0.04, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Conductor PBR material.
    #[must_use]
    pub fn metallic(color: Color, roughness: f32) -> Self {
        let r: f32 = roughness.clamp(0.04, 1.0);
        Self::pbr(color, 0.92, r, (1.0 - r * 0.8).clamp(0.15, 0.96))
    }

    /// Creates a non-metallic dielectric PBR material (asphalt, concrete, matte plastic, cloth).
    ///
    /// # Arguments
    ///
    /// - `Color` - Base diffuse albedo color.
    /// - `f32` - Surface roughness in `[0.04, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Dielectric PBR material.
    #[must_use]
    pub fn dielectric(color: Color, roughness: f32) -> Self {
        let r: f32 = roughness.clamp(0.04, 1.0);
        Self::pbr(color, 0.04, r, (0.35 * (1.0 - r)).clamp(0.04, 0.35))
    }

    /// Creates a highly reflective architectural glass PBR material.
    ///
    /// # Arguments
    ///
    /// - `Color` - Subtle glass tint color.
    /// - `f32` - Surface roughness in `[0.04, 0.4]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Reflective glass PBR material.
    #[must_use]
    pub fn glass(color: Color, roughness: f32) -> Self {
        let r: f32 = roughness.clamp(0.04, 0.4);
        Self::pbr(color, 0.12, r, 0.88)
    }

    /// Creates an emissive neon PBR material for city signage, vehicle tail-lights, and beacons.
    ///
    /// # Arguments
    ///
    /// - `Color` - Base and emissive neon color.
    /// - `f32` - Emissive intensity multiplier.
    ///
    /// # Returns
    ///
    /// - `Self` - Emissive PBR material.
    #[must_use]
    pub fn neon(color: Color, intensity: f32) -> Self {
        let rgb: Vec3 = Vec3::from_color(color);
        Self {
            base_color: rgb,
            metallic: 0.2,
            roughness: 0.2,
            emissive: rgb.scale(intensity.max(0.2)),
            reflectivity: 0.3,
            surface_detail: SurfaceDetailMode::None,
        }
    }

    /// Returns a copy of this material with custom linear HDR emissive radiance.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base material.
    /// - `Vec3` - Emissive HDR RGB radiance.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated material.
    #[must_use]
    pub fn with_emissive(mut self, emissive: Vec3) -> Self {
        self.set_emissive(emissive);
        self
    }

    /// Returns a copy of this material with adjusted surface roughness.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base material.
    /// - `f32` - Surface roughness clamped to `[0.04, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated material.
    #[must_use]
    pub fn with_roughness(mut self, roughness: f32) -> Self {
        self.set_roughness(roughness.clamp(0.04, 1.0));
        self
    }

    /// Returns a copy of this material with adjusted metallic factor.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base material.
    /// - `f32` - Metallic factor clamped to `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated material.
    #[must_use]
    pub fn with_metallic(mut self, metallic: f32) -> Self {
        self.set_metallic(metallic.clamp(0.0, 1.0));
        self
    }

    /// Returns a copy of this material with adjusted ray-traced reflectivity.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base material.
    /// - `f32` - Reflectivity weight clamped to `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated material.
    #[must_use]
    pub fn with_reflectivity(mut self, reflectivity: f32) -> Self {
        self.set_reflectivity(reflectivity.clamp(0.0, 1.0));
        self
    }

    /// Returns a copy of this material with the requested procedural surface detail overlay.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base material.
    /// - `SurfaceDetailMode` - Procedural surface detail mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated material.
    #[must_use]
    pub fn with_surface_detail(mut self, mode: SurfaceDetailMode) -> Self {
        self.set_surface_detail(mode);
        self
    }
}

/// Fluent constructors for `BoxMeshDesc`.
impl BoxMeshDesc {
    /// Creates a box mesh descriptor with default ground pivot and neutral PBR material.
    ///
    /// # Arguments
    ///
    /// - `&str` - Asset name.
    /// - `Vec3` - Box extents `(width, height, depth)`.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready descriptor.
    #[must_use]
    pub fn new(name: &str, size: Vec3) -> Self {
        Self {
            name: name.to_string(),
            size,
            ground_pivot: true,
            material: PbrMaterial::default(),
        }
    }

    /// Sets whether the box rests on `y = 0` or is centered at the origin.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base descriptor.
    /// - `bool` - Ground pivot toggle.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated descriptor.
    #[must_use]
    pub fn with_ground_pivot(mut self, ground_pivot: bool) -> Self {
        self.set_ground_pivot(ground_pivot);
        self
    }

    /// Assigns the PBR material for the generated box mesh.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base descriptor.
    /// - `PbrMaterial` - Surface material.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated descriptor.
    #[must_use]
    pub fn with_material(mut self, material: PbrMaterial) -> Self {
        self.set_material(material);
        self
    }
}

/// Fluent constructors for `GroundPlaneDesc`.
impl GroundPlaneDesc {
    /// Creates a ground plane descriptor with default subdivisions and neutral PBR material.
    ///
    /// # Arguments
    ///
    /// - `&str` - Asset name.
    /// - `f32` - Width along X.
    /// - `f32` - Depth along Z.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready descriptor.
    #[must_use]
    pub fn new(name: &str, width: f32, depth: f32) -> Self {
        Self {
            name: name.to_string(),
            width,
            depth,
            subdivisions: 8,
            material: PbrMaterial::default(),
        }
    }

    /// Sets the grid subdivision count along each axis.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base descriptor.
    /// - `usize` - Subdivision count.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated descriptor.
    #[must_use]
    pub fn with_subdivisions(mut self, subdivisions: usize) -> Self {
        self.set_subdivisions(subdivisions);
        self
    }

    /// Assigns the PBR material for the generated ground plane.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base descriptor.
    /// - `PbrMaterial` - Surface material.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated descriptor.
    #[must_use]
    pub fn with_material(mut self, material: PbrMaterial) -> Self {
        self.set_material(material);
        self
    }
}

/// Mesh construction, Nanite-style meshlet clustering, decimation, and procedural asset generators for `Mesh3D`.
impl Mesh3D {
    /// Partitions indexed triangles into Nanite-style `Meshlet3D` clusters with tight AABBs and normal cones.
    ///
    /// # Arguments
    ///
    /// - `&[Vertex3D]` - Source vertex buffer.
    /// - `&[[u32; 3]]` - Source triangle indices.
    ///
    /// # Returns
    ///
    /// - `Vec<Meshlet3D>` - Contiguous cluster descriptors covering all triangles.
    #[must_use]
    pub fn build_meshlets_from_parts(
        vertices: &[Vertex3D],
        indices: &[[u32; 3]],
    ) -> Vec<Meshlet3D> {
        if indices.is_empty() {
            return Vec::new();
        }
        let cluster_count: usize = indices.len().div_ceil(MESHLET_MAX_TRIS);
        let mut out: Vec<Meshlet3D> = Vec::with_capacity(cluster_count);
        let mut start: usize = 0;
        while start < indices.len() {
            let end: usize = (start + MESHLET_MAX_TRIS).min(indices.len());
            let mut cluster_box: Aabb3D = Aabb3D::empty();
            let mut avg_n: Vec3 = Vec3::zero();
            let mut valid_tris: usize = 0;
            for tri in &indices[start..end] {
                let (i0, i1, i2): (usize, usize, usize) =
                    (tri[0] as usize, tri[1] as usize, tri[2] as usize);
                if i0 < vertices.len() && i1 < vertices.len() && i2 < vertices.len() {
                    let p0: Vec3 = vertices[i0].get_position();
                    let p1: Vec3 = vertices[i1].get_position();
                    let p2: Vec3 = vertices[i2].get_position();
                    cluster_box.expand_point(p0);
                    cluster_box.expand_point(p1);
                    cluster_box.expand_point(p2);
                    let fn_vec: Vec3 = p1.sub(p0).cross(p2.sub(p0)).normalize();
                    avg_n = avg_n.add(fn_vec);
                    valid_tris += 1;
                }
            }
            if valid_tris == 0 {
                cluster_box = Aabb3D::new(Vec3::zero(), Vec3::zero());
            }
            let cone_axis: Vec3 = avg_n.normalize();
            let mut min_dot: f32 = 1.0;
            for tri in &indices[start..end] {
                let (i0, i1, i2): (usize, usize, usize) =
                    (tri[0] as usize, tri[1] as usize, tri[2] as usize);
                if i0 < vertices.len() && i1 < vertices.len() && i2 < vertices.len() {
                    let p0: Vec3 = vertices[i0].get_position();
                    let p1: Vec3 = vertices[i1].get_position();
                    let p2: Vec3 = vertices[i2].get_position();
                    let fn_vec: Vec3 = p1.sub(p0).cross(p2.sub(p0)).normalize();
                    min_dot = min_dot.min(fn_vec.dot(cone_axis));
                }
            }
            let radius: f32 = cluster_box.radius().max(0.05);
            let screen_error: f32 = radius / ((end - start).max(1) as f32).sqrt();
            out.push(Meshlet3D::new(
                start as u32,
                (end - start) as u32,
                cluster_box,
                cone_axis,
                min_dot,
                screen_error,
            ));
            start = end;
        }
        out
    }

    /// Constructs a `Mesh3D`, computes its local `Aabb3D`, and partitions Nanite-style `Meshlet3D` clusters.
    ///
    /// # Arguments
    ///
    /// - `String` - Asset name.
    /// - `Vec<Vertex3D>` - Vertex list.
    /// - `Vec<[u32; 3]>` - Triangle indices.
    /// - `PbrMaterial` - Default material.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready indexed mesh with meshlets.
    #[must_use]
    pub fn from_parts(
        name: String,
        vertices: Vec<Vertex3D>,
        indices: Vec<[u32; 3]>,
        material: PbrMaterial,
    ) -> Self {
        let mut bounds: Aabb3D = Aabb3D::empty();
        for v in vertices.iter() {
            bounds.expand_point(v.get_position());
        }
        if vertices.is_empty() {
            bounds = Aabb3D::new(Vec3::zero(), Vec3::zero());
        }
        let meshlets: Vec<Meshlet3D> = Self::build_meshlets_from_parts(&vertices, &indices);
        Self {
            name,
            vertices,
            indices,
            bounds,
            material,
            meshlets,
        }
    }

    /// Generates an axis-aligned box mesh of dimensions `(w, h, d)` resting on `y = 0` or centered.
    ///
    /// # Arguments
    ///
    /// - `&str` - Mesh name.
    /// - `f32` - Width along X.
    /// - `f32` - Height along Y.
    /// - `f32` - Depth along Z.
    /// - `bool` - When `true`, base rests on `y = 0`; otherwise centered at origin.
    /// - `PbrMaterial` - Surface PBR material.
    ///
    /// # Returns
    ///
    /// - `Self` - Indexed 12-triangle box mesh with crisp flat face normals.
    #[must_use]
    pub fn box_mesh(
        name: &str,
        w: f32,
        h: f32,
        d: f32,
        ground_pivot: bool,
        material: PbrMaterial,
    ) -> Self {
        let hx: f32 = w * 0.5;
        let hz: f32 = d * 0.5;
        let y0: f32 = if ground_pivot { 0.0 } else { -h * 0.5 };
        let y1: f32 = if ground_pivot { h } else { h * 0.5 };
        let faces: [([Vec3; 4], Vec3); 6] = [
            (
                [
                    Vec3::new(-hx, y0, hz),
                    Vec3::new(hx, y0, hz),
                    Vec3::new(hx, y1, hz),
                    Vec3::new(-hx, y1, hz),
                ],
                Vec3::new(0.0, 0.0, 1.0),
            ),
            (
                [
                    Vec3::new(hx, y0, -hz),
                    Vec3::new(-hx, y0, -hz),
                    Vec3::new(-hx, y1, -hz),
                    Vec3::new(hx, y1, -hz),
                ],
                Vec3::new(0.0, 0.0, -1.0),
            ),
            (
                [
                    Vec3::new(hx, y0, hz),
                    Vec3::new(hx, y0, -hz),
                    Vec3::new(hx, y1, -hz),
                    Vec3::new(hx, y1, hz),
                ],
                Vec3::new(1.0, 0.0, 0.0),
            ),
            (
                [
                    Vec3::new(-hx, y0, -hz),
                    Vec3::new(-hx, y0, hz),
                    Vec3::new(-hx, y1, hz),
                    Vec3::new(-hx, y1, -hz),
                ],
                Vec3::new(-1.0, 0.0, 0.0),
            ),
            (
                [
                    Vec3::new(-hx, y1, hz),
                    Vec3::new(hx, y1, hz),
                    Vec3::new(hx, y1, -hz),
                    Vec3::new(-hx, y1, -hz),
                ],
                Vec3::new(0.0, 1.0, 0.0),
            ),
            (
                [
                    Vec3::new(-hx, y0, -hz),
                    Vec3::new(hx, y0, -hz),
                    Vec3::new(hx, y0, hz),
                    Vec3::new(-hx, y0, hz),
                ],
                Vec3::new(0.0, -1.0, 0.0),
            ),
        ];
        let mut vertices: Vec<Vertex3D> = Vec::with_capacity(24);
        let mut indices: Vec<[u32; 3]> = Vec::with_capacity(12);
        for (quad, n) in faces.into_iter() {
            let base: u32 = vertices.len() as u32;
            for p in quad.into_iter() {
                vertices.push(Vertex3D::new(p, n));
            }
            indices.push([base, base + 1, base + 2]);
            indices.push([base, base + 2, base + 3]);
        }
        Self::from_parts(name.to_string(), vertices, indices, material)
    }

    /// Generates a horizontal ground/road quad plane at `y = 0` subdivided for crisp specular/shadow falloff.
    ///
    /// # Arguments
    ///
    /// - `&str` - Mesh name.
    /// - `f32` - Width along X.
    /// - `f32` - Depth along Z.
    /// - `usize` - Grid subdivisions per axis.
    /// - `PbrMaterial` - Surface PBR material.
    ///
    /// # Returns
    ///
    /// - `Self` - Subdivided ground plane mesh.
    #[must_use]
    pub fn ground_plane(
        name: &str,
        w: f32,
        d: f32,
        subdivisions: usize,
        material: PbrMaterial,
    ) -> Self {
        let segs: usize = subdivisions.clamp(1, 16);
        let mut vertices: Vec<Vertex3D> = Vec::with_capacity((segs + 1) * (segs + 1));
        let mut indices: Vec<[u32; 3]> = Vec::with_capacity(segs * segs * 2);
        let up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        for iz in 0..=segs {
            let vz: f32 = ((iz as f32) / (segs as f32) - 0.5) * d;
            for ix in 0..=segs {
                let vx: f32 = ((ix as f32) / (segs as f32) - 0.5) * w;
                vertices.push(Vertex3D::new(Vec3::new(vx, 0.0, vz), up));
            }
        }
        let stride: u32 = (segs + 1) as u32;
        for iz in 0..segs {
            for ix in 0..segs {
                let i0: u32 = (iz as u32) * stride + (ix as u32);
                let i1: u32 = i0 + 1;
                let i2: u32 = i0 + stride;
                let i3: u32 = i2 + 1;
                indices.push([i0, i2, i1]);
                indices.push([i1, i2, i3]);
            }
        }
        Self::from_parts(name.to_string(), vertices, indices, material)
    }

    /// Generates a low-poly sculpted sports car chassis resting on `y = 0`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Asset name.
    /// - `PbrMaterial` - Car body paint PBR material.
    ///
    /// # Returns
    ///
    /// - `Self` - Composite vehicle mesh.
    #[must_use]
    pub fn sports_car(name: &str, material: PbrMaterial) -> Self {
        let lower: Self = Self::box_mesh(MESH_CAR_LOWER, 2.2, 0.80, 4.6, true, material);
        let cabin: Self = Self::box_mesh(MESH_CAR_CABIN, 1.82, 0.72, 2.2, true, material);
        let wheel: Self = Self::box_mesh(MESH_CAR_WHEEL, 0.34, 0.56, 0.62, true, material);
        let mut vertices: Vec<Vertex3D> = Vec::with_capacity(144);
        let mut indices: Vec<[u32; 3]> = Vec::with_capacity(72);
        let lower_offset: u32 = vertices.len() as u32;
        for v in lower.get_vertices().iter() {
            let p: Vec3 = v.get_position();
            vertices.push(Vertex3D::new(
                Vec3::new(p.get_x(), p.get_y() + 0.24, p.get_z()),
                v.get_normal(),
            ));
        }
        for tri in lower.get_indices().iter() {
            indices.push([
                tri[0] + lower_offset,
                tri[1] + lower_offset,
                tri[2] + lower_offset,
            ]);
        }
        let cabin_offset: u32 = vertices.len() as u32;
        for v in cabin.get_vertices().iter() {
            let p: Vec3 = v.get_position();
            vertices.push(Vertex3D::new(
                Vec3::new(p.get_x(), p.get_y() + 1.04, p.get_z() - 0.15),
                v.get_normal(),
            ));
        }
        for tri in cabin.get_indices().iter() {
            indices.push([
                tri[0] + cabin_offset,
                tri[1] + cabin_offset,
                tri[2] + cabin_offset,
            ]);
        }
        let wheel_offsets: [(f32, f32); 4] =
            [(-1.12, -1.45), (1.12, -1.45), (-1.12, 1.45), (1.12, 1.45)];
        for (wx, wz) in wheel_offsets.into_iter() {
            let base: u32 = vertices.len() as u32;
            for v in wheel.get_vertices().iter() {
                let p: Vec3 = v.get_position();
                vertices.push(Vertex3D::new(
                    Vec3::new(p.get_x() + wx, p.get_y() + 0.02, p.get_z() + wz),
                    v.get_normal(),
                ));
            }
            for tri in wheel.get_indices().iter() {
                indices.push([tri[0] + base, tri[1] + base, tri[2] + base]);
            }
        }
        Self::from_parts(name.to_string(), vertices, indices, material)
    }

    /// Builds a box mesh from a fluent `BoxMeshDesc`.
    ///
    /// # Arguments
    ///
    /// - `&BoxMeshDesc` - Box mesh descriptor.
    ///
    /// # Returns
    ///
    /// - `Self` - Indexed 12-triangle box mesh.
    #[must_use]
    pub fn from_box_desc(desc: &BoxMeshDesc) -> Self {
        let size: Vec3 = desc.get_size();
        Self::box_mesh(
            desc.get_name(),
            size.get_x(),
            size.get_y(),
            size.get_z(),
            desc.get_ground_pivot(),
            desc.get_material(),
        )
    }

    /// Builds a subdivided ground plane from a fluent `GroundPlaneDesc`.
    ///
    /// # Arguments
    ///
    /// - `&GroundPlaneDesc` - Ground plane descriptor.
    ///
    /// # Returns
    ///
    /// - `Self` - Subdivided ground plane mesh.
    #[must_use]
    pub fn from_ground_desc(desc: &GroundPlaneDesc) -> Self {
        Self::ground_plane(
            desc.get_name(),
            desc.get_width(),
            desc.get_depth(),
            desc.get_subdivisions(),
            desc.get_material(),
        )
    }

    /// Builds a simplified proxy mesh enclosing this mesh's AABB for distant LOD tiers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source mesh reference.
    /// - `&str` - LOD suffix label.
    ///
    /// # Returns
    ///
    /// - `Self` - Lightweight 12-triangle LOD proxy mesh.
    #[must_use]
    pub fn build_lod_proxy(&self, suffix: &str) -> Self {
        let b: Aabb3D = self.get_bounds();
        let size: Vec3 = b.get_max().sub(b.get_min());
        let center: Vec3 = b.center();
        let mut proxy: Self = Self::box_mesh(
            &format!("{}_{suffix}", self.get_name()),
            size.get_x().max(0.2),
            size.get_y().max(0.2),
            size.get_z().max(0.2),
            false,
            self.get_material(),
        );
        for v in proxy.get_mut_vertices().iter_mut() {
            v.set_position(v.get_position().add(center));
        }
        proxy.set_bounds(b);
        let meshlets: Vec<Meshlet3D> =
            Self::build_meshlets_from_parts(proxy.get_vertices(), proxy.get_indices());
        proxy.set_meshlets(meshlets);
        proxy
    }

    /// Produces a mid-detail decimated mesh by retaining dominant triangles while preserving bounding silhouette.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Source mesh reference.
    /// - `&str` - LOD suffix label.
    ///
    /// # Returns
    ///
    /// - `Self` - Decimated intermediate LOD mesh.
    #[must_use]
    pub fn build_mid_lod(&self, suffix: &str) -> Self {
        let verts: &[Vertex3D] = self.get_vertices().as_slice();
        let tris: &[[u32; 3]] = self.get_indices().as_slice();
        if tris.len() <= 24 {
            return self.build_lod_proxy(suffix);
        }
        let mut scored: Vec<(f32, [u32; 3])> = Vec::with_capacity(tris.len());
        for tri in tris.iter().copied() {
            let (i0, i1, i2): (usize, usize, usize) =
                (tri[0] as usize, tri[1] as usize, tri[2] as usize);
            if i0 < verts.len() && i1 < verts.len() && i2 < verts.len() {
                let p0: Vec3 = verts[i0].get_position();
                let p1: Vec3 = verts[i1].get_position();
                let p2: Vec3 = verts[i2].get_position();
                let area2: f32 = p1.sub(p0).cross(p2.sub(p0)).length_sq();
                scored.push((area2, tri));
            }
        }
        scored.sort_unstable_by(|a: &(f32, [u32; 3]), b: &(f32, [u32; 3])| b.0.total_cmp(&a.0));
        let keep: usize = (scored.len() / 2).max(12);
        let reduced: Vec<[u32; 3]> = scored
            .into_iter()
            .take(keep)
            .map(|(_, t): (f32, [u32; 3])| t)
            .collect();
        Self::from_parts(
            format!("{}_{suffix}", self.get_name()),
            self.get_vertices().clone(),
            reduced,
            self.get_material(),
        )
    }
}

/// Multi-level continuous LOD chain creation and screen-error selection for `MeshLodChain`.
impl MeshLodChain {
    /// Builds a continuous 3-tier Nanite-style LOD chain (`LOD0` full -> `LOD1` mid -> `LOD2` proxy) automatically.
    ///
    /// # Arguments
    ///
    /// - `Mesh3D` - Full-resolution `LOD0` mesh.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready LOD chain.
    #[must_use]
    pub fn from_mesh(mesh: Mesh3D) -> Self {
        let tri_count: usize = mesh.get_indices().len();
        if tri_count > 32 {
            let lod1: Mesh3D = mesh.build_mid_lod(MESH_LOD1_SUFFIX);
            let lod2: Mesh3D = mesh.build_lod_proxy(MESH_LOD2_SUFFIX);
            Self {
                levels: vec![mesh, lod1, lod2],
            }
        } else if tri_count > 16 {
            let lod1: Mesh3D = mesh.build_lod_proxy(MESH_LOD1_SUFFIX);
            Self {
                levels: vec![mesh, lod1],
            }
        } else {
            let clone_lod1: Mesh3D = mesh.clone();
            Self {
                levels: vec![mesh, clone_lod1],
            }
        }
    }

    /// Selects the appropriate `Mesh3D` detail level for `distance` and `lod_bias`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - LOD chain reference.
    /// - `f32` - Camera-to-object distance in meters.
    /// - `f32` - Quality LOD bias multiplier.
    ///
    /// # Returns
    ///
    /// - `(&Mesh3D, usize)` - Selected mesh reference and its LOD tier index.
    #[must_use]
    pub fn select(&self, distance: f32, lod_bias: f32) -> (&Mesh3D, usize) {
        let levels: &[Mesh3D] = self.get_levels().as_slice();
        let effective_dist: f32 = distance * lod_bias.max(0.25);
        if levels.len() > 2 && effective_dist > 135.0 {
            (&levels[2], 2)
        } else if levels.len() > 1 && effective_dist > 78.0 {
            (&levels[1], 1)
        } else {
            (&levels[0], 0)
        }
    }

    /// Nanite-style continuous LOD selection driven by projected screen-space pixel coverage.
    ///
    /// # Arguments
    ///
    /// - `&Self` - LOD chain reference.
    /// - `f32` - Camera-to-object distance in meters.
    /// - `f32` - World-space bounding sphere radius in meters.
    /// - `f32` - Projected viewport focal length in pixels (`height / (2 * tan(fov/2))`).
    /// - `f32` - Quality LOD bias multiplier (`< 1.0` higher detail, `> 1.0` aggressive simplification).
    ///
    /// # Returns
    ///
    /// - `(&Mesh3D, usize)` - Selected mesh reference and its LOD tier index.
    #[must_use]
    pub fn select_screen_error(
        &self,
        distance: f32,
        bounds_radius: f32,
        focal_px: f32,
        lod_bias: f32,
    ) -> (&Mesh3D, usize) {
        let levels: &[Mesh3D] = self.get_levels().as_slice();
        let safe_dist: f32 = distance.max(0.5);
        let projected_px: f32 =
            (bounds_radius.max(0.2) * focal_px) / (safe_dist * lod_bias.max(0.25));
        if levels.len() > 2 && projected_px < 22.0 {
            (&levels[2], 2)
        } else if levels.len() > 1 && projected_px < 58.0 {
            (&levels[1], 1)
        } else {
            (&levels[0], 0)
        }
    }
}

/// Native Wavefront `.obj` and glTF 2.0 `.gltf` JSON parsers for `ModelImporter3D`.
impl ModelImporter3D {
    /// Imports a 3D model from text source in the specified `ModelFormat` and generates LOD chains.
    ///
    /// # Arguments
    ///
    /// - `ModelFormat` - Interchange format (`Obj` or `GltfJson`).
    /// - `&str` - Raw `.obj` or `.gltf` text content.
    /// - `PbrMaterial` - Fallback PBR material when source omits material definitions.
    ///
    /// # Returns
    ///
    /// - `Result<ImportedModel3D, String>` - Imported model bundle or parse error diagnostic.
    pub fn import(
        format: ModelFormat,
        source: &str,
        fallback_material: PbrMaterial,
    ) -> Result<ImportedModel3D, String> {
        match format {
            ModelFormat::Obj => Self::parse_obj(source, fallback_material),
            ModelFormat::GltfJson => Self::parse_gltf_json(source, fallback_material),
        }
    }

    /// Parses a Wavefront `.obj` mesh (`o`, `v`, `vn`, `f` with n-gon fan triangulation and auto-normals).
    ///
    /// # Arguments
    ///
    /// - `&str` - Raw `.obj` ASCII text.
    /// - `PbrMaterial` - Material assigned to the parsed mesh.
    ///
    /// # Returns
    ///
    /// - `Result<ImportedModel3D, String>` - Parsed model or error string.
    pub fn parse_obj(source: &str, material: PbrMaterial) -> Result<ImportedModel3D, String> {
        let mut positions: Vec<Vec3> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut vertices: Vec<Vertex3D> = Vec::new();
        let mut indices: Vec<[u32; 3]> = Vec::new();
        let mut model_name: String = MESH_OBJ_DEFAULT_NAME.to_string();
        for raw_line in source.lines() {
            let line: &str = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            match parts[0] {
                "o" | "g" if parts.len() >= 2 => {
                    model_name = parts[1].to_string();
                }
                "v" if parts.len() >= 4 => {
                    let x: f32 = parts[1].parse::<f32>().unwrap_or(0.0);
                    let y: f32 = parts[2].parse::<f32>().unwrap_or(0.0);
                    let z: f32 = parts[3].parse::<f32>().unwrap_or(0.0);
                    positions.push(Vec3::new(x, y, z));
                }
                "vn" if parts.len() >= 4 => {
                    let x: f32 = parts[1].parse::<f32>().unwrap_or(0.0);
                    let y: f32 = parts[2].parse::<f32>().unwrap_or(1.0);
                    let z: f32 = parts[3].parse::<f32>().unwrap_or(0.0);
                    normals.push(Vec3::new(x, y, z).normalize());
                }
                "f" if parts.len() >= 4 => {
                    let mut face_verts: Vec<(Vec3, Option<Vec3>)> =
                        Vec::with_capacity(parts.len() - 1);
                    for token in parts.iter().skip(1) {
                        let comps: Vec<&str> = token.split('/').collect();
                        let v_idx_raw: i32 = comps
                            .first()
                            .and_then(|s: &&str| s.parse::<i32>().ok())
                            .unwrap_or(0);
                        let p_opt: Option<Vec3> = if v_idx_raw > 0 {
                            positions.get((v_idx_raw - 1) as usize).copied()
                        } else if v_idx_raw < 0 {
                            let idx: isize = (positions.len() as isize) + (v_idx_raw as isize);
                            if idx >= 0 {
                                positions.get(idx as usize).copied()
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        let n_opt: Option<Vec3> = comps
                            .get(2)
                            .and_then(|s: &&str| s.parse::<i32>().ok())
                            .and_then(|n_raw: i32| {
                                if n_raw > 0 {
                                    normals.get((n_raw - 1) as usize).copied()
                                } else {
                                    None
                                }
                            });
                        if let Some(pos) = p_opt {
                            face_verts.push((pos, n_opt));
                        }
                    }
                    if face_verts.len() >= 3 {
                        for k in 1..(face_verts.len() - 1) {
                            let (p0, n0): (Vec3, Option<Vec3>) = face_verts[0];
                            let (p1, n1): (Vec3, Option<Vec3>) = face_verts[k];
                            let (p2, n2): (Vec3, Option<Vec3>) = face_verts[k + 1];
                            let face_n: Vec3 = p1.sub(p0).cross(p2.sub(p0)).normalize();
                            let base: u32 = vertices.len() as u32;
                            vertices.push(Vertex3D::new(p0, n0.unwrap_or(face_n)));
                            vertices.push(Vertex3D::new(p1, n1.unwrap_or(face_n)));
                            vertices.push(Vertex3D::new(p2, n2.unwrap_or(face_n)));
                            indices.push([base, base + 1, base + 2]);
                        }
                    }
                }
                _ => {}
            }
        }
        if indices.is_empty() {
            return Err(ERR_OBJ_EMPTY.to_string());
        }
        let total_vertices: usize = vertices.len();
        let total_triangles: usize = indices.len();
        let mesh: Mesh3D = Mesh3D::from_parts(model_name.clone(), vertices, indices, material);
        Ok(ImportedModel3D {
            name: model_name,
            format: ModelFormat::Obj,
            meshes: vec![MeshLodChain::from_mesh(mesh)],
            total_vertices,
            total_triangles,
        })
    }

    /// Parses a glTF 2.0 JSON asset supporting inline PBR materials and embedded mesh positions/indices.
    ///
    /// # Arguments
    ///
    /// - `&str` - Raw `.gltf` JSON string.
    /// - `PbrMaterial` - Fallback PBR material when JSON omits `pbrMetallicRoughness`.
    ///
    /// # Returns
    ///
    /// - `Result<ImportedModel3D, String>` - Parsed model bundle or validation error.
    pub fn parse_gltf_json(
        source: &str,
        fallback_material: PbrMaterial,
    ) -> Result<ImportedModel3D, String> {
        let root: Value = from_str::<Value>(source)
            .ok()
            .ok_or_else(|| ERR_GLTF_JSON.to_string())?;
        let name: String = root
            .get(GLTF_KEY_ASSET)
            .and_then(|a: &Value| a.get(GLTF_KEY_GENERATOR))
            .and_then(Value::as_str)
            .unwrap_or(MESH_GLTF_DEFAULT_NAME)
            .to_string();
        let mut mat: PbrMaterial = fallback_material;
        if let Some(pbr) = root
            .get(GLTF_KEY_MATERIALS)
            .and_then(Value::as_array)
            .and_then(|arr: &Vec<Value>| arr.first())
            .and_then(|m: &Value| m.get(GLTF_KEY_PBR))
        {
            if let Some(bc) = pbr.get(GLTF_KEY_BASE_COLOR).and_then(Value::as_array)
                && bc.len() >= 3
            {
                let r: f32 = bc[0].as_f64().unwrap_or(0.8) as f32;
                let g: f32 = bc[1].as_f64().unwrap_or(0.8) as f32;
                let b: f32 = bc[2].as_f64().unwrap_or(0.8) as f32;
                mat.set_base_color(Vec3::new(r, g, b));
            }
            if let Some(met) = pbr.get(GLTF_KEY_METALLIC).and_then(Value::as_f64) {
                mat.set_metallic((met as f32).clamp(0.0, 1.0));
            }
            if let Some(rough) = pbr.get(GLTF_KEY_ROUGHNESS).and_then(Value::as_f64) {
                mat.set_roughness((rough as f32).clamp(0.04, 1.0));
                mat.set_reflectivity((1.0 - (rough as f32)).clamp(0.05, 0.95));
            }
        }
        let mut chains: Vec<MeshLodChain> = Vec::new();
        let mut total_vertices: usize = 0;
        let mut total_triangles: usize = 0;
        if let Some(meshes_arr) = root.get(GLTF_KEY_MESHES).and_then(Value::as_array) {
            for (m_idx, m_val) in meshes_arr.iter().enumerate() {
                let mesh_name: String = m_val
                    .get(GLTF_KEY_NAME)
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("{name}_mesh_{m_idx}"));
                let Some(prims) = m_val.get(GLTF_KEY_PRIMITIVES).and_then(Value::as_array) else {
                    continue;
                };
                for prim in prims.iter() {
                    let extras: &Value = prim.get(GLTF_KEY_EXTRAS).unwrap_or(prim);
                    let pos_vals: Vec<f32> = extras
                        .get(GLTF_KEY_POSITIONS)
                        .and_then(Value::as_array)
                        .map(|a: &Vec<Value>| {
                            a.iter()
                                .filter_map(|v: &Value| v.as_f64().map(|n: f64| n as f32))
                                .collect()
                        })
                        .unwrap_or_default();
                    let idx_vals: Vec<u32> = extras
                        .get(GLTF_KEY_INDICES)
                        .and_then(Value::as_array)
                        .map(|a: &Vec<Value>| {
                            a.iter()
                                .filter_map(|v: &Value| v.as_u64().map(|n: u64| n as u32))
                                .collect()
                        })
                        .unwrap_or_default();
                    let mut raw_pts: Vec<Vec3> = Vec::new();
                    for chunk in pos_vals.as_chunks::<3>().0 {
                        raw_pts.push(Vec3::new(chunk[0], chunk[1], chunk[2]));
                    }
                    let mut verts: Vec<Vertex3D> = Vec::new();
                    let mut tris: Vec<[u32; 3]> = Vec::new();
                    for tri in idx_vals.as_chunks::<3>().0 {
                        let (i0, i1, i2): (usize, usize, usize) =
                            (tri[0] as usize, tri[1] as usize, tri[2] as usize);
                        if i0 < raw_pts.len() && i1 < raw_pts.len() && i2 < raw_pts.len() {
                            let p0: Vec3 = raw_pts[i0];
                            let p1: Vec3 = raw_pts[i1];
                            let p2: Vec3 = raw_pts[i2];
                            let n: Vec3 = p1.sub(p0).cross(p2.sub(p0)).normalize();
                            let base: u32 = verts.len() as u32;
                            verts.push(Vertex3D::new(p0, n));
                            verts.push(Vertex3D::new(p1, n));
                            verts.push(Vertex3D::new(p2, n));
                            tris.push([base, base + 1, base + 2]);
                        }
                    }
                    if !tris.is_empty() {
                        total_vertices += verts.len();
                        total_triangles += tris.len();
                        let mesh: Mesh3D = Mesh3D::from_parts(mesh_name.clone(), verts, tris, mat);
                        chains.push(MeshLodChain::from_mesh(mesh));
                    }
                }
            }
        }
        if chains.is_empty() {
            return Err(ERR_GLTF_EMPTY.to_string());
        }
        Ok(ImportedModel3D {
            name,
            format: ModelFormat::GltfJson,
            meshes: chains,
            total_vertices,
            total_triangles,
        })
    }
}
