use super::*;

/// Returns `true` when the host can give out a WebGL2 context.
///
/// The check opens a throw-away canvas, so it costs one context creation
/// and is only ever run once per chart. It is the probe the backend
/// itself uses, so the associated function and the module-level one can
/// never disagree.
///
/// # Returns
///
/// - `bool` - A boolean.
pub(crate) fn is_supported() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Some(document) = window.document() else {
        return false;
    };
    let Ok(element) = document.create_element("canvas") else {
        return false;
    };
    let Ok(canvas) = element.dyn_into::<HtmlCanvasElement>() else {
        return false;
    };
    canvas
        .get_context("webgl2")
        .ok()
        .flatten()
        .map(|context: Object| !context.is_null())
        .unwrap_or(false)
}

/// Returns the fingerprint of a display list.
///
/// The fingerprint covers everything that can change the picture: the
/// number of items, the paint order, the paths, the paints, the text and
/// its style, and the positions of the images. Two lists with the same
/// fingerprint draw exactly the same frame, so the backend can keep the
/// GPU buffers it already has.
///
/// # Arguments
///
/// - `&DisplayList` - Shared reference to a `DisplayList`.
///
/// # Returns
///
/// - `u64` - A `u64` value.
pub(crate) fn fingerprint(list: &DisplayList) -> u64 {
    let mut hasher: DisplayHasher = DisplayHasher::new();
    hasher.write_u64(list.len() as u64);
    for item in list.items() {
        hasher.write_u32(item.z as u32);
        match &item.cmd {
            DrawCmd::Fill {
                path,
                paint,
                opacity,
            } => {
                hasher.write_u32(0);
                hash_path(&mut hasher, path);
                hash_fill(&mut hasher, paint);
                hasher.write_f64(*opacity);
            }
            DrawCmd::Stroke {
                path,
                paint,
                style,
                opacity,
            } => {
                hasher.write_u32(1);
                hash_path(&mut hasher, path);
                hash_stroke(&mut hasher, paint);
                hash_line_style(&mut hasher, style);
                hasher.write_f64(*opacity);
            }
            DrawCmd::Strip {
                points,
                baseline,
                paint,
                opacity,
            } => {
                hasher.write_u32(2);
                hasher.write_u64(points.len() as u64);
                for point in points {
                    hasher.write_f32(point.x);
                    hasher.write_f32(point.y);
                }
                hasher.write_f32(*baseline);
                hash_fill(&mut hasher, paint);
                hasher.write_f64(*opacity);
            }
            DrawCmd::Text { text, x, y, style } => {
                hasher.write_u32(3);
                hasher.write_str(text);
                hasher.write_f64(*x);
                hasher.write_f64(*y);
                hash_text_style(&mut hasher, style);
            }
            DrawCmd::Image {
                image,
                x,
                y,
                width,
                height,
            } => {
                hasher.write_u32(4);
                hasher.write_str(&image.src());
                // An image that has not finished loading is uploaded as
                // an empty texture, so the moment it becomes complete
                // has to invalidate the cache.
                hasher.write_u32(u32::from(image.complete()));
                hasher.write_f64(*x);
                hasher.write_f64(*y);
                hasher.write_f64(*width);
                hasher.write_f64(*height);
            }
            DrawCmd::Clip { path } => {
                hasher.write_u32(5);
                hash_path(&mut hasher, path);
            }
            DrawCmd::PopClip => {
                hasher.write_u32(6);
            }
        }
    }
    hasher.finish()
}

/// Mixes a path into the hash.
///
/// # Arguments
///
/// - `&mut DisplayHasher` - Mutable reference to a `DisplayHasher` (mutated in place).
/// - `&Path` - Shared reference to a `Path`.
fn hash_path(hasher: &mut DisplayHasher, path: &Path) {
    let commands: &[PathCommand] = path.commands();
    hasher.write_u64(commands.len() as u64);
    for command in commands {
        match *command {
            PathCommand::MoveTo(x, y) => {
                hasher.write_u32(0);
                hasher.write_f32(x);
                hasher.write_f32(y);
            }
            PathCommand::LineTo(x, y) => {
                hasher.write_u32(1);
                hasher.write_f32(x);
                hasher.write_f32(y);
            }
            PathCommand::QuadTo(cx, cy, x, y) => {
                hasher.write_u32(2);
                hasher.write_f32(cx);
                hasher.write_f32(cy);
                hasher.write_f32(x);
                hasher.write_f32(y);
            }
            PathCommand::CubicTo(c1x, c1y, c2x, c2y, x, y) => {
                hasher.write_u32(3);
                hasher.write_f32(c1x);
                hasher.write_f32(c1y);
                hasher.write_f32(c2x);
                hasher.write_f32(c2y);
                hasher.write_f32(x);
                hasher.write_f32(y);
            }
            PathCommand::Close => {
                hasher.write_u32(4);
            }
        }
    }
}

