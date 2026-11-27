use super::*;

/// Vector arithmetic, normalization, dot/cross products, and reflection helpers for `Vec3`.
impl Vec3 {
    /// Zero vector `(0, 0, 0)`.
    ///
    /// # Returns
    ///
    /// - `Self` - Zero vector.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Uniform vector `(v, v, v)`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Scalar value replicated across all axes.
    ///
    /// # Returns
    ///
    /// - `Self` - Splat vector.
    #[must_use]
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }

    /// Scales `self` by scalar `s`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source vector.
    /// - `f32` - Scalar multiplier.
    ///
    /// # Returns
    ///
    /// - `Self` - Scaled vector.
    #[inline(always)]
    #[must_use]
    pub fn scale(self, s: f32) -> Self {
        Self {
            x: self.get_x() * s,
            y: self.get_y() * s,
            z: self.get_z() * s,
        }
    }

    /// Computes the dot product `self · rhs`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Left vector.
    /// - `Self` - Right vector.
    ///
    /// # Returns
    ///
    /// - `f32` - Scalar dot product.
    #[inline(always)]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f32 {
        self.get_x().mul_add(
            rhs.get_x(),
            self.get_y()
                .mul_add(rhs.get_y(), self.get_z() * rhs.get_z()),
        )
    }

    /// Computes the 3D cross product `self × rhs`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Left vector.
    /// - `Self` - Right vector.
    ///
    /// # Returns
    ///
    /// - `Self` - Orthogonal cross product vector.
    #[inline(always)]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.get_y() * rhs.get_z() - self.get_z() * rhs.get_y(),
            y: self.get_z() * rhs.get_x() - self.get_x() * rhs.get_z(),
            z: self.get_x() * rhs.get_y() - self.get_y() * rhs.get_x(),
        }
    }

    /// Computes squared Euclidean length.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source vector.
    ///
    /// # Returns
    ///
    /// - `f32` - Squared magnitude.
    #[inline(always)]
    #[must_use]
    pub fn length_sq(self) -> f32 {
        self.dot(self)
    }

    /// Computes Euclidean length.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source vector.
    ///
    /// # Returns
    ///
    /// - `f32` - Magnitude.
    #[inline(always)]
    #[must_use]
    pub fn length(self) -> f32 {
        self.length_sq().sqrt()
    }

    /// Returns a unit-length vector pointing along `self`, or `(0, 1, 0)` if degenerate.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source vector.
    ///
    /// # Returns
    ///
    /// - `Self` - Normalized unit vector.
    #[inline(always)]
    #[must_use]
    pub fn normalize(self) -> Self {
        let len: f32 = self.length();
        if len > 1e-6 {
            self.scale(1.0 / len)
        } else {
            Self::new(0.0, 1.0, 0.0)
        }
    }

    /// Linearly interpolates between `self` and `other` by `t`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Start vector.
    /// - `Self` - End vector.
    /// - `f32` - Interpolation factor.
    ///
    /// # Returns
    ///
    /// - `Self` - Interpolated vector.
    #[inline(always)]
    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: (other.get_x() - self.get_x()).mul_add(t, self.get_x()),
            y: (other.get_y() - self.get_y()).mul_add(t, self.get_y()),
            z: (other.get_z() - self.get_z()).mul_add(t, self.get_z()),
        }
    }

    /// Reflects incident vector `self` across unit surface normal `normal`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Incident vector.
    /// - `Self` - Unit surface normal.
    ///
    /// # Returns
    ///
    /// - `Self` - Reflected direction vector.
    #[inline(always)]
    #[must_use]
    pub fn reflect(self, normal: Self) -> Self {
        self.sub(normal.scale(2.0 * self.dot(normal)))
    }

    /// Returns component-wise minimum of `self` and `rhs`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Left vector.
    /// - `Self` - Right vector.
    ///
    /// # Returns
    ///
    /// - `Self` - Component-wise minimum.
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.get_x().min(rhs.get_x()),
            y: self.get_y().min(rhs.get_y()),
            z: self.get_z().min(rhs.get_z()),
        }
    }

    /// Returns component-wise maximum of `self` and `rhs`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Left vector.
    /// - `Self` - Right vector.
    ///
    /// # Returns
    ///
    /// - `Self` - Component-wise maximum.
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.get_x().max(rhs.get_x()),
            y: self.get_y().max(rhs.get_y()),
            z: self.get_z().max(rhs.get_z()),
        }
    }

    /// Converts an sRGB `Color` into linear RGB `Vec3` radiance in `[0.0, 1.0]`.
    ///
    /// # Arguments
    ///
    /// - `Color` - Source RGBA color.
    ///
    /// # Returns
    ///
    /// - `Self` - Normalized linear RGB vector.
    #[must_use]
    pub fn from_color(c: Color) -> Self {
        Self {
            x: (c.get_r() as f32) / 255.0,
            y: (c.get_g() as f32) / 255.0,
            z: (c.get_b() as f32) / 255.0,
        }
    }

    /// Maps a single HDR channel through the ACES filmic tone curve into sRGB byte space.
    ///
    /// # Arguments
    ///
    /// - `f32` - Linear HDR scalar.
    ///
    /// # Returns
    ///
    /// - `u8` - Gamma-encoded sRGB byte.
    #[inline(always)]
    #[must_use]
    fn aces_channel(v: f32) -> u8 {
        let x: f32 = v.max(0.0);
        let num: f32 = x * x.mul_add(2.51, 0.03);
        let den: f32 = x.mul_add(x.mul_add(2.43, 0.59), 0.14);
        let mapped: f32 = num / den;
        let idx: usize = mapped.clamp(0.0, 1.0).mul_add(1024.0, 0.5) as usize;
        LINEAR_TO_SRGB_LUT[idx.min(1024)]
    }

    /// Applies ACES filmic tone mapping and converts linear radiance to RGBA8 `[r, g, b, 255]`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Linear HDR RGB radiance.
    ///
    /// # Returns
    ///
    /// - `[u8; 4]` - Tone-mapped RGBA8 pixel bytes.
    #[inline(always)]
    #[must_use]
    pub fn to_aces_rgba8(self) -> [u8; 4] {
        [
            Self::aces_channel(self.get_x()),
            Self::aces_channel(self.get_y()),
            Self::aces_channel(self.get_z()),
            255,
        ]
    }
}

