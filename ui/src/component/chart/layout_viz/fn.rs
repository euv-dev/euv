use super::*;

/// Renders a squarified area-proportional hierarchical `Treemap` series with group headers and contrast-safe labels.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartTreemapSeries` - Treemap descriptor.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_treemap_series(
    dl: &mut DisplayList,
    plot: Rect,
    meta: SeriesMeta<'_>,
    tm: &EuvChartTreemapSeries,
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    if tm.get_nodes().is_empty() {
        return;
    }
    let mut walk: TreemapWalk<'_> = TreemapWalk {
        show_headers: tm.get_show_headers(),
        cell_gap: tm.get_cell_gap().clamp(0.0, 8.0),
        corner_radius: tm.get_corner_radius().clamp(0.0, 12.0),
        meta,
        hit_idx: 0,
        hits,
    };
    render_treemap_level(dl, &mut walk, tm.get_nodes(), plot, frame.palette, None);
}

/// Recursively lays out one level of treemap siblings using a squarified slice-and-dice partition.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `&mut TreemapWalk<'_>` - Shared traversal state.
/// - `&[EuvChartHierarchyNode]` - Sibling nodes at this depth.
/// - `Rect` - Bounding rectangle for this sibling group.
/// - `&[Color]` - Categorical palette.
/// - `Option<Color>` - Optional inherited ancestor color.
fn render_treemap_level(
    dl: &mut DisplayList,
    walk: &mut TreemapWalk<'_>,
    nodes: &[EuvChartHierarchyNode],
    bounds: Rect,
    palette: &[Color],
    inherited: Option<Color>,
) {
    if nodes.is_empty() || bounds.get_width() < 4.0 || bounds.get_height() < 4.0 {
        return;
    }
    let mut indexed: Vec<(usize, f32)> = nodes
        .iter()
        .enumerate()
        .map(|(i, n): (usize, &EuvChartHierarchyNode)| (i, n.total_value().max(0.0)))
        .filter(|&(_, w): &(usize, f32)| w > 0.0)
        .collect();
    if indexed.is_empty() {
        return;
    }
    indexed.sort_by(|a: &(usize, f32), b: &(usize, f32)| {
        b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal)
    });
    let rects: Vec<(usize, Rect)> = squarify_rects(&indexed, bounds);
    let pal_len: usize = palette.len().max(1);
    let gap: f32 = walk.cell_gap;
    for (orig_i, raw_rect) in rects.into_iter() {
        let node: &EuvChartHierarchyNode = &nodes[orig_i];
        let inset: f32 = (gap * 0.5)
            .min(raw_rect.get_width() * 0.2)
            .min(raw_rect.get_height() * 0.2);
        let rect: Rect = Rect::new(
            raw_rect.get_x() + inset,
            raw_rect.get_y() + inset,
            (raw_rect.get_width() - inset * 2.0).max(2.0),
            (raw_rect.get_height() - inset * 2.0).max(2.0),
        );
        let base_col: Color = node.color_opt().unwrap_or_else(|| {
            if let Some(anc) = inherited {
                let tint: f32 = (((orig_i % 4) as f32) * 0.08).clamp(0.0, 0.28);
                anc.lerp(Color::hex(0xFFFFFF), tint)
            } else {
                palette[orig_i % pal_len]
            }
        });
        let has_children: bool = !node.get_children().is_empty();
        dl.push(DrawCommand::FillRoundedRect {
            rect,
            color: if has_children {
                base_col.multiply_alpha(0.28)
            } else {
                base_col
            },
            radius: CornerRadius::all(walk.corner_radius),
            border: Border::new(1.0, base_col.multiply_alpha(0.8)),
        });
        let val: f32 = node.total_value();
        if has_children {
            let header_h: f32 = if walk.show_headers && rect.get_height() >= 30.0 {
                LAYOUT_TREEMAP_HEADER_H
            } else {
                0.0
            };
            if header_h > 0.0 {
                dl.push(DrawCommand::FillRoundedRect {
                    rect: Rect::new(rect.get_x(), rect.get_y(), rect.get_width(), header_h),
                    color: base_col,
                    radius: CornerRadius::new(walk.corner_radius, walk.corner_radius, 0.0, 0.0),
                    border: Border::default(),
                });
                if rect.get_width() >= 28.0 {
                    draw_label(
                        dl,
                        Rect::new(
                            rect.get_x() + 4.0,
                            rect.get_y() + 2.0,
                            rect.get_width() - 8.0,
                            12.0,
                        ),
                        node.get_name().clone(),
                        9.5,
                        FontWeight::SemiBold,
                        TextAlign::Left,
                        contrast_ink_for(base_col),
                    );
                }
            }
            let inner: Rect = Rect::new(
                rect.get_x() + 2.0,
                rect.get_y() + header_h + 2.0,
                (rect.get_width() - 4.0).max(1.0),
                (rect.get_height() - header_h - 4.0).max(1.0),
            );
            render_treemap_level(
                dl,
                walk,
                node.get_children(),
                inner,
                palette,
                Some(base_col),
            );
        } else {
            let ink: Color = contrast_ink_for(base_col);
            if rect.get_width() >= 34.0 && rect.get_height() >= 18.0 {
                draw_label(
                    dl,
                    Rect::new(
                        rect.get_x() + 4.0,
                        rect.get_y() + 4.0,
                        rect.get_width() - 8.0,
                        13.0,
                    ),
                    node.get_name().clone(),
                    10.0,
                    FontWeight::SemiBold,
                    TextAlign::Left,
                    ink,
                );
                if rect.get_height() >= 34.0 {
                    draw_label(
                        dl,
                        Rect::new(
                            rect.get_x() + 4.0,
                            rect.get_y() + 18.0,
                            rect.get_width() - 8.0,
                            12.0,
                        ),
                        format_chart_number(val, ""),
                        9.0,
                        FontWeight::Normal,
                        TextAlign::Left,
                        ink.multiply_alpha(0.85),
                    );
                }
            }
            let idx: usize = walk.hit_idx;
            walk.hit_idx += 1;
            walk.hits.push(EuvChartDatumHit::rect_hit(
                (walk.meta.index, walk.meta.name.to_string(), idx),
                (node.get_name().clone(), val, format_chart_number(val, "")),
                base_col,
                Point::new(
                    rect.get_x() + rect.get_width() * 0.5,
                    rect.get_y() + rect.get_height() * 0.5,
                ),
                rect,
            ));
        }
    }
}

