use super::*;

fn epsilon(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 1e-9
}

const EPSILON: f64 = 1e-9;

#[test]
fn nine_slice_source_rects_cover_full_source() {
    let insets: NineSliceInsets = NineSliceInsets::new(1.0, 1.0, 1.0, 1.0);
    let rects: Vec<Rect> = insets.source_rects(Rect::new(0.0, 0.0, 3.0, 3.0)).to_vec();
    for rect in rects.iter() {
        assert!(
            (rect.get_width() - 1.0).abs() < EPSILON && (rect.get_height() - 1.0).abs() < EPSILON,
            "expected every 3x3 source patch to be 1x1, got {}x{} at ({}, {})",
            rect.get_width(),
            rect.get_height(),
            rect.get_x(),
            rect.get_y(),
        );
    }
    let area: f64 = rects
        .iter()
        .map(|rect: &Rect| rect.get_width() * rect.get_height())
        .sum();
    assert!(
        (area - 9.0).abs() < EPSILON,
        "expected the nine source patches to cover 9.0 square pixels, got {}",
        area,
    );
}

#[test]
fn nine_slice_dest_rects_tile_destination_exactly() {
    let insets: NineSliceInsets = NineSliceInsets::new(1.0, 1.0, 1.0, 1.0);
    let rects: NineSliceRects = insets.dest_rects(Rect::new(0.0, 0.0, 10.0, 10.0));
    assert!(
        (rects.get_top_left().get_width() - 1.0).abs() < EPSILON
            && (rects.get_top_left().get_height() - 1.0).abs() < EPSILON,
        "expected top-left corner 1x1, got {}x{}",
        rects.get_top_left().get_width(),
        rects.get_top_left().get_height(),
    );
    assert!(
        (rects.get_top().get_width() - 8.0).abs() < EPSILON
            && (rects.get_top().get_height() - 1.0).abs() < EPSILON,
        "expected top edge 8x1, got {}x{}",
        rects.get_top().get_width(),
        rects.get_top().get_height(),
    );
    assert!(
        (rects.get_left().get_width() - 1.0).abs() < EPSILON
            && (rects.get_left().get_height() - 8.0).abs() < EPSILON,
        "expected left edge 1x8, got {}x{}",
        rects.get_left().get_width(),
        rects.get_left().get_height(),
    );
    assert!(
        (rects.get_center().get_width() - 8.0).abs() < EPSILON
            && (rects.get_center().get_height() - 8.0).abs() < EPSILON,
        "expected center 8x8, got {}x{}",
        rects.get_center().get_width(),
        rects.get_center().get_height(),
    );
    let flat: Vec<Rect> = rects.to_vec();
    let area: f64 = flat
        .iter()
        .map(|rect: &Rect| rect.get_width() * rect.get_height())
        .sum();
    assert!(
        (area - 100.0).abs() < EPSILON,
        "expected the nine dest patches to cover 100.0 square pixels, got {}",
        area,
    );
    let top_row_width: f64 = flat[0].get_width() + flat[1].get_width() + flat[2].get_width();
    assert!(
        (top_row_width - 10.0).abs() < EPSILON,
        "expected the top row to span 10.0 px, got {}",
        top_row_width,
    );
    for pair in [(0usize, 1usize), (1, 2), (3, 4), (4, 5), (6, 7), (7, 8)] {
        let left_rect: Rect = flat[pair.0];
        let right_rect: Rect = flat[pair.1];
        let boundary: f64 = left_rect.get_x() + left_rect.get_width();
        assert!(
            (boundary - right_rect.get_x()).abs() < EPSILON,
            "expected patches {} and {} to be contiguous at x={}, got {} and {}",
            pair.0,
            pair.1,
            boundary,
            left_rect.get_x() + left_rect.get_width(),
            right_rect.get_x(),
        );
    }
    for pair in [(0usize, 3usize), (3, 6), (1, 4), (4, 7), (2, 5), (5, 8)] {
        let top_rect: Rect = flat[pair.0];
        let bottom_rect: Rect = flat[pair.1];
        let boundary: f64 = top_rect.get_y() + top_rect.get_height();
        assert!(
            (boundary - bottom_rect.get_y()).abs() < EPSILON,
            "expected patches {} and {} to be contiguous at y={}, got {} and {}",
            pair.0,
            pair.1,
            boundary,
            top_rect.get_y() + top_rect.get_height(),
            bottom_rect.get_y(),
        );
    }
}

