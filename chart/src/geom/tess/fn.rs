use super::*;

/// Returns the signed area of the polygon starting at `start` and running
/// for `count` vertices.
///
/// The sign encodes the winding direction.
///
/// # Arguments
///
/// - `&[Point]` - Shared reference to a `[Point]`.
/// - `usize` - A non-negative integer (`usize`).
///
/// # Returns
///
/// - `f64` - A 64-bit float.
pub(crate) fn polygon_signed_area(points: &[Point], start: usize, count: usize) -> f64 {
    if count < 3 {
        return 0.0;
    }
    let mut sum: f64 = 0.0;
    for index in 0..count {
        let current: Point = points[(start + index) % count];
        let next: Point = points[(start + index + 1) % count];
        sum += current.x as f64 * next.y as f64 - next.x as f64 * current.y as f64;
    }
    sum * 0.5
}

/// Returns `true` when the point lies inside the polygon.
///
/// The test uses the crossing-number rule, which counts a crossing only
/// when the edge passes the horizontal ray of the point; the half-open
/// comparison on the `y` axis keeps vertices from being counted twice.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
/// - `&[Point]` - Shared reference to a `[Point]`.
///
/// # Returns
///
/// - `bool` - A boolean.
pub(crate) fn point_in_polygon(x: f64, y: f64, points: &[Point]) -> bool {
    if points.len() < 3 {
        return false;
    }
    let mut inside: bool = false;
    let mut previous: Point = points[points.len() - 1];
    for current in points.iter() {
        let (x0, y0) = (previous.x as f64, previous.y as f64);
        let (x1, y1) = (current.x as f64, current.y as f64);
        if (y0 > y) != (y1 > y) {
            let t: f64 = (y - y0) / (y1 - y0);
            if x < x0 + t * (x1 - x0) {
                inside = !inside;
            }
        }
        previous = *current;
    }
    inside
}

/// Evaluates a quadratic Bézier at parameter `t`.
///
/// # Arguments
///
/// - `(f64, f64)` - A `(f64, f64)` parameter.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `(f64, f64)` - A `(f64, f64)` value.
pub(crate) fn eval_quad(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), t: f64) -> (f64, f64) {
    let inverse: f64 = 1.0 - t;
    let a: f64 = inverse * inverse;
    let b: f64 = 2.0 * inverse * t;
    let c: f64 = t * t;
    (
        a * p0.0 + b * p1.0 + c * p2.0,
        a * p0.1 + b * p1.1 + c * p2.1,
    )
}

/// Evaluates a cubic Bézier at parameter `t`.
///
/// # Arguments
///
/// - `(f64, f64)` - A `(f64, f64)` parameter.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `(f64, f64)` - A `(f64, f64)` value.
pub(crate) fn eval_cubic(
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
    t: f64,
) -> (f64, f64) {
    let inverse: f64 = 1.0 - t;
    let a: f64 = inverse * inverse * inverse;
    let b: f64 = 3.0 * inverse * inverse * t;
    let c: f64 = 3.0 * inverse * t * t;
    let d: f64 = t * t * t;
    (
        a * p0.0 + b * p1.0 + c * p2.0 + d * p3.0,
        a * p0.1 + b * p1.1 + c * p2.1 + d * p3.1,
    )
}

/// Returns the number of segments needed to flatten a quadratic Bézier.
///
/// # Arguments
///
/// - `(f64, f64)` - A `(f64, f64)` parameter.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `usize` - A non-negative integer.
pub(crate) fn quad_segment_count(
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    tolerance: f64,
) -> usize {
    let dx: f64 = p0.0 - 2.0 * p1.0 + p2.0;
    let dy: f64 = p0.1 - 2.0 * p1.1 + p2.1;
    let deviation: f64 = (dx * dx + dy * dy).sqrt() / 8.0;
    if deviation <= tolerance {
        1
    } else {
        ((deviation / tolerance).sqrt().ceil() as usize).clamp(1, 64)
    }
}

