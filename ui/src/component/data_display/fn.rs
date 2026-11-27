use super::*;

/// Scales normalized 16x16 icon coordinates into logical screen coordinates.
///
/// # Arguments
///
/// - `f32` - Icon box origin X in logical pixels.
/// - `f32` - Icon box origin Y in logical pixels.
/// - `f32` - Unit scale factor (`side / 16.0`).
/// - `f32` - Normalized X coordinate in `[0.0, 16.0]`.
/// - `f32` - Normalized Y coordinate in `[0.0, 16.0]`.
///
/// # Returns
///
/// - `Point` - Scaled logical point.
#[inline(always)]
fn icon_pt(ox: f32, oy: f32, u: f32, px: f32, py: f32) -> Point {
    Point::new(ox + px * u, oy + py * u)
}

/// Appends a single vector stroke line segment into `dl`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Destination vector draw command list.
/// - `Point` - Segment start point.
/// - `Point` - Segment end point.
/// - `Color` - Stroke color.
/// - `f32` - Stroke thickness in logical pixels.
#[inline(always)]
fn icon_line(dl: &mut DisplayList, from: Point, to: Point, color: Color, width: f32) {
    dl.push(DrawCommand::Line {
        from,
        to,
        color,
        width,
    });
}

/// Draws a crisp native vector icon glyph for `name` directly into `dl` inside `bounds` with zero font-rasterization overhead.
///
/// # Arguments
///
/// - `EuvIconName` - Built-in vector icon variant to render.
/// - `Rect` - Target logical bounding box.
/// - `&mut DisplayList` - Destination vector draw command list.
/// - `Color` - Foreground stroke and accent fill color.
pub fn draw_euv_icon(name: EuvIconName, bounds: Rect, dl: &mut DisplayList, color: Color) {
    let w: f32 = bounds.get_width().max(6.0);
    let h: f32 = bounds.get_height().max(6.0);
    let side: f32 = w.min(h);
    let ox: f32 = bounds.get_x() + (w - side) * 0.5;
    let oy: f32 = bounds.get_y() + (h - side) * 0.5;
    let u: f32 = side / 16.0;
    let sw: f32 = (side * 0.11).clamp(1.3, 2.2);
    match name {
        EuvIconName::Copy => {
            dl.push(DrawCommand::FillRoundedRect {
                rect: Rect::new(ox + 5.5 * u, oy + 5.5 * u, 8.0 * u, 8.5 * u),
                color: COLOR_TRANSPARENT,
                radius: CornerRadius::all((1.8 * u).max(1.0)),
                border: Border::new(sw, color),
            });
            icon_line(
                dl,
                icon_pt(ox, oy, u, 10.5, 4.0),
                icon_pt(ox, oy, u, 10.5, 2.5),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 10.5, 2.5),
                icon_pt(ox, oy, u, 3.2, 2.5),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 3.2, 2.5),
                icon_pt(ox, oy, u, 3.2, 10.8),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 3.2, 10.8),
                icon_pt(ox, oy, u, 4.6, 10.8),
                color,
                sw,
            );
        }
        EuvIconName::Check => {
            icon_line(
                dl,
                icon_pt(ox, oy, u, 3.0, 8.4),
                icon_pt(ox, oy, u, 6.6, 12.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 6.6, 12.0),
                icon_pt(ox, oy, u, 13.2, 4.4),
                color,
                sw,
            );
        }
        EuvIconName::Maximize => {
            let x0: f32 = 2.0;
            let y0: f32 = 2.0;
            let x1: f32 = 14.0;
            let y1: f32 = 14.0;
            let arm: f32 = 4.2;
            icon_line(
                dl,
                icon_pt(ox, oy, u, x0, y0 + arm),
                icon_pt(ox, oy, u, x0, y0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, x0, y0),
                icon_pt(ox, oy, u, x0 + arm, y0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, x1 - arm, y0),
                icon_pt(ox, oy, u, x1, y0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, x1, y0),
                icon_pt(ox, oy, u, x1, y0 + arm),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, x0, y1 - arm),
                icon_pt(ox, oy, u, x0, y1),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, x0, y1),
                icon_pt(ox, oy, u, x0 + arm, y1),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, x1 - arm, y1),
                icon_pt(ox, oy, u, x1, y1),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, x1, y1),
                icon_pt(ox, oy, u, x1, y1 - arm),
                color,
                sw,
            );
        }
        EuvIconName::Minimize => {
            let arm: f32 = 3.8;
            icon_line(
                dl,
                icon_pt(ox, oy, u, 2.2, 6.0),
                icon_pt(ox, oy, u, 6.0, 6.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 6.0, 6.0),
                icon_pt(ox, oy, u, 6.0, 2.2),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 10.0, 2.2),
                icon_pt(ox, oy, u, 10.0, 6.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 10.0, 6.0),
                icon_pt(ox, oy, u, 13.8, 6.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 2.2, 10.0),
                icon_pt(ox, oy, u, 6.0, 10.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 6.0, 10.0),
                icon_pt(ox, oy, u, 6.0, 13.8),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 10.0, 13.8),
                icon_pt(ox, oy, u, 10.0, 10.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 10.0, 10.0),
                icon_pt(ox, oy, u, 10.0 + arm, 10.0),
                color,
                sw,
            );
        }
        EuvIconName::Close => {
            icon_line(
                dl,
                icon_pt(ox, oy, u, 3.5, 3.5),
                icon_pt(ox, oy, u, 12.5, 12.5),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 12.5, 3.5),
                icon_pt(ox, oy, u, 3.5, 12.5),
                color,
                sw,
            );
        }
        EuvIconName::Search => {
            let d: f32 = 8.4 * u;
            dl.push(DrawCommand::FillRoundedRect {
                rect: Rect::new(ox + 2.4 * u, oy + 2.4 * u, d, d),
                color: COLOR_TRANSPARENT,
                radius: CornerRadius::all(d * 0.5),
                border: Border::new(sw, color),
            });
            icon_line(
                dl,
                icon_pt(ox, oy, u, 10.0, 10.0),
                icon_pt(ox, oy, u, 13.8, 13.8),
                color,
                sw,
            );
        }
        EuvIconName::Sun => {
            let d: f32 = 5.6 * u;
            dl.push(DrawCommand::FillRoundedRect {
                rect: Rect::new(ox + 5.2 * u, oy + 5.2 * u, d, d),
                color: COLOR_TRANSPARENT,
                radius: CornerRadius::all(d * 0.5),
                border: Border::new(sw, color),
            });
            icon_line(
                dl,
                icon_pt(ox, oy, u, 8.0, 1.6),
                icon_pt(ox, oy, u, 8.0, 3.3),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 8.0, 12.7),
                icon_pt(ox, oy, u, 8.0, 14.4),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 1.6, 8.0),
                icon_pt(ox, oy, u, 3.3, 8.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 12.7, 8.0),
                icon_pt(ox, oy, u, 14.4, 8.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 3.5, 3.5),
                icon_pt(ox, oy, u, 4.8, 4.8),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 11.2, 11.2),
                icon_pt(ox, oy, u, 12.5, 12.5),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 12.5, 3.5),
                icon_pt(ox, oy, u, 11.2, 4.8),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 4.8, 11.2),
                icon_pt(ox, oy, u, 3.5, 12.5),
                color,
                sw,
            );
        }
        EuvIconName::Moon => {
            let pts: [Point; 9] = [
                icon_pt(ox, oy, u, 10.2, 2.2),
                icon_pt(ox, oy, u, 6.6, 3.2),
                icon_pt(ox, oy, u, 3.8, 5.8),
                icon_pt(ox, oy, u, 3.4, 9.6),
                icon_pt(ox, oy, u, 5.6, 12.8),
                icon_pt(ox, oy, u, 9.4, 13.8),
                icon_pt(ox, oy, u, 12.8, 12.0),
                icon_pt(ox, oy, u, 9.0, 10.6),
                icon_pt(ox, oy, u, 7.8, 6.8),
            ];
            for idx in 0..8 {
                icon_line(dl, pts[idx], pts[idx + 1], color, sw);
            }
            icon_line(dl, pts[8], pts[0], color, sw);
        }
        EuvIconName::Palette => {
            let d: f32 = 11.6 * u;
            dl.push(DrawCommand::FillRoundedRect {
                rect: Rect::new(ox + 2.2 * u, oy + 2.2 * u, d, d),
                color: COLOR_TRANSPARENT,
                radius: CornerRadius::all(d * 0.5),
                border: Border::new(sw, color),
            });
            let dot_r: f32 = (1.35 * u).max(1.2);
            dl.push(DrawCommand::Circle {
                center: icon_pt(ox, oy, u, 6.0, 6.2),
                radius: dot_r,
                color,
            });
            dl.push(DrawCommand::Circle {
                center: icon_pt(ox, oy, u, 10.0, 6.2),
                radius: dot_r,
                color,
            });
            dl.push(DrawCommand::Circle {
                center: icon_pt(ox, oy, u, 8.0, 10.2),
                radius: dot_r,
                color,
            });
        }
        EuvIconName::Menu => {
            icon_line(
                dl,
                icon_pt(ox, oy, u, 2.5, 4.2),
                icon_pt(ox, oy, u, 13.5, 4.2),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 2.5, 8.0),
                icon_pt(ox, oy, u, 13.5, 8.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 2.5, 11.8),
                icon_pt(ox, oy, u, 13.5, 11.8),
                color,
                sw,
            );
        }
        EuvIconName::Play => {
            icon_line(
                dl,
                icon_pt(ox, oy, u, 4.6, 3.0),
                icon_pt(ox, oy, u, 12.6, 8.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 12.6, 8.0),
                icon_pt(ox, oy, u, 4.6, 13.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 4.6, 13.0),
                icon_pt(ox, oy, u, 4.6, 3.0),
                color,
                sw,
            );
        }
        EuvIconName::Pause => {
            let bar_w: f32 = (2.6 * u).max(2.0);
            let bar_h: f32 = 9.6 * u;
            dl.push(DrawCommand::FillRoundedRect {
                rect: Rect::new(ox + 4.0 * u, oy + 3.2 * u, bar_w, bar_h),
                color,
                radius: CornerRadius::all(1.0),
                border: Border::default(),
            });
            dl.push(DrawCommand::FillRoundedRect {
                rect: Rect::new(ox + 9.4 * u, oy + 3.2 * u, bar_w, bar_h),
                color,
                radius: CornerRadius::all(1.0),
                border: Border::default(),
            });
        }
        EuvIconName::Refresh => {
            let arc: [Point; 6] = [
                icon_pt(ox, oy, u, 12.8, 6.0),
                icon_pt(ox, oy, u, 10.6, 3.2),
                icon_pt(ox, oy, u, 6.4, 3.0),
                icon_pt(ox, oy, u, 3.4, 5.8),
                icon_pt(ox, oy, u, 3.6, 10.2),
                icon_pt(ox, oy, u, 7.2, 13.0),
            ];
            for idx in 0..5 {
                icon_line(dl, arc[idx], arc[idx + 1], color, sw);
            }
            icon_line(
                dl,
                icon_pt(ox, oy, u, 7.2, 13.0),
                icon_pt(ox, oy, u, 11.6, 11.8),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 12.8, 3.2),
                icon_pt(ox, oy, u, 12.8, 6.4),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 9.6, 6.4),
                icon_pt(ox, oy, u, 12.8, 6.4),
                color,
                sw,
            );
        }
        EuvIconName::Trash => {
            icon_line(
                dl,
                icon_pt(ox, oy, u, 2.8, 4.4),
                icon_pt(ox, oy, u, 13.2, 4.4),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 6.0, 2.6),
                icon_pt(ox, oy, u, 10.0, 2.6),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 4.4, 4.4),
                icon_pt(ox, oy, u, 5.0, 13.2),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 5.0, 13.2),
                icon_pt(ox, oy, u, 11.0, 13.2),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 11.0, 13.2),
                icon_pt(ox, oy, u, 11.6, 4.4),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 6.8, 6.8),
                icon_pt(ox, oy, u, 6.8, 11.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 9.2, 6.8),
                icon_pt(ox, oy, u, 9.2, 11.0),
                color,
                sw,
            );
        }
        EuvIconName::ChevronDown => {
            icon_line(
                dl,
                icon_pt(ox, oy, u, 3.6, 5.8),
                icon_pt(ox, oy, u, 8.0, 10.6),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 8.0, 10.6),
                icon_pt(ox, oy, u, 12.4, 5.8),
                color,
                sw,
            );
        }
        EuvIconName::ArrowLeft => {
            icon_line(
                dl,
                icon_pt(ox, oy, u, 12.6, 8.0),
                icon_pt(ox, oy, u, 3.6, 8.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 7.4, 4.2),
                icon_pt(ox, oy, u, 3.6, 8.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 3.6, 8.0),
                icon_pt(ox, oy, u, 7.4, 11.8),
                color,
                sw,
            );
        }
        EuvIconName::ArrowRight => {
            icon_line(
                dl,
                icon_pt(ox, oy, u, 3.4, 8.0),
                icon_pt(ox, oy, u, 12.4, 8.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 8.6, 4.2),
                icon_pt(ox, oy, u, 12.4, 8.0),
                color,
                sw,
            );
            icon_line(
                dl,
                icon_pt(ox, oy, u, 12.4, 8.0),
                icon_pt(ox, oy, u, 8.6, 11.8),
                color,
                sw,
            );
        }
    }
}