/// Mixes a fill paint into the hash.
///
/// # Arguments
///
/// - `&mut DisplayHasher` - Mutable reference to a `DisplayHasher` (mutated in place).
/// - `&Fill` - Shared reference to a `Fill`.
fn hash_fill(hasher: &mut DisplayHasher, paint: &Fill) {
    match paint {
        Fill::None => {
            hasher.write_u32(0);
        }
        Fill::Solid(color) => {
            hasher.write_u32(1);
            hasher.write_u32(color.to_u32());
        }
        Fill::Linear(gradient) => {
            hasher.write_u32(2);
            hash_linear_gradient(hasher, gradient);
        }
        Fill::Radial(gradient) => {
            hasher.write_u32(3);
            hash_radial_gradient(hasher, gradient);
        }
    }
}

/// Mixes a stroke paint into the hash.
///
/// # Arguments
///
/// - `&mut DisplayHasher` - Mutable reference to a `DisplayHasher` (mutated in place).
/// - `&Stroke` - Shared reference to a `Stroke`.
fn hash_stroke(hasher: &mut DisplayHasher, paint: &Stroke) {
    match paint {
        Stroke::None => {
            hasher.write_u32(0);
        }
        Stroke::Solid(color) => {
            hasher.write_u32(1);
            hasher.write_u32(color.to_u32());
        }
        Stroke::Linear(gradient) => {
            hasher.write_u32(2);
            hash_linear_gradient(hasher, gradient);
        }
        Stroke::Radial(gradient) => {
            hasher.write_u32(3);
            hash_radial_gradient(hasher, gradient);
        }
    }
}

/// Mixes a linear gradient into the hash.
///
/// # Arguments
///
/// - `&mut DisplayHasher` - Mutable reference to a `DisplayHasher` (mutated in place).
/// - `&LinearGradient` - Shared reference to a `LinearGradient`.
fn hash_linear_gradient(hasher: &mut DisplayHasher, gradient: &LinearGradient) {
    hasher.write_f64(gradient.start.0);
    hasher.write_f64(gradient.start.1);
    hasher.write_f64(gradient.end.0);
    hasher.write_f64(gradient.end.1);
    hash_stops(hasher, &gradient.stops);
}

/// Mixes a radial gradient into the hash.
///
/// # Arguments
///
/// - `&mut DisplayHasher` - Mutable reference to a `DisplayHasher` (mutated in place).
/// - `&RadialGradient` - Shared reference to a `RadialGradient`.
fn hash_radial_gradient(hasher: &mut DisplayHasher, gradient: &RadialGradient) {
    hasher.write_f64(gradient.center.0);
    hasher.write_f64(gradient.center.1);
    hasher.write_f64(gradient.radius);
    hash_stops(hasher, &gradient.stops);
}

/// Mixes the colour stops of a gradient into the hash.
///
/// # Arguments
///
/// - `&mut DisplayHasher` - Mutable reference to a `DisplayHasher` (mutated in place).
/// - `&[GradientStop]` - Shared reference to a `[GradientStop]`.
fn hash_stops(hasher: &mut DisplayHasher, stops: &[GradientStop]) {
    hasher.write_u64(stops.len() as u64);
    for stop in stops {
        hasher.write_f64(stop.offset);
        hasher.write_u32(stop.color.to_u32());
    }
}