/// Returns the point offset from `from` to `to` by `distance`, on the left
/// side of the direction of travel.
///
/// A negative distance offsets to the right.
///
/// # Arguments
///
/// - `Point` - A `Point` parameter.
/// - `f32` - A 32-bit float (`f32`).
///
/// # Returns
///
/// - `Point` - A `Point` value.
pub(crate) fn offset_point(from: Point, to: Point, distance: f32) -> Point {
    let dx: f32 = to.x - from.x;
    let dy: f32 = to.y - from.y;
    let length: f32 = (dx * dx + dy * dy).sqrt();
    if length < 1e-6 {
        return from;
    }
    Point::new(to.x - dy / length * distance, to.y + dx / length * distance)
}

/// Triangulates a set of contours and appends the triangles to the mesh.
///
/// A contour that is nested inside another contour and wound the other way
/// is treated as a hole: it is not emitted, and the outer contour is
/// filled as if the hole were not there. The hole is then punched out by
/// the fill rule at rasterisation time — for the non-zero rule, which is
/// what the chart renderer uses, a hole wound the other way leaves the
/// interior empty.
///
/// The `working` and `indices` buffers are scratch space owned by the
/// caller, so the function allocates nothing.
/// Returns the corners of a path that is a rectangle.
///
/// A rectangle is by far the most common shape of a chart — every bar
/// of a bar chart is one — and it never needs to be triangulated: two
/// triangles are the whole of it. Recognising it here saves the ear
/// clipping, and with it the allocations that come with it: a chart
/// over a million bars would otherwise allocate a million times over.
///
/// The rectangle has to be axis-aligned. A tilted rectangle is rare
/// enough that it can go the long way round.
///
/// # Arguments
///
/// - `&Path` - Shared reference to a `Path`.
///
/// # Returns
///
/// - `Option<[Point; 4]>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn rectangle_of(path: &Path) -> Option<[Point; 4]> {
    let commands: &[PathCommand] = path.commands();
    if commands.len() < 4 || commands.len() > 5 {
        return None;
    }
    let PathCommand::MoveTo(x0, y0) = commands[0] else {
        return None;
    };
    let mut corners: [(f32, f32); 4] = [(x0, y0); 4];
    let mut count: usize = 1usize;
    for command in &commands[1..] {
        match *command {
            PathCommand::LineTo(x, y) => {
                if count >= 4 {
                    return None;
                }
                corners[count] = (x, y);
                count += 1;
            }
            PathCommand::Close => {}
            _ => return None,
        }
    }
    if count != 4 {
        return None;
    }
    let [(x0, y0), (x1, y1), (x2, y2), (x3, y3)] = corners;
    // The corners of an axis-aligned rectangle come in two pairs that
    // share a coordinate.
    let upright = x0 == x1 && y1 == y2 && x2 == x3;
    let sideways = y0 == y1 && x1 == x2 && y2 == y3;
    if !upright && !sideways {
        return None;
    }
    Some([
        Point::new(x0, y0),
        Point::new(x1, y1),
        Point::new(x2, y2),
        Point::new(x3, y3),
    ])
}