/// Computes squarified area-proportional rectangles for sorted `(index, weight)` items inside `bounds`.
///
/// # Arguments
///
/// - `&[(usize, f32)]` - Descending weight-sorted `(index, positive_weight)` pairs.
/// - `Rect` - Enclosing target rectangle.
///
/// # Returns
///
/// - `Vec<(usize, Rect)>` - Assigned rectangle per item index.
#[must_use]
fn squarify_rects(items: &[(usize, f32)], bounds: Rect) -> Vec<(usize, Rect)> {
    let mut out: Vec<(usize, Rect)> = Vec::with_capacity(items.len());
    let mut rem: Rect = bounds;
    let mut rem_weight: f32 = items.iter().map(|&(_, w): &(usize, f32)| w).sum();
    let mut cursor: usize = 0;
    while cursor < items.len()
        && rem_weight > 1e-6
        && rem.get_width() > 0.5
        && rem.get_height() > 0.5
    {
        let short_side: f32 = rem.get_width().min(rem.get_height()).max(1.0);
        let rem_area: f32 = rem.get_width() * rem.get_height();
        let mut row_end: usize = cursor + 1;
        let mut row_weight: f32 = items[cursor].1;
        let mut best_worst: f32 = worst_aspect(
            items[cursor..row_end]
                .iter()
                .map(|&(_, w): &(usize, f32)| w * rem_area / rem_weight),
            short_side,
        );
        while row_end < items.len() {
            let next_w: f32 = row_weight + items[row_end].1;
            let cand_worst: f32 = worst_aspect(
                items[cursor..=row_end]
                    .iter()
                    .map(|&(_, w): &(usize, f32)| w * rem_area / rem_weight),
                short_side,
            );
            if cand_worst <= best_worst {
                best_worst = cand_worst;
                row_weight = next_w;
                row_end += 1;
            } else {
                break;
            }
        }
        let frac: f32 = (row_weight / rem_weight).clamp(0.0, 1.0);
        let vertical_strip: bool = rem.get_width() >= rem.get_height();
        if vertical_strip {
            let strip_w: f32 = rem.get_width() * frac;
            let mut y_cur: f32 = rem.get_y();
            for &(idx, w) in items[cursor..row_end].iter() {
                let h: f32 = rem.get_height() * (w / row_weight.max(1e-6));
                out.push((idx, Rect::new(rem.get_x(), y_cur, strip_w, h)));
                y_cur += h;
            }
            rem = Rect::new(
                rem.get_x() + strip_w,
                rem.get_y(),
                (rem.get_width() - strip_w).max(0.0),
                rem.get_height(),
            );
        } else {
            let strip_h: f32 = rem.get_height() * frac;
            let mut x_cur: f32 = rem.get_x();
            for &(idx, w) in items[cursor..row_end].iter() {
                let w_px: f32 = rem.get_width() * (w / row_weight.max(1e-6));
                out.push((idx, Rect::new(x_cur, rem.get_y(), w_px, strip_h)));
                x_cur += w_px;
            }
            rem = Rect::new(
                rem.get_x(),
                rem.get_y() + strip_h,
                rem.get_width(),
                (rem.get_height() - strip_h).max(0.0),
            );
        }
        rem_weight = (rem_weight - row_weight).max(0.0);
        cursor = row_end;
    }
    out
}

/// Evaluates the worst rectangle aspect ratio for a candidate squarified treemap strip.
///
/// # Arguments
///
/// - `I` - Iterator yielding scaled tile areas inside the strip.
/// - `f32` - Length of the strip edge.
///
/// # Returns
///
/// - `f32` - Maximum aspect ratio (`>= 1.0`).
#[must_use]
fn worst_aspect<I>(areas: I, side: f32) -> f32
where
    I: Iterator<Item = f32>,
{
    let mut sum: f32 = 0.0;
    let mut min_a: f32 = f32::INFINITY;
    let mut max_a: f32 = 0.0;
    for a in areas {
        let safe: f32 = a.max(1e-4);
        sum += safe;
        min_a = min_a.min(safe);
        max_a = max_a.max(safe);
    }
    if sum <= 1e-4 || !min_a.is_finite() {
        return f32::INFINITY;
    }
    let s2: f32 = side * side;
    let sum2: f32 = sum * sum;
    ((s2 * max_a) / sum2).max(sum2 / (s2 * min_a))
}

