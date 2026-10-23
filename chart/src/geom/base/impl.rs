use super::*;

/// Inherent implementation of [`Transform2D`].
impl Transform2D {
    /// Returns the identity transform.
    pub fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Returns a pure translation.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn translate(tx: f64, ty: f64) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: tx,
            f: ty,
        }
    }

    /// Returns a pure scale.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn scale(sx: f64, sy: f64) -> Self {
        Self {
            a: sx,
            b: 0.0,
            c: 0.0,
            d: sy,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Returns a pure rotation, in radians, around the origin.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn rotate(angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            a: cos,
            b: sin,
            c: -sin,
            d: cos,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Returns the transform that maps the `from` rectangle onto the `to`
    /// rectangle, scaling uniformly on both axes and centering the result.
    ///
    /// # Arguments
    ///
    /// - `BBox` - A `BBox` parameter.
    pub fn from_rect_to_rect(from: BBox, to: BBox) -> Self {
        let from_width: f64 = from.max_x - from.min_x;
        let from_height: f64 = from.max_y - from.min_y;
        if from_width <= 0.0 || from_height <= 0.0 {
            return Self::identity();
        }
        let scale_x: f64 = (to.max_x - to.min_x) / from_width;
        let scale_y: f64 = (to.max_y - to.min_y) / from_height;
        Self {
            a: scale_x,
            b: 0.0,
            c: 0.0,
            d: scale_y,
            e: to.min_x - from.min_x * scale_x,
            f: to.min_y - from.min_y * scale_y,
        }
    }

    /// Returns the composition of this transform with `other`, where
    /// `other` is applied first.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Shared reference to a `Self`.
    pub fn then(&self, other: &Self) -> Self {
        Self {
            a: self.get_a() * other.get_a() + self.get_c() * other.get_b(),
            b: self.get_b() * other.get_a() + self.get_d() * other.get_b(),
            c: self.get_a() * other.get_c() + self.get_c() * other.get_d(),
            d: self.get_b() * other.get_c() + self.get_d() * other.get_d(),
            e: self.get_a() * other.get_e() + self.get_c() * other.get_f() + self.get_e(),
            f: self.get_b() * other.get_e() + self.get_d() * other.get_f() + self.get_f(),
        }
    }

    /// Applies the transform to a point.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `(f64, f64)` - A `(f64, f64)` value.
    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.get_a() * x + self.get_c() * y + self.get_e(),
            self.get_b() * x + self.get_d() * y + self.get_f(),
        )
    }

    /// Applies the transform to a point, keeping 32-bit precision.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    ///
    /// # Returns
    ///
    /// - `(f32, f32)` - A `(f32, f32)` value.
    pub fn apply_f32(&self, x: f32, y: f32) -> (f32, f32) {
        let (x, y) = self.apply(x as f64, y as f64);
        (x as f32, y as f32)
    }

    /// Returns the transform's determinant.
    ///
    /// A determinant of zero means the transform collapses the plane onto a
    /// line or a point; such a transform has no inverse.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn determinant(&self) -> f64 {
        self.get_a() * self.get_d() - self.get_b() * self.get_c()
    }

    /// Returns the inverse transform, or `None` when the transform is not
    /// invertible.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - `Some(...)` on success, `None` otherwise.
    pub fn invert(&self) -> Option<Self> {
        let determinant: f64 = self.determinant();
        if determinant.abs() < f64::EPSILON {
            return None;
        }
        let inverse: f64 = 1.0 / determinant;
        Some(Self {
            a: self.get_d() * inverse,
            b: -self.get_b() * inverse,
            c: -self.get_c() * inverse,
            d: self.get_a() * inverse,
            e: (self.get_c() * self.get_f() - self.get_d() * self.get_e()) * inverse,
            f: (self.get_b() * self.get_e() - self.get_a() * self.get_f()) * inverse,
        })
    }
}

/// Default construction for [`BBox`].
impl Default for BBox {
    /// Constructs a default [`BBox`] value.
    fn default() -> Self {
        Self::empty()
    }
}

