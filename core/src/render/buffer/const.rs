use super::*;

/// 64-bit FNV-1a offset basis used for deterministic cross-platform pixel hashing.
pub(crate) const FNV1A_OFFSET_BASIS: u64 = 0xCBF2_9CE4_8422_2325;

/// 64-bit FNV-1a prime multiplier.
pub(crate) const FNV1A_PRIME: u64 = 0x0000_0100_0000_01B3;

/// Compile-time 1 KB lookup table converting sRGB `u8` channels into gamma-2.0 linear light.
pub(crate) const SRGB_TO_LIN_LUT: [f32; 256] = build_srgb_to_lin_lut();

/// Compile-time 8 KB lookup table converting linear-light `[0.0, 1.0]` into Q12.4 sRGB (`sRGB * 16`) for zero-`sqrt` dithered compositing.
pub(crate) const LIN_TO_SRGB_Q4_LUT: [u16; 4096] = build_lin_to_srgb_q4_lut();
