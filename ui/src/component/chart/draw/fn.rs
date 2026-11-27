use super::*;

/// Returns an accessible high-contrast text color (dark ink or white ink) atop `bg`.
///
/// # Arguments
///
/// - `Color` - Background surface color.
///
/// # Returns
///
/// - `Color` - Contrasting text ink color.
#[must_use]
pub(crate) fn contrast_ink_for(bg: Color) -> Color {
    let lum: u32 =
        (bg.get_r() as u32) * 299 + (bg.get_g() as u32) * 587 + (bg.get_b() as u32) * 114;
    if lum < 145_000 {
        Color::hex(0xFFFFFF)
    } else {
        Color::hex(0x0F172A)
    }
}

/// Emits a crisp dashed straight line segment from `from` to `to`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Point` - Segment start point.
/// - `Point` - Segment end point.
/// - `Color` - Stroke color.
/// - `f32` - Stroke width in logical pixels.
/// - `f32` - Dash length in logical pixels.
/// - `f32` - Gap length in logical pixels.
pub(crate) fn draw_dashed_line(
    dl: &mut DisplayList,
    from: Point,
    to: Point,
    color: Color,
    width: f32,
    dash: f32,
    gap: f32,
) {
    let dx: f32 = to.get_x() - from.get_x();
    let dy: f32 = to.get_y() - from.get_y();
    let total_len: f32 = (dx * dx + dy * dy).sqrt();
    if !total_len.is_finite() || total_len < 0.5 {
        return;
    }
    let ux: f32 = dx / total_len;
    let uy: f32 = dy / total_len;
    let step: f32 = (dash + gap).max(1.0);
    let mut dist: f32 = 0.0;
    while dist < total_len {
        let end_d: f32 = (dist + dash).min(total_len);
        dl.push(DrawCommand::Line {
            from: Point::new(from.get_x() + ux * dist, from.get_y() + uy * dist),
            to: Point::new(from.get_x() + ux * end_d, from.get_y() + uy * end_d),
            color,
            width,
        });
        dist += step;
    }
}

/// Emits a connected polyline with optional dashing.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `&[Point]` - Ordered polyline vertices.
/// - `Color` - Stroke color.
/// - `f32` - Stroke width.
/// - `bool` - When `true`, strokes with dashes.
pub(crate) fn draw_polyline(
    dl: &mut DisplayList,
    pts: &[Point],
    color: Color,
    width: f32,
    dashed: bool,
) {
    if pts.len() < 2 {
        return;
    }
    for i in 0..(pts.len() - 1) {
        if dashed {
            draw_dashed_line(
                dl,
                pts[i],
                pts[i + 1],
                color,
                width,
                DRAW_DASH_LEN,
                DRAW_GAP_LEN,
            );
        } else {
            dl.push(DrawCommand::Line {
                from: pts[i],
                to: pts[i + 1],
                color,
                width,
            });
        }
    }
}

