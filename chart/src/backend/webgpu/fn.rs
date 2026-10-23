use super::*;

/// Builds a JavaScript object from a list of properties.
///
/// The properties are written with `Reflect::set`, which cannot fail on a
/// freshly created plain object, so the result is returned unconditionally.
///
/// # Arguments
///
/// - `&[(&str, &JsValue)]` - Shared reference to a `[(&str, &JsValue)]`.
///
/// # Returns
///
/// - `JsValue` - A `JsValue` value.
pub(crate) fn gpu_object(entries: &[(&str, &JsValue)]) -> JsValue {
    let object: Object = js_sys::Object::new();
    for (key, value) in entries {
        let _: Result<bool, JsValue> =
            js_sys::Reflect::set(object.as_ref(), &JsValue::from_str(key), value);
    }
    object.into()
}

/// Builds a JavaScript array from a list of values.
///
/// # Arguments
///
/// - `&[JsValue]` - Shared reference to a `[JsValue]`.
///
/// # Returns
///
/// - `JsValue` - A `JsValue` value.
pub(crate) fn gpu_array(values: &[JsValue]) -> JsValue {
    let array: Array = js_sys::Array::new();
    for value in values {
        array.push(value);
    }
    array.into()
}

/// Reads a property from a JavaScript object.
///
/// # Arguments
///
/// - `&JsValue` - Shared reference to a `JsValue`.
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn gpu_get(target: &JsValue, key: &str) -> Option<JsValue> {
    js_sys::Reflect::get(target, &JsValue::from_str(key)).ok()
}

/// Reads a method from a JavaScript object.
///
/// # Arguments
///
/// - `&JsValue` - Shared reference to a `JsValue`.
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Option<js_sys::Function>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn gpu_method(target: &JsValue, key: &str) -> Option<js_sys::Function> {
    gpu_get(target, key).and_then(|value: JsValue| value.dyn_into::<js_sys::Function>().ok())
}

/// Writes a property into a JavaScript object.
///
/// # Arguments
///
/// - `&JsValue` - Shared reference to a `JsValue`.
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `bool` - A boolean.
pub(crate) fn gpu_set(target: &JsValue, key: &str, value: &JsValue) -> bool {
    js_sys::Reflect::set(target, &JsValue::from_str(key), value).unwrap_or(false)
}

/// Returns the queue of a GPU device.
///
/// # Arguments
///
/// - `&JsValue` - Shared reference to a `JsValue`.
///
/// # Returns
///
/// - `Option<JsValue>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn gpu_queue(device: &JsValue) -> Option<JsValue> {
    gpu_get(device, "queue")
}

/// Mixes a path into the fingerprint.
///
/// # Arguments
///
/// - `&mut GpuHasher` - Mutable reference to a `GpuHasher` (mutated in place).
/// - `&Path` - Shared reference to a `Path`.
pub(crate) fn hash_path(hasher: &mut GpuHasher, path: &Path) {
    hasher.write_u32(path.commands().len() as u32);
    for command in path.commands() {
        match *command {
            PathCommand::MoveTo(x, y) => {
                hasher.write_u8(0);
                hasher.write_f32(x);
                hasher.write_f32(y);
            }
            PathCommand::LineTo(x, y) => {
                hasher.write_u8(1);
                hasher.write_f32(x);
                hasher.write_f32(y);
            }
            PathCommand::QuadTo(cx, cy, x, y) => {
                hasher.write_u8(2);
                hasher.write_f32(cx);
                hasher.write_f32(cy);
                hasher.write_f32(x);
                hasher.write_f32(y);
            }
            PathCommand::CubicTo(c1x, c1y, c2x, c2y, x, y) => {
                hasher.write_u8(3);
                hasher.write_f32(c1x);
                hasher.write_f32(c1y);
                hasher.write_f32(c2x);
                hasher.write_f32(c2y);
                hasher.write_f32(x);
                hasher.write_f32(y);
            }
            PathCommand::Close => {
                hasher.write_u8(4);
            }
        }
    }
}