#[test]
fn nine_slice_dest_rects_stretch_only_stretchable_axes() {
    let insets: NineSliceInsets = NineSliceInsets::new(1.0, 1.0, 1.0, 1.0);
    let rects: NineSliceRects = insets.dest_rects(Rect::new(0.0, 0.0, 20.0, 5.0));
    assert!(
        (rects.get_center().get_width() - 18.0).abs() < EPSILON,
        "expected the center to stretch horizontally to 18.0, got {}",
        rects.get_center().get_width(),
    );
    assert!(
        (rects.get_center().get_height() - 3.0).abs() < EPSILON,
        "expected the center to stretch vertically to 3.0, got {}",
        rects.get_center().get_height(),
    );
    assert!(
        (rects.get_top_left().get_width() - 1.0).abs() < EPSILON
            && (rects.get_top_left().get_height() - 1.0).abs() < EPSILON,
        "expected the corner to keep its natural 1x1, got {}x{}",
        rects.get_top_left().get_width(),
        rects.get_top_left().get_height(),
    );
    assert!(
        (rects.get_top().get_height() - 1.0).abs() < EPSILON,
        "expected the top edge to keep its natural height of 1.0, got {}",
        rects.get_top().get_height(),
    );
    assert!(
        (rects.get_left().get_width() - 1.0).abs() < EPSILON,
        "expected the left edge to keep its natural width of 1.0, got {}",
        rects.get_left().get_width(),
    );
    let flat: Vec<Rect> = rects.to_vec();
    let area: f64 = flat
        .iter()
        .map(|rect: &Rect| rect.get_width() * rect.get_height())
        .sum();
    assert!(
        (area - 100.0).abs() < EPSILON,
        "expected the nine dest patches to cover 20x5 = 100.0 square pixels, got {}",
        area,
    );
}

#[test]
fn nine_slice_dest_rects_collapse_center_when_destination_too_small() {
    let insets: NineSliceInsets = NineSliceInsets::new(1.0, 1.0, 1.0, 1.0);
    let rects: NineSliceRects = insets.dest_rects(Rect::new(0.0, 0.0, 1.0, 1.0));
    assert!(
        rects.get_center().get_width().abs() < EPSILON
            && rects.get_center().get_height().abs() < EPSILON,
        "expected a zero-size center for a 1x1 destination, got {}x{}",
        rects.get_center().get_width(),
        rects.get_center().get_height(),
    );
    let flat: Vec<Rect> = rects.to_vec();
    for rect in flat.iter() {
        assert!(
            rect.get_width() >= 0.0 && rect.get_height() >= 0.0,
            "expected no negative dest patch, got {}x{}",
            rect.get_width(),
            rect.get_height(),
        );
    }
}

#[test]
fn nine_slice_source_rects_clamp_insets_to_source_size() {
    let insets: NineSliceInsets = NineSliceInsets::new(10.0, 10.0, 10.0, 10.0);
    let flat: Vec<Rect> = insets.source_rects(Rect::new(0.0, 0.0, 4.0, 4.0)).to_vec();
    for rect in flat.iter() {
        assert!(
            rect.get_width() >= 0.0 && rect.get_height() >= 0.0,
            "expected over-large insets to clamp to a non-negative patch, got {}x{}",
            rect.get_width(),
            rect.get_height(),
        );
    }
    let area: f64 = flat
        .iter()
        .map(|rect: &Rect| rect.get_width() * rect.get_height())
        .sum();
    assert!(
        (area - 16.0).abs() < EPSILON,
        "expected over-large insets to still tile the 4x4 source (16.0), got {}",
        area,
    );
}

