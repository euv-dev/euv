use super::*;

/// Implements construction, insertion, and querying for `SpatialHashGrid2D`.
impl SpatialHashGrid2D {
    /// Creates a new 2D spatial hash grid with the given cell size.
    ///
    /// A cell size that is not finite and strictly positive cannot produce a
    /// usable grid: clamping it up to [`EPSILON`] leaves an inverse cell size
    /// of `1e6`, so a body of ordinary size maps to a column range in the
    /// millions and a single `insert` or `query` degenerates into a loop over
    /// billions of empty cells. Such a request is treated as "no cell size
    /// given" and falls back to [`SPATIAL_DEFAULT_CELL_SIZE_2D`], which keeps
    /// both the grid and the per-operation work bounded.
    ///
    /// # Arguments
    ///
    /// - `f64` - The world-space size of each grid cell.
    ///
    /// # Returns
    ///
    /// - `SpatialHashGrid2D` - The new grid.
    pub fn create(cell_size: f64) -> SpatialHashGrid2D {
        let safe_size: f64 = Self::usable_cell_size(cell_size, SPATIAL_DEFAULT_CELL_SIZE_2D);
        let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::new(safe_size);
        grid.set_inverse_cell_size(1.0 / safe_size);
        grid
    }

    /// Returns `cell_size` when it is finite and strictly positive, and
    /// `fallback` otherwise.
    ///
    /// Shared by the 2D and 3D grid constructors so both reject a degenerate
    /// cell size identically.
    ///
    /// # Arguments
    ///
    /// - `f64` - The requested cell size.
    /// - `f64` - The cell size to use when the request is unusable.
    ///
    /// # Returns
    ///
    /// - `f64` - A cell size that yields a bounded column range.
    fn usable_cell_size(cell_size: f64, fallback: f64) -> f64 {
        if cell_size.is_finite() && cell_size > 0.0 {
            cell_size
        } else {
            fallback
        }
    }

    /// Creates a new 2D spatial hash grid with the default cell size.
    ///
    /// # Returns
    ///
    /// - `SpatialHashGrid2D` - The new grid.
    pub fn with_default_size() -> SpatialHashGrid2D {
        Self::create(SPATIAL_DEFAULT_CELL_SIZE_2D)
    }

    /// Inserts a body index into all cells overlapping the given bounding box.
    ///
    /// # Arguments
    ///
    /// - `usize` - The body index to insert.
    /// - `Vector2D` - The minimum corner of the bounding box.
    /// - `Vector2D` - The maximum corner of the bounding box.
    pub fn insert(&mut self, index: usize, min: Vector2D, max: Vector2D) {
        let inv: f64 = self.get_inverse_cell_size();
        let min_col: i32 = (min.get_x() * inv).floor() as i32;
        let min_row: i32 = (min.get_y() * inv).floor() as i32;
        let max_col: i32 = (max.get_x() * inv).floor() as i32;
        let max_row: i32 = (max.get_y() * inv).floor() as i32;
        for col in min_col..=max_col {
            for row in min_row..=max_row {
                self.get_mut_cells()
                    .entry((col, row))
                    .or_default()
                    .push(index);
            }
        }
    }

    /// Returns all candidate body indices whose cells overlap the given bounding box.
    ///
    /// Deduplicates indices so each candidate appears at most once.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The minimum corner of the query box.
    /// - `Vector2D` - The maximum corner of the query box.
    ///
    /// # Returns
    ///
    /// - `Vec<usize>` - The list of candidate body indices.
    pub fn query(&self, min: Vector2D, max: Vector2D) -> Vec<usize> {
        let inv: f64 = self.get_inverse_cell_size();
        let min_col: i32 = (min.get_x() * inv).floor() as i32;
        let min_row: i32 = (min.get_y() * inv).floor() as i32;
        let max_col: i32 = (max.get_x() * inv).floor() as i32;
        let max_row: i32 = (max.get_y() * inv).floor() as i32;
        let mut seen: HashSet<usize> = HashSet::new();
        let mut result: Vec<usize> = Vec::new();
        for col in min_col..=max_col {
            for row in min_row..=max_row {
                if let Some(entries) = self.get_cells().get(&(col, row)) {
                    for index in entries {
                        if seen.insert(*index) {
                            result.push(*index);
                        }
                    }
                }
            }
        }
        result
    }

