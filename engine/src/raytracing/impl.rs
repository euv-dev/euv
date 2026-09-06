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

#[cfg(test)]
mod tests {
    use super::*;

    /// A ray that escapes an empty scene returns the ambient color.
    #[test]
    fn trace_miss_returns_ambient() {
        let eye: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
        lights.set_ambient(Vector3D::new(0.2, 0.4, 0.6));
        let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0));
        let occluders: Vec<Occluder> = Vec::new();
        let scene: RayTraceScene = RayTraceScene::new(occluders);
        let color: Vector3D = scene.trace(ray, &lights);
        assert!(
            (color.get_x() - 0.2).abs() < EPSILON,
            "expected ambient red 0.2, got {}",
            color.get_x(),
        );
        assert!(
            (color.get_y() - 0.4).abs() < EPSILON,
            "expected ambient green 0.4, got {}",
            color.get_y(),
        );
        assert!(
            (color.get_z() - 0.6).abs() < EPSILON,
            "expected ambient blue 0.6, got {}",
            color.get_z(),
        );
    }

    /// A ray that hits an emissive sphere returns the sphere's emissive
    /// color (no shadow attenuation because the surface IS the light).
    #[test]
    fn trace_emissive_sphere() {
        let eye: Vector3D = Vector3D::new(0.0, 0.0, 5.0);
        let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
        lights.set_ambient(Vector3D::zero());
        let sphere_material: Material = Material::emissive(Vector3D::new(1.0, 0.0, 0.0));
        let sphere: Occluder = Occluder::sphere(Vector3D::zero(), 1.0, sphere_material);
        let occluders: Vec<Occluder> = vec![sphere];
        let scene: RayTraceScene = RayTraceScene::new(occluders);
        let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 5.0), Vector3D::new(0.0, 0.0, -1.0));
        let color: Vector3D = scene.trace(ray, &lights);
        assert!(
            (color.get_x() - 1.0).abs() < EPSILON,
            "expected emissive red 1.0, got {}",
            color.get_x(),
        );
        assert!(
            color.get_y().abs() < EPSILON,
            "expected emissive green 0.0, got {}",
            color.get_y(),
        );
        assert!(
            color.get_z().abs() < EPSILON,
            "expected emissive blue 0.0, got {}",
            color.get_z(),
        );
    }

    /// A ray that hits a mirror sphere (Phong specular = 1.0) reflects
    /// once and lands on an emissive sphere, returning a mixed color.
    #[test]
    fn trace_reflection_single_bounce() {
        let eye: Vector3D = Vector3D::new(0.0, 0.0, 10.0);
        let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
        lights.set_ambient(Vector3D::zero());
        let mirror_material: Material = Material::phong(Vector3D::zero(), 1.0, 32.0);
        let mirror: Occluder = Occluder::sphere(Vector3D::zero(), 1.0, mirror_material);
        // Emissive sphere along +z past the mirror. Ray bounces straight
        // back along +z after hitting the dead-center +z hemisphere, so
        // place the emissive on that line.
        let emissive_material: Material = Material::emissive(Vector3D::new(0.0, 1.0, 0.0));
        let emissive: Occluder =
            Occluder::sphere(Vector3D::new(0.0, 0.0, 15.0), 1.0, emissive_material);
        let occluders: Vec<Occluder> = vec![mirror, emissive];
        let scene: RayTraceScene = RayTraceScene::new(occluders);
        let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 10.0), Vector3D::new(0.0, 0.0, -1.0));
        let color: Vector3D = scene.trace(ray, &lights);
        assert!(
            color.get_y() > 0.0,
            "expected bounce to bring back some green, got {}",
            color.get_y(),
        );
        assert!(
            color.get_x().abs() < EPSILON,
            "expected red ~0 (no red light), got {}",
            color.get_x(),
        );
        assert!(
            color.get_z().abs() < EPSILON,
            "expected blue ~0 (no blue light), got {}",
            color.get_z(),
        );
    }

    /// Builds the scene mirrored from the /raytrace example: a ground
    /// AABB, a mirror sphere, and an emissive sphere, lit by one
    /// directional sun with a fixed yaw.
    ///
    /// # Returns
    ///
    /// - `(Vec<Occluder>, LightingUniforms)` - The scene occluders and the
    ///   lighting uniforms.
    fn demo_scene() -> (Vec<Occluder>, LightingUniforms) {
        let ground: Occluder = Occluder::aabb(
            Vector3D::new(-5.0, -0.6, -5.0),
            Vector3D::new(5.0, -0.5, 5.0),
            Material::phong(Vector3D::new(0.30, 0.32, 0.36), 0.30, 24.0),
        );
        let mirror: Occluder = Occluder::sphere(
            Vector3D::new(0.0, 0.4, 0.0),
            0.9,
            Material::phong(Vector3D::new(0.05, 0.05, 0.06), 1.0, 64.0),
        );
        let emissive: Occluder = Occluder::sphere(
            Vector3D::new(1.6, 0.6, -1.4),
            0.45,
            Material::emissive(Vector3D::new(1.0, 0.45, 0.10)),
        );
        let occluders: Vec<Occluder> = vec![ground, mirror, emissive];
        let eye: Vector3D = Vector3D::new(0.0, 0.8, 3.5);
        let yaw: f64 = 0.7;
        let light_dir: Vector3D = Vector3D::new(-yaw.cos(), -0.5, -yaw.sin()).normalized();
        let sun: Light = Light::new_directional(light_dir, Vector3D::new(1.0, 0.95, 0.85));
        let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
        lights.set_ambient(Vector3D::new(0.10, 0.10, 0.14));
        lights.add_light(sun);
        (occluders, lights)
    }

    /// `RayTraceScene::trace` is exactly [`RayTraceScene::trace_with_bounces`]
    /// evaluated at the [`RAYTRACE_DEFAULT_MAX_BOUNCES`] limit.
    #[test]
    fn trace_matches_trace_with_bounces_at_default_limit() {
        let (occluders, lights): (Vec<Occluder>, LightingUniforms) = demo_scene();
        let scene: RayTraceScene = RayTraceScene::new(occluders);
        let ray: Ray = Ray::new(
            Vector3D::new(0.0, 0.8, 3.5),
            Vector3D::new(0.0, -0.4, -3.5).normalized(),
        );
        let default_color: Vector3D = scene.trace(ray.clone(), &lights);
        let explicit_color: Vector3D =
            scene.trace_with_bounces(ray, &lights, RAYTRACE_DEFAULT_MAX_BOUNCES);
        assert_eq!(
            default_color, explicit_color,
            "trace must equal trace_with_bounces at the default bounce limit",
        );
    }

    /// `RayTraceScene::closest_hit` matches the analytic intersection
    /// distance for a dead-center ray and returns `None` on a miss.
    #[test]
    fn closest_hit_returns_analytic_t() {
        let (occluders, _lights): (Vec<Occluder>, LightingUniforms) = demo_scene();
        let scene: RayTraceScene = RayTraceScene::new(occluders);
        let dead_center: Ray =
            Ray::new(Vector3D::new(0.0, 0.4, 5.0), Vector3D::new(0.0, 0.0, -1.0));
        let expected_t: f64 = 5.0 - 0.9;
        let hit: Option<Hit> = scene.closest_hit(&dead_center);
        assert!(hit.is_some(), "expected dead-center ray to hit the mirror");
        assert!(
            (hit.expect("checked above").get_t() - expected_t).abs() < 1e-9,
            "expected analytic t {expected_t}",
        );
        let away: Ray = Ray::new(Vector3D::new(0.0, 0.4, 5.0), Vector3D::new(0.0, 0.0, 1.0));
        assert!(
            scene.closest_hit(&away).is_none(),
            "expected ray pointing away from the scene to miss",
        );
    }
}