/// Renders a layered topological `Sankey` flow diagram with smooth cubic Bézier ribbons.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartSankeySeries` - Sankey descriptor.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_sankey_series(
    dl: &mut DisplayList,
    plot: Rect,
    meta: SeriesMeta<'_>,
    sankey: &EuvChartSankeySeries,
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let nodes: &[EuvChartSankeyNode] = sankey.get_nodes().as_slice();
    if nodes.is_empty() {
        return;
    }
    let mut name_to_idx: HashMap<&str, usize> = HashMap::new();
    for (i, n) in nodes.iter().enumerate() {
        let _: Option<usize> = name_to_idx.insert(n.get_name().as_str(), i);
    }
    let n_count: usize = nodes.len();
    let mut depths: Vec<usize> = vec![0; n_count];
    for (i, n) in nodes.iter().enumerate() {
        if let Some(d) = n.depth_opt() {
            depths[i] = d;
        }
    }
    for _ in 0..n_count {
        let mut changed: bool = false;
        for link in sankey.get_links().iter() {
            if let (Some(&s_i), Some(&t_i)) = (
                name_to_idx.get(link.get_source().as_str()),
                name_to_idx.get(link.get_target().as_str()),
            ) && nodes[t_i].depth_opt().is_none()
                && depths[t_i] <= depths[s_i]
            {
                depths[t_i] = (depths[s_i] + 1).min(n_count);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let mut in_flow: Vec<f32> = vec![0.0; n_count];
    let mut out_flow: Vec<f32> = vec![0.0; n_count];
    for link in sankey.get_links().iter() {
        let v: f32 = link.get_value().max(0.0);
        if let (Some(&s_i), Some(&t_i)) = (
            name_to_idx.get(link.get_source().as_str()),
            name_to_idx.get(link.get_target().as_str()),
        ) {
            out_flow[s_i] += v;
            in_flow[t_i] += v;
        }
    }
    let max_depth: usize = depths.iter().copied().max().unwrap_or(0);
    let col_count: usize = max_depth + 1;
    let node_w: f32 = sankey.get_node_width().clamp(6.0, 28.0);
    let node_gap: f32 = sankey.get_node_gap().clamp(3.0, 24.0);
    let inner_x0: f32 = plot.get_x() + 6.0;
    let usable_w: f32 = (plot.get_width() - 12.0 - node_w).max(20.0);
    let pal_len: usize = frame.palette.len().max(1);
    let mut boxes: Vec<SankeyNodeBox> = Vec::with_capacity(n_count);
    for i in 0..n_count {
        boxes.push(SankeyNodeBox {
            depth: depths[i],
            rect: Rect::default(),
            color: nodes[i].color_opt().unwrap_or(frame.palette[i % pal_len]),
            flow: in_flow[i].max(out_flow[i]).max(1.0),
            out_cursor_y: 0.0,
            in_cursor_y: 0.0,
        });
    }
    let mut max_col_flow: f32 = 1.0;
    for col in 0..col_count {
        let sum: f32 = boxes
            .iter()
            .filter(|b: &&SankeyNodeBox| b.depth == col)
            .map(|b: &SankeyNodeBox| b.flow)
            .sum();
        max_col_flow = max_col_flow.max(sum);
    }
    for col in 0..col_count {
        let col_indices: Vec<usize> = (0..n_count)
            .filter(|&i: &usize| boxes[i].depth == col)
            .collect();
        if col_indices.is_empty() {
            continue;
        }
        let col_x: f32 = if col_count == 1 {
            inner_x0
        } else {
            inner_x0 + usable_w * ((col as f32) / ((col_count - 1) as f32))
        };
        let total_gaps: f32 = node_gap * ((col_indices.len().saturating_sub(1)) as f32);
        let avail_h: f32 = (plot.get_height() - total_gaps - 8.0).max(20.0);
        let col_sum: f32 = col_indices.iter().map(|&i: &usize| boxes[i].flow).sum();
        let scale: f32 = avail_h / max_col_flow.max(col_sum);
        let used_h: f32 = col_sum * scale + total_gaps;
        let mut y_cur: f32 = plot.get_y() + ((plot.get_height() - used_h) * 0.5).max(4.0);
        for &idx in col_indices.iter() {
            let h: f32 = (boxes[idx].flow * scale).max(6.0);
            boxes[idx].rect = Rect::new(col_x, y_cur, node_w, h);
            boxes[idx].out_cursor_y = y_cur;
            boxes[idx].in_cursor_y = y_cur;
            y_cur += h + node_gap;
        }
    }
    let ribbon_alpha: f32 = sankey.get_ribbon_opacity().clamp(0.1, 0.85);
    for link in sankey.get_links().iter() {
        let Some(&s_i) = name_to_idx.get(link.get_source().as_str()) else {
            continue;
        };
        let Some(&t_i) = name_to_idx.get(link.get_target().as_str()) else {
            continue;
        };
        let v: f32 = link.get_value().max(0.0);
        if v <= 1e-6 {
            continue;
        }
        let s_rect: Rect = boxes[s_i].rect;
        let t_rect: Rect = boxes[t_i].rect;
        let sh: f32 = (s_rect.get_height() * (v / boxes[s_i].flow.max(1e-5))).max(1.5);
        let th: f32 = (t_rect.get_height() * (v / boxes[t_i].flow.max(1e-5))).max(1.5);
        let sy0: f32 = boxes[s_i].out_cursor_y;
        let sy1: f32 = (sy0 + sh).min(s_rect.get_y() + s_rect.get_height());
        boxes[s_i].out_cursor_y = sy1;
        let ty0: f32 = boxes[t_i].in_cursor_y;
        let ty1: f32 = (ty0 + th).min(t_rect.get_y() + t_rect.get_height());
        boxes[t_i].in_cursor_y = ty1;
        let sx: f32 = s_rect.get_x() + s_rect.get_width();
        let tx: f32 = t_rect.get_x();
        let mid_x: f32 = (sx + tx) * 0.5;
        let top_pts: Vec<Point> = sample_cubic_bezier(
            Point::new(sx, sy0),
            Point::new(mid_x, sy0),
            Point::new(mid_x, ty0),
            Point::new(tx, ty0),
            DRAW_BEZIER_SEGMENTS,
        );
        let bot_pts: Vec<Point> = sample_cubic_bezier(
            Point::new(sx, sy1),
            Point::new(mid_x, sy1),
            Point::new(mid_x, ty1),
            Point::new(tx, ty1),
            DRAW_BEZIER_SEGMENTS,
        );
        let col: Color = boxes[s_i].color.lerp(boxes[t_i].color, 0.35);
        draw_ribbon_fill(dl, &top_pts, &bot_pts, col.multiply_alpha(ribbon_alpha));
    }
    let label_col: Color = frame.tokens.get_palette().get_foreground();
    for (i, b) in boxes.iter().enumerate() {
        dl.push(DrawCommand::FillRoundedRect {
            rect: b.rect,
            color: b.color,
            radius: CornerRadius::all(2.0),
            border: Border::default(),
        });
        let right_label: bool = b.depth < max_depth;
        let (lx, align): (f32, TextAlign) = if right_label {
            (b.rect.get_x() + b.rect.get_width() + 5.0, TextAlign::Left)
        } else {
            ((b.rect.get_x() - 78.0).max(plot.get_x()), TextAlign::Right)
        };
        draw_label(
            dl,
            Rect::new(
                lx,
                b.rect.get_y() + (b.rect.get_height() - 12.0) * 0.5,
                74.0,
                12.0,
            ),
            nodes[i].get_name().clone(),
            9.5,
            FontWeight::Medium,
            align,
            label_col,
        );
        hits.push(EuvChartDatumHit::rect_hit(
            (meta.index, meta.name.to_string(), i),
            (
                nodes[i].get_name().clone(),
                b.flow,
                format_chart_number(b.flow, ""),
            ),
            b.color,
            Point::new(
                b.rect.get_x() + b.rect.get_width() * 0.5,
                b.rect.get_y() + b.rect.get_height() * 0.5,
            ),
            b.rect,
        ));
    }
}

/// Renders a relational `Graph` network with deterministic Fruchterman-Reingold force-directed or circular layout.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartGraphSeries` - Graph descriptor.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_graph_series(
    dl: &mut DisplayList,
    plot: Rect,
    meta: SeriesMeta<'_>,
    graph: &EuvChartGraphSeries,
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let nodes: &[EuvChartGraphNode] = graph.get_nodes().as_slice();
    if nodes.is_empty() {
        return;
    }
    let n: usize = nodes.len();
    let mut id_map: HashMap<&str, usize> = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        let _: Option<usize> = id_map.insert(node.get_id().as_str(), i);
    }
    let pad: f32 = 28.0;
    let inner: Rect = Rect::new(
        plot.get_x() + pad,
        plot.get_y() + pad,
        (plot.get_width() - pad * 2.0).max(40.0),
        (plot.get_height() - pad * 2.0).max(40.0),
    );
    let mut positions: Vec<(f32, f32)> = Vec::with_capacity(n);
    match graph.get_layout() {
        EuvChartGraphLayout::Circular => {
            let cx: f32 = inner.get_x() + inner.get_width() * 0.5;
            let cy: f32 = inner.get_y() + inner.get_height() * 0.5;
            let rad: f32 = inner.get_width().min(inner.get_height()) * 0.44;
            for i in 0..n {
                let ang: f32 = -FRAC_PI_2 + TAU * ((i as f32) / (n as f32));
                positions.push((cx + rad * ang.cos(), cy + rad * ang.sin()));
            }
        }
        EuvChartGraphLayout::None => {
            for (i, node) in nodes.iter().enumerate() {
                let (ox, oy): (Option<f32>, Option<f32>) = node.pos_opt();
                let nx: f32 = ox.unwrap_or(((i + 1) as f32) / ((n + 1) as f32));
                let ny: f32 = oy.unwrap_or(0.5);
                positions.push((
                    inner.get_x() + nx.clamp(0.0, 1.0) * inner.get_width(),
                    inner.get_y() + ny.clamp(0.0, 1.0) * inner.get_height(),
                ));
            }
        }
        EuvChartGraphLayout::Force => {
            for (i, node) in nodes.iter().enumerate() {
                let (ox, oy): (Option<f32>, Option<f32>) = node.pos_opt();
                if let (Some(px), Some(py)) = (ox, oy) {
                    positions.push((px, py));
                } else {
                    let r: f32 = (((i + 1) as f32) / ((n + 1) as f32)).sqrt() * 0.42;
                    let ang: f32 = (i as f32) * LAYOUT_GOLDEN_ANGLE;
                    positions.push((0.5 + r * ang.cos(), 0.5 + r * ang.sin()));
                }
            }
            let k: f32 = (1.0 / (n as f32)).sqrt().max(0.08);
            let iters: usize = graph.get_iterations().clamp(8, 120);
            for iter in 0..iters {
                let mut disp: Vec<(f32, f32)> = vec![(0.0, 0.0); n];
                for i in 0..n {
                    for j in (i + 1)..n {
                        let dx: f32 = positions[i].0 - positions[j].0;
                        let dy: f32 = positions[i].1 - positions[j].1;
                        let dist: f32 = (dx * dx + dy * dy).sqrt().max(0.01);
                        let force: f32 = (k * k) / dist;
                        let fx: f32 = (dx / dist) * force;
                        let fy: f32 = (dy / dist) * force;
                        disp[i].0 += fx;
                        disp[i].1 += fy;
                        disp[j].0 -= fx;
                        disp[j].1 -= fy;
                    }
                }
                for edge in graph.get_edges().iter() {
                    if let (Some(&s_i), Some(&t_i)) = (
                        id_map.get(edge.get_source().as_str()),
                        id_map.get(edge.get_target().as_str()),
                    ) && s_i != t_i
                    {
                        let dx: f32 = positions[s_i].0 - positions[t_i].0;
                        let dy: f32 = positions[s_i].1 - positions[t_i].1;
                        let dist: f32 = (dx * dx + dy * dy).sqrt().max(0.01);
                        let force: f32 = (dist * dist) / k;
                        let fx: f32 = (dx / dist) * force;
                        let fy: f32 = (dy / dist) * force;
                        disp[s_i].0 -= fx;
                        disp[s_i].1 -= fy;
                        disp[t_i].0 += fx;
                        disp[t_i].1 += fy;
                    }
                }
                let temp: f32 = 0.14 * (1.0 - (iter as f32) / (iters as f32));
                for i in 0..n {
                    let dl_len: f32 = (disp[i].0 * disp[i].0 + disp[i].1 * disp[i].1)
                        .sqrt()
                        .max(1e-5);
                    let step: f32 = dl_len.min(temp);
                    positions[i].0 += (disp[i].0 / dl_len) * step;
                    positions[i].1 += (disp[i].1 / dl_len) * step;
                }
            }
            let mut min_x: f32 = f32::INFINITY;
            let mut max_x: f32 = f32::NEG_INFINITY;
            let mut min_y: f32 = f32::INFINITY;
            let mut max_y: f32 = f32::NEG_INFINITY;
            for &(px, py) in positions.iter() {
                min_x = min_x.min(px);
                max_x = max_x.max(px);
                min_y = min_y.min(py);
                max_y = max_y.max(py);
            }
            let span_x: f32 = (max_x - min_x).max(1e-4);
            let span_y: f32 = (max_y - min_y).max(1e-4);
            for pos in positions.iter_mut() {
                pos.0 = inner.get_x() + ((pos.0 - min_x) / span_x) * inner.get_width();
                pos.1 = inner.get_y() + ((pos.1 - min_y) / span_y) * inner.get_height();
            }
        }
    }
    let edge_col: Color = frame
        .tokens
        .get_palette()
        .get_muted_foreground()
        .multiply_alpha(0.55);
    for edge in graph.get_edges().iter() {
        let Some(&s_i) = id_map.get(edge.get_source().as_str()) else {
            continue;
        };
        let Some(&t_i) = id_map.get(edge.get_target().as_str()) else {
            continue;
        };
        let p0: Point = Point::new(positions[s_i].0, positions[s_i].1);
        let p2: Point = Point::new(positions[t_i].0, positions[t_i].1);
        let w: f32 = edge.get_weight().clamp(1.0, 4.0);
        let dx: f32 = p2.get_x() - p0.get_x();
        let dy: f32 = p2.get_y() - p0.get_y();
        let dist: f32 = (dx * dx + dy * dy).sqrt().max(1e-3);
        let target_r: f32 = nodes[t_i].get_size().clamp(4.0, 22.0);
        let trim_tip: Point = Point::new(
            p2.get_x() - (dx / dist) * (target_r + 2.0),
            p2.get_y() - (dy / dist) * (target_r + 2.0),
        );
        if edge.get_curveness().abs() > 0.01 {
            let ctrl: Point = Point::new(
                (p0.get_x() + p2.get_x()) * 0.5 - dy * edge.get_curveness(),
                (p0.get_y() + p2.get_y()) * 0.5 + dx * edge.get_curveness(),
            );
            let pts: Vec<Point> = sample_quad_bezier(p0, ctrl, trim_tip, DRAW_BEZIER_SEGMENTS);
            draw_polyline(dl, &pts, edge_col, w, false);
            if graph.get_directed() && pts.len() >= 2 {
                draw_arrow_head(dl, pts[pts.len() - 2], trim_tip, 7.0, edge_col);
            }
        } else {
            dl.push(DrawCommand::Line {
                from: p0,
                to: trim_tip,
                color: edge_col,
                width: w,
            });
            if graph.get_directed() {
                draw_arrow_head(dl, p0, trim_tip, 7.0, edge_col);
            }
        }
    }
    let halo: Color = frame.tokens.get_palette().get_card();
    let label_col: Color = frame.tokens.get_palette().get_foreground();
    let pal_len: usize = frame.palette.len().max(1);
    for (i, node) in nodes.iter().enumerate() {
        let pos: Point = Point::new(positions[i].0, positions[i].1);
        let r: f32 = node.get_size().clamp(5.0, 22.0);
        let col: Color = node
            .color_opt()
            .unwrap_or(frame.palette[node.get_category() % pal_len]);
        draw_symbol(dl, EuvChartSymbol::Circle, pos, r, col, halo);
        if graph.get_show_labels() && !node.get_name().is_empty() {
            draw_label(
                dl,
                Rect::new(pos.get_x() - 36.0, pos.get_y() + r + 2.0, 72.0, 12.0),
                node.get_name().clone(),
                9.5,
                FontWeight::Medium,
                TextAlign::Center,
                label_col,
            );
        }
        hits.push(EuvChartDatumHit::point_hit(
            (meta.index, meta.name.to_string(), i),
            (
                node.get_name().clone(),
                node.get_value(),
                format_chart_number(node.get_value(), ""),
            ),
            col,
            pos,
            (r + 6.0).max(10.0),
        ));
    }
}

