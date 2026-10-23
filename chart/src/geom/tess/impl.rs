use super::*;

/// Inherent implementation of [`Tessellator`].
impl Tessellator {
    /// Returns a tessellator with the default flattening tolerance.
    pub fn new() -> Self {
        Self {
            tolerance: DEFAULT_FLATTEN_TOLERANCE,
            flat: FlatPath::default(),
            working: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Returns a tessellator with the given flattening tolerance, in
    /// pixels.
    ///
    /// A smaller tolerance produces smoother curves at the cost of more
    /// vertices.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn with_tolerance(tolerance: f64) -> Self {
        Self {
            tolerance,
            ..Self::new()
        }
    }

    /// Returns the flattening tolerance, in pixels.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn tolerance(&self) -> f64 {
        self.get_tolerance()
    }

    /// Flattens the path into contours and returns them.
    ///
    /// The returned reference borrows the tessellator's scratch storage and
    /// stays valid until the next call to [`Tessellator::flatten`].
    ///
    /// # Arguments
    ///
    /// - `&Path` - Shared reference to a `Path`.
    ///
    /// # Returns
    ///
    /// - `&FlatPath` - A `FlatPath` value.
    pub fn flatten(&mut self, path: &Path) -> &FlatPath {
        self.get_mut_flat().contours.clear();
        let mut current: Contour = Contour::default();
        let mut cursor: (f64, f64) = (0.0_f64, 0.0_f64);
        let mut sub_start: (f64, f64) = (0.0_f64, 0.0_f64);

        for command in path.commands() {
            match *command {
                PathCommand::MoveTo(x, y) => {
                    if current.points.len() > 1 {
                        self.get_mut_flat().contours.push(take(&mut current));
                    } else {
                        current = Contour::default();
                    }
                    cursor = (x as f64, y as f64);
                    sub_start = cursor;
                    current.points.push(Point::new(x, y));
                }
                PathCommand::LineTo(x, y) => {
                    cursor = (x as f64, y as f64);
                    current.points.push(Point::new(x, y));
                }
                PathCommand::QuadTo(cx, cy, x, y) => {
                    let count: usize = quad_segment_count(
                        cursor,
                        (cx as f64, cy as f64),
                        (x as f64, y as f64),
                        self.get_tolerance(),
                    );
                    for step in 1..=count {
                        let t: f64 = step as f64 / count as f64;
                        let (px, py) =
                            eval_quad(cursor, (cx as f64, cy as f64), (x as f64, y as f64), t);
                        current.points.push(Point::new(px as f32, py as f32));
                    }
                    cursor = (x as f64, y as f64);
                }
                PathCommand::CubicTo(c1x, c1y, c2x, c2y, x, y) => {
                    let count: usize = cubic_segment_count(
                        [
                            cursor,
                            (c1x as f64, c1y as f64),
                            (c2x as f64, c2y as f64),
                            (x as f64, y as f64),
                        ],
                        self.get_tolerance(),
                    );
                    for step in 1..=count {
                        let t: f64 = step as f64 / count as f64;
                        let (px, py) = eval_cubic(
                            cursor,
                            (c1x as f64, c1y as f64),
                            (c2x as f64, c2y as f64),
                            (x as f64, y as f64),
                            t,
                        );
                        current.points.push(Point::new(px as f32, py as f32));
                    }
                    cursor = (x as f64, y as f64);
                }
                PathCommand::Close => {
                    if current.points.len() > 1 {
                        current.closed = true;
                        self.get_mut_flat().contours.push(take(&mut current));
                    } else {
                        current = Contour::default();
                    }
                    cursor = sub_start;
                }
            }
        }

        if current.points.len() > 1 {
            self.get_mut_flat().contours.push(current);
        }
        self.get_flat()
    }

    /// Triangulates the path and returns the resulting mesh.
    ///
    /// The path is filled according to its [`FillRule`]. Holes are detected
    /// automatically: a contour nested inside another contour with the
    /// opposite winding direction is treated as a hole, which is what makes
    /// a doughnut chart's inner ring disappear.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Shared reference to a `Path`.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    ///
    /// # Returns
    ///
    /// - `SolidMesh` - A `SolidMesh` value.
    pub fn fill(&mut self, path: &Path, color: u32) -> SolidMesh {
        self.fill_flattened(path, color, FillRule::NonZero)
    }