/// Samples a smooth Catmull-Rom spline passing cleanly through all control points in `pts`.
///
/// # Arguments
///
/// - `&[Point]` - Control vertices.
/// - `usize` - Subdivision steps per span.
///
/// # Returns
///
/// - `Vec<Point>` - Dense sampled curve points.
#[must_use]
pub(crate) fn sample_catmull_rom(pts: &[Point], steps: usize) -> Vec<Point> {
    if pts.len() <= 2 {
        return pts.to_vec();
    }
    let segs: usize = steps.max(2);
    let n: usize = pts.len();
    let mut out: Vec<Point> = Vec::with_capacity((n - 1) * segs + 1);
    for i in 0..(n - 1) {
        let p0: Point = if i == 0 { pts[0] } else { pts[i - 1] };
        let p1: Point = pts[i];
        let p2: Point = pts[i + 1];
        let p3: Point = if i + 2 < n { pts[i + 2] } else { pts[n - 1] };
        for s in 0..segs {
            let t: f32 = (s as f32) / (segs as f32);
            let t2: f32 = t * t;
            let t3: f32 = t2 * t;
            let x: f32 = 0.5
                * ((2.0 * p1.get_x())
                    + (-p0.get_x() + p2.get_x()) * t
                    + (2.0 * p0.get_x() - 5.0 * p1.get_x() + 4.0 * p2.get_x() - p3.get_x()) * t2
                    + (-p0.get_x() + 3.0 * p1.get_x() - 3.0 * p2.get_x() + p3.get_x()) * t3);
            let y: f32 = 0.5
                * ((2.0 * p1.get_y())
                    + (-p0.get_y() + p2.get_y()) * t
                    + (2.0 * p0.get_y() - 5.0 * p1.get_y() + 4.0 * p2.get_y() - p3.get_y()) * t2
                    + (-p0.get_y() + 3.0 * p1.get_y() - 3.0 * p2.get_y() + p3.get_y()) * t3);
            out.push(Point::new(x, y));
        }
    }
    if let Some(&last) = pts.last() {
        out.push(last);
    }
    out
}

/// Samples a quadratic Bézier curve `(p0, ctrl, p2)`.
///
/// # Arguments
///
/// - `Point` - Start point.
/// - `Point` - Quadratic control point.
/// - `Point` - End point.
/// - `usize` - Subdivision count.
///
/// # Returns
///
/// - `Vec<Point>` - Sampled curve points.
#[must_use]
pub(crate) fn sample_quad_bezier(p0: Point, ctrl: Point, p2: Point, steps: usize) -> Vec<Point> {
    let segs: usize = steps.max(4);
    let mut out: Vec<Point> = Vec::with_capacity(segs + 1);
    for i in 0..=segs {
        let t: f32 = (i as f32) / (segs as f32);
        let u: f32 = 1.0 - t;
        let x: f32 = u * u * p0.get_x() + 2.0 * u * t * ctrl.get_x() + t * t * p2.get_x();
        let y: f32 = u * u * p0.get_y() + 2.0 * u * t * ctrl.get_y() + t * t * p2.get_y();
        out.push(Point::new(x, y));
    }
    out
}

/// Samples a cubic Bézier curve `(p0, c1, c2, p3)`.
///
/// # Arguments
///
/// - `Point` - Start point.
/// - `Point` - First control point.
/// - `Point` - Second control point.
/// - `Point` - End point.
/// - `usize` - Subdivision count.
///
/// # Returns
///
/// - `Vec<Point>` - Sampled curve points.
#[must_use]
pub(crate) fn sample_cubic_bezier(
    p0: Point,
    c1: Point,
    c2: Point,
    p3: Point,
    steps: usize,
) -> Vec<Point> {
    let segs: usize = steps.max(4);
    let mut out: Vec<Point> = Vec::with_capacity(segs + 1);
    for i in 0..=segs {
        let t: f32 = (i as f32) / (segs as f32);
        let u: f32 = 1.0 - t;
        let u2: f32 = u * u;
        let t2: f32 = t * t;
        let x: f32 = u2 * u * p0.get_x()
            + 3.0 * u2 * t * c1.get_x()
            + 3.0 * u * t2 * c2.get_x()
            + t2 * t * p3.get_x();
        let y: f32 = u2 * u * p0.get_y()
            + 3.0 * u2 * t * c1.get_y()
            + 3.0 * u * t2 * c2.get_y()
            + t2 * t * p3.get_y();
        out.push(Point::new(x, y));
    }
    out
}

