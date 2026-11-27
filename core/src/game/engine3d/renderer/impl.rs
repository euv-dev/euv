use super::*;

/// Persistent work-stealing multi-core thread pool for `Engine3dWorkerPool`.
#[cfg(not(target_arch = "wasm32"))]
/// Inherent implementation of [`Engine3dWorkerPool`].
impl Engine3dWorkerPool {
    /// Creates a warm persistent worker pool sized to host CPU cores (`1..=8` total threads including caller).
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized worker pool with zero per-frame thread spawn overhead.
    #[must_use]
    pub(crate) fn new() -> Self {
        let total: usize = available_parallelism()
            .map(usize::from)
            .unwrap_or(4)
            .clamp(1, 8);
        let worker_count: usize = total.saturating_sub(1);
        let shared: Arc<(Mutex<Engine3dPoolState>, Condvar, Condvar)> = Arc::new((
            Mutex::new(Engine3dPoolState::default()),
            Condvar::new(),
            Condvar::new(),
        ));
        for idx in 0..worker_count {
            let shared_clone: Arc<(Mutex<Engine3dPoolState>, Condvar, Condvar)> =
                Arc::clone(&shared);
            let name: String = format!("{ENGINE3D_WORKER_PREFIX}-{idx}");
            let _: io::Result<JoinHandle<()>> = Builder::new().name(name).spawn(move || {
                let mut seen_epoch: u64 = 0;
                loop {
                    let job_ptr: usize = {
                        let (lock, wake_workers, _): &(Mutex<Engine3dPoolState>, Condvar, Condvar) =
                            &shared_clone;
                        let Ok(mut state) = lock.lock() else {
                            break;
                        };
                        while state.get_epoch() == seen_epoch || state.get_active_job() == 0 {
                            let Ok(next_guard) = wake_workers.wait(state) else {
                                return;
                            };
                            state = next_guard;
                        }
                        seen_epoch = state.get_epoch();
                        state.get_active_job()
                    };
                    let job: &Engine3dJob = unsafe { &*(job_ptr as *const Engine3dJob) };
                    run_engine3d_job_loop(job);
                    let finished: usize = job.workers_done.fetch_add(1, Ordering::AcqRel) + 1;
                    if finished >= worker_count {
                        let (lock, _, wake_caller): &(Mutex<Engine3dPoolState>, Condvar, Condvar) =
                            &shared_clone;
                        let _guard: Result<
                            MutexGuard<'_, Engine3dPoolState>,
                            PoisonError<MutexGuard<'_, Engine3dPoolState>>,
                        > = lock.lock();
                        wake_caller.notify_one();
                    }
                }
            });
        }
        Self {
            worker_count,
            dispatch_lock: Mutex::new(()),
            shared,
        }
    }
}

