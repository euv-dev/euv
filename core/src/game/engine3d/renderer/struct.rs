use super::*;

/// Synchronized dispatch epoch and active job pointer for the persistent 3D worker pool.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Data, Debug, Default)]
pub(crate) struct Engine3dPoolState {
    /// Monotonically increasing dispatch epoch counter.
    #[get(type(copy))]
    pub(crate) epoch: u64,
    /// Raw pointer address of the active `Engine3dJob` (`0` when idle).
    #[get(type(copy))]
    pub(crate) active_job: usize,
}

/// Active work-stealing range job executed synchronously across caller and persistent 3D worker threads.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub(crate) struct Engine3dJob {
    /// Type-erased chunk execution trampoline.
    pub(crate) trampoline: Engine3dChunkFn,
    /// Caller-pinned closure reference encoded as `usize`.
    pub(crate) ctx_ptr: usize,
    /// Total logical items across all chunks.
    pub(crate) total_items: usize,
    /// Number of items per dynamic work-stealing chunk.
    pub(crate) chunk_size: usize,
    /// Total chunks to claim (`total_items.div_ceil(chunk_size)`).
    pub(crate) total_chunks: usize,
    /// Atomic cursor of the next unclaimed chunk index.
    pub(crate) next_chunk: AtomicUsize,
    /// Atomic count of background workers finished observing this epoch.
    pub(crate) workers_done: AtomicUsize,
}

/// Persistent warm thread pool eliminating per-frame OS thread spawns in `Renderer3D`.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub(crate) struct Engine3dWorkerPool {
    /// Number of background worker threads (excluding the calling thread).
    pub(crate) worker_count: usize,
    /// Non-blocking re-entrancy guard serializing top-level dispatches cleanly.
    pub(crate) dispatch_lock: Mutex<()>,
    /// Shared dispatch state mutex and worker/caller condition variables.
    pub(crate) shared: Arc<(Mutex<Engine3dPoolState>, Condvar, Condvar)>,
}

/// Per-frame telemetry emitted by `Renderer3D` for live FPS/RT/LOD/Nanite/culling diagnostics.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct RenderStats3D {
    /// Internal rasterized framebuffer width in pixels.
    #[get(type(copy))]
    pub(crate) render_width: u32,
    /// Internal rasterized framebuffer height in pixels.
    #[get(type(copy))]
    pub(crate) render_height: u32,
    /// Active objects rendered inside frustum & chunk stream radius.
    #[get(type(copy))]
    pub(crate) visible_objects: usize,
    /// Objects rejected by 6-plane frustum culling or chunk draw distance.
    #[get(type(copy))]
    pub(crate) culled_objects: usize,
    /// Deduplicated GPU instancing draw batches.
    #[get(type(copy))]
    pub(crate) instanced_batches: usize,
    /// Triangles rasterized after backface culling.
    #[get(type(copy))]
    pub(crate) triangles_drawn: usize,
    /// BVH shadow/reflection/GI rays traced during this frame.
    #[get(type(copy))]
    pub(crate) rays_traced: usize,
    /// Total BVH nodes active in the acceleration structure.
    #[get(type(copy))]
    pub(crate) bvh_nodes: usize,
    /// Objects rendered at simplified distant LOD (`LOD1+`).
    #[get(type(copy))]
    pub(crate) lod_simplified_objects: usize,
    /// Nanite-style meshlet clusters drawn this frame.
    #[get(type(copy))]
    pub(crate) meshlets_drawn: usize,
    /// Nanite-style meshlet clusters culled by frustum or backface normal cones.
    #[get(type(copy))]
    pub(crate) meshlets_culled: usize,
}

/// Precomputed per-triangle Cook-Torrance GGX PBR shading record referenced by deferred G-Buffer indices.
#[derive(Clone, Copy, Debug)]
pub(crate) struct GShadeTri {
    /// Camera-facing unit shading normal.
    pub(crate) shade_n: Vec3,
    /// Material linear base albedo.
    pub(crate) mat_base: Vec3,
    /// Material linear emissive radiance.
    pub(crate) mat_emissive: Vec3,
    /// Precomputed hemisphere sky ambient tint.
    pub(crate) ambient_sky: Vec3,
    /// Precomputed Schlick base specular reflectance `F0 = lerp(0.04, albedo, metallic)`.
    pub(crate) f0: Vec3,
    /// Clamped directional sun Lambertian term `max(N·L, 0)`.
    pub(crate) n_dot_l: f32,
    /// GGX roughness squared (`alpha^2 = roughness^4`).
    pub(crate) alpha2: f32,
    /// Energy-conserving diffuse scale `(1.0 - metallic)`.
    pub(crate) diffuse_scale: f32,
    /// Surface base reflectivity weight `[0.0, 1.0]`.
    pub(crate) mat_refl: f32,
    /// Surface metallic scalar `[0.0, 1.0]`.
    pub(crate) metallic: f32,
    /// Whether this surface traces specular reflections.
    pub(crate) do_refl: bool,
    /// Procedural surface detail mode attached to the triangle's material.
    pub(crate) surface_detail: SurfaceDetailMode,
}