    /// Removes all entries from the grid, preparing it for a fresh insertion pass.
    ///
    /// Cells that held bodies during the just-finished pass keep their key and
    /// their underlying `Vec` buffer, so a body that stays put pays no
    /// allocation on the next tick. Cells that ended the pass empty are dropped
    /// outright: without that, a scene whose bodies roam across a large world
    /// accumulates one permanent map entry per cell ever visited, and both the
    /// map and the per-tick clear below it grow without bound for the lifetime
    /// of the process. Dropping on empty bounds the map to the cells the
    /// current pass actually populates, at the cost of one small `Vec`
    /// allocation for a cell a body leaves and then re-enters.
    pub fn clear(&mut self) {
        // Preserve each occupied cell's underlying Vec buffer across frames so
        // the spatial hash doesn't pay a fresh allocation cost on every tick,
        // while reclaiming the keys of cells that went empty.
        let cells: &mut SpatialCellMap2D = self.get_mut_cells();
        cells.retain(|_, entries: &mut Vec<usize>| !entries.is_empty());
        cells.values_mut().for_each(Vec::clear);
    }

    /// Appends all candidate body indices overlapping the query box into `out`,
    /// deduplicating via the caller-provided `seen` set.
    ///
    /// Both `out` and `seen` are cleared first, so the caller can reuse the same
    /// buffers across all queries in a step without any per-query allocation.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The minimum corner of the query box.
    /// - `Vector2D` - The maximum corner of the query box.
    /// - `&mut Vec<usize>` - The output buffer, cleared then filled with candidates.
    /// - `&mut HashSet<usize>` - The dedup scratch set, cleared then reused.
    pub fn query_into(
        &self,
        min: Vector2D,
        max: Vector2D,
        out: &mut Vec<usize>,
        seen: &mut HashSet<usize>,
    ) {
        out.clear();
        seen.clear();
        let inv: f64 = self.get_inverse_cell_size();
        let min_col: i32 = (min.get_x() * inv).floor() as i32;
        let min_row: i32 = (min.get_y() * inv).floor() as i32;
        let max_col: i32 = (max.get_x() * inv).floor() as i32;
        let max_row: i32 = (max.get_y() * inv).floor() as i32;
        for col in min_col..=max_col {
            for row in min_row..=max_row {
                if let Some(entries) = self.get_cells().get(&(col, row)) {
                    for index in entries {
                        if seen.insert(*index) {
                            out.push(*index);
                        }
                    }
                }
            }
        }
    }
}

/// Implements construction, insertion, and querying for `SpatialHashGrid3D`.
impl SpatialHashGrid3D {
    /// Creates a new 3D spatial hash grid with the given cell size.
    ///
    /// A cell size that is not finite and strictly positive is rejected in
    /// favour of [`SPATIAL_DEFAULT_CELL_SIZE_3D`], for the same reason as
    /// [`SpatialHashGrid2D::create`]: a near-zero cell size makes the column
    /// range of an ordinary body span millions of cells.
    ///
    /// # Arguments
    ///
    /// - `f64` - The world-space size of each grid cell.
    ///
    /// # Returns
    ///
    /// - `SpatialHashGrid3D` - The new grid.
    pub fn create(cell_size: f64) -> SpatialHashGrid3D {
        let safe_size: f64 =
            SpatialHashGrid2D::usable_cell_size(cell_size, SPATIAL_DEFAULT_CELL_SIZE_3D);
        let mut grid: SpatialHashGrid3D = SpatialHashGrid3D::new(safe_size);
        grid.set_inverse_cell_size(1.0 / safe_size);
        grid
    }

