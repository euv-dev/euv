use super::*;

/// Converts one `f64` component into a packed `f32`, rounding to nearest.
///
/// Rejects non-finite input and values whose magnitude exceeds the finite
/// `f32` range instead of silently truncating. Round-to-nearest is correct
/// for positions, colours and material scalars, where a half-ulp
/// difference is imperceptible.
///
/// # Arguments
///
/// - `f64` - The value to narrow.
/// - `&str` - The error message for a non-finite value.
/// - `&str` - The error message for a value outside the `f32` range.
///
/// # Returns
///
/// - `Result<f32, String>` - The narrowed value, or the applicable error.
pub fn pack_finite(value: f64, non_finite: &str, out_of_range: &str) -> Result<f32, String> {
    if !value.is_finite() {
        return Err(non_finite.to_string());
    }
    let narrowed: f32 = value as f32;
    if !narrowed.is_finite() {
        return Err(out_of_range.to_string());
    }
    Ok(narrowed)
}

/// Returns the next representable `f32` below `value`.
///
/// The step is one unit in the last place in the direction of decreasing
/// magnitude, which is what a directed rounding needs. Signed zero is
/// handled so the function is total: `-0.0` steps to the smallest negative
/// subnormal and `+0.0` to the smallest positive one.
///
/// # Arguments
///
/// - `f32` - The value to step down from.
///
/// # Returns
///
/// - `f32` - The next lower `f32`, or `value` itself at the negative
///   infinity boundary where no lower value exists.
pub(crate) fn next_down(value: f32) -> f32 {
    if value.is_nan() || value == f32::NEG_INFINITY {
        return value;
    }
    if value > 0.0 {
        return f32::from_bits(value.to_bits() - 1);
    }
    if value == 0.0 {
        return -f32::from_bits(1);
    }
    f32::from_bits(value.to_bits() + 1)
}

/// Returns the next representable `f32` above `value`, the mirror of
/// [`next_down`].
///
/// # Arguments
///
/// - `f32` - The value to step up from.
///
/// # Returns
///
/// - `f32` - The next higher `f32`, or `value` itself at the positive
///   infinity boundary where no higher value exists.
pub(crate) fn next_up(value: f32) -> f32 {
    if value.is_nan() || value == f32::INFINITY {
        return value;
    }
    if value >= 0.0 {
        return f32::from_bits(value.to_bits() + 1);
    }
    if value == 0.0 {
        return f32::from_bits(1);
    }
    f32::from_bits(value.to_bits() - 1)
}

/// Converts one BVH **lower** bound to `f32`, rounding toward negative
/// infinity.
///
/// A plain `as f32` rounds to nearest, so a lower bound can be nudged
/// *inward*; the packed node then no longer contains geometry the CPU
/// hierarchy considers inside it, and the shader would miss a hit the CPU
/// finds. Snapping outward by one ulp whenever the conversion was inexact
/// makes the packed node a conservative superset of the source node, which
/// is the only safe direction for a bounding volume.
///
/// # Arguments
///
/// - `f64` - The source bound.
///
/// # Returns
///
/// - `Result<f32, String>` - A representable `f32` no greater than the
///   source.
pub(crate) fn pack_lower_bound(value: f64) -> Result<f32, String> {
    let narrowed: f32 = pack_finite(value, RAYTRACE_ERROR_NON_FINITE, RAYTRACE_ERROR_F32_RANGE)?;
    if (narrowed as f64) > value {
        // Round-to-nearest landed above the source, so step one ulp down.
        return Ok(next_down(narrowed));
    }
    Ok(narrowed)
}

/// Converts one BVH **upper** bound to `f32`, rounding toward positive
/// infinity. The mirror of [`pack_lower_bound`].
///
/// # Arguments
///
/// - `f64` - The source bound.
///
/// # Returns
///
/// - `Result<f32, String>` - A representable `f32` no less than the source.
pub(crate) fn pack_upper_bound(value: f64) -> Result<f32, String> {
    let narrowed: f32 = pack_finite(value, RAYTRACE_ERROR_NON_FINITE, RAYTRACE_ERROR_F32_RANGE)?;
    if (narrowed as f64) < value {
        // Round-to-nearest landed below the source, so step one ulp up.
        return Ok(next_up(narrowed));
    }
    Ok(narrowed)
}

