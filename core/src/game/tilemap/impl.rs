use super::*;

/// Tile lookup, mutation, and world-space queries for `TileMap`.
impl TileMap {
    /// Creates an empty `columns x rows` tilemap with cell size `tile_size`.
    ///
    /// # Arguments
    ///
    /// - `usize` - Number of columns.
    /// - `usize` - Number of rows.
    /// - `f32` - Tile width/height in world units.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized tilemap with default air entry at index `0`.
    #[must_use]
    pub fn empty(columns: usize, rows: usize, tile_size: f32) -> Self {
        let c: usize = columns.max(1);
        let r: usize = rows.max(1);
        Self {
            columns: c,
            rows: r,
            tile_size: tile_size.max(4.0),
            cells: vec![0; c.saturating_mul(r)],
            palette: vec![TilePaletteEntry::new(
                COLOR_TRANSPARENT,
                COLOR_TRANSPARENT,
                false,
            )],
        }
    }

    /// Registers a new palette entry and returns its assigned tile ID.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable tilemap.
    /// - `TilePaletteEntry` - Visual and solidity properties.
    ///
    /// # Returns
    ///
    /// - `u16` - Assigned tile identifier.
    pub fn add_palette_entry(&mut self, entry: TilePaletteEntry) -> u16 {
        let id: u16 = self.get_palette().len() as u16;
        self.get_mut_palette().push(entry);
        id
    }

    /// Sets the tile identifier at grid coordinate `(col, row)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable tilemap.
    /// - `usize` - Column index.
    /// - `usize` - Row index.
    /// - `u16` - Tile identifier.
    pub fn set_tile(&mut self, col: usize, row: usize, tile_id: u16) {
        if col < self.get_columns() && row < self.get_rows() {
            let idx: usize = row * self.get_columns() + col;
            self.get_mut_cells()[idx] = tile_id;
        }
    }

    /// Reads the tile identifier at `(col, row)`, returning `0` when out of bounds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Tilemap reference.
    /// - `usize` - Column index.
    /// - `usize` - Row index.
    ///
    /// # Returns
    ///
    /// - `u16` - Tile ID at `(col, row)`.
    #[must_use]
    pub fn tile_at(&self, col: usize, row: usize) -> u16 {
        if col < self.get_columns() && row < self.get_rows() {
            self.get_cells()[row * self.get_columns() + col]
        } else {
            0
        }
    }

    /// Returns `true` if the world-space point `world_pt` falls inside a solid tile.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Tilemap reference.
    /// - `Point` - World-space query coordinate.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the tile at `world_pt` is solid.
    #[must_use]
    pub fn is_solid_at_world(&self, world_pt: Point) -> bool {
        if world_pt.get_x() < 0.0 || world_pt.get_y() < 0.0 {
            return false;
        }
        let col: usize = (world_pt.get_x() / self.get_tile_size()).floor() as usize;
        let row: usize = (world_pt.get_y() / self.get_tile_size()).floor() as usize;
        let tid: u16 = self.tile_at(col, row);
        self.get_palette()
            .get(tid as usize)
            .map(TilePaletteEntry::get_solid)
            .unwrap_or(false)
    }

    /// Returns total world-space dimensions `(width, height)` of the tilemap.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Tilemap reference.
    ///
    /// # Returns
    ///
    /// - `Rect` - World bounds rectangle starting at `(0, 0)`.
    #[must_use]
    pub fn world_bounds(&self) -> Rect {
        Rect::new(
            0.0,
            0.0,
            (self.get_columns() as f32) * self.get_tile_size(),
            (self.get_rows() as f32) * self.get_tile_size(),
        )
    }
}

/// Frame slicing helpers for `SpriteSheet`.
impl SpriteSheet {
    /// Creates a `SpriteSheet` over `texture` with fixed `(frame_width, frame_height)` cells.
    ///
    /// # Arguments
    ///
    /// - `ImageBuffer` - Backing RGBA8 sprite atlas image.
    /// - `u32` - Width of each frame in pixels.
    /// - `u32` - Height of each frame in pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed sprite sheet.
    #[must_use]
    pub fn from_grid(texture: ImageBuffer, frame_width: u32, frame_height: u32) -> Self {
        let fw: u32 = frame_width.max(1);
        let fh: u32 = frame_height.max(1);
        let cols: u32 = (texture.get_width() / fw).max(1);
        Self {
            texture,
            frame_width: fw,
            frame_height: fh,
            columns: cols,
        }
    }

    /// Computes the source pixel sub-rectangle for frame index `frame_index`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Sprite sheet reference.
    /// - `u32` - Zero-based frame index.
    ///
    /// # Returns
    ///
    /// - `Rect` - Pixel sub-rectangle `(x, y, frame_width, frame_height)`.
    #[must_use]
    pub fn frame_rect(&self, frame_index: u32) -> Rect {
        let col: u32 = frame_index % self.get_columns().max(1);
        let row: u32 = frame_index / self.get_columns().max(1);
        Rect::new(
            (col * self.get_frame_width()) as f32,
            (row * self.get_frame_height()) as f32,
            self.get_frame_width() as f32,
            self.get_frame_height() as f32,
        )
    }
}

/// Playback stepping and current-frame queries for `SpriteAnimator`.
impl SpriteAnimator {
    /// Advances animation playback clock by `dt_secs` and returns the active frame index.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable sprite animator.
    /// - `f32` - Elapsed time delta in seconds.
    ///
    /// # Returns
    ///
    /// - `u32` - Active sprite sheet frame index.
    pub fn tick(&mut self, dt_secs: f32) -> u32 {
        let next: f32 = self.get_elapsed_secs() + dt_secs.max(0.0);
        self.set_elapsed_secs(next);
        self.current_frame()
    }

    /// Computes the active frame index from `elapsed_secs`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Sprite animator reference.
    ///
    /// # Returns
    ///
    /// - `u32` - Current sprite sheet frame index.
    #[must_use]
    pub fn current_frame(&self) -> u32 {
        let count: u32 = self.get_frame_count().max(1);
        let raw: u32 = (self.get_elapsed_secs() * self.get_fps().max(0.1)).floor() as u32;
        let offset: u32 = if self.get_looping() {
            raw % count
        } else {
            raw.min(count - 1)
        };
        self.get_start_frame().saturating_add(offset)
    }
}
