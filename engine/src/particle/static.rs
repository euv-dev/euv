use super::*;

/// The fixed 32-color palette used to quantize particle colors for
/// batch-friendly rendering.
///
/// Each entry covers a point in RGB-alpha space chosen to approximate
/// common particle effects (mono-color, fade-to-white, fade-to-transparent,
/// bright sparks, cool blues, warm oranges, etc.). The palette uses alpha
/// `1.0` for every entry so a particle that fades out visibly still maps
/// to a deterministic bucket; particle lifetime fade is applied by the
/// global alpha state set before drawing rather than via per-particle
/// alpha variation.
///
/// The order matters only insofar as `quantize` must return the same
/// index for the same input every call; the choice of order here is
/// arbitrary.
///
/// LazyLock because `Color::new` (lombok-generated) is not a `const fn`,
/// so the array cannot be initialized at static-construction time. The
/// first call to `quantize` materializes the palette; subsequent calls
/// reuse the same allocation.
pub(crate) static PARTICLE_PALETTE: LazyLock<[Color; PARTICLE_PALETTE_SIZE]> =
    LazyLock::new(|| {
        [
            Color::new(1.000, 1.000, 1.000, 1.000),
            Color::new(0.875, 0.875, 0.875, 1.000),
            Color::new(0.750, 0.750, 0.750, 1.000),
            Color::new(0.625, 0.625, 0.625, 1.000),
            Color::new(0.500, 0.500, 0.500, 1.000),
            Color::new(0.375, 0.375, 0.375, 1.000),
            Color::new(0.250, 0.250, 0.250, 1.000),
            Color::new(0.125, 0.125, 0.125, 1.000),
            Color::new(0.000, 0.000, 0.000, 1.000),
            Color::new(1.000, 0.000, 0.000, 1.000),
            Color::new(0.000, 1.000, 0.000, 1.000),
            Color::new(0.000, 0.000, 1.000, 1.000),
            Color::new(1.000, 1.000, 0.000, 1.000),
            Color::new(0.000, 1.000, 1.000, 1.000),
            Color::new(1.000, 0.000, 1.000, 1.000),
            Color::new(1.000, 0.500, 0.000, 1.000),
            Color::new(0.500, 1.000, 0.000, 1.000),
            Color::new(0.000, 1.000, 0.500, 1.000),
            Color::new(0.000, 0.500, 1.000, 1.000),
            Color::new(0.500, 0.000, 1.000, 1.000),
            Color::new(1.000, 0.000, 0.500, 1.000),
            Color::new(0.875, 0.250, 0.125, 1.000),
            Color::new(0.250, 0.875, 0.125, 1.000),
            Color::new(0.125, 0.250, 0.875, 1.000),
            Color::new(0.875, 0.125, 0.250, 1.000),
            Color::new(0.250, 0.125, 0.875, 1.000),
            Color::new(0.125, 0.875, 0.250, 1.000),
            Color::new(0.625, 0.625, 0.125, 1.000),
            Color::new(0.125, 0.625, 0.625, 1.000),
            Color::new(0.625, 0.125, 0.625, 1.000),
            Color::new(0.375, 0.375, 0.875, 1.000),
            Color::new(0.875, 0.375, 0.375, 1.000),
        ]
    });