/// Body of the `fill_contours_into` free function.
///
/// # Arguments
///
/// - `&mut SolidMesh` - Mutable reference to a `SolidMesh` (mutated in place).
/// - `&[Contour]` - Shared reference to a `[Contour]`.
/// - `u32` - A 32-bit unsigned integer (`u32`).
/// - `&mut Vec<Point>` - Mutable reference to a `Vec<Point>` (mutated in place).
/// - `&mut Vec<usize>` - Mutable reference to a `Vec<usize>` (mutated in place).
pub(crate) fn fill_contours_into(
    mesh: &mut SolidMesh,
    contours: &[Contour],
    color: u32,
    working: &mut Vec<Point>,
    indices: &mut Vec<usize>,
) {
    if contours.is_empty() {
        return;
    }

    // Collect every contour into a single working vertex array, and
    // remember where each contour starts.
    working.clear();
    let mut starts: Vec<usize> = Vec::with_capacity(contours.len());
    for contour in contours {
        starts.push(working.len());
        working.extend_from_slice(&contour.points);
    }

    // Decide which contours are holes. A contour is a hole when it is
    // contained by another contour and winds the other way.
    let mut holes: Vec<usize> = Vec::new();
    let mut outers: Vec<usize> = Vec::new();
    for index in 0..contours.len() {
        let count: usize = contours[index].points.len();
        if count < 3 {
            continue;
        }
        let area: f64 = polygon_signed_area(working, starts[index], count);
        if area.abs() < 1e-12 {
            continue;
        }
        if is_hole(working, &starts, contours, index) {
            holes.push(index);
        } else {
            outers.push(index);
        }
    }

    if outers.is_empty() {
        // Every contour is a hole: fall back to filling them as
        // individual polygons, which keeps degenerate input visible
        // rather than silently dropping it.
        for index in holes {
            let count: usize = contours[index].points.len();
            append_ear_clipped(
                mesh,
                &working[starts[index]..starts[index] + count],
                color,
                indices,
            );
        }
        return;
    }

    for index in outers {
        let count: usize = contours[index].points.len();
        append_ear_clipped(
            mesh,
            &working[starts[index]..starts[index] + count],
            color,
            indices,
        );
    }
}

/// Returns `true` when the contour at `index` is nested inside another
/// contour and therefore acts as a hole.
///
/// # Arguments
///
/// - `&[Point]` - Shared reference to a `[Point]`.
/// - `&[usize]` - Shared reference to a `[usize]`.
/// - `&[Contour]` - Shared reference to a `[Contour]`.
/// - `usize` - A non-negative integer (`usize`).
///
/// # Returns
///
/// - `bool` - A boolean.
fn is_hole(working: &[Point], starts: &[usize], contours: &[Contour], index: usize) -> bool {
    let points: &Vec<Point> = &contours[index].points;
    if points.is_empty() {
        return false;
    }
    let probe: Point = points[0];
    for other in 0..contours.len() {
        if other == index {
            continue;
        }
        let other_points: &Vec<Point> = &contours[other].points;
        if other_points.len() < 3 || points.len() < 3 {
            continue;
        }
        if point_in_polygon(
            probe.x as f64,
            probe.y as f64,
            &working[starts[other]..starts[other] + other_points.len()],
        ) {
            let inner: f64 = polygon_signed_area(working, starts[index], points.len());
            let outer: f64 = polygon_signed_area(working, starts[other], other_points.len());
            // Nested and wound the other way: a hole.
            if inner * outer < 0.0 {
                return true;
            }
        }
    }
    false
}

/// Ear-clips one simple polygon and appends the triangles to the mesh.
///
/// # Arguments
///
/// - `&mut SolidMesh` - Mutable reference to a `SolidMesh` (mutated in place).
/// - `&[Point]` - Shared reference to a `[Point]`.
/// - `u32` - A 32-bit unsigned integer (`u32`).
/// - `&mut Vec<usize>` - Mutable reference to a `Vec<usize>` (mutated in place).
fn append_ear_clipped(
    mesh: &mut SolidMesh,
    points: &[Point],
    color: u32,
    indices: &mut Vec<usize>,
) {
    if points.len() < 3 {
        return;
    }
    indices.clear();
    ear_clip(points, indices);
    for triple in indices.as_chunks::<3>().0 {
        mesh.push_triangle(
            SolidVertex {
                x: points[triple[0]].x,
                y: points[triple[0]].y,
                color,
            },
            SolidVertex {
                x: points[triple[1]].x,
                y: points[triple[1]].y,
                color,
            },
            SolidVertex {
                x: points[triple[2]].x,
                y: points[triple[2]].y,
                color,
            },
        );
    }
}

