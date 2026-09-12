use super::*;

/// Implements static math utility methods on the `Numeric` namespace struct.
impl Numeric {
    /// Clamps a value between a minimum and maximum bound.
    ///
    /// # Arguments
    ///
    /// - `f64` - The value to clamp.
    /// - `f64` - The minimum allowed value.
    /// - `f64` - The maximum allowed value.
    ///
    /// # Returns
    ///
    /// - `f64` - The clamped value.
    pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
        value.max(min).min(max)
    }

    /// Performs linear interpolation between two values.
    ///
    /// # Arguments
    ///
    /// - `f64` - The start value.
    /// - `f64` - The end value.
    /// - `f64` - The interpolation factor, typically in the range 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// - `f64` - The interpolated value.
    pub fn lerp(start: f64, end: f64, factor: f64) -> f64 {
        start + (end - start) * factor
    }

    /// Converts an angle from degrees to radians.
    ///
    /// # Arguments
    ///
    /// - `f64` - The angle in degrees.
    ///
    /// # Returns
    ///
    /// - `f64` - The angle in radians.
    pub fn deg_to_rad(degrees: f64) -> f64 {
        degrees * DEG_TO_RAD
    }

    /// Converts an angle from radians to degrees.
    ///
    /// # Arguments
    ///
    /// - `f64` - The angle in radians.
    ///
    /// # Returns
    ///
    /// - `f64` - The angle in degrees.
    pub fn rad_to_deg(radians: f64) -> f64 {
        radians * RAD_TO_DEG
    }

    /// Normalizes an angle to the range -PI to PI.
    ///
    /// # Arguments
    ///
    /// - `f64` - The angle in radians.
    ///
    /// # Returns
    ///
    /// - `f64` - The normalized angle in the range -PI to PI.
    pub fn normalize_angle(radians: f64) -> f64 {
        let mut angle: f64 = radians % TWO_PI;
        if angle < -r#const::PI {
            angle += TWO_PI;
        }
        if angle > r#const::PI {
            angle -= TWO_PI;
        }
        angle
    }

    /// Computes the shortest angular difference between two angles.
    ///
    /// # Arguments
    ///
    /// - `f64` - The source angle in radians.
    /// - `f64` - The target angle in radians.
    ///
    /// # Returns
    ///
    /// - `f64` - The signed angular delta in the range -PI to PI.
    pub fn angle_delta(from: f64, to: f64) -> f64 {
        Self::normalize_angle(to - from)
    }

    /// Performs angular interpolation taking the shortest path around the circle.
    ///
    /// # Arguments
    ///
    /// - `f64` - The source angle in radians.
    /// - `f64` - The target angle in radians.
    /// - `f64` - The interpolation factor, typically in the range 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// - `f64` - The interpolated angle in radians.
    pub fn lerp_angle(from: f64, to: f64, factor: f64) -> f64 {
        from + Self::angle_delta(from, to) * factor
    }

    /// Computes the Euclidean distance between two 2D points.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The first point.
    /// - `Vector2D` - The second point.
    ///
    /// # Returns
    ///
    /// - `f64` - The distance between the two points.
    pub fn distance(a: Vector2D, b: Vector2D) -> f64 {
        (b - a).magnitude()
    }

    /// Computes the squared Euclidean distance between two 2D points.
    ///
    /// Avoids a square root, making it faster for comparison-only use cases.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The first point.
    /// - `Vector2D` - The second point.
    ///
    /// # Returns
    ///
    /// - `f64` - The squared distance between the two points.
    pub fn distance_squared(a: Vector2D, b: Vector2D) -> f64 {
        (b - a).magnitude_squared()
    }

    /// Computes a smoothstep interpolation factor using a cubic Hermite polynomial.
    ///
    /// # Arguments
    ///
    /// - `f64` - The edge minimum.
    /// - `f64` - The edge maximum.
    /// - `f64` - The input value.
    ///
    /// # Returns
    ///
    /// - `f64` - The smoothstep result in the range 0.0 to 1.0.
    pub fn smoothstep(edge_min: f64, edge_max: f64, value: f64) -> f64 {
        let clamped: f64 = Self::clamp((value - edge_min) / (edge_max - edge_min), 0.0, 1.0);
        clamped * clamped * (3.0 - 2.0 * clamped)
    }

    /// Moves `current` towards `target` by at most `max_delta`.
    ///
    /// # Arguments
    ///
    /// - `f64` - The current value.
    /// - `f64` - The target value.
    /// - `f64` - The maximum allowed change.
    ///
    /// # Returns
    ///
    /// - `f64` - The new value moved towards target.
    pub fn approach(current: f64, target: f64, max_delta: f64) -> f64 {
        if (target - current).abs() <= max_delta {
            return target;
        }
        current + max_delta.signum() * max_delta
    }

    /// Returns the sign of a value as -1.0, 0.0, or 1.0.
    ///
    /// # Arguments
    ///
    /// - `f64` - The input value.
    ///
    /// # Returns
    ///
    /// - `f64` - -1.0 if negative, 0.0 if zero, 1.0 if positive.
    pub fn sign(value: f64) -> f64 {
        if value > 0.0 {
            1.0
        } else if value < 0.0 {
            -1.0
        } else {
            0.0
        }
    }

    /// Wraps a value into the range 0.0 to `max`.
    ///
    /// # Arguments
    ///
    /// - `f64` - The value to wrap.
    /// - `f64` - The upper bound of the range.
    ///
    /// # Returns
    ///
    /// - `f64` - The wrapped value in the range 0.0 to `max`.
    pub fn wrap(value: f64, max: f64) -> f64 {
        let result: f64 = value % max;
        if result < 0.0 { result + max } else { result }
    }

    /// Returns 1.0 if the value is positive, -1.0 otherwise.
    ///
    /// # Arguments
    ///
    /// - `f64` - The input value.
    ///
    /// # Returns
    ///
    /// - `f64` - 1.0 if the value is non-negative, -1.0 otherwise.
    pub fn sign_or_positive(value: f64) -> f64 {
        if value < 0.0 { -1.0 } else { 1.0 }
    }

    /// Computes the Euclidean distance between two 3D points.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The first point.
    /// - `Vector3D` - The second point.
    ///
    /// # Returns
    ///
    /// - `f64` - The distance between the two points.
    pub fn distance_3d(a: Vector3D, b: Vector3D) -> f64 {
        (b - a).magnitude()
    }

    /// Computes the squared Euclidean distance between two 3D points.
    ///
    /// Avoids a square root, making it faster for comparison-only use cases.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The first point.
    /// - `Vector3D` - The second point.
    ///
    /// # Returns
    ///
    /// - `f64` - The squared distance between the two points.
    pub fn distance_squared_3d(a: Vector3D, b: Vector3D) -> f64 {
        (b - a).magnitude_squared()
    }
}

/// Implements the `Interpolable` trait for `f64`.
impl Interpolable for f64 {
    /// Linearly interpolates toward `other` by the supplied `factor`.
    ///
    /// # Arguments
    ///
    /// - `f64` - The opposite endpoint of the interpolation.
    /// - `f64` - Interpolation factor; typically `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `f64` - The linearly-interpolated value.
    fn lerp(&self, other: f64, factor: f64) -> f64 {
        *self + (other - *self) * factor
    }
}

/// Implements the [`Vector`] trait for `Vector2D`, forwarding every method to
/// the inherent implementation on the struct.
///
/// `Vector2D` also offers 2D-specific operations that are not part of the
/// trait surface: `perp`, `cross` (returning `f64`), `from_angle`, `angle`,
/// `angle_to`, `rotated`, `rotate`, `distance_to`, `distance_squared_to`,
/// `direction_to`, `scale`, and `normalize`. These remain inherent.
impl Vector for Vector2D {
    /// Returns the zero vector of this dimension.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The zero vector of this dimension.
    fn zero() -> Vector2D {
        Vector2D::zero()
    }

    /// Returns the dot product of `self` and `other`.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - Other vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The dot product of `self` and `other`.
    fn dot(&self, other: Vector2D) -> f64 {
        Vector2D::dot(self, other)
    }