/// Converts a vector to three `f32` components, rounding to nearest.
///
/// # Arguments
///
/// - `Vector3D` - The vector to pack.
/// - `&str` - The error message used for a non-finite component.
/// - `&str` - The error message used for an `f32`-overflowing component.
///
/// # Returns
///
/// - `Result<[f32; 3], String>` - The packed components, or the first
///   rejection reason.
pub fn pack_vector(
    vector: Vector3D,
    non_finite: &str,
    out_of_range: &str,
) -> Result<[f32; 3], String> {
    let x: f32 = pack_finite(vector.get_x(), non_finite, out_of_range)?;
    let y: f32 = pack_finite(vector.get_y(), non_finite, out_of_range)?;
    let z: f32 = pack_finite(vector.get_z(), non_finite, out_of_range)?;
    Ok([x, y, z])
}

/// Converts a vector to three `f32` components, rounding every component
/// toward negative infinity.
///
/// # Arguments
///
/// - `Vector3D` - The vector to narrow.
/// - `&str` - The error message for a non-finite component.
/// - `&str` - The error message for an out-of-range component.
///
/// # Returns
///
/// - `Result<[f32; 3], String>` - The narrowed components, or the first
///   applicable error.
pub fn pack_vector_lower(
    vector: Vector3D,
    non_finite: &str,
    out_of_range: &str,
) -> Result<[f32; 3], String> {
    if !vector.get_x().is_finite() || !vector.get_y().is_finite() || !vector.get_z().is_finite() {
        return Err(non_finite.to_string());
    }
    let magnitude: f64 = vector
        .get_x()
        .abs()
        .max(vector.get_y().abs())
        .max(vector.get_z().abs());
    if magnitude > RAYTRACE_MAX_F32_MAGNITUDE {
        return Err(out_of_range.to_string());
    }
    Ok([
        pack_lower_bound(vector.get_x())?,
        pack_lower_bound(vector.get_y())?,
        pack_lower_bound(vector.get_z())?,
    ])
}

/// Converts a vector to three `f32` components, rounding every component
/// toward positive infinity.
///
/// # Arguments
///
/// - `Vector3D` - The vector to narrow.
/// - `&str` - The error message for a non-finite component.
/// - `&str` - The error message for an out-of-range component.
///
/// # Returns
///
/// - `Result<[f32; 3], String>` - The narrowed components, or the first
///   applicable error.
pub fn pack_vector_upper(
    vector: Vector3D,
    non_finite: &str,
    out_of_range: &str,
) -> Result<[f32; 3], String> {
    if !vector.get_x().is_finite() || !vector.get_y().is_finite() || !vector.get_z().is_finite() {
        return Err(non_finite.to_string());
    }
    let magnitude: f64 = vector
        .get_x()
        .abs()
        .max(vector.get_y().abs())
        .max(vector.get_z().abs());
    if magnitude > RAYTRACE_MAX_F32_MAGNITUDE {
        return Err(out_of_range.to_string());
    }
    Ok([
        pack_upper_bound(vector.get_x())?,
        pack_upper_bound(vector.get_y())?,
        pack_upper_bound(vector.get_z())?,
    ])
}

/// Converts a `usize` node or primitive index into an exactly
/// representable `f32`.
///
/// `f32` represents integers exactly only up to `2^24`; above that a
/// plain `as f32` truncates, which would make a shader read the wrong node
/// or primitive rather than failing loudly.
///
/// # Arguments
///
/// - `usize` - The index to convert.
///
/// # Returns
///
/// - `Result<f32, String>` - The exactly representable index, or an error
///   once the index passes the `2^24` exact range.
pub fn pack_index(index: usize) -> Result<f32, String> {
    if index > RAYTRACE_MAX_EXACT_F32_INDEX {
        return Err(RAYTRACE_ERROR_INDEX_RANGE.to_string());
    }
    Ok(index as f32)
}