/// Coarse deferred ray-tracing probe storing soft shadow, multi-bounce reflection, colored Lumen GI, and bilateral edge metadata.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RtProbeSample {
    /// Sun visibility factor (`1.0` lit, `0.22` shadowed).
    pub(crate) shadow: f32,
    /// Traced reflection linear red radiance.
    pub(crate) refl_r: f32,
    /// Traced reflection linear green radiance.
    pub(crate) refl_g: f32,
    /// Traced reflection linear blue radiance.
    pub(crate) refl_b: f32,
    /// Colored indirect Lumen GI bounce linear radiance (red).
    pub(crate) gi_r: f32,
    /// Colored indirect Lumen GI bounce linear radiance (green).
    pub(crate) gi_g: f32,
    /// Colored indirect Lumen GI bounce linear radiance (blue).
    pub(crate) gi_b: f32,
    /// Ambient occlusion factor in `[0.42, 1.0]`.
    pub(crate) ao: f32,
    /// Camera-space linear depth at probe center.
    pub(crate) depth: f32,
    /// Surface normal X for bilateral edge stopping.
    pub(crate) nx: f32,
    /// Surface normal Y for bilateral edge stopping.
    pub(crate) ny: f32,
    /// Surface normal Z for bilateral edge stopping.
    pub(crate) nz: f32,
    /// Non-zero when this probe sits on valid scene geometry.
    pub(crate) valid: u8,
}

/// Immutable per-frame camera, lighting, volumetric, and ray-tracing configuration shared across worker threads.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DeferredFrameCtx {
    /// Framebuffer width in pixels.
    pub(crate) w: usize,
    /// Framebuffer height in pixels.
    pub(crate) h: usize,
    /// Coarse RT grid width in probes.
    pub(crate) gw: usize,
    /// Coarse RT grid height in probes.
    pub(crate) gh: usize,
    /// Quarter-res HDR bloom width.
    pub(crate) bw: usize,
    /// Pixel stride per coarse RT probe.
    pub(crate) rt_step: usize,
    /// Reciprocal framebuffer width.
    pub(crate) inv_w_f: f32,
    /// Reciprocal framebuffer height.
    pub(crate) inv_h_f: f32,
    /// Camera world position.
    pub(crate) cam_pos: Vec3,
    /// Camera unit forward basis.
    pub(crate) cam_fwd: Vec3,
    /// Camera right basis scaled by `half_w`.
    pub(crate) cam_right_scaled: Vec3,
    /// Camera up basis scaled by `half_h`.
    pub(crate) cam_up_scaled: Vec3,
    /// Active directional sun & sky parameters.
    pub(crate) sun: DirectionalLight3D,
    /// Precomputed safe reciprocal sun direction for fast shadow BVH traversal.
    pub(crate) inv_sun_dir: Vec3,
    /// Active ray-tracing quality tier.
    pub(crate) rt_mode: RayTracingMode,
    /// Maximum reflection/GI bounce depth (`0..=3`).
    pub(crate) reflection_bounces: u8,
    /// Whether volumetric height/distance fog and crepuscular sun-shafts are enabled.
    pub(crate) fog_on: bool,
    /// Precomputed inverse fog attenuation distance.
    pub(crate) inv_fog_dist: f32,
    /// Exposure scale derived from physically based camera EV100.
    pub(crate) exposure_scale: f32,
    /// Whether quarter-res emissive/specular bloom extraction is active.
    pub(crate) bloom_on: bool,
}

/// Zero-allocation UE5-class Deferred GGX PBR + Persistent Multi-Core SAH BVH + Lumen GI + Bloom + Morphological AA/TAA 3D renderer.
#[derive(Clone, Data, Debug, Default)]
pub struct Renderer3D {
    /// Pre-allocated RGBA8 tone-mapped intermediate pixel buffer.
    pub(crate) color_buffer: Vec<u8>,
    /// Pre-allocated RGBA8 temporal history buffer for variance-clamped TAA.
    pub(crate) aa_buffer: Vec<u8>,
    /// Pre-allocated floating-point linear `1/w` reverse-Z depth buffer (`0.0` = sky).
    pub(crate) depth_buffer: Vec<f32>,
    /// Deferred G-Buffer triangle index per pixel (`u32::MAX` = sky).
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) g_tri_idx: Vec<u32>,
    /// Single-byte per-pixel Rec.601 luma buffer populated during deferred shading for fast morphological AA.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) luma_buffer: Vec<u8>,
    /// Per-frame precomputed GGX triangle shading records.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) shaded_tris: Vec<GShadeTri>,
    /// Coarse coherent ray-tracing probe grid.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) rt_probes: Vec<RtProbeSample>,
    /// Bilaterally denoised coarse Lumen GI and soft-shadow probe grid.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) rt_probes_denoised: Vec<RtProbeSample>,
    /// Quarter-res HDR bloom extraction and vertical blur buffer.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) bloom_a: Vec<[f32; 3]>,
    /// Quarter-res HDR bloom horizontal blur scratch buffer.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) bloom_b: Vec<[f32; 3]>,
    /// Reusable front-to-back object sorting scratch buffer.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) visible_scratch: Vec<(f32, usize)>,
    /// Reusable meshlet triangle-range scratch buffer eliminating per-object heap allocations.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) cluster_ranges_scratch: Vec<(usize, usize)>,
    /// Reusable per-object transformed vertex cache `(clip_pos, world_pos)` avoiding redundant vertex transforms.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) projected_verts_scratch: Vec<([f32; 4], Vec3)>,
    /// Reusable unique mesh-id set for instancing telemetry.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) unique_meshes_scratch: HashSet<usize>,
    /// Whether `aa_buffer` holds a valid previous frame compatible with temporal anti-aliasing.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) has_taa_history: bool,
    /// Previous frame view-projection fingerprint used to detect camera cuts for TAA.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) prev_vp_m0: f32,
    /// Monotonic frame counter driving Halton(2, 3) temporal subpixel jitter.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) frame_index: u32,
    /// Last rendered internal width.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// Last rendered internal height.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// Telemetry from the most recent frame.
    #[get(type(copy))]
    pub(crate) last_stats: RenderStats3D,
}
