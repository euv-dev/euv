use super::*;

/// Builds the world-space direction for one supersample of one pixel.
///
/// The camera ray is
/// `normalize(forward + right * (ndc_x * aspect * tan(fov/2)) + up *
/// (ndc_y * tan(fov/2)))`. The `tan(fov/2)` factor is what makes the
/// rendered vertical field of view exactly `fov`; omitting it renders
/// wider than the camera asks for and no longer matches either shader,
/// which both apply it.
///
/// Rows are addressed top-down, matching the WGSL fragment position and the
/// `gl_FragCoord`-flipped WebGL2 ordering, so all three backends agree row
/// for row.
///
/// # Arguments
///
/// - `&FrameBasis` - The validated camera basis.
/// - `&PixelSample` - The pixel coordinates and the sub-sample's offset
///   within that pixel.
///
/// # Returns
///
/// - `Vector3D` - The unit world-space ray direction.
pub(crate) fn pixel_direction(basis: &FrameBasis, sample: &PixelSample) -> Vector3D {
    let ndc_x: f64 =
        ((sample.get_x() + sample.get_offset_x()) / basis.get_width() as f64) * 2.0 - 1.0;
    let ndc_y: f64 =
        1.0 - ((sample.get_y() + sample.get_offset_y()) / basis.get_height() as f64) * 2.0;
    let tan_half: f64 = basis.get_tan_half_fov();
    (basis.get_forward()
        + basis
            .get_right()
            .scaled(ndc_x * basis.get_aspect() * tan_half)
        + basis.get_up().scaled(ndc_y * tan_half))
    .normalized()
}

/// Converts one linear channel to a gamma-corrected byte.
///
/// The clamp to `[0, 1]` happens before the curve and the result is
/// rounded, matching both shaders' `pow(clamp(c, 0, 1), 1/2.2)`.
///
/// # Arguments
///
/// - `f64` - The linear channel, which may fall outside `[0, 1]`.
///
/// # Returns
///
/// - `u8` - The gamma-corrected byte, clamped to `[0, 255]`.
pub fn gamma_byte(channel: f64) -> u8 {
    let clamped: f64 = channel.clamp(0.0, 1.0);
    let encoded: f64 = clamped.powf(1.0 / RAYTRACE_GAMMA);
    (encoded * 255.0).round().clamp(0.0, 255.0) as u8
}

/// Renders a ray-traced RGBA frame on the CPU.
///
/// This is the CPU reference and the Canvas 2D backend's renderer. It does
/// not re-implement traversal or shading: each pixel builds a [`Ray`] — a
/// stack value, so no allocation happens per pixel, per bounce, or per
/// frame — and hands it to [`RayTraceScene::trace_with_bounces`], which
/// runs the scene's own bounding volume hierarchy and the real
/// [`LightingUniforms`] shading. The two GPU backends implement the same
/// math in shader form against the packed buffers; they are the only
/// duplicates, and they must agree with this path rather than with each
/// other.
///
/// The frame is described by a [`FrameBasis`], a stack value built by the
/// same validator [`pack_raytrace_frame`] uses, so this path never packs a
/// GPU buffer, never re-encodes the scene, and allocates nothing at all
/// beyond the caller-owned `out`. A steady-state frame therefore performs
/// exactly one heap operation: none.
///
/// Each pixel traces `samples_per_axis` squared supersampled rays, averages
/// them, clamps the linear color to `[0, 1]`, applies the `1 / 2.2` gamma
/// curve, and writes an opaque alpha. Rows are emitted top-down.
///
/// Frame inputs arrive through the same [`RayTraceFrame`] parameter type
/// accepted by [`pack_raytrace_frame`]. The scene and mutable output remain
/// separate, so the descriptor itself contains only read-only frame state.
///
/// # Arguments
///
/// - `&RayTraceScene` - The scene to render.
/// - `&RayTraceFrame<'_>` - The camera, lights, settings, and dimensions
///   to render.
/// - `&mut Vec<u8>` - The destination RGBA buffer, cleared and reused.
///
/// # Returns
///
/// - `Result<(), String>` - `Ok` once the frame is written, or the reason
///   the frame cannot be rendered.
pub fn render_raytrace_rgba(
    scene: &RayTraceScene,
    frame: &RayTraceFrame<'_>,
    out: &mut Vec<u8>,
) -> Result<(), String> {
    let lights: &LightingUniforms = frame.get_lights();
    frame.get_settings().validate()?;
    let basis: FrameBasis = frame_basis(frame.get_camera(), frame.get_width(), frame.get_height())?;
    let pixel_count: usize = (basis.get_width() as usize) * (basis.get_height() as usize);
    let byte_count: usize = match pixel_count.checked_mul(4) {
        Some(count) => count,
        None => return Err(RAYTRACE_ERROR_DIMENSION_OVERFLOW.to_string()),
    };
    let samples: u32 = frame.get_settings().get_samples_per_axis();
    let sample_scale: f64 = 1.0 / samples as f64;
    let rays: f64 = (samples * samples) as f64;
    let bounces: u32 = frame.get_settings().get_max_bounces();
    out.clear();
    out.resize(byte_count, 0);
    for y in 0..basis.get_height() {
        for x in 0..basis.get_width() {
            let mut accumulator: Vector3D = Vector3D::zero();
            for sy in 0..samples {
                for sx in 0..samples {
                    let sample: PixelSample = PixelSample {
                        x: x as f64,
                        y: y as f64,
                        offset_x: (sx as f64 + 0.5) * sample_scale,
                        offset_y: (sy as f64 + 0.5) * sample_scale,
                    };
                    let direction: Vector3D = pixel_direction(&basis, &sample);
                    let ray: Ray = Ray::new(basis.get_eye(), direction)
                        .with_range(basis.get_near_t(), basis.get_far_t());
                    accumulator += scene.trace_with_bounces(ray, lights, bounces);
                }
            }
            let linear: Vector3D = accumulator.scaled(1.0 / rays);
            let offset: usize = ((y as usize) * (basis.get_width() as usize) + x as usize) * 4;
            out[offset] = gamma_byte(linear.get_x());
            out[offset + 1] = gamma_byte(linear.get_y());
            out[offset + 2] = gamma_byte(linear.get_z());
            out[offset + 3] = 255;
        }
    }
    Ok(())
}
