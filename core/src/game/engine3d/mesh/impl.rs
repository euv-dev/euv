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
        }
    }
}

/// Material presets for urban architecture, reflective asphalt, glass towers, and vehicles.
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
        }
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
        }
    }
}

/// Mesh construction, bounding-box calculation, decimation, and procedural asset generators for `Mesh3D`.
impl Mesh3D {
    /// Constructs a `Mesh3D` and computes its exact local-space `Aabb3D`.
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
    /// - `Self` - Ready indexed mesh.
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
        Self {
            name,
            vertices,
            indices,
            bounds,
            material,
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
        let lower: Self = Self::box_mesh("lower", 2.2, 0.80, 4.6, true, material);
        let cabin: Self = Self::box_mesh("cabin", 1.82, 0.72, 2.2, true, material);
        let wheel: Self = Self::box_mesh("wheel", 0.34, 0.56, 0.62, true, material);
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

    /// Builds a simplified proxy mesh enclosing this mesh's AABB for distant LOD tiers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - High-detail source mesh.
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
        proxy
    }
}

/// Multi-level LOD chain creation and distance-based selection for `MeshLodChain`.
impl MeshLodChain {
    /// Builds a 2-tier LOD chain (`LOD0` full mesh + `LOD1` simplified bounding proxy) automatically.
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
        let lod1: Mesh3D = if mesh.get_indices().len() > 16 {
            mesh.build_lod_proxy("lod1")
        } else {
            mesh.clone()
        };
        Self {
            levels: vec![mesh, lod1],
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
        if levels.len() > 1 && effective_dist > 85.0 {
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
        let mut model_name: String = "imported_obj".to_string();
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
            return Err("OBJ source contained no valid triangles".to_string());
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
            .ok_or_else(|| "invalid glTF JSON".to_string())?;
        let name: String = root
            .get("asset")
            .and_then(|a: &Value| a.get("generator"))
            .and_then(Value::as_str)
            .unwrap_or("imported_gltf")
            .to_string();
        let mut mat: PbrMaterial = fallback_material;
        if let Some(pbr) = root
            .get("materials")
            .and_then(Value::as_array)
            .and_then(|arr: &Vec<Value>| arr.first())
            .and_then(|m: &Value| m.get("pbrMetallicRoughness"))
        {
            if let Some(bc) = pbr.get("baseColorFactor").and_then(Value::as_array)
                && bc.len() >= 3
            {
                let r: f32 = bc[0].as_f64().unwrap_or(0.8) as f32;
                let g: f32 = bc[1].as_f64().unwrap_or(0.8) as f32;
                let b: f32 = bc[2].as_f64().unwrap_or(0.8) as f32;
                mat.set_base_color(Vec3::new(r, g, b));
            }
            if let Some(met) = pbr.get("metallicFactor").and_then(Value::as_f64) {
                mat.set_metallic((met as f32).clamp(0.0, 1.0));
            }
            if let Some(rough) = pbr.get("roughnessFactor").and_then(Value::as_f64) {
                mat.set_roughness((rough as f32).clamp(0.04, 1.0));
                mat.set_reflectivity((1.0 - (rough as f32)).clamp(0.05, 0.95));
            }
        }
        let mut chains: Vec<MeshLodChain> = Vec::new();
        let mut total_vertices: usize = 0;
        let mut total_triangles: usize = 0;
        if let Some(meshes_arr) = root.get("meshes").and_then(Value::as_array) {
            for (m_idx, m_val) in meshes_arr.iter().enumerate() {
                let mesh_name: String = m_val
                    .get("name")
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("{name}_mesh_{m_idx}"));
                let Some(prims) = m_val.get("primitives").and_then(Value::as_array) else {
                    continue;
                };
                for prim in prims.iter() {
                    let extras: &Value = prim.get("extras").unwrap_or(prim);
                    let pos_vals: Vec<f32> = extras
                        .get("positions")
                        .and_then(Value::as_array)
                        .map(|a: &Vec<Value>| {
                            a.iter()
                                .filter_map(|v: &Value| v.as_f64().map(|n: f64| n as f32))
                                .collect()
                        })
                        .unwrap_or_default();
                    let idx_vals: Vec<u32> = extras
                        .get("indices")
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
            return Err("glTF 2.0 JSON contained no valid triangle primitives".to_string());
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
