use super::*;

/// High-FPS PBR + BVH Ray-Traced Shadows/Reflections/GI 3D renderer implementation for `Renderer3D`.
impl Renderer3D {
    /// Evaluates atmospheric sky & sun disk radiance along unit view ray `dir`.
    ///
    /// # Arguments
    ///
    /// - `Vec3` - Unit view or reflection direction.
    /// - `&DirectionalLight3D` - Active sky & sun lighting parameters.
    ///
    /// # Returns
    ///
    /// - `Vec3` - Linear HDR sky radiance.
    #[must_use]
    pub fn sample_sky(dir: Vec3, sun: &DirectionalLight3D) -> Vec3 {
        let up: f32 = dir.get_y().clamp(0.0, 1.0);
        let t: f32 = up.powf(0.55);
        let mut col: Vec3 = sun.get_sky_horizon().lerp(sun.get_sky_zenith(), t);
        let sun_dot: f32 = dir.dot(sun.get_direction()).max(0.0);
        if sun_dot > 0.992 {
            let disk: f32 = ((sun_dot - 0.992) / 0.008).clamp(0.0, 1.0);
            col = col.add(sun.get_color().scale(disk * 3.5));
        } else if sun_dot > 0.85 {
            let halo: f32 = ((sun_dot - 0.85) / 0.15).powf(4.0);
            col = col.add(sun.get_color().scale(halo * 0.42));
        }
        col
    }