    /// Creates a new 3D spatial hash grid with the default cell size.
    ///
    /// # Returns
    ///
    /// - `SpatialHashGrid3D` - The new grid.
    pub fn with_default_size() -> SpatialHashGrid3D {
        Self::create(SPATIAL_DEFAULT_CELL_SIZE_3D)
    }

    /// Inserts a body index into all cells overlapping the given 3D bounding box.
    ///
    /// # Arguments
    ///
    /// - `usize` - The body index to insert.
    /// - `Vector3D` - The minimum corner of the bounding box.
    /// - `Vector3D` - The maximum corner of the bounding box.
    pub fn insert(&mut self, index: usize, min: Vector3D, max: Vector3D) {
        let inv: f64 = self.get_inverse_cell_size();
        let min_col: i32 = (min.get_x() * inv).floor() as i32;
        let min_row: i32 = (min.get_y() * inv).floor() as i32;
        let min_layer: i32 = (min.get_z() * inv).floor() as i32;
        let max_col: i32 = (max.get_x() * inv).floor() as i32;
        let max_row: i32 = (max.get_y() * inv).floor() as i32;
        let max_layer: i32 = (max.get_z() * inv).floor() as i32;
        for col in min_col..=max_col {
            for row in min_row..=max_row {
                for layer in min_layer..=max_layer {
                    self.get_mut_cells()
                        .entry((col, row, layer))
                        .or_default()
                        .push(index);
                }
            }
        }
    }

    /// Returns all candidate body indices whose cells overlap the given 3D bounding box.
    ///
    /// Deduplicates indices so each candidate appears at most once.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The minimum corner of the query box.
    /// - `Vector3D` - The maximum corner of the query box.
    ///
    /// # Returns
    ///
    /// - `Vec<usize>` - The list of candidate body indices.
    pub fn query(&self, min: Vector3D, max: Vector3D) -> Vec<usize> {
        let inv: f64 = self.get_inverse_cell_size();
        let min_col: i32 = (min.get_x() * inv).floor() as i32;
        let min_row: i32 = (min.get_y() * inv).floor() as i32;
        let min_layer: i32 = (min.get_z() * inv).floor() as i32;
        let max_col: i32 = (max.get_x() * inv).floor() as i32;
        let max_row: i32 = (max.get_y() * inv).floor() as i32;
        let max_layer: i32 = (max.get_z() * inv).floor() as i32;
        let mut seen: HashSet<usize> = HashSet::new();
        let mut result: Vec<usize> = Vec::new();
        for col in min_col..=max_col {
            for row in min_row..=max_row {
                for layer in min_layer..=max_layer {
                    if let Some(entries) = self.get_cells().get(&(col, row, layer)) {
                        for index in entries {
                            if seen.insert(*index) {
                                result.push(*index);
                            }
                        }
                    }
                }
            }
        }
        result
    }

    /// Removes all entries from the grid, preparing it for a fresh insertion pass.
    ///
    /// Cells that held bodies during the just-finished pass keep their key and
    /// their underlying `Vec` buffer, so a body that stays put pays no
    /// allocation on the next tick. Cells that ended the pass empty are dropped
    /// outright: without that, a scene whose bodies roam across a large world
    /// accumulates one permanent map entry per cell ever visited, and both the
    /// map and the per-tick clear below it grow without bound for the lifetime
    /// of the process. Dropping on empty bounds the map to the cells the
    /// current pass actually populates, at the cost of one small `Vec`
    /// allocation for a cell a body leaves and then re-enters.
    pub fn clear(&mut self) {
        // Preserve each occupied cell's underlying Vec buffer across frames so
        // the spatial hash doesn't pay a fresh allocation cost on every tick,
        // while reclaiming the keys of cells that went empty.
        let cells: &mut SpatialCellMap3D = self.get_mut_cells();
        cells.retain(|_, entries: &mut Vec<usize>| !entries.is_empty());
        cells.values_mut().for_each(Vec::clear);
    }

