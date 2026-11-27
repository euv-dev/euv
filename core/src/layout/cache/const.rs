/// Number of 2-way associative sets in `TextSizeCache` (power of two).
pub(crate) const TEXT_SIZE_CACHE_SETS: usize = 1024;

/// Bitmask for mapping a mixed hash to a `TextSizeCache` set index.
pub(crate) const TEXT_SIZE_CACHE_SET_MASK: usize = TEXT_SIZE_CACHE_SETS - 1;

/// Number of associative ways per set in `TextSizeCache`.
pub(crate) const TEXT_SIZE_CACHE_WAYS: usize = 2;

/// Total number of slots in `TextSizeCache`.
pub(crate) const TEXT_SIZE_CACHE_SLOTS: usize = TEXT_SIZE_CACHE_SETS * TEXT_SIZE_CACHE_WAYS;

/// Occupancy bit set in `TextSizeEntry::style_flags` for populated cache slots.
pub(crate) const TEXT_SIZE_ENTRY_OCCUPIED: u16 = 1 << 8;

/// Sentinel bit pattern representing `max_width == None` in `TextSizeEntry`.
pub(crate) const TEXT_SIZE_MAX_WIDTH_NONE_BITS: u32 = u32::MAX;

/// Primary 64-bit hash seed for `TextSizeCache`.
pub(crate) const TEXT_HASH_SEED_A: u64 = 0xcbf2_9ce4_8422_2325;

/// Secondary 64-bit hash seed for `TextSizeCache`.
pub(crate) const TEXT_HASH_SEED_B: u64 = 0x9e37_79b9_7f4a_7c15;

/// Primary 64-bit hash multiplier for `TextSizeCache`.
pub(crate) const TEXT_HASH_PRIME_A: u64 = 0x0000_0100_0000_01b3;

/// Secondary 64-bit hash multiplier for `TextSizeCache`.
pub(crate) const TEXT_HASH_PRIME_B: u64 = 0x517c_c1b7_2722_0a95;

/// Primary Murmur3/SplitMix64 avalanche constant for `TextSizeCache`.
pub(crate) const TEXT_HASH_FINAL_A: u64 = 0xff51_afd7_ed55_8ccd;

/// Secondary Murmur3/SplitMix64 avalanche constant for `TextSizeCache`.
pub(crate) const TEXT_HASH_FINAL_B: u64 = 0xc4ce_b9fe_1a85_ec53;

/// `FlexItemRecord::flags` bit indicating `LayoutStyle::width == Length::Auto`.
pub(crate) const FLEX_ITEM_WIDTH_AUTO: u16 = 1 << 0;

/// `FlexItemRecord::flags` bit indicating `LayoutStyle::height == Length::Auto`.
pub(crate) const FLEX_ITEM_HEIGHT_AUTO: u16 = 1 << 1;

/// `FlexItemRecord::flags` bit indicating `LayoutStyle::width` is `Length::Percent(_)`.
pub(crate) const FLEX_ITEM_WIDTH_PERCENT: u16 = 1 << 2;

/// `FlexItemRecord::flags` bit indicating `LayoutStyle::height` is `Length::Percent(_)`.
pub(crate) const FLEX_ITEM_HEIGHT_PERCENT: u16 = 1 << 3;

/// `FlexItemRecord::flags` bit indicating `min_width` or `max_width` is `Length::Percent(_)`.
pub(crate) const FLEX_ITEM_MINMAX_W_PERCENT: u16 = 1 << 4;

/// `FlexItemRecord::flags` bit indicating `min_height` or `max_height` is `Length::Percent(_)`.
pub(crate) const FLEX_ITEM_MINMAX_H_PERCENT: u16 = 1 << 5;

/// `FlexItemRecord::flags` bit indicating `aspect_ratio` is configured on the child.
pub(crate) const FLEX_ITEM_HAS_ASPECT_RATIO: u16 = 1 << 6;

/// `FlexItemRecord::flags` bit indicating `min_height` or `max_height` is explicitly set.
pub(crate) const FLEX_ITEM_MINMAX_H_SET: u16 = 1 << 7;

/// `FlexItemRecord::flags` bit indicating the child was laid out or cache-verified during probing.
pub(crate) const FLEX_ITEM_PROBED: u16 = 1 << 8;

/// `FlexItemRecord::flags` bit indicating the child is a leaf node without nested children.
pub(crate) const FLEX_ITEM_IS_LEAF: u16 = 1 << 9;

/// Default `FlexItemRecord::flags` mask for an unattached auto-sized leaf fallback entry.
pub(crate) const FLEX_ITEM_DEFAULT_LEAF_FLAGS: u16 =
    FLEX_ITEM_WIDTH_AUTO + FLEX_ITEM_HEIGHT_AUTO + FLEX_ITEM_IS_LEAF;