/// Mixes the geometry of a stroke into the hash.
///
/// # Arguments
///
/// - `&mut DisplayHasher` - Mutable reference to a `DisplayHasher` (mutated in place).
/// - `&LineStyle` - Shared reference to a `LineStyle`.
fn hash_line_style(hasher: &mut DisplayHasher, style: &LineStyle) {
    hasher.write_f64(style.width);
    hasher.write_u32(style.cap as u32);
    hasher.write_u32(style.join as u32);
    hasher.write_f64(style.opacity);
    hasher.write_u64(style.dash.pattern.len() as u64);
    for length in &style.dash.pattern {
        hasher.write_f32(*length);
    }
    hasher.write_f32(style.dash.offset);
}

/// Mixes the parts of a text style that change the picture into the hash.
///
/// The style carries a few fields the chart backend does not draw — the
/// border, the rotation, and the text outline — but they are mixed in
/// all the same, so that a style change always invalidates the cache.
///
/// # Arguments
///
/// - `&mut DisplayHasher` - Mutable reference to a `DisplayHasher` (mutated in place).
/// - `&TextStyle` - Shared reference to a `TextStyle`.
fn hash_text_style(hasher: &mut DisplayHasher, style: &TextStyle) {
    hasher.write_str(&style.family);
    hasher.write_f64(style.size);
    hasher.write_str(&style.weight);
    hasher.write_str(&style.style);
    hasher.write_u32(style.align as u32);
    hasher.write_u32(style.vertical_align as u32);
    hasher.write_f64(style.line_height);
    hasher.write_u32(style.color.to_u32());
    match &style.background {
        Some(color) => {
            hasher.write_u32(1);
            hasher.write_u32(color.to_u32());
        }
        None => {
            hasher.write_u32(0);
        }
    }
    hasher.write_f64(style.padding.0);
    hasher.write_f64(style.padding.1);
    hasher.write_f64(style.padding.2);
    hasher.write_f64(style.padding.3);
}

/// Returns the colour ramp of a gradient as RGBA texels.
///
/// The ramp is a row of [`GL_RAMP_WIDTH`] texels that the shader samples
/// with linear filtering, so the stops are interpolated on the CPU and
/// the gradient comes out smooth.
///
/// # Arguments
///
/// - `&WebGl2Gradient` - Shared reference to a `WebGl2Gradient`.
///
/// # Returns
///
/// - `Vec<u8>` - A `Vec<u8>` value.
pub(crate) fn ramp_bytes(gradient: &WebGl2Gradient) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![0_u8; GL_RAMP_WIDTH * 4];
    let stops: &[GradientStop] = match gradient {
        WebGl2Gradient::Linear(linear) => &linear.stops,
        WebGl2Gradient::Radial(radial) => &radial.stops,
    };
    if stops.is_empty() {
        return bytes;
    }
    for index in 0..GL_RAMP_WIDTH {
        let position: f64 = index as f64 / (GL_RAMP_WIDTH - 1) as f64;
        let color: Color = sample_stops(stops, position);
        let offset: usize = index * 4;
        bytes[offset] = color.red;
        bytes[offset + 1] = color.green;
        bytes[offset + 2] = color.blue;
        bytes[offset + 3] = color.alpha;
    }
    bytes
}

/// Returns the geometry of a gradient as the two texels the shader
/// reads.
///
/// The first texel holds the axis of a linear gradient — its start and
/// its end — or the centre and the radius of a radial one. The second
/// texel holds the kind, which tells the shader how to read the first.
///
/// # Arguments
///
/// - `&WebGl2Gradient` - Shared reference to a `WebGl2Gradient`.
///
/// # Returns
///
/// - `[f32; 8]` - A `[f32` value.
pub(crate) fn parameter_values(gradient: &WebGl2Gradient) -> [f32; 8] {
    match gradient {
        WebGl2Gradient::Linear(linear) => [
            linear.start.0 as f32,
            linear.start.1 as f32,
            linear.end.0 as f32,
            linear.end.1 as f32,
            0.0,
            0.0,
            0.0,
            0.0,
        ],
        WebGl2Gradient::Radial(radial) => [
            radial.center.0 as f32,
            radial.center.1 as f32,
            radial.radius as f32,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
        ],
    }
}