    /// Appends all candidate body indices overlapping the query box into `out`,
    /// deduplicating via the caller-provided `seen` set.
    ///
    /// Both `out` and `seen` are cleared first, so the caller can reuse the same
    /// buffers across all queries in a step without any per-query allocation.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The minimum corner of the query box.
    /// - `Vector3D` - The maximum corner of the query box.
    /// - `&mut Vec<usize>` - The output buffer, cleared then filled with candidates.
    /// - `&mut HashSet<usize>` - The dedup scratch set, cleared then reused.
    pub fn query_into(
        &self,
        min: Vector3D,
        max: Vector3D,
        out: &mut Vec<usize>,
        seen: &mut HashSet<usize>,
    ) {
        out.clear();
        seen.clear();
        let inv: f64 = self.get_inverse_cell_size();
        let min_col: i32 = (min.get_x() * inv).floor() as i32;
        let min_row: i32 = (min.get_y() * inv).floor() as i32;
        let min_layer: i32 = (min.get_z() * inv).floor() as i32;
        let max_col: i32 = (max.get_x() * inv).floor() as i32;
        let max_row: i32 = (max.get_y() * inv).floor() as i32;
        let max_layer: i32 = (max.get_z() * inv).floor() as i32;
        for col in min_col..=max_col {
            for row in min_row..=max_row {
                for layer in min_layer..=max_layer {
                    if let Some(entries) = self.get_cells().get(&(col, row, layer)) {
                        for index in entries {
                            if seen.insert(*index) {
                                out.push(*index);
                            }
                        }
                    }
                }
            }
        }
    }
}
/// Default-construction for [`SpatialHashGrid2D`].
impl Default for SpatialHashGrid2D {
    /// Constructs a default [`SpatialHashGrid2D`] value.
    ///
    /// # Returns
    ///
    /// - `SpatialHashGrid2D` - A default-constructed instance with the documented initial state.
    fn default() -> SpatialHashGrid2D {
        SpatialHashGrid2D::with_default_size()
    }
}

/// Implements `Default` for `SpatialHashGrid3D` with the default cell size.
impl Default for SpatialHashGrid3D {
    /// Constructs a default [`SpatialHashGrid3D`] value.
    ///
    /// # Returns
    ///
    /// - `SpatialHashGrid3D` - A default-constructed instance with the documented initial state.
    fn default() -> SpatialHashGrid3D {
        SpatialHashGrid3D::with_default_size()
    }
}
/// Implements construction, insertion, subdivision, and querying for `QuadTree2D`.
impl QuadTree2D {
    /// Creates a new 2D quadtree rooted at the given region.
    ///
    /// The root region is normalized so the first corner is always the minimum,
    /// and a zero-width or zero-height region is widened by [`EPSILON`] so
    /// subdivision always makes progress. A capacity below one is raised to
    /// one, and a depth above [`SPATIAL_MAX_DEPTH_2D`] is clamped.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - One corner of the root region.
    /// - `Vector2D` - The opposite corner of the root region.
    /// - `usize` - The number of entries a node may hold before subdividing.
    /// - `usize` - The deepest subdivision level a node may reach.
    ///
    /// # Returns
    ///
    /// - `QuadTree2D` - The new empty quadtree.
    pub fn create(
        bounds_min: Vector2D,
        bounds_max: Vector2D,
        capacity: usize,
        max_depth: usize,
    ) -> QuadTree2D {
        let min: Vector2D = Vector2D::new(
            bounds_min.get_x().min(bounds_max.get_x()),
            bounds_min.get_y().min(bounds_max.get_y()),
        );
        let max: Vector2D = Vector2D::new(
            bounds_min.get_x().max(bounds_max.get_x()) + EPSILON,
            bounds_min.get_y().max(bounds_max.get_y()) + EPSILON,
        );
        let root: QuadTreeNode2D = QuadTreeNode2D::new(
            min,
            max,
            0,
            [SPATIAL_QUAD_TREE_NO_CHILD; 4],
            true,
            false,
            Vec::new(),
        );
        QuadTree2D {
            nodes: vec![root],
            capacity: capacity.max(1),
            max_depth: max_depth.min(SPATIAL_MAX_DEPTH_2D),
            count: 0,
        }
    }