/// Renders an orthogonal (`LeftRight`/`TopBottom`) or `Radial` hierarchical node-link `Tree` series.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartTreeSeries` - Tree descriptor.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_tree_series(
    dl: &mut DisplayList,
    plot: Rect,
    meta: SeriesMeta<'_>,
    tree: &EuvChartTreeSeries,
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let mut flat: Vec<TreeLayoutNode> = Vec::new();
    let mut leaf_cursor: usize = 0;
    let _: f32 = collect_tree_nodes(tree.get_root(), 0, None, &mut leaf_cursor, &mut flat);
    if flat.is_empty() {
        return;
    }
    let max_depth: usize = flat
        .iter()
        .map(|n: &TreeLayoutNode| n.depth)
        .max()
        .unwrap_or(0)
        .max(1);
    let total_leaves: f32 = (leaf_cursor.max(1)) as f32;
    for item in flat.iter_mut() {
        item.order = (item.order + 0.5) / total_leaves;
    }
    let pad_x: f32 = 34.0;
    let pad_y: f32 = 22.0;
    let inner_w: f32 = (plot.get_width() - pad_x * 2.0).max(40.0);
    let inner_h: f32 = (plot.get_height() - pad_y * 2.0).max(40.0);
    let cx: f32 = plot.get_x() + plot.get_width() * 0.5;
    let cy: f32 = plot.get_y() + plot.get_height() * 0.5;
    let max_r: f32 = (plot.get_width().min(plot.get_height()) * 0.42).max(24.0);
    for item in flat.iter_mut() {
        let d_frac: f32 = (item.depth as f32) / (max_depth as f32);
        item.pos = match tree.get_layout() {
            EuvChartTreeLayout::OrthogonalLeftRight => Point::new(
                plot.get_x() + pad_x + d_frac * inner_w,
                plot.get_y() + pad_y + item.order * inner_h,
            ),
            EuvChartTreeLayout::OrthogonalTopBottom => Point::new(
                plot.get_x() + pad_x + item.order * inner_w,
                plot.get_y() + pad_y + d_frac * inner_h,
            ),
            EuvChartTreeLayout::Radial => {
                if item.depth == 0 {
                    Point::new(cx, cy)
                } else {
                    let ang: f32 = -FRAC_PI_2 + item.order * TAU;
                    let rr: f32 = d_frac * max_r;
                    Point::new(cx + rr * ang.cos(), cy + rr * ang.sin())
                }
            }
        };
    }
    let edge_col: Color = frame.tokens.get_palette().get_border();
    for item in flat.iter() {
        let Some(p_idx) = item.parent else {
            continue;
        };
        let p0: Point = flat[p_idx].pos;
        let p3: Point = item.pos;
        let curve: Vec<Point> = match tree.get_layout() {
            EuvChartTreeLayout::OrthogonalLeftRight => {
                let mx: f32 = (p0.get_x() + p3.get_x()) * 0.5;
                sample_cubic_bezier(
                    p0,
                    Point::new(mx, p0.get_y()),
                    Point::new(mx, p3.get_y()),
                    p3,
                    DRAW_BEZIER_SEGMENTS,
                )
            }
            EuvChartTreeLayout::OrthogonalTopBottom => {
                let my: f32 = (p0.get_y() + p3.get_y()) * 0.5;
                sample_cubic_bezier(
                    p0,
                    Point::new(p0.get_x(), my),
                    Point::new(p3.get_x(), my),
                    p3,
                    DRAW_BEZIER_SEGMENTS,
                )
            }
            EuvChartTreeLayout::Radial => vec![p0, p3],
        };
        draw_polyline(dl, &curve, edge_col, 1.4, false);
    }
    let halo: Color = frame.tokens.get_palette().get_card();
    let label_col: Color = frame.tokens.get_palette().get_foreground();
    let pal_len: usize = frame.palette.len().max(1);
    let r: f32 = tree.get_symbol_size().clamp(3.0, 12.0);
    for (i, item) in flat.iter().enumerate() {
        let col: Color = item.color.unwrap_or(frame.palette[item.depth % pal_len]);
        draw_symbol(dl, EuvChartSymbol::Circle, item.pos, r, col, halo);
        if tree.get_show_labels() && !item.name.is_empty() {
            let (lx, ly, align): (f32, f32, TextAlign) = match tree.get_layout() {
                EuvChartTreeLayout::OrthogonalLeftRight => (
                    item.pos.get_x() + r + 4.0,
                    item.pos.get_y() - 6.0,
                    TextAlign::Left,
                ),
                EuvChartTreeLayout::OrthogonalTopBottom | EuvChartTreeLayout::Radial => (
                    item.pos.get_x() - 32.0,
                    item.pos.get_y() + r + 2.0,
                    TextAlign::Center,
                ),
            };
            draw_label(
                dl,
                Rect::new(lx, ly, 64.0, 12.0),
                item.name.clone(),
                9.5,
                FontWeight::Medium,
                align,
                label_col,
            );
        }
        hits.push(EuvChartDatumHit::point_hit(
            (meta.index, meta.name.to_string(), i),
            (
                item.name.clone(),
                item.value,
                format_chart_number(item.value, ""),
            ),
            col,
            item.pos,
            (r + 6.0).max(10.0),
        ));
    }
}