#[test]
fn nine_slice_source_rects_honor_asymmetric_insets() {
    let insets: NineSliceInsets = NineSliceInsets::new(2.0, 3.0, 4.0, 5.0);
    let rects: NineSliceRects = insets.source_rects(Rect::new(0.0, 0.0, 10.0, 12.0));
    assert!(
        (rects.get_top_left().get_width() - 2.0).abs() < EPSILON
            && (rects.get_top_left().get_height() - 4.0).abs() < EPSILON,
        "expected the 2x4 top-left corner, got {}x{}",
        rects.get_top_left().get_width(),
        rects.get_top_left().get_height(),
    );
    assert!(
        (rects.get_center().get_width() - 5.0).abs() < EPSILON
            && (rects.get_center().get_height() - 3.0).abs() < EPSILON,
        "expected the 5x3 center remaining after 2/3 and 4/5 insets, got {}x{}",
        rects.get_center().get_width(),
        rects.get_center().get_height(),
    );
    let bottom_right: Rect = rects.get_bottom_right();
    assert!(
        (bottom_right.get_x() - 7.0).abs() < EPSILON
            && (bottom_right.get_y() - 7.0).abs() < EPSILON
            && (bottom_right.get_width() - 3.0).abs() < EPSILON
            && (bottom_right.get_height() - 5.0).abs() < EPSILON,
        "expected the bottom-right corner at (7, 7) sized 3x5, got ({}, {}) sized {}x{}",
        bottom_right.get_x(),
        bottom_right.get_y(),
        bottom_right.get_width(),
        bottom_right.get_height(),
    );
}

#[test]
fn nine_slice_source_rects_are_offset_by_source_origin() {
    let insets: NineSliceInsets = NineSliceInsets::new(1.0, 1.0, 1.0, 1.0);
    let rects: NineSliceRects = insets.source_rects(Rect::new(7.0, 9.0, 3.0, 3.0));
    assert!(
        (rects.get_top_left().get_x() - 7.0).abs() < EPSILON
            && (rects.get_top_left().get_y() - 9.0).abs() < EPSILON,
        "expected the first patch to sit at the source origin (7, 9), got ({}, {})",
        rects.get_top_left().get_x(),
        rects.get_top_left().get_y(),
    );
    let center: Rect = rects.get_center();
    assert!(
        (center.get_x() - 8.0).abs() < EPSILON && (center.get_y() - 10.0).abs() < EPSILON,
        "expected the center at (8, 10), got ({}, {})",
        center.get_x(),
        center.get_y(),
    );
}

#[test]
fn nine_slice_rects_to_vec_preserves_reading_order() {
    let insets: NineSliceInsets = NineSliceInsets::new(1.0, 2.0, 3.0, 4.0);
    let rects: NineSliceRects = insets.source_rects(Rect::new(0.0, 0.0, 20.0, 20.0));
    let flat: Vec<Rect> = rects.to_vec();
    assert!(
        flat.len() == 9,
        "expected exactly nine sub-rectangles, got {}",
        flat.len(),
    );
    let expected: Vec<Rect> = vec![
        rects.get_top_left(),
        rects.get_top(),
        rects.get_top_right(),
        rects.get_left(),
        rects.get_center(),
        rects.get_right(),
        rects.get_bottom_left(),
        rects.get_bottom(),
        rects.get_bottom_right(),
    ];
    for index in 0..flat.len() {
        assert!(
            flat[index] == expected[index],
            "expected flat[{}] to match the named field, got ({}, {}, {}, {})",
            index,
            flat[index].get_x(),
            flat[index].get_y(),
            flat[index].get_width(),
            flat[index].get_height(),
        );
    }
}

#[test]
fn nine_slice_default_insets_are_zero() {
    let insets: NineSliceInsets = NineSliceInsets::default();
    assert!(
        insets.get_left().abs() < EPSILON
            && insets.get_right().abs() < EPSILON
            && insets.get_top().abs() < EPSILON
            && insets.get_bottom().abs() < EPSILON,
        "expected default insets to be all zero, got ({}, {}, {}, {})",
        insets.get_left(),
        insets.get_right(),
        insets.get_top(),
        insets.get_bottom(),
    );
    let rects: NineSliceRects = insets.dest_rects(Rect::new(2.0, 3.0, 8.0, 6.0));
    let center: Rect = rects.get_center();
    assert!(
        center == Rect::new(2.0, 3.0, 8.0, 6.0),
        "expected zero insets to make the center the whole destination, got ({}, {}, {}, {})",
        center.get_x(),
        center.get_y(),
        center.get_width(),
        center.get_height(),
    );
}

#[test]
fn nine_slice_uv_rect_fields_round_trip() {
    let uv: UvRect = UvRect::new(0.25, 0.5, 0.75, 1.0);
    assert!(
        (uv.get_u0() - 0.25).abs() < EPSILON
            && (uv.get_v0() - 0.5).abs() < EPSILON
            && (uv.get_u1() - 0.75).abs() < EPSILON
            && (uv.get_v1() - 1.0).abs() < EPSILON,
        "expected the UvRect getters to return the constructed values, got ({}, {}, {}, {})",
        uv.get_u0(),
        uv.get_v0(),
        uv.get_u1(),
        uv.get_v1(),
    );
}