/// Mixes a fill paint into the fingerprint.
///
/// # Arguments
///
/// - `&mut GpuHasher` - Mutable reference to a `GpuHasher` (mutated in place).
/// - `&Fill` - Shared reference to a `Fill`.
pub(crate) fn hash_fill(hasher: &mut GpuHasher, paint: &Fill) {
    match paint {
        Fill::Solid(color) => {
            hasher.write_u8(0);
            color.hash(hasher);
        }
        Fill::Linear(gradient) => {
            hasher.write_u8(1);
            hash_linear_gradient(hasher, gradient);
        }
        Fill::Radial(gradient) => {
            hasher.write_u8(2);
            hash_radial_gradient(hasher, gradient);
        }
        Fill::None => {
            hasher.write_u8(3);
        }
    }
}

/// Mixes a stroke paint into the fingerprint.
///
/// # Arguments
///
/// - `&mut GpuHasher` - Mutable reference to a `GpuHasher` (mutated in place).
/// - `&Stroke` - Shared reference to a `Stroke`.
pub(crate) fn hash_stroke(hasher: &mut GpuHasher, paint: &Stroke) {
    match paint {
        Stroke::Solid(color) => {
            hasher.write_u8(0);
            color.hash(hasher);
        }
        Stroke::Linear(gradient) => {
            hasher.write_u8(1);
            hash_linear_gradient(hasher, gradient);
        }
        Stroke::Radial(gradient) => {
            hasher.write_u8(2);
            hash_radial_gradient(hasher, gradient);
        }
        Stroke::None => {
            hasher.write_u8(3);
        }
    }
}

/// Mixes a linear gradient into the fingerprint.
///
/// # Arguments
///
/// - `&mut GpuHasher` - Mutable reference to a `GpuHasher` (mutated in place).
/// - `&LinearGradient` - Shared reference to a `LinearGradient`.
fn hash_linear_gradient(hasher: &mut GpuHasher, gradient: &LinearGradient) {
    hasher.write_f64(gradient.start.0);
    hasher.write_f64(gradient.start.1);
    hasher.write_f64(gradient.end.0);
    hasher.write_f64(gradient.end.1);
    hash_stops(hasher, &gradient.stops);
}

/// Mixes a radial gradient into the fingerprint.
///
/// # Arguments
///
/// - `&mut GpuHasher` - Mutable reference to a `GpuHasher` (mutated in place).
/// - `&RadialGradient` - Shared reference to a `RadialGradient`.
fn hash_radial_gradient(hasher: &mut GpuHasher, gradient: &RadialGradient) {
    hasher.write_f64(gradient.center.0);
    hasher.write_f64(gradient.center.1);
    hasher.write_f64(gradient.radius);
    hash_stops(hasher, &gradient.stops);
}

/// Mixes a list of gradient stops into the fingerprint.
///
/// # Arguments
///
/// - `&mut GpuHasher` - Mutable reference to a `GpuHasher` (mutated in place).
/// - `&[GradientStop]` - Shared reference to a `[GradientStop]`.
fn hash_stops(hasher: &mut GpuHasher, stops: &[GradientStop]) {
    hasher.write_u32(stops.len() as u32);
    for stop in stops {
        hasher.write_f64(stop.offset);
        stop.color.hash(hasher);
    }
}