    /// Returns the Euclidean magnitude (length) of the vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The Euclidean magnitude of the vector.
    fn magnitude(&self) -> f64 {
        Vector2D::magnitude(self)
    }

    /// Returns the squared magnitude (no square-root) of the vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The squared magnitude of the vector (no square root).
    fn magnitude_squared(&self) -> f64 {
        Vector2D::magnitude_squared(self)
    }

    /// Returns the unit-length direction along `self`.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The unit-length direction; undefined when the vector is zero.
    fn normalized(&self) -> Vector2D {
        Vector2D::normalized(self)
    }

    /// Returns the vector multiplied by `scalar`.
    ///
    /// # Arguments
    ///
    /// - `f64` - Scalar multiplier.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The vector scaled by `scalar`.
    fn scaled(&self, scalar: f64) -> Vector2D {
        Vector2D::scaled(self, scalar)
    }

    /// Linearly interpolates toward `other` by the supplied `factor`.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The opposite endpoint of the interpolation.
    /// - `f64` - Interpolation factor; typically `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The linearly-interpolated value.
    fn lerp(&self, other: Vector2D, factor: f64) -> Vector2D {
        Vector2D::lerp(self, other, factor)
    }
}

/// Implements methods and operator overloading for `Vector2D`.
impl Vector2D {
    /// Returns the zero vector (0.0, 0.0).
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The zero vector.
    pub fn zero() -> Vector2D {
        Vector2D::new(0.0, 0.0)
    }

    /// Returns the unit vector pointing right (1.0, 0.0).
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The right unit vector.
    pub fn right() -> Vector2D {
        Vector2D::new(1.0, 0.0)
    }

    /// Returns the unit vector pointing up (0.0, -1.0).
    ///
    /// In screen coordinates where y increases downward.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The up unit vector.
    pub fn up() -> Vector2D {
        Vector2D::new(0.0, -1.0)
    }

    /// Creates a unit vector from an angle in radians.
    ///
    /// # Arguments
    ///
    /// - `f64` - The angle in radians.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The unit vector pointing in the given direction.
    pub fn from_angle(radians: f64) -> Vector2D {
        Vector2D::new(radians.cos(), radians.sin())
    }

    /// Returns the magnitude (length) of the vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The magnitude of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.get_x() * self.get_x() + self.get_y() * self.get_y()).sqrt()
    }

    /// Returns the squared magnitude of the vector.
    ///
    /// Avoids a square root, making it faster for comparison-only use cases.
    ///
    /// # Returns
    ///
    /// - `f64` - The squared magnitude of the vector.
    pub fn magnitude_squared(&self) -> f64 {
        self.get_x() * self.get_x() + self.get_y() * self.get_y()
    }

    /// Returns a normalized (unit length) copy of this vector.
    ///
    /// Returns the zero vector if the magnitude is zero.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The normalized vector.
    pub fn normalized(&self) -> Vector2D {
        let mag: f64 = self.magnitude();
        if mag < EPSILON {
            return Vector2D::zero();
        }
        Vector2D::new(self.get_x() / mag, self.get_y() / mag)
    }

    /// Normalizes this vector in place.
    pub fn normalize(&mut self) {
        let mag: f64 = self.magnitude();
        if mag < EPSILON {
            self.set_x(0.0);
            self.set_y(0.0);
            return;
        }
        self.set_x(self.get_x() / mag);
        self.set_y(self.get_y() / mag);
    }

    /// Computes the dot product with another vector.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The other vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The dot product.
    pub fn dot(&self, other: Vector2D) -> f64 {
        self.get_x() * other.get_x() + self.get_y() * other.get_y()
    }

    /// Computes the 2D cross product (scalar) with another vector.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The other vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The cross product scalar.
    pub fn cross(&self, other: Vector2D) -> f64 {
        self.get_x() * other.get_y() - self.get_y() * other.get_x()
    }

    /// Returns the perpendicular vector (rotated 90 degrees counter-clockwise).
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The perpendicular vector.
    pub fn perp(&self) -> Vector2D {
        Vector2D::new(-self.get_y(), self.get_x())
    }

    /// Returns the angle of this vector in radians.
    ///
    /// # Returns
    ///
    /// - `f64` - The angle in radians.
    pub fn angle(&self) -> f64 {
        self.get_y().atan2(self.get_x())
    }

    /// Returns the angle from this vector to another.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The target vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The signed angle in radians.
    pub fn angle_to(&self, other: Vector2D) -> f64 {
        (other - *self).angle()
    }

    /// Returns a rotated copy of this vector.
    ///
    /// # Arguments
    ///
    /// - `f64` - The rotation angle in radians.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The rotated vector.
    pub fn rotated(&self, radians: f64) -> Vector2D {
        let cos: f64 = radians.cos();
        let sin: f64 = radians.sin();
        Vector2D::new(
            self.get_x() * cos - self.get_y() * sin,
            self.get_x() * sin + self.get_y() * cos,
        )
    }

    /// Rotates this vector in place.
    ///
    /// # Arguments
    ///
    /// - `f64` - The rotation angle in radians.
    pub fn rotate(&mut self, radians: f64) {
        let cos: f64 = radians.cos();
        let sin: f64 = radians.sin();
        let new_x: f64 = self.get_x() * cos - self.get_y() * sin;
        let new_y: f64 = self.get_x() * sin + self.get_y() * cos;
        self.set_x(new_x);
        self.set_y(new_y);
    }

    /// Returns the distance from this point to another.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The target point.
    ///
    /// # Returns
    ///
    /// - `f64` - The Euclidean distance.
    pub fn distance_to(&self, other: Vector2D) -> f64 {
        (other - *self).magnitude()
    }

    /// Returns the squared distance from this point to another.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The target point.
    ///
    /// # Returns
    ///
    /// - `f64` - The squared Euclidean distance.
    pub fn distance_squared_to(&self, other: Vector2D) -> f64 {
        (other - *self).magnitude_squared()
    }

    /// Returns a unit vector pointing from this point to another.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The target point.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The direction unit vector.
    pub fn direction_to(&self, other: Vector2D) -> Vector2D {
        (other - *self).normalized()
    }

    /// Returns a linearly interpolated vector between this and another.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The target vector.
    /// - `f64` - The interpolation factor.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The interpolated vector.
    pub fn lerp(&self, other: Vector2D, factor: f64) -> Vector2D {
        Vector2D::new(
            self.get_x() + (other.get_x() - self.get_x()) * factor,
            self.get_y() + (other.get_y() - self.get_y()) * factor,
        )
    }

    /// Scales this vector by a scalar factor.
    ///
    /// # Arguments
    ///
    /// - `f64` - The scalar factor.
    pub fn scale(&mut self, scalar: f64) {
        self.set_x(self.get_x() * scalar);
        self.set_y(self.get_y() * scalar);
    }

    /// Returns a scaled copy of this vector.
    ///
    /// # Arguments
    ///
    /// - `f64` - The scalar factor.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The scaled vector.
    pub fn scaled(&self, scalar: f64) -> Vector2D {
        Vector2D::new(self.get_x() * scalar, self.get_y() * scalar)
    }
}

/// Implements `Interpolable` for `Vector2D`.
impl Interpolable for Vector2D {
    /// Linearly interpolates toward `other` by the supplied `factor`.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The opposite endpoint of the interpolation.
    /// - `f64` - Interpolation factor; typically `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The linearly-interpolated value.
    fn lerp(&self, other: Vector2D, factor: f64) -> Vector2D {
        Vector2D::lerp(self, other, factor)
    }
}

/// Implements vector addition.
impl Add for Vector2D {
    type Output = Vector2D;
    /// Adds `other` to `self`.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - Other operand.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - Sum of `self` and `other`.
    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D::new(self.get_x() + other.get_x(), self.get_y() + other.get_y())
    }
}

/// Implements vector subtraction.
impl Sub for Vector2D {
    type Output = Vector2D;
    /// Subtracts `other` from `self`.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - Operand to subtract.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - `self` minus `other`.
    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D::new(self.get_x() - other.get_x(), self.get_y() - other.get_y())
    }
}

