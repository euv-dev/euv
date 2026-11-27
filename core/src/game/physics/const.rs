/// Default spatial-hash cell edge size in world units.
pub const DEFAULT_SPATIAL_CELL_SIZE: f32 = 64.0;

/// Default collision layer bitmask matching all layers.
pub const COLLISION_LAYER_ALL: u32 = u32::MAX;

/// Collision layer bit for player entities.
pub const COLLISION_LAYER_PLAYER: u32 = 1 << 0;

/// Collision layer bit for enemy entities.
pub const COLLISION_LAYER_ENEMY: u32 = 1 << 1;

/// Collision layer bit for projectiles.
pub const COLLISION_LAYER_PROJECTILE: u32 = 1 << 2;

/// Collision layer bit for static arena obstacles and walls.
pub const COLLISION_LAYER_WORLD: u32 = 1 << 3;

/// Collision layer bit for collectible pickups.
pub const COLLISION_LAYER_PICKUP: u32 = 1 << 4;

/// Combined collision mask for player interactions with enemies, obstacles, and pickups.
pub const COLLISION_MASK_PLAYER_DEFAULT: u32 =
    COLLISION_LAYER_ENEMY + COLLISION_LAYER_WORLD + COLLISION_LAYER_PICKUP;

/// Maximum dense flat grid cell count (`128x128`) before falling back to open-addressed sparse hashing.
pub(crate) const FLAT_GRID_MAX_CELLS: usize = 16_384;

/// Maximum single-axis span in cells eligible for direct dense flat grid indexing.
pub(crate) const FLAT_GRID_MAX_AXIS: i64 = 256;

/// Multiplicative mixing constant for sparse flat spatial-hash bucket indexing.
pub(crate) const SPARSE_GRID_HASH_MULT: u64 = 0x517C_C1B7_2722_0A95;