/// Fills a closed polygon via ear-clipping triangulation (handles both convex and concave vector map/radar shapes).
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `&[Point]` - Closed polygon perimeter vertices.
/// - `Color` - Interior fill color.
pub(crate) fn draw_polygon_fill(dl: &mut DisplayList, poly: &[Point], color: Color) {
    if poly.len() < 3 {
        return;
    }
    let mut pts: Vec<Point> = poly.to_vec();
    if pts.len() >= 2 {
        let first: Point = pts[0];
        let last: Point = pts[pts.len() - 1];
        if (first.get_x() - last.get_x()).abs() < 1e-3
            && (first.get_y() - last.get_y()).abs() < 1e-3
        {
            let _: Option<Point> = pts.pop();
        }
    }
    if pts.len() < 3 {
        return;
    }
    let mut signed_area: f32 = 0.0;
    for i in 0..pts.len() {
        let a: Point = pts[i];
        let b: Point = pts[(i + 1) % pts.len()];
        signed_area += a.get_x() * b.get_y() - b.get_x() * a.get_y();
    }
    if signed_area < 0.0 {
        pts.reverse();
    }
    let mut guard: usize = 0;
    let max_iters: usize = pts.len() * pts.len() + 4;
    while pts.len() > 3 && guard < max_iters {
        guard += 1;
        let len: usize = pts.len();
        let mut clipped: bool = false;
        for i in 0..len {
            let prev: Point = pts[(i + len - 1) % len];
            let curr: Point = pts[i];
            let next: Point = pts[(i + 1) % len];
            let cross: f32 = (curr.get_x() - prev.get_x()) * (next.get_y() - prev.get_y())
                - (curr.get_y() - prev.get_y()) * (next.get_x() - prev.get_x());
            if cross <= 1e-5 {
                continue;
            }
            let mut has_inside: bool = false;
            for (j, &probe) in pts.iter().enumerate() {
                if j == i || j == (i + len - 1) % len || j == (i + 1) % len {
                    continue;
                }
                if point_in_triangle(probe, prev, curr, next) {
                    has_inside = true;
                    break;
                }
            }
            if !has_inside {
                dl.push(DrawCommand::Triangle {
                    p0: prev,
                    p1: curr,
                    p2: next,
                    color,
                });
                let _: Point = pts.remove(i);
                clipped = true;
                break;
            }
        }
        if !clipped {
            break;
        }
    }
    if pts.len() >= 3 {
        let anchor: Point = pts[0];
        for i in 1..(pts.len() - 1) {
            dl.push(DrawCommand::Triangle {
                p0: anchor,
                p1: pts[i],
                p2: pts[i + 1],
                color,
            });
        }
    }
}

/// Tests whether `p` lies strictly inside or on the boundary of triangle `(a, b, c)`.
///
/// # Arguments
///
/// - `Point` - Probe point.
/// - `Point` - Triangle vertex A.
/// - `Point` - Triangle vertex B.
/// - `Point` - Triangle vertex C.
///
/// # Returns
///
/// - `bool` - True if `p` lies within the triangle.
#[must_use]
fn point_in_triangle(p: Point, a: Point, b: Point, c: Point) -> bool {
    let c1: f32 = (b.get_x() - a.get_x()) * (p.get_y() - a.get_y())
        - (b.get_y() - a.get_y()) * (p.get_x() - a.get_x());
    let c2: f32 = (c.get_x() - b.get_x()) * (p.get_y() - b.get_y())
        - (c.get_y() - b.get_y()) * (p.get_x() - b.get_x());
    let c3: f32 = (a.get_x() - c.get_x()) * (p.get_y() - c.get_y())
        - (a.get_y() - c.get_y()) * (p.get_x() - c.get_x());
    let has_neg: bool = (c1 < -1e-4) || (c2 < -1e-4) || (c3 < -1e-4);
    let has_pos: bool = (c1 > 1e-4) || (c2 > 1e-4) || (c3 > 1e-4);
    !(has_neg && has_pos)
}