/// Returns the colour of a gradient at the given position.
///
/// The stops are assumed to be ordered by offset, which is what the
/// gradient builders guarantee. A position before the first stop or
/// after the last one takes the colour of that stop, which is what a
/// canvas gradient does as well.
///
/// # Arguments
///
/// - `&[GradientStop]` - Shared reference to a `[GradientStop]`.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `Color` - A `Color` value.
fn sample_stops(stops: &[GradientStop], position: f64) -> Color {
    let first: GradientStop = stops[0];
    if position <= first.offset {
        return first.color;
    }
    let last: GradientStop = stops[stops.len() - 1];
    if position >= last.offset {
        return last.color;
    }
    for window in stops.windows(2) {
        let start: GradientStop = window[0];
        let end: GradientStop = window[1];
        if position >= start.offset && position <= end.offset {
            let span: f64 = end.offset - start.offset;
            let ratio: f64 = if span.abs() < f64::EPSILON {
                0.0
            } else {
                (position - start.offset) / span
            };
            return start.color.mix(end.color, ratio);
        }
    }
    last.color
}

/// Splits the contours into the runs a dash pattern describes.
///
/// The tessellator strokes a path as one piece of geometry, so a dashed
/// stroke has to be broken up before it reaches it. The function walks
/// the flattened contours, measures the distance along them, and cuts
/// them where the pattern switches between on and off. Every run comes
/// back as an open contour, which means the stroke's caps are applied at
/// the ends of each dash — the same thing a canvas does.
///
/// # Arguments
///
/// - `&[Contour]` - Shared reference to a `[Contour]`.
/// - `&[f32]` - Shared reference to a `[f32]`.
/// - `f32` - A 32-bit float (`f32`).
///
/// # Returns
///
/// - `Vec<Contour>` - A `Vec<Contour>` value.
pub(crate) fn dash_contours(contours: &[Contour], pattern: &[f32], offset: f32) -> Vec<Contour> {
    if pattern.is_empty() {
        return contours.to_vec();
    }

    // An odd pattern describes an incomplete on/off alternation, so it is
    // repeated once to make the alternation well defined.
    let mut lengths: Vec<f32> = pattern.to_vec();
    if lengths.len() % 2 == 1 {
        let copy: Vec<f32> = lengths.clone();
        lengths.extend(copy);
    }
    if lengths.iter().all(|length: &f32| *length <= 0.0) {
        return contours.to_vec();
    }

    let mut dashed: Vec<Contour> = Vec::new();
    for contour in contours {
        if contour.points.len() < 2 {
            continue;
        }
        // A closed contour is walked as a loop, so that the dash pattern
        // continues across the closing edge.
        let mut points: Vec<Point> = contour.points.clone();
        if contour.closed {
            points.push(contour.points[0]);
        }

        let mut index: usize = 0_usize;
        let mut phase: f32 = 0.0_f32;
        let mut on: bool = true;

        // The offset shifts the pattern along the path.
        let mut skip: f32 = offset.max(0.0);
        while skip > 0.0 {
            let length: f32 = lengths[index];
            if skip < length {
                phase = skip;
                break;
            }
            skip -= length;
            index = (index + 1) % lengths.len();
            on = !on;
        }

        let mut run: Vec<Point> = Vec::new();
        if on {
            run.push(points[0]);
        }

        for window in points.windows(2) {
            let start: Point = window[0];
            let end: Point = window[1];
            let length: f32 = distance(start, end);
            if length <= 1e-6 {
                continue;
            }
            let mut travelled: f32 = 0.0_f32;
            while travelled < length {
                let available: f32 = (lengths[index] - phase).max(1e-6);
                let step: f32 = available.min(length - travelled);
                let ratio: f32 = (travelled + step) / length;
                let point: Point = Point::new(
                    start.x + (end.x - start.x) * ratio,
                    start.y + (end.y - start.y) * ratio,
                );
                if on {
                    run.push(point);
                }
                travelled += step;
                phase += step;
                if phase >= lengths[index] - 1e-6 {
                    phase = 0.0;
                    index = (index + 1) % lengths.len();
                    if on {
                        if run.len() >= 2 {
                            dashed.push(Contour {
                                points: take(&mut run),
                                closed: false,
                            });
                        } else {
                            run.clear();
                        }
                    }
                    on = !on;
                    if on {
                        run.push(point);
                    }
                }
            }
        }

        if on && run.len() >= 2 {
            dashed.push(Contour {
                points: run,
                closed: false,
            });
        }
    }
    dashed
}

