use super::*;

/// Implements factory constructors and shading for [`Light`], [`Material`],
/// and [`LightingUniforms`].
impl Light {
    /// Creates a new directional light pointing in `direction` with `color`.
    ///
    /// The `direction` is normalized internally; intensity defaults to 1.0.
    /// Falloff and spot half-angle are unused for directional lights.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The unit direction toward the light source.
    ///
    /// # Returns
    ///
    /// - `Light` - The new directional light.
    pub fn new_directional(direction: Vector3D, color: Vector3D) -> Light {
        Light::new(
            LightType::Directional,
            Vector3D::zero(),
            direction.normalized(),
            color,
            1.0,
            0.0,
            0.0,
        )
    }

    /// Creates a new point light at `position` with `color` and `intensity`.
    ///
    /// Falloff defaults to 1.0 (inverse-square). The `direction` field is
    /// unused for point lights and is set to the zero vector.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The world-space position of the light.
    /// - `f64` - The intensity scalar.
    ///
    /// # Returns
    ///
    /// - `Light` - The new point light.
    pub fn new_point(position: Vector3D, color: Vector3D, intensity: f64) -> Light {
        Light::new(
            LightType::Point,
            position,
            Vector3D::zero(),
            color,
            intensity,
            1.0,
            0.0,
        )
    }

    /// Creates a new spotlight at `position` shining in `direction`.
    ///
    /// The cone is defined by `half_angle_rad`; the cosine of that angle
    /// is stored for fast cone-test comparisons during shading.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The world-space position of the light.
    /// - `f64` - The intensity scalar.
    ///
    /// # Returns
    ///
    /// - `Light` - The new spotlight.
    pub fn new_spot(
        position: Vector3D,
        direction: Vector3D,
        color: Vector3D,
        intensity: f64,
        half_angle_rad: f64,
    ) -> Light {
        Light::new(
            LightType::Spot,
            position,
            direction.normalized(),
            color,
            intensity,
            1.0,
            half_angle_rad.cos(),
        )
    }
}

/// Implements factory constructors for [`Material`].
impl Material {
    /// Creates a pure-Lambert material with the given albedo.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The diffuse albedo color.
    ///
    /// # Returns
    ///
    /// - `Material` - A Lambertian material.
    pub fn lambert(albedo: Vector3D) -> Material {
        Material::new(
            MaterialKind::Lambert,
            albedo,
            0.0,
            LIGHTING_DEFAULT_SHININESS,
            Vector3D::zero(),
        )
    }

    /// Creates a Phong material with the given albedo, specular
    /// strength, and specular exponent.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The diffuse albedo color.
    /// - `f64` - The specular intensity in the range 0.0..=1.0.
    ///
    /// # Returns
    ///
    /// - `Material` - A Phong material.
    pub fn phong(albedo: Vector3D, specular: f64, shininess: f64) -> Material {
        Material::new(
            MaterialKind::Phong,
            albedo,
            specular,
            shininess,
            Vector3D::zero(),
        )
    }

    /// Creates a material using the [`MaterialKind::Pbr`] approximation.
    ///
    /// The resulting material is shaded as a Lambertian diffuse term scaled
    /// by a Schlick fresnel factor; its `specular` and `shininess` fields are
    /// not consulted on that path. See [`MaterialKind::Pbr`] for what this
    /// model does and does not include — in particular, read it as
    /// "diffuse plus a fresnel rim", not as a physical BRDF.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The diffuse albedo color.
    ///
    /// # Returns
    ///
    /// - `Material` - A PBR-approximation material.
    pub fn pbr(albedo: Vector3D) -> Material {
        Material::new(
            MaterialKind::Pbr,
            albedo,
            0.0,
            LIGHTING_DEFAULT_SHININESS,
            Vector3D::zero(),
        )
    }

    /// Creates a purely emissive material (light source with no shading).
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The self-illumination color.
    ///
    /// # Returns
    ///
    /// - `Material` - An emissive material.
    pub fn emissive(color: Vector3D) -> Material {
        Material::new(MaterialKind::Lambert, Vector3D::zero(), 0.0, 0.0, color)
    }
}

