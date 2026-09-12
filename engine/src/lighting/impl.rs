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
    /// - `Vector3D` - The RGB intensity multiplier.
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
    /// - `Vector3D` - The RGB intensity multiplier.
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
    /// - `Vector3D` - The unit direction the cone opens along.
    /// - `Vector3D` - The RGB intensity multiplier.
    /// - `f64` - The intensity scalar.
    /// - `f64` - The half-angle of the cone in radians.
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

    /// Creates a Blinn-Phong material with the given albedo, specular
    /// strength, and specular exponent.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The diffuse albedo color.
    /// - `f64` - The specular intensity in the range 0.0..=1.0.
    /// - `f64` - The Phong specular exponent.
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

/// Implements [`LightingUniforms`] builders and the [`LightingUniforms::shade`]
/// entry point used by the ray tracer.
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

    /// Shades a surface point by summing ambient, per-light Lambertian, and
    /// per-light Phong contributions, gated by a soft shadow factor.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The world-space position of the shaded point.
    /// - `Vector3D` - The surface normal (unit length).
    /// - `&Material` - The material at the shaded point.
    /// - `&[(Vector3D, f64)]` - `(center, radius)` occluder tuples used by
    ///   [`soft_shadow_factor`].
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
        let mut color: Vector3D = self.get_ambient();
        let eye: Vector3D = self.get_eye();
        let to_eye: Vector3D = eye - position;
        let view_dist: f64 = to_eye.magnitude();
        let view_dir: Vector3D = if view_dist > EPSILON {
            to_eye.scaled(1.0 / view_dist)
        } else {
            Vector3D::zero()
        };
        for light in self.get_lights().iter() {
            let kind: LightType = light.get_kind();
            let shadow: f64 = match kind {
                LightType::Directional => 1.0,
                LightType::Point | LightType::Spot => {
                    soft_shadow_factor(position, light.get_position(), occluders)
                }
            };
            if shadow <= 0.0 {
                continue;
            }
            let mut lambert_input: Light = light.clone();
            match kind {
                LightType::Directional => {}
                LightType::Point | LightType::Spot => {
                    let to_light: Vector3D = light.get_position() - position;
                    let dist: f64 = to_light.magnitude().max(LIGHTING_POINT_LIGHT_MIN_DISTANCE);
                    let dir: Vector3D = to_light.scaled(1.0 / dist);
                    lambert_input.set_direction(dir);
                }
            }
            let diffuse: Vector3D = compute_lambert(&lambert_input, normal, material);
            let mut spec_input: Light = lambert_input.clone();
            spec_input.set_intensity(
                light.get_intensity() * apply_falloff(view_dist, light.get_falloff()),
            );
            let specular: Vector3D = compute_phong(&spec_input, normal, view_dir, material);
            let mut contribution: Vector3D = diffuse + specular;
            contribution = contribution.scaled(shadow);
            color += contribution;
        }
        let emissive: Vector3D = material.get_emissive();
        color += emissive;
        color
    }
}