/// Returns the `f32` discriminator both shaders switch on for an
/// [`OccluderKind`].
///
/// # Arguments
///
/// - `OccluderKind` - The source shape.
///
/// # Returns
///
/// - `f32` - The shader discriminator: sphere `0.0`, AABB `1.0`, triangle
///   `2.0`.
pub fn packed_shape_code(kind: OccluderKind) -> f32 {
    match kind {
        OccluderKind::Sphere => 0.0,
        OccluderKind::Aabb => 1.0,
        OccluderKind::Triangle => 2.0,
    }
}

/// Returns the `f32` discriminator both shaders switch on for a
/// [`MaterialKind`].
///
/// # Arguments
///
/// - `MaterialKind` - The source material.
///
/// # Returns
///
/// - `f32` - The shader discriminator: Lambert `0.0`, Phong `1.0`, PBR
///   `2.0`.
pub fn packed_material_code(kind: MaterialKind) -> f32 {
    match kind {
        MaterialKind::Lambert => 0.0,
        MaterialKind::Phong => 1.0,
        MaterialKind::Pbr => 2.0,
    }
}

/// Returns the `f32` discriminator both shaders switch on for a
/// [`LightType`].
///
/// # Arguments
///
/// - `LightType` - The source light.
///
/// # Returns
///
/// - `f32` - The shader discriminator: directional `0.0`, point `1.0`, spot
///   `2.0`.
pub fn packed_light_code(kind: LightType) -> f32 {
    match kind {
        LightType::Directional => 0.0,
        LightType::Point => 1.0,
        LightType::Spot => 2.0,
    }
}

/// Returns half the `f32` unit in the last place at `probe`.
///
/// This is the worst-case magnitude by which a round-to-nearest `f64` to
/// `f32` conversion can move a value, and is what a packed sphere radius is
/// widened by.
///
/// # Arguments
///
/// - `f32` - A probe with the magnitude of interest.
///
/// # Returns
///
/// - `f64` - Half an ulp at `probe`, never zero.
pub(crate) fn half_ulp(probe: f32) -> f64 {
    let up: f64 = next_up(probe) as f64;
    let down: f64 = next_down(probe) as f64;
    let span: f64 = (up - down).abs() * 0.5;
    if span.is_finite() && span > 0.0 {
        return span;
    }
    f32::from_bits(1) as f64
}

/// Returns the packed radius of a sphere whose centre is packed toward
/// negative infinity.
///
/// The packed centre is never above the source, so it can sit up to half an
/// ulp below it on each axis. The radius absorbs the worst-case corner
/// displacement of that drift, `sqrt(3)` half-ulp, and is then rounded up so
/// the packed sphere still contains every point of the source sphere.
///
/// # Arguments
///
/// - `Vector3D` - The sphere centre, sampled for its ulp scale.
/// - `f64` - The source radius.
///
/// # Returns
///
/// - `Result<f32, String>` - The widened radius, rounded up.
pub fn pack_sphere_radius(center: Vector3D, radius: f64) -> Result<f32, String> {
    let probe: f32 = pack_finite(
        center.get_x(),
        RAYTRACE_ERROR_NON_FINITE,
        RAYTRACE_ERROR_F32_RANGE,
    )?;
    let widened: f64 = radius + half_ulp(probe) * 3.0_f64.sqrt();
    pack_upper_bound(widened)
}