/// Component-wise vector addition for `Vec3`.
impl Add for Vec3 {
    type Output = Self;

    /// Adds two vectors component-wise.
    ///
    /// # Arguments
    ///
    /// - `Self` - Left vector.
    /// - `Self` - Right vector.
    ///
    /// # Returns
    ///
    /// - `Self` - Component-wise sum.
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.get_x() + rhs.get_x(),
            y: self.get_y() + rhs.get_y(),
            z: self.get_z() + rhs.get_z(),
        }
    }
}

/// Component-wise vector subtraction for `Vec3`.
impl Sub for Vec3 {
    type Output = Self;

    /// Subtracts `rhs` from `self`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Left vector.
    /// - `Self` - Right vector.
    ///
    /// # Returns
    ///
    /// - `Self` - Component-wise difference.
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.get_x() - rhs.get_x(),
            y: self.get_y() - rhs.get_y(),
            z: self.get_z() - rhs.get_z(),
        }
    }
}

/// Component-wise Hadamard vector multiplication for `Vec3`.
impl Mul for Vec3 {
    type Output = Self;

    /// Multiplies two vectors component-wise.
    ///
    /// # Arguments
    ///
    /// - `Self` - Left vector.
    /// - `Self` - Right vector.
    ///
    /// # Returns
    ///
    /// - `Self` - Component-wise product.
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.get_x() * rhs.get_x(),
            y: self.get_y() * rhs.get_y(),
            z: self.get_z() * rhs.get_z(),
        }
    }
}

/// Matrix multiplication `self * rhs` for `Mat4`.
impl Mul for Mat4 {
    type Output = Self;

    /// Multiplies `self * rhs`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Left matrix.
    /// - `Self` - Right matrix.
    ///
    /// # Returns
    ///
    /// - `Self` - Product matrix.
    fn mul(self, rhs: Self) -> Self {
        let a: [f32; 16] = self.get_m();
        let b: [f32; 16] = rhs.get_m();
        let mut out: [f32; 16] = [0.0; 16];
        for col in 0..4 {
            for row in 0..4 {
                out[col * 4 + row] = a[row] * b[col * 4]
                    + a[4 + row] * b[col * 4 + 1]
                    + a[8 + row] * b[col * 4 + 2]
                    + a[12 + row] * b[col * 4 + 3];
            }
        }
        Self { m: out }
    }
}