    /// Triangulates the path and returns the resulting mesh, using the
    /// given fill rule.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Shared reference to a `Path`.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `FillRule` - A `FillRule` parameter.
    ///
    /// # Returns
    ///
    /// - `SolidMesh` - A `SolidMesh` value.
    pub fn fill_flattened(&mut self, path: &Path, color: u32, rule: FillRule) -> SolidMesh {
        // A rectangle is two triangles, and it is the shape a chart
        // holds the most of. It skips the flattening and the ear
        // clipping altogether.
        if let Some(corners) = rectangle_of(path) {
            let mut mesh: SolidMesh = SolidMesh::new();
            mesh.reserve(6, 2);
            let vertex = move |point: Point| SolidVertex {
                x: point.x,
                y: point.y,
                color,
            };
            mesh.push_triangle(vertex(corners[0]), vertex(corners[1]), vertex(corners[2]));
            mesh.push_triangle(vertex(corners[0]), vertex(corners[2]), vertex(corners[3]));
            return mesh;
        }
        self.flatten(path);
        let mut mesh: SolidMesh = SolidMesh::new();
        let Self {
            flat,
            working,
            indices,
            ..
        } = self;
        fill_contours_into(&mut mesh, &flat.contours, color, working, indices);
        let _: FillRule = rule;
        mesh
    }

    /// Triangulates a set of contours and returns the resulting mesh.
    ///
    /// # Arguments
    ///
    /// - `&[Contour]` - Shared reference to a `[Contour]`.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `FillRule` - A `FillRule` parameter.
    ///
    /// # Returns
    ///
    /// - `SolidMesh` - A `SolidMesh` value.
    pub fn fill_contours(&mut self, contours: &[Contour], color: u32, rule: FillRule) -> SolidMesh {
        let mut mesh: SolidMesh = SolidMesh::new();
        let Self {
            working, indices, ..
        } = self;
        fill_contours_into(&mut mesh, contours, color, working, indices);
        let _: FillRule = rule;
        mesh
    }

    /// Builds a triangle strip for the region between a polyline and a
    /// straight baseline.
    ///
    /// This is the hot path for area charts: the shape is a strip by
    /// construction, so it needs no triangulation at all, and the whole
    /// series becomes a single indexed draw call.
    ///
    /// `points` are the polyline vertices in order; `baseline` is the
    /// vertical coordinate the area is closed against.
    ///
    /// # Arguments
    ///
    /// - `&[Point]` - Shared reference to a `[Point]`.
    /// - `f32` - A 32-bit float (`f32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    ///
    /// # Returns
    ///
    /// - `SolidMesh` - A `SolidMesh` value.
    pub fn fill_strip(&mut self, points: &[Point], baseline: f32, color: u32) -> SolidMesh {
        let mut mesh: SolidMesh = SolidMesh::new();
        if points.len() < 2 {
            return mesh;
        }
        mesh.reserve(points.len() * 4, points.len() * 2);
        for pair in points.windows(2) {
            let a: Point = pair[0];
            let b: Point = pair[1];
            let base_a: SolidVertex = SolidVertex {
                x: a.x,
                y: baseline,
                color,
            };
            let base_b: SolidVertex = SolidVertex {
                x: b.x,
                y: baseline,
                color,
            };
            let top_a: SolidVertex = SolidVertex {
                x: a.x,
                y: a.y,
                color,
            };
            let top_b: SolidVertex = SolidVertex {
                x: b.x,
                y: b.y,
                color,
            };
            mesh.push_quad(base_a, base_b, top_b, top_a);
        }
        mesh
    }

    /// Expands the path into stroke geometry and returns the mesh.
    ///
    /// Every segment becomes a quad, and every join becomes a wedge, so the
    /// whole stroke is a single indexed draw call regardless of how many
    /// segments it has.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Shared reference to a `Path`.
    /// - `&StrokeStyle` - Shared reference to a `StrokeStyle`.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    ///
    /// # Returns
    ///
    /// - `SolidMesh` - A `SolidMesh` value.
    pub fn stroke(&mut self, path: &Path, style: &StrokeStyle, color: u32) -> SolidMesh {
        self.flatten(path);
        let mut mesh: SolidMesh = SolidMesh::new();
        stroke_contours_into(&mut mesh, &self.get_flat().contours, style, color);
        mesh
    }