/// Implements [`LightingUniforms`] builders and the shading entry points
/// used by the ray tracer and by any external visibility provider.
impl LightingUniforms {
    /// Creates a uniform set with an empty light list, default ambient,
    /// and the supplied eye position.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The view position used for specular calculations.
    ///
    /// # Returns
    ///
    /// - `LightingUniforms` - The new uniform set.
    pub fn with_eye(eye: Vector3D) -> LightingUniforms {
        LightingUniforms::new(Vec::new(), LIGHTING_DEFAULT_AMBIENT, eye)
    }

    /// Adds a light to the uniform set.
    ///
    /// # Arguments
    ///
    /// - `Light` - The light to append.
    pub fn add_light(&mut self, light: Light) {
        self.get_mut_lights().push(light);
    }

    /// Shades a surface point against `(center, radius)` shadow bounding
    /// spheres, the pre-existing entry point.
    ///
    /// A compatibility shim over
    /// [`LightingUniforms::shade_with_visibility`] that supplies a callback
    /// backed by [`soft_shadow_factor`], retained so existing ray-tracing
    /// callers keep compiling unchanged. New code holding real geometry
    /// should call [`LightingUniforms::shade_with_visibility`] directly and
    /// get exact shadows instead of bounding-sphere ones.
    ///
    /// Directional lights report full visibility here, unchanged from
    /// before: a bounding sphere stores a position but no direction, so
    /// there is no finite segment to cast along and a callback using one
    /// would have to invent a far bound. That far bound is
    /// [`LIGHTING_DIRECTIONAL_VISIBILITY_DISTANCE`] and belongs to
    /// [`LightingUniforms::shade_with_visibility`], where the caller
    /// supplies its own geometry.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The world-space position of the shaded point.
    /// - `&Material` - The material at the shaded point.
    /// - `&[(Vector3D, f64)]` - `(center, radius)` occluder bounding
    ///   spheres, one per occluder.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The final shaded color.
    pub fn shade(
        &self,
        position: Vector3D,
        normal: Vector3D,
        material: &Material,
        occluders: &[(Vector3D, f64)],
    ) -> Vector3D {
        self.shade_with_visibility(
            position,
            normal,
            material,
            |light: &Light, _dir: Vector3D, _max_distance: f64| -> f64 {
                match light.get_kind() {
                    LightType::Directional => 1.0,
                    LightType::Point | LightType::Spot => {
                        soft_shadow_factor(position, light.get_position(), occluders)
                    }
                }
            },
        )
    }