/// Strokes the perimeter of a closed polygon.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `&[Point]` - Closed polygon vertices.
/// - `Color` - Stroke color.
/// - `f32` - Stroke width in logical pixels.
pub(crate) fn draw_polygon_stroke(dl: &mut DisplayList, poly: &[Point], color: Color, width: f32) {
    if poly.len() < 2 {
        return;
    }
    for i in 0..poly.len() {
        dl.push(DrawCommand::Line {
            from: poly[i],
            to: poly[(i + 1) % poly.len()],
            color,
            width,
        });
    }
}

/// Fills a vertical strip or ribbon between upper polyline `top` and lower polyline `bottom`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `&[Point]` - Upper boundary vertices.
/// - `&[Point]` - Lower boundary vertices (same length as `top`).
/// - `Color` - Ribbon fill color.
pub(crate) fn draw_ribbon_fill(
    dl: &mut DisplayList,
    top: &[Point],
    bottom: &[Point],
    color: Color,
) {
    let n: usize = top.len().min(bottom.len());
    if n < 2 {
        return;
    }
    for i in 0..(n - 1) {
        let t0: Point = top[i];
        let t1: Point = top[i + 1];
        let b0: Point = bottom[i];
        let b1: Point = bottom[i + 1];
        dl.push(DrawCommand::Triangle {
            p0: t0,
            p1: t1,
            p2: b1,
            color,
        });
        dl.push(DrawCommand::Triangle {
            p0: t0,
            p1: b1,
            p2: b0,
            color,
        });
    }
}

/// Fills a circular wedge or annular ring sector centered at `center` from `start_rad` to `end_rad`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Point` - Circle center.
/// - `f32` - Inner radius (`0.0` for solid pie slice).
/// - `f32` - Outer radius.
/// - `f32` - Start angle in radians (standard screen math coordinates: `0` = east, `-PI/2` = north).
/// - `f32` - End angle in radians.
/// - `Color` - Fill color.
pub(crate) fn draw_annular_sector(
    dl: &mut DisplayList,
    center: Point,
    inner_r: f32,
    outer_r: f32,
    start_rad: f32,
    end_rad: f32,
    color: Color,
) {
    let sweep: f32 = end_rad - start_rad;
    if sweep.abs() < 1e-4 || outer_r <= inner_r.max(0.0) {
        return;
    }
    let steps: usize = ((sweep.abs() / TAU) * (DRAW_ARC_FULL_STEPS as f32))
        .ceil()
        .max(3.0) as usize;
    let r_in: f32 = inner_r.max(0.0);
    for i in 0..steps {
        let a0: f32 = start_rad + sweep * ((i as f32) / (steps as f32));
        let a1: f32 = start_rad + sweep * (((i + 1) as f32) / (steps as f32));
        let o0: Point = Point::new(
            center.get_x() + outer_r * a0.cos(),
            center.get_y() + outer_r * a0.sin(),
        );
        let o1: Point = Point::new(
            center.get_x() + outer_r * a1.cos(),
            center.get_y() + outer_r * a1.sin(),
        );
        if r_in < 0.5 {
            dl.push(DrawCommand::Triangle {
                p0: center,
                p1: o0,
                p2: o1,
                color,
            });
        } else {
            let i0: Point = Point::new(
                center.get_x() + r_in * a0.cos(),
                center.get_y() + r_in * a0.sin(),
            );
            let i1: Point = Point::new(
                center.get_x() + r_in * a1.cos(),
                center.get_y() + r_in * a1.sin(),
            );
            dl.push(DrawCommand::Triangle {
                p0: i0,
                p1: o0,
                p2: o1,
                color,
            });
            dl.push(DrawCommand::Triangle {
                p0: i0,
                p1: o1,
                p2: i1,
                color,
            });
        }
    }
}