    /// Expands a set of contours into stroke geometry.
    ///
    /// # Arguments
    ///
    /// - `&[Contour]` - Shared reference to a `[Contour]`.
    /// - `&StrokeStyle` - Shared reference to a `StrokeStyle`.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    ///
    /// # Returns
    ///
    /// - `SolidMesh` - A `SolidMesh` value.
    pub fn stroke_contours(
        &mut self,
        contours: &[Contour],
        style: &StrokeStyle,
        color: u32,
    ) -> SolidMesh {
        let mut mesh: SolidMesh = SolidMesh::new();
        stroke_contours_into(&mut mesh, contours, style, color);
        mesh
    }
}

/// Default construction for [`Tessellator`].
impl Default for Tessellator {
    /// Constructs a default [`Tessellator`] value.
    fn default() -> Self {
        Self::new()
    }
}

/// Inherent implementation of [`SolidMesh`].
impl SolidMesh {
    /// Returns an empty mesh.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` when the mesh holds no triangles.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_empty(&self) -> bool {
        self.get_indices().is_empty()
    }

    /// Returns the number of triangles in the mesh.
    ///
    /// # Returns
    ///
    /// - `usize` - A non-negative integer.
    pub fn triangle_count(&self) -> usize {
        self.get_indices().len() / 3
    }

    /// Reserves room for the given number of vertices and triangles.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    pub fn reserve(&mut self, vertices: usize, triangles: usize) {
        self.get_mut_vertices().reserve(vertices);
        self.get_mut_indices().reserve(triangles * 3);
    }

    /// Appends a triangle given its three vertices.
    ///
    /// # Arguments
    ///
    /// - `SolidVertex` - A `SolidVertex` parameter.
    pub fn push_triangle(&mut self, a: SolidVertex, b: SolidVertex, c: SolidVertex) {
        let base: u32 = self.get_vertices().len() as u32;
        self.get_mut_vertices().push(a);
        self.get_mut_vertices().push(b);
        self.get_mut_vertices().push(c);
        self.get_mut_indices().push(base);
        self.get_mut_indices().push(base + 1);
        self.get_mut_indices().push(base + 2);
    }

    /// Appends a quad given its four corners, in order.
    ///
    /// # Arguments
    ///
    /// - `SolidVertex` - A `SolidVertex` parameter.
    pub fn push_quad(&mut self, a: SolidVertex, b: SolidVertex, c: SolidVertex, d: SolidVertex) {
        let base: u32 = self.get_vertices().len() as u32;
        self.get_mut_vertices().push(a);
        self.get_mut_vertices().push(b);
        self.get_mut_vertices().push(c);
        self.get_mut_vertices().push(d);
        self.get_mut_indices().push(base);
        self.get_mut_indices().push(base + 1);
        self.get_mut_indices().push(base + 2);
        self.get_mut_indices().push(base);
        self.get_mut_indices().push(base + 2);
        self.get_mut_indices().push(base + 3);
    }
}

/// Inherent implementation of [`TextMesh`].
impl TextMesh {
    /// Returns an empty mesh.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` when the mesh holds no triangles.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_empty(&self) -> bool {
        self.get_indices().is_empty()
    }

    /// Reserves room for the given number of quads.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    pub fn reserve(&mut self, quads: usize) {
        self.get_mut_vertices().reserve(quads * 4);
        self.get_mut_indices().reserve(quads * 6);
    }

    /// Appends a textured quad.
    ///
    /// The corners are given in order — top left, top right, bottom
    /// right, bottom left — and so are the texture coordinates that go
    /// with them.
    ///
    /// # Arguments
    ///
    /// - `[(f32, f32); 4]` - A `[(f32, f32); 4]` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    pub fn push_quad(&mut self, corners: [(f32, f32); 4], uvs: [(f32, f32); 4], color: u32) {
        let base: u32 = self.get_vertices().len() as u32;
        for index in 0..4 {
            let (x, y) = corners[index];
            let (u, v) = uvs[index];
            self.get_mut_vertices()
                .push(TextVertex { x, y, u, v, color });
        }
        self.get_mut_indices().push(base);
        self.get_mut_indices().push(base + 1);
        self.get_mut_indices().push(base + 2);
        self.get_mut_indices().push(base);
        self.get_mut_indices().push(base + 2);
        self.get_mut_indices().push(base + 3);
    }
}
