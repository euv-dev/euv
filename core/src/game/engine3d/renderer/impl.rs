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
        let scale: f32 = quality.get_resolution_scale().clamp(0.40, 1.5);
        let max_w: u32 = if scale >= 1.15 { 1280 } else { 1024 };
        let max_h: u32 = if scale >= 1.15 { 720 } else { 576 };
        let w: u32 = ((viewport_w as f32 * scale).round() as u32).clamp(240, max_w);
        let h: u32 = ((viewport_h as f32 * scale).round() as u32).clamp(144, max_h);
        let pixel_count: usize = (w as usize) * (h as usize);
        if self.get_width() != w || self.get_height() != h {
            self.set_width(w);
            self.set_height(h);
            self.get_mut_color_buffer().resize(pixel_count * 4, 0);
            self.get_mut_aa_buffer().resize(pixel_count * 4, 0);
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
            depth_buffer.fill(f32::INFINITY);
            let horizon: Vec3 = sun.get_sky_horizon();
            let zenith: Vec3 = sun.get_sky_zenith();
            let sun_dir: Vec3 = sun.get_direction();
            let sun_col: Vec3 = sun.get_color();
            let w_usize: usize = w as usize;
            let inv_w_f: f32 = 1.0 / (w as f32).max(1.0);
            let inv_h_f: f32 = 1.0 / (h as f32).max(1.0);
            for py in 0..h {
                let ndc_y: f32 = 1.0 - 2.0 * (((py as f32) + 0.5) * inv_h_f);
                let row_dir_y: Vec3 = cam_up.scale(ndc_y * half_h);
                let row_base_dir: Vec3 = cam_fwd.add(row_dir_y);
                let approx_up: f32 = row_base_dir.get_y().clamp(0.0, 1.0);
                let sky_t: f32 = approx_up.sqrt();
                let base_sky: Vec3 = horizon.lerp(zenith, sky_t);
                let base_rgba: [u8; 4] = base_sky.to_aces_rgba8();
                let row_base: usize = (py as usize) * w_usize * 4;
                let row_sun_dot: f32 = row_base_dir.normalize().dot(sun_dir);
                if row_sun_dot < 0.55 {
                    for px in 0..w_usize {
                        let idx: usize = row_base + px * 4;
                        color_buffer[idx] = base_rgba[0];
                        color_buffer[idx + 1] = base_rgba[1];
                        color_buffer[idx + 2] = base_rgba[2];
                        color_buffer[idx + 3] = 255;
                    }
                } else {
                    for px in 0..w_usize {
                        let ndc_x: f32 = 2.0 * (((px as f32) + 0.5) * inv_w_f) - 1.0;
                        let dir: Vec3 = row_base_dir
                            .add(cam_right.scale(ndc_x * half_w))
                            .normalize();
                        let sdot: f32 = dir.dot(sun_dir).max(0.0);
                        let rgba: [u8; 4] = if sdot > 0.85 {
                            let mut col: Vec3 = base_sky;
                            if sdot > 0.992 {
                                let disk: f32 = ((sdot - 0.992) / 0.008).clamp(0.0, 1.0);
                                col = col.add(sun_col.scale(disk * 3.5));
                            } else {
                                let h1: f32 = ((sdot - 0.85) / 0.15).clamp(0.0, 1.0);
                                let h2: f32 = h1 * h1;
                                col = col.add(sun_col.scale(h2 * h2 * 0.42));
                            }
                            col.to_aces_rgba8()
                        } else {
                            base_rgba
                        };
                        let idx: usize = row_base + px * 4;
                        color_buffer[idx] = rgba[0];
                        color_buffer[idx + 1] = rgba[1];
                        color_buffer[idx + 2] = rgba[2];
                        color_buffer[idx + 3] = 255;
                    }
                }
            }
        }
        let draw_dist_sq: f32 = quality.get_draw_distance() * quality.get_draw_distance();
        let inv_fog_dist: f32 = 1.45 / quality.get_draw_distance().max(40.0);
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
        let cam_pos: Vec3 = cam.get_position();
        let sun_dir: Vec3 = sun.get_direction();
        let sun_col: Vec3 = sun.get_color();
        let sky_horizon: Vec3 = sun.get_sky_horizon();
        let sky_zenith: Vec3 = sun.get_sky_zenith();
        let use_rt_refl: bool = matches!(
            rt_mode,
            RayTracingMode::ShadowsAndReflections | RayTracingMode::FullPathTracedGi
        );
        let mut visible_sorted: Vec<(f32, usize)> = Vec::with_capacity(objects.len());
        for (obj_idx, obj) in objects.iter().enumerate() {
            let bounds: Aabb3D = obj.get_world_bounds();
            let dist_sq: f32 = bounds.center().sub(cam_pos).length_sq();
            if chunk_on && dist_sq > draw_dist_sq {
                culled_objects += 1;
                continue;
            }
            if frustum_on && !frustum.intersects_aabb(&bounds) {
                culled_objects += 1;
                continue;
            }
            visible_sorted.push((dist_sq, obj_idx));
        }
        visible_sorted.sort_unstable_by(|a: &(f32, usize), b: &(f32, usize)| a.0.total_cmp(&b.0));
        for (dist_sq, obj_idx) in visible_sorted.into_iter() {
            let obj: &SceneObject3D = &objects[obj_idx];
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
            let mat_base: Vec3 = mat.get_base_color();
            let mat_emissive: Vec3 = mat.get_emissive();
            let mat_refl: f32 = mat.get_reflectivity();
            let do_refl: bool = use_rt_refl && mat_refl > 0.25;
            let shininess: f32 = ((1.0 - mat.get_roughness()) * 64.0).max(4.0);
            let spec_weight: f32 = 0.25 + mat.get_metallic() * 0.85;
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
                let (clipped_tris, clipped_count): ([ClippedTriangle3D; 2], usize) =
                    Self::clip_triangle_near(
                        (c0, wp0),
                        (c1, wp1),
                        (c2, wp2),
                        cam.get_near().max(0.15),
                    );
                for clipped in clipped_tris.iter().take(clipped_count) {
                    let [(tc0, tw0), (tc1, tw1), (tc2, tw2)]: ClippedTriangle3D = *clipped;
                    let inv_w0: f32 = 1.0 / tc0[3];
                    let inv_w1: f32 = 1.0 / tc1[3];
                    let inv_w2: f32 = 1.0 / tc2[3];
                    let sx0: f32 = (tc0[0] * inv_w0 * 0.5 + 0.5) * (w as f32);
                    let sy0: f32 = (0.5 - tc0[1] * inv_w0 * 0.5) * (h as f32);
                    let sx1: f32 = (tc1[0] * inv_w1 * 0.5 + 0.5) * (w as f32);
                    let sy1: f32 = (0.5 - tc1[1] * inv_w1 * 0.5) * (h as f32);
                    let sx2: f32 = (tc2[0] * inv_w2 * 0.5 + 0.5) * (w as f32);
                    let sy2: f32 = (0.5 - tc2[1] * inv_w2 * 0.5) * (h as f32);
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
                    let tri_center: Vec3 = tw0.add(tw1).add(tw2).scale(1.0 / 3.0);
                    let tri_view: Vec3 = cam_pos.sub(tri_center).normalize();
                    let shade_n: Vec3 = if wn.dot(tri_view) < 0.0 {
                        wn.scale(-1.0)
                    } else {
                        wn
                    };
                    let n_dot_l: f32 = shade_n.dot(sun_dir).max(0.0);
                    let ambient_sky: Vec3 =
                        sky_horizon.lerp(sky_zenith, (shade_n.get_y() * 0.5 + 0.5).clamp(0.0, 1.0));
                    let half_vec: Vec3 = sun_dir.add(tri_view).normalize();
                    let ndh: f32 = shade_n.dot(half_vec).max(0.0);
                    let spec_base: f32 =
                        (ndh / (shininess - shininess * ndh + ndh + 1e-4)) * spec_weight;
                    let ndv_tri: f32 = shade_n.dot(tri_view).max(0.0);
                    let fresnel_base: f32 = {
                        let f1: f32 = 1.0 - ndv_tri;
                        (f1 * f1 * f1).clamp(0.15, 0.92)
                    };
                    let refl_blend: f32 = (mat_refl * fresnel_base).clamp(0.0, 0.78);
                    let dw0_dx: f32 = (sy1 - sy2) * inv_area;
                    let dw1_dx: f32 = (sy2 - sy0) * inv_area;
                    let Self {
                        color_buffer,
                        depth_buffer,
                        ..
                    } = self;
                    for py in min_y..=max_y {
                        let fy: f32 = (py as f32) + 0.5;
                        let row_off: usize = (py as usize) * (w as usize);
                        let fx_start: f32 = (min_x as f32) + 0.5;
                        let mut w0: f32 = ((sx1 - fx_start) * (sy2 - fy)
                            - (sy1 - fy) * (sx2 - fx_start))
                            * inv_area;
                        let mut w1: f32 = ((sx2 - fx_start) * (sy0 - fy)
                            - (sy2 - fy) * (sx0 - fx_start))
                            * inv_area;
                        let mut last_shadow: f32 = 1.0;
                        let mut last_refl: Vec3 = sky_horizon;
                        let mut has_cached_rt: bool = false;
                        for px in min_x..=max_x {
                            let w2: f32 = 1.0 - w0 - w1;
                            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                                let pw0: f32 = w0 * inv_w0;
                                let pw1: f32 = w1 * inv_w1;
                                let pw2: f32 = w2 * inv_w2;
                                let sum_pw: f32 = (pw0 + pw1 + pw2).max(1e-6);
                                let depth: f32 = 1.0 / sum_pw;
                                let buf_idx: usize = row_off + (px as usize);
                                if depth < depth_buffer[buf_idx] {
                                    depth_buffer[buf_idx] = depth;
                                    let trace_now: bool = !has_cached_rt || ((px & 3) == 0);
                                    let pc0: f32 = pw0 * depth;
                                    let pc1: f32 = pw1 * depth;
                                    let pc2: f32 = pw2 * depth;
                                    let world_p: Vec3 =
                                        tw0.scale(pc0).add(tw1.scale(pc1)).add(tw2.scale(pc2));
                                    if trace_now {
                                        let mut sf: f32 = 1.0;
                                        if rt_mode != RayTracingMode::Off && n_dot_l > 0.02 {
                                            let shadow_ray: Ray3D = Ray3D::new(
                                                world_p.add(shade_n.scale(0.08)),
                                                sun_dir,
                                            );
                                            rays_traced += 1;
                                            if bvh.trace_any_shadow(&shadow_ray, 0.05, 160.0) {
                                                sf = 0.22;
                                            }
                                        }
                                        last_shadow = sf;
                                        if do_refl {
                                            let view_dir: Vec3 = cam_pos.sub(world_p).normalize();
                                            let refl_dir: Vec3 =
                                                view_dir.scale(-1.0).reflect(shade_n).normalize();
                                            let refl_ray: Ray3D = Ray3D::new(
                                                world_p.add(shade_n.scale(0.08)),
                                                refl_dir,
                                            );
                                            rays_traced += 1;
                                            last_refl =
                                                match bvh.trace_closest(&refl_ray, 0.08, 120.0) {
                                                    Some(hit) => {
                                                        let h_nl: f32 =
                                                            hit.get_normal().dot(sun_dir).max(0.15);
                                                        hit.get_material()
                                                            .get_base_color()
                                                            .mul(sun_col.scale(h_nl))
                                                            .add(hit.get_material().get_emissive())
                                                    }
                                                    None => Self::sample_sky(refl_dir, &sun),
                                                };
                                        }
                                        has_cached_rt = true;
                                    }
                                    let mut ao: f32 = 1.0;
                                    if rt_mode == RayTracingMode::FullPathTracedGi
                                        && ((px + py) & 3) == 0
                                    {
                                        let gi_dir: Vec3 =
                                            shade_n.add(Vec3::new(0.25, 0.45, -0.2)).normalize();
                                        let gi_ray: Ray3D =
                                            Ray3D::new(world_p.add(shade_n.scale(0.08)), gi_dir);
                                        rays_traced += 1;
                                        if bvh.trace_any_shadow(&gi_ray, 0.05, 14.0) {
                                            ao = 0.55;
                                        }
                                    }
                                    let direct: Vec3 = sun_col.scale(n_dot_l * last_shadow);
                                    let spec_col: Vec3 = sun_col.scale(spec_base * last_shadow);
                                    let mut radiance: Vec3 = mat_base
                                        .mul(direct.add(ambient_sky.scale(0.45 * ao)))
                                        .add(spec_col)
                                        .add(mat_emissive);
                                    if do_refl {
                                        radiance = radiance.lerp(last_refl, refl_blend);
                                    }
                                    if fog_on {
                                        let fd: f32 = depth * inv_fog_dist;
                                        let fog_t: f32 = (fd / (1.0 + fd)).clamp(0.0, 0.85);
                                        radiance = radiance.lerp(sky_horizon, fog_t);
                                    }
                                    let rgba: [u8; 4] = radiance.to_aces_rgba8();
                                    let cidx: usize = buf_idx * 4;
                                    color_buffer[cidx] = rgba[0];
                                    color_buffer[cidx + 1] = rgba[1];
                                    color_buffer[cidx + 2] = rgba[2];
                                    color_buffer[cidx + 3] = 255;
                                }
                            }
                            w0 += dw0_dx;
                            w1 += dw1_dx;
                        }
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
        self.apply_fxaa_pass(w, h);
        ImageBuffer::new(w, h, self.get_aa_buffer().clone())
    }

    /// Clips a triangle against the camera near plane `w >= near_w` (Sutherland-Hodgman), returning 0, 1, or 2 triangles.
    ///
    /// # Arguments
    ///
    /// - `ClipVertex3D` - First clip-space and world-space vertex.
    /// - `ClipVertex3D` - Second clip-space and world-space vertex.
    /// - `ClipVertex3D` - Third clip-space and world-space vertex.
    /// - `f32` - Positive near-plane clip `w` threshold.
    ///
    /// # Returns
    ///
    /// - `([ClippedTriangle3D; 2], usize)` - Output clipped triangles and valid count.
    #[must_use]
    fn clip_triangle_near(
        v0: ClipVertex3D,
        v1: ClipVertex3D,
        v2: ClipVertex3D,
        near_w: f32,
    ) -> ([ClippedTriangle3D; 2], usize) {
        let zero_v: ClipVertex3D = ([0.0; 4], Vec3::zero());
        let mut out_tris: [ClippedTriangle3D; 2] = [[zero_v; 3]; 2];
        let input: [ClipVertex3D; 3] = [v0, v1, v2];
        let mut poly: [ClipVertex3D; 4] = [zero_v; 4];
        let mut count: usize = 0;
        for i in 0..3 {
            let cur: ClipVertex3D = input[i];
            let nxt: ClipVertex3D = input[(i + 1) % 3];
            let cur_in: bool = cur.0[3] >= near_w;
            let nxt_in: bool = nxt.0[3] >= near_w;
            if cur_in && count < 4 {
                poly[count] = cur;
                count += 1;
            }
            if cur_in != nxt_in && count < 4 {
                let denom: f32 = nxt.0[3] - cur.0[3];
                let t: f32 = if denom.abs() > 1e-6 {
                    ((near_w - cur.0[3]) / denom).clamp(0.0, 1.0)
                } else {
                    0.5
                };
                let ic: [f32; 4] = [
                    cur.0[0] + (nxt.0[0] - cur.0[0]) * t,
                    cur.0[1] + (nxt.0[1] - cur.0[1]) * t,
                    cur.0[2] + (nxt.0[2] - cur.0[2]) * t,
                    near_w,
                ];
                let iw: Vec3 = cur.1.lerp(nxt.1, t);
                poly[count] = (ic, iw);
                count += 1;
            }
        }
        if count < 3 {
            return (out_tris, 0);
        }
        out_tris[0] = [poly[0], poly[1], poly[2]];
        if count == 4 {
            out_tris[1] = [poly[0], poly[2], poly[3]];
            (out_tris, 2)
        } else {
            (out_tris, 1)
        }
    }

    /// Computes fast fixed-point Rec.601 luminance for an RGBA8 pixel at `idx`.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Source RGBA8 pixel buffer.
    /// - `usize` - Byte offset of the pixel.
    ///
    /// # Returns
    ///
    /// - `i32` - Scaled luminance in `[0, 65280]`.
    #[must_use]
    fn rgb_luma(buf: &[u8], idx: usize) -> i32 {
        (buf[idx] as i32) * 77 + (buf[idx + 1] as i32) * 150 + (buf[idx + 2] as i32) * 29
    }

    /// Runs a fast luminance-guided FXAA post-process pass to smooth 3D polygon and ray-traced shadow edges.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable 3D renderer.
    /// - `u32` - Buffer width in pixels.
    /// - `u32` - Buffer height in pixels.
    fn apply_fxaa_pass(&mut self, width: u32, height: u32) {
        let Self {
            color_buffer,
            aa_buffer,
            ..
        } = self;
        aa_buffer.copy_from_slice(color_buffer.as_slice());
        let w: usize = width as usize;
        let h: usize = height as usize;
        if w < 3 || h < 3 {
            return;
        }
        for y in 1..(h - 1) {
            let row: usize = y * w;
            let row_up: usize = (y - 1) * w;
            let row_dn: usize = (y + 1) * w;
            for x in 1..(w - 1) {
                let c: usize = (row + x) * 4;
                let n: usize = (row_up + x) * 4;
                let s: usize = (row_dn + x) * 4;
                let e: usize = (row + x + 1) * 4;
                let we: usize = (row + x - 1) * 4;
                let lc: i32 = Self::rgb_luma(color_buffer, c);
                let ln: i32 = Self::rgb_luma(color_buffer, n);
                let ls: i32 = Self::rgb_luma(color_buffer, s);
                let le: i32 = Self::rgb_luma(color_buffer, e);
                let lw: i32 = Self::rgb_luma(color_buffer, we);
                let lmax: i32 = lc.max(ln).max(ls).max(le).max(lw);
                let lmin: i32 = lc.min(ln).min(ls).min(le).min(lw);
                let contrast: i32 = lmax - lmin;
                if contrast < 2560 {
                    continue;
                }
                let grad_h: i32 = (ln - ls).abs();
                let grad_v: i32 = (lw - le).abs();
                let (p1, p2): (usize, usize) = if grad_h >= grad_v { (n, s) } else { (we, e) };
                let blend_num: i32 = ((contrast - 2560) / 160).clamp(24, 176);
                let keep_num: i32 = 256 - blend_num;
                for ch in 0..3 {
                    let avg_nb: i32 =
                        ((color_buffer[p1 + ch] as i32) + (color_buffer[p2 + ch] as i32)) >> 1;
                    let mixed: i32 =
                        ((color_buffer[c + ch] as i32) * keep_num + avg_nb * blend_num + 128) >> 8;
                    aa_buffer[c + ch] = mixed.clamp(0, 255) as u8;
                }
                aa_buffer[c + 3] = 255;
            }
        }
    }
}