    /// Creates a new 2D quadtree with the default capacity, depth, and a root
    /// region centred on the world origin.
    ///
    /// # Returns
    ///
    /// - `QuadTree2D` - The new empty quadtree.
    pub fn with_default_size() -> QuadTree2D {
        let extent: f64 = SPATIAL_DEFAULT_HALF_EXTENT_2D;
        QuadTree2D::create(
            Vector2D::new(-extent, -extent),
            Vector2D::new(extent, extent),
            SPATIAL_DEFAULT_CAPACITY_2D,
            SPATIAL_DEFAULT_MAX_DEPTH_2D,
        )
    }

    /// Creates a new 2D quadtree over a square region of the given half-extent,
    /// using the default capacity and depth.
    ///
    /// # Arguments
    ///
    /// - `f64` - The half-extent of the root region in world units.
    ///
    /// # Returns
    ///
    /// - `QuadTree2D` - The new empty quadtree.
    pub fn with_half_extent(half_extent: f64) -> QuadTree2D {
        let extent: f64 = half_extent.abs().max(EPSILON);
        QuadTree2D::create(
            Vector2D::new(-extent, -extent),
            Vector2D::new(extent, extent),
            SPATIAL_DEFAULT_CAPACITY_2D,
            SPATIAL_DEFAULT_MAX_DEPTH_2D,
        )
    }

    /// Returns the number of bodies currently stored in the tree.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of inserted bodies.
    pub fn len(&self) -> usize {
        self.get_count()
    }

    /// Reports whether the tree holds no bodies.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when no body is stored.
    pub fn is_empty(&self) -> bool {
        self.get_count() == 0
    }

    /// Returns the region covered by the root node.
    ///
    /// # Returns
    ///
    /// - `(Vector2D, Vector2D)` - The minimum and maximum corners of the root region.
    pub fn bounds(&self) -> (Vector2D, Vector2D) {
        let root: &QuadTreeNode2D = &self.get_nodes()[SPATIAL_QUAD_TREE_ROOT_INDEX];
        (root.get_min(), root.get_max())
    }

    /// Inserts a body index together with its exact bounding box.
    ///
    /// The entry descends into the child region that fully contains it. An
    /// entry that straddles a split boundary, or that is larger than the node
    /// it lands in, stays in the current node and marks that node `loose` —
    /// that is the standard quadtree invariant, and it is what guarantees a
    /// body is stored exactly once, so queries never need dedup beyond the
    /// caller's scratch set.
    ///
    /// # Arguments
    ///
    /// - `usize` - The body index to insert.
    /// - `Vector2D` - The minimum corner of the bounding box.
    /// - `Vector2D` - The maximum corner of the bounding box.
    pub fn insert(&mut self, index: usize, min: Vector2D, max: Vector2D) {
        let entry: QuadTreeEntry2D = QuadTreeEntry2D::new(index, min, max);
        let mut handle: usize = SPATIAL_QUAD_TREE_ROOT_INDEX;
        loop {
            let node: &QuadTreeNode2D = &self.get_nodes()[handle];
            if node.get_leaf() {
                break;
            }
            let next: usize = QuadTree2D::child_containing(self.get_nodes(), handle, &entry);
            if next == SPATIAL_QUAD_TREE_NO_CHILD {
                break;
            }
            handle = next;
        }
        if !QuadTree2D::contains_box(
            self.get_nodes()[handle].get_min(),
            self.get_nodes()[handle].get_max(),
            entry.get_min(),
            entry.get_max(),
        ) {
            self.get_mut_nodes()[handle].set_loose(true);
        }
        self.get_mut_nodes()[handle].get_mut_entries().push(entry);
        let capacity: usize = self.get_capacity();
        let max_depth: usize = self.get_max_depth();
        QuadTree2D::subdivide(self.get_mut_nodes(), handle, capacity, max_depth);
        let count: usize = self.get_count();
        self.set_count(count + 1);
    }

