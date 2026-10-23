use super::*;

/// Implements `Collider` trait and convenience methods for `AabbCollider`.
impl AabbCollider {
    /// Creates a new AABB collider from a center point and dimensions.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The center point.
    /// - `f64` - The width.
    ///
    /// # Returns
    ///
    /// - `AabbCollider` - The new collider.
    pub fn from_center(center: Vector2D, width: f64, height: f64) -> AabbCollider {
        AabbCollider::new(Rect::from_center(center, width, height))
    }

    /// Tests collision with another AABB collider and returns the collision result.
    ///
    /// # Arguments
    ///
    /// - `&AabbCollider` - The other collider.
    ///
    /// # Returns
    ///
    /// - `Option<CollisionResult>` - The collision result, or `None` if no collision.
    pub fn collide_with_aabb(&self, other: &AabbCollider) -> Option<CollisionResult> {
        let self_rect: Rect = self.get_rect();
        let other_rect: Rect = other.get_rect();
        let a_min: Vector2D = self_rect.min();
        let a_max: Vector2D = self_rect.max();
        let b_min: Vector2D = other_rect.min();
        let b_max: Vector2D = other_rect.max();
        let overlap_x: f64 =
            (a_max.get_x().min(b_max.get_x()) - a_min.get_x().max(b_min.get_x())).max(0.0);
        let overlap_y: f64 =
            (a_max.get_y().min(b_max.get_y()) - a_min.get_y().max(b_min.get_y())).max(0.0);
        if overlap_x <= COLLIDER_CONTACT_EPSILON || overlap_y <= COLLIDER_CONTACT_EPSILON {
            return None;
        }
        let (normal, depth) = if overlap_x < overlap_y {
            let direction: f64 = if self_rect.center().get_x() < other_rect.center().get_x() {
                -1.0
            } else {
                1.0
            };
            (Vector2D::new(direction, 0.0), overlap_x)
        } else {
            let direction: f64 = if self_rect.center().get_y() < other_rect.center().get_y() {
                -1.0
            } else {
                1.0
            };
            (Vector2D::new(0.0, direction), overlap_y)
        };
        let contact_point: Vector2D = Vector2D::new(
            a_min
                .get_x()
                .max(b_min.get_x())
                .min(a_max.get_x().min(b_max.get_x())),
            a_min
                .get_y()
                .max(b_min.get_y())
                .min(a_max.get_y().min(b_max.get_y())),
        );
        Some(CollisionResult::new(normal, depth, contact_point))
    }

    /// Tests collision with a circle collider and returns the collision result.
    ///
    /// # Arguments
    ///
    /// - `&CircleCollider` - The circle collider.
    ///
    /// # Returns
    ///
    /// - `Option<CollisionResult>` - The collision result, or `None` if no collision.
    pub fn collide_with_circle(&self, circle: &CircleCollider) -> Option<CollisionResult> {
        let self_rect: Rect = self.get_rect();
        let circle_inner: Circle = circle.get_circle();
        let rect_min: Vector2D = self_rect.min();
        let rect_max: Vector2D = self_rect.max();
        let closest_x: f64 = circle_inner
            .get_center()
            .get_x()
            .clamp(rect_min.get_x(), rect_max.get_x());
        let closest_y: f64 = circle_inner
            .get_center()
            .get_y()
            .clamp(rect_min.get_y(), rect_max.get_y());
        let delta: Vector2D = circle_inner.get_center() - Vector2D::new(closest_x, closest_y);
        let distance_sq: f64 = delta.magnitude_squared();
        if distance_sq >= circle_inner.get_radius() * circle_inner.get_radius() {
            return None;
        }
        let distance: f64 = distance_sq.sqrt();
        let normal: Vector2D = if distance < EPSILON {
            let aabb_center: Vector2D = self_rect.center();
            let center_delta: Vector2D = circle_inner.get_center() - aabb_center;
            if center_delta.magnitude() < EPSILON {
                Vector2D::up()
            } else {
                center_delta.normalized()
            }
        } else {
            delta.scaled(1.0 / distance)
        };
        let depth: f64 = circle_inner.get_radius() - distance;
        let contact_point: Vector2D = Vector2D::new(closest_x, closest_y);
        Some(CollisionResult::new(normal, depth, contact_point))
    }
}

/// Implements the `Collider` trait for `AabbCollider`.
impl Collider for AabbCollider {
    /// Returns the concrete shape variant of the collider.
    ///
    /// # Returns
    ///
    /// - `ColliderShape` - The concrete shape variant for this collider.
    fn shape(&self) -> ColliderShape {
        ColliderShape::Aabb
    }