#[test]
fn atlas_uv_normalizes_top_left_quarter_of_square_atlas() {
    let uv: UvRect = SpriteAtlas::normalize_uv(Rect::new(0.0, 0.0, 16.0, 16.0), 32.0, 32.0);
    assert!(
        (uv.get_u0() - 0.0).abs() < EPSILON
            && (uv.get_v0() - 0.0).abs() < EPSILON
            && (uv.get_u1() - 0.5).abs() < EPSILON
            && (uv.get_v1() - 0.5).abs() < EPSILON,
        "expected (0,0,16,16) in a 32x32 atlas to normalize to (0.0, 0.0, 0.5, 0.5), got ({}, {}, {}, {})",
        uv.get_u0(),
        uv.get_v0(),
        uv.get_u1(),
        uv.get_v1(),
    );
}

#[test]
fn atlas_uv_normalizes_offset_eighth_of_square_atlas() {
    let uv: UvRect = SpriteAtlas::normalize_uv(Rect::new(16.0, 16.0, 8.0, 8.0), 32.0, 32.0);
    assert!(
        (uv.get_u0() - 0.5).abs() < EPSILON
            && (uv.get_v0() - 0.5).abs() < EPSILON
            && (uv.get_u1() - 0.75).abs() < EPSILON
            && (uv.get_v1() - 0.75).abs() < EPSILON,
        "expected (16,16,8,8) in a 32x32 atlas to normalize to (0.5, 0.5, 0.75, 0.75), got ({}, {}, {}, {})",
        uv.get_u0(),
        uv.get_v0(),
        uv.get_u1(),
        uv.get_v1(),
    );
}

#[test]
fn atlas_uv_handles_non_square_atlas_axes_independently() {
    let uv: UvRect = SpriteAtlas::normalize_uv(Rect::new(8.0, 4.0, 16.0, 12.0), 32.0, 16.0);
    assert!(
        (uv.get_u0() - 0.25).abs() < EPSILON
            && (uv.get_v0() - 0.25).abs() < EPSILON
            && (uv.get_u1() - 0.75).abs() < EPSILON
            && (uv.get_v1() - 1.0).abs() < EPSILON,
        "expected (8,4,16,12) in a 32x16 atlas to normalize to (0.25, 0.25, 0.75, 1.0), got ({}, {}, {}, {})",
        uv.get_u0(),
        uv.get_v0(),
        uv.get_u1(),
        uv.get_v1(),
    );
}

#[test]
fn atlas_uv_zero_sized_atlas_yields_zeros_not_nan() {
    for size in [(0.0, 0.0), (0.0, 32.0), (32.0, 0.0)] {
        let uv: UvRect = SpriteAtlas::normalize_uv(Rect::new(16.0, 16.0, 8.0, 8.0), size.0, size.1);
        assert!(
            uv.get_u0().is_finite()
                && uv.get_v0().is_finite()
                && uv.get_u1().is_finite()
                && uv.get_v1().is_finite(),
            "expected finite UVs for a zero-sized atlas ({}x{}), got ({}, {}, {}, {})",
            size.0,
            size.1,
            uv.get_u0(),
            uv.get_v0(),
            uv.get_u1(),
            uv.get_v1(),
        );
        assert!(
            uv.get_u0().abs() < EPSILON
                && uv.get_v0().abs() < EPSILON
                && uv.get_u1().abs() < EPSILON
                && uv.get_v1().abs() < EPSILON,
            "expected all-zero UVs for a zero-sized atlas ({}x{}), got ({}, {}, {}, {})",
            size.0,
            size.1,
            uv.get_u0(),
            uv.get_v0(),
            uv.get_u1(),
            uv.get_v1(),
        );
    }
}