/// Strokes a circular arc centered at `center` from `start_rad` to `end_rad`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Point` - Arc center.
/// - `f32` - Arc radius.
/// - `f32` - Start angle in radians.
/// - `f32` - End angle in radians.
/// - `Color` - Stroke color.
/// - `f32` - Stroke width in logical pixels.
pub(crate) fn draw_arc_stroke(
    dl: &mut DisplayList,
    center: Point,
    radius: f32,
    start_rad: f32,
    end_rad: f32,
    color: Color,
    width: f32,
) {
    let sweep: f32 = end_rad - start_rad;
    if sweep.abs() < 1e-4 || radius <= 0.5 {
        return;
    }
    let steps: usize = ((sweep.abs() / TAU) * (DRAW_ARC_FULL_STEPS as f32))
        .ceil()
        .max(4.0) as usize;
    let mut prev: Point = Point::new(
        center.get_x() + radius * start_rad.cos(),
        center.get_y() + radius * start_rad.sin(),
    );
    for i in 1..=steps {
        let a: f32 = start_rad + sweep * ((i as f32) / (steps as f32));
        let cur: Point = Point::new(
            center.get_x() + radius * a.cos(),
            center.get_y() + radius * a.sin(),
        );
        dl.push(DrawCommand::Line {
            from: prev,
            to: cur,
            color,
            width,
        });
        prev = cur;
    }
}

/// Draws a vector data marker symbol (`Circle`, `Rect`, `RoundRect`, `Triangle`, `Diamond`, `Pin`).
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `EuvChartSymbol` - Symbol shape.
/// - `Point` - Symbol center point.
/// - `f32` - Symbol radius in logical pixels.
/// - `Color` - Fill color.
/// - `Color` - Subtle outline / contrast halo color.
pub(crate) fn draw_symbol(
    dl: &mut DisplayList,
    symbol: EuvChartSymbol,
    center: Point,
    radius: f32,
    fill: Color,
    halo: Color,
) {
    let r: f32 = radius.max(1.5);
    let cx: f32 = center.get_x();
    let cy: f32 = center.get_y();
    match symbol {
        EuvChartSymbol::None => {}
        EuvChartSymbol::Circle => {
            dl.push(DrawCommand::Circle {
                center,
                radius: r + 1.0,
                color: halo,
            });
            dl.push(DrawCommand::Circle {
                center,
                radius: r,
                color: fill,
            });
        }
        EuvChartSymbol::Rect => {
            dl.push(DrawCommand::FillRoundedRect {
                rect: Rect::new(cx - r, cy - r, r * 2.0, r * 2.0),
                color: fill,
                radius: CornerRadius::all(0.0),
                border: Border::new(1.0, halo),
            });
        }
        EuvChartSymbol::RoundRect => {
            dl.push(DrawCommand::FillRoundedRect {
                rect: Rect::new(cx - r, cy - r * 0.72, r * 2.0, r * 1.44),
                color: fill,
                radius: CornerRadius::all((r * 0.45).max(2.0)),
                border: Border::new(1.0, halo),
            });
        }
        EuvChartSymbol::Triangle => {
            dl.push(DrawCommand::Triangle {
                p0: Point::new(cx, cy - r * 1.2),
                p1: Point::new(cx + r * 1.1, cy + r * 0.9),
                p2: Point::new(cx - r * 1.1, cy + r * 0.9),
                color: fill,
            });
        }
        EuvChartSymbol::Diamond => {
            let top: Point = Point::new(cx, cy - r * 1.25);
            let right: Point = Point::new(cx + r * 1.25, cy);
            let bottom: Point = Point::new(cx, cy + r * 1.25);
            let left: Point = Point::new(cx - r * 1.25, cy);
            dl.push(DrawCommand::Triangle {
                p0: top,
                p1: right,
                p2: bottom,
                color: fill,
            });
            dl.push(DrawCommand::Triangle {
                p0: top,
                p1: bottom,
                p2: left,
                color: fill,
            });
        }
        EuvChartSymbol::Pin => {
            let head: Point = Point::new(cx, cy - r * 1.3);
            dl.push(DrawCommand::Triangle {
                p0: Point::new(cx - r * 0.75, cy - r * 0.9),
                p1: Point::new(cx + r * 0.75, cy - r * 0.9),
                p2: Point::new(cx, cy),
                color: fill,
            });
            dl.push(DrawCommand::Circle {
                center: head,
                radius: r,
                color: fill,
            });
            dl.push(DrawCommand::Circle {
                center: head,
                radius: (r * 0.42).max(1.2),
                color: halo,
            });
        }
    }
}