    /// Returns all body indices whose exact bounding box overlaps the query box.
    ///
    /// Each candidate index appears at most once.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The minimum corner of the query box.
    /// - `Vector2D` - The maximum corner of the query box.
    ///
    /// # Returns
    ///
    /// - `Vec<usize>` - The list of candidate body indices.
    pub fn query(&self, min: Vector2D, max: Vector2D) -> Vec<usize> {
        let mut out: Vec<usize> = Vec::new();
        let mut seen: HashSet<usize> = HashSet::new();
        self.query_into(min, max, &mut out, &mut seen);
        out
    }

    /// Appends all candidate body indices overlapping the query box into `out`,
    /// deduplicating via the caller-provided `seen` set.
    ///
    /// Both `out` and `seen` are cleared first, so the caller can reuse the same
    /// buffers across all queries in a step without any per-query allocation.
    ///
    /// Nodes are pruned by region overlap, then every entry that survives
    /// pruning is confirmed with an exact box-vs-box test. A node holding a body
    /// larger than its own region is marked `loose` and skips the region prune
    /// entirely, which is what keeps such a body — including one larger than the
    /// whole root region — returned by every query that overlaps it. The
    /// traversal uses an explicit stack, so no recursion is involved.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The minimum corner of the query box.
    /// - `Vector2D` - The maximum corner of the query box.
    /// - `&mut Vec<usize>` - The output buffer, cleared then filled with candidates.
    /// - `&mut HashSet<usize>` - The dedup scratch set, cleared then reused.
    pub fn query_into(
        &self,
        min: Vector2D,
        max: Vector2D,
        out: &mut Vec<usize>,
        seen: &mut HashSet<usize>,
    ) {
        out.clear();
        seen.clear();
        let mut stack: QuadTreeNodeStack2D = Vec::new();
        stack.push(SPATIAL_QUAD_TREE_ROOT_INDEX);
        while let Some(handle) = stack.pop() {
            let node: &QuadTreeNode2D = &self.get_nodes()[handle];
            if !node.get_loose()
                && !QuadTree2D::boxes_overlap(node.get_min(), node.get_max(), min, max)
            {
                continue;
            }
            for entry in node.get_entries() {
                if !QuadTree2D::boxes_overlap(entry.get_min(), entry.get_max(), min, max) {
                    continue;
                }
                let index: usize = entry.get_index();
                if seen.insert(index) {
                    out.push(index);
                }
            }
            if !node.get_leaf() {
                for &child in node.get_children().iter() {
                    stack.push(child);
                }
            }
        }
    }

    /// Removes every entry and child node, keeping only the empty root so the
    /// same tree can be reused for a fresh insertion pass.
    pub fn clear(&mut self) {
        let mut root: QuadTreeNode2D = self.get_nodes()[SPATIAL_QUAD_TREE_ROOT_INDEX].clone();
        root.set_children([SPATIAL_QUAD_TREE_NO_CHILD; 4]);
        root.set_leaf(true);
        root.set_loose(false);
        root.get_mut_entries().clear();
        self.get_mut_nodes().clear();
        self.get_mut_nodes().push(root);
        self.set_count(0);
    }