/// Expands a set of contours into stroke geometry and appends the result
/// to the mesh.
///
/// Every segment becomes a quad and every join becomes a wedge, so the
/// whole stroke ends up in one indexed draw call no matter how many
/// segments it has.
///
/// # Arguments
///
/// - `&mut SolidMesh` - Mutable reference to a `SolidMesh` (mutated in place).
/// - `&[Contour]` - Shared reference to a `[Contour]`.
/// - `&StrokeStyle` - Shared reference to a `StrokeStyle`.
/// - `u32` - A 32-bit unsigned integer (`u32`).
pub(crate) fn stroke_contours_into(
    mesh: &mut SolidMesh,
    contours: &[Contour],
    style: &StrokeStyle,
    color: u32,
) {
    let half_width: f32 = style.width * 0.5;
    if half_width <= 0.0 {
        return;
    }
    for contour in contours {
        stroke_one_contour(mesh, contour, style, color, half_width);
    }
}

/// Expands one contour into stroke geometry.
///
/// # Arguments
///
/// - `&mut SolidMesh` - Mutable reference to a `SolidMesh` (mutated in place).
/// - `&Contour` - Shared reference to a `Contour`.
/// - `&StrokeStyle` - Shared reference to a `StrokeStyle`.
/// - `u32` - A 32-bit unsigned integer (`u32`).
/// - `f32` - A 32-bit float (`f32`).
fn stroke_one_contour(
    mesh: &mut SolidMesh,
    contour: &Contour,
    style: &StrokeStyle,
    color: u32,
    half_width: f32,
) {
    let points: &Vec<Point> = &contour.points;
    if points.len() < 2 {
        if points.len() == 1 && style.cap == LineCap::Round {
            push_disc(mesh, points[0], half_width, color);
        }
        return;
    }

    let closed: bool = contour.closed;
    let segment_count: usize = if closed {
        points.len()
    } else {
        points.len() - 1
    };

    for index in 0..segment_count {
        let a: Point = points[index];
        let b: Point = points[(index + 1) % points.len()];
        push_segment_quad(mesh, a, b, half_width, color);
    }

    // Joins sit on the interior vertices of the contour.
    let join_start: usize = if closed { 0 } else { 1 };
    let join_end: usize = if closed {
        points.len()
    } else {
        points.len().saturating_sub(1)
    };
    for index in join_start..join_end {
        let previous: Point = points[(index + points.len() - 1) % points.len()];
        let current: Point = points[index % points.len()];
        let next: Point = points[(index + 1) % points.len()];
        push_join(mesh, previous, current, next, style, color, half_width);
    }

    // Caps sit on the two end vertices of an open contour.
    if !closed {
        match style.cap {
            LineCap::Round => {
                push_disc(mesh, points[0], half_width, color);
                push_disc(mesh, points[points.len() - 1], half_width, color);
            }
            LineCap::Square => {
                let first: Point = points[0];
                let second: Point = points[1];
                let (dx, dy) = normalized(first.x - second.x, first.y - second.y);
                let extended: Point =
                    Point::new(first.x + dx * half_width, first.y + dy * half_width);
                push_segment_quad(mesh, extended, first, half_width, color);

                let last: Point = points[points.len() - 1];
                let before_last: Point = points[points.len() - 2];
                let (dx, dy) = normalized(last.x - before_last.x, last.y - before_last.y);
                let extended: Point =
                    Point::new(last.x + dx * half_width, last.y + dy * half_width);
                push_segment_quad(mesh, last, extended, half_width, color);
            }
            LineCap::Butt => {}
        }
    }
}

/// Appends the quad covering one stroke segment.
///
/// # Arguments
///
/// - `&mut SolidMesh` - Mutable reference to a `SolidMesh` (mutated in place).
/// - `Point` - A `Point` parameter.
/// - `f32` - A 32-bit float (`f32`).
/// - `u32` - A 32-bit unsigned integer (`u32`).
fn push_segment_quad(mesh: &mut SolidMesh, a: Point, b: Point, half_width: f32, color: u32) {
    let dx: f32 = b.x - a.x;
    let dy: f32 = b.y - a.y;
    let length: f32 = (dx * dx + dy * dy).sqrt();
    if length < 1e-6 {
        return;
    }
    let nx: f32 = -dy / length * half_width;
    let ny: f32 = dx / length * half_width;
    mesh.push_quad(
        SolidVertex {
            x: a.x + nx,
            y: a.y + ny,
            color,
        },
        SolidVertex {
            x: b.x + nx,
            y: b.y + ny,
            color,
        },
        SolidVertex {
            x: b.x - nx,
            y: b.y - ny,
            color,
        },
        SolidVertex {
            x: a.x - nx,
            y: a.y - ny,
            color,
        },
    );
}