    /// Returns the axis-aligned bounding rectangle (2D) or box (3D) of the collider.
    ///
    /// # Returns
    ///
    /// - `Rect` - The axis-aligned bounding box of the collider.
    fn bounding_box(&self) -> Rect {
        self.get_rect()
    }

    /// Returns `true` when the supplied point lies inside the collider.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - Point to test.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the point lies inside the collider.
    fn contains_point(&self, point: Vector2D) -> bool {
        self.get_rect().contains(point)
    }

    /// Returns the geometric center of the collider.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The geometric centre of the collider.
    fn center(&self) -> Vector2D {
        self.get_rect().center()
    }
}

/// Implements `Collider` trait and convenience methods for `CircleCollider`.
impl CircleCollider {
    /// Creates a new circle collider from a center point and radius.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The center point.
    /// - `f64` - The radius.
    ///
    /// # Returns
    ///
    /// - `CircleCollider` - The new collider.
    pub fn from_center(center: Vector2D, radius: f64) -> CircleCollider {
        CircleCollider::new(Circle::new(center, radius))
    }

    /// Tests collision with another circle collider and returns the collision result.
    ///
    /// # Arguments
    ///
    /// - `&CircleCollider` - The other collider.
    ///
    /// # Returns
    ///
    /// - `Option<CollisionResult>` - The collision result, or `None` if no collision.
    pub fn collide_with_circle(&self, other: &CircleCollider) -> Option<CollisionResult> {
        let self_circle: Circle = self.get_circle();
        let other_circle: Circle = other.get_circle();
        let delta: Vector2D = other_circle.get_center() - self_circle.get_center();
        let distance: f64 = delta.magnitude();
        let radius_sum: f64 = self_circle.get_radius() + other_circle.get_radius();
        if distance >= radius_sum {
            return None;
        }
        let normal: Vector2D = if distance < EPSILON {
            Vector2D::right()
        } else {
            delta.scaled(1.0 / distance)
        };
        let depth: f64 = radius_sum - distance;
        let contact_point: Vector2D =
            self_circle.get_center() + normal.scaled(self_circle.get_radius());
        Some(CollisionResult::new(normal, depth, contact_point))
    }
}

/// Implements the `Collider` trait for `CircleCollider`.
impl Collider for CircleCollider {
    /// Returns the concrete shape variant of the collider.
    ///
    /// # Returns
    ///
    /// - `ColliderShape` - The concrete shape variant for this collider.
    fn shape(&self) -> ColliderShape {
        ColliderShape::Circle
    }

    /// Returns the axis-aligned bounding rectangle (2D) or box (3D) of the collider.
    ///
    /// # Returns
    ///
    /// - `Rect` - The axis-aligned bounding box of the collider.
    fn bounding_box(&self) -> Rect {
        let circle: Circle = self.get_circle();
        let diameter: f64 = circle.get_radius() * 2.0;
        Rect::from_center(circle.get_center(), diameter, diameter)
    }

    /// Returns `true` when the supplied point lies inside the collider.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - Point to test.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the point lies inside the collider.
    fn contains_point(&self, point: Vector2D) -> bool {
        self.get_circle().contains(point)
    }

    /// Returns the geometric center of the collider.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The geometric centre of the collider.
    fn center(&self) -> Vector2D {
        self.get_circle().get_center()
    }
}

/// Implements `Collider3D` trait and convenience methods for `AabbCollider3D`.
impl AabbCollider3D {
    /// Creates a new 3D AABB collider from a center point and dimensions.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The center point.
    /// - `f64` - The width.
    ///
    /// # Returns
    ///
    /// - `AabbCollider3D` - The new collider.
    pub fn from_center(center: Vector3D, width: f64, height: f64, depth: f64) -> AabbCollider3D {
        AabbCollider3D::new(AABB3D::from_center(center, width, height, depth))
    }