/// Implements scalar multiplication.
impl Mul<f64> for Vector2D {
    type Output = Vector2D;
    /// Multiplies `self` and `other` (or `scalar`).
    ///
    /// # Arguments
    ///
    /// - `f64` - Other operand or scalar.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - Product of `self` and the operand.
    fn mul(self, scalar: f64) -> Vector2D {
        Vector2D::new(self.get_x() * scalar, self.get_y() * scalar)
    }
}

/// Implements vector negation.
impl Neg for Vector2D {
    type Output = Vector2D;
    /// Negates `self`.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - Negated vector.
    fn neg(self) -> Vector2D {
        Vector2D::new(-self.get_x(), -self.get_y())
    }
}

/// Implements in-place vector addition.
impl AddAssign for Vector2D {
    /// Adds `other` to `self` in place.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - Other operand.
    fn add_assign(&mut self, other: Vector2D) {
        self.set_x(self.get_x() + other.get_x());
        self.set_y(self.get_y() + other.get_y());
    }
}

/// Implements in-place vector subtraction.
impl SubAssign for Vector2D {
    /// Subtracts `other` from `self` in place.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - Operand to subtract.
    fn sub_assign(&mut self, other: Vector2D) {
        self.set_x(self.get_x() - other.get_x());
        self.set_y(self.get_y() - other.get_y());
    }
}

/// Implements in-place scalar multiplication.
impl MulAssign<f64> for Vector2D {
    /// Multiplies `self` by `scalar` in place.
    ///
    /// # Arguments
    ///
    /// - `f64` - Scalar multiplier.
    fn mul_assign(&mut self, scalar: f64) {
        self.set_x(self.get_x() * scalar);
        self.set_y(self.get_y() * scalar);
    }
}

/// Implements methods for `Rect`.
impl Rect {
    /// Creates a rectangle from a center point and dimensions.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The center point.
    /// - `f64` - The width.
    /// - `f64` - The height.
    ///
    /// # Returns
    ///
    /// - `Rect` - The new rectangle.
    pub fn from_center(center: Vector2D, width: f64, height: f64) -> Rect {
        Rect::new(
            center.get_x() - width * 0.5,
            center.get_y() - height * 0.5,
            width,
            height,
        )
    }

    /// Returns the center point of the rectangle.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The center point.
    pub fn center(&self) -> Vector2D {
        Vector2D::new(
            self.get_x() + self.get_width() * 0.5,
            self.get_y() + self.get_height() * 0.5,
        )
    }

    /// Returns the minimum corner (top-left).
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The minimum corner.
    pub fn min(&self) -> Vector2D {
        Vector2D::new(self.get_x(), self.get_y())
    }

    /// Returns the maximum corner (bottom-right).
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The maximum corner.
    pub fn max(&self) -> Vector2D {
        Vector2D::new(
            self.get_x() + self.get_width(),
            self.get_y() + self.get_height(),
        )
    }

    /// Returns the size as a vector.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The size vector.
    pub fn size(&self) -> Vector2D {
        Vector2D::new(self.get_width(), self.get_height())
    }

    /// Tests whether a point is inside this rectangle.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The point to test.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the point is inside.
    pub fn contains(&self, point: Vector2D) -> bool {
        point.get_x() >= self.get_x()
            && point.get_x() <= self.get_x() + self.get_width()
            && point.get_y() >= self.get_y()
            && point.get_y() <= self.get_y() + self.get_height()
    }

    /// Tests whether this rectangle intersects another.
    ///
    /// # Arguments
    ///
    /// - `Rect` - The other rectangle.
    ///
    /// # Returns
    ///
    /// - `bool` - True if they intersect.
    pub fn intersects(&self, other: Rect) -> bool {
        self.get_x() < other.get_x() + other.get_width()
            && self.get_x() + self.get_width() > other.get_x()
            && self.get_y() < other.get_y() + other.get_height()
            && self.get_y() + self.get_height() > other.get_y()
    }

    /// Alias for `intersects` — used by the physics module's broad-phase
    /// collision check. (`Rect::broad_phase(a, b)` was being referenced by
    /// upstream code; we expose it here so the engine compiles cleanly.)
    ///
    /// # Arguments
    ///
    /// - `Rect` - A `Rect` parameter.
    /// - `Rect` - A `Rect` parameter.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn broad_phase_alias(a: Rect, b: Rect) -> bool {
        a.intersects(b)
    }

    /// Returns the intersection of two rectangles, or `None` if they do not overlap.
    ///
    /// # Arguments
    ///
    /// - `Rect` - The other rectangle.
    ///
    /// # Returns
    ///
    /// - `Option<Rect>` - The intersection rectangle, or `None`.
    pub fn intersection(&self, other: Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }
        let max_x: f64 = self.get_x().max(other.get_x());
        let max_y: f64 = self.get_y().max(other.get_y());
        let min_right: f64 =
            (self.get_x() + self.get_width()).min(other.get_x() + other.get_width());
        let min_bottom: f64 =
            (self.get_y() + self.get_height()).min(other.get_y() + other.get_height());
        Some(Rect::new(
            max_x,
            max_y,
            min_right - max_x,
            min_bottom - max_y,
        ))
    }
}

/// Implements methods for `Circle`.
impl Circle {
    /// Tests whether a point is inside this circle.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The point to test.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the point is inside.
    pub fn contains(&self, point: Vector2D) -> bool {
        self.get_center().distance_squared_to(point) <= self.get_radius() * self.get_radius()
    }

    /// Tests whether this circle intersects another.
    ///
    /// # Arguments
    ///
    /// - `Circle` - The other circle.
    ///
    /// # Returns
    ///
    /// - `bool` - True if they intersect.
    pub fn intersects(&self, other: Circle) -> bool {
        let distance_sq: f64 = self.get_center().distance_squared_to(other.get_center());
        let radius_sum: f64 = self.get_radius() + other.get_radius();
        distance_sq <= radius_sum * radius_sum
    }

    /// Returns the circumference of the circle.
    ///
    /// # Returns
    ///
    /// - `f64` - The circumference.
    pub fn circumference(&self) -> f64 {
        TWO_PI * self.get_radius()
    }

    /// Returns the area of the circle.
    ///
    /// # Returns
    ///
    /// - `f64` - The area.
    pub fn area(&self) -> f64 {
        r#const::PI * self.get_radius() * self.get_radius()
    }
}

/// Implements methods for `Transform2D`.
impl Transform2D {
    /// Creates a new transform at the origin with no rotation and unit scale.
    ///
    /// # Returns
    ///
    /// - `Transform2D` - The identity transform.
    pub fn identity() -> Transform2D {
        Transform2D::new(Vector2D::zero(), 0.0, Vector2D::new(1.0, 1.0))
    }

    /// Translates the position by the given offset.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The translation offset.
    pub fn translate(&mut self, offset: Vector2D) {
        self.set_position(self.get_position() + offset);
    }

    /// Rotates by the given angle in radians.
    ///
    /// # Arguments
    ///
    /// - `f64` - The rotation delta in radians.
    pub fn rotate(&mut self, radians: f64) {
        self.set_rotation(self.get_rotation() + radians);
    }

    /// Scales by the given factors.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The scale factors.
    pub fn scale_by(&mut self, factors: Vector2D) {
        let mut scale: Vector2D = self.get_scale();
        scale.set_x(scale.get_x() * factors.get_x());
        scale.set_y(scale.get_y() * factors.get_y());
        self.set_scale(scale);
    }

    /// Applies this transform to a local-space point, returning world-space coordinates.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The local-space point.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The transformed world-space point.
    pub fn apply_to_point(&self, point: Vector2D) -> Vector2D {
        let scaled: Vector2D = Vector2D::new(
            point.get_x() * self.get_scale().get_x(),
            point.get_y() * self.get_scale().get_y(),
        );
        scaled.rotated(self.get_rotation()) + self.get_position()
    }
}

/// Implements `Default` for `Transform2D` as the identity transform.
impl Default for Transform2D {
    /// Constructs a default [`Transform2D`] value.
    ///
    /// # Returns
    ///
    /// - `Transform2D` - A default-constructed instance with the documented initial state.
    fn default() -> Transform2D {
        Transform2D::identity()
    }
}