/// Matrix constructors, projection, and point projection for `Mat4`.
impl Mat4 {
    /// Identity 4x4 matrix.
    ///
    /// # Returns
    ///
    /// - `Self` - Identity matrix.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            m: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    /// Builds a right-handed perspective projection matrix.
    ///
    /// # Arguments
    ///
    /// - `f32` - Vertical FOV in radians.
    /// - `f32` - Viewport aspect ratio `width / height`.
    /// - `f32` - Near plane distance.
    /// - `f32` - Far plane distance.
    ///
    /// # Returns
    ///
    /// - `Self` - Perspective projection matrix.
    #[must_use]
    pub fn perspective(fov_rad: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f: f32 = 1.0 / (fov_rad * 0.5).tan().max(1e-4);
        let nf: f32 = 1.0 / (near - far);
        Self {
            m: [
                f / aspect.max(0.01),
                0.0,
                0.0,
                0.0,
                0.0,
                f,
                0.0,
                0.0,
                0.0,
                0.0,
                (far + near) * nf,
                -1.0,
                0.0,
                0.0,
                2.0 * far * near * nf,
                0.0,
            ],
        }
    }

    /// Builds a right-handed look-at view matrix.
    ///
    /// # Arguments
    ///
    /// - `Vec3` - Camera eye position.
    /// - `Vec3` - Target look-at position.
    /// - `Vec3` - World up direction.
    ///
    /// # Returns
    ///
    /// - `Self` - View transformation matrix.
    #[must_use]
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f: Vec3 = target.sub(eye).normalize();
        let s: Vec3 = f.cross(up).normalize();
        let u: Vec3 = s.cross(f);
        Self {
            m: [
                s.get_x(),
                u.get_x(),
                -f.get_x(),
                0.0,
                s.get_y(),
                u.get_y(),
                -f.get_y(),
                0.0,
                s.get_z(),
                u.get_z(),
                -f.get_z(),
                0.0,
                -s.dot(eye),
                -u.dot(eye),
                f.dot(eye),
                1.0,
            ],
        }
    }

    /// Builds a TRS model matrix (`translate * rotate_y * scale`).
    ///
    /// # Arguments
    ///
    /// - `Vec3` - World translation.
    /// - `f32` - Y-axis yaw rotation in radians.
    /// - `Vec3` - Non-uniform scale.
    ///
    /// # Returns
    ///
    /// - `Self` - Composed model transform matrix.
    #[must_use]
    pub fn from_trs(position: Vec3, yaw_rad: f32, scale: Vec3) -> Self {
        let c: f32 = yaw_rad.cos();
        let s: f32 = yaw_rad.sin();
        Self {
            m: [
                c * scale.get_x(),
                0.0,
                -s * scale.get_x(),
                0.0,
                0.0,
                scale.get_y(),
                0.0,
                0.0,
                s * scale.get_z(),
                0.0,
                c * scale.get_z(),
                0.0,
                position.get_x(),
                position.get_y(),
                position.get_z(),
                1.0,
            ],
        }
    }

    /// Builds a full 3D Euler TRS model matrix (`translate * rotate_y(yaw) * rotate_x(pitch) * rotate_z(roll) * scale`).
    ///
    /// # Arguments
    ///
    /// - `Vec3` - World translation.
    /// - `f32` - Y-axis yaw rotation in radians.
    /// - `f32` - X-axis pitch rotation in radians.
    /// - `f32` - Z-axis roll rotation in radians.
    /// - `Vec3` - Non-uniform scale.
    ///
    /// # Returns
    ///
    /// - `Self` - Composed 3D model transform matrix.
    #[must_use]
    pub fn from_trs_euler(
        position: Vec3,
        yaw_rad: f32,
        pitch_rad: f32,
        roll_rad: f32,
        scale: Vec3,
    ) -> Self {
        if pitch_rad.abs() < 1e-6 && roll_rad.abs() < 1e-6 {
            return Self::from_trs(position, yaw_rad, scale);
        }
        let (cy, sy): (f32, f32) = (yaw_rad.cos(), yaw_rad.sin());
        let (cp, sp): (f32, f32) = (pitch_rad.cos(), pitch_rad.sin());
        let (cr, sr): (f32, f32) = (roll_rad.cos(), roll_rad.sin());
        let r00: f32 = cy * cr + sy * sp * sr;
        let r10: f32 = cp * sr;
        let r20: f32 = -sy * cr + cy * sp * sr;
        let r01: f32 = -cy * sr + sy * sp * cr;
        let r11: f32 = cp * cr;
        let r21: f32 = sy * sr + cy * sp * cr;
        let r02: f32 = sy * cp;
        let r12: f32 = -sp;
        let r22: f32 = cy * cp;
        let sx: f32 = scale.get_x();
        let sy_sc: f32 = scale.get_y();
        let sz: f32 = scale.get_z();
        Self {
            m: [
                r00 * sx,
                r10 * sx,
                r20 * sx,
                0.0,
                r01 * sy_sc,
                r11 * sy_sc,
                r21 * sy_sc,
                0.0,
                r02 * sz,
                r12 * sz,
                r22 * sz,
                0.0,
                position.get_x(),
                position.get_y(),
                position.get_z(),
                1.0,
            ],
        }
    }

    /// Transforms a 3D point returning homogeneous clip coordinates `(x, y, z, w)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Transformation matrix.
    /// - `Vec3` - Input 3D position.
    ///
    /// # Returns
    ///
    /// - `[f32; 4]` - Homogeneous clip-space coordinates.
    #[must_use]
    pub fn mul_point4(&self, p: Vec3) -> [f32; 4] {
        let m: [f32; 16] = self.get_m();
        [
            m[0] * p.get_x() + m[4] * p.get_y() + m[8] * p.get_z() + m[12],
            m[1] * p.get_x() + m[5] * p.get_y() + m[9] * p.get_z() + m[13],
            m[2] * p.get_x() + m[6] * p.get_y() + m[10] * p.get_z() + m[14],
            m[3] * p.get_x() + m[7] * p.get_y() + m[11] * p.get_z() + m[15],
        ]
    }
}