#[test]
fn atlas_regions_insert_get_and_contains() {
    let mut regions: AtlasRegions = AtlasRegions::default();
    assert!(
        regions.is_empty() && regions.is_empty(),
        "expected a fresh index to be empty, got len {}",
        regions.len(),
    );
    let hero_name: &str = "hero";
    regions.insert(hero_name, Rect::new(0.0, 0.0, 16.0, 32.0));
    regions.insert("coin", Rect::new(16.0, 0.0, 8.0, 8.0));
    assert!(
        regions.len() == 2,
        "expected two stored regions, got {}",
        regions.len(),
    );
    assert!(
        regions.contains("hero") && regions.contains("coin"),
        "expected both inserted names to be present",
    );
    assert!(
        !regions.contains("missing"),
        "expected an unknown name to be absent",
    );
    assert!(
        regions.get(hero_name) == Some(Rect::new(0.0, 0.0, 16.0, 32.0)),
        "expected the stored rect back for hero, got {:?}",
        regions.get("hero"),
    );
    assert!(
        regions.get("missing").is_none(),
        "expected None for an unknown name",
    );
    let mut names: Vec<String> = regions.names();
    names.sort();
    assert!(
        names == vec!["coin".to_string(), hero_name.to_string()],
        "expected the two stored names, got {:?}",
        names,
    );
}

#[test]
fn atlas_regions_insert_replaces_existing_name() {
    let mut regions: AtlasRegions = AtlasRegions::default();
    regions.insert("hero", Rect::new(0.0, 0.0, 16.0, 16.0));
    regions.insert("hero", Rect::new(32.0, 32.0, 8.0, 8.0));
    assert!(
        regions.len() == 1,
        "expected a replace to keep the count at 1, got {}",
        regions.len(),
    );
    assert!(
        regions.get("hero") == Some(Rect::new(32.0, 32.0, 8.0, 8.0)),
        "expected the second insert to win, got {:?}",
        regions.get("hero"),
    );
}

#[test]
fn atlas_regions_is_independent_per_instance() {
    let mut first: AtlasRegions = AtlasRegions::default();
    first.insert("hero", Rect::new(0.0, 0.0, 1.0, 1.0));
    let second: AtlasRegions = AtlasRegions::default();
    assert!(
        second.is_empty(),
        "expected a separately constructed index to stay empty, got len {}",
        second.len(),
    );
    assert!(
        !second.contains("hero"),
        "expected a separately constructed index not to see the first index's names",
    );
}

fn close(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 1e-9
}

fn rect(x: f64, y: f64, w: f64, h: f64) -> Rect {
    Rect::new(x, y, w, h)
}

fn uniform_insets(border: f64) -> NineSliceInsets {
    NineSliceInsets::new(border, border, border, border)
}

#[test]
fn nine_slice_rects_expose_all_nine_patches_by_name() {
    let grid: NineSliceRects = NineSliceRects::new([
        [
            rect(0.0, 0.0, 1.0, 1.0),
            rect(1.0, 0.0, 1.0, 1.0),
            rect(2.0, 0.0, 1.0, 1.0),
        ],
        [
            rect(0.0, 1.0, 1.0, 1.0),
            rect(1.0, 1.0, 1.0, 1.0),
            rect(2.0, 1.0, 1.0, 1.0),
        ],
        [
            rect(0.0, 2.0, 1.0, 1.0),
            rect(1.0, 2.0, 1.0, 1.0),
            rect(2.0, 2.0, 1.0, 1.0),
        ],
    ]);
    assert_eq!(grid.get_top_left(), rect(0.0, 0.0, 1.0, 1.0), "top left");
    assert_eq!(grid.get_top(), rect(1.0, 0.0, 1.0, 1.0), "top");
    assert_eq!(grid.get_top_right(), rect(2.0, 0.0, 1.0, 1.0), "top right");
    assert_eq!(grid.get_left(), rect(0.0, 1.0, 1.0, 1.0), "left");
    assert_eq!(grid.get_center(), rect(1.0, 1.0, 1.0, 1.0), "center");
    assert_eq!(grid.get_right(), rect(2.0, 1.0, 1.0, 1.0), "right");
    assert_eq!(
        grid.get_bottom_left(),
        rect(0.0, 2.0, 1.0, 1.0),
        "bottom left"
    );
    assert_eq!(grid.get_bottom(), rect(1.0, 2.0, 1.0, 1.0), "bottom");
    assert_eq!(
        grid.get_bottom_right(),
        rect(2.0, 2.0, 1.0, 1.0),
        "bottom right"
    );
}