/// Inherent implementation of [`BBox`].
impl BBox {
    /// Returns an empty bounding box.
    ///
    /// The empty box has `min` greater than `max` on both axes, which
    /// makes [`BBox::extend_point`] work without a special case for the
    /// first vertex.
    pub fn empty() -> Self {
        Self {
            min_x: f64::INFINITY,
            min_y: f64::INFINITY,
            max_x: f64::NEG_INFINITY,
            max_y: f64::NEG_INFINITY,
        }
    }

    /// Returns a bounding box from its minimum corner and its extent.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn from_min_size(min_x: f64, min_y: f64, width: f64, height: f64) -> Self {
        Self {
            min_x,
            min_y,
            max_x: min_x + width,
            max_y: min_y + height,
        }
    }

    /// Returns a bounding box from two opposite corners.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn from_corners(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self {
            min_x: x0.min(x1),
            min_y: y0.min(y1),
            max_x: x0.max(x1),
            max_y: y0.max(y1),
        }
    }

    /// Returns `true` when the box contains no geometry.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_empty(&self) -> bool {
        self.get_min_x() > self.get_max_x() || self.get_min_y() > self.get_max_y()
    }

    /// Returns the width of the box, or zero when it is empty.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn width(&self) -> f64 {
        if self.is_empty() {
            0.0
        } else {
            self.get_max_x() - self.get_min_x()
        }
    }

    /// Returns the height of the box, or zero when it is empty.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn height(&self) -> f64 {
        if self.is_empty() {
            0.0
        } else {
            self.get_max_y() - self.get_min_y()
        }
    }

    /// Returns the horizontal center of the box.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn center_x(&self) -> f64 {
        (self.get_min_x() + self.get_max_x()) * 0.5
    }

    /// Returns the vertical center of the box.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn center_y(&self) -> f64 {
        (self.get_min_y() + self.get_max_y()) * 0.5
    }

    /// Grows the box so that it contains the given point.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn extend_point(&mut self, x: f64, y: f64) {
        self.set_min_x(self.get_min_x().min(x));
        self.set_min_y(self.get_min_y().min(y));
        self.set_max_x(self.get_max_x().max(x));
        self.set_max_y(self.get_max_y().max(y));
    }

    /// Grows the box so that it contains the given box.
    ///
    /// # Arguments
    ///
    /// - `&BBox` - Shared reference to a `BBox`.
    pub fn extend_box(&mut self, other: &BBox) {
        if other.is_empty() {
            return;
        }
        self.set_min_x(self.get_min_x().min(other.min_x));
        self.set_min_y(self.get_min_y().min(other.min_y));
        self.set_max_x(self.get_max_x().max(other.max_x));
        self.set_max_y(self.get_max_y().max(other.max_y));
    }

    /// Returns `true` when the box fully contains the given point.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn contains(&self, x: f64, y: f64) -> bool {
        !self.is_empty()
            && x >= self.get_min_x()
            && x <= self.get_max_x()
            && y >= self.get_min_y()
            && y <= self.get_max_y()
    }

    /// Returns `true` when the two boxes overlap.
    ///
    /// # Arguments
    ///
    /// - `&BBox` - Shared reference to a `BBox`.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn intersects(&self, other: &BBox) -> bool {
        !self.is_empty()
            && !other.is_empty()
            && self.get_min_x() <= other.max_x
            && self.get_max_x() >= other.min_x
            && self.get_min_y() <= other.max_y
            && self.get_max_y() >= other.min_y
    }

    /// Returns the overlap of the two boxes, or `None` when they do not
    /// overlap.
    ///
    /// # Arguments
    ///
    /// - `&BBox` - Shared reference to a `BBox`.
    ///
    /// # Returns
    ///
    /// - `Option<BBox>` - `Some(...)` on success, `None` otherwise.
    pub fn intersection(&self, other: &BBox) -> Option<BBox> {
        if !self.intersects(other) {
            return None;
        }
        Some(BBox {
            min_x: self.get_min_x().max(other.min_x),
            min_y: self.get_min_y().max(other.min_y),
            max_x: self.get_max_x().min(other.max_x),
            max_y: self.get_max_y().min(other.max_y),
        })
    }

    /// Returns the union of the two boxes.
    ///
    /// # Arguments
    ///
    /// - `&BBox` - Shared reference to a `BBox`.
    ///
    /// # Returns
    ///
    /// - `BBox` - A `BBox` value.
    pub fn union(&self, other: &BBox) -> BBox {
        let mut result: BBox = *self;
        result.extend_box(other);
        result
    }
}