/// Draws a filled triangular arrowhead pointing from `from` into `tip`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Point` - Approach point preceding `tip`.
/// - `Point` - Arrowhead tip coordinate.
/// - `f32` - Arrowhead barb length in logical pixels.
/// - `Color` - Fill color.
pub(crate) fn draw_arrow_head(
    dl: &mut DisplayList,
    from: Point,
    tip: Point,
    size: f32,
    color: Color,
) {
    let dx: f32 = tip.get_x() - from.get_x();
    let dy: f32 = tip.get_y() - from.get_y();
    let len: f32 = (dx * dx + dy * dy).sqrt();
    if !len.is_finite() || len < 1e-3 {
        return;
    }
    let angle: f32 = dy.atan2(dx);
    let left: Point = Point::new(
        tip.get_x() - size * (angle - DRAW_ARROW_ANGLE_RAD).cos(),
        tip.get_y() - size * (angle - DRAW_ARROW_ANGLE_RAD).sin(),
    );
    let right: Point = Point::new(
        tip.get_x() - size * (angle + DRAW_ARROW_ANGLE_RAD).cos(),
        tip.get_y() - size * (angle + DRAW_ARROW_ANGLE_RAD).sin(),
    );
    dl.push(DrawCommand::Triangle {
        p0: tip,
        p1: left,
        p2: right,
        color,
    });
}

/// Emits a single-line text label inside `rect` with explicit alignment, weight, and size.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Bounding text rectangle.
/// - `String` - Text string to render.
/// - `f32` - Font size in logical pixels.
/// - `FontWeight` - Font weight.
/// - `TextAlign` - Horizontal alignment.
/// - `Color` - Ink color.
pub(crate) fn draw_label(
    dl: &mut DisplayList,
    rect: Rect,
    text: String,
    font_size: f32,
    weight: FontWeight,
    align: TextAlign,
    color: Color,
) {
    if text.is_empty() || rect.get_width() <= 1.0 || rect.get_height() <= 1.0 {
        return;
    }
    let mut style: TextStyle = TextStyle::default();
    style
        .set_font_size(font_size)
        .set_weight(weight)
        .set_align(align)
        .set_overflow(TextOverflow::Clip)
        .set_color(color);
    dl.push(DrawCommand::TextRun { rect, text, style });
}

/// Emits a crisp compact pill badge centered at `anchor` for mark-line or mark-point labels.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Point` - Center anchor point.
/// - `&str` - Short badge caption.
/// - `Color` - Pill background color.
pub(crate) fn draw_pill_badge(dl: &mut DisplayList, anchor: Point, text: &str, bg: Color) {
    if text.is_empty() {
        return;
    }
    let est_w: f32 = ((text.chars().count() as f32) * 6.4 + 10.0).clamp(24.0, 128.0);
    let pill_h: f32 = 16.0;
    let rect: Rect = Rect::new(
        anchor.get_x() - est_w * 0.5,
        anchor.get_y() - pill_h * 0.5,
        est_w,
        pill_h,
    );
    dl.push(DrawCommand::FillRoundedRect {
        rect,
        color: bg,
        radius: CornerRadius::all(pill_h * 0.5),
        border: Border::default(),
    });
    draw_label(
        dl,
        Rect::new(rect.get_x() + 2.0, rect.get_y() + 2.0, est_w - 4.0, 12.0),
        text.to_string(),
        9.5,
        FontWeight::SemiBold,
        TextAlign::Center,
        contrast_ink_for(bg),
    );
}