#[test]
fn the_named_patches_read_in_reading_order() {
    let grid: NineSliceRects = NineSliceRects::new([
        [
            rect(0.0, 0.0, 1.0, 1.0),
            rect(1.0, 0.0, 1.0, 1.0),
            rect(2.0, 0.0, 1.0, 1.0),
        ],
        [
            rect(0.0, 1.0, 1.0, 1.0),
            rect(1.0, 1.0, 1.0, 1.0),
            rect(2.0, 1.0, 1.0, 1.0),
        ],
        [
            rect(0.0, 2.0, 1.0, 1.0),
            rect(1.0, 2.0, 1.0, 1.0),
            rect(2.0, 2.0, 1.0, 1.0),
        ],
    ]);
    let reading_order: Vec<Rect> = grid.to_vec();
    let named: Vec<Rect> = vec![
        grid.get_top_left(),
        grid.get_top(),
        grid.get_top_right(),
        grid.get_left(),
        grid.get_center(),
        grid.get_right(),
        grid.get_bottom_left(),
        grid.get_bottom(),
        grid.get_bottom_right(),
    ];
    assert_eq!(
        reading_order, named,
        "to_vec must agree with the named accessors"
    );
    assert_eq!(reading_order.len(), 9, "there are exactly nine patches");
}

#[test]
fn source_rects_split_the_source_into_a_three_by_three_grid() {
    let insets: NineSliceInsets = uniform_insets(10.0);
    let source: Rect = rect(0.0, 0.0, 90.0, 90.0);
    let grid: NineSliceRects = insets.source_rects(source);
    assert!(
        close(grid.get_top_left().get_width(), 10.0),
        "the corner keeps its natural size"
    );
    assert!(
        close(grid.get_center().get_width(), 70.0),
        "the centre absorbs the rest, got {}",
        grid.get_center().get_width()
    );
    assert!(
        close(grid.get_top().get_height(), 10.0),
        "the top strip is one border tall"
    );
    assert!(
        close(grid.get_center().get_height(), 70.0),
        "and the centre strip is the remainder"
    );
}

#[test]
fn source_rects_honour_the_source_rectangle_offset() {
    let insets: NineSliceInsets = uniform_insets(10.0);
    let source: Rect = rect(100.0, 200.0, 90.0, 90.0);
    let grid: NineSliceRects = insets.source_rects(source);
    assert!(
        close(grid.get_top_left().get_x(), 100.0),
        "x offset carried through"
    );
    assert!(
        close(grid.get_top_left().get_y(), 200.0),
        "y offset carried through"
    );
    assert!(
        close(grid.get_center().get_x(), 110.0),
        "the centre starts one border in"
    );
}

#[test]
fn insets_larger_than_the_source_collapse_the_centre_instead_of_inverting_it() {
    let insets: NineSliceInsets = uniform_insets(80.0);
    let source: Rect = rect(0.0, 0.0, 90.0, 90.0);
    let grid: NineSliceRects = insets.source_rects(source);
    assert!(
        grid.get_center().get_width() >= 0.0 && grid.get_center().get_height() >= 0.0,
        "a degenerate centre must collapse to zero, not go negative: {:?}",
        grid.get_center()
    );
    assert!(
        close(grid.get_center().get_width(), 0.0),
        "left takes 80 and right is clamped to the remaining 10, so the centre is exactly zero, got {}",
        grid.get_center().get_width()
    );
}

#[test]
fn a_negative_inset_is_clamped_to_zero() {
    let insets: NineSliceInsets = uniform_insets(-5.0);
    let source: Rect = rect(0.0, 0.0, 90.0, 90.0);
    let grid: NineSliceRects = insets.source_rects(source);
    assert!(
        close(grid.get_top_left().get_width(), 0.0),
        "a negative inset becomes zero"
    );
    assert!(
        close(grid.get_center().get_width(), 90.0),
        "so the centre takes the whole width"
    );
}

#[test]
fn dest_rects_stretch_the_corners_and_keep_them_fixed() {
    let insets: NineSliceInsets = uniform_insets(10.0);
    let dest: Rect = rect(0.0, 0.0, 200.0, 100.0);
    let grid: NineSliceRects = insets.dest_rects(dest);
    assert!(
        close(grid.get_top_left().get_width(), 10.0),
        "a corner never stretches"
    );
    assert!(
        close(grid.get_center().get_width(), 180.0),
        "the centre absorbs the remaining width, got {}",
        grid.get_center().get_width()
    );
    assert!(
        close(grid.get_center().get_height(), 80.0),
        "and the remaining height"
    );
}