/// Recursively flattens a hierarchy tree assigning vertical/angular leaf ordering.
///
/// # Arguments
///
/// - `&EuvChartHierarchyNode` - Current tree node.
/// - `usize` - Current depth.
/// - `Option<usize>` - Parent index in `out`.
/// - `&mut usize` - Running leaf counter.
/// - `&mut Vec<TreeLayoutNode>` - Output flattened tree nodes.
///
/// # Returns
///
/// - `f32` - Mean leaf slot order assigned to this subtree.
fn collect_tree_nodes(
    node: &EuvChartHierarchyNode,
    depth: usize,
    parent: Option<usize>,
    leaf_cursor: &mut usize,
    out: &mut Vec<TreeLayoutNode>,
) -> f32 {
    let my_idx: usize = out.len();
    out.push(TreeLayoutNode {
        name: node.get_name().clone(),
        value: node.total_value(),
        color: node.color_opt(),
        depth,
        order: 0.0,
        pos: Point::default(),
        parent,
    });
    let children: &[EuvChartHierarchyNode] = node.get_children().as_slice();
    let order_val: f32 = if children.is_empty() {
        let cur: f32 = *leaf_cursor as f32;
        *leaf_cursor += 1;
        cur
    } else {
        let mut first_o: f32 = 0.0;
        let mut last_o: f32 = 0.0;
        for (idx, ch) in children.iter().enumerate() {
            let co: f32 = collect_tree_nodes(ch, depth + 1, Some(my_idx), leaf_cursor, out);
            if idx == 0 {
                first_o = co;
            }
            last_o = co;
        }
        (first_o + last_o) * 0.5
    };
    out[my_idx].order = order_val;
    order_val
}