/// Packs one occluder into its eight `vec4` record.
///
/// The record layout is frozen by the shader contract:
///
/// ```text
/// 0: center.xyz, shape
/// 1: extent.xyz, material_kind
/// 2: vertex0.xyz, original_primitive_index
/// 3: vertex1.xyz, 0
/// 4: vertex2.xyz, 0
/// 5: albedo.xyz, specular
/// 6: emissive.xyz, shininess
/// 7: 0, 0, 0, 0
/// ```
///
/// Spheres carry their radius in `extent.x` and are packed conservatively:
/// the centre rounds toward `-inf` on every axis and the radius is enlarged
/// by the same absolute error, so the packed sphere encloses the source
/// sphere after narrowing. AABBs are normalized so `center` is the minimum
/// corner and both bounds round outward. Triangles carry their three
/// vertices explicitly.
///
/// # Arguments
///
/// - `&mut Vec<f32>` - The destination packed buffer.
/// - `&Occluder` - The occluder to pack.
/// - `usize` - The original primitive index, stored as the tie breaker.
///
/// # Returns
///
/// - `Result<(), String>` - `Ok` once the record is appended, or the
///   rejection reason.
pub(crate) fn pack_primitive(
    out: &mut Vec<f32>,
    occluder: &Occluder,
    original_index: usize,
) -> Result<(), String> {
    let bound_index: f32 = pack_index(original_index)?;
    let kind: OccluderKind = occluder.get_kind();
    let (center, extent): (Vector3D, Vector3D) = match kind {
        OccluderKind::Aabb => {
            let lo: Vector3D = occluder.get_center();
            let hi: Vector3D = occluder.get_extent();
            (
                Vector3D::new(
                    lo.get_x().min(hi.get_x()),
                    lo.get_y().min(hi.get_y()),
                    lo.get_z().min(hi.get_z()),
                ),
                Vector3D::new(
                    lo.get_x().max(hi.get_x()),
                    lo.get_y().max(hi.get_y()),
                    lo.get_z().max(hi.get_z()),
                ),
            )
        }
        _ => (occluder.get_center(), occluder.get_extent()),
    };
    let material: &Material = occluder.get_material();
    let center_packed: [f32; 3] = match kind {
        OccluderKind::Sphere => {
            pack_vector_lower(center, RAYTRACE_ERROR_NON_FINITE, RAYTRACE_ERROR_F32_RANGE)?
        }
        _ => pack_vector(center, RAYTRACE_ERROR_NON_FINITE, RAYTRACE_ERROR_F32_RANGE)?,
    };
    let extent_packed: [f32; 3] = match kind {
        OccluderKind::Sphere => {
            let radius: f32 = pack_sphere_radius(center, extent.get_x())?;
            [radius, 0.0, 0.0]
        }
        OccluderKind::Aabb => {
            pack_vector_upper(extent, RAYTRACE_ERROR_NON_FINITE, RAYTRACE_ERROR_F32_RANGE)?
        }
        OccluderKind::Triangle => [0.0, 0.0, 0.0],
    };
    let albedo: [f32; 3] = pack_vector(
        material.get_albedo(),
        RAYTRACE_ERROR_NON_FINITE,
        RAYTRACE_ERROR_F32_RANGE,
    )?;
    let emissive: [f32; 3] = pack_vector(
        material.get_emissive(),
        RAYTRACE_ERROR_NON_FINITE,
        RAYTRACE_ERROR_F32_RANGE,
    )?;
    let specular: f32 = pack_finite(
        material.get_specular(),
        RAYTRACE_ERROR_NON_FINITE,
        RAYTRACE_ERROR_F32_RANGE,
    )?;
    let shininess: f32 = pack_finite(
        material.get_shininess(),
        RAYTRACE_ERROR_NON_FINITE,
        RAYTRACE_ERROR_F32_RANGE,
    )?;
    let vertices: [Vector3D; 3] = occluder.get_vertices();
    out.extend_from_slice(&[
        center_packed[0],
        center_packed[1],
        center_packed[2],
        packed_shape_code(kind),
    ]);
    out.extend_from_slice(&[
        extent_packed[0],
        extent_packed[1],
        extent_packed[2],
        packed_material_code(material.get_kind()),
    ]);
    let vertex0: [f32; 3] = pack_vector(
        vertices[0],
        RAYTRACE_ERROR_NON_FINITE,
        RAYTRACE_ERROR_F32_RANGE,
    )?;
    out.extend_from_slice(&[vertex0[0], vertex0[1], vertex0[2], bound_index]);
    let vertex1: [f32; 3] = pack_vector(
        vertices[1],
        RAYTRACE_ERROR_NON_FINITE,
        RAYTRACE_ERROR_F32_RANGE,
    )?;
    out.extend_from_slice(&[vertex1[0], vertex1[1], vertex1[2], 0.0]);
    let vertex2: [f32; 3] = pack_vector(
        vertices[2],
        RAYTRACE_ERROR_NON_FINITE,
        RAYTRACE_ERROR_F32_RANGE,
    )?;
    out.extend_from_slice(&[vertex2[0], vertex2[1], vertex2[2], 0.0]);
    out.extend_from_slice(&[albedo[0], albedo[1], albedo[2], specular]);
    out.extend_from_slice(&[emissive[0], emissive[1], emissive[2], shininess]);
    out.extend_from_slice(&[0.0, 0.0, 0.0, 0.0]);
    Ok(())
}