/// Mixes a line style into the fingerprint.
///
/// # Arguments
///
/// - `&mut GpuHasher` - Mutable reference to a `GpuHasher` (mutated in place).
/// - `&LineStyle` - Shared reference to a `LineStyle`.
pub(crate) fn hash_line_style(hasher: &mut GpuHasher, style: &LineStyle) {
    hasher.write_f64(style.width);
    style.cap.hash(hasher);
    style.join.hash(hasher);
    hasher.write_u32(style.dash.pattern.len() as u32);
    for length in &style.dash.pattern {
        hasher.write_f32(*length);
    }
    hasher.write_f32(style.dash.offset);
    hasher.write_f64(style.opacity);
    style.color.hash(hasher);
    match &style.shadow {
        Some(shadow) => {
            hasher.write_u8(1);
            hasher.write_f64(shadow.offset_x);
            hasher.write_f64(shadow.offset_y);
            hasher.write_f64(shadow.blur);
            shadow.color.hash(hasher);
        }
        None => {
            hasher.write_u8(0);
        }
    }
}

/// Mixes a text style into the fingerprint.
///
/// The font is hashed through its parts rather than through
/// [`TextStyle::font`], because the composed shorthand would allocate a
/// string for every label of every frame.
///
/// # Arguments
///
/// - `&mut GpuHasher` - Mutable reference to a `GpuHasher` (mutated in place).
/// - `&TextStyle` - Shared reference to a `TextStyle`.
pub(crate) fn hash_text_style(hasher: &mut GpuHasher, style: &TextStyle) {
    style.family.hash(hasher);
    hasher.write_f64(style.size);
    style.weight.hash(hasher);
    style.style.hash(hasher);
    style.align.hash(hasher);
    style.vertical_align.hash(hasher);
    hasher.write_f64(style.line_height);
    style.color.hash(hasher);
    style.background.hash(hasher);
    let (top, right, bottom, left) = style.padding;
    hasher.write_f64(top);
    hasher.write_f64(right);
    hasher.write_f64(bottom);
    hasher.write_f64(left);
    hasher.write_f64(style.border_width);
    style.border_color.hash(hasher);
    hasher.write_f64(style.border_radius);
    hasher.write_f64(style.rotate);
    hasher.write_f64(style.stroke_width);
    style.stroke_color.hash(hasher);
    hasher.write_u32(style.rich.len() as u32);
}

/// Returns the colour of a linear gradient at the given point.
///
/// The gradient is a linear function of the position, so evaluating it at
/// the vertices of a triangle and letting the rasteriser interpolate
/// between them is exact.
///
/// # Arguments
///
/// - `&LinearGradient` - Shared reference to a `LinearGradient`.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `Color` - A `Color` value.
pub(crate) fn linear_gradient_color(gradient: &LinearGradient, x: f64, y: f64) -> Color {
    let delta_x: f64 = gradient.end.0 - gradient.start.0;
    let delta_y: f64 = gradient.end.1 - gradient.start.1;
    let length: f64 = delta_x * delta_x + delta_y * delta_y;
    let position: f64 = if length <= f64::EPSILON {
        0.0
    } else {
        ((x - gradient.start.0) * delta_x + (y - gradient.start.1) * delta_y) / length
    };
    sample_gradient(&gradient.stops, position.clamp(0.0, 1.0))
}

/// Returns the colour of a radial gradient at the given point.
///
/// A radial gradient is not a linear function of the position, so the value
/// is sampled at the vertices and interpolated across the triangle. The
/// tessellator splits curves into short segments, so the error stays
/// small, but a large triangle over a radial gradient shows a slight
/// banding.
///
/// # Arguments
///
/// - `&RadialGradient` - Shared reference to a `RadialGradient`.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `Color` - A `Color` value.
pub(crate) fn radial_gradient_color(gradient: &RadialGradient, x: f64, y: f64) -> Color {
    let delta_x: f64 = x - gradient.center.0;
    let delta_y: f64 = y - gradient.center.1;
    let radius: f64 = gradient.radius.max(f64::EPSILON);
    let position: f64 = (delta_x * delta_x + delta_y * delta_y).sqrt() / radius;
    sample_gradient(&gradient.stops, position.clamp(0.0, 1.0))
}