    /// Tests collision with another 3D AABB collider and returns the collision result.
    ///
    /// # Arguments
    ///
    /// - `&AabbCollider3D` - The other collider.
    ///
    /// # Returns
    ///
    /// - `Option<CollisionResult3D>` - The collision result, or `None` if no collision.
    pub fn collide_with_aabb(&self, other: &AabbCollider3D) -> Option<CollisionResult3D> {
        let self_aabb: AABB3D = self.get_aabb();
        let other_aabb: AABB3D = other.get_aabb();
        let a_center: Vector3D = self_aabb.center();
        let b_center: Vector3D = other_aabb.center();
        let a_size: Vector3D = self_aabb.size();
        let b_size: Vector3D = other_aabb.size();
        let overlap_x: f64 =
            (a_size.get_x() + b_size.get_x()) * 0.5 - (a_center.get_x() - b_center.get_x()).abs();
        if overlap_x <= COLLIDER_CONTACT_EPSILON {
            return None;
        }
        let overlap_y: f64 =
            (a_size.get_y() + b_size.get_y()) * 0.5 - (a_center.get_y() - b_center.get_y()).abs();
        if overlap_y <= COLLIDER_CONTACT_EPSILON {
            return None;
        }
        let overlap_z: f64 =
            (a_size.get_z() + b_size.get_z()) * 0.5 - (a_center.get_z() - b_center.get_z()).abs();
        if overlap_z <= COLLIDER_CONTACT_EPSILON {
            return None;
        }
        let (normal, depth) = if overlap_x <= overlap_y && overlap_x <= overlap_z {
            let direction: f64 = if a_center.get_x() < b_center.get_x() {
                -1.0
            } else {
                1.0
            };
            (Vector3D::new(direction, 0.0, 0.0), overlap_x)
        } else if overlap_y <= overlap_z {
            let direction: f64 = if a_center.get_y() < b_center.get_y() {
                -1.0
            } else {
                1.0
            };
            (Vector3D::new(0.0, direction, 0.0), overlap_y)
        } else {
            let direction: f64 = if a_center.get_z() < b_center.get_z() {
                -1.0
            } else {
                1.0
            };
            (Vector3D::new(0.0, 0.0, direction), overlap_z)
        };
        let self_min: Vector3D = self_aabb.get_min();
        let self_max: Vector3D = self_aabb.get_max();
        let other_min: Vector3D = other_aabb.get_min();
        let other_max: Vector3D = other_aabb.get_max();
        let contact_point: Vector3D = Vector3D::new(
            self_min
                .get_x()
                .max(other_min.get_x())
                .min(self_max.get_x().min(other_max.get_x())),
            self_min
                .get_y()
                .max(other_min.get_y())
                .min(self_max.get_y().min(other_max.get_y())),
            self_min
                .get_z()
                .max(other_min.get_z())
                .min(self_max.get_z().min(other_max.get_z())),
        );
        Some(CollisionResult3D::new(normal, depth, contact_point))
    }

    /// Tests collision with a sphere collider and returns the collision result.
    ///
    /// # Arguments
    ///
    /// - `&SphereCollider3D` - The sphere collider.
    ///
    /// # Returns
    ///
    /// - `Option<CollisionResult3D>` - The collision result, or `None` if no collision.
    pub fn collide_with_sphere(&self, sphere: &SphereCollider3D) -> Option<CollisionResult3D> {
        let self_aabb: AABB3D = self.get_aabb();
        let sphere_inner: Sphere = sphere.get_sphere();
        let aabb_min: Vector3D = self_aabb.get_min();
        let aabb_max: Vector3D = self_aabb.get_max();
        let closest_x: f64 = sphere_inner
            .get_center()
            .get_x()
            .clamp(aabb_min.get_x(), aabb_max.get_x());
        let closest_y: f64 = sphere_inner
            .get_center()
            .get_y()
            .clamp(aabb_min.get_y(), aabb_max.get_y());
        let closest_z: f64 = sphere_inner
            .get_center()
            .get_z()
            .clamp(aabb_min.get_z(), aabb_max.get_z());
        let closest: Vector3D = Vector3D::new(closest_x, closest_y, closest_z);
        let delta: Vector3D = sphere_inner.get_center() - closest;
        let distance_sq: f64 = delta.magnitude_squared();
        if distance_sq >= sphere_inner.get_radius() * sphere_inner.get_radius() {
            return None;
        }
        let distance: f64 = distance_sq.sqrt();
        let normal: Vector3D = if distance < EPSILON {
            let aabb_center: Vector3D = self_aabb.center();
            let center_delta: Vector3D = sphere_inner.get_center() - aabb_center;
            if center_delta.magnitude() < EPSILON {
                Vector3D::up()
            } else {
                center_delta.normalized()
            }
        } else {
            delta.scaled(1.0 / distance)
        };
        let depth: f64 = sphere_inner.get_radius() - distance;
        let contact_point: Vector3D = closest;
        Some(CollisionResult3D::new(normal, depth, contact_point))
    }
}