/// Renders a multi-axis `Parallel` coordinates polyline series colored by category or `VisualMap`.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `SeriesMeta<'_>` - Series index, name, and color.
/// - `&EuvChartParallelSeries` - Parallel descriptor.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_parallel_series(
    dl: &mut DisplayList,
    plot: Rect,
    meta: SeriesMeta<'_>,
    parallel: &EuvChartParallelSeries,
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let axes: &[EuvChartParallelAxis] = parallel.get_axes().as_slice();
    if axes.len() < 2 {
        return;
    }
    let p: ThemePalette = frame.tokens.get_palette();
    let pad_x: f32 = 28.0;
    let pad_top: f32 = 24.0;
    let pad_bot: f32 = 16.0;
    let x0: f32 = plot.get_x() + pad_x;
    let w: f32 = (plot.get_width() - pad_x * 2.0).max(40.0);
    let y0: f32 = plot.get_y() + pad_top;
    let h: f32 = (plot.get_height() - pad_top - pad_bot).max(40.0);
    let y1: f32 = y0 + h;
    let mut bounds: Vec<(f32, f32)> = Vec::with_capacity(axes.len());
    for (dim_i, ax) in axes.iter().enumerate() {
        let (o_min, o_max): (Option<f32>, Option<f32>) = ax.bounds_opt();
        let mut lo: f32 = f32::INFINITY;
        let mut hi: f32 = f32::NEG_INFINITY;
        for row in parallel.get_rows().iter() {
            if let Some(&v) = row.get_values().get(dim_i)
                && v.is_finite()
            {
                lo = lo.min(v);
                hi = hi.max(v);
            }
        }
        let r_lo: f32 = o_min.unwrap_or(if lo.is_finite() { lo } else { 0.0 });
        let r_hi: f32 = o_max.unwrap_or(if hi.is_finite() && hi > r_lo {
            hi
        } else {
            r_lo + 100.0
        });
        bounds.push((r_lo, r_hi));
    }
    for (dim_i, ax) in axes.iter().enumerate() {
        let ax_x: f32 = x0 + w * ((dim_i as f32) / ((axes.len() - 1) as f32));
        dl.push(DrawCommand::Line {
            from: Point::new(ax_x, y0),
            to: Point::new(ax_x, y1),
            color: p.get_border(),
            width: 1.4,
        });
        draw_label(
            dl,
            Rect::new(ax_x - 36.0, plot.get_y() + 4.0, 72.0, 14.0),
            ax.get_name().clone(),
            9.5,
            FontWeight::SemiBold,
            TextAlign::Center,
            p.get_foreground(),
        );
        let (lo, hi): (f32, f32) = bounds[dim_i];
        draw_label(
            dl,
            Rect::new(ax_x + 4.0, y0 - 2.0, 38.0, 11.0),
            format_chart_number(hi, ""),
            8.5,
            FontWeight::Normal,
            TextAlign::Left,
            p.get_muted_foreground(),
        );
        draw_label(
            dl,
            Rect::new(ax_x + 4.0, y1 - 10.0, 38.0, 11.0),
            format_chart_number(lo, ""),
            8.5,
            FontWeight::Normal,
            TextAlign::Left,
            p.get_muted_foreground(),
        );
    }
    let pal_len: usize = frame.palette.len().max(1);
    let alpha: f32 = parallel.get_line_opacity().clamp(0.15, 1.0);
    for (r_i, row) in parallel.get_rows().iter().enumerate() {
        let mut pts: Vec<Point> = Vec::with_capacity(axes.len());
        for (dim_i, &(lo, hi)) in bounds.iter().enumerate() {
            let v: f32 = row.get_values().get(dim_i).copied().unwrap_or(lo);
            let frac: f32 = ((v - lo) / (hi - lo).max(1e-5)).clamp(0.0, 1.0);
            let ax_x: f32 = x0 + w * ((dim_i as f32) / ((axes.len() - 1) as f32));
            let py: f32 = y1 - frac * h;
            pts.push(Point::new(ax_x, py));
        }
        let last_v: f32 = row.get_values().last().copied().unwrap_or(0.0);
        let col: Color = row.color_opt().unwrap_or_else(|| {
            if let Some(vm) = frame.visual_eval {
                vm.map_color(last_v)
            } else {
                frame.palette[row.get_category() % pal_len]
            }
        });
        draw_polyline(
            dl,
            &pts,
            col.multiply_alpha(alpha),
            parallel.get_line_width().max(1.0),
            false,
        );
        if let Some(&mid) = pts.get(pts.len() / 2) {
            hits.push(EuvChartDatumHit::point_hit(
                (meta.index, meta.name.to_string(), r_i),
                (
                    row.get_name().clone(),
                    last_v,
                    format_chart_number(last_v, ""),
                ),
                col,
                mid,
                10.0,
            ));
        }
    }
}