/// Bounding-box union, centroid, surface area, and ray-slab intersection for `Aabb3D`.
impl Aabb3D {
    /// Creates an inverted empty AABB ready to accumulate points via `expand_point`.
    ///
    /// # Returns
    ///
    /// - `Self` - Empty AABB.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            min: Vec3::splat(f32::INFINITY),
            max: Vec3::splat(f32::NEG_INFINITY),
        }
    }

    /// Expands this bounding box to include `point`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable AABB.
    /// - `Vec3` - Point to enclose.
    pub fn expand_point(&mut self, point: Vec3) {
        let mn: Vec3 = self.get_min().min(point);
        let mx: Vec3 = self.get_max().max(point);
        self.set_min(mn);
        self.set_max(mx);
    }

    /// Expands this bounding box to include `other`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable AABB.
    /// - `Self` - Other bounding box.
    pub fn expand_box(&mut self, other: Self) {
        let mn: Vec3 = self.get_min().min(other.get_min());
        let mx: Vec3 = self.get_max().max(other.get_max());
        self.set_min(mn);
        self.set_max(mx);
    }

    /// Returns the geometric center of the AABB.
    ///
    /// # Arguments
    ///
    /// - `&Self` - AABB reference.
    ///
    /// # Returns
    ///
    /// - `Vec3` - Box centroid.
    #[must_use]
    pub fn center(&self) -> Vec3 {
        self.get_min().add(self.get_max()).scale(0.5)
    }

    /// Computes bounding-sphere radius enclosing this box.
    ///
    /// # Arguments
    ///
    /// - `&Self` - AABB reference.
    ///
    /// # Returns
    ///
    /// - `f32` - Enclosing radius.
    #[must_use]
    pub fn radius(&self) -> f32 {
        self.get_max().sub(self.get_min()).length() * 0.5
    }

    /// Computes half surface area (`dx*dy + dy*dz + dz*dx`) for Surface Area Heuristic (SAH) BVH splits.
    ///
    /// # Arguments
    ///
    /// - `&Self` - AABB reference.
    ///
    /// # Returns
    ///
    /// - `f32` - Non-negative half surface area.
    #[must_use]
    pub fn surface_area(&self) -> f32 {
        let d: Vec3 = self.get_max().sub(self.get_min());
        let dx: f32 = d.get_x().max(0.0);
        let dy: f32 = d.get_y().max(0.0);
        let dz: f32 = d.get_z().max(0.0);
        dx * dy + dy * dz + dz * dx
    }

    /// Branchless slab Ray-AABB intersection returning entry distance `tmin` or `f32::INFINITY` on miss.
    ///
    /// # Arguments
    ///
    /// - `&Self` - AABB reference.
    /// - `Vec3` - Ray origin.
    /// - `Vec3` - Precomputed safe reciprocal ray direction.
    /// - `f32` - Minimum valid distance.
    /// - `f32` - Maximum valid distance.
    ///
    /// # Returns
    ///
    /// - `f32` - Near intersection distance if hit, or `f32::INFINITY` if missed.
    #[must_use]
    pub fn intersect_ray_near(&self, origin: Vec3, inv_dir: Vec3, t_min: f32, t_max: f32) -> f32 {
        let mn: Vec3 = self.get_min();
        let mx: Vec3 = self.get_max();
        let tx1: f32 = (mn.get_x() - origin.get_x()) * inv_dir.get_x();
        let tx2: f32 = (mx.get_x() - origin.get_x()) * inv_dir.get_x();
        let mut tmin: f32 = t_min.max(tx1.min(tx2));
        let mut tmax: f32 = t_max.min(tx1.max(tx2));
        let ty1: f32 = (mn.get_y() - origin.get_y()) * inv_dir.get_y();
        let ty2: f32 = (mx.get_y() - origin.get_y()) * inv_dir.get_y();
        tmin = tmin.max(ty1.min(ty2));
        tmax = tmax.min(ty1.max(ty2));
        let tz1: f32 = (mn.get_z() - origin.get_z()) * inv_dir.get_z();
        let tz2: f32 = (mx.get_z() - origin.get_z()) * inv_dir.get_z();
        tmin = tmin.max(tz1.min(tz2));
        tmax = tmax.min(tz1.max(tz2));
        if tmax >= tmin && tmax >= 0.0 {
            tmin
        } else {
            f32::INFINITY
        }
    }

    /// Branchless slab Ray-AABB intersection returning `true` when `ray` hits within `[t_min, t_max]`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - AABB reference.
    /// - `&Ray3D` - Ray with precomputed inverse direction.
    /// - `f32` - Minimum valid distance.
    /// - `f32` - Maximum valid distance.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the ray intersects this box.
    #[must_use]
    pub fn intersects_ray(&self, ray: &Ray3D, t_min: f32, t_max: f32) -> bool {
        self.intersect_ray_near(ray.get_origin(), ray.get_inv_direction(), t_min, t_max)
            .is_finite()
    }
}