/// Packs one BVH node into its three `vec4` record.
///
/// The record layout is frozen by the shader contract:
///
/// ```text
/// 0: lower.xyz, start
/// 1: upper.xyz, count
/// 2: escape, 0, 0, 0
/// ```
///
/// `lower` rounds toward negative infinity and `upper` toward positive
/// infinity, so the packed node is always a conservative superset of the
/// CPU node and traversal cannot prune a subtree the CPU would have
/// visited. `start` indexes the reordered packed primitive records, and
/// `count` is zero for internal nodes, which both shaders detect to choose
/// between `start + 1` descent and an `escape` skip.
///
/// # Arguments
///
/// - `&mut Vec<f32>` - The destination, which keeps its capacity.
/// - `&RayTraceNode` - The node to pack.
///
/// # Returns
///
/// - `Result<(), String>` - `Ok` once three slots are appended, or the
///   first component that could not be represented.
pub(crate) fn pack_node(out: &mut Vec<f32>, node: &RayTraceNode) -> Result<(), String> {
    let lower: [f32; 3] = pack_vector_lower(
        node.get_lower(),
        RAYTRACE_ERROR_NON_FINITE,
        RAYTRACE_ERROR_F32_RANGE,
    )?;
    let upper: [f32; 3] = pack_vector_upper(
        node.get_upper(),
        RAYTRACE_ERROR_NON_FINITE,
        RAYTRACE_ERROR_F32_RANGE,
    )?;
    let start: f32 = pack_index(node.get_start())?;
    let count: f32 = pack_index(node.get_count())?;
    let escape: f32 = pack_index(node.get_escape())?;
    out.extend_from_slice(&[lower[0], lower[1], lower[2], start]);
    out.extend_from_slice(&[upper[0], upper[1], upper[2], count]);
    out.extend_from_slice(&[escape, 0.0, 0.0, 0.0]);
    Ok(())
}

