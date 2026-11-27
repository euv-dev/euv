/// Thread name prefix for persistent 3D parallel rendering workers.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const ENGINE3D_WORKER_PREFIX: &str = "euv-3d";

/// Maximum total 3D renderer threads (caller plus background workers) in `Engine3dWorkerPool`.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const ENGINE3D_WORKER_MAX_THREADS: usize = 16;

/// Short spin-loop iterations performed by idle 3D workers before parking on the condition variable.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const ENGINE3D_WORKER_SPIN_LIMIT: u32 = 64;

/// Short spin-loop iterations performed by the calling thread before parking on `wake_caller`.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const ENGINE3D_CALLER_SPIN_LIMIT: u32 = 64;

/// Maximum tangent edge-walk radius in pixels for morphological silhouette anti-aliasing.
pub(crate) const MLAA_MAX_WALK_STEPS: usize = 8;

/// 2x2 tangent offsets for precomputed per-frame soft-shadow ray directions.
pub(crate) const SOFT_SHADOW_OFFSETS_4: [(f32, f32); 4] = [
    (0.016, 0.012),
    (-0.014, 0.016),
    (0.012, -0.016),
    (-0.016, -0.012),
];

/// 2x2 hemisphere jitter vectors for precomputed per-triangle Lumen GI/AO bounce rays.
pub(crate) const GI_JITTER_4: [[f32; 3]; 4] = [
    [0.38, 0.52, -0.24],
    [-0.34, 0.56, 0.28],
    [0.24, 0.48, 0.36],
    [-0.28, 0.50, -0.34],
];

/// 8-tap Halton(2, 3) subpixel jitter pattern centered in `[-0.35, +0.35]` pixels for temporal super-sampling (TAA).
pub(crate) const HALTON_JITTER_8: [(f32, f32); 8] = [
    (0.0, 0.0),
    (-0.25, 0.166_667),
    (0.25, -0.166_667),
    (-0.125, -0.333_333),
    (0.125, 0.333_333),
    (-0.312_5, -0.055_556),
    (0.312_5, 0.055_556),
    (-0.062_5, 0.277_778),
];