/// Inherent implementation of [`Point`].
impl Point {
    /// Builds a point from its coordinates.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns the distance between two points.
    ///
    /// # Arguments
    ///
    /// - `&Point` - Shared reference to a `Point`.
    ///
    /// # Returns
    ///
    /// - `f32` - A 32-bit float.
    pub fn distance_to(&self, other: &Point) -> f32 {
        let dx: f32 = self.get_x() - other.x;
        let dy: f32 = self.get_y() - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Returns the squared distance between two points.
    ///
    /// # Arguments
    ///
    /// - `&Point` - Shared reference to a `Point`.
    ///
    /// # Returns
    ///
    /// - `f32` - A 32-bit float.
    pub fn distance_squared_to(&self, other: &Point) -> f32 {
        let dx: f32 = self.get_x() - other.x;
        let dy: f32 = self.get_y() - other.y;
        dx * dx + dy * dy
    }
}

/// Inherent implementation of [`Polygon`].
impl Polygon {
    /// Builds a polygon from a flat list of coordinates.
    ///
    /// # Arguments
    ///
    /// - `&[f32]` - Shared reference to a `[f32]`.
    pub fn from_coordinates(coordinates: &[f32]) -> Self {
        let mut points: Vec<Point> = Vec::with_capacity(coordinates.len() / 2);
        for pair in coordinates.as_chunks::<2>().0 {
            points.push(Point::new(pair[0], pair[1]));
        }
        Self { points }
    }

    /// Returns the polygon's area, using the shoelace formula.
    ///
    /// The sign of the result encodes the winding direction: a positive
    /// area means the vertices run clockwise in a y-down coordinate system.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn signed_area(&self) -> f64 {
        let count: usize = self.get_points().len();
        if count < 3 {
            return 0.0;
        }
        let mut sum: f64 = 0.0;
        for index in 0..count {
            let current: Point = self.get_points()[index];
            let next: Point = self.get_points()[(index + 1) % count];
            sum += current.x as f64 * next.y as f64 - next.x as f64 * current.y as f64;
        }
        sum * 0.5
    }

    /// Returns `true` when the polygon's vertices wind clockwise.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_clockwise(&self) -> bool {
        self.signed_area() > 0.0
    }

    /// Reverses the winding order of the polygon's vertices.
    pub fn reverse(&mut self) {
        self.get_mut_points().reverse();
    }
}