/// Appends the geometry of one join between two segments.
///
/// # Arguments
///
/// - `&mut SolidMesh` - Mutable reference to a `SolidMesh` (mutated in place).
/// - `Point` - A `Point` parameter.
/// - `&StrokeStyle` - Shared reference to a `StrokeStyle`.
/// - `u32` - A 32-bit unsigned integer (`u32`).
/// - `f32` - A 32-bit float (`f32`).
fn push_join(
    mesh: &mut SolidMesh,
    previous: Point,
    current: Point,
    next: Point,
    style: &StrokeStyle,
    color: u32,
    half_width: f32,
) {
    match style.join {
        LineJoin::Round => push_disc(mesh, current, half_width, color),
        LineJoin::Bevel => push_bevel(mesh, previous, current, next, half_width, color),
        LineJoin::Miter => {
            if !push_miter(mesh, previous, current, next, half_width, color) {
                push_bevel(mesh, previous, current, next, half_width, color);
            }
        }
    }
}

/// Appends a fan approximating a disc, used for round caps and joins.
///
/// # Arguments
///
/// - `&mut SolidMesh` - Mutable reference to a `SolidMesh` (mutated in place).
/// - `Point` - A `Point` parameter.
/// - `f32` - A 32-bit float (`f32`).
/// - `u32` - A 32-bit unsigned integer (`u32`).
fn push_disc(mesh: &mut SolidMesh, center: Point, radius: f32, color: u32) {
    let segments: usize = DEFAULT_CIRCLE_SEGMENTS;
    let mut previous: Point = center;
    for step in 1..=segments {
        let angle: f64 = step as f64 / segments as f64 * core::f64::consts::TAU;
        let current: Point = Point::new(
            center.x + radius * angle.cos() as f32,
            center.y + radius * angle.sin() as f32,
        );
        mesh.push_triangle(
            SolidVertex {
                x: center.x,
                y: center.y,
                color,
            },
            SolidVertex {
                x: previous.x,
                y: previous.y,
                color,
            },
            SolidVertex {
                x: current.x,
                y: current.y,
                color,
            },
        );
        previous = current;
    }
}

/// Appends the triangle that fills a bevelled join.
///
/// # Arguments
///
/// - `&mut SolidMesh` - Mutable reference to a `SolidMesh` (mutated in place).
/// - `Point` - A `Point` parameter.
/// - `f32` - A 32-bit float (`f32`).
/// - `u32` - A 32-bit unsigned integer (`u32`).
fn push_bevel(
    mesh: &mut SolidMesh,
    previous: Point,
    current: Point,
    next: Point,
    half_width: f32,
    color: u32,
) {
    let first: Point = offset_point(previous, current, half_width);
    let second: Point = offset_point(current, next, half_width);
    let first_inner: Point = offset_point(previous, current, -half_width);
    let second_inner: Point = offset_point(current, next, -half_width);
    mesh.push_triangle(
        SolidVertex {
            x: first.x,
            y: first.y,
            color,
        },
        SolidVertex {
            x: first_inner.x,
            y: first_inner.y,
            color,
        },
        SolidVertex {
            x: second_inner.x,
            y: second_inner.y,
            color,
        },
    );
    mesh.push_triangle(
        SolidVertex {
            x: first.x,
            y: first.y,
            color,
        },
        SolidVertex {
            x: second_inner.x,
            y: second_inner.y,
            color,
        },
        SolidVertex {
            x: second.x,
            y: second.y,
            color,
        },
    );
}