#[test]
fn dest_rects_beyond_the_destination_clamp_without_producing_negative_patches() {
    let insets: NineSliceInsets = uniform_insets(60.0);
    let dest: Rect = rect(0.0, 0.0, 90.0, 90.0);
    let grid: NineSliceRects = insets.dest_rects(dest);
    for patch in grid.to_vec() {
        assert!(
            patch.get_width() >= 0.0 && patch.get_height() >= 0.0,
            "no patch may invert, got {patch:?}"
        );
    }
    assert!(
        close(grid.get_center().get_width(), 0.0),
        "the left edge takes 60 and the right edge the remaining 30, so the centre is exactly zero, got {}",
        grid.get_center().get_width()
    );
}

#[test]
fn dest_rects_honour_the_destination_offset() {
    let insets: NineSliceInsets = uniform_insets(10.0);
    let dest: Rect = rect(50.0, 60.0, 200.0, 100.0);
    let grid: NineSliceRects = insets.dest_rects(dest);
    assert!(
        close(grid.get_top_left().get_x(), 50.0),
        "x offset carried through"
    );
    assert!(
        close(grid.get_top_left().get_y(), 60.0),
        "y offset carried through"
    );
    assert!(
        close(grid.get_center().get_x(), 60.0),
        "the centre starts one border in"
    );
}

#[test]
fn a_zero_sized_destination_produces_all_zero_patches() {
    let insets: NineSliceInsets = uniform_insets(10.0);
    let dest: Rect = rect(0.0, 0.0, 0.0, 0.0);
    let grid: NineSliceRects = insets.dest_rects(dest);
    for patch in grid.to_vec() {
        assert!(
            close(patch.get_width(), 0.0) && close(patch.get_height(), 0.0),
            "got {patch:?}"
        );
    }
}

fn frame(x: f64, duration: f64) -> SpriteFrame {
    SpriteFrame::new(Rect::new(x, 0.0, 8.0, 8.0), duration)
}

fn animation(frames: Vec<SpriteFrame>, mode: AnimationMode) -> SpriteAnimation {
    SpriteAnimation::new(String::from("walk"), frames, mode)
}

#[test]
fn a_fresh_animator_is_idle_with_nothing_to_show() {
    let animator: Animator = Animator::create();
    assert!(
        animator.current_frame_source().is_none(),
        "with no animation loaded there is no current frame"
    );
    assert!(
        epsilon(animator.get_elapsed_time(), 0.0),
        "and no time has passed"
    );
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "the frame index starts at zero"
    );
}

#[test]
fn updating_a_stopped_animator_never_advances_it() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(
        vec![frame(0.0, 0.1), frame(8.0, 0.1)],
        AnimationMode::Loop,
    ));
    animator.pause();
    for _ in 0..20 {
        animator.update(1.0);
    }
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "a paused animator must not advance no matter how much time is pushed in"
    );
    assert!(
        epsilon(animator.get_elapsed_time(), 0.0),
        "and it accumulates no elapsed time"
    );
}

#[test]
fn playing_an_animation_puts_the_animator_into_the_playing_state() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(vec![frame(0.0, 0.1)], AnimationMode::Loop));
    assert_eq!(
        animator.get_state(),
        AnimationState::Playing,
        "play starts the clock"
    );
    assert!(
        animator.current_frame_source().is_some(),
        "and there is now a current frame to report"
    );
}

#[test]
fn pause_and_resume_only_move_between_the_playing_and_paused_states() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(vec![frame(0.0, 0.1)], AnimationMode::Loop));
    animator.resume();
    assert_eq!(
        animator.get_state(),
        AnimationState::Playing,
        "resume from Playing is a no-op rather than a toggle"
    );
    animator.pause();
    assert_eq!(
        animator.get_state(),
        AnimationState::Paused,
        "pause takes effect"
    );
    animator.pause();
    assert_eq!(
        animator.get_state(),
        AnimationState::Paused,
        "a second pause is also a no-op"
    );
    animator.resume();
    assert_eq!(
        animator.get_state(),
        AnimationState::Playing,
        "resume takes effect"
    );
}

#[test]
fn stop_parks_the_animator_without_resuming_it() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(vec![frame(0.0, 0.1)], AnimationMode::Loop));
    animator.stop();
    assert_eq!(
        animator.get_state(),
        AnimationState::Paused,
        "stop parks the animator at Paused, it does not reach Finished"
    );
    animator.update(1.0);
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "and a parked animator does not advance"
    );
}