/// Builds the active list of geographic polygon regions for `geo` (built-in World/Continents or Custom).
///
/// # Arguments
///
/// - `&EuvChartGeo` - Geographic coordinate system configuration.
///
/// # Returns
///
/// - `Vec<EuvChartGeoRegion>` - Resolved regions ready for projection.
#[must_use]
pub(crate) fn resolve_geo_regions(geo: &EuvChartGeo) -> Vec<EuvChartGeoRegion> {
    if geo.get_preset() == EuvChartGeoMapPreset::Custom {
        return geo.get_regions().clone();
    }
    let mut regions: Vec<EuvChartGeoRegion> = vec![
        EuvChartGeoRegion::new(
            GEO_NAME_NORTH_AMERICA,
            GEO_CODE_NA,
            vec![GEO_POLY_NORTH_AMERICA.to_vec()],
        ),
        EuvChartGeoRegion::new(
            GEO_NAME_SOUTH_AMERICA,
            GEO_CODE_SA,
            vec![GEO_POLY_SOUTH_AMERICA.to_vec()],
        ),
        EuvChartGeoRegion::new(GEO_NAME_EUROPE, GEO_CODE_EU, vec![GEO_POLY_EUROPE.to_vec()]),
        EuvChartGeoRegion::new(GEO_NAME_AFRICA, GEO_CODE_AF, vec![GEO_POLY_AFRICA.to_vec()]),
        EuvChartGeoRegion::new(GEO_NAME_ASIA, GEO_CODE_AS, vec![GEO_POLY_ASIA.to_vec()]),
        EuvChartGeoRegion::new(
            GEO_NAME_OCEANIA,
            GEO_CODE_OC,
            vec![GEO_POLY_OCEANIA.to_vec()],
        ),
    ];
    if geo.get_preset() == EuvChartGeoMapPreset::World {
        regions.push(EuvChartGeoRegion::new(
            GEO_NAME_UNITED_STATES,
            GEO_CODE_US,
            vec![GEO_POLY_UNITED_STATES.to_vec()],
        ));
        regions.push(EuvChartGeoRegion::new(
            GEO_NAME_BRAZIL,
            GEO_CODE_BR,
            vec![GEO_POLY_BRAZIL.to_vec()],
        ));
        regions.push(EuvChartGeoRegion::new(
            GEO_NAME_CHINA,
            GEO_CODE_CN,
            vec![GEO_POLY_CHINA.to_vec()],
        ));
    }
    regions.extend(geo.get_regions().iter().cloned());
    regions
}

/// Projects a `(longitude, latitude)` degree pair through `proj` onto logical screen pixels.
///
/// # Arguments
///
/// - `GeoProjection` - Active map viewport projection.
/// - `f32` - Longitude in degrees.
/// - `f32` - Latitude in degrees.
///
/// # Returns
///
/// - `Point` - Projected screen coordinate.
#[must_use]
fn project_geo_point(proj: GeoProjection, lon: f32, lat: f32) -> Point {
    let tx: f32 = (lon - proj.min_lon) / (proj.max_lon - proj.min_lon).max(1e-4);
    let ty: f32 = 1.0 - (lat - proj.min_lat) / (proj.max_lat - proj.min_lat).max(1e-4);
    Point::new(
        proj.viewport.get_x() + tx * proj.viewport.get_width(),
        proj.viewport.get_y() + ty * proj.viewport.get_height(),
    )
}