/// Implements methods for `Color`.
impl Color {
    /// Creates a color from RGB hex values (0-255), with full opacity.
    ///
    /// # Arguments
    ///
    /// - `u8` - The red channel (0-255).
    /// - `u8` - The green channel (0-255).
    /// - `u8` - The blue channel (0-255).
    ///
    /// # Returns
    ///
    /// - `Color` - The new color.
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color::new(
            red as f64 / 255.0,
            green as f64 / 255.0,
            blue as f64 / 255.0,
            1.0,
        )
    }

    /// Converts the color to a CSS `rgba()` string.
    ///
    /// # Returns
    ///
    /// - `String` - The CSS color string.
    pub fn to_css_rgba(&self) -> String {
        let mut buffer: String = String::with_capacity(32);
        self.write_css_rgba(&mut buffer);
        buffer
    }

    /// Writes the CSS `rgba()` representation into the provided buffer.
    ///
    /// Reuses the caller's allocation so per-frame color conversions in tight
    /// render loops avoid the `format!` machinery and repeated allocation.
    ///
    /// # Arguments
    ///
    /// - `&mut String` - The buffer to append the CSS color string to.
    pub fn write_css_rgba(&self, buffer: &mut String) {
        let red: i32 = (self.get_red() * 255.0).round() as i32;
        let green: i32 = (self.get_green() * 255.0).round() as i32;
        let blue: i32 = (self.get_blue() * 255.0).round() as i32;
        let alpha: f64 = self.get_alpha();
        let _: FmtResult = write!(buffer, "rgba({red}, {green}, {blue}, {alpha})");
    }

    /// Returns black (0, 0, 0, 1).
    ///
    /// # Returns
    ///
    /// - `Color` - The black color.
    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0, 1.0)
    }

    /// Returns white (1, 1, 1, 1).
    ///
    /// # Returns
    ///
    /// - `Color` - The white color.
    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0, 1.0)
    }

    /// Returns transparent (0, 0, 0, 0).
    ///
    /// # Returns
    ///
    /// - `Color` - The transparent color.
    pub fn transparent() -> Color {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Performs linear interpolation between this color and `other` by `t`,
    /// interpolating each channel (red, green, blue, alpha) independently.
    ///
    /// # Arguments
    ///
    /// - `Color` - The target color.
    /// - `f64` - The interpolation factor, typically in the range 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// - `Color` - The interpolated color.
    pub fn lerp(&self, other: Color, factor: f64) -> Color {
        Color::new(
            self.get_red().lerp(other.get_red(), factor),
            self.get_green().lerp(other.get_green(), factor),
            self.get_blue().lerp(other.get_blue(), factor),
            self.get_alpha().lerp(other.get_alpha(), factor),
        )
    }
}

/// Implements `Interpolable` for `Color`.
impl Interpolable for Color {
    /// Linearly interpolates toward `other` by the supplied `factor`.
    ///
    /// # Arguments
    ///
    /// - `Color` - The opposite endpoint of the interpolation.
    /// - `f64` - Interpolation factor; typically `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Color` - The linearly-interpolated value.
    fn lerp(&self, other: Color, factor: f64) -> Color {
        Color::lerp(self, other, factor)
    }
}

/// Implements `Default` for `Color` as opaque black.
impl Default for Color {
    /// Constructs a default [`Color`] value.
    ///
    /// # Returns
    ///
    /// - `Color` - A default-constructed instance with the documented initial state.
    fn default() -> Color {
        Color::black()
    }
}

/// Implements the [`Vector`] trait for `Vector3D`, forwarding every method to
/// the inherent implementation on the struct.
///
/// `Vector3D` also offers 3D-specific operations that are not part of the
/// trait surface: `cross` (returning `Vector3D`), `direction_to`,
/// `distance_to`, `scale`, and `normalize`. These remain inherent.
impl Vector for Vector3D {
    /// Returns the zero vector of this dimension.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The zero vector of this dimension.
    fn zero() -> Vector3D {
        Vector3D::zero()
    }

    /// Returns the dot product of `self` and `other`.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - Other vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The dot product of `self` and `other`.
    fn dot(&self, other: Vector3D) -> f64 {
        Vector3D::dot(self, other)
    }

    /// Returns the Euclidean magnitude (length) of the vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The Euclidean magnitude of the vector.
    fn magnitude(&self) -> f64 {
        Vector3D::magnitude(self)
    }

    /// Returns the squared magnitude (no square-root) of the vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The squared magnitude of the vector (no square root).
    fn magnitude_squared(&self) -> f64 {
        Vector3D::magnitude_squared(self)
    }

    /// Returns the unit-length direction along `self`.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The unit-length direction; undefined when the vector is zero.
    fn normalized(&self) -> Vector3D {
        Vector3D::normalized(self)
    }

    /// Returns the vector multiplied by `scalar`.
    ///
    /// # Arguments
    ///
    /// - `f64` - Scalar multiplier.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The vector scaled by `scalar`.
    fn scaled(&self, scalar: f64) -> Vector3D {
        Vector3D::scaled(self, scalar)
    }

    /// Linearly interpolates toward `other` by the supplied `factor`.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The opposite endpoint of the interpolation.
    /// - `f64` - Interpolation factor; typically `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The linearly-interpolated value.
    fn lerp(&self, other: Vector3D, factor: f64) -> Vector3D {
        Vector3D::lerp(self, other, factor)
    }
}

/// Implements methods and operator overloading for `Vector3D`.
impl Vector3D {
    /// Returns the zero vector (0.0, 0.0, 0.0).
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The zero vector.
    pub fn zero() -> Vector3D {
        Vector3D::new(0.0, 0.0, 0.0)
    }

    /// Returns the unit vector pointing right (1.0, 0.0, 0.0).
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The right unit vector.
    pub fn right() -> Vector3D {
        Vector3D::new(1.0, 0.0, 0.0)
    }

    /// Returns the unit vector pointing up (0.0, 1.0, 0.0).
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The up unit vector.
    pub fn up() -> Vector3D {
        Vector3D::new(0.0, 1.0, 0.0)
    }

    /// Returns the unit vector pointing forward (0.0, 0.0, -1.0).
    ///
    /// In a right-handed coordinate system where -z is forward.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The forward unit vector.
    pub fn forward() -> Vector3D {
        Vector3D::new(0.0, 0.0, -1.0)
    }

