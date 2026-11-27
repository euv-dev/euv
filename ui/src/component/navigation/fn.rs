use super::*;

/// Draws a crisp anti-aliased vector icon for a floating `EuvBottomNav` tab item.
///
/// # Arguments
///
/// - `Rect` - Logical bounds of the icon canvas box.
/// - `&mut DisplayList` - Active draw command list.
/// - `&str` - Tab route path used for icon selection.
/// - `&str` - Tab display label used for icon selection.
/// - `Color` - Foreground stroke and fill color.
pub(crate) fn draw_bottom_nav_icon(
    bounds: Rect,
    dl: &mut DisplayList,
    path: &str,
    label: &str,
    color: Color,
) {
    let cx: f32 = bounds.get_x() + bounds.get_width() * 0.5;
    let cy: f32 = bounds.get_y() + bounds.get_height() * 0.5;
    let stroke: f32 = 1.65;
    let p_low: String = path.to_ascii_lowercase();
    let l_low: String = label.to_ascii_lowercase();
    if p_low == "/"
        || p_low.contains(NAV_ICON_HOME)
        || p_low.contains(NAV_ICON_OVERVIEW)
        || l_low.contains(NAV_ICON_HOME)
    {
        let top: Point = Point::new(cx, cy - 6.5);
        let roof_l: Point = Point::new(cx - 7.0, cy - 0.5);
        let roof_r: Point = Point::new(cx + 7.0, cy - 0.5);
        let wall_tl: Point = Point::new(cx - 5.0, cy - 1.5);
        let wall_bl: Point = Point::new(cx - 5.0, cy + 6.0);
        let wall_br: Point = Point::new(cx + 5.0, cy + 6.0);
        let wall_tr: Point = Point::new(cx + 5.0, cy - 1.5);
        dl.push(DrawCommand::Line {
            from: roof_l,
            to: top,
            color,
            width: stroke,
        });
        dl.push(DrawCommand::Line {
            from: top,
            to: roof_r,
            color,
            width: stroke,
        });
        dl.push(DrawCommand::Line {
            from: wall_tl,
            to: wall_bl,
            color,
            width: stroke,
        });
        dl.push(DrawCommand::Line {
            from: wall_bl,
            to: wall_br,
            color,
            width: stroke,
        });
        dl.push(DrawCommand::Line {
            from: wall_br,
            to: wall_tr,
            color,
            width: stroke,
        });
        dl.push(DrawCommand::FillRoundedRect {
            rect: Rect::new(cx - 1.6, cy + 1.5, 3.2, 4.5),
            color,
            radius: CornerRadius::new(1.0, 1.0, 0.0, 0.0),
            border: Border::default(),
        });
        return;
    }
    if p_low.contains(NAV_ICON_MARKDOWN)
        || p_low.contains(NAV_ICON_DOCS)
        || l_low.contains(NAV_ICON_DOCS)
    {
        dl.push(DrawCommand::FillRoundedRect {
            rect: Rect::new(cx - 5.5, cy - 6.5, 11.0, 13.0),
            color: COLOR_TRANSPARENT,
            radius: CornerRadius::all(2.4),
            border: Border::new(1.5, color),
        });
        dl.push(DrawCommand::Line {
            from: Point::new(cx - 2.8, cy - 2.2),
            to: Point::new(cx + 2.8, cy - 2.2),
            color,
            width: 1.4,
        });
        dl.push(DrawCommand::Line {
            from: Point::new(cx - 2.8, cy + 0.8),
            to: Point::new(cx + 2.8, cy + 0.8),
            color,
            width: 1.4,
        });
        dl.push(DrawCommand::Line {
            from: Point::new(cx - 2.8, cy + 3.6),
            to: Point::new(cx + 0.8, cy + 3.6),
            color,
            width: 1.4,
        });
        return;
    }
    if p_low.contains(NAV_ICON_CASES)
        || p_low.contains(NAV_ICON_FORMS)
        || p_low.ends_with(NAV_ICON_LAB_SUFFIX)
        || l_low.contains(NAV_ICON_CASES)
    {
        let sz: f32 = 5.2;
        let gap: f32 = 1.6;
        let x0: f32 = cx - sz - gap * 0.5;
        let x1: f32 = cx + gap * 0.5;
        let y0: f32 = cy - sz - gap * 0.5;
        let y1: f32 = cy + gap * 0.5;
        for (rx, ry) in [(x0, y0), (x1, y0), (x0, y1), (x1, y1)].into_iter() {
            dl.push(DrawCommand::FillRoundedRect {
                rect: Rect::new(rx, ry, sz, sz),
                color: COLOR_TRANSPARENT,
                radius: CornerRadius::all(1.6),
                border: Border::new(1.45, color),
            });
        }
        return;
    }
    if p_low.contains(NAV_ICON_STRESS)
        || p_low.contains(NAV_ICON_PERF)
        || l_low.contains(NAV_ICON_STRESS)
    {
        let p0: Point = Point::new(cx + 1.5, cy - 6.5);
        let p1: Point = Point::new(cx - 4.8, cy + 0.6);
        let p2: Point = Point::new(cx + 0.4, cy + 0.6);
        let p3: Point = Point::new(cx - 1.5, cy + 6.5);
        let p4: Point = Point::new(cx + 4.8, cy - 0.8);
        let p5: Point = Point::new(cx - 0.2, cy - 0.8);
        dl.push(DrawCommand::Line {
            from: p0,
            to: p1,
            color,
            width: stroke,
        });
        dl.push(DrawCommand::Line {
            from: p1,
            to: p2,
            color,
            width: stroke,
        });
        dl.push(DrawCommand::Line {
            from: p2,
            to: p3,
            color,
            width: stroke,
        });
        dl.push(DrawCommand::Line {
            from: p3,
            to: p4,
            color,
            width: stroke,
        });
        dl.push(DrawCommand::Line {
            from: p4,
            to: p5,
            color,
            width: stroke,
        });
        return;
    }
    if p_low.contains(NAV_ICON_GAME) || p_low.contains("3d") || l_low.contains("3d") {
        let top: Point = Point::new(cx, cy - 6.6);
        let mid: Point = Point::new(cx, cy + 0.2);
        let bot: Point = Point::new(cx, cy + 6.8);
        let ul: Point = Point::new(cx - 6.2, cy - 3.2);
        let ur: Point = Point::new(cx + 6.2, cy - 3.2);
        let bl: Point = Point::new(cx - 6.2, cy + 3.4);
        let br: Point = Point::new(cx + 6.2, cy + 3.4);
        for (a, b) in [
            (top, ur),
            (ur, br),
            (br, bot),
            (bot, bl),
            (bl, ul),
            (ul, top),
            (ul, mid),
            (ur, mid),
            (mid, bot),
        ]
        .into_iter()
        {
            dl.push(DrawCommand::Line {
                from: a,
                to: b,
                color,
                width: 1.45,
            });
        }
        return;
    }
    let top: Point = Point::new(cx, cy - 6.2);
    let right: Point = Point::new(cx + 6.2, cy);
    let bot: Point = Point::new(cx, cy + 6.2);
    let left: Point = Point::new(cx - 6.2, cy);
    for (a, b) in [(top, right), (right, bot), (bot, left), (left, top)].into_iter() {
        dl.push(DrawCommand::Line {
            from: a,
            to: b,
            color,
            width: stroke,
        });
    }
    dl.push(DrawCommand::Circle {
        center: Point::new(cx, cy),
        radius: 1.8,
        color,
    });
}