/// Inherent implementation of [`Path`].
impl Path {
    /// Returns an empty path.
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            fill_rule: FillRule::default(),
        }
    }

    /// Returns a path builder.
    ///
    /// # Returns
    ///
    /// - `PathBuilder` - A `PathBuilder` value.
    pub fn builder() -> PathBuilder {
        PathBuilder::new()
    }

    /// Returns the path's command buffer.
    ///
    /// # Returns
    ///
    /// - `&[PathCommand]` - A `[PathCommand]` value.
    pub fn commands(&self) -> &[PathCommand] {
        self.get_commands()
    }

    /// Returns the rule used to decide which parts of the path are inside.
    ///
    /// # Returns
    ///
    /// - `FillRule` - A `FillRule` value.
    pub fn fill_rule(&self) -> FillRule {
        self.get_fill_rule()
    }

    /// Returns `true` when the path holds no commands.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_empty(&self) -> bool {
        self.get_commands().is_empty()
    }

    /// Returns the number of commands in the path.
    ///
    /// # Returns
    ///
    /// - `usize` - A non-negative integer.
    pub fn len(&self) -> usize {
        self.get_commands().len()
    }

    /// Returns the tightest axis-aligned box containing the path.
    ///
    /// The bounds are computed from the control points of every curve,
    /// which is a conservative over-estimate for Béziers — the true
    /// extremum of a curve lies strictly inside the control polygon.
    /// Over-estimating is the safe direction for culling: a shape is never
    /// culled when it should be drawn.
    ///
    /// # Returns
    ///
    /// - `BBox` - A `BBox` value.
    pub fn bounds(&self) -> BBox {
        let mut bounds: BBox = BBox::empty();
        for command in self.get_commands() {
            match *command {
                PathCommand::MoveTo(x, y) | PathCommand::LineTo(x, y) => {
                    bounds.extend_point(x as f64, y as f64);
                }
                PathCommand::QuadTo(cx, cy, x, y) => {
                    bounds.extend_point(cx as f64, cy as f64);
                    bounds.extend_point(x as f64, y as f64);
                }
                PathCommand::CubicTo(c1x, c1y, c2x, c2y, x, y) => {
                    bounds.extend_point(c1x as f64, c1y as f64);
                    bounds.extend_point(c2x as f64, c2y as f64);
                    bounds.extend_point(x as f64, y as f64);
                }
                PathCommand::Close => {}
            }
        }
        bounds
    }

    /// Returns a copy of the path with the given transform applied.
    ///
    /// The transform is baked into the coordinates rather than kept
    /// alongside the path, so the tessellator never has to apply it
    /// per-vertex.
    ///
    /// # Arguments
    ///
    /// - `&Transform2D` - Shared reference to a `Transform2D`.
    ///
    /// # Returns
    ///
    /// - `Path` - A `Path` value.
    pub fn transformed(&self, transform: &Transform2D) -> Path {
        let mut commands: Vec<PathCommand> = Vec::with_capacity(self.get_commands().len());
        for command in self.get_commands() {
            commands.push(match *command {
                PathCommand::MoveTo(x, y) => {
                    let (x, y) = transform.apply_f32(x, y);
                    PathCommand::MoveTo(x, y)
                }
                PathCommand::LineTo(x, y) => {
                    let (x, y) = transform.apply_f32(x, y);
                    PathCommand::LineTo(x, y)
                }
                PathCommand::QuadTo(cx, cy, x, y) => {
                    let (cx, cy) = transform.apply_f32(cx, cy);
                    let (x, y) = transform.apply_f32(x, y);
                    PathCommand::QuadTo(cx, cy, x, y)
                }
                PathCommand::CubicTo(c1x, c1y, c2x, c2y, x, y) => {
                    let (c1x, c1y) = transform.apply_f32(c1x, c1y);
                    let (c2x, c2y) = transform.apply_f32(c2x, c2y);
                    let (x, y) = transform.apply_f32(x, y);
                    PathCommand::CubicTo(c1x, c1y, c2x, c2y, x, y)
                }
                PathCommand::Close => PathCommand::Close,
            });
        }
        Path {
            commands,
            fill_rule: self.get_fill_rule(),
        }
    }
}

/// Inherent implementation of [`PathBuilder`].
impl PathBuilder {
    /// Returns an empty builder.
    pub fn new() -> Self {
        Self {
            // The room for a handful of commands is reserved right
            // away: a chart over a million bars builds a million paths,
            // and a path that has to grow its buffer twice costs two
            // allocations instead of one.
            commands: Vec::with_capacity(PATH_COMMAND_CAPACITY),
            fill_rule: FillRule::default(),
            current: Point::default(),
            start: Point::default(),
        }
    }

    /// Sets the fill rule used when the path is built.
    ///
    /// # Arguments
    ///
    /// - `FillRule` - A `FillRule` parameter.
    pub fn fill_rule(mut self, rule: FillRule) -> Self {
        self.set_fill_rule(rule);
        self
    }