/// Appends the geometry of a mitred join, returning `false` when the
/// miter would be too long and the caller should bevel instead.
///
/// # Arguments
///
/// - `&mut SolidMesh` - Mutable reference to a `SolidMesh` (mutated in place).
/// - `Point` - A `Point` parameter.
/// - `f32` - A 32-bit float (`f32`).
/// - `u32` - A 32-bit unsigned integer (`u32`).
///
/// # Returns
///
/// - `bool` - A boolean.
fn push_miter(
    mesh: &mut SolidMesh,
    previous: Point,
    current: Point,
    next: Point,
    half_width: f32,
    color: u32,
) -> bool {
    let (unit_in_x, unit_in_y) = normalized(current.x - previous.x, current.y - previous.y);
    let (unit_out_x, unit_out_y) = normalized(next.x - current.x, next.y - current.y);
    let cross: f32 = unit_in_x * unit_out_y - unit_in_y * unit_out_x;
    if cross.abs() < 1e-6 {
        // The two segments are collinear: nothing to fill.
        return true;
    }
    let dot: f32 = (unit_in_x * unit_out_x + unit_in_y * unit_out_y).clamp(-1.0, 1.0);
    let half_angle: f32 = ((1.0 + dot) * 0.5).sqrt();
    if half_angle < 1e-6 {
        return false;
    }
    let miter_length: f32 = half_width / half_angle;
    if miter_length > half_width * MITER_LIMIT {
        return false;
    }
    let normal_in_x: f32 = -unit_in_y;
    let normal_in_y: f32 = unit_in_x;
    let normal_out_x: f32 = -unit_out_y;
    let normal_out_y: f32 = unit_out_x;
    let sum_x: f32 = normal_in_x + normal_out_x;
    let sum_y: f32 = normal_in_y + normal_out_y;
    let sum_length: f32 = (sum_x * sum_x + sum_y * sum_y).sqrt();
    if sum_length < 1e-6 {
        return false;
    }
    let miter: Point = Point::new(
        current.x + sum_x / sum_length * miter_length,
        current.y + sum_y / sum_length * miter_length,
    );
    let first: Point = offset_point(previous, current, half_width);
    let second: Point = offset_point(current, next, half_width);
    let first_inner: Point = offset_point(previous, current, -half_width);
    let second_inner: Point = offset_point(current, next, -half_width);
    mesh.push_triangle(
        SolidVertex {
            x: first.x,
            y: first.y,
            color,
        },
        SolidVertex {
            x: first_inner.x,
            y: first_inner.y,
            color,
        },
        SolidVertex {
            x: miter.x,
            y: miter.y,
            color,
        },
    );
    mesh.push_triangle(
        SolidVertex {
            x: first_inner.x,
            y: first_inner.y,
            color,
        },
        SolidVertex {
            x: second_inner.x,
            y: second_inner.y,
            color,
        },
        SolidVertex {
            x: miter.x,
            y: miter.y,
            color,
        },
    );
    mesh.push_triangle(
        SolidVertex {
            x: miter.x,
            y: miter.y,
            color,
        },
        SolidVertex {
            x: second_inner.x,
            y: second_inner.y,
            color,
        },
        SolidVertex {
            x: second.x,
            y: second.y,
            color,
        },
    );
    true
}

/// Normalises a vector, returning `(0.0, 0.0)` for a zero-length vector.
///
/// # Arguments
///
/// - `f32` - A 32-bit float (`f32`).
///
/// # Returns
///
/// - `(f32, f32)` - A `(f32, f32)` value.
fn normalized(dx: f32, dy: f32) -> (f32, f32) {
    let length: f32 = (dx * dx + dy * dy).sqrt();
    if length < 1e-6 {
        (0.0, 0.0)
    } else {
        (dx / length, dy / length)
    }
}