    /// Reports whether two axis-aligned boxes overlap.
    ///
    /// Touching edges count as overlapping, matching the inclusive cell
    /// semantics of [`SpatialHashGrid2D::query`].
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The minimum corner of the first box.
    /// - `Vector2D` - The maximum corner of the first box.
    /// - `Vector2D` - The minimum corner of the second box.
    /// - `Vector2D` - The maximum corner of the second box.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the two boxes share at least one point.
    pub fn boxes_overlap(
        a_min: Vector2D,
        a_max: Vector2D,
        b_min: Vector2D,
        b_max: Vector2D,
    ) -> bool {
        a_min.get_x() <= b_max.get_x()
            && b_min.get_x() <= a_max.get_x()
            && a_min.get_y() <= b_max.get_y()
            && b_min.get_y() <= a_max.get_y()
    }

    /// Returns the child handle whose region fully contains the entry, or
    /// [`SPATIAL_QUAD_TREE_NO_CHILD`] when the entry straddles the split.
    ///
    /// # Arguments
    ///
    /// - `&QuadTreeNodeList2D` - The node arena holding the child regions.
    /// - `usize` - The handle of the subdivided parent node.
    /// - `&QuadTreeEntry2D` - The entry being placed.
    ///
    /// # Returns
    ///
    /// - `usize` - The containing child handle, or `SPATIAL_QUAD_TREE_NO_CHILD`.
    fn child_containing(
        nodes: &QuadTreeNodeList2D,
        handle: usize,
        entry: &QuadTreeEntry2D,
    ) -> usize {
        let children: &QuadTreeChildren2D = nodes[handle].get_children();
        for &child in children.iter() {
            if child == SPATIAL_QUAD_TREE_NO_CHILD {
                continue;
            }
            let region: &QuadTreeNode2D = &nodes[child];
            if QuadTree2D::contains_box(
                region.get_min(),
                region.get_max(),
                entry.get_min(),
                entry.get_max(),
            ) {
                return child;
            }
        }
        SPATIAL_QUAD_TREE_NO_CHILD
    }

    /// Reports whether the first region fully contains the second box.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The minimum corner of the containing region.
    /// - `Vector2D` - The maximum corner of the containing region.
    /// - `Vector2D` - The minimum corner of the contained box.
    /// - `Vector2D` - The maximum corner of the contained box.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the region covers the box on both axes.
    fn contains_box(
        outer_min: Vector2D,
        outer_max: Vector2D,
        inner_min: Vector2D,
        inner_max: Vector2D,
    ) -> bool {
        outer_min.get_x() <= inner_min.get_x()
            && outer_min.get_y() <= inner_min.get_y()
            && outer_max.get_x() >= inner_max.get_x()
            && outer_max.get_y() >= inner_max.get_y()
    }