/// Ray construction with safe reciprocal components for `Ray3D`.
impl Ray3D {
    /// Safe reciprocal scalar preventing division by zero in slab ray-box tests.
    ///
    /// # Arguments
    ///
    /// - `f32` - Component scalar.
    ///
    /// # Returns
    ///
    /// - `f32` - Clamped reciprocal value.
    #[must_use]
    fn safe_inv(v: f32) -> f32 {
        if v.abs() < 1e-7 {
            if v < 0.0 { -1e7 } else { 1e7 }
        } else {
            1.0 / v
        }
    }

    /// Computes safe reciprocal vector for an already-normalized ray direction.
    ///
    /// # Arguments
    ///
    /// - `Vec3` - Unit ray direction.
    ///
    /// # Returns
    ///
    /// - `Vec3` - Reciprocal direction safe for slab ray-AABB intersection.
    #[must_use]
    pub fn safe_inv_vec(dir: Vec3) -> Vec3 {
        Vec3::new(
            Self::safe_inv(dir.get_x()),
            Self::safe_inv(dir.get_y()),
            Self::safe_inv(dir.get_z()),
        )
    }

    /// Constructs a normalized `Ray3D` with safe reciprocal direction components.
    ///
    /// # Arguments
    ///
    /// - `Vec3` - Ray origin.
    /// - `Vec3` - Ray direction (normalized internally).
    ///
    /// # Returns
    ///
    /// - `Self` - Ready BVH ray.
    #[must_use]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        let dir: Vec3 = direction.normalize();
        Self {
            origin,
            direction: dir,
            inv_direction: Vec3::new(
                Self::safe_inv(dir.get_x()),
                Self::safe_inv(dir.get_y()),
                Self::safe_inv(dir.get_z()),
            ),
        }
    }
}

