use super::*;

const EPSILON: f64 = 1e-9;

#[test]
fn nine_slice_source_rects_cover_full_source() {
    let insets: NineSliceInsets = NineSliceInsets::new(1.0, 1.0, 1.0, 1.0);
    let rects: Vec<Rect> =
        insets.source_rects(Rect::new(0.0, 0.0, 3.0, 3.0)).to_vec();
    for rect in rects.iter() {
        assert!(
            (rect.get_width() - 1.0).abs() < EPSILON
                && (rect.get_height() - 1.0).abs() < EPSILON,
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
        let uv: UvRect =
            SpriteAtlas::normalize_uv(Rect::new(16.0, 16.0, 8.0, 8.0), size.0, size.1);
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
        regions.is_empty() && regions.len() == 0,
        "expected a fresh index to be empty, got len {}",
        regions.len(),
    );
    let hero: Rect = Rect::new(0.0, 0.0, 16.0, 32.0);
    regions.insert("hero", hero);
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
        regions.get("hero") == Some(hero),
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
        names == vec!["coin".to_string(), "hero".to_string()],
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