/// Validates one frame's camera and returns its basis.
///
/// This is the single place a frame is accepted or rejected: the packed
/// header writer and the CPU pixel loop both go through it, so they can
/// never disagree about which cameras render and which do not. The checks
/// are the dimensions being non-zero and not overflowing the packed-buffer
/// limit, a field of view strictly inside `(0, PI)`, a depth range
/// satisfying `0 <= near < far` that survives the `f32` narrowing without
/// collapsing, a finite eye, and a non-degenerate basis.
///
/// Nothing here allocates: the result is a stack value, so a caller
/// rendering every frame pays no heap cost for validation, and the CPU
/// renderer needs no packed buffer at all.
///
/// # Arguments
///
/// - `&Camera3D` - The camera supplying the eye, target, up, fov, and
///   depth range.
/// - `u32` - The frame width in pixels.
/// - `u32` - The frame height in pixels.
///
/// # Returns
///
/// - `Result<FrameBasis, String>` - The validated basis, or the reason the
///   frame cannot be rendered.
pub(crate) fn frame_basis(
    camera: &Camera3D,
    width: u32,
    height: u32,
) -> Result<FrameBasis, String> {
    if width == 0 || height == 0 {
        return Err(RAYTRACE_ERROR_ZERO_DIMENSION.to_string());
    }
    let pixel_count: usize = match (width as usize).checked_mul(height as usize) {
        Some(product) => product,
        None => return Err(RAYTRACE_ERROR_DIMENSION_OVERFLOW.to_string()),
    };
    if pixel_count > RAYTRACE_MAX_PACKED_FLOATS {
        return Err(RAYTRACE_ERROR_DIMENSION_OVERFLOW.to_string());
    }
    let eye: Vector3D = camera.get_position();
    if !eye.get_x().is_finite() || !eye.get_y().is_finite() || !eye.get_z().is_finite() {
        return Err(RAYTRACE_ERROR_CAMERA_NON_FINITE.to_string());
    }
    let fov: f64 = camera.get_fov();
    let fov_usable: bool = fov.is_finite() && fov > 0.0 && fov < PI;
    if !fov_usable {
        return Err(RAYTRACE_ERROR_CAMERA_FOV.to_string());
    }
    let tan_half_fov: f64 = (fov * 0.5).tan();
    let tan_usable: bool = tan_half_fov.is_finite() && tan_half_fov > 0.0;
    if !tan_usable {
        return Err(RAYTRACE_ERROR_CAMERA_FOV.to_string());
    }
    let forward: Vector3D = camera.forward();
    if forward == Vector3D::zero() {
        return Err(RAYTRACE_ERROR_CAMERA_BASIS.to_string());
    }
    let right: Vector3D = forward.cross(camera.get_up()).normalized();
    let up: Vector3D = right.cross(forward);
    if right == Vector3D::zero() || up == Vector3D::zero() {
        return Err(RAYTRACE_ERROR_CAMERA_BASIS.to_string());
    }
    let near_t: f64 = camera.get_near();
    let far_t: f64 = camera.get_far();
    let range_ordered: bool =
        near_t.is_finite() && far_t.is_finite() && near_t >= 0.0 && far_t > near_t;
    if !range_ordered {
        return Err(RAYTRACE_ERROR_CAMERA_RANGE.to_string());
    }
    let near_packed: f32 = pack_finite(
        near_t,
        RAYTRACE_ERROR_CAMERA_NON_FINITE,
        RAYTRACE_ERROR_CAMERA_F32_RANGE,
    )?;
    let far_packed: f32 = pack_finite(
        far_t,
        RAYTRACE_ERROR_CAMERA_NON_FINITE,
        RAYTRACE_ERROR_CAMERA_F32_RANGE,
    )?;
    let packed_range_ordered: bool = far_packed > near_packed;
    if !packed_range_ordered {
        return Err(RAYTRACE_ERROR_CAMERA_RANGE.to_string());
    }
    Ok(FrameBasis {
        eye,
        forward,
        right,
        up,
        tan_half_fov,
        aspect: width as f64 / height as f64,
        width,
        height,
        near_t,
        far_t,
    })
}