    /// Returns the magnitude (length) of the vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The magnitude of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.get_x() * self.get_x() + self.get_y() * self.get_y() + self.get_z() * self.get_z())
            .sqrt()
    }

    /// Returns the squared magnitude of the vector.
    ///
    /// Avoids a square root, making it faster for comparison-only use cases.
    ///
    /// # Returns
    ///
    /// - `f64` - The squared magnitude of the vector.
    pub fn magnitude_squared(&self) -> f64 {
        self.get_x() * self.get_x() + self.get_y() * self.get_y() + self.get_z() * self.get_z()
    }

    /// Returns a normalized (unit length) copy of this vector.
    ///
    /// Returns the zero vector if the magnitude is zero.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The normalized vector.
    pub fn normalized(&self) -> Vector3D {
        let mag: f64 = self.magnitude();
        if mag < EPSILON {
            return Vector3D::zero();
        }
        Vector3D::new(self.get_x() / mag, self.get_y() / mag, self.get_z() / mag)
    }

    /// Normalizes this vector in place.
    pub fn normalize(&mut self) {
        let mag: f64 = self.magnitude();
        if mag < EPSILON {
            self.set_x(0.0);
            self.set_y(0.0);
            self.set_z(0.0);
            return;
        }
        self.set_x(self.get_x() / mag);
        self.set_y(self.get_y() / mag);
        self.set_z(self.get_z() / mag);
    }

    /// Computes the dot product with another vector.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The other vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The dot product.
    pub fn dot(&self, other: Vector3D) -> f64 {
        self.get_x() * other.get_x() + self.get_y() * other.get_y() + self.get_z() * other.get_z()
    }

    /// Computes the 3D cross product with another vector.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The other vector.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The cross product vector.
    pub fn cross(&self, other: Vector3D) -> Vector3D {
        Vector3D::new(
            self.get_y() * other.get_z() - self.get_z() * other.get_y(),
            self.get_z() * other.get_x() - self.get_x() * other.get_z(),
            self.get_x() * other.get_y() - self.get_y() * other.get_x(),
        )
    }

    /// Returns the distance from this point to another.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The target point.
    ///
    /// # Returns
    ///
    /// - `f64` - The Euclidean distance.
    pub fn distance_to(&self, other: Vector3D) -> f64 {
        (other - *self).magnitude()
    }

    /// Returns the squared distance from this point to another.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The target point.
    ///
    /// # Returns
    ///
    /// - `f64` - The squared Euclidean distance.
    pub fn distance_squared_to(&self, other: Vector3D) -> f64 {
        (other - *self).magnitude_squared()
    }

    /// Returns a unit vector pointing from this point to another.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The target point.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The direction unit vector.
    pub fn direction_to(&self, other: Vector3D) -> Vector3D {
        (other - *self).normalized()
    }

    /// Returns a linearly interpolated vector between this and another.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The target vector.
    /// - `f64` - The interpolation factor.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The interpolated vector.
    pub fn lerp(&self, other: Vector3D, factor: f64) -> Vector3D {
        Vector3D::new(
            self.get_x() + (other.get_x() - self.get_x()) * factor,
            self.get_y() + (other.get_y() - self.get_y()) * factor,
            self.get_z() + (other.get_z() - self.get_z()) * factor,
        )
    }

    /// Scales this vector by a scalar factor.
    ///
    /// # Arguments
    ///
    /// - `f64` - The scalar factor.
    pub fn scale(&mut self, scalar: f64) {
        self.set_x(self.get_x() * scalar);
        self.set_y(self.get_y() * scalar);
        self.set_z(self.get_z() * scalar);
    }

    /// Returns a scaled copy of this vector.
    ///
    /// # Arguments
    ///
    /// - `f64` - The scalar factor.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The scaled vector.
    pub fn scaled(&self, scalar: f64) -> Vector3D {
        Vector3D::new(
            self.get_x() * scalar,
            self.get_y() * scalar,
            self.get_z() * scalar,
        )
    }

    /// Rotates this vector by a quaternion.
    ///
    /// # Arguments
    ///
    /// - `Quaternion` - The rotation quaternion.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The rotated vector.
    pub fn rotated_by(&self, quaternion: Quaternion) -> Vector3D {
        let pure: Quaternion = Quaternion::new(self.get_x(), self.get_y(), self.get_z(), 0.0);
        let result: Quaternion = quaternion * pure * quaternion.conjugate();
        Vector3D::new(result.get_x(), result.get_y(), result.get_z())
    }
}

/// Implements `Interpolable` for `Vector3D`.
impl Interpolable for Vector3D {
    /// Linearly interpolates toward `other` by the supplied `factor`.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The opposite endpoint of the interpolation.
    /// - `f64` - Interpolation factor; typically `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The linearly-interpolated value.
    fn lerp(&self, other: Vector3D, factor: f64) -> Vector3D {
        Vector3D::lerp(self, other, factor)
    }
}

/// Implements vector addition.
impl Add for Vector3D {
    type Output = Vector3D;
    /// Adds `other` to `self`.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - Other operand.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - Sum of `self` and `other`.
    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D::new(
            self.get_x() + other.get_x(),
            self.get_y() + other.get_y(),
            self.get_z() + other.get_z(),
        )
    }
}

/// Implements vector subtraction.
impl Sub for Vector3D {
    type Output = Vector3D;
    /// Subtracts `other` from `self`.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - Operand to subtract.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - `self` minus `other`.
    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D::new(
            self.get_x() - other.get_x(),
            self.get_y() - other.get_y(),
            self.get_z() - other.get_z(),
        )
    }
}

/// Implements scalar multiplication.
impl Mul<f64> for Vector3D {
    type Output = Vector3D;
    /// Multiplies `self` and `other` (or `scalar`).
    ///
    /// # Arguments
    ///
    /// - `f64` - Other operand or scalar.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - Product of `self` and the operand.
    fn mul(self, scalar: f64) -> Vector3D {
        Vector3D::new(
            self.get_x() * scalar,
            self.get_y() * scalar,
            self.get_z() * scalar,
        )
    }
}

/// Implements vector negation.
impl Neg for Vector3D {
    type Output = Vector3D;
    /// Negates `self`.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - Negated vector.
    fn neg(self) -> Vector3D {
        Vector3D::new(-self.get_x(), -self.get_y(), -self.get_z())
    }
}

/// Implements in-place vector addition.
impl AddAssign for Vector3D {
    /// Adds `other` to `self` in place.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - Other operand.
    fn add_assign(&mut self, other: Vector3D) {
        self.set_x(self.get_x() + other.get_x());
        self.set_y(self.get_y() + other.get_y());
        self.set_z(self.get_z() + other.get_z());
    }
}

/// Implements in-place vector subtraction.
impl SubAssign for Vector3D {
    /// Subtracts `other` from `self` in place.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - Operand to subtract.
    fn sub_assign(&mut self, other: Vector3D) {
        self.set_x(self.get_x() - other.get_x());
        self.set_y(self.get_y() - other.get_y());
        self.set_z(self.get_z() - other.get_z());
    }
}

/// Implements in-place scalar multiplication.
impl MulAssign<f64> for Vector3D {
    /// Multiplies `self` by `scalar` in place.
    ///
    /// # Arguments
    ///
    /// - `f64` - Scalar multiplier.
    fn mul_assign(&mut self, scalar: f64) {
        self.set_x(self.get_x() * scalar);
        self.set_y(self.get_y() * scalar);
        self.set_z(self.get_z() * scalar);
    }
}

/// Implements quaternion operations for `Quaternion`.
impl Quaternion {
    /// Returns the identity quaternion (0, 0, 0, 1) representing no rotation.
    ///
    /// # Returns
    ///
    /// - `Quaternion` - The identity quaternion.
    pub fn identity() -> Quaternion {
        Quaternion::new(0.0, 0.0, 0.0, 1.0)
    }

    /// Creates a quaternion from a rotation around an axis.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The rotation axis (should be normalized).
    /// - `f64` - The rotation angle in radians.
    ///
    /// # Returns
    ///
    /// - `Quaternion` - The rotation quaternion.
    pub fn from_axis_angle(axis: Vector3D, angle: f64) -> Quaternion {
        let half: f64 = angle * 0.5;
        let sin_half: f64 = half.sin();
        let cos_half: f64 = half.cos();
        let normalized_axis: Vector3D = axis.normalized();
        Quaternion::new(
            normalized_axis.get_x() * sin_half,
            normalized_axis.get_y() * sin_half,
            normalized_axis.get_z() * sin_half,
            cos_half,
        )
    }

    /// Creates a quaternion from Euler angles (yaw, pitch, roll) in radians.
    ///
    /// # Arguments
    ///
    /// - `f64` - The yaw (rotation around y axis) in radians.
    /// - `f64` - The pitch (rotation around x axis) in radians.
    /// - `f64` - The roll (rotation around z axis) in radians.
    ///
    /// # Returns
    ///
    /// - `Quaternion` - The rotation quaternion.
    pub fn from_euler(yaw: f64, pitch: f64, roll: f64) -> Quaternion {
        let half_yaw: f64 = yaw * 0.5;
        let half_pitch: f64 = pitch * 0.5;
        let half_roll: f64 = roll * 0.5;
        let cy: f64 = half_yaw.cos();
        let sy: f64 = half_yaw.sin();
        let cp: f64 = half_pitch.cos();
        let sp: f64 = half_pitch.sin();
        let cr: f64 = half_roll.cos();
        let sr: f64 = half_roll.sin();
        Quaternion::new(
            sp * cy * cr + cp * sy * sr,
            cp * sy * cr - sp * cy * sr,
            cp * cy * sr - sp * sy * cr,
            sp * sy * sr + cp * cy * cr,
        )
    }

    /// Returns the magnitude of the quaternion.
    ///
    /// # Returns
    ///
    /// - `f64` - The magnitude.
    pub fn magnitude(&self) -> f64 {
        (self.get_x() * self.get_x()
            + self.get_y() * self.get_y()
            + self.get_z() * self.get_z()
            + self.get_w() * self.get_w())
        .sqrt()
    }