    /// Splits the node at `handle` into four children when it is over capacity
    /// and still below the maximum depth, redistributing the entries it holds.
    ///
    /// Entries that fit entirely inside one child move down; everything else
    /// stays in the parent and marks it `loose`. A node whose depth already
    /// equals the maximum depth is left alone, so a pile of coincident bodies
    /// terminates instead of subdividing forever.
    ///
    /// # Arguments
    ///
    /// - `&mut QuadTreeNodeList2D` - The mutable node arena to subdivide in place.
    /// - `usize` - The handle of the node to subdivide.
    /// - `usize` - The per-node entry capacity.
    /// - `usize` - The deepest subdivision level allowed.
    fn subdivide(nodes: &mut QuadTreeNodeList2D, handle: usize, capacity: usize, max_depth: usize) {
        if !nodes[handle].get_leaf() || nodes[handle].get_depth() >= max_depth {
            return;
        }
        if nodes[handle].get_entries().len() <= capacity {
            return;
        }
        // Phase 1: take the parent's entries out and materialize the four child
        // regions. Draining first is what keeps a straddling entry from being
        // stored twice, once here and once in the child that accepted it.
        let drained: QuadTreeEntryList2D = mem::take(nodes[handle].get_mut_entries());
        let depth: usize = nodes[handle].get_depth() + 1;
        let mut mid: Vector2D = Vector2D::new(
            nodes[handle].get_min().get_x()
                + (nodes[handle].get_max().get_x() - nodes[handle].get_min().get_x())
                    * SPATIAL_QUAD_TREE_MID_RATIO,
            nodes[handle].get_min().get_y()
                + (nodes[handle].get_max().get_y() - nodes[handle].get_min().get_y())
                    * SPATIAL_QUAD_TREE_MID_RATIO,
        );
        if mid.get_x() <= nodes[handle].get_min().get_x() {
            mid = Vector2D::new(nodes[handle].get_min().get_x() + EPSILON, mid.get_y());
        }
        if mid.get_y() <= nodes[handle].get_min().get_y() {
            mid = Vector2D::new(mid.get_x(), nodes[handle].get_min().get_y() + EPSILON);
        }
        let regions: [QuadTreeNode2D; 4] = [
            QuadTreeNode2D::new(
                nodes[handle].get_min(),
                Vector2D::new(mid.get_x(), mid.get_y()),
                depth,
                [SPATIAL_QUAD_TREE_NO_CHILD; 4],
                true,
                false,
                Vec::new(),
            ),
            QuadTreeNode2D::new(
                Vector2D::new(mid.get_x(), nodes[handle].get_min().get_y()),
                Vector2D::new(nodes[handle].get_max().get_x(), mid.get_y()),
                depth,
                [SPATIAL_QUAD_TREE_NO_CHILD; 4],
                true,
                false,
                Vec::new(),
            ),
            QuadTreeNode2D::new(
                Vector2D::new(nodes[handle].get_min().get_x(), mid.get_y()),
                Vector2D::new(mid.get_x(), nodes[handle].get_max().get_y()),
                depth,
                [SPATIAL_QUAD_TREE_NO_CHILD; 4],
                true,
                false,
                Vec::new(),
            ),
            QuadTreeNode2D::new(
                mid,
                nodes[handle].get_max(),
                depth,
                [SPATIAL_QUAD_TREE_NO_CHILD; 4],
                true,
                false,
                Vec::new(),
            ),
        ];
        let mut handles: QuadTreeChildren2D = [SPATIAL_QUAD_TREE_NO_CHILD; 4];
        for (slot, region) in handles.iter_mut().zip(regions.iter()) {
            *slot = nodes.len();
            nodes.push(region.clone());
        }
        // Phase 2: demote the parent, then push every fully contained entry one
        // level down and return the rest to the parent.
        nodes[handle].set_children(handles);
        nodes[handle].set_leaf(false);
        for entry in drained.iter() {
            let mut target: Option<usize> = None;
            for &child in handles.iter() {
                let region: &QuadTreeNode2D = &nodes[child];
                if QuadTree2D::contains_box(
                    region.get_min(),
                    region.get_max(),
                    entry.get_min(),
                    entry.get_max(),
                ) {
                    target = Some(child);
                    break;
                }
            }
            match target {
                Some(child) => {
                    nodes[child].get_mut_entries().push(*entry);
                }
                None => {
                    nodes[handle].set_loose(true);
                    nodes[handle].get_mut_entries().push(*entry);
                }
            }
        }
        // Phase 3: a child that is itself over capacity splits immediately, so
        // one insert call always leaves the whole subtree consistent.
        for &child in handles.iter() {
            QuadTree2D::subdivide(nodes, child, capacity, max_depth);
        }
    }
}

/// Default-construction for [`QuadTree2D`].
impl Default for QuadTree2D {
    /// Constructs a default [`QuadTree2D`] value.
    ///
    /// # Returns
    ///
    /// - `QuadTree2D` - A default-constructed instance with the documented initial state.
    fn default() -> QuadTree2D {
        QuadTree2D::with_default_size()
    }
}