    /// Starts a new sub-path at the given point.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.set_current(Point::new(x, y));
        self.set_start(self.get_current());
        self.get_mut_commands().push(PathCommand::MoveTo(x, y));
        self
    }

    /// Appends a straight line segment.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    pub fn line_to(mut self, x: f32, y: f32) -> Self {
        self.set_current(Point::new(x, y));
        self.get_mut_commands().push(PathCommand::LineTo(x, y));
        self
    }

    /// Appends a quadratic Bézier segment.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    pub fn quad_to(mut self, control_x: f32, control_y: f32, x: f32, y: f32) -> Self {
        self.set_current(Point::new(x, y));
        self.get_mut_commands()
            .push(PathCommand::QuadTo(control_x, control_y, x, y));
        self
    }

    /// Appends a cubic Bézier segment.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    pub fn cubic_to(
        mut self,
        control1_x: f32,
        control1_y: f32,
        control2_x: f32,
        control2_y: f32,
        x: f32,
        y: f32,
    ) -> Self {
        self.set_current(Point::new(x, y));
        self.get_mut_commands().push(PathCommand::CubicTo(
            control1_x, control1_y, control2_x, control2_y, x, y,
        ));
        self
    }

    /// Closes the current sub-path.
    pub fn close(mut self) -> Self {
        self.set_current(self.get_start());
        self.get_mut_commands().push(PathCommand::Close);
        self
    }

    /// Appends an elliptical arc centered at the given point.
    ///
    /// `start_angle` and `end_angle` are measured in radians, clockwise
    /// from the positive x-axis (the y-axis points down, matching the
    /// Canvas2D and SVG conventions). The arc is approximated with cubic
    /// Bézier segments.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    /// - `f64` - A 64-bit float (`f64`).
    pub fn arc(
        mut self,
        center_x: f32,
        center_y: f32,
        radius_x: f32,
        radius_y: f32,
        start_angle: f64,
        end_angle: f64,
    ) -> Self {
        let sweep: f64 = end_angle - start_angle;
        if sweep == 0.0 {
            return self;
        }
        let segments: usize = arc_segment_count(sweep, radius_x.max(radius_y) as f64, 0.1);
        let (sin, cos) = start_angle.sin_cos();
        let start_x: f64 = center_x as f64 + radius_x as f64 * cos;
        let start_y: f64 = center_y as f64 + radius_y as f64 * sin;
        if self.get_commands().is_empty() {
            self.get_mut_commands()
                .push(PathCommand::MoveTo(start_x as f32, start_y as f32));
        } else {
            self.get_mut_commands()
                .push(PathCommand::LineTo(start_x as f32, start_y as f32));
        }
        self.set_current(Point::new(start_x as f32, start_y as f32));
        self.set_start(self.get_current());

        arc_to_cubic(
            Ellipse {
                center_x: center_x as f64,
                center_y: center_y as f64,
                radius_x: radius_x as f64,
                radius_y: radius_y as f64,
            },
            start_angle,
            sweep,
            segments,
            |c1x: f64, c1y: f64, c2x: f64, c2y: f64, x: f64, y: f64| {
                self.get_mut_commands().push(PathCommand::CubicTo(
                    c1x as f32, c1y as f32, c2x as f32, c2y as f32, x as f32, y as f32,
                ));
            },
        );
        self
    }

    /// Appends a full ellipse centered at the given point.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    pub fn ellipse(mut self, center_x: f32, center_y: f32, radius_x: f32, radius_y: f32) -> Self {
        let start: f64 = core::f64::consts::FRAC_PI_2;
        self = self.move_to(center_x, center_y - radius_y);
        self = self.arc(
            center_x,
            center_y,
            radius_x,
            radius_y,
            start,
            start + core::f64::consts::TAU,
        );
        self.close()
    }

    /// Appends an axis-aligned rectangle.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    pub fn rect(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.get_mut_commands().push(PathCommand::MoveTo(x, y));
        self.get_mut_commands()
            .push(PathCommand::LineTo(x + width, y));
        self.get_mut_commands()
            .push(PathCommand::LineTo(x + width, y + height));
        self.get_mut_commands()
            .push(PathCommand::LineTo(x, y + height));
        self.get_mut_commands().push(PathCommand::Close);
        self
    }

    /// Appends a rectangle with rounded corners.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    pub fn round_rect(mut self, x: f32, y: f32, width: f32, height: f32, radius: f32) -> Self {
        let radius: f32 = radius.min(width * 0.5).min(height * 0.5);
        if radius <= 0.0 {
            return self.rect(x, y, width, height);
        }
        let right: f32 = x + width;
        let bottom: f32 = y + height;
        let kappa: f64 = radius as f64 * ARC_BEZIER_KAPPA;
        self.get_mut_commands()
            .push(PathCommand::MoveTo(x + radius, y));
        self.get_mut_commands()
            .push(PathCommand::LineTo(right - radius, y));
        self.get_mut_commands().push(PathCommand::CubicTo(
            right - radius + kappa as f32,
            y,
            right,
            y + radius - kappa as f32,
            right,
            y + radius,
        ));
        self.get_mut_commands()
            .push(PathCommand::LineTo(right, bottom - radius));
        self.get_mut_commands().push(PathCommand::CubicTo(
            right,
            bottom - radius + kappa as f32,
            right - radius + kappa as f32,
            bottom,
            right - radius,
            bottom,
        ));
        self.get_mut_commands()
            .push(PathCommand::LineTo(x + radius, bottom));
        self.get_mut_commands().push(PathCommand::CubicTo(
            x + radius - kappa as f32,
            bottom,
            x,
            bottom - radius + kappa as f32,
            x,
            bottom - radius,
        ));
        self.get_mut_commands()
            .push(PathCommand::LineTo(x, y + radius));
        self.get_mut_commands().push(PathCommand::CubicTo(
            x,
            y + radius - kappa as f32,
            x + radius - kappa as f32,
            y,
            x + radius,
            y,
        ));
        self.get_mut_commands().push(PathCommand::Close);
        self
    }

    /// Appends a closed polygon through the given points.
    ///
    /// # Arguments
    ///
    /// - `&[Point]` - Shared reference to a `[Point]`.
    pub fn polygon(mut self, points: &[Point]) -> Self {
        if let Some(first) = points.first() {
            self.get_mut_commands()
                .push(PathCommand::MoveTo(first.x, first.y));
            for point in &points[1..] {
                self.get_mut_commands()
                    .push(PathCommand::LineTo(point.x, point.y));
            }
            self.get_mut_commands().push(PathCommand::Close);
        }
        self
    }

    /// Returns the finished path.
    ///
    /// # Returns
    ///
    /// - `Path` - A `Path` value.
    pub fn build(self) -> Path {
        let Self {
            commands,
            fill_rule,
            ..
        } = self;
        Path {
            commands,
            fill_rule,
        }
    }
}

/// Default construction for [`PathBuilder`].
impl Default for PathBuilder {
    /// Constructs a default [`PathBuilder`] value.
    fn default() -> Self {
        Self::new()
    }
}

/// Inherent implementation of [`DashPattern`].
impl DashPattern {
    /// Builds a solid (undashed) pattern.
    pub fn solid() -> Self {
        Self {
            pattern: Vec::new(),
            offset: 0.0,
        }
    }

    /// Builds a dash pattern from alternating on/off lengths.
    ///
    /// # Arguments
    ///
    /// - `Vec<f32>` - A `Vec<f32>` parameter.
    /// - `f32` - A 32-bit float (`f32`).
    pub fn new(pattern: Vec<f32>, offset: f32) -> Self {
        Self { pattern, offset }
    }

    /// Returns `true` when the pattern draws an unbroken line.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_solid(&self) -> bool {
        self.get_pattern().is_empty()
    }
}
