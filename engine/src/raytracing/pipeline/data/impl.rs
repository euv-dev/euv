use super::*;

/// Implements the constructors and validation for [`RayTraceSettings`].
impl RayTraceSettings {
    /// Returns the default settings: one ray per pixel and the
    /// [`RAYTRACE_DEFAULT_MAX_BOUNCES_LIMIT`] bounce limit.
    ///
    /// # Returns
    ///
    /// - `RayTraceSettings` - The default settings.
    pub fn new_default() -> RayTraceSettings {
        RayTraceSettings::new(
            RAYTRACE_DEFAULT_SAMPLES_PER_AXIS,
            RAYTRACE_DEFAULT_MAX_BOUNCES_LIMIT,
        )
    }

    /// Returns a settings block with an explicit sampling grid and bounce
    /// limit.
    ///
    /// The arguments are not validated here; call
    /// [`RayTraceSettings::validate`] before packing or rendering.
    ///
    /// # Arguments
    ///
    /// - `u32` - The supersampling grid resolution per axis.
    /// - `u32` - The maximum number of secondary bounces.
    ///
    /// # Returns
    ///
    /// - `RayTraceSettings` - The new settings.
    pub fn with_quality(samples_per_axis: u32, max_bounces: u32) -> RayTraceSettings {
        RayTraceSettings::new(samples_per_axis, max_bounces)
    }

    /// Returns the total number of rays traced per pixel, which is
    /// `samples_per_axis` squared.
    ///
    /// # Returns
    ///
    /// - `u32` - The per-pixel ray count.
    pub fn rays_per_pixel(&self) -> u32 {
        let axis: u32 = self.get_samples_per_axis();
        axis * axis
    }

    /// Checks that both fields are inside the range the CPU renderer and
    /// both shaders support.
    ///
    /// `samples_per_axis` must be within
    /// `1..=`[`RAYTRACE_SAMPLES_PER_AXIS_MAX`] and `max_bounces` within
    /// `0..=`[`RAYTRACE_MAX_BOUNCES_MAX`]. Anything else is rejected rather
    /// than clamped, so a caller never silently gets a different image than
    /// it asked for.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - `Ok(())` when both fields are in range,
    ///   otherwise a message naming the offending field.
    pub fn validate(&self) -> Result<(), String> {
        let samples: u32 = self.get_samples_per_axis();
        if !(RAYTRACE_SAMPLES_PER_AXIS_MIN..=RAYTRACE_SAMPLES_PER_AXIS_MAX).contains(&samples) {
            return Err(RAYTRACE_ERROR_SAMPLES_PER_AXIS.to_string());
        }
        let bounces: u32 = self.get_max_bounces();
        if !(RAYTRACE_MAX_BOUNCES_MIN..=RAYTRACE_MAX_BOUNCES_MAX).contains(&bounces) {
            return Err(RAYTRACE_ERROR_MAX_BOUNCES.to_string());
        }
        Ok(())
    }
}

/// Implements the GPU packing entry point for [`RayTraceScene`].
impl RayTraceScene {
    /// Packs the scene into the `vec4` array both shader backends read.
    ///
    /// The layout is frozen by the shader contract:
    ///
    /// ```text
    /// vec4[0]: node_count, primitive_count, primitive_offset, 0
    /// vec4[1 + 3 * n]: node lower.xyz, start
    /// vec4[2 + 3 * n]: node upper.xyz, count
    /// vec4[3 + 3 * n]: node escape, 0, 0, 0
    /// vec4[primitive_offset + 8 * p]: primitive records
    /// ```
    ///
    /// Primitives are stored in `indices` order so a leaf's `start` field
    /// reads directly into the packed primitive array while each record
    /// still carries the original primitive index the closest-hit tie
    /// breaker compares. The source occluder ordering is never altered.
    ///
    /// An empty scene packs a two-slot header with `node_count == 0`, which
    /// both shaders detect and treat as "every ray misses".
    ///
    /// Non-finite geometry components, components whose magnitude exceeds
    /// the finite `f32` range, and indices above `2^24` (the exact-integer
    /// limit of `f32`) are rejected rather than truncated, because a
    /// truncated node bound would round inward and lose real geometry.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<f32>, String>` - The packed `vec4` array, or the
    ///   reason the scene cannot be represented on a GPU.
    pub fn encode_gpu(&self) -> Result<Vec<f32>, String> {
        let nodes: &[RayTraceNode] = self.get_nodes();
        let indices: &[usize] = self.get_indices();
        let node_count: usize = nodes.len();
        let primitive_count: usize = indices.len();
        let node_slots: usize = node_count
            .checked_mul(RAYTRACE_PACKED_NODE_VEC4S)
            .ok_or_else(|| RAYTRACE_ERROR_SCENE_OVERFLOW.to_string())?;
        let primitive_slots: usize = primitive_count
            .checked_mul(RAYTRACE_PACKED_PRIMITIVE_VEC4S)
            .ok_or_else(|| RAYTRACE_ERROR_SCENE_OVERFLOW.to_string())?;
        let total_slots: usize = RAYTRACE_PACKED_SCENE_HEADER_VEC4S
            .checked_add(node_slots)
            .and_then(|sum: usize| sum.checked_add(primitive_slots))
            .ok_or_else(|| RAYTRACE_ERROR_SCENE_OVERFLOW.to_string())?;
        let total_floats: usize = total_slots
            .checked_mul(4)
            .ok_or_else(|| RAYTRACE_ERROR_SCENE_OVERFLOW.to_string())?;
        if total_floats > RAYTRACE_MAX_PACKED_FLOATS {
            return Err(RAYTRACE_ERROR_SCENE_OVERFLOW.to_string());
        }
        let mut out: Vec<f32> = Vec::with_capacity(total_floats);
        let primitive_offset: usize = RAYTRACE_PACKED_SCENE_HEADER_VEC4S + node_slots;
        out.extend_from_slice(&[
            pack_index(node_count)?,
            pack_index(primitive_count)?,
            pack_index(primitive_offset)?,
            0.0,
        ]);
        for node in nodes.iter() {
            pack_node(&mut out, node)?;
        }
        let occluders: &[Occluder] = self.get_occluders();
        for index in indices.iter() {
            let occluder: &Occluder = occluders.get(*index).ok_or_else(|| {
                format!(
                    "{}: primitive index {} is out of range for {} occluders",
                    RAYTRACE_ERROR_INDEX_RANGE,
                    index,
                    occluders.len()
                )
            })?;
            pack_primitive(&mut out, occluder, *index)?;
        }
        Ok(out)
    }
}
