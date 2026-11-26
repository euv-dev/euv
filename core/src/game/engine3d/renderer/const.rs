/// Thread name prefix for persistent 3D parallel rendering workers.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const ENGINE3D_WORKER_PREFIX: &str = "euv-3d";

/// Maximum tangent edge-walk radius in pixels for morphological silhouette anti-aliasing.
pub(crate) const MLAA_MAX_WALK_STEPS: usize = 8;

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