/// Samples a list of gradient stops at the given position.
///
/// # Arguments
///
/// - `&[GradientStop]` - Shared reference to a `[GradientStop]`.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `Color` - A `Color` value.
pub(crate) fn sample_gradient(stops: &[GradientStop], position: f64) -> Color {
    let Some(first) = stops.first() else {
        return Color::default();
    };
    if position <= first.offset {
        return first.color;
    }
    let Some(last) = stops.last() else {
        return Color::default();
    };
    if position >= last.offset {
        return last.color;
    }
    for window in stops.windows(2) {
        let start: GradientStop = window[0];
        let end: GradientStop = window[1];
        if position >= start.offset && position <= end.offset {
            let span: f64 = end.offset - start.offset;
            let local: f64 = if span <= f64::EPSILON {
                0.0
            } else {
                (position - start.offset) / span
            };
            return Color {
                red: lerp_channel(start.color.red, end.color.red, local),
                green: lerp_channel(start.color.green, end.color.green, local),
                blue: lerp_channel(start.color.blue, end.color.blue, local),
                alpha: lerp_channel(start.color.alpha, end.color.alpha, local),
            };
        }
    }
    last.color
}

/// Interpolates one colour channel.
///
/// # Arguments
///
/// - `u8` - A `u8` parameter.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `u8` - A `u8` value.
fn lerp_channel(start: u8, end: u8, amount: f64) -> u8 {
    let value: f64 = f64::from(start) + (f64::from(end) - f64::from(start)) * amount;
    value.round().clamp(0.0, 255.0) as u8
}

/// Appends one flat-coloured vertex to a vertex buffer.
///
/// The colour is written as four bytes in red, green, blue, alpha order so
/// that the shader can read it as an `unorm8x4` attribute without having to
/// swizzle the channels.
///
/// # Arguments
///
/// - `&mut Vec<u8>` - Mutable reference to a `Vec<u8>` (mutated in place).
/// - `f32` - A 32-bit float (`f32`).
/// - `Color` - A `Color` parameter.
/// - `u8` - A `u8` parameter.
pub(crate) fn push_solid_vertex(out: &mut Vec<u8>, x: f32, y: f32, color: Color, alpha: u8) {
    out.extend_from_slice(&x.to_le_bytes());
    out.extend_from_slice(&y.to_le_bytes());
    out.extend_from_slice(&color.red.to_le_bytes());
    out.extend_from_slice(&color.green.to_le_bytes());
    out.extend_from_slice(&color.blue.to_le_bytes());
    out.extend_from_slice(&alpha.to_le_bytes());
}

/// Appends one textured vertex to a vertex buffer.
///
/// # Arguments
///
/// - `&mut Vec<u8>` - Mutable reference to a `Vec<u8>` (mutated in place).
/// - `f32` - A 32-bit float (`f32`).
/// - `Color` - A `Color` parameter.
/// - `u8` - A `u8` parameter.
pub(crate) fn push_textured_vertex(
    out: &mut Vec<u8>,
    x: f32,
    y: f32,
    u: f32,
    v: f32,
    color: Color,
    alpha: u8,
) {
    out.extend_from_slice(&x.to_le_bytes());
    out.extend_from_slice(&y.to_le_bytes());
    out.extend_from_slice(&u.to_le_bytes());
    out.extend_from_slice(&v.to_le_bytes());
    out.extend_from_slice(&color.red.to_le_bytes());
    out.extend_from_slice(&color.green.to_le_bytes());
    out.extend_from_slice(&color.blue.to_le_bytes());
    out.extend_from_slice(&alpha.to_le_bytes());
}

/// Returns the alpha of a colour after the overall opacity is applied.
///
/// # Arguments
///
/// - `Color` - A `Color` parameter.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `u8` - A `u8` value.
pub(crate) fn scaled_alpha(color: Color, opacity: f64) -> u8 {
    let alpha: f64 = f64::from(color.alpha) * opacity.clamp(0.0, 1.0);
    alpha.round().clamp(0.0, 255.0) as u8
}