#[test]
fn a_frame_advances_once_its_own_duration_elapses() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(
        vec![frame(0.0, 0.5), frame(8.0, 0.5), frame(16.0, 0.5)],
        AnimationMode::Loop,
    ));
    animator.update(0.2);
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "a partial frame does not advance"
    );
    assert!(
        epsilon(animator.get_elapsed_time(), 0.2),
        "the partial time is banked"
    );
    animator.update(0.4);
    assert_eq!(
        animator.get_current_frame_index(),
        1,
        "crossing the duration advances exactly one frame"
    );
    assert!(
        epsilon(animator.get_elapsed_time(), 0.0),
        "the overshoot is discarded rather than carried, so the next frame starts from zero, got {}",
        animator.get_elapsed_time()
    );
}

#[test]
fn a_looping_animation_wraps_back_to_the_first_frame() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(
        vec![frame(0.0, 0.1), frame(8.0, 0.1), frame(16.0, 0.1)],
        AnimationMode::Loop,
    ));
    for expected in [1usize, 2, 0, 1] {
        animator.update(0.2);
        assert_eq!(
            animator.get_current_frame_index(),
            expected,
            "a looping animation must wrap rather than stop at the end"
        );
    }
}

#[test]
fn a_once_animation_stops_at_its_last_frame() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(
        vec![frame(0.0, 0.1), frame(8.0, 0.1)],
        AnimationMode::Once,
    ));
    animator.update(0.2);
    assert_eq!(
        animator.get_current_frame_index(),
        1,
        "it advances to the last frame"
    );
    animator.update(0.2);
    assert_eq!(
        animator.get_current_frame_index(),
        1,
        "and a once animation must not advance past the end"
    );
}

#[test]
fn an_animation_with_no_frames_is_safe_to_update() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(vec![], AnimationMode::Loop));
    animator.update(1.0);
    assert!(
        animator.current_frame_source().is_none(),
        "an empty animation reports no current frame rather than panicking"
    );
    assert_eq!(
        animator.get_current_frame_index(),
        0,
        "and the index stays put"
    );
}

#[test]
fn the_current_frame_source_tracks_the_frame_the_animator_is_on() {
    let mut animator: Animator = Animator::create();
    animator.play(animation(
        vec![frame(0.0, 0.1), frame(64.0, 0.1)],
        AnimationMode::Loop,
    ));
    let first: Option<Rect> = animator.current_frame_source();
    let first_rect: Rect = first.expect("the first frame has a source");
    assert!(
        epsilon(first_rect.get_x(), 0.0),
        "the first frame is reported first"
    );
    animator.update(0.2);
    let second: Option<Rect> = animator.current_frame_source();
    let second_rect: Rect = second.expect("the second frame has a source");
    assert!(
        epsilon(second_rect.get_x(), 64.0),
        "and the reported source follows the frame index, got {second_rect:?}"
    );
}

#[test]
fn a_sprite_frame_carries_its_source_and_duration() {
    let sprite_frame: SpriteFrame = frame(32.0, 0.125);
    assert!(
        epsilon(sprite_frame.get_source().get_x(), 32.0),
        "the source rect round-trips"
    );
    assert!(
        epsilon(sprite_frame.get_duration(), 0.125),
        "and so does the duration"
    );
}

#[test]
fn a_sprite_animation_carries_its_name_frames_and_mode() {
    let sprite_animation: SpriteAnimation = animation(
        vec![frame(0.0, 0.1), frame(8.0, 0.2)],
        AnimationMode::PingPong,
    );
    assert_eq!(sprite_animation.get_name(), "walk", "the name round-trips");
    assert_eq!(
        sprite_animation.get_frames().len(),
        2,
        "both frames are held"
    );
    assert_eq!(
        sprite_animation.get_mode(),
        AnimationMode::PingPong,
        "and the mode sticks"
    );
}

#[test]
fn the_animation_state_enum_has_three_distinct_states() {
    let states: Vec<AnimationState> = vec![
        AnimationState::Playing,
        AnimationState::Paused,
        AnimationState::Finished,
    ];
    assert_eq!(
        states.len(),
        3,
        "a sprite animation is in one of three states"
    );
    for (index, state) in states.iter().enumerate() {
        for other in states.iter().skip(index + 1) {
            assert_ne!(state, other, "the states must be distinguishable");
        }
    }
}