    /// Shades a surface point, asking `visibility` whether each light is
    /// unoccluded along the segment to that light.
    ///
    /// The entry point for real shadow geometry. The callback receives the
    /// light, the unit direction from the shaded point toward the light,
    /// and the maximum distance to cast along that direction, and returns a
    /// visibility factor that multiplies the light's whole contribution.
    /// Returning `1.0` for every light reproduces the unshadowed result
    /// exactly.
    ///
    /// ## The callback contract
    ///
    /// ```text
    /// F: FnMut(&Light, surface_to_light_dir: Vector3D, max_distance: f64) -> f64
    /// ```
    ///
    /// - `surface_to_light_dir` is the **normalized** direction from
    ///   `position` toward the light. For a spot light this points *into*
    ///   the lamp (it is the negation of the cone axis), matching the
    ///   shading convention used for every other term — cast your shadow
    ///   ray along this vector and it runs surface→light.
    /// - `max_distance` is the far bound for that ray: the true
    ///   surface→light distance for point and spot lights, so the ray can
    ///   never reach past the lamp, and
    ///   [`LIGHTING_DIRECTIONAL_VISIBILITY_DISTANCE`] (1000.0, matching
    ///   the ray tracer's `RAYTRACE_DEFAULT_T_MAX`) for directional
    ///   lights, which have no distance of their own.
    /// - The returned factor is clamped to `0.0..=1.0`.
    ///
    /// ## When the callback runs
    ///
    /// At most once per light, and only for lights that can actually
    /// contribute. Lights are skipped before any callback work when they
    /// carry no intensity, fall outside their spot cone, or face away from
    /// the surface (`dot(normal, light_dir) <= 0`). Those tests are free,
    /// and skipping them is the point of the ordering: back-facing and
    /// out-of-cone lights are the common reject path. A light that is fully
    /// occluded still costs one callback — only the callback knows the
    /// geometry.
    ///
    /// ## Evaluation order
    ///
    /// Per light: cone test → facing test → compute `light_dir`, `dist`,
    /// and `view_dir` **once** → visibility callback → falloff →
    /// [`MaterialKind`] dispatch. The [`Light`] itself is only read, never
    /// cloned: a point or spot light's derived direction and attenuating
    /// intensity live in locals for the duration of the iteration, so the
    /// diffuse, specular, and fresnel terms all read the same values.
    ///
    /// [`MaterialKind`] selects which terms run per light:
    /// [`MaterialKind::Lambert`] contributes diffuse only,
    /// [`MaterialKind::Phong`] adds a Phong specular lobe that is zero
    /// unless the surface faces the light, and [`MaterialKind::Pbr`]
    /// scales the diffuse term by a Schlick fresnel factor and adds no
    /// separate specular lobe.
    ///
    /// Ambient is added once and is never shadowed; the material's
    /// emissive term is added last and is likewise unaffected by
    /// visibility.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The world-space position of the shaded point.
    /// - `&Material` - The material at the shaded point.
    /// - `F` - The visibility provider, called once per contributing
    ///   light.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The final shaded color.
    pub fn shade_with_visibility<F>(
        &self,
        position: Vector3D,
        normal: Vector3D,
        material: &Material,
        mut visibility: F,
    ) -> Vector3D
    where
        F: FnMut(&Light, Vector3D, f64) -> f64,
    {
        let mut color: Vector3D = self.get_ambient();
        let view_dir: Vector3D = unit_vector(self.get_eye() - position);
        let material_kind: MaterialKind = material.get_kind();
        for light in self.get_lights().iter() {
            let kind: LightType = light.get_kind();
            // Derived once per light and shared by every term below. The
            // magnitude is taken once, alongside the unit direction, since
            // both come from the same vector. The `Light` itself is only
            // read, never cloned or mutated.
            let (light_dir, distance): (Vector3D, f64) = match kind {
                LightType::Directional => (light.get_direction(), 0.0),
                LightType::Point | LightType::Spot => {
                    unit_vector_and_length(light.get_position() - position)
                }
            };
            // Free rejects first: a light that cannot reach this surface
            // costs no visibility callback at all.
            if !light_reaches_position(light, light_dir) {
                continue;
            }
            // A back-facing surface has zero Lambert and zero specular, so
            // culling here saves a shadow ray per light.
            if normal.dot(light_dir) <= 0.0 {
                continue;
            }
            // `max_distance` is the **true** surface→light distance, so a
            // shadow ray can never reach past the lamp. Only the falloff
            // distance is floored at `LIGHTING_POINT_LIGHT_MIN_DISTANCE`,
            // to keep a coincident surface from dividing by zero; a
            // directional light attenuates not at all and its shadow ray
            // runs to the ray tracer's far bound.
            let (falloff_factor, shadow_distance): (f64, f64) = match kind {
                LightType::Directional => (1.0, LIGHTING_DIRECTIONAL_VISIBILITY_DISTANCE),
                LightType::Point | LightType::Spot => (
                    apply_falloff(
                        distance.max(LIGHTING_POINT_LIGHT_MIN_DISTANCE),
                        light.get_falloff(),
                    ),
                    distance,
                ),
            };
            let shadow: f64 = visibility(light, light_dir, shadow_distance).clamp(0.0, 1.0);
            if shadow <= 0.0 {
                continue;
            }
            // One resolved light, shared by every term below: the diffuse
            // and specular lobes and the fresnel scale all read this one
            // colour, the one attenuated intensity, and the one derived
            // direction, instead of each re-reading the `Light`.
            let sample: LightSample = LightSample {
                color: light.get_color(),
                intensity: light.get_intensity() * falloff_factor,
                direction: light_dir,
            };
            let diffuse: Vector3D = match material_kind {
                MaterialKind::Pbr => {
                    let fresnel: f64 = apply_schlick_fresnel(light_dir, view_dir, normal);
                    lambert_term(&sample, normal, material).scaled(
                        LIGHTING_PBR_ENERGY_CONSERVATION
                            + (1.0 - LIGHTING_PBR_ENERGY_CONSERVATION) * fresnel,
                    )
                }
                MaterialKind::Lambert | MaterialKind::Phong => {
                    lambert_term(&sample, normal, material)
                }
            };
            let specular: Vector3D = match material_kind {
                MaterialKind::Lambert | MaterialKind::Pbr => Vector3D::zero(),
                MaterialKind::Phong => phong_specular_term(&sample, normal, view_dir, material),
            };
            color += (diffuse + specular).scaled(shadow);
        }
        color += material.get_emissive();
        color
    }
}