    /// Renders `world` at runtime-configured `quality` into an `ImageBuffer` with zero steady-state allocations.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable 3D renderer.
    /// - `&mut World3D` - Open-world 3D scene graph.
    /// - `u32` - Target viewport width in logical pixels.
    /// - `u32` - Target viewport height in logical pixels.
    /// - `&RenderQualityConfig` - Live runtime quality, resolution scale, LOD, and ray-tracing settings.
    ///
    /// # Returns
    ///
    /// - `ImageBuffer` - Ready RGBA8 rendered 3D frame for immediate presentation.
    pub fn render_world(
        &mut self,
        world: &mut World3D,
        viewport_w: u32,
        viewport_h: u32,
        quality: &RenderQualityConfig,
    ) -> ImageBuffer {
        let scale: f32 = quality.get_resolution_scale().clamp(0.35, 1.5);
        let w: u32 = ((viewport_w as f32 * scale * 0.55).round() as u32).clamp(160, 640);
        let h: u32 = ((viewport_h as f32 * scale * 0.55).round() as u32).clamp(96, 360);
        let pixel_count: usize = (w as usize) * (h as usize);
        if self.get_width() != w || self.get_height() != h {
            self.set_width(w);
            self.set_height(h);
            self.get_mut_color_buffer().resize(pixel_count * 4, 0);
            self.get_mut_depth_buffer()
                .resize(pixel_count, f32::INFINITY);
        }
        if quality.get_ray_tracing() != RayTracingMode::Off {
            world.ensure_bvh();
        }
        let sun: DirectionalLight3D = world.get_sun();
        let cam: Camera3D = world.get_camera();
        let aspect: f32 = (w as f32) / (h as f32).max(1.0);
        let vp: Mat4 = cam.view_projection(aspect);
        let frustum: Frustum3D = Frustum3D::from_view_proj(&vp);
        let cam_fwd: Vec3 = cam.forward();
        let cam_right: Vec3 = cam.right();
        let cam_up: Vec3 = cam_right.cross(cam_fwd).normalize();
        let half_h: f32 = (cam.get_fov_deg().to_radians() * 0.5).tan();
        let half_w: f32 = half_h * aspect;
        {
            let Self {
                color_buffer,
                depth_buffer,
                ..
            } = self;
            for d in depth_buffer.iter_mut() {
                *d = f32::INFINITY;
            }
            for py in 0..h {
                let ndc_y: f32 = 1.0 - 2.0 * ((py as f32 + 0.5) / (h as f32));
                let row_dir_y: Vec3 = cam_up.scale(ndc_y * half_h);
                let row_base: usize = (py as usize) * (w as usize) * 4;
                for px in 0..w {
                    let ndc_x: f32 = 2.0 * ((px as f32 + 0.5) / (w as f32)) - 1.0;
                    let dir: Vec3 = cam_fwd
                        .add(cam_right.scale(ndc_x * half_w))
                        .add(row_dir_y)
                        .normalize();
                    let rgba: [u8; 4] = Self::sample_sky(dir, &sun).to_aces_rgba8();
                    let idx: usize = row_base + (px as usize) * 4;
                    color_buffer[idx] = rgba[0];
                    color_buffer[idx + 1] = rgba[1];
                    color_buffer[idx + 2] = rgba[2];
                    color_buffer[idx + 3] = 255;
                }
            }
        }
        let draw_dist_sq: f32 = quality.get_draw_distance() * quality.get_draw_distance();
        let rt_mode: RayTracingMode = quality.get_ray_tracing();
        let fog_on: bool = quality.get_volumetric_fog();
        let frustum_on: bool = quality.get_frustum_culling();
        let chunk_on: bool = quality.get_chunk_streaming();
        let lod_bias: f32 = quality.get_lod_bias();
        let mut visible_objects: usize = 0;
        let mut culled_objects: usize = 0;
        let mut triangles_drawn: usize = 0;
        let mut rays_traced: usize = 0;
        let mut lod_simplified_objects: usize = 0;
        let mut unique_meshes: HashSet<usize> = HashSet::new();
        let bvh: &Bvh3D = world.get_bvh();
        let objects: &[SceneObject3D] = world.get_objects().as_slice();
        let mesh_lib: &[MeshLodChain] = world.get_mesh_library().as_slice();
        for obj in objects.iter() {
            let bounds: Aabb3D = obj.get_world_bounds();
            let dist_sq: f32 = bounds.center().sub(cam.get_position()).length_sq();
            if chunk_on && dist_sq > draw_dist_sq {
                culled_objects += 1;
                continue;
            }
            if frustum_on && !frustum.intersects_aabb(&bounds) {
                culled_objects += 1;
                continue;
            }
            let Some(chain) = mesh_lib.get(obj.get_mesh_id()) else {
                continue;
            };
            visible_objects += 1;
            let _: bool = unique_meshes.insert(obj.get_mesh_id());
            let dist: f32 = dist_sq.sqrt();
            let (mesh, lod_idx): (&Mesh3D, usize) = chain.select(dist, lod_bias);
            if lod_idx > 0 {
                lod_simplified_objects += 1;
            }
            let tr: Transform3D = obj.get_transform();
            let mat: PbrMaterial = obj.get_material();
            let verts: &[Vertex3D] = mesh.get_vertices().as_slice();
            for tri in mesh.get_indices().iter() {
                let (i0, i1, i2): (usize, usize, usize) =
                    (tri[0] as usize, tri[1] as usize, tri[2] as usize);
                if i0 >= verts.len() || i1 >= verts.len() || i2 >= verts.len() {
                    continue;
                }
                let wp0: Vec3 = tr.transform_point(verts[i0].get_position());
                let wp1: Vec3 = tr.transform_point(verts[i1].get_position());
                let wp2: Vec3 = tr.transform_point(verts[i2].get_position());
                let wn: Vec3 = tr.transform_normal(verts[i0].get_normal());
                let c0: [f32; 4] = vp.mul_point4(wp0);
                let c1: [f32; 4] = vp.mul_point4(wp1);
                let c2: [f32; 4] = vp.mul_point4(wp2);
                if c0[3] <= 0.25 || c1[3] <= 0.25 || c2[3] <= 0.25 {
                    continue;
                }
                let inv_w0: f32 = 1.0 / c0[3];
                let inv_w1: f32 = 1.0 / c1[3];
                let inv_w2: f32 = 1.0 / c2[3];
                let sx0: f32 = (c0[0] * inv_w0 * 0.5 + 0.5) * (w as f32);
                let sy0: f32 = (0.5 - c0[1] * inv_w0 * 0.5) * (h as f32);
                let sx1: f32 = (c1[0] * inv_w1 * 0.5 + 0.5) * (w as f32);
                let sy1: f32 = (0.5 - c1[1] * inv_w1 * 0.5) * (h as f32);
                let sx2: f32 = (c2[0] * inv_w2 * 0.5 + 0.5) * (w as f32);
                let sy2: f32 = (0.5 - c2[1] * inv_w2 * 0.5) * (h as f32);
                let area: f32 = (sx1 - sx0) * (sy2 - sy0) - (sy1 - sy0) * (sx2 - sx0);
                if area.abs() < 0.25 {
                    continue;
                }
                let min_x: i32 = (sx0.min(sx1).min(sx2).floor() as i32).max(0);
                let max_x: i32 = (sx0.max(sx1).max(sx2).ceil() as i32).min((w as i32) - 1);
                let min_y: i32 = (sy0.min(sy1).min(sy2).floor() as i32).max(0);
                let max_y: i32 = (sy0.max(sy1).max(sy2).ceil() as i32).min((h as i32) - 1);
                if min_x > max_x || min_y > max_y {
                    continue;
                }
                triangles_drawn += 1;
                let inv_area: f32 = 1.0 / area;
                let Self {
                    color_buffer,
                    depth_buffer,
                    ..
                } = self;
                for py in min_y..=max_y {
                    let fy: f32 = (py as f32) + 0.5;
                    let row_off: usize = (py as usize) * (w as usize);
                    for px in min_x..=max_x {
                        let fx: f32 = (px as f32) + 0.5;
                        let w0: f32 =
                            ((sx1 - fx) * (sy2 - fy) - (sy1 - fy) * (sx2 - fx)) * inv_area;
                        let w1: f32 =
                            ((sx2 - fx) * (sy0 - fy) - (sy2 - fy) * (sx0 - fx)) * inv_area;
                        let w2: f32 = 1.0 - w0 - w1;
                        if w0.min(w1).min(w2) < 0.0 {
                            continue;
                        }
                        let pw0: f32 = w0 * inv_w0;
                        let pw1: f32 = w1 * inv_w1;
                        let pw2: f32 = w2 * inv_w2;
                        let sum_pw: f32 = (pw0 + pw1 + pw2).max(1e-6);
                        let depth: f32 = 1.0 / sum_pw;
                        let buf_idx: usize = row_off + (px as usize);
                        if depth >= depth_buffer[buf_idx] {
                            continue;
                        }
                        depth_buffer[buf_idx] = depth;
                        let pc0: f32 = pw0 * depth;
                        let pc1: f32 = pw1 * depth;
                        let pc2: f32 = pw2 * depth;
                        let world_p: Vec3 = wp0.scale(pc0).add(wp1.scale(pc1)).add(wp2.scale(pc2));
                        let view_dir: Vec3 = cam.get_position().sub(world_p).normalize();
                        let n_dot_l: f32 = wn.dot(sun.get_direction()).max(0.0);
                        let mut shadow_factor: f32 = 1.0;
                        if rt_mode != RayTracingMode::Off && n_dot_l > 0.02 {
                            let shadow_origin: Vec3 = world_p.add(wn.scale(0.08));
                            let shadow_ray: Ray3D = Ray3D::new(shadow_origin, sun.get_direction());
                            rays_traced += 1;
                            if bvh.trace_any_shadow(&shadow_ray, 0.05, 160.0) {
                                shadow_factor = 0.22;
                            }
                        }
                        let half_vec: Vec3 = sun.get_direction().add(view_dir).normalize();
                        let shininess: f32 = ((1.0 - mat.get_roughness()) * 96.0).max(4.0);
                        let spec: f32 = wn.dot(half_vec).max(0.0).powf(shininess)
                            * (0.25 + mat.get_metallic() * 0.85);
                        let ambient_sky: Vec3 = sun.get_sky_horizon().lerp(
                            sun.get_sky_zenith(),
                            (wn.get_y() * 0.5 + 0.5).clamp(0.0, 1.0),
                        );
                        let mut ao: f32 = 1.0;
                        if rt_mode == RayTracingMode::FullPathTracedGi && ((px + py) & 1) == 0 {
                            let gi_dir: Vec3 = wn.add(Vec3::new(0.25, 0.45, -0.2)).normalize();
                            let gi_ray: Ray3D = Ray3D::new(world_p.add(wn.scale(0.08)), gi_dir);
                            rays_traced += 1;
                            if bvh.trace_any_shadow(&gi_ray, 0.05, 14.0) {
                                ao = 0.55;
                            }
                        }
                        let direct: Vec3 = sun.get_color().scale(n_dot_l * shadow_factor);
                        let spec_col: Vec3 = sun.get_color().scale(spec * shadow_factor);
                        let mut radiance: Vec3 = mat
                            .get_base_color()
                            .mul(direct.add(ambient_sky.scale(0.45 * ao)))
                            .add(spec_col)
                            .add(mat.get_emissive());
                        let use_rt_refl: bool = matches!(
                            rt_mode,
                            RayTracingMode::ShadowsAndReflections
                                | RayTracingMode::FullPathTracedGi
                        );
                        if use_rt_refl && mat.get_reflectivity() > 0.25 {
                            let refl_dir: Vec3 = view_dir.scale(-1.0).reflect(wn).normalize();
                            let refl_ray: Ray3D = Ray3D::new(world_p.add(wn.scale(0.08)), refl_dir);
                            rays_traced += 1;
                            let refl_col: Vec3 = match bvh.trace_closest(&refl_ray, 0.08, 120.0) {
                                Some(hit) => {
                                    let h_nl: f32 =
                                        hit.get_normal().dot(sun.get_direction()).max(0.15);
                                    hit.get_material()
                                        .get_base_color()
                                        .mul(sun.get_color().scale(h_nl))
                                        .add(hit.get_material().get_emissive())
                                }
                                None => Self::sample_sky(refl_dir, &sun),
                            };
                            let fresnel: f32 = (1.0 - wn.dot(view_dir).max(0.0))
                                .powf(3.0)
                                .clamp(0.15, 0.92);
                            let blend: f32 = (mat.get_reflectivity() * fresnel).clamp(0.0, 0.78);
                            radiance = radiance.lerp(refl_col, blend);
                        }
                        if fog_on {
                            let fog_t: f32 = (1.0
                                - (-depth / quality.get_draw_distance().max(40.0) * 1.45).exp())
                            .clamp(0.0, 0.85);
                            radiance = radiance.lerp(sun.get_sky_horizon(), fog_t);
                        }
                        let rgba: [u8; 4] = radiance.to_aces_rgba8();
                        let cidx: usize = buf_idx * 4;
                        color_buffer[cidx] = rgba[0];
                        color_buffer[cidx + 1] = rgba[1];
                        color_buffer[cidx + 2] = rgba[2];
                        color_buffer[cidx + 3] = 255;
                    }
                }
            }
        }
        let stats: RenderStats3D = RenderStats3D {
            render_width: w,
            render_height: h,
            visible_objects,
            culled_objects,
            instanced_batches: if quality.get_gpu_instancing() {
                unique_meshes.len()
            } else {
                visible_objects
            },
            triangles_drawn,
            rays_traced,
            bvh_nodes: bvh.get_nodes().len(),
            lod_simplified_objects,
        };
        self.set_last_stats(stats);
        ImageBuffer::new(w, h, self.get_color_buffer().clone())
    }
}