/// Returns the distance between two points.
///
/// # Arguments
///
/// - `Point` - A `Point` parameter.
///
/// # Returns
///
/// - `f32` - A 32-bit float.
fn distance(a: Point, b: Point) -> f32 {
    let dx: f32 = b.x - a.x;
    let dy: f32 = b.y - a.y;
    (dx * dx + dy * dy).sqrt()
}

/// Returns a colour as the four normalised channels the shader expects.
///
/// # Arguments
///
/// - `u32` - A 32-bit unsigned integer (`u32`).
///
/// # Returns
///
/// - `[f32; 4]` - A `[f32` value.
pub(crate) fn unpack_color(packed: u32) -> [f32; 4] {
    [
        ((packed >> 24) & 0xFF) as f32 / 255.0,
        ((packed >> 16) & 0xFF) as f32 / 255.0,
        ((packed >> 8) & 0xFF) as f32 / 255.0,
        (packed & 0xFF) as f32 / 255.0,
    ]
}

/// Returns the scissor rectangle that bounds a box.
///
/// WebGL2 cannot clip against a path, so a clip path is approximated by
/// the rectangle around it. The rectangle is kept in the chart's own
/// coordinate system, with the origin at the top left.
///
/// # Arguments
///
/// - `&BBox` - Shared reference to a `BBox`.
///
/// # Returns
///
/// - `WebGl2Scissor` - A `WebGl2Scissor` value.
pub(crate) fn scissor_of(bounds: &BBox) -> WebGl2Scissor {
    WebGl2Scissor {
        x: bounds.min_x,
        y: bounds.min_y,
        width: bounds.max_x - bounds.min_x,
        height: bounds.max_y - bounds.min_y,
    }
}

/// Returns the intersection of the current clip rectangle and another.
///
/// The clips of a display list nest, so every clip path narrows the
/// rectangle that is already in force.
///
/// # Arguments
///
/// - `Option<WebGl2Scissor>` - A `Option<WebGl2Scissor>` parameter.
/// - `WebGl2Scissor` - A `WebGl2Scissor` parameter.
///
/// # Returns
///
/// - `WebGl2Scissor` - A `WebGl2Scissor` value.
pub(crate) fn intersect_scissor(
    current: Option<WebGl2Scissor>,
    rectangle: WebGl2Scissor,
) -> WebGl2Scissor {
    let Some(current) = current else {
        return rectangle;
    };
    let left: f64 = current.x.max(rectangle.x);
    let top: f64 = current.y.max(rectangle.y);
    let right: f64 = (current.x + current.width).min(rectangle.x + rectangle.width);
    let bottom: f64 = (current.y + current.height).min(rectangle.y + rectangle.height);
    WebGl2Scissor {
        x: left,
        y: top,
        width: (right - left).max(0.0),
        height: (bottom - top).max(0.0),
    }
}

/// Returns a colour scaled by an opacity, as the four normalised
/// channels the shader expects.
///
/// # Arguments
///
/// - `Color` - A `Color` parameter.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `[f32; 4]` - A `[f32` value.
pub(crate) fn scaled_color(color: Color, opacity: f64) -> [f32; 4] {
    let alpha: u8 = (color.alpha as f64 * opacity).round().clamp(0.0, 255.0) as u8;
    unpack_color(color.with_alpha(alpha).to_u32())
}

/// Compiles one shader stage, returning `None` when it fails.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - Shared reference to a `WebGl2RenderingContext`.
/// - `u32` - A 32-bit unsigned integer (`u32`).
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Option<WebGlShader>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn compile_shader(
    context: &WebGl2RenderingContext,
    kind: u32,
    source: &str,
) -> Option<WebGlShader> {
    let shader: WebGlShader = context.create_shader(kind)?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);
    let compiled: bool = context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false);
    if !compiled {
        let log: String = context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "the shader did not compile".to_string());
        web_sys::console::error_1(&format!("euv-chart: {log}").into());
        context.delete_shader(Some(&shader));
        return None;
    }
    Some(shader)
}