/// Gribb-Hartmann 6-plane frustum extraction and AABB visibility testing for `Frustum3D`.
impl Frustum3D {
    /// Normalizes a 4-component plane equation `(a, b, c, d)`.
    ///
    /// # Arguments
    ///
    /// - `[f32; 4]` - Raw plane coefficients.
    ///
    /// # Returns
    ///
    /// - `[f32; 4]` - Unit-normal plane coefficients.
    #[must_use]
    fn normalize_plane(p: [f32; 4]) -> [f32; 4] {
        let len: f32 = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt().max(1e-6);
        [p[0] / len, p[1] / len, p[2] / len, p[3] / len]
    }

    /// Extracts the 6 normalized view-frustum planes from a column-major view-projection matrix.
    ///
    /// # Arguments
    ///
    /// - `&Mat4` - Combined `projection * view` matrix.
    ///
    /// # Returns
    ///
    /// - `Self` - Normalized 6-plane frustum.
    #[must_use]
    pub fn from_view_proj(vp: &Mat4) -> Self {
        let m: [f32; 16] = vp.get_m();
        let r0: [f32; 4] = [m[0], m[4], m[8], m[12]];
        let r1: [f32; 4] = [m[1], m[5], m[9], m[13]];
        let r2: [f32; 4] = [m[2], m[6], m[10], m[14]];
        let r3: [f32; 4] = [m[3], m[7], m[11], m[15]];
        Self {
            planes: [
                Self::normalize_plane([r3[0] + r0[0], r3[1] + r0[1], r3[2] + r0[2], r3[3] + r0[3]]),
                Self::normalize_plane([r3[0] - r0[0], r3[1] - r0[1], r3[2] - r0[2], r3[3] - r0[3]]),
                Self::normalize_plane([r3[0] + r1[0], r3[1] + r1[1], r3[2] + r1[2], r3[3] + r1[3]]),
                Self::normalize_plane([r3[0] - r1[0], r3[1] - r1[1], r3[2] - r1[2], r3[3] - r1[3]]),
                Self::normalize_plane([r3[0] + r2[0], r3[1] + r2[1], r3[2] + r2[2], r3[3] + r2[3]]),
                Self::normalize_plane([r3[0] - r2[0], r3[1] - r2[1], r3[2] - r2[2], r3[3] - r2[3]]),
            ],
        }
    }

    /// Returns `true` when `aabb` is at least partially inside the view frustum.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Frustum reference.
    /// - `&Aabb3D` - World-space AABB to test.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if potentially visible, `false` if completely culled.
    #[must_use]
    pub fn intersects_aabb(&self, aabb: &Aabb3D) -> bool {
        let mn: Vec3 = aabb.get_min();
        let mx: Vec3 = aabb.get_max();
        for p in self.get_planes().iter() {
            let px: f32 = if p[0] >= 0.0 { mx.get_x() } else { mn.get_x() };
            let py: f32 = if p[1] >= 0.0 { mx.get_y() } else { mn.get_y() };
            let pz: f32 = if p[2] >= 0.0 { mx.get_z() } else { mn.get_z() };
            if p[0] * px + p[1] * py + p[2] * pz + p[3] < 0.0 {
                return false;
            }
        }
        true
    }
}

/// Default 3D camera framing an open-world street boulevard.
impl Default for Camera3D {
    /// Creates a default street-level perspective camera.
    ///
    /// # Returns
    ///
    /// - `Self` - Default `Camera3D`.
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 9.5, 28.0),
            yaw: -FRAC_PI_2,
            pitch: -0.22,
            fov_deg: 60.0,
            near: 0.3,
            far: 450.0,
        }
    }
}

/// Orientation, movement, and view-projection helpers for `Camera3D`.
impl Camera3D {
    /// Returns the unit forward look vector derived from `(yaw, pitch)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Camera reference.
    ///
    /// # Returns
    ///
    /// - `Vec3` - Forward unit vector.
    #[must_use]
    pub fn forward(&self) -> Vec3 {
        let yaw: f32 = self.get_yaw();
        let pitch: f32 = self.get_pitch();
        let cp: f32 = pitch.cos();
        Vec3::new(yaw.cos() * cp, pitch.sin(), yaw.sin() * cp).normalize()
    }