/// Renders a `Geo` choropleth map plus any Geo-bound `Scatter`, `EffectScatter`, and `Lines` overlays.
///
/// # Arguments
///
/// - `&mut DisplayList` - Target draw command buffer.
/// - `Rect` - Active plot rectangle.
/// - `&EuvChartGeo` - Geographic coordinate system configuration.
/// - `&[EuvChartSeries]` - All chart series.
/// - `&[bool]` - Visibility flags aligned with `series`.
/// - `&ChartDrawFrame<'_>` - Active chart frame context.
/// - `&mut Vec<EuvChartDatumHit>` - Hit-test output list.
pub(crate) fn render_geo_system(
    dl: &mut DisplayList,
    plot: Rect,
    geo: &EuvChartGeo,
    series: &[EuvChartSeries],
    visible: &[bool],
    frame: &ChartDrawFrame<'_>,
    hits: &mut Vec<EuvChartDatumHit>,
) {
    let regions: Vec<EuvChartGeoRegion> = resolve_geo_regions(geo);
    let mut min_lon: f32 = f32::INFINITY;
    let mut max_lon: f32 = f32::NEG_INFINITY;
    let mut min_lat: f32 = f32::INFINITY;
    let mut max_lat: f32 = f32::NEG_INFINITY;
    for r in regions.iter() {
        for ring in r.get_polygons().iter() {
            for &(lon, lat) in ring.iter() {
                min_lon = min_lon.min(lon);
                max_lon = max_lon.max(lon);
                min_lat = min_lat.min(lat);
                max_lat = max_lat.max(lat);
            }
        }
    }
    if !min_lon.is_finite() {
        min_lon = -180.0;
        max_lon = 180.0;
        min_lat = -60.0;
        max_lat = 75.0;
    }
    let pad: f32 = 10.0;
    let inner_w: f32 = (plot.get_width() - pad * 2.0).max(20.0);
    let inner_h: f32 = (plot.get_height() - pad * 2.0).max(20.0);
    let geo_aspect: f32 = ((max_lon - min_lon).max(1.0)) / ((max_lat - min_lat).max(1.0));
    let (vp_w, vp_h): (f32, f32) = if inner_w / inner_h > geo_aspect {
        (inner_h * geo_aspect, inner_h)
    } else {
        (inner_w, inner_w / geo_aspect)
    };
    let proj: GeoProjection = GeoProjection {
        min_lon,
        max_lon,
        min_lat,
        max_lat,
        viewport: Rect::new(
            plot.get_x() + (plot.get_width() - vp_w) * 0.5,
            plot.get_y() + (plot.get_height() - vp_h) * 0.5,
            vp_w,
            vp_h,
        ),
    };
    let p: ThemePalette = frame.tokens.get_palette();
    let ocean_bg: Color = p.get_muted().multiply_alpha(0.32);
    dl.push(DrawCommand::FillRoundedRect {
        rect: plot,
        color: ocean_bg,
        radius: CornerRadius::all(6.0),
        border: Border::new(1.0, p.get_border().multiply_alpha(0.5)),
    });
    let mut region_values: HashMap<String, (f32, Option<Color>, usize, String)> = HashMap::new();
    let mut show_map_labels: bool = geo.get_show_labels();
    let mut border_w: f32 = 1.0;
    for (s_i, s) in series.iter().enumerate() {
        if !visible.get(s_i).copied().unwrap_or(true) {
            continue;
        }
        if let EuvChartSeriesData::GeoMap(gm) = s.get_data() {
            show_map_labels = show_map_labels && gm.get_show_labels();
            border_w = gm.get_border_width().max(0.8);
            for d in gm.get_data().iter() {
                let _: Option<(f32, Option<Color>, usize, String)> = region_values.insert(
                    d.get_region_name().to_ascii_lowercase(),
                    (d.get_value(), d.color_opt(), s_i, s.get_name().clone()),
                );
            }
        }
    }
    let fallback_land: Color = p.get_secondary();
    let border_col: Color = p.get_card();
    for (r_i, reg) in regions.iter().enumerate() {
        let lookup: Option<&(f32, Option<Color>, usize, String)> = region_values
            .get(&reg.get_name().to_ascii_lowercase())
            .or_else(|| region_values.get(&reg.get_code().to_ascii_lowercase()));
        let fill: Color = match lookup {
            Some(&(v, exp_col, _, _)) => exp_col.unwrap_or_else(|| {
                frame
                    .visual_eval
                    .map(|vm: &EuvChartVisualEvaluator| vm.map_color(v))
                    .unwrap_or_else(|| p.get_primary().multiply_alpha(0.75))
            }),
            None => fallback_land,
        };
        let mut sum_x: f32 = 0.0;
        let mut sum_y: f32 = 0.0;
        let mut total_pts: usize = 0;
        for ring in reg.get_polygons().iter() {
            let pts: Vec<Point> = ring
                .iter()
                .map(|&(lon, lat): &(f32, f32)| project_geo_point(proj, lon, lat))
                .collect();
            for &pt in pts.iter() {
                sum_x += pt.get_x();
                sum_y += pt.get_y();
                total_pts += 1;
            }
            draw_polygon_fill(dl, &pts, fill);
            draw_polygon_stroke(dl, &pts, border_col, border_w);
        }
        let anchor: Point = if let Some((lon, lat)) = reg.label_center_opt() {
            project_geo_point(proj, lon, lat)
        } else if total_pts > 0 {
            Point::new(sum_x / (total_pts as f32), sum_y / (total_pts as f32))
        } else {
            Point::new(
                proj.viewport.get_x() + proj.viewport.get_width() * 0.5,
                proj.viewport.get_y() + proj.viewport.get_height() * 0.5,
            )
        };
        if show_map_labels && vp_w >= 260.0 {
            draw_label(
                dl,
                Rect::new(anchor.get_x() - 34.0, anchor.get_y() - 6.0, 68.0, 12.0),
                reg.get_code().clone(),
                8.5,
                FontWeight::SemiBold,
                TextAlign::Center,
                contrast_ink_for(fill),
            );
        }
        if let Some(&(v, _, s_i, ref s_name)) = lookup {
            hits.push(EuvChartDatumHit::point_hit(
                (s_i, s_name.clone(), r_i),
                (reg.get_name().clone(), v, format_chart_number(v, "")),
                fill,
                anchor,
                20.0,
            ));
        }
    }
    let halo: Color = p.get_card();
    let pal_len: usize = frame.palette.len().max(1);
    for (s_i, s) in series.iter().enumerate() {
        if !visible.get(s_i).copied().unwrap_or(true) {
            continue;
        }
        let base_col: Color = s.color_opt().unwrap_or(frame.palette[s_i % pal_len]);
        match s.get_data() {
            EuvChartSeriesData::Lines(lines)
                if lines.get_coordinate_system() == EuvChartCoordinateSystem::Geo =>
            {
                for (a_i, arc) in lines.get_arcs().iter().enumerate() {
                    let p0: Point = project_geo_point(proj, arc.get_from_x(), arc.get_from_y());
                    let p2: Point = project_geo_point(proj, arc.get_to_x(), arc.get_to_y());
                    let dx: f32 = p2.get_x() - p0.get_x();
                    let dy: f32 = p2.get_y() - p0.get_y();
                    let ctrl: Point = Point::new(
                        (p0.get_x() + p2.get_x()) * 0.5 - dy * lines.get_curveness(),
                        (p0.get_y() + p2.get_y()) * 0.5 + dx * lines.get_curveness(),
                    );
                    let stroke: Color = arc.color_opt().unwrap_or(base_col);
                    let pts: Vec<Point> = sample_quad_bezier(p0, ctrl, p2, DRAW_BEZIER_SEGMENTS);
                    draw_polyline(dl, &pts, stroke, lines.get_line_width().max(1.4), false);
                    if lines.get_show_arrows() && pts.len() >= 2 {
                        draw_arrow_head(dl, pts[pts.len() - 2], p2, 7.5, stroke);
                    }
                    hits.push(EuvChartDatumHit::point_hit(
                        (s_i, s.get_name().clone(), a_i),
                        (
                            format!("{} -> {}", arc.get_from_name(), arc.get_to_name()),
                            arc.get_value(),
                            format_chart_number(arc.get_value(), ""),
                        ),
                        stroke,
                        pts[pts.len() / 2],
                        10.0,
                    ));
                }
            }
            EuvChartSeriesData::Scatter(sc) | EuvChartSeriesData::EffectScatter(sc)
                if sc.get_coordinate_system() == EuvChartCoordinateSystem::Geo =>
            {
                let is_effect: bool = matches!(s.get_data(), EuvChartSeriesData::EffectScatter(_));
                for (pt_i, pt) in sc.get_points().iter().enumerate() {
                    let center: Point = project_geo_point(proj, pt.get_x(), pt.get_y());
                    let r: f32 = if pt.get_size() > 0.5 {
                        pt.get_size().clamp(3.0, 20.0)
                    } else {
                        sc.get_default_size().clamp(3.0, 16.0)
                    };
                    let fill: Color = match (frame.visual_eval, pt.value_opt()) {
                        (Some(vm), Some(mv)) => vm.map_color(mv),
                        _ => base_col,
                    };
                    if is_effect {
                        let rings: usize = sc.get_ripple_rings().clamp(1, 4);
                        for ring in (1..=rings).rev() {
                            let frac: f32 = (ring as f32) / (rings as f32);
                            let rr: f32 = r * (1.0 + (sc.get_ripple_scale() - 1.0) * frac);
                            draw_arc_stroke(
                                dl,
                                center,
                                rr,
                                0.0,
                                TAU,
                                fill.multiply_alpha(0.35 * (1.0 - frac * 0.65)),
                                1.5,
                            );
                        }
                    }
                    draw_symbol(dl, sc.get_symbol(), center, r, fill, halo);
                    if sc.get_show_labels() && !pt.get_label().is_empty() {
                        draw_label(
                            dl,
                            Rect::new(center.get_x() - 32.0, center.get_y() - r - 13.0, 64.0, 12.0),
                            pt.get_label().clone(),
                            9.0,
                            FontWeight::SemiBold,
                            TextAlign::Center,
                            p.get_foreground(),
                        );
                    }
                    hits.push(EuvChartDatumHit::point_hit(
                        (s_i, s.get_name().clone(), pt_i),
                        (
                            pt.get_label().clone(),
                            pt.value_opt().unwrap_or(pt.get_y()),
                            format_chart_number(pt.value_opt().unwrap_or(pt.get_y()), ""),
                        ),
                        fill,
                        center,
                        (r + 6.0).max(10.0),
                    ));
                }
            }
            _ => {}
        }
    }
}
