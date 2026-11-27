use super::*;

/// 64-bit FNV-1a offset basis used for deterministic cross-platform pixel hashing.
pub(crate) const FNV1A_OFFSET_BASIS: u64 = 0xCBF2_9CE4_8422_2325;

/// 64-bit FNV-1a prime multiplier.
pub(crate) const FNV1A_PRIME: u64 = 0x0000_0100_0000_01B3;

/// Compile-time 1 KB lookup table converting sRGB `u8` channels into gamma-2.0 linear light.
pub(crate) const SRGB_TO_LIN_LUT: [f32; 256] = build_srgb_to_lin_lut();

/// Compile-time 8 KB lookup table converting linear-light `[0.0, 1.0]` into Q12.4 sRGB (`sRGB * 16`) for zero-`sqrt` dithered compositing.
pub(crate) const LIN_TO_SRGB_Q4_LUT: [u16; 4096] = build_lin_to_srgb_q4_lut();

/// 0.32 fixed-point horizontal IGN phase increment `round(0.06711056 * 2^32)` for branchless incremental dithering.
pub(crate) const IGN_X_STEP_U32: u32 = 288_237_642;

/// 0.32 fixed-point vertical IGN phase increment `round(0.00583715 * 2^32)` for branchless incremental dithering.
pub(crate) const IGN_Y_STEP_U32: u32 = 25_070_371;

/// 6.26 fixed-point IGN hash multiplier `round(52.982918 * 2^26)` producing exact 4-bit dither values via `(phase * mul) >> 54`.
pub(crate) const IGN_MUL_Q26: u64 = 3_555_623_499;