    /// Returns a normalized copy of this quaternion.
    ///
    /// # Returns
    ///
    /// - `Quaternion` - The normalized quaternion.
    pub fn normalized(&self) -> Quaternion {
        let mag: f64 = self.magnitude();
        if mag < EPSILON {
            return Quaternion::identity();
        }
        let inv: f64 = 1.0 / mag;
        Quaternion::new(
            self.get_x() * inv,
            self.get_y() * inv,
            self.get_z() * inv,
            self.get_w() * inv,
        )
    }

    /// Returns the conjugate of this quaternion.
    ///
    /// # Returns
    ///
    /// - `Quaternion` - The conjugate quaternion.
    pub fn conjugate(&self) -> Quaternion {
        Quaternion::new(-self.get_x(), -self.get_y(), -self.get_z(), self.get_w())
    }

    /// Computes the dot product with another quaternion.
    ///
    /// # Arguments
    ///
    /// - `Quaternion` - The other quaternion.
    ///
    /// # Returns
    ///
    /// - `f64` - The dot product.
    pub fn dot(&self, other: Quaternion) -> f64 {
        self.get_x() * other.get_x()
            + self.get_y() * other.get_y()
            + self.get_z() * other.get_z()
            + self.get_w() * other.get_w()
    }

    /// Performs spherical linear interpolation between this and another quaternion.
    ///
    /// # Arguments
    ///
    /// - `Quaternion` - The target quaternion.
    /// - `f64` - The interpolation factor in the range 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// - `Quaternion` - The interpolated quaternion.
    pub fn slerp(&self, other: Quaternion, factor: f64) -> Quaternion {
        let mut cos_theta: f64 = self.dot(other);
        let target: Quaternion = if cos_theta < 0.0 {
            cos_theta = -cos_theta;
            Quaternion::new(
                -other.get_x(),
                -other.get_y(),
                -other.get_z(),
                -other.get_w(),
            )
        } else {
            other
        };
        if cos_theta > 1.0 - EPSILON {
            return Quaternion::new(
                self.get_x() + (target.get_x() - self.get_x()) * factor,
                self.get_y() + (target.get_y() - self.get_y()) * factor,
                self.get_z() + (target.get_z() - self.get_z()) * factor,
                self.get_w() + (target.get_w() - self.get_w()) * factor,
            )
            .normalized();
        }
        let theta: f64 = cos_theta.acos();
        let sin_theta: f64 = theta.sin();
        let factor_a: f64 = ((1.0 - factor) * theta).sin() / sin_theta;
        let factor_b: f64 = (factor * theta).sin() / sin_theta;
        Quaternion::new(
            self.get_x() * factor_a + target.get_x() * factor_b,
            self.get_y() * factor_a + target.get_y() * factor_b,
            self.get_z() * factor_a + target.get_z() * factor_b,
            self.get_w() * factor_a + target.get_w() * factor_b,
        )
    }
}

/// Implements quaternion multiplication.
impl Mul for Quaternion {
    type Output = Quaternion;
    /// Multiplies `self` and `other` (or `scalar`).
    ///
    /// # Arguments
    ///
    /// - `Quaternion` - Other operand or scalar.
    ///
    /// # Returns
    ///
    /// - `Quaternion` - Product of `self` and the operand.
    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion::new(
            self.get_w() * other.get_x()
                + self.get_x() * other.get_w()
                + self.get_y() * other.get_z()
                - self.get_z() * other.get_y(),
            self.get_w() * other.get_y() - self.get_x() * other.get_z()
                + self.get_y() * other.get_w()
                + self.get_z() * other.get_x(),
            self.get_w() * other.get_z() + self.get_x() * other.get_y()
                - self.get_y() * other.get_x()
                + self.get_z() * other.get_w(),
            self.get_w() * other.get_w()
                - self.get_x() * other.get_x()
                - self.get_y() * other.get_y()
                - self.get_z() * other.get_z(),
        )
    }
}