/// Packs the per-frame camera, lighting, and settings block consumed by
/// both shader backends.
///
/// The buffer layout is frozen by the shader contract:
///
/// ```text
/// 0: eye.xyz, tan(fov / 2)
/// 1: forward.xyz, aspect
/// 2: right.xyz, width
/// 3: up.xyz, height
/// 4: ambient.xyz, light_count
/// 5: lighting_eye.xyz, max_bounces
/// 6: near_t, far_t, samples_per_axis, 0
/// 7: 0, 0, 0, 0
/// ```
///
/// followed by four `vec4` per light:
///
/// ```text
/// 0: position.xyz, kind
/// 1: direction.xyz, intensity
/// 2: color.xyz, falloff
/// 3: spot_cos, 0, 0, 0
/// ```
///
/// The camera basis is recomputed from `position`, `target`, and `up`, so a
/// caller never supplies a basis that disagrees with `target`. A degenerate
/// basis — a zero forward, a zero right, or an up parallel to forward — is
/// rejected rather than packed as a zero direction that would collapse every
/// ray onto one axis. The field of view must lie in `(0, PI)` and the depth
/// range must satisfy `0 <= near < far`; both are packed through
/// [`pack_finite`], so a `f64` far plane beyond the `f32` range is reported
/// rather than written as infinity.
///
/// The light count is taken from the input and never capped: a frame with
/// more lights than the packed-buffer limit allows is reported through the
/// overflow check, not silently truncated.
///
/// The supplied `out` is cleared and then reused, so its capacity is kept
/// and a caller packing every frame performs no allocation. Every value
/// the packer reads arrives through one [`RayTraceFrame`], the same
/// parameter type the CPU renderer takes, so the two backends cannot be
/// called with mismatched camera, lights, settings, or dimensions.
///
/// `out` is cleared only after the settings and the camera have been
/// validated, so the rejections a caller is most likely to hit — an
/// out-of-range sample grid, a zero or overflowing dimension, a
/// degenerate camera — leave the previous frame's contents intact. A
/// light whose components cannot be represented is a different case: that
/// check runs inside the write loop, after the clear, so such a frame
/// leaves a partially written buffer behind rather than the old one.
///
/// # Arguments
///
/// - `&mut Vec<f32>` - The destination packed buffer, cleared then reused.
/// - `&RayTraceFrame<'_>` - The camera, lights, settings, and dimensions
///   to pack.
///
/// # Returns
///
/// - `Result<(), String>` - `Ok` once the block is packed, or the reason
///   the frame cannot be represented.
pub fn pack_raytrace_frame(out: &mut Vec<f32>, frame: &RayTraceFrame<'_>) -> Result<(), String> {
    let camera: &Camera3D = frame.get_camera();
    let lights: &LightingUniforms = frame.get_lights();
    let settings: &RayTraceSettings = frame.get_settings();
    settings.validate()?;
    let basis: FrameBasis = frame_basis(camera, frame.get_width(), frame.get_height())?;
    let light_count: usize = lights.get_lights().len();
    let light_slots: usize = light_count
        .checked_mul(RAYTRACE_PACKED_LIGHT_VEC4S)
        .ok_or_else(|| RAYTRACE_ERROR_FRAME_OVERFLOW.to_string())?;
    let total_slots: usize = RAYTRACE_PACKED_FRAME_HEADER_VEC4S
        .checked_add(light_slots)
        .ok_or_else(|| RAYTRACE_ERROR_FRAME_OVERFLOW.to_string())?;
    let total_floats: usize = total_slots
        .checked_mul(4)
        .ok_or_else(|| RAYTRACE_ERROR_FRAME_OVERFLOW.to_string())?;
    if total_floats > RAYTRACE_MAX_PACKED_FLOATS {
        return Err(RAYTRACE_ERROR_FRAME_OVERFLOW.to_string());
    }
    let eye: [f32; 3] = pack_vector(
        camera.get_position(),
        RAYTRACE_ERROR_CAMERA_NON_FINITE,
        RAYTRACE_ERROR_CAMERA_NON_FINITE,
    )?;
    let forward_packed: [f32; 3] = pack_vector(
        basis.get_forward(),
        RAYTRACE_ERROR_CAMERA_NON_FINITE,
        RAYTRACE_ERROR_CAMERA_NON_FINITE,
    )?;
    let right_packed: [f32; 3] = pack_vector(
        basis.get_right(),
        RAYTRACE_ERROR_CAMERA_NON_FINITE,
        RAYTRACE_ERROR_CAMERA_NON_FINITE,
    )?;
    let up_packed: [f32; 3] = pack_vector(
        basis.get_up(),
        RAYTRACE_ERROR_CAMERA_NON_FINITE,
        RAYTRACE_ERROR_CAMERA_NON_FINITE,
    )?;
    let tan_half_fov: f32 = pack_finite(
        basis.get_tan_half_fov(),
        RAYTRACE_ERROR_CAMERA_NON_FINITE,
        RAYTRACE_ERROR_CAMERA_NON_FINITE,
    )?;
    let ambient: [f32; 3] = pack_vector(
        lights.get_ambient(),
        RAYTRACE_ERROR_LIGHT_NON_FINITE,
        RAYTRACE_ERROR_LIGHT_F32_RANGE,
    )?;
    let lighting_eye: [f32; 3] = pack_vector(
        lights.get_eye(),
        RAYTRACE_ERROR_LIGHT_NON_FINITE,
        RAYTRACE_ERROR_LIGHT_F32_RANGE,
    )?;
    out.clear();
    out.extend_from_slice(&[eye[0], eye[1], eye[2], tan_half_fov]);
    out.extend_from_slice(&[
        forward_packed[0],
        forward_packed[1],
        forward_packed[2],
        basis.get_aspect() as f32,
    ]);
    out.extend_from_slice(&[
        right_packed[0],
        right_packed[1],
        right_packed[2],
        basis.get_width() as f32,
    ]);
    out.extend_from_slice(&[
        up_packed[0],
        up_packed[1],
        up_packed[2],
        basis.get_height() as f32,
    ]);
    out.extend_from_slice(&[ambient[0], ambient[1], ambient[2], light_count as f32]);
    out.extend_from_slice(&[
        lighting_eye[0],
        lighting_eye[1],
        lighting_eye[2],
        settings.get_max_bounces() as f32,
    ]);
    out.extend_from_slice(&[
        basis.get_near_t() as f32,
        basis.get_far_t() as f32,
        settings.get_samples_per_axis() as f32,
        0.0,
    ]);
    out.extend_from_slice(&[0.0, 0.0, 0.0, 0.0]);
    for light in lights.get_lights().iter() {
        let position: [f32; 3] = pack_vector(
            light.get_position(),
            RAYTRACE_ERROR_LIGHT_NON_FINITE,
            RAYTRACE_ERROR_LIGHT_F32_RANGE,
        )?;
        let direction: [f32; 3] = pack_vector(
            light.get_direction(),
            RAYTRACE_ERROR_LIGHT_NON_FINITE,
            RAYTRACE_ERROR_LIGHT_F32_RANGE,
        )?;
        let color: [f32; 3] = pack_vector(
            light.get_color(),
            RAYTRACE_ERROR_LIGHT_NON_FINITE,
            RAYTRACE_ERROR_LIGHT_F32_RANGE,
        )?;
        let intensity: f32 = pack_finite(
            light.get_intensity(),
            RAYTRACE_ERROR_LIGHT_NON_FINITE,
            RAYTRACE_ERROR_LIGHT_F32_RANGE,
        )?;
        let falloff: f32 = pack_finite(
            light.get_falloff(),
            RAYTRACE_ERROR_LIGHT_NON_FINITE,
            RAYTRACE_ERROR_LIGHT_F32_RANGE,
        )?;
        let spot_cos: f32 = pack_finite(
            light.get_spot_cos(),
            RAYTRACE_ERROR_LIGHT_NON_FINITE,
            RAYTRACE_ERROR_LIGHT_F32_RANGE,
        )?;
        out.extend_from_slice(&[
            position[0],
            position[1],
            position[2],
            packed_light_code(light.get_kind()),
        ]);
        out.extend_from_slice(&[direction[0], direction[1], direction[2], intensity]);
        out.extend_from_slice(&[color[0], color[1], color[2], falloff]);
        out.extend_from_slice(&[spot_cos, 0.0, 0.0, 0.0]);
    }
    Ok(())
}
