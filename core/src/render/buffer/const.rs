/// 64-bit FNV-1a offset basis used for deterministic cross-platform pixel hashing.
pub(crate) const FNV1A_OFFSET_BASIS: u64 = 0xCBF2_9CE4_8422_2325;

/// 64-bit FNV-1a prime multiplier.
pub(crate) const FNV1A_PRIME: u64 = 0x0000_0100_0000_01B3;