/// Implements matrix operations for `Matrix4x4`.
impl Matrix4x4 {
    /// Returns the identity matrix.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The identity matrix.
    pub fn identity() -> Matrix4x4 {
        Matrix4x4::new([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    /// Creates a translation matrix.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The translation vector.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The translation matrix.
    pub fn translation(translation: Vector3D) -> Matrix4x4 {
        let mut elements: [f64; 16] = Self::identity().get_elements();
        elements[12] = translation.get_x();
        elements[13] = translation.get_y();
        elements[14] = translation.get_z();
        Matrix4x4::new(elements)
    }

    /// Creates a scaling matrix.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The scale factors.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The scaling matrix.
    pub fn scaling(scale: Vector3D) -> Matrix4x4 {
        Matrix4x4::new([
            scale.get_x(),
            0.0,
            0.0,
            0.0,
            0.0,
            scale.get_y(),
            0.0,
            0.0,
            0.0,
            0.0,
            scale.get_z(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }

    /// Creates a rotation matrix from a quaternion.
    ///
    /// # Arguments
    ///
    /// - `Quaternion` - The rotation quaternion.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The rotation matrix.
    pub fn rotation(quaternion: Quaternion) -> Matrix4x4 {
        let xx: f64 = quaternion.get_x() * quaternion.get_x();
        let yy: f64 = quaternion.get_y() * quaternion.get_y();
        let zz: f64 = quaternion.get_z() * quaternion.get_z();
        let xy: f64 = quaternion.get_x() * quaternion.get_y();
        let xz: f64 = quaternion.get_x() * quaternion.get_z();
        let yz: f64 = quaternion.get_y() * quaternion.get_z();
        let wx: f64 = quaternion.get_w() * quaternion.get_x();
        let wy: f64 = quaternion.get_w() * quaternion.get_y();
        let wz: f64 = quaternion.get_w() * quaternion.get_z();
        Matrix4x4::new([
            1.0 - 2.0 * (yy + zz),
            2.0 * (xy + wz),
            2.0 * (xz - wy),
            0.0,
            2.0 * (xy - wz),
            1.0 - 2.0 * (xx + zz),
            2.0 * (yz + wx),
            0.0,
            2.0 * (xz + wy),
            2.0 * (yz - wx),
            1.0 - 2.0 * (xx + yy),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }

    /// Creates a perspective projection matrix.
    ///
    /// # Arguments
    ///
    /// - `f64` - The vertical field of view in radians.
    /// - `f64` - The aspect ratio (width / height).
    /// - `f64` - The near clipping plane distance.
    /// - `f64` - The far clipping plane distance.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The perspective projection matrix.
    pub fn perspective(fov: f64, aspect: f64, near: f64, far: f64) -> Matrix4x4 {
        let f: f64 = 1.0 / (fov * 0.5).tan();
        let range: f64 = far - near;
        Matrix4x4::new([
            f / aspect,
            0.0,
            0.0,
            0.0,
            0.0,
            f,
            0.0,
            0.0,
            0.0,
            0.0,
            -(far + near) / range,
            -1.0,
            0.0,
            0.0,
            -(2.0 * far * near) / range,
            0.0,
        ])
    }

    /// Creates an orthographic projection matrix.
    ///
    /// # Arguments
    ///
    /// - `f64` - The left boundary.
    /// - `f64` - The right boundary.
    /// - `f64` - The bottom boundary.
    /// - `f64` - The top boundary.
    /// - `f64` - The near clipping plane distance.
    /// - `f64` - The far clipping plane distance.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The orthographic projection matrix.
    pub fn orthographic(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) -> Matrix4x4 {
        let rml: f64 = right - left;
        let tmb: f64 = top - bottom;
        let fmn: f64 = far - near;
        Matrix4x4::new([
            2.0 / rml,
            0.0,
            0.0,
            0.0,
            0.0,
            2.0 / tmb,
            0.0,
            0.0,
            0.0,
            0.0,
            -2.0 / fmn,
            0.0,
            -(right + left) / rml,
            -(top + bottom) / tmb,
            -(far + near) / fmn,
            1.0,
        ])
    }

    /// Creates a view matrix using the "look at" convention.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The eye position.
    /// - `Vector3D` - The target position to look at.
    /// - `Vector3D` - The up direction.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The view matrix.
    pub fn look_at(eye: Vector3D, target: Vector3D, up: Vector3D) -> Matrix4x4 {
        let forward: Vector3D = (target - eye).normalized();
        let right: Vector3D = forward.cross(up).normalized();
        let up_orthogonal: Vector3D = right.cross(forward);
        Matrix4x4::new([
            right.get_x(),
            up_orthogonal.get_x(),
            -forward.get_x(),
            0.0,
            right.get_y(),
            up_orthogonal.get_y(),
            -forward.get_y(),
            0.0,
            right.get_z(),
            up_orthogonal.get_z(),
            -forward.get_z(),
            0.0,
            -right.dot(eye),
            -up_orthogonal.dot(eye),
            forward.dot(eye),
            1.0,
        ])
    }

    /// Multiplies this matrix by another using fully unrolled arithmetic.
    ///
    /// Eliminates all loop overhead and allows the compiler to maximize
    /// register allocation and instruction-level parallelism.
    ///
    /// # Arguments
    ///
    /// - `Matrix4x4` - The other matrix.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The product matrix.
    pub fn multiply(&self, other: Matrix4x4) -> Matrix4x4 {
        let a: [f64; 16] = self.get_elements();
        let b: [f64; 16] = other.get_elements();
        Matrix4x4::new([
            a[0] * b[0] + a[4] * b[1] + a[8] * b[2] + a[12] * b[3],
            a[1] * b[0] + a[5] * b[1] + a[9] * b[2] + a[13] * b[3],
            a[2] * b[0] + a[6] * b[1] + a[10] * b[2] + a[14] * b[3],
            a[3] * b[0] + a[7] * b[1] + a[11] * b[2] + a[15] * b[3],
            a[0] * b[4] + a[4] * b[5] + a[8] * b[6] + a[12] * b[7],
            a[1] * b[4] + a[5] * b[5] + a[9] * b[6] + a[13] * b[7],
            a[2] * b[4] + a[6] * b[5] + a[10] * b[6] + a[14] * b[7],
            a[3] * b[4] + a[7] * b[5] + a[11] * b[6] + a[15] * b[7],
            a[0] * b[8] + a[4] * b[9] + a[8] * b[10] + a[12] * b[11],
            a[1] * b[8] + a[5] * b[9] + a[9] * b[10] + a[13] * b[11],
            a[2] * b[8] + a[6] * b[9] + a[10] * b[10] + a[14] * b[11],
            a[3] * b[8] + a[7] * b[9] + a[11] * b[10] + a[15] * b[11],
            a[0] * b[12] + a[4] * b[13] + a[8] * b[14] + a[12] * b[15],
            a[1] * b[12] + a[5] * b[13] + a[9] * b[14] + a[13] * b[15],
            a[2] * b[12] + a[6] * b[13] + a[10] * b[14] + a[14] * b[15],
            a[3] * b[12] + a[7] * b[13] + a[11] * b[14] + a[15] * b[15],
        ])
    }

    /// Transforms a 3D point by this matrix, applying the perspective divide.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The point to transform.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The transformed point.
    pub fn transform_point(&self, point: Vector3D) -> Vector3D {
        let elements: [f64; 16] = self.get_elements();
        let x: f64 = elements[0] * point.get_x()
            + elements[4] * point.get_y()
            + elements[8] * point.get_z()
            + elements[12];
        let y: f64 = elements[1] * point.get_x()
            + elements[5] * point.get_y()
            + elements[9] * point.get_z()
            + elements[13];
        let z: f64 = elements[2] * point.get_x()
            + elements[6] * point.get_y()
            + elements[10] * point.get_z()
            + elements[14];
        let w: f64 = elements[3] * point.get_x()
            + elements[7] * point.get_y()
            + elements[11] * point.get_z()
            + elements[15];
        if w.abs() < EPSILON {
            return Vector3D::new(x, y, z);
        }
        Vector3D::new(x / w, y / w, z / w)
    }
}

/// Implements `Default` for `Quaternion` as the identity quaternion.
impl Default for Quaternion {
    /// Constructs a default [`Quaternion`] value.
    ///
    /// # Returns
    ///
    /// - `Quaternion` - A default-constructed instance with the documented initial state.
    fn default() -> Quaternion {
        Quaternion::identity()
    }
}

/// Implements `Default` for `Matrix4x4` as the identity matrix.
impl Default for Matrix4x4 {
    /// Constructs a default [`Matrix4x4`] value.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - A default-constructed instance with the documented initial state.
    fn default() -> Matrix4x4 {
        Matrix4x4::identity()
    }
}

/// Implements methods for `Transform3D`.
impl Transform3D {
    /// Creates a new transform at the origin with no rotation and unit scale.
    ///
    /// # Returns
    ///
    /// - `Transform3D` - The identity transform.
    pub fn identity() -> Transform3D {
        Transform3D::new(
            Vector3D::zero(),
            Quaternion::identity(),
            Vector3D::new(1.0, 1.0, 1.0),
        )
    }

    /// Translates the position by the given offset.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The translation offset.
    pub fn translate(&mut self, offset: Vector3D) {
        self.set_position(self.get_position() + offset);
    }

    /// Rotates by the given quaternion (post-multiplies).
    ///
    /// # Arguments
    ///
    /// - `Quaternion` - The rotation to apply.
    pub fn rotate(&mut self, rotation: Quaternion) {
        self.set_rotation(rotation * self.get_rotation());
    }

    /// Scales by the given factors.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The scale factors.
    pub fn scale_by(&mut self, factors: Vector3D) {
        let mut scale: Vector3D = self.get_scale();
        scale.set_x(scale.get_x() * factors.get_x());
        scale.set_y(scale.get_y() * factors.get_y());
        scale.set_z(scale.get_z() * factors.get_z());
        self.set_scale(scale);
    }

    /// Applies this transform to a local-space point, returning world-space coordinates.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The local-space point.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The transformed world-space point.
    pub fn apply_to_point(&self, point: Vector3D) -> Vector3D {
        let scale: Vector3D = self.get_scale();
        let scaled: Vector3D = Vector3D::new(
            point.get_x() * scale.get_x(),
            point.get_y() * scale.get_y(),
            point.get_z() * scale.get_z(),
        );
        scaled.rotated_by(self.get_rotation()) + self.get_position()
    }

    /// Converts this transform to a `Matrix4x4`.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The composed transformation matrix.
    pub fn to_matrix(&self) -> Matrix4x4 {
        let translation: Matrix4x4 = Matrix4x4::translation(self.get_position());
        let rotation: Matrix4x4 = Matrix4x4::rotation(self.get_rotation());
        let scaling: Matrix4x4 = Matrix4x4::scaling(self.get_scale());
        translation.multiply(rotation).multiply(scaling)
    }
}

/// Implements `Default` for `Transform3D` as the identity transform.
impl Default for Transform3D {
    /// Constructs a default [`Transform3D`] value.
    ///
    /// # Returns
    ///
    /// - `Transform3D` - A default-constructed instance with the documented initial state.
    fn default() -> Transform3D {
        Transform3D::identity()
    }
}

/// Implements methods for `AABB3D`.
impl AABB3D {
    /// Creates an AABB from a center point and dimensions.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The center point.
    /// - `f64` - The width.
    /// - `f64` - The height.
    /// - `f64` - The depth.
    ///
    /// # Returns
    ///
    /// - `AABB3D` - The new bounding box.
    pub fn from_center(center: Vector3D, width: f64, height: f64, depth: f64) -> AABB3D {
        AABB3D::new(
            Vector3D::new(
                center.get_x() - width * 0.5,
                center.get_y() - height * 0.5,
                center.get_z() - depth * 0.5,
            ),
            Vector3D::new(
                center.get_x() + width * 0.5,
                center.get_y() + height * 0.5,
                center.get_z() + depth * 0.5,
            ),
        )
    }

    /// Returns the center point of the bounding box.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The center point.
    pub fn center(&self) -> Vector3D {
        Vector3D::new(
            (self.get_min().get_x() + self.get_max().get_x()) * 0.5,
            (self.get_min().get_y() + self.get_max().get_y()) * 0.5,
            (self.get_min().get_z() + self.get_max().get_z()) * 0.5,
        )
    }

    /// Returns the dimensions of the bounding box as a vector.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The size vector (width, height, depth).
    pub fn size(&self) -> Vector3D {
        Vector3D::new(
            self.get_max().get_x() - self.get_min().get_x(),
            self.get_max().get_y() - self.get_min().get_y(),
            self.get_max().get_z() - self.get_min().get_z(),
        )
    }

    /// Tests whether a point is inside this bounding box.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The point to test.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the point is inside.
    pub fn contains(&self, point: Vector3D) -> bool {
        point.get_x() >= self.get_min().get_x()
            && point.get_x() <= self.get_max().get_x()
            && point.get_y() >= self.get_min().get_y()
            && point.get_y() <= self.get_max().get_y()
            && point.get_z() >= self.get_min().get_z()
            && point.get_z() <= self.get_max().get_z()
    }

    /// Tests whether this bounding box intersects another.
    ///
    /// # Arguments
    ///
    /// - `AABB3D` - The other bounding box.
    ///
    /// # Returns
    ///
    /// - `bool` - True if they intersect.
    pub fn intersects(&self, other: AABB3D) -> bool {
        self.get_min().get_x() <= other.get_max().get_x()
            && self.get_max().get_x() >= other.get_min().get_x()
            && self.get_min().get_y() <= other.get_max().get_y()
            && self.get_max().get_y() >= other.get_min().get_y()
            && self.get_min().get_z() <= other.get_max().get_z()
            && self.get_max().get_z() >= other.get_min().get_z()
    }
}

/// Implements methods for `Sphere`.
impl Sphere {
    /// Tests whether a point is inside this sphere.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The point to test.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the point is inside.
    pub fn contains(&self, point: Vector3D) -> bool {
        self.get_center().distance_squared_to(point) <= self.get_radius() * self.get_radius()
    }

    /// Tests whether this sphere intersects another.
    ///
    /// # Arguments
    ///
    /// - `Sphere` - The other sphere.
    ///
    /// # Returns
    ///
    /// - `bool` - True if they intersect.
    pub fn intersects(&self, other: Sphere) -> bool {
        let distance_sq: f64 = self.get_center().distance_squared_to(other.get_center());
        let radius_sum: f64 = self.get_radius() + other.get_radius();
        distance_sq <= radius_sum * radius_sum
    }

    /// Returns the volume of the sphere.
    ///
    /// # Returns
    ///
    /// - `f64` - The volume.
    pub fn volume(&self) -> f64 {
        (4.0 / 3.0) * r#const::PI * self.get_radius() * self.get_radius() * self.get_radius()
    }

    /// Returns the surface area of the sphere.
    ///
    /// # Returns
    ///
    /// - `f64` - The surface area.
    pub fn surface_area(&self) -> f64 {
        4.0 * r#const::PI * self.get_radius() * self.get_radius()
    }
}

/// Implements methods for `Plane`.
impl Plane {
    /// Creates a plane from a normal and a point on the plane.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The normal vector.
    /// - `Vector3D` - A point on the plane.
    ///
    /// # Returns
    ///
    /// - `Plane` - The new plane.
    pub fn from_normal_and_point(normal: Vector3D, point: Vector3D) -> Plane {
        let normalized_normal: Vector3D = normal.normalized();
        Plane::new(normalized_normal, -normalized_normal.dot(point))
    }

    /// Returns the signed distance from a point to this plane.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The point to test.
    ///
    /// # Returns
    ///
    /// - `f64` - The signed distance (positive on the normal side).
    pub fn distance_to_point(&self, point: Vector3D) -> f64 {
        self.get_normal().dot(point) + self.get_distance()
    }

    /// Normalizes the plane normal and adjusts the distance accordingly.
    pub fn normalize(&mut self) {
        let mut normal: Vector3D = self.get_normal();
        let mag: f64 = normal.magnitude();
        if mag < EPSILON {
            return;
        }
        normal.set_x(normal.get_x() / mag);
        normal.set_y(normal.get_y() / mag);
        normal.set_z(normal.get_z() / mag);
        self.set_normal(normal);
        self.set_distance(self.get_distance() / mag);
    }
}

/// Implements methods for `Ray3D`.
impl Ray3D {
    /// Returns the point on the ray at the given parameter value.
    ///
    /// # Arguments
    ///
    /// - `f64` - The parameter value (distance along the ray).
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The point at the given distance.
    pub fn point_at(&self, t: f64) -> Vector3D {
        self.get_origin() + self.get_direction().scaled(t)
    }

    /// Tests for intersection with a sphere, returning the nearest distance if hit.
    ///
    /// # Arguments
    ///
    /// - `Sphere` - The sphere to test.
    ///
    /// # Returns
    ///
    /// - `Option<f64>` - The distance to the intersection, or `None`.
    pub fn intersect_sphere(&self, sphere: Sphere) -> Option<f64> {
        let oc: Vector3D = self.get_origin() - sphere.get_center();
        let direction: Vector3D = self.get_direction();
        let a: f64 = direction.dot(direction);
        let b: f64 = 2.0 * oc.dot(direction);
        let c: f64 = oc.dot(oc) - sphere.get_radius() * sphere.get_radius();
        let discriminant: f64 = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d: f64 = discriminant.sqrt();
        let t1: f64 = (-b - sqrt_d) / (2.0 * a);
        if t1 >= 0.0 {
            return Some(t1);
        }
        let t2: f64 = (-b + sqrt_d) / (2.0 * a);
        if t2 >= 0.0 {
            return Some(t2);
        }
        None
    }

    /// Tests for intersection with a plane, returning the distance if hit.
    ///
    /// # Arguments
    ///
    /// - `Plane` - The plane to test.
    ///
    /// # Returns
    ///
    /// - `Option<f64>` - The distance to the intersection, or `None`.
    pub fn intersect_plane(&self, plane: Plane) -> Option<f64> {
        let direction: Vector3D = self.get_direction();
        let normal: Vector3D = plane.get_normal();
        let denom: f64 = direction.dot(normal);
        if denom.abs() < EPSILON {
            return None;
        }
        let t: f64 = -(normal.dot(self.get_origin()) + plane.get_distance()) / denom;
        if t >= 0.0 { Some(t) } else { None }
    }

    /// Tests for intersection with an AABB, returning the nearest distance if hit.
    ///
    /// # Arguments
    ///
    /// - `AABB3D` - The bounding box to test.
    ///
    /// # Returns
    ///
    /// - `Option<f64>` - The distance to the intersection, or `None`.
    pub fn intersect_aabb(&self, aabb: AABB3D) -> Option<f64> {
        let mut t_min: f64 = f64::MIN;
        let mut t_max: f64 = f64::MAX;
        let direction: Vector3D = self.get_direction();
        let origin: Vector3D = self.get_origin();
        let aabb_min: Vector3D = aabb.get_min();
        let aabb_max: Vector3D = aabb.get_max();
        for axis in 0..3usize {
            let (dir_component, origin_component, min_component, max_component) = match axis {
                0 => (
                    direction.get_x(),
                    origin.get_x(),
                    aabb_min.get_x(),
                    aabb_max.get_x(),
                ),
                1 => (
                    direction.get_y(),
                    origin.get_y(),
                    aabb_min.get_y(),
                    aabb_max.get_y(),
                ),
                _ => (
                    direction.get_z(),
                    origin.get_z(),
                    aabb_min.get_z(),
                    aabb_max.get_z(),
                ),
            };
            if dir_component.abs() < EPSILON {
                if origin_component < min_component || origin_component > max_component {
                    return None;
                }
            } else {
                let inv_dir: f64 = 1.0 / dir_component;
                let t1: f64 = (min_component - origin_component) * inv_dir;
                let t2: f64 = (max_component - origin_component) * inv_dir;
                let t_near: f64 = t1.min(t2);
                let t_far: f64 = t1.max(t2);
                t_min = t_min.max(t_near);
                t_max = t_max.min(t_far);
                if t_min > t_max {
                    return None;
                }
            }
        }
        if t_min >= 0.0 {
            Some(t_min)
        } else if t_max >= 0.0 {
            Some(t_max)
        } else {
            None
        }
    }
}
