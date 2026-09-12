use super::*;

/// Implements factory constructors and accessors for [`Ray`] and
/// [`Occluder`].
impl Ray {
    /// Creates a new ray starting at `origin` pointing in `direction`.
    ///
    /// `t_min` and `t_max` default to [`RAYTRACE_DEFAULT_T_MIN`] and
    /// [`RAYTRACE_DEFAULT_T_MAX`]. `depth` defaults to 0.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The ray origin.
    /// - `Vector3D` - The unit direction.
    ///
    /// # Returns
    ///
    /// - `Ray` - The new ray.
    pub fn new(origin: Vector3D, direction: Vector3D) -> Ray {
        Ray {
            origin,
            direction,
            t_min: RAYTRACE_DEFAULT_T_MIN,
            t_max: RAYTRACE_DEFAULT_T_MAX,
            depth: 0,
        }
    }

    /// Computes the world-space point at distance `t` along this ray.
    ///
    /// # Arguments
    ///
    /// - `f64` - The ray parameter.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - `origin + direction * t`.
    pub fn at(&self, t: f64) -> Vector3D {
        self.get_origin() + self.get_direction().scaled(t)
    }

    /// Returns a clone of this ray with `depth` replaced by `depth`.
    ///
    /// # Arguments
    ///
    /// - `u32` - The new recursion depth.
    ///
    /// # Returns
    ///
    /// - `Ray` - The cloned ray with updated depth.
    pub fn with_depth(&self, depth: u32) -> Ray {
        Ray {
            origin: self.get_origin(),
            direction: self.get_direction(),
            t_min: self.get_t_min(),
            t_max: self.get_t_max(),
            depth,
        }
    }
}

/// Implements factory constructors for [`Occluder`].
impl Occluder {
    /// Creates a spherical occluder centered at `center` with `radius`.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The sphere center.
    /// - `f64` - The sphere radius.
    /// - `Material` - The surface material.
    ///
    /// # Returns
    ///
    /// - `Occluder` - The new sphere occluder.
    pub fn sphere(center: Vector3D, radius: f64, material: Material) -> Occluder {
        Occluder {
            kind: OccluderKind::Sphere,
            center,
            extent: Vector3D::new(radius, radius, radius),
            material,
        }
    }

    /// Creates an axis-aligned bounding-box occluder from `min` to `max`.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The AABB minimum corner.
    /// - `Vector3D` - The AABB maximum corner.
    /// - `Material` - The surface material.
    ///
    /// # Returns
    ///
    /// - `Occluder` - The new AABB occluder.
    pub fn aabb(min: Vector3D, max: Vector3D, material: Material) -> Occluder {
        Occluder {
            kind: OccluderKind::Aabb,
            center: min,
            extent: max,
            material,
        }
    }

    /// Returns a list of `(center, radius)` sphere tuples approximating
    /// this occluder, suitable for [`soft_shadow_factor`].
    ///
    /// For sphere occluders this returns `(center, radius)`. For AABB
    /// occluders the bounding sphere is computed conservatively from the
    /// AABB extents.
    ///
    /// # Returns
    ///
    /// - `Vec<(Vector3D, f64)>` - One bounding sphere per occluder.
    pub fn occluder_points(&self) -> Vec<(Vector3D, f64)> {
        collect_occluder_points(std::slice::from_ref(self))
    }
}

/// Implements the constructor and zero-allocation tracing entry points for
/// [`RayTraceScene`].
impl RayTraceScene {
    /// Creates a new scene taking ownership of `occluders` and precomputing
    /// the `(center, radius)` shadow bounding spheres used by
    /// [`soft_shadow_factor`].
    ///
    /// # Arguments
    ///
    /// - `Vec<Occluder>` - All occluding surfaces in the scene.
    ///
    /// # Returns
    ///
    /// - `RayTraceScene` - The new scene with precomputed shadow data.
    pub fn new(occluders: Vec<Occluder>) -> RayTraceScene {
        let shadow_points: Vec<(Vector3D, f64)> = collect_occluder_points(&occluders);
        RayTraceScene {
            occluders,
            shadow_points,
        }
    }

    /// Iteratively traces a ray through the scene and returns the final
    /// shaded color, using the [`RAYTRACE_DEFAULT_MAX_BOUNCES`] constant as
    /// the bounce limit.
    ///
    /// Performs no heap allocation per ray or per bounce: the shadow
    /// bounding spheres precomputed at construction are reused, and no
    /// [`Material`] is cloned. Use [`RayTraceScene::trace_with_bounces`] to
    /// override the bounce limit.
    ///
    /// # Arguments
    ///
    /// - `Ray` - The ray to trace.
    /// - `&LightingUniforms` - Lighting parameters used during shading.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The final traced color.
    pub fn trace(&self, ray: Ray, lights: &LightingUniforms) -> Vector3D {
        self.trace_with_bounces(ray, lights, RAYTRACE_DEFAULT_MAX_BOUNCES)
    }

    /// Iteratively traces a ray through the scene with an explicit bounce
    /// limit and returns the final shaded color.
    ///
    /// On a miss the ambient color scaled by the accumulated specular
    /// throughput is added. On a hit the surface material is evaluated with
    /// [`LightingUniforms::shade`] and, when the hit material has a
    /// non-zero specular component, the trace continues with a reflected
    /// ray up to `max_bounces` times (incrementing the ray's `depth` field
    /// per bounce).
    ///
    /// # Arguments
    ///
    /// - `Ray` - The ray to trace.
    /// - `&LightingUniforms` - Lighting parameters used during shading.
    /// - `u32` - The maximum number of bounces allowed for this ray.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The final traced color.
    pub fn trace_with_bounces(
        &self,
        ray: Ray,
        lights: &LightingUniforms,
        max_bounces: u32,
    ) -> Vector3D {
        trace_bounces(
            ray,
            self.get_occluders(),
            &self.shadow_points,
            lights,
            max_bounces,
        )
    }

    /// Finds the closest intersection between a ray and the scene
    /// occluders.
    ///
    /// The winning occluder's [`Material`] is cloned exactly once, when the
    /// returned [`Hit`] is constructed; losing candidates are never cloned.
    ///
    /// # Arguments
    ///
    /// - `&Ray` - The ray to test.
    ///
    /// # Returns
    ///
    /// - `Option<Hit>` - The closest hit, or `None` if the ray misses.
    pub fn closest_hit(&self, ray: &Ray) -> Option<Hit> {
        let occluders: &[Occluder] = self.get_occluders();
        closest_hit_indexed(ray, occluders).map(
            |(index, t, position, normal): (usize, f64, Vector3D, Vector3D)| Hit {
                t,
                position,
                normal,
                material: occluders[index].get_material().clone(),
            },
        )
    }
}