/// High-FPS UE5-class Deferred GGX PBR + Persistent Multi-Core 32B SAH BVH + Lumen GI + Volumetric Fog + Bilinear Bloom + Geometry-Aware Morphological AA/TAA renderer.
impl Renderer3D {
    /// Evaluates physically inspired Rayleigh + Mie atmospheric sky and HDR solar disk radiance along `dir`.
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
        let t: f32 = up.sqrt();
        let horizon_glow: f32 = (1.0 - up) * (1.0 - up);
        let mut col: Vec3 = sun
            .get_sky_horizon()
            .lerp(sun.get_sky_zenith(), t)
            .add(sun.get_sky_horizon().scale(horizon_glow * 0.14));
        let sun_dot: f32 = dir.dot(sun.get_direction()).max(0.0);
        if sun_dot > 0.992 {
            let disk: f32 = ((sun_dot - 0.992) / 0.008).clamp(0.0, 1.0);
            col = col.add(sun.get_color().scale(disk * 3.8 + 0.55));
        } else if sun_dot > 0.72 {
            let h1: f32 = ((sun_dot - 0.72) / 0.28).clamp(0.0, 1.0);
            let h2: f32 = h1 * h1;
            let mie: f32 = h2 * h2;
            col = col.add(sun.get_color().scale(mie * 0.58 + h2 * 0.08));
        }
        col
    }

    /// Reconstructs exact world-space position for pixel `(px, py)` at camera depth `depth`.
    ///
    /// # Arguments
    ///
    /// - `&DeferredFrameCtx` - Shared per-frame camera basis.
    /// - `usize` - Pixel X coordinate.
    /// - `usize` - Pixel Y coordinate.
    /// - `f32` - Linear camera-space depth (`1 / inv_w`).
    ///
    /// # Returns
    ///
    /// - `(Vec3, Vec3)` - `(world_pos, unnormalized_view_ray_dir)`.
    #[must_use]
    fn pixel_world_pos(fctx: &DeferredFrameCtx, px: usize, py: usize, depth: f32) -> (Vec3, Vec3) {
        let ndc_x: f32 = 2.0 * (((px as f32) + 0.5) * fctx.inv_w_f) - 1.0;
        let ndc_y: f32 = 1.0 - 2.0 * (((py as f32) + 0.5) * fctx.inv_h_f);
        let ray_vec: Vec3 = fctx
            .cam_fwd
            .add(fctx.cam_right_scaled.scale(ndc_x))
            .add(fctx.cam_up_scaled.scale(ndc_y));
        (fctx.cam_pos.add(ray_vec.scale(depth)), ray_vec)
    }

    /// Shades a single BVH closest-hit surface with multi-bounce sky/emissive radiance for reflections and Lumen GI.
    ///
    /// # Arguments
    ///
    /// - `&Bvh3D` - Scene BVH holding triangle materials.
    /// - `usize` - Winning triangle index.
    /// - `&DirectionalLight3D` - Active directional sun light.
    /// - `bool` - Enables secondary hemisphere sky energy conservation for multi-bounce Lumen GI.
    ///
    /// # Returns
    ///
    /// - `Vec3` - Linear HDR hit radiance.
    #[must_use]
    fn shade_bvh_hit(
        bvh: &Bvh3D,
        tri_idx: usize,
        sun: &DirectionalLight3D,
        multi_bounce: bool,
    ) -> Vec3 {
        let hit_tri: &BvhTriangle = &bvh.get_triangles()[tri_idx];
        let hit_n: Vec3 = hit_tri.get_normal();
        let h_nl: f32 = hit_n.dot(sun.get_direction()).abs().max(0.18);
        let mat: PbrMaterial = hit_tri.get_material();
        let mut col: Vec3 = mat
            .get_base_color()
            .mul(sun.get_color().scale(h_nl))
            .add(mat.get_emissive());
        if multi_bounce {
            let sky_hemi: Vec3 = sun.get_sky_horizon().lerp(
                sun.get_sky_zenith(),
                (hit_n.get_y() * 0.5 + 0.5).clamp(0.0, 1.0),
            );
            col = col.add(mat.get_base_color().mul(sky_hemi.scale(0.32)));
        }
        col
    }

    /// Evaluates a band of coarse deferred RT probes `[gy_start..gy_end)` with soft shadows, multi-bounce reflections, and Lumen GI.
    ///
    /// # Arguments
    ///
    /// - `&DeferredFrameCtx` - Shared per-frame camera and RT parameters.
    /// - `&[f32]` - Linear `1/w` depth buffer.
    /// - `&[u32]` - G-Buffer triangle index buffer.
    /// - `&[GShadeTri]` - Per-triangle shading records.
    /// - `&Bvh3D` - Scene SAH BVH.
    /// - `(usize, &mut [RtProbeSample])` - `(gy_start, mutable probe band slice)`.
    ///
    /// # Returns
    ///
    /// - `usize` - Total BVH rays traced within this band.
    fn trace_probe_band(
        fctx: &DeferredFrameCtx,
        depth_buf: &[f32],
        g_tri_idx: &[u32],
        shaded_tris: &[GShadeTri],
        bvh: &Bvh3D,
        band: (usize, &mut [RtProbeSample]),
    ) -> usize {
        let (gy_start, out_probes): (usize, &mut [RtProbeSample]) = band;
        let gw: usize = fctx.gw;
        let step: usize = fctx.rt_step;
        let half_step: usize = step / 2;
        let sun_dir: Vec3 = fctx.sun.get_direction();
        let inv_sun: Vec3 = fctx.inv_sun_dir;
        let sun_t1: Vec3 = sun_dir.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let sun_t2: Vec3 = sun_dir.cross(sun_t1).normalize();
        let soft_offsets: [(f32, f32); 4] = [
            (0.016, 0.012),
            (-0.014, 0.016),
            (0.012, -0.016),
            (-0.016, -0.012),
        ];
        let gi_jitter: [Vec3; 4] = [
            Vec3::new(0.38, 0.52, -0.24),
            Vec3::new(-0.34, 0.56, 0.28),
            Vec3::new(0.24, 0.48, 0.36),
            Vec3::new(-0.28, 0.50, -0.34),
        ];
        let use_soft_shadows: bool = fctx.rt_mode == RayTracingMode::FullPathTracedGi;
        let multi_bounce_refl: bool = fctx.reflection_bounces >= 2;
        let mut rays: usize = 0;
        let mut shadow_cache: u32 = u32::MAX;
        let band_rows: usize = out_probes.len() / gw.max(1);
        for local_gy in 0..band_rows {
            let gy: usize = gy_start + local_gy;
            let py: usize = (gy * step + half_step).min(fctx.h - 1);
            let row_off: usize = py * fctx.w;
            let probe_row: usize = local_gy * gw;
            for gx in 0..gw {
                let px: usize = (gx * step + half_step).min(fctx.w - 1);
                let pix_idx: usize = row_off + px;
                let tid: u32 = g_tri_idx[pix_idx];
                if tid == u32::MAX {
                    out_probes[probe_row + gx] = RtProbeSample::default();
                    continue;
                }
                let inv_w: f32 = depth_buf[pix_idx];
                let depth: f32 = 1.0 / inv_w.max(1e-6);
                let tri: &GShadeTri = &shaded_tris[tid as usize];
                let (world_p, ray_vec): (Vec3, Vec3) = Self::pixel_world_pos(fctx, px, py, depth);
                let origin: Vec3 = world_p.add(tri.shade_n.scale(0.08));
                let pattern: usize = (gx & 1) | ((gy & 1) << 1);
                let mut shadow: f32 = 1.0;
                if tri.n_dot_l > 0.02 {
                    rays += 1;
                    let (s_dir, s_inv): (Vec3, Vec3) = if use_soft_shadows {
                        let (ox, oy): (f32, f32) = soft_offsets[pattern];
                        let jittered: Vec3 = sun_dir
                            .add(sun_t1.scale(ox))
                            .add(sun_t2.scale(oy))
                            .normalize();
                        (jittered, Ray3D::safe_inv_vec(jittered))
                    } else {
                        (sun_dir, inv_sun)
                    };
                    if bvh.trace_any_shadow_cached(
                        origin,
                        s_dir,
                        s_inv,
                        0.05,
                        140.0,
                        &mut shadow_cache,
                    ) {
                        shadow = 0.22;
                    }
                }
                let mut refl: Vec3 = fctx.sun.get_sky_horizon();
                if tri.do_refl {
                    let view_in: Vec3 = ray_vec.normalize();
                    let refl_dir: Vec3 = view_in.reflect(tri.shade_n).normalize();
                    if depth > 64.0 {
                        refl = Self::sample_sky(refl_dir, &fctx.sun);
                    } else {
                        let inv_refl: Vec3 = Ray3D::safe_inv_vec(refl_dir);
                        rays += 1;
                        refl = match bvh.trace_closest_raw(origin, refl_dir, inv_refl, 0.08, 100.0)
                        {
                            Some((hit_t, hit_idx)) => {
                                let mut hit_rad: Vec3 =
                                    Self::shade_bvh_hit(bvh, hit_idx, &fctx.sun, use_soft_shadows);
                                if multi_bounce_refl && hit_t < 30.0 && depth < 32.0 {
                                    let hit_tri: &BvhTriangle = &bvh.get_triangles()[hit_idx];
                                    let hit_mat: PbrMaterial = hit_tri.get_material();
                                    if hit_mat.get_reflectivity() > 0.55 {
                                        let p2: Vec3 = origin
                                            .add(refl_dir.scale(hit_t))
                                            .add(hit_tri.get_normal().scale(0.08));
                                        let r2: Vec3 =
                                            refl_dir.reflect(hit_tri.get_normal()).normalize();
                                        let inv_r2: Vec3 = Ray3D::safe_inv_vec(r2);
                                        rays += 1;
                                        let bounce2: Vec3 = match bvh
                                            .trace_closest_raw(p2, r2, inv_r2, 0.08, 56.0)
                                        {
                                            Some((_, idx2)) => {
                                                Self::shade_bvh_hit(bvh, idx2, &fctx.sun, false)
                                            }
                                            None => Self::sample_sky(r2, &fctx.sun),
                                        };
                                        hit_rad = hit_rad.lerp(
                                            bounce2,
                                            (hit_mat.get_reflectivity() * 0.45).clamp(0.0, 0.5),
                                        );
                                    }
                                }
                                hit_rad
                            }
                            None => Self::sample_sky(refl_dir, &fctx.sun),
                        };
                    }
                }
                let mut gi_col: Vec3 = Vec3::zero();
                let mut ao: f32 = 1.0;
                if use_soft_shadows {
                    if depth > 52.0 {
                        gi_col = fctx.sun.get_sky_horizon().scale(0.18);
                        ao = 0.88;
                    } else {
                        let mut jit: Vec3 = gi_jitter[pattern];
                        if tri.shade_n.dot(jit) < -0.15 {
                            jit = jit.scale(-1.0);
                        }
                        let gi_dir: Vec3 = tri.shade_n.add(jit).normalize();
                        let inv_gi: Vec3 = Ray3D::safe_inv_vec(gi_dir);
                        rays += 1;
                        match bvh.trace_closest_raw(origin, gi_dir, inv_gi, 0.06, 20.0) {
                            Some((dist, hit_idx)) => {
                                let bounce: Vec3 =
                                    Self::shade_bvh_hit(bvh, hit_idx, &fctx.sun, true);
                                let falloff: f32 = (1.0 - dist / 20.0).clamp(0.14, 1.0);
                                gi_col = bounce.scale(0.36 * falloff);
                                ao = 0.42 + 0.58 * (dist / 14.0).clamp(0.0, 1.0);
                            }
                            None => {
                                gi_col = Self::sample_sky(gi_dir, &fctx.sun).scale(0.20);
                                ao = 1.0;
                            }
                        }
                    }
                }
                out_probes[probe_row + gx] = RtProbeSample {
                    shadow,
                    refl_r: refl.get_x(),
                    refl_g: refl.get_y(),
                    refl_b: refl.get_z(),
                    gi_r: gi_col.get_x(),
                    gi_g: gi_col.get_y(),
                    gi_b: gi_col.get_z(),
                    ao,
                    depth,
                    nx: tri.shade_n.get_x(),
                    ny: tri.shade_n.get_y(),
                    nz: tri.shade_n.get_z(),
                    valid: 1,
                };
            }
        }
        rays
    }

    /// Runs a 3x3 depth- and normal-aware joint bilateral denoiser on a horizontal band of coarse Lumen GI probes.
    ///
    /// # Arguments
    ///
    /// - `usize` - Coarse probe grid width.
    /// - `usize` - Coarse probe grid height.
    /// - `&[RtProbeSample]` - Full noisy input probe grid.
    /// - `usize` - Inclusive starting probe row `gy_start`.
    /// - `&mut [RtProbeSample]` - Disjoint denoised output band slice.
    fn denoise_gi_probes_band(
        gw: usize,
        gh: usize,
        src: &[RtProbeSample],
        gy_start: usize,
        dst_band: &mut [RtProbeSample],
    ) {
        let band_rows: usize = dst_band.len() / gw.max(1);
        for local_gy in 0..band_rows {
            let gy: usize = gy_start + local_gy;
            let y0: usize = gy.saturating_sub(1);
            let y1: usize = (gy + 1).min(gh - 1);
            let dst_row: usize = local_gy * gw;
            for gx in 0..gw {
                let idx: usize = gy * gw + gx;
                let center: RtProbeSample = src[idx];
                if center.valid == 0 {
                    dst_band[dst_row + gx] = center;
                    continue;
                }
                let x0: usize = gx.saturating_sub(1);
                let x1: usize = (gx + 1).min(gw - 1);
                let depth_tol: f32 = center.depth * 0.22 + 0.75;
                let mut w_sum: f32 = 0.0;
                let mut s_sum: f32 = 0.0;
                let mut rr_sum: f32 = 0.0;
                let mut rg_sum: f32 = 0.0;
                let mut rb_sum: f32 = 0.0;
                let mut gi_r: f32 = 0.0;
                let mut gi_g: f32 = 0.0;
                let mut gi_b: f32 = 0.0;
                let mut ao_sum: f32 = 0.0;
                for ny in y0..=y1 {
                    let row_off: usize = ny * gw;
                    for nx in x0..=x1 {
                        let nb: RtProbeSample = src[row_off + nx];
                        if nb.valid == 0 {
                            continue;
                        }
                        let ndot: f32 = center.nx * nb.nx + center.ny * nb.ny + center.nz * nb.nz;
                        if ndot < 0.75 {
                            continue;
                        }
                        let d_diff: f32 = (center.depth - nb.depth).abs();
                        if d_diff > depth_tol {
                            continue;
                        }
                        let spatial: f32 = if nx == gx && ny == gy {
                            1.8
                        } else if nx == gx || ny == gy {
                            1.0
                        } else {
                            0.6
                        };
                        let w: f32 = spatial * (1.0 - d_diff / depth_tol);
                        w_sum += w;
                        s_sum += nb.shadow * w;
                        rr_sum += nb.refl_r * w;
                        rg_sum += nb.refl_g * w;
                        rb_sum += nb.refl_b * w;
                        gi_r += nb.gi_r * w;
                        gi_g += nb.gi_g * w;
                        gi_b += nb.gi_b * w;
                        ao_sum += nb.ao * w;
                    }
                }
                let mut out: RtProbeSample = center;
                if w_sum > 1e-4 {
                    let inv_w: f32 = 1.0 / w_sum;
                    out.shadow = s_sum * inv_w;
                    out.refl_r = rr_sum * inv_w;
                    out.refl_g = rg_sum * inv_w;
                    out.refl_b = rb_sum * inv_w;
                    out.gi_r = gi_r * inv_w;
                    out.gi_g = gi_g * inv_w;
                    out.gi_b = gi_b * inv_w;
                    out.ao = ao_sum * inv_w;
                }
                dst_band[dst_row + gx] = out;
            }
        }
    }

    /// Runs a fast 5-tap separable Gaussian blur over the quarter-resolution HDR bloom buffer.
    ///
    /// # Arguments
    ///
    /// - `usize` - Quarter-res bloom width.
    /// - `usize` - Quarter-res bloom height.
    /// - `&mut [[f32; 3]]` - Input bloom buffer (receives final blurred output).
    /// - `&mut [[f32; 3]]` - Intermediate horizontal scratch buffer.
    fn blur_bloom_separable(bw: usize, bh: usize, buf_a: &mut [[f32; 3]], buf_b: &mut [[f32; 3]]) {
        if bw < 3 || bh < 3 {
            return;
        }
        let min_len: usize = bw * bh;
        if buf_a.len() < min_len || buf_b.len() < min_len {
            return;
        }
        let max_x: usize = bw - 1;
        let max_y: usize = bh - 1;
        for y in 0..bh {
            let row: usize = y * bw;
            for x in 0..bw {
                let x_m2: usize = x.saturating_sub(2);
                let x_m1: usize = x.saturating_sub(1);
                let x_p1: usize = (x + 1).min(max_x);
                let x_p2: usize = (x + 2).min(max_x);
                let c0: [f32; 3] = buf_a[row + x_m2];
                let c1: [f32; 3] = buf_a[row + x_m1];
                let c2: [f32; 3] = buf_a[row + x];
                let c3: [f32; 3] = buf_a[row + x_p1];
                let c4: [f32; 3] = buf_a[row + x_p2];
                buf_b[row + x] = [
                    c0[0] * 0.08 + c1[0] * 0.22 + c2[0] * 0.40 + c3[0] * 0.22 + c4[0] * 0.08,
                    c0[1] * 0.08 + c1[1] * 0.22 + c2[1] * 0.40 + c3[1] * 0.22 + c4[1] * 0.08,
                    c0[2] * 0.08 + c1[2] * 0.22 + c2[2] * 0.40 + c3[2] * 0.22 + c4[2] * 0.08,
                ];
            }
        }
        for y in 0..bh {
            let r_m2: usize = y.saturating_sub(2) * bw;
            let r_m1: usize = y.saturating_sub(1) * bw;
            let r_0: usize = y * bw;
            let r_p1: usize = (y + 1).min(max_y) * bw;
            let r_p2: usize = (y + 2).min(max_y) * bw;
            for x in 0..bw {
                let c0: [f32; 3] = buf_b[r_m2 + x];
                let c1: [f32; 3] = buf_b[r_m1 + x];
                let c2: [f32; 3] = buf_b[r_0 + x];
                let c3: [f32; 3] = buf_b[r_p1 + x];
                let c4: [f32; 3] = buf_b[r_p2 + x];
                buf_a[r_0 + x] = [
                    c0[0] * 0.08 + c1[0] * 0.22 + c2[0] * 0.40 + c3[0] * 0.22 + c4[0] * 0.08,
                    c0[1] * 0.08 + c1[1] * 0.22 + c2[1] * 0.40 + c3[1] * 0.22 + c4[1] * 0.08,
                    c0[2] * 0.08 + c1[2] * 0.22 + c2[2] * 0.40 + c3[2] * 0.22 + c4[2] * 0.08,
                ];
            }
        }
    }

    /// Shades a horizontal pixel band with Cook-Torrance GGX PBR, analytically anti-aliased facades/roads, bilateral Lumen GI, volumetric fog, and HDR bloom extraction.
    ///
    /// # Arguments
    ///
    /// - `&DeferredFrameCtx` - Shared per-frame camera, sun, and RT settings.
    /// - `(&[f32], &[u32], &[GShadeTri], &[RtProbeSample], &Bvh3D)` - Read-only scene, G-Buffer, and probe slices.
    /// - `(usize, &mut [u8], &mut [u8], &mut [[f32; 3]])` - `(py_start, color band, luma band, bloom band)`.
    ///
    /// # Returns
    ///
    /// - `usize` - Edge-refinement BVH rays traced within this band.
    fn shade_deferred_band(
        fctx: &DeferredFrameCtx,
        inputs: (&[f32], &[u32], &[GShadeTri], &[RtProbeSample], &Bvh3D),
        outputs: (usize, &mut [u8], &mut [u8], &mut [[f32; 3]]),
    ) -> usize {
        let (depth_buf, g_tri_idx, shaded_tris, probes, bvh): (
            &[f32],
            &[u32],
            &[GShadeTri],
            &[RtProbeSample],
            &Bvh3D,
        ) = inputs;
        let (py_start, color_band, luma_band, bloom_band): (
            usize,
            &mut [u8],
            &mut [u8],
            &mut [[f32; 3]],
        ) = outputs;
        let w: usize = fctx.w;
        let gw: usize = fctx.gw;
        let gh: usize = fctx.gh;
        let bw: usize = fctx.bw;
        let step_f: f32 = (fctx.rt_step.max(1)) as f32;
        let inv_step: f32 = 1.0 / step_f;
        let sun_dir: Vec3 = fctx.sun.get_direction();
        let inv_sun: Vec3 = fctx.inv_sun_dir;
        let sun_col: Vec3 = fctx.sun.get_color();
        let sky_horizon: Vec3 = fctx.sun.get_sky_horizon();
        let sky_zenith: Vec3 = fctx.sun.get_sky_zenith();
        let rt_active: bool = fctx.rt_mode != RayTracingMode::Off && !probes.is_empty();
        let gi_active: bool = fctx.rt_mode == RayTracingMode::FullPathTracedGi && rt_active;
        let band_rows: usize = luma_band.len() / w.max(1);
        let ray_dx: Vec3 = fctx.cam_right_scaled.scale(2.0 * fctx.inv_w_f);
        let ndc_x0: f32 = fctx.inv_w_f - 1.0;
        let mut edge_rays: usize = 0;
        let mut shadow_cache: u32 = u32::MAX;
        for local_y in 0..band_rows {
            let py: usize = py_start + local_y;
            let row_off: usize = py * w;
            let out_row_luma: usize = local_y * w;
            let out_row_rgba: usize = out_row_luma * 4;
            let write_bloom_row: bool = fctx.bloom_on && (py & 3) == 0 && bw > 0;
            let bloom_row_off: usize = if write_bloom_row {
                (local_y / 4) * bw
            } else {
                0
            };
            let ndc_y: f32 = 1.0 - 2.0 * (((py as f32) + 0.5) * fctx.inv_h_f);
            let row_base_dir: Vec3 = fctx.cam_fwd.add(fctx.cam_up_scaled.scale(ndc_y));
            let base_up: f32 = row_base_dir.get_y().clamp(0.0, 1.0);
            let base_sky: Vec3 = sky_horizon
                .lerp(sky_zenith, base_up.sqrt())
                .add(sky_horizon.scale((1.0 - base_up) * (1.0 - base_up) * 0.14));
            let base_sky_rgba: [u8; 4] = base_sky.scale(fctx.exposure_scale).to_aces_rgba8();
            let base_sky_luma: u8 = (((base_sky_rgba[0] as u32) * 77
                + (base_sky_rgba[1] as u32) * 150
                + (base_sky_rgba[2] as u32) * 29)
                >> 8) as u8;
            let row_has_sun: bool = row_base_dir.normalize().dot(sun_dir) >= 0.45;
            let gy_f: f32 =
                (((py as f32) + 0.5) * inv_step - 0.5).clamp(0.0, (gh.saturating_sub(1)) as f32);
            let gy0: usize = (gy_f.floor() as usize).min(gh.saturating_sub(1));
            let gy1: usize = (gy0 + 1).min(gh.saturating_sub(1));
            let fy: f32 = gy_f - (gy0 as f32);
            let wy0: f32 = 1.0 - fy;
            let wy1: f32 = fy;
            let p_row0: usize = gy0 * gw;
            let p_row1: usize = gy1 * gw;
            let mut raw_ray: Vec3 = row_base_dir.add(fctx.cam_right_scaled.scale(ndc_x0));
            for px in 0..w {
                let pix_idx: usize = row_off + px;
                let tid: u32 = g_tri_idx[pix_idx];
                let cidx: usize = out_row_rgba + px * 4;
                if tid == u32::MAX {
                    let (rgba, luma, sky_hdr): ([u8; 4], u8, Vec3) = if row_has_sun {
                        let dir: Vec3 = raw_ray.normalize();
                        let sdot: f32 = dir.dot(sun_dir).max(0.0);
                        if sdot > 0.72 {
                            let mut col: Vec3 = base_sky;
                            if sdot > 0.992 {
                                let disk: f32 = ((sdot - 0.992) / 0.008).clamp(0.0, 1.0);
                                col = col.add(sun_col.scale(disk * 3.8 + 0.55));
                            } else {
                                let h1: f32 = ((sdot - 0.72) / 0.28).clamp(0.0, 1.0);
                                let h2: f32 = h1 * h1;
                                col = col.add(sun_col.scale(h2 * h2 * 0.58 + h2 * 0.08));
                            }
                            let exposed: Vec3 = col.scale(fctx.exposure_scale);
                            let px_rgba: [u8; 4] = exposed.to_aces_rgba8();
                            let px_luma: u8 = (((px_rgba[0] as u32) * 77
                                + (px_rgba[1] as u32) * 150
                                + (px_rgba[2] as u32) * 29)
                                >> 8) as u8;
                            (px_rgba, px_luma, exposed)
                        } else {
                            (base_sky_rgba, base_sky_luma, base_sky)
                        }
                    } else {
                        (base_sky_rgba, base_sky_luma, base_sky)
                    };
                    if write_bloom_row && (px & 3) == 0 {
                        let bidx: usize = bloom_row_off + (px >> 2);
                        if bidx < bloom_band.len() {
                            let er: f32 = (sky_hdr.get_x() - 1.15).max(0.0);
                            let eg: f32 = (sky_hdr.get_y() - 1.15).max(0.0);
                            let eb: f32 = (sky_hdr.get_z() - 1.15).max(0.0);
                            bloom_band[bidx] = [er, eg, eb];
                        }
                    }
                    color_band[cidx] = rgba[0];
                    color_band[cidx + 1] = rgba[1];
                    color_band[cidx + 2] = rgba[2];
                    color_band[cidx + 3] = 255;
                    luma_band[out_row_luma + px] = luma;
                    raw_ray = raw_ray.add(ray_dx);
                    continue;
                }
                let tri: &GShadeTri = &shaded_tris[tid as usize];
                let depth: f32 = 1.0 / depth_buf[pix_idx].max(1e-6);
                let world_p_px: Vec3 = fctx.cam_pos.add(raw_ray.scale(depth));
                let mut shadow: f32 = 1.0;
                let mut refl: Vec3 = sky_horizon;
                let mut gi_col: Vec3 = Vec3::zero();
                let mut ao: f32 = 1.0;
                if rt_active {
                    let gx_f: f32 = (((px as f32) + 0.5) * inv_step - 0.5)
                        .clamp(0.0, (gw.saturating_sub(1)) as f32);
                    let gx0: usize = (gx_f.floor() as usize).min(gw.saturating_sub(1));
                    let gx1: usize = (gx0 + 1).min(gw.saturating_sub(1));
                    let fx: f32 = gx_f - (gx0 as f32);
                    let wx0: f32 = 1.0 - fx;
                    let wx1: f32 = fx;
                    let taps: [(RtProbeSample, f32); 4] = [
                        (probes[p_row0 + gx0], wx0 * wy0),
                        (probes[p_row0 + gx1], wx1 * wy0),
                        (probes[p_row1 + gx0], wx0 * wy1),
                        (probes[p_row1 + gx1], wx1 * wy1),
                    ];
                    let snx: f32 = tri.shade_n.get_x();
                    let sny: f32 = tri.shade_n.get_y();
                    let snz: f32 = tri.shade_n.get_z();
                    let mut w_sum: f32 = 0.0;
                    let mut s_sum: f32 = 0.0;
                    let mut s_min: f32 = 2.0;
                    let mut s_max: f32 = -1.0;
                    let mut rr_sum: f32 = 0.0;
                    let mut rg_sum: f32 = 0.0;
                    let mut rb_sum: f32 = 0.0;
                    let mut gr_sum: f32 = 0.0;
                    let mut gg_sum: f32 = 0.0;
                    let mut gb_sum: f32 = 0.0;
                    let mut ao_sum: f32 = 0.0;
                    let mut valid_matches: u32 = 0;
                    for (pr, bw_tap) in taps.into_iter() {
                        if pr.valid == 0 {
                            continue;
                        }
                        let ndot: f32 = snx * pr.nx + sny * pr.ny + snz * pr.nz;
                        if ndot < 0.78 {
                            continue;
                        }
                        let depth_tol: f32 = if ndot > 0.95 {
                            depth * 0.58 + 2.4
                        } else {
                            depth * 0.20 + 0.75
                        };
                        let d_diff: f32 = (depth - pr.depth).abs();
                        if d_diff > depth_tol {
                            continue;
                        }
                        let wt: f32 = bw_tap * (1.0 - d_diff / depth_tol) + 1e-4;
                        w_sum += wt;
                        s_sum += pr.shadow * wt;
                        s_min = s_min.min(pr.shadow);
                        s_max = s_max.max(pr.shadow);
                        rr_sum += pr.refl_r * wt;
                        rg_sum += pr.refl_g * wt;
                        rb_sum += pr.refl_b * wt;
                        gr_sum += pr.gi_r * wt;
                        gg_sum += pr.gi_g * wt;
                        gb_sum += pr.gi_b * wt;
                        ao_sum += pr.ao * wt;
                        valid_matches += 1;
                    }
                    let inv_ws: f32 = if w_sum > 1e-5 { 1.0 / w_sum } else { 0.0 };
                    let need_exact_shadow: bool = tri.n_dot_l > 0.02
                        && depth < 42.0
                        && (valid_matches == 0 || (!gi_active && (s_max - s_min) > 0.12));
                    if need_exact_shadow {
                        let origin: Vec3 = world_p_px.add(tri.shade_n.scale(0.08));
                        edge_rays += 1;
                        shadow = if bvh.trace_any_shadow_cached(
                            origin,
                            sun_dir,
                            inv_sun,
                            0.05,
                            140.0,
                            &mut shadow_cache,
                        ) {
                            0.22
                        } else {
                            1.0
                        };
                    } else if valid_matches > 0 {
                        shadow = s_sum * inv_ws;
                    }
                    if tri.do_refl {
                        if valid_matches > 0 {
                            refl = Vec3::new(rr_sum * inv_ws, rg_sum * inv_ws, rb_sum * inv_ws);
                        } else if depth < 32.0 {
                            let origin: Vec3 = world_p_px.add(tri.shade_n.scale(0.08));
                            let refl_dir: Vec3 =
                                raw_ray.normalize().reflect(tri.shade_n).normalize();
                            let inv_refl: Vec3 = Ray3D::safe_inv_vec(refl_dir);
                            edge_rays += 1;
                            refl = match bvh
                                .trace_closest_raw(origin, refl_dir, inv_refl, 0.08, 96.0)
                            {
                                Some((_, hit_idx)) => {
                                    Self::shade_bvh_hit(bvh, hit_idx, &fctx.sun, gi_active)
                                }
                                None => Self::sample_sky(refl_dir, &fctx.sun),
                            };
                        } else {
                            let refl_dir: Vec3 =
                                raw_ray.normalize().reflect(tri.shade_n).normalize();
                            refl = Self::sample_sky(refl_dir, &fctx.sun);
                        }
                    }
                    if gi_active && valid_matches > 0 {
                        gi_col = Vec3::new(gr_sum * inv_ws, gg_sum * inv_ws, gb_sum * inv_ws);
                        ao = ao_sum * inv_ws;
                    }
                }
                let view_in: Vec3 = raw_ray.normalize();
                let view_to_cam: Vec3 = view_in.scale(-1.0);
                let ndv: f32 = tri.shade_n.dot(view_to_cam).clamp(0.02, 1.0);
                let half_vec: Vec3 = sun_dir.add(view_to_cam).normalize();
                let ndh: f32 = tri.shade_n.dot(half_vec).max(0.0);
                let ggx_denom: f32 = ndh * ndh * (tri.alpha2 - 1.0) + 1.0;
                let ggx_d: f32 = tri.alpha2 / (PI * ggx_denom * ggx_denom + 1e-4);
                let f_om: f32 = 1.0 - ndv;
                let f_om2: f32 = f_om * f_om;
                let fresnel_t: f32 = f_om2 * f_om2 * f_om;
                let fresnel_rgb: Vec3 = tri.f0.add(Vec3::splat(1.0).sub(tri.f0).scale(fresnel_t));
                let vis_term: f32 = 0.25 / (ndv * tri.n_dot_l.max(0.06) + 0.08);
                let spec_scalar: f32 = (ggx_d * vis_term * tri.n_dot_l * shadow).min(4.5);
                let spec_col: Vec3 = fresnel_rgb.mul(sun_col.scale(spec_scalar));
                let mut surf_base: Vec3 = tri.mat_base;
                let mut surf_emissive: Vec3 = tri.mat_emissive;
                let mut surf_refl: f32 = tri.mat_refl;
                let world_px_size: f32 = depth * fctx.inv_h_f * 1.85;
                match tri.surface_detail {
                    SurfaceDetailMode::ArchitecturalFacade
                        if tri.shade_n.get_y().abs() < 0.25
                            && world_p_px.get_y() > 2.4
                            && surf_emissive.length_sq() < 0.02 =>
                    {
                        let u_coord: f32 = if tri.shade_n.get_x().abs() > 0.5 {
                            world_p_px.get_z()
                        } else {
                            world_p_px.get_x()
                        };
                        let fx_win: f32 = (u_coord * 0.45).rem_euclid(1.0);
                        let fy_win: f32 = (world_p_px.get_y() * 0.32).rem_euclid(1.0);
                        let aa_u: f32 = (world_px_size * 0.45).clamp(0.02, 0.34);
                        let aa_v: f32 = (world_px_size * 0.32).clamp(0.02, 0.34);
                        let pane_u: f32 = smoothstep_f32(0.16 - aa_u, 0.16 + aa_u, fx_win)
                            * (1.0 - smoothstep_f32(0.84 - aa_u, 0.84 + aa_u, fx_win));
                        let pane_v: f32 = smoothstep_f32(0.20 - aa_v, 0.20 + aa_v, fy_win)
                            * (1.0 - smoothstep_f32(0.82 - aa_v, 0.82 + aa_v, fy_win));
                        let dist_fade: f32 = 1.0 - smoothstep_f32(48.0, 125.0, depth);
                        let pane_w: f32 = (pane_u * pane_v * dist_fade).clamp(0.0, 1.0);
                        let wall_base: Vec3 = surf_base.scale(0.84);
                        if pane_w > 0.01 {
                            let cell_x: i32 = (u_coord * 0.45).floor() as i32;
                            let cell_y: i32 = (world_p_px.get_y() * 0.32).floor() as i32;
                            let h_bits: u32 = ((cell_x.wrapping_mul(73856093))
                                ^ (cell_y.wrapping_mul(19349663)))
                                as u32;
                            let lit_bucket: u32 = h_bits % 11;
                            if lit_bucket == 0 {
                                surf_base = wall_base.lerp(surf_base, pane_w);
                                surf_emissive =
                                    surf_emissive.add(Vec3::new(1.15, 0.92, 0.48).scale(pane_w));
                            } else if lit_bucket == 1 {
                                surf_base = wall_base.lerp(surf_base, pane_w);
                                surf_emissive =
                                    surf_emissive.add(Vec3::new(0.32, 0.85, 1.25).scale(pane_w));
                            } else {
                                surf_base = wall_base.lerp(surf_base.scale(0.62), pane_w);
                                surf_refl = (surf_refl + 0.22 * pane_w).min(0.95);
                            }
                        } else {
                            surf_base = wall_base;
                        }
                    }
                    SurfaceDetailMode::RoadSurface
                        if tri.shade_n.get_y() > 0.92 && world_p_px.get_y().abs() < 0.15 =>
                    {
                        let abs_x: f32 = world_p_px.get_x().abs();
                        let edge_aa: f32 = (world_px_size * 1.4).clamp(0.05, 0.42);
                        let curb_w: f32 = smoothstep_f32(9.9 - edge_aa, 9.9 + edge_aa, abs_x);
                        let walk_w: f32 = smoothstep_f32(10.6 - edge_aa, 10.6 + edge_aa, abs_x);
                        let micro_fade: f32 = 1.0 - smoothstep_f32(18.0, 58.0, depth);
                        let micro: f32 = (((world_p_px.get_x() * 3.1).floor() as i32
                            ^ (world_p_px.get_z() * 3.1).floor() as i32)
                            & 3) as f32;
                        let road_col: Vec3 =
                            surf_base.scale(1.0 + (micro * 0.03 - 0.045) * micro_fade);
                        let curb_col: Vec3 = Vec3::new(0.55, 0.57, 0.60);
                        let walk_col: Vec3 = Vec3::new(0.26, 0.28, 0.32);
                        surf_base = road_col.lerp(curb_col, curb_w).lerp(walk_col, walk_w);
                        surf_refl = surf_refl * (1.0 - curb_w)
                            + 0.18 * curb_w * (1.0 - walk_w)
                            + 0.12 * walk_w;
                    }
                    _ => {}
                }
                let kd_scale: f32 = (tri.diffuse_scale * (1.0 - fresnel_t * 0.55)).max(0.08);
                let direct: Vec3 = sun_col.scale(tri.n_dot_l * shadow * kd_scale);
                let indirect: Vec3 = tri.ambient_sky.scale(0.44 * ao).add(gi_col);
                let mut radiance: Vec3 = surf_base
                    .mul(direct.add(indirect))
                    .add(spec_col)
                    .add(surf_emissive);
                if tri.do_refl {
                    let refl_fresnel: f32 =
                        (0.18 + 0.82 * fresnel_t + tri.metallic * 0.25).clamp(0.18, 0.95);
                    let refl_blend: f32 = (surf_refl * refl_fresnel).clamp(0.0, 0.82);
                    radiance = radiance.lerp(refl, refl_blend);
                }
                if fctx.fog_on {
                    let height_atten: f32 =
                        (1.0 / (1.0 + world_p_px.get_y().max(0.0) * 0.045)).clamp(0.32, 1.0);
                    let fd: f32 = depth * fctx.inv_fog_dist * height_atten;
                    let fog_t: f32 = (fd / (1.0 + fd)).clamp(0.0, 0.85);
                    let sun_align: f32 = view_in.dot(sun_dir).max(0.0);
                    let sa2: f32 = sun_align * sun_align;
                    let shaft: f32 = sa2 * sa2 * fog_t * (0.35 + 0.65 * shadow);
                    let fog_col: Vec3 = sky_horizon.add(sun_col.scale(shaft * 0.55));
                    radiance = radiance.lerp(fog_col, fog_t);
                }
                let exposed: Vec3 = radiance.scale(fctx.exposure_scale);
                if write_bloom_row && (px & 3) == 0 {
                    let bidx: usize = bloom_row_off + (px >> 2);
                    if bidx < bloom_band.len() {
                        bloom_band[bidx] = [
                            (exposed.get_x() - 1.05).max(0.0),
                            (exposed.get_y() - 1.05).max(0.0),
                            (exposed.get_z() - 1.05).max(0.0),
                        ];
                    }
                }
                let rgba: [u8; 4] = exposed.to_aces_rgba8();
                color_band[cidx] = rgba[0];
                color_band[cidx + 1] = rgba[1];
                color_band[cidx + 2] = rgba[2];
                color_band[cidx + 3] = 255;
                luma_band[out_row_luma + px] =
                    (((rgba[0] as u32) * 77 + (rgba[1] as u32) * 150 + (rgba[2] as u32) * 29) >> 8)
                        as u8;
                raw_ray = raw_ray.add(ray_dx);
            }
        }
        edge_rays
    }

    /// Samples the quarter-resolution HDR bloom buffer with smooth bilinear filtering to eliminate 4x4 blocky bloom halos.
    ///
    /// # Arguments
    ///
    /// - `&[[f32; 3]]` - Blurred quarter-resolution HDR bloom slice.
    /// - `usize` - Bloom grid width `bw`.
    /// - `usize` - Bloom grid height `bh`.
    /// - `usize` - Full-resolution pixel X.
    /// - `usize` - Full-resolution pixel Y.
    ///
    /// # Returns
    ///
    /// - `[f32; 3]` - Bilinearly filtered bloom RGB radiance.
    #[must_use]
    fn sample_bloom_bilinear(
        bloom: &[[f32; 3]],
        bw: usize,
        bh: usize,
        x: usize,
        y: usize,
    ) -> [f32; 3] {
        let fx: f32 = ((x as f32) * 0.25).clamp(0.0, (bw.saturating_sub(1)) as f32);
        let fy: f32 = ((y as f32) * 0.25).clamp(0.0, (bh.saturating_sub(1)) as f32);
        let x0: usize = (fx as usize).min(bw.saturating_sub(1));
        let y0: usize = (fy as usize).min(bh.saturating_sub(1));
        let x1: usize = (x0 + 1).min(bw.saturating_sub(1));
        let y1: usize = (y0 + 1).min(bh.saturating_sub(1));
        let tx: f32 = fx - (x0 as f32);
        let ty: f32 = fy - (y0 as f32);
        let c00: [f32; 3] = bloom[y0 * bw + x0];
        let c10: [f32; 3] = bloom[y0 * bw + x1];
        let c01: [f32; 3] = bloom[y1 * bw + x0];
        let c11: [f32; 3] = bloom[y1 * bw + x1];
        let w00: f32 = (1.0 - tx) * (1.0 - ty);
        let w10: f32 = tx * (1.0 - ty);
        let w01: f32 = (1.0 - tx) * ty;
        let w11: f32 = tx * ty;
        [
            c00[0] * w00 + c10[0] * w10 + c01[0] * w01 + c11[0] * w11,
            c00[1] * w00 + c10[1] * w10 + c01[1] * w01 + c11[1] * w11,
            c00[2] * w00 + c10[2] * w10 + c01[2] * w01 + c11[2] * w11,
        ]
    }

    /// Fused post-process pass applying Geometry-Aware FXAA 3.11 + 8-step Morphological Silhouette Edge-Walking AA, 9-tap variance-clamped TAA, and smooth bilinear HDR Bloom.
    ///
    /// # Arguments
    ///
    /// - `(usize, usize, usize, usize)` - Dimensions `(w, h, bloom_w, bloom_h)`.
    /// - `PostProcessInputs<'_>` - Read-only `(color_src, luma, g_tri_idx, bloom, taa_history)`.
    /// - `(bool, bool, bool)` - Flags `(use_fxaa, use_taa, use_bloom)`.
    /// - `(usize, &mut [u8])` - `(y_start, mutable RGBA output band)`.
    fn post_process_band(
        dims: (usize, usize, usize, usize),
        inputs: PostProcessInputs<'_>,
        flags: (bool, bool, bool),
        band: (usize, &mut [u8]),
    ) {
        let (w, h, bw, bh): (usize, usize, usize, usize) = dims;
        let (src, luma, g_tri_idx, bloom, history): PostProcessInputs<'_> = inputs;
        let (use_fxaa, use_taa, use_bloom): (bool, bool, bool) = flags;
        let (y_start, dst_band): (usize, &mut [u8]) = band;
        let row_bytes: usize = w * 4;
        let band_rows: usize = dst_band.len() / row_bytes.max(1);
        let has_bloom: bool = use_bloom && bw > 0 && bh > 0 && bloom.len() >= bw * bh;
        let has_taa: bool = use_taa && history.len() == src.len();
        let w_i32: i32 = w as i32;
        let h_i32: i32 = h as i32;
        for local_y in 0..band_rows {
            let y: usize = y_start + local_y;
            let src_off: usize = y * row_bytes;
            let dst_off: usize = local_y * row_bytes;
            if !use_fxaa && !has_bloom && !has_taa {
                dst_band[dst_off..(dst_off + row_bytes)]
                    .copy_from_slice(&src[src_off..(src_off + row_bytes)]);
                continue;
            }
            if !use_fxaa || y == 0 || y + 1 >= h || w < 3 {
                for x in 0..w {
                    let sc: usize = src_off + x * 4;
                    let dc: usize = dst_off + x * 4;
                    let mut r: i32 = src[sc] as i32;
                    let mut g: i32 = src[sc + 1] as i32;
                    let mut b: i32 = src[sc + 2] as i32;
                    if has_bloom {
                        let bl: [f32; 3] = Self::sample_bloom_bilinear(bloom, bw, bh, x, y);
                        r = (r + (bl[0] * 64.0) as i32).min(255);
                        g = (g + (bl[1] * 64.0) as i32).min(255);
                        b = (b + (bl[2] * 64.0) as i32).min(255);
                    }
                    dst_band[dc] = r as u8;
                    dst_band[dc + 1] = g as u8;
                    dst_band[dc + 2] = b as u8;
                    dst_band[dc + 3] = 255;
                }
                continue;
            }
            dst_band[dst_off..(dst_off + 4)].copy_from_slice(&src[src_off..(src_off + 4)]);
            let last_px: usize = (w - 1) * 4;
            dst_band[(dst_off + last_px)..(dst_off + last_px + 4)]
                .copy_from_slice(&src[(src_off + last_px)..(src_off + last_px + 4)]);
            let row: usize = y * w;
            let row_up: usize = (y - 1) * w;
            let row_dn: usize = (y + 1) * w;
            let y_i32: i32 = y as i32;
            for x in 1..(w - 1) {
                let lc: i32 = luma[row + x] as i32;
                let ln: i32 = luma[row_up + x] as i32;
                let ls: i32 = luma[row_dn + x] as i32;
                let le: i32 = luma[row + x + 1] as i32;
                let lw: i32 = luma[row + x - 1] as i32;
                let lmax: i32 = lc.max(ln).max(ls).max(le).max(lw);
                let lmin: i32 = lc.min(ln).min(ls).min(le).min(lw);
                let contrast: i32 = lmax - lmin;
                let tid_c: u32 = g_tri_idx[row + x];
                let geom_edge: bool = tid_c != g_tri_idx[row_up + x]
                    || tid_c != g_tri_idx[row_dn + x]
                    || tid_c != g_tri_idx[row + x - 1]
                    || tid_c != g_tri_idx[row + x + 1];
                let sc: usize = (row + x) * 4;
                let dc: usize = dst_off + x * 4;
                let mut rgb: [i32; 3] = [src[sc] as i32, src[sc + 1] as i32, src[sc + 2] as i32];
                if contrast >= 7 || (geom_edge && contrast >= 3) {
                    let lnw: i32 = luma[row_up + x - 1] as i32;
                    let lne: i32 = luma[row_up + x + 1] as i32;
                    let lsw: i32 = luma[row_dn + x - 1] as i32;
                    let lse: i32 = luma[row_dn + x + 1] as i32;
                    let edge_h: i32 = (lnw + lne - 2 * ln).abs()
                        + 2 * (lw + le - 2 * lc).abs()
                        + (lsw + lse - 2 * ls).abs();
                    let edge_v: i32 = (lnw + lsw - 2 * lw).abs()
                        + 2 * (ln + ls - 2 * lc).abs()
                        + (lne + lse - 2 * le).abs();
                    let is_horiz: bool = edge_h >= edge_v;
                    let (l_neg, l_pos): (i32, i32) = if is_horiz { (ln, ls) } else { (lw, le) };
                    let grad_neg: i32 = (l_neg - lc).abs();
                    let grad_pos: i32 = (l_pos - lc).abs();
                    let step_sign: i32 = if grad_neg >= grad_pos { -1 } else { 1 };
                    let grad_steep: i32 = grad_neg.max(grad_pos);
                    let l_other: i32 = if step_sign < 0 { l_neg } else { l_pos };
                    let edge_avg_2: i32 = lc + l_other;
                    let grad_thresh_2: i32 = (grad_steep >> 1).max(3);
                    let (perp_dx, perp_dy, tan_dx, tan_dy): (i32, i32, i32, i32) = if is_horiz {
                        (0, step_sign, 1, 0)
                    } else {
                        (step_sign, 0, 0, 1)
                    };
                    let x_i32: i32 = x as i32;
                    let mut dist_neg: i32 = MLAA_MAX_WALK_STEPS as i32;
                    let mut end_neg_delta: i32 = 0;
                    for s in 1..=(MLAA_MAX_WALK_STEPS as i32) {
                        let sx: i32 = x_i32 - tan_dx * s;
                        let sy: i32 = y_i32 - tan_dy * s;
                        let ox: i32 = sx + perp_dx;
                        let oy: i32 = sy + perp_dy;
                        if sx < 0
                            || sx >= w_i32
                            || sy < 0
                            || sy >= h_i32
                            || ox < 0
                            || ox >= w_i32
                            || oy < 0
                            || oy >= h_i32
                        {
                            dist_neg = s;
                            break;
                        }
                        let pair_sum: i32 = (luma[(sy as usize) * w + (sx as usize)] as i32)
                            + (luma[(oy as usize) * w + (ox as usize)] as i32);
                        let delta: i32 = pair_sum - edge_avg_2;
                        if delta.abs() >= grad_thresh_2 {
                            dist_neg = s;
                            end_neg_delta = delta;
                            break;
                        }
                    }
                    let mut dist_pos: i32 = MLAA_MAX_WALK_STEPS as i32;
                    let mut end_pos_delta: i32 = 0;
                    for s in 1..=(MLAA_MAX_WALK_STEPS as i32) {
                        let sx: i32 = x_i32 + tan_dx * s;
                        let sy: i32 = y_i32 + tan_dy * s;
                        let ox: i32 = sx + perp_dx;
                        let oy: i32 = sy + perp_dy;
                        if sx < 0
                            || sx >= w_i32
                            || sy < 0
                            || sy >= h_i32
                            || ox < 0
                            || ox >= w_i32
                            || oy < 0
                            || oy >= h_i32
                        {
                            dist_pos = s;
                            break;
                        }
                        let pair_sum: i32 = (luma[(sy as usize) * w + (sx as usize)] as i32)
                            + (luma[(oy as usize) * w + (ox as usize)] as i32);
                        let delta: i32 = pair_sum - edge_avg_2;
                        if delta.abs() >= grad_thresh_2 {
                            dist_pos = s;
                            end_pos_delta = delta;
                            break;
                        }
                    }
                    let center_lt_avg: bool = (lc * 2) < edge_avg_2;
                    let (closer_dist, closer_delta): (i32, i32) = if dist_neg <= dist_pos {
                        (dist_neg, end_neg_delta)
                    } else {
                        (dist_pos, end_pos_delta)
                    };
                    let good_span: bool = (closer_delta < 0) != center_lt_avg;
                    let span_total: f32 = ((dist_neg + dist_pos).max(1)) as f32;
                    let morph_blend: f32 = if good_span {
                        (0.5 - ((closer_dist as f32) - 0.5).max(0.0) / span_total).clamp(0.0, 0.5)
                    } else {
                        0.0
                    };
                    let avg_8: i32 = ((ln + ls + le + lw) * 2 + lnw + lne + lsw + lse + 6) / 12;
                    let subpix_raw: f32 =
                        (((avg_8 - lc).abs() as f32) / (contrast.max(1) as f32)).clamp(0.0, 1.0);
                    let subpix_s: f32 = subpix_raw * subpix_raw * (3.0 - 2.0 * subpix_raw);
                    let subpix_blend: f32 =
                        subpix_s * subpix_s * if geom_edge { 0.78 } else { 0.52 };
                    let final_blend: f32 = morph_blend.max(subpix_blend).clamp(0.0, 0.78);
                    let blend_q8: i32 = (final_blend * 256.0).round() as i32;
                    if blend_q8 > 0 {
                        let nx: usize = (x_i32 + perp_dx).clamp(0, w_i32 - 1) as usize;
                        let ny: usize = (y_i32 + perp_dy).clamp(0, h_i32 - 1) as usize;
                        let n_sc: usize = (ny * w + nx) * 4;
                        let keep_q8: i32 = 256 - blend_q8;
                        for ch in 0..3 {
                            rgb[ch] =
                                ((rgb[ch] * keep_q8 + (src[n_sc + ch] as i32) * blend_q8 + 128)
                                    >> 8)
                                    .clamp(0, 255);
                        }
                    }
                }
                if has_taa {
                    let offsets: [usize; 8] = [
                        (row_up + x - 1) * 4,
                        (row_up + x) * 4,
                        (row_up + x + 1) * 4,
                        (row + x - 1) * 4,
                        (row + x + 1) * 4,
                        (row_dn + x - 1) * 4,
                        (row_dn + x) * 4,
                        (row_dn + x + 1) * 4,
                    ];
                    for ch in 0..3 {
                        let c0: i32 = rgb[ch];
                        let mut n_min: i32 = c0;
                        let mut n_max: i32 = c0;
                        for off in offsets.into_iter() {
                            let v: i32 = src[off + ch] as i32;
                            n_min = n_min.min(v);
                            n_max = n_max.max(v);
                        }
                        let hist_clamped: i32 = (history[sc + ch] as i32).clamp(n_min, n_max);
                        rgb[ch] = ((c0 * 164 + hist_clamped * 92 + 128) >> 8).clamp(0, 255);
                    }
                }
                if has_bloom {
                    let bl: [f32; 3] = Self::sample_bloom_bilinear(bloom, bw, bh, x, y);
                    rgb[0] = (rgb[0] + (bl[0] * 64.0) as i32).min(255);
                    rgb[1] = (rgb[1] + (bl[1] * 64.0) as i32).min(255);
                    rgb[2] = (rgb[2] + (bl[2] * 64.0) as i32).min(255);
                }
                dst_band[dc] = rgb[0] as u8;
                dst_band[dc + 1] = rgb[1] as u8;
                dst_band[dc + 2] = rgb[2] as u8;
                dst_band[dc + 3] = 255;
            }
        }
    }

    /// Renders `world` at runtime-configured `quality` into an `ImageBuffer` using the persistent zero-spawn multi-core worker pool.
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
        let scale: f32 = quality.get_resolution_scale().clamp(0.40, 4.0);
        let w: u32 = ((viewport_w as f32 * scale).round() as u32).clamp(240, 3840);
        let h: u32 = ((viewport_h as f32 * scale).round() as u32).clamp(144, 2160);
        let w_usize: usize = w as usize;
        let h_usize: usize = h as usize;
        let pixel_count: usize = w_usize * h_usize;
        let bw: usize = w_usize.div_ceil(4);
        let bh: usize = h_usize.div_ceil(4);
        if self.get_width() != w || self.get_height() != h {
            self.set_width(w);
            self.set_height(h);
            let Self {
                color_buffer,
                aa_buffer,
                depth_buffer,
                g_tri_idx,
                luma_buffer,
                bloom_a,
                bloom_b,
                has_taa_history,
                ..
            } = self;
            color_buffer.resize(pixel_count * 4, 0);
            aa_buffer.resize(pixel_count * 4, 0);
            depth_buffer.resize(pixel_count, 0.0);
            g_tri_idx.resize(pixel_count, u32::MAX);
            luma_buffer.resize(pixel_count, 0);
            bloom_a.resize(bw * bh, [0.0; 3]);
            bloom_b.resize(bw * bh, [0.0; 3]);
            *has_taa_history = false;
        }
        let rt_mode: RayTracingMode = quality.get_ray_tracing();
        if rt_mode != RayTracingMode::Off {
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
        let focal_px: f32 = (h as f32) * 0.5 / half_h.max(0.05);
        let draw_dist_sq: f32 = quality.get_draw_distance() * quality.get_draw_distance();
        let frustum_on: bool = quality.get_frustum_culling();
        let chunk_on: bool = quality.get_chunk_streaming();
        let lod_bias: f32 = quality.get_lod_bias();
        let aa_mode: AntiAliasingMode = quality.get_anti_aliasing();
        let mut visible_objects: usize = 0;
        let mut culled_objects: usize = 0;
        let mut triangles_drawn: usize = 0;
        let mut lod_simplified_objects: usize = 0;
        let mut meshlets_drawn: usize = 0;
        let mut meshlets_culled: usize = 0;
        let bvh: &Bvh3D = world.get_bvh();
        let objects: &[SceneObject3D] = world.get_objects().as_slice();
        let mesh_lib: &[MeshLodChain] = world.get_mesh_library().as_slice();
        let cam_pos: Vec3 = cam.get_position();
        let sun_dir: Vec3 = sun.get_direction();
        let sky_horizon: Vec3 = sun.get_sky_horizon();
        let sky_zenith: Vec3 = sun.get_sky_zenith();
        let use_rt_refl: bool = matches!(
            rt_mode,
            RayTracingMode::ShadowsAndReflections | RayTracingMode::FullPathTracedGi
        );
        let Self {
            color_buffer,
            aa_buffer,
            depth_buffer,
            g_tri_idx,
            luma_buffer,
            shaded_tris,
            rt_probes,
            rt_probes_denoised,
            bloom_a,
            bloom_b,
            visible_scratch,
            cluster_ranges_scratch,
            projected_verts_scratch,
            unique_meshes_scratch,
            has_taa_history,
            prev_vp_m0,
            frame_index,
            ..
        } = self;
        let (jitter_x, jitter_y): (f32, f32) = if aa_mode == AntiAliasingMode::Taa {
            HALTON_JITTER_8[(*frame_index as usize) & 7]
        } else {
            (0.0, 0.0)
        };
        *frame_index = frame_index.wrapping_add(1);
        visible_scratch.clear();
        unique_meshes_scratch.clear();
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
            visible_scratch.push((dist_sq, obj_idx));
        }
        visible_scratch.sort_unstable_by(|a: &(f32, usize), b: &(f32, usize)| a.0.total_cmp(&b.0));
        depth_buffer.fill(0.0);
        g_tri_idx.fill(u32::MAX);
        shaded_tris.clear();
        for &(dist_sq, obj_idx) in visible_scratch.iter() {
            let obj: &SceneObject3D = &objects[obj_idx];
            let Some(chain) = mesh_lib.get(obj.get_mesh_id()) else {
                continue;
            };
            visible_objects += 1;
            let _: bool = unique_meshes_scratch.insert(obj.get_mesh_id());
            let dist: f32 = dist_sq.sqrt();
            let obj_radius: f32 = obj.get_world_bounds().radius();
            let (mesh, lod_idx): (&Mesh3D, usize) =
                chain.select_screen_error(dist, obj_radius, focal_px, lod_bias);
            if lod_idx > 0 {
                lod_simplified_objects += 1;
            }
            let tr: Transform3D = obj.get_transform();
            let mat: PbrMaterial = obj.get_material();
            let mat_base: Vec3 = mat.get_base_color();
            let mat_emissive: Vec3 = mat.get_emissive();
            let mat_refl: f32 = mat.get_reflectivity();
            let metallic: f32 = mat.get_metallic();
            let rough: f32 = mat.get_roughness();
            let alpha: f32 = (rough * rough).max(0.018);
            let alpha2: f32 = alpha * alpha;
            let f0: Vec3 = Vec3::splat(0.04).lerp(mat_base, metallic);
            let diffuse_scale: f32 = 1.0 - metallic;
            let do_refl: bool = use_rt_refl && mat_refl > 0.25;
            let verts: &[Vertex3D] = mesh.get_vertices().as_slice();
            let indices: &[[u32; 3]] = mesh.get_indices().as_slice();
            let meshlets: &[Meshlet3D] = mesh.get_meshlets().as_slice();
            cluster_ranges_scratch.clear();
            if meshlets.len() > 1 && frustum_on {
                for mlet in meshlets.iter() {
                    let wb: Aabb3D = tr.transform_aabb(mlet.get_bounds());
                    if !frustum.intersects_aabb(&wb) {
                        meshlets_culled += 1;
                        continue;
                    }
                    meshlets_drawn += 1;
                    let s0: usize = mlet.get_first_tri() as usize;
                    let s1: usize = (s0 + (mlet.get_tri_count() as usize)).min(indices.len());
                    cluster_ranges_scratch.push((s0, s1));
                }
            } else {
                meshlets_drawn += meshlets.len().max(1);
                cluster_ranges_scratch.push((0, indices.len()));
            }
            if cluster_ranges_scratch.is_empty() {
                continue;
            }
            projected_verts_scratch.clear();
            projected_verts_scratch.reserve(verts.len());
            for v in verts.iter() {
                let wp: Vec3 = tr.transform_point(v.get_position());
                let cp: [f32; 4] = vp.mul_point4(wp);
                projected_verts_scratch.push((cp, wp));
            }
            let near_plane: f32 = cam.get_near().max(0.15);
            let w_f: f32 = w as f32;
            let h_f: f32 = h as f32;
            let w_max_i: i32 = (w as i32) - 1;
            let h_max_i: i32 = (h as i32) - 1;
            let vert_count: usize = projected_verts_scratch.len();
            for &(tri_start, tri_end) in cluster_ranges_scratch.iter() {
                for tri in indices[tri_start..tri_end].iter() {
                    let (i0, i1, i2): (usize, usize, usize) =
                        (tri[0] as usize, tri[1] as usize, tri[2] as usize);
                    let oob_01: bool = i0 >= vert_count || i1 >= vert_count;
                    if oob_01 || i2 >= vert_count {
                        continue;
                    }
                    let (c0, wp0): ([f32; 4], Vec3) = projected_verts_scratch[i0];
                    let (c1, wp1): ([f32; 4], Vec3) = projected_verts_scratch[i1];
                    let (c2, wp2): ([f32; 4], Vec3) = projected_verts_scratch[i2];
                    if c0[3] < near_plane && c1[3] < near_plane && c2[3] < near_plane {
                        continue;
                    }
                    let out_l: bool = c0[0] < -c0[3] && c1[0] < -c1[3] && c2[0] < -c2[3];
                    let out_r: bool = c0[0] > c0[3] && c1[0] > c1[3] && c2[0] > c2[3];
                    let out_b: bool = c0[1] < -c0[3] && c1[1] < -c1[3] && c2[1] < -c2[3];
                    let out_t: bool = c0[1] > c0[3] && c1[1] > c1[3] && c2[1] > c2[3];
                    let out_lr: bool = out_l || out_r;
                    let out_bt: bool = out_b || out_t;
                    if out_lr || out_bt {
                        continue;
                    }
                    let (clipped_tris, clipped_count): ([ClippedTriangle3D; 2], usize) =
                        Self::clip_triangle_near((c0, wp0), (c1, wp1), (c2, wp2), near_plane);
                    if clipped_count == 0 {
                        continue;
                    }
                    let mut cached_wn: Option<Vec3> = None;
                    for clipped in clipped_tris.iter().take(clipped_count) {
                        let [(tc0, tw0), (tc1, tw1), (tc2, tw2)]: ClippedTriangle3D = *clipped;
                        let inv_w0: f32 = 1.0 / tc0[3];
                        let inv_w1: f32 = 1.0 / tc1[3];
                        let inv_w2: f32 = 1.0 / tc2[3];
                        let sx0: f32 = (tc0[0] * inv_w0 * 0.5 + 0.5) * w_f + jitter_x;
                        let sy0: f32 = (0.5 - tc0[1] * inv_w0 * 0.5) * h_f + jitter_y;
                        let sx1: f32 = (tc1[0] * inv_w1 * 0.5 + 0.5) * w_f + jitter_x;
                        let sy1: f32 = (0.5 - tc1[1] * inv_w1 * 0.5) * h_f + jitter_y;
                        let sx2: f32 = (tc2[0] * inv_w2 * 0.5 + 0.5) * w_f + jitter_x;
                        let sy2: f32 = (0.5 - tc2[1] * inv_w2 * 0.5) * h_f + jitter_y;
                        let area: f32 = (sx1 - sx0) * (sy2 - sy0) - (sy1 - sy0) * (sx2 - sx0);
                        if area.abs() < 0.25 {
                            continue;
                        }
                        let min_x: i32 = (sx0.min(sx1).min(sx2).floor() as i32).max(0);
                        let max_x: i32 = (sx0.max(sx1).max(sx2).ceil() as i32).min(w_max_i);
                        let min_y: i32 = (sy0.min(sy1).min(sy2).floor() as i32).max(0);
                        let max_y: i32 = (sy0.max(sy1).max(sy2).ceil() as i32).min(h_max_i);
                        if min_x > max_x || min_y > max_y {
                            continue;
                        }
                        let wn: Vec3 = match cached_wn {
                            Some(n) => n,
                            None => {
                                let computed: Vec3 = tr.transform_normal(verts[i0].get_normal());
                                cached_wn = Some(computed);
                                computed
                            }
                        };
                        triangles_drawn += 1;
                        let tri_id: u32 = shaded_tris.len() as u32;
                        let tri_center: Vec3 = tw0.add(tw1).add(tw2).scale(1.0 / 3.0);
                        let tri_view: Vec3 = cam_pos.sub(tri_center).normalize();
                        let shade_n: Vec3 = if wn.dot(tri_view) < 0.0 {
                            wn.scale(-1.0)
                        } else {
                            wn
                        };
                        let n_dot_l: f32 = shade_n.dot(sun_dir).max(0.0);
                        let ambient_sky: Vec3 = sky_horizon
                            .lerp(sky_zenith, (shade_n.get_y() * 0.5 + 0.5).clamp(0.0, 1.0));
                        shaded_tris.push(GShadeTri {
                            shade_n,
                            mat_base,
                            mat_emissive,
                            ambient_sky,
                            f0,
                            n_dot_l,
                            alpha2,
                            diffuse_scale,
                            mat_refl,
                            metallic,
                            do_refl,
                            surface_detail: mat.get_surface_detail(),
                        });
                        let inv_area: f32 = 1.0 / area;
                        let dw0_dx: f32 = (sy1 - sy2) * inv_area;
                        let dw1_dx: f32 = (sy2 - sy0) * inv_area;
                        let dw2_dx: f32 = -(dw0_dx + dw1_dx);
                        let dw0_dy: f32 = (sx2 - sx1) * inv_area;
                        let dw1_dy: f32 = (sx0 - sx2) * inv_area;
                        let dw2_dy: f32 = -(dw0_dy + dw1_dy);
                        let d_inv_w_dx: f32 = dw0_dx * inv_w0 + dw1_dx * inv_w1 + dw2_dx * inv_w2;
                        let d_inv_w_dy: f32 = dw0_dy * inv_w0 + dw1_dy * inv_w1 + dw2_dy * inv_w2;
                        let neg_inv_slopes: [f32; 3] = [
                            if dw0_dx.abs() > 1e-9 {
                                -1.0 / dw0_dx
                            } else {
                                0.0
                            },
                            if dw1_dx.abs() > 1e-9 {
                                -1.0 / dw1_dx
                            } else {
                                0.0
                            },
                            if dw2_dx.abs() > 1e-9 {
                                -1.0 / dw2_dx
                            } else {
                                0.0
                            },
                        ];
                        let slopes: [f32; 3] = [dw0_dx, dw1_dx, dw2_dx];
                        let span_max: i32 = max_x - min_x;
                        let span_max_f: f32 = span_max as f32;
                        let fx_start: f32 = (min_x as f32) + 0.5;
                        let fy_start: f32 = (min_y as f32) + 0.5;
                        let mut w0_row: f32 = ((sx1 - fx_start) * (sy2 - fy_start)
                            - (sy1 - fy_start) * (sx2 - fx_start))
                            * inv_area;
                        let mut w1_row: f32 = ((sx2 - fx_start) * (sy0 - fy_start)
                            - (sy2 - fy_start) * (sx0 - fx_start))
                            * inv_area;
                        let mut w2_row: f32 = 1.0 - w0_row - w1_row;
                        let mut inv_w_row: f32 =
                            w0_row * inv_w0 + w1_row * inv_w1 + w2_row * inv_w2;
                        let mut row_off: usize = (min_y as usize) * w_usize;
                        for _py in min_y..=max_y {
                            let vals: [f32; 3] = [w0_row + 1e-5, w1_row + 1e-5, w2_row + 1e-5];
                            let mut k_lo: f32 = 0.0;
                            let mut k_hi: f32 = span_max_f;
                            let mut valid_span: bool = true;
                            for e in 0..3 {
                                let slope: f32 = slopes[e];
                                let val: f32 = vals[e];
                                if slope > 1e-9 {
                                    k_lo = k_lo.max((val * neg_inv_slopes[e]).ceil());
                                } else if slope < -1e-9 {
                                    k_hi = k_hi.min((val * neg_inv_slopes[e]).floor());
                                } else if val < 0.0 {
                                    valid_span = false;
                                    break;
                                }
                            }
                            if valid_span && k_lo <= k_hi {
                                let start_k: i32 = (k_lo as i32).max(0);
                                let end_k: i32 = (k_hi as i32).min(span_max);
                                if start_k <= end_k {
                                    let mut inv_w: f32 =
                                        d_inv_w_dx.mul_add(start_k as f32, inv_w_row);
                                    let px0: usize = row_off + ((min_x + start_k) as usize);
                                    let px1: usize = row_off + ((min_x + end_k) as usize);
                                    for (d_slot, g_slot) in depth_buffer[px0..=px1]
                                        .iter_mut()
                                        .zip(g_tri_idx[px0..=px1].iter_mut())
                                    {
                                        if inv_w > *d_slot {
                                            *d_slot = inv_w;
                                            *g_slot = tri_id;
                                        }
                                        inv_w += d_inv_w_dx;
                                    }
                                }
                            }
                            w0_row += dw0_dy;
                            w1_row += dw1_dy;
                            w2_row += dw2_dy;
                            inv_w_row += d_inv_w_dy;
                            row_off += w_usize;
                        }
                    }
                }
            }
        }
        let rt_step: usize = if rt_mode == RayTracingMode::Off {
            0
        } else if w_usize >= 780 {
            6
        } else if w_usize >= 460 {
            5
        } else if w_usize >= 300 {
            4
        } else {
            2
        };
        let gw: usize = if rt_step > 0 {
            w_usize.div_ceil(rt_step)
        } else {
            0
        };
        let gh: usize = if rt_step > 0 {
            h_usize.div_ceil(rt_step)
        } else {
            0
        };
        let bloom_on: bool = matches!(
            quality.get_preset(),
            QualityPreset::High | QualityPreset::Ultra | QualityPreset::Cinematic
        );
        let fctx: DeferredFrameCtx = DeferredFrameCtx {
            w: w_usize,
            h: h_usize,
            gw,
            gh,
            bw,
            rt_step,
            inv_w_f: 1.0 / (w as f32).max(1.0),
            inv_h_f: 1.0 / (h as f32).max(1.0),
            cam_pos,
            cam_fwd,
            cam_right_scaled: cam_right.scale(half_w),
            cam_up_scaled: cam_up.scale(half_h),
            sun,
            inv_sun_dir: Ray3D::safe_inv_vec(sun_dir),
            rt_mode,
            reflection_bounces: quality.get_reflection_bounces(),
            fog_on: quality.get_volumetric_fog(),
            inv_fog_dist: 1.45 / quality.get_draw_distance().max(40.0),
            exposure_scale: 1.0,
            bloom_on,
        };
        let rays_atom: AtomicUsize = AtomicUsize::new(0);
        if rt_step > 0 {
            let probe_count: usize = gw * gh;
            if rt_probes.len() != probe_count {
                rt_probes.resize(probe_count, RtProbeSample::default());
            }
            let depth_ref: &[f32] = depth_buffer.as_slice();
            let gtri_ref: &[u32] = g_tri_idx.as_slice();
            let stri_ref: &[GShadeTri] = shaded_tris.as_slice();
            let probes_base: usize = rt_probes.as_mut_ptr() as usize;
            let probe_grain: usize = (gh / 16).clamp(4, 16);
            parallel_for_ranges(gh, probe_grain, |gy0: usize, gy1: usize| {
                let slice: &mut [RtProbeSample] = unsafe {
                    from_raw_parts_mut(
                        (probes_base as *mut RtProbeSample).add(gy0 * gw),
                        (gy1 - gy0) * gw,
                    )
                };
                let r: usize =
                    Self::trace_probe_band(&fctx, depth_ref, gtri_ref, stri_ref, bvh, (gy0, slice));
                if r > 0 {
                    let _: usize = rays_atom.fetch_add(r, Ordering::Relaxed);
                }
            });
            let need_denoise: bool = matches!(
                rt_mode,
                RayTracingMode::ShadowsAndReflections | RayTracingMode::FullPathTracedGi
            );
            if need_denoise {
                if rt_probes_denoised.len() != probe_count {
                    rt_probes_denoised.resize(probe_count, RtProbeSample::default());
                }
                let src_probes: &[RtProbeSample] = rt_probes.as_slice();
                let dst_base: usize = rt_probes_denoised.as_mut_ptr() as usize;
                parallel_for_ranges(gh, probe_grain, |gy0: usize, gy1: usize| {
                    let dst_slice: &mut [RtProbeSample] = unsafe {
                        from_raw_parts_mut(
                            (dst_base as *mut RtProbeSample).add(gy0 * gw),
                            (gy1 - gy0) * gw,
                        )
                    };
                    Self::denoise_gi_probes_band(gw, gh, src_probes, gy0, dst_slice);
                });
            }
        }
        let active_probes: &[RtProbeSample] = if rt_step == 0 {
            &[]
        } else if matches!(
            rt_mode,
            RayTracingMode::ShadowsAndReflections | RayTracingMode::FullPathTracedGi
        ) {
            rt_probes_denoised.as_slice()
        } else {
            rt_probes.as_slice()
        };
        {
            let depth_ref: &[f32] = depth_buffer.as_slice();
            let gtri_ref: &[u32] = g_tri_idx.as_slice();
            let stri_ref: &[GShadeTri] = shaded_tris.as_slice();
            let color_base: usize = color_buffer.as_mut_ptr() as usize;
            let luma_base: usize = luma_buffer.as_mut_ptr() as usize;
            let bloom_base: usize = bloom_a.as_mut_ptr() as usize;
            let bloom_total_len: usize = bloom_a.len();
            let shade_grain: usize = ((h_usize / 16).div_ceil(4) * 4).clamp(16, 48);
            parallel_for_ranges(h_usize, shade_grain, |y0: usize, y1: usize| {
                let rows: usize = y1 - y0;
                let c_band: &mut [u8] = unsafe {
                    from_raw_parts_mut(
                        (color_base as *mut u8).add(y0 * w_usize * 4),
                        rows * w_usize * 4,
                    )
                };
                let l_band: &mut [u8] = unsafe {
                    from_raw_parts_mut((luma_base as *mut u8).add(y0 * w_usize), rows * w_usize)
                };
                let b_start: usize = (y0 / 4) * bw;
                let b_end: usize = ((y1.div_ceil(4)) * bw).min(bloom_total_len);
                let b_band: &mut [[f32; 3]] = if b_start < b_end {
                    unsafe {
                        from_raw_parts_mut(
                            (bloom_base as *mut [f32; 3]).add(b_start),
                            b_end - b_start,
                        )
                    }
                } else {
                    &mut []
                };
                let r: usize = Self::shade_deferred_band(
                    &fctx,
                    (depth_ref, gtri_ref, stri_ref, active_probes, bvh),
                    (y0, c_band, l_band, b_band),
                );
                if r > 0 {
                    let _: usize = rays_atom.fetch_add(r, Ordering::Relaxed);
                }
            });
        }
        let rays_traced: usize = rays_atom.load(Ordering::Relaxed);
        if bloom_on {
            Self::blur_bloom_separable(bw, bh, bloom_a.as_mut_slice(), bloom_b.as_mut_slice());
        }
        let use_fxaa: bool = aa_mode != AntiAliasingMode::Off;
        let vp_m0: f32 = vp.get_m()[0] + vp.get_m()[12] * 0.01 + vp.get_m()[14] * 0.01;
        let cam_stable: bool = (*prev_vp_m0 - vp_m0).abs() < 0.35;
        let use_taa: bool = aa_mode == AntiAliasingMode::Taa && *has_taa_history && cam_stable;
        let mut out_pixels: Vec<u8> = vec![0; pixel_count * 4];
        {
            let src_ref: &[u8] = color_buffer.as_slice();
            let luma_ref: &[u8] = luma_buffer.as_slice();
            let gtri_ref: &[u32] = g_tri_idx.as_slice();
            let bloom_ref: &[[f32; 3]] = bloom_a.as_slice();
            let hist_ref: &[u8] = aa_buffer.as_slice();
            let out_base: usize = out_pixels.as_mut_ptr() as usize;
            let post_grain: usize = (h_usize / 16).clamp(16, 48);
            parallel_for_ranges(h_usize, post_grain, |y0: usize, y1: usize| {
                let dst_band: &mut [u8] = unsafe {
                    from_raw_parts_mut(
                        (out_base as *mut u8).add(y0 * w_usize * 4),
                        (y1 - y0) * w_usize * 4,
                    )
                };
                Self::post_process_band(
                    (w_usize, h_usize, bw, bh),
                    (src_ref, luma_ref, gtri_ref, bloom_ref, hist_ref),
                    (use_fxaa, use_taa, bloom_on),
                    (y0, dst_band),
                );
            });
        }
        if aa_mode == AntiAliasingMode::Taa {
            swap(aa_buffer, color_buffer);
            *has_taa_history = true;
            *prev_vp_m0 = vp_m0;
        } else {
            *has_taa_history = false;
        }
        let instanced_batches: usize = if quality.get_gpu_instancing() {
            unique_meshes_scratch.len()
        } else {
            visible_objects
        };
        let stats: RenderStats3D = RenderStats3D {
            render_width: w,
            render_height: h,
            visible_objects,
            culled_objects,
            instanced_batches,
            triangles_drawn,
            rays_traced,
            bvh_nodes: bvh.get_nodes().len(),
            lod_simplified_objects,
            meshlets_drawn,
            meshlets_culled,
        };
        self.set_last_stats(stats);
        ImageBuffer::new(w, h, out_pixels)
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
}