/// Triangulates a simple polygon with the ear-clipping algorithm.
///
/// The output is a list of vertex indices, three per triangle. The
/// algorithm keeps the polygon in a circular doubly-linked list stored in
/// one array, so removing an ear is a constant-time splice rather than a
/// shift of the whole vector.
///
/// Degenerate input — a polygon with fewer than three vertices, or one
/// that is entirely collinear — produces no triangles.
///
/// # Arguments
///
/// - `&[Point]` - Shared reference to a `[Point]`.
/// - `&mut Vec<usize>` - Mutable reference to a `Vec<usize>` (mutated in place).
pub(crate) fn ear_clip(points: &[Point], output: &mut Vec<usize>) {
    let count: usize = points.len();
    if count < 3 {
        return;
    }
    // Orient the polygon so that the interior is on the left of every
    // directed edge, which is the orientation the ear test assumes.
    let area: f64 = polygon_signed_area(points, 0, count);
    let mut ordered: Vec<Point> = Vec::with_capacity(count);
    if area > 0.0 {
        ordered.extend_from_slice(points);
    } else {
        ordered.extend(points.iter().rev().copied());
    }
    // The linked list: `next` and `previous` hold the neighbour of each
    // vertex in the remaining polygon.
    let mut next: Vec<usize> = vec![0usize; count];
    let mut previous: Vec<usize> = vec![0usize; count];
    for index in 0..count {
        next[index] = (index + 1) % count;
        previous[index] = (index + count - 1) % count;
    }
    let mut remaining: usize = count;
    let mut cursor: usize = 0usize;
    let mut guard: usize = 0usize;
    let guard_limit: usize = count * count + count;
    while remaining > 3 {
        if guard > guard_limit {
            break;
        }
        guard += 1;

        let current: usize = cursor;
        let a: usize = previous[current];
        let b: usize = current;
        let c: usize = next[current];

        if is_ear(&ordered, &previous, &next, a, b, c) {
            output.push(a);
            output.push(b);
            output.push(c);
            next[a] = c;
            previous[c] = a;
            remaining -= 1;
            cursor = a;
            continue;
        }

        cursor = c;
    }
    if remaining == 3 {
        let a: usize = cursor;
        let b: usize = next[a];
        let c: usize = next[b];
        output.push(a);
        output.push(b);
        output.push(c);
    }
}

/// Returns `true` when the triangle `(a, b, c)` is an ear: it is convex
/// and contains no other vertex of the polygon.
///
/// # Arguments
///
/// - `&[Point]` - Shared reference to a `[Point]`.
/// - `&[usize]` - Shared reference to a `[usize]`.
/// - `usize` - A non-negative integer (`usize`).
///
/// # Returns
///
/// - `bool` - A boolean.
fn is_ear(
    points: &[Point],
    previous: &[usize],
    next: &[usize],
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let pa: Point = points[a];
    let pb: Point = points[b];
    let pc: Point = points[c];

    let cross: f64 =
        (pb.x - pa.x) as f64 * (pc.y - pa.y) as f64 - (pb.y - pa.y) as f64 * (pc.x - pa.x) as f64;
    if cross <= 0.0 {
        // Reflex or collinear: not an ear.
        return false;
    }

    let mut current: usize = next[c];
    while current != a {
        let p: Point = points[current];
        if point_in_triangle(p.x as f64, p.y as f64, pa, pb, pc) {
            return false;
        }
        current = next[current];
    }
    let _: &[usize] = previous;
    true
}

/// Returns `true` when the point lies inside the given triangle.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
/// - `Point` - A `Point` parameter.
///
/// # Returns
///
/// - `bool` - A boolean.
fn point_in_triangle(px: f64, py: f64, a: Point, b: Point, c: Point) -> bool {
    let d1: f64 = sign(px, py, a.x as f64, a.y as f64, b.x as f64, b.y as f64);
    let d2: f64 = sign(px, py, b.x as f64, b.y as f64, c.x as f64, c.y as f64);
    let d3: f64 = sign(px, py, c.x as f64, c.y as f64, a.x as f64, a.y as f64);
    let has_negative: bool = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_positive: bool = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);
    !(has_negative && has_positive)
}

/// Returns the sign of the cross product of two vectors.
///
/// # Arguments
///
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `f64` - A 64-bit float.
fn sign(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> f64 {
    ((x1 - x3) * (y2 - y3) - (x2 - x3) * (y1 - y3)).signum()
}