/// Implements the `Collider3D` trait for `AabbCollider3D`.
impl Collider3D for AabbCollider3D {
    /// Returns the concrete shape variant of the collider.
    ///
    /// # Returns
    ///
    /// - `ColliderShape3D` - The concrete shape variant for this collider.
    fn shape(&self) -> ColliderShape3D {
        ColliderShape3D::Aabb
    }

    /// Returns the axis-aligned bounding rectangle (2D) or box (3D) of the collider.
    ///
    /// # Returns
    ///
    /// - `AABB3D` - The axis-aligned bounding box of the collider.
    fn bounding_box(&self) -> AABB3D {
        self.get_aabb()
    }

    /// Returns `true` when the supplied point lies inside the collider.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - Point to test.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the point lies inside the collider.
    fn contains_point(&self, point: Vector3D) -> bool {
        self.get_aabb().contains(point)
    }

    /// Returns the geometric center of the collider.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The geometric centre of the collider.
    fn center(&self) -> Vector3D {
        self.get_aabb().center()
    }
}

/// Implements `Collider3D` trait and convenience methods for `SphereCollider3D`.
impl SphereCollider3D {
    /// Creates a new 3D sphere collider from a center point and radius.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The center point.
    /// - `f64` - The radius.
    ///
    /// # Returns
    ///
    /// - `SphereCollider3D` - The new collider.
    pub fn from_center(center: Vector3D, radius: f64) -> SphereCollider3D {
        SphereCollider3D::new(Sphere::new(center, radius))
    }

    /// Tests collision with another sphere collider and returns the collision result.
    ///
    /// # Arguments
    ///
    /// - `&SphereCollider3D` - The other collider.
    ///
    /// # Returns
    ///
    /// - `Option<CollisionResult3D>` - The collision result, or `None` if no collision.
    pub fn collide_with_sphere(&self, other: &SphereCollider3D) -> Option<CollisionResult3D> {
        let self_sphere: Sphere = self.get_sphere();
        let other_sphere: Sphere = other.get_sphere();
        let delta: Vector3D = other_sphere.get_center() - self_sphere.get_center();
        let distance: f64 = delta.magnitude();
        let radius_sum: f64 = self_sphere.get_radius() + other_sphere.get_radius();
        if distance >= radius_sum {
            return None;
        }
        let normal: Vector3D = if distance < EPSILON {
            Vector3D::right()
        } else {
            delta.scaled(1.0 / distance)
        };
        let depth: f64 = radius_sum - distance;
        let contact_point: Vector3D =
            self_sphere.get_center() + normal.scaled(self_sphere.get_radius());
        Some(CollisionResult3D::new(normal, depth, contact_point))
    }
}

/// Implements the `Collider3D` trait for `SphereCollider3D`.
impl Collider3D for SphereCollider3D {
    /// Returns the concrete shape variant of the collider.
    ///
    /// # Returns
    ///
    /// - `ColliderShape3D` - The concrete shape variant for this collider.
    fn shape(&self) -> ColliderShape3D {
        ColliderShape3D::Sphere
    }

    /// Returns the axis-aligned bounding rectangle (2D) or box (3D) of the collider.
    ///
    /// # Returns
    ///
    /// - `AABB3D` - The axis-aligned bounding box of the collider.
    fn bounding_box(&self) -> AABB3D {
        let sphere: Sphere = self.get_sphere();
        let diameter: f64 = sphere.get_radius() * 2.0;
        AABB3D::from_center(sphere.get_center(), diameter, diameter, diameter)
    }

    /// Returns `true` when the supplied point lies inside the collider.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - Point to test.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the point lies inside the collider.
    fn contains_point(&self, point: Vector3D) -> bool {
        self.get_sphere().contains(point)
    }

    /// Returns the geometric center of the collider.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The geometric centre of the collider.
    fn center(&self) -> Vector3D {
        self.get_sphere().get_center()
    }
}

/// Implements broad-phase collision checking for `AABB3D`.
impl AABB3D {
    /// Performs a broad-phase check using 3D bounding boxes to quickly reject non-colliding pairs.
    ///
    /// # Arguments
    ///
    /// - `AABB3D` - The first bounding box.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the bounding boxes overlap.
    pub fn broad_phase(a: AABB3D, b: AABB3D) -> bool {
        a.intersects(b)
    }
}