    /// Returns the unit right vector orthogonal to `forward()` and world up.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Camera reference.
    ///
    /// # Returns
    ///
    /// - `Vec3` - Right unit vector.
    #[must_use]
    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::new(0.0, 1.0, 0.0)).normalize()
    }

    /// Converts a physical camera focal length and full-frame sensor height in millimeters to vertical FOV degrees.
    ///
    /// # Arguments
    ///
    /// - `f32` - Lens focal length in millimeters (e.g., `24.0`, `35.0`, `50.0`, `85.0`).
    /// - `f32` - Camera sensor height in millimeters (`24.0` for 35mm full-frame).
    ///
    /// # Returns
    ///
    /// - `f32` - Vertical field of view in degrees clamped to `[12.0, 120.0]`.
    #[must_use]
    pub fn focal_length_to_fov_deg(focal_length_mm: f32, sensor_height_mm: f32) -> f32 {
        let f_mm: f32 = focal_length_mm.max(8.0);
        let s_mm: f32 = sensor_height_mm.max(4.0);
        (2.0 * (s_mm / (2.0 * f_mm)).atan().to_degrees()).clamp(12.0, 120.0)
    }

    /// Constructs a physically based 35mm full-frame `Camera3D` using lens focal length in millimeters.
    ///
    /// # Arguments
    ///
    /// - `Vec3` - World-space eye position.
    /// - `f32` - Horizontal yaw angle in radians.
    /// - `f32` - Vertical pitch angle in radians.
    /// - `f32` - Lens focal length in millimeters.
    ///
    /// # Returns
    ///
    /// - `Self` - Physically configured perspective camera.
    #[must_use]
    pub fn from_lens(position: Vec3, yaw: f32, pitch: f32, focal_length_mm: f32) -> Self {
        Self {
            position,
            yaw,
            pitch: pitch.clamp(-1.4, 1.4),
            fov_deg: Self::focal_length_to_fov_deg(focal_length_mm, 24.0),
            near: 0.25,
            far: 480.0,
        }
    }

    /// Computes physical exposure value `EV100` from lens aperture (f-stop), shutter speed (seconds), and ISO.
    ///
    /// # Arguments
    ///
    /// - `f32` - Aperture f-number (e.g., `2.8`, `4.0`, `8.0`).
    /// - `f32` - Shutter exposure time in seconds (e.g., `1.0 / 125.0`).
    /// - `f32` - Sensor ISO sensitivity (e.g., `100.0`).
    ///
    /// # Returns
    ///
    /// - `f32` - Physical exposure value `EV100`.
    #[must_use]
    pub fn ev100(aperture_fstop: f32, shutter_seconds: f32, iso: f32) -> f32 {
        let n: f32 = aperture_fstop.max(0.7);
        let t: f32 = shutter_seconds.max(1e-5);
        let s: f32 = iso.max(25.0);
        ((n * n) / t * (100.0 / s)).log2()
    }

    /// Converts an `EV100` exposure value into a linear HDR scene exposure multiplier.
    ///
    /// # Arguments
    ///
    /// - `f32` - Physical exposure value `EV100`.
    ///
    /// # Returns
    ///
    /// - `f32` - Linear exposure multiplier clamped to `[0.05, 8.0]`.
    #[must_use]
    pub fn exposure_scale_from_ev100(ev100: f32) -> f32 {
        let max_lum: f32 = 1.2 * 2.0_f32.powf(ev100);
        (1.0 / max_lum.max(1e-4)).clamp(0.05, 8.0)
    }

    /// Orients this camera's yaw and pitch to aim directly at `target`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable camera reference.
    /// - `Vec3` - World-space look target.
    pub fn look_at(&mut self, target: Vec3) {
        let dir: Vec3 = target.sub(self.get_position()).normalize();
        self.set_yaw(dir.get_z().atan2(dir.get_x()));
        self.set_pitch(dir.get_y().clamp(-0.98, 0.98).asin());
    }

    /// Computes the combined `projection * view` matrix for viewport `aspect`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Camera reference.
    /// - `f32` - Viewport aspect ratio `width / height`.
    ///
    /// # Returns
    ///
    /// - `Mat4` - View-projection matrix.
    #[must_use]
    pub fn view_projection(&self, aspect: f32) -> Mat4 {
        let pos: Vec3 = self.get_position();
        let view: Mat4 = Mat4::look_at(pos, pos.add(self.forward()), Vec3::new(0.0, 1.0, 0.0));
        let proj: Mat4 = Mat4::perspective(
            self.get_fov_deg().to_radians(),
            aspect.max(0.1),
            self.get_near().max(0.05),
            self.get_far().max(10.0),
        );
        proj.mul(view)
    }
}
