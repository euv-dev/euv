use super::*;

fn epsilon(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 1e-9
}

fn box_at(x: f64, y: f64) -> (Vector2D, Vector2D) {
    (Vector2D::new(x, y), Vector2D::new(x + 10.0, y + 10.0))
}

fn sorted(items: &[usize]) -> Vec<usize> {
    let mut out: Vec<usize> = items.to_vec();
    out.sort_unstable();
    out
}

#[test]
fn empty_grid_query_returns_no_candidates() {
    let grid: SpatialHashGrid2D = SpatialHashGrid2D::with_default_size();
    let (min, max) = box_at(0.0, 0.0);
    let found: Vec<usize> = grid.query(min, max);
    assert!(
        found.is_empty(),
        "a grid with no insertions must return no candidates"
    );
}

#[test]
fn inserted_body_is_found_by_a_query_over_its_own_box() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(32.0);
    let (min, max) = box_at(100.0, 200.0);
    grid.insert(7, min, max);
    let found: Vec<usize> = grid.query(min, max);
    assert_eq!(
        found,
        vec![7],
        "a query over the inserted box must return that body"
    );
}

#[test]
fn a_fresh_2d_grid_has_the_requested_cell_size() {
    let grid: SpatialHashGrid2D = SpatialHashGrid2D::create(32.0);
    assert_eq!(
        grid.get_cell_size(),
        32.0,
        "the cell size is stored as given"
    );
}

#[test]
fn query_misses_a_body_in_a_distant_cell() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(32.0);
    let (min, max) = box_at(0.0, 0.0);
    grid.insert(1, min, max);
    let (far_min, far_max) = box_at(5000.0, 5000.0);
    let found: Vec<usize> = grid.query(far_min, far_max);
    assert!(
        found.is_empty(),
        "a query far from the only insertion must return nothing"
    );
}

#[test]
fn body_spanning_several_cells_is_reported_once() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(4.0);
    let min: Vector2D = Vector2D::new(0.0, 0.0);
    let max: Vector2D = Vector2D::new(40.0, 40.0);
    grid.insert(3, min, max);
    let found: Vec<usize> = grid.query(min, max);
    assert_eq!(
        found,
        vec![3],
        "a body covering many cells must still be deduplicated to one hit"
    );
}

#[test]
fn query_into_agrees_with_query_and_resets_the_caller_buffers() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(32.0);
    for index in 0..5usize {
        let (min, max) = box_at(index as f64 * 40.0, 0.0);
        grid.insert(index, min, max);
    }
    let (min, max) = box_at(0.0, 0.0);
    let expected: Vec<usize> = grid.query(min, max);
    let mut out: Vec<usize> = vec![99, 98];
    let mut seen: HashSet<usize> = HashSet::new();
    seen.insert(12345);
    grid.query_into(min, max, &mut out, &mut seen);
    assert_eq!(
        sorted(&out),
        sorted(&expected),
        "query_into must return the same candidate set as query"
    );
    assert!(
        !seen.contains(&12345),
        "query_into must clear the caller scratch set"
    );
    assert!(
        !out.contains(&99) && !out.contains(&98),
        "query_into must clear the caller output buffer before filling it"
    );
}

#[test]
fn clear_empties_the_grid_and_leaves_it_reusable() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(32.0);
    let (min, max) = box_at(10.0, 10.0);
    grid.insert(2, min, max);
    grid.clear();
    assert!(
        grid.query(min, max).is_empty(),
        "clear must remove every insertion"
    );
    grid.insert(5, min, max);
    assert_eq!(
        grid.query(min, max),
        vec![5],
        "the grid must accept fresh insertions after a clear"
    );
}

#[test]
fn clear_reclaims_the_cells_a_body_walked_away_from() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(4.0);
    let (near_min, near_max) = box_at(0.0, 0.0);
    for index in 0..4usize {
        grid.insert(index, near_min, near_max);
    }
    grid.clear();
    let retained: usize = grid.get_cells().len();
    let (far_min, far_max) = box_at(10000.0, 10000.0);
    for index in 0..4usize {
        grid.insert(index, far_min, far_max);
    }
    grid.clear();
    let after_walk: usize = grid.get_cells().len();
    assert_eq!(
        retained, after_walk,
        "a body that leaves its cells must not leave those cells behind forever"
    );
}

#[test]
fn clear_keeps_occupied_cells_so_a_stationary_body_reuses_its_buffer() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(32.0);
    let (min, max) = box_at(0.0, 0.0);
    grid.insert(0, min, max);
    grid.clear();
    let occupied: usize = grid.get_cells().len();
    assert_eq!(
        occupied, 1,
        "a cell that still holds a body must survive clear so its Vec is reused"
    );
    grid.insert(0, min, max);
    assert_eq!(
        grid.query(min, max),
        vec![0],
        "a stationary body must still be found after repeated clear and reinsert"
    );
}

#[test]
fn a_degenerate_cell_size_falls_back_to_the_default_instead_of_exploding_the_column_range() {
    let default_size: f64 = SpatialHashGrid2D::with_default_size().get_cell_size();
    let grid: SpatialHashGrid2D = SpatialHashGrid2D::create(0.0);
    assert_eq!(
        grid.get_cell_size(),
        default_size,
        "a zero cell size must fall back to the documented default, not to EPSILON"
    );
    let mut grid: SpatialHashGrid2D = grid;
    let (min, max) = box_at(1.0, 1.0);
    grid.insert(1, min, max);
    assert_eq!(
        grid.query(min, max),
        vec![1],
        "the grid must stay usable after a degenerate cell size"
    );
}

#[test]
fn a_zero_cell_size_is_clamped_away_from_division_by_zero() {
    let grid: SpatialHashGrid2D = SpatialHashGrid2D::create(0.0);
    assert!(
        grid.get_cell_size() > 0.0,
        "a zero cell size must be lifted off zero, got {}",
        grid.get_cell_size()
    );
}

#[test]
fn a_negative_cell_size_is_clamped_to_a_positive_one() {
    let grid: SpatialHashGrid2D = SpatialHashGrid2D::create(-10.0);
    assert!(
        grid.get_cell_size() > 0.0,
        "a negative cell size must be clamped positive, got {}",
        grid.get_cell_size()
    );
}

#[test]
fn the_default_2d_grid_uses_the_documented_default_cell_size() {
    let explicit: SpatialHashGrid2D = SpatialHashGrid2D::create(64.0);
    let default: SpatialHashGrid2D = SpatialHashGrid2D::with_default_size();
    assert_eq!(
        default.get_cell_size(),
        explicit.get_cell_size(),
        "the default constructor uses a 64-unit cell"
    );
}

#[test]
fn an_empty_2d_grid_finds_nothing() {
    let grid: SpatialHashGrid2D = SpatialHashGrid2D::create(64.0);
    let observed: Vec<usize> = grid.query(Vector2D::new(0.0, 0.0), Vector2D::new(10.0, 10.0));
    assert!(
        observed.is_empty(),
        "a grid with no inserts returns nothing"
    );
}

#[test]
fn a_query_finds_a_body_inserted_in_the_same_cell() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(64.0);
    grid.insert(7, Vector2D::new(10.0, 10.0), Vector2D::new(20.0, 20.0));
    let observed: Vec<usize> = grid.query(Vector2D::new(0.0, 0.0), Vector2D::new(64.0, 64.0));
    assert_eq!(observed, vec![7], "the inserted index comes back");
}

#[test]
fn a_query_ignores_a_body_in_a_distant_cell() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(64.0);
    grid.insert(
        1,
        Vector2D::new(1000.0, 1000.0),
        Vector2D::new(1010.0, 1010.0),
    );
    let observed: Vec<usize> = grid.query(Vector2D::new(0.0, 0.0), Vector2D::new(64.0, 64.0));
    assert!(
        observed.is_empty(),
        "a body in another cell must not be a candidate"
    );
}

#[test]
fn a_body_spanning_many_cells_is_found_from_each_covered_cell() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(10.0);
    grid.insert(3, Vector2D::new(0.0, 0.0), Vector2D::new(35.0, 5.0));
    let from_start: Vec<usize> = grid.query(Vector2D::new(0.0, 0.0), Vector2D::new(5.0, 5.0));
    let from_far_end: Vec<usize> = grid.query(Vector2D::new(30.0, 0.0), Vector2D::new(35.0, 5.0));
    assert_eq!(
        from_start,
        vec![3],
        "the body is reachable at its near edge"
    );
    assert_eq!(
        from_far_end,
        vec![3],
        "the body is reachable at its far edge"
    );
}

#[test]
fn a_body_spanning_many_cells_appears_only_once_in_a_wide_query() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(10.0);
    grid.insert(3, Vector2D::new(0.0, 0.0), Vector2D::new(35.0, 5.0));
    let observed: Vec<usize> = grid.query(Vector2D::new(0.0, 0.0), Vector2D::new(40.0, 10.0));
    assert_eq!(
        observed,
        vec![3],
        "a wide query spanning the body's cells must deduplicate it"
    );
}

#[test]
fn a_query_returns_every_overlapping_body() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(64.0);
    grid.insert(1, Vector2D::new(0.0, 0.0), Vector2D::new(5.0, 5.0));
    grid.insert(2, Vector2D::new(6.0, 6.0), Vector2D::new(7.0, 7.0));
    grid.insert(3, Vector2D::new(500.0, 500.0), Vector2D::new(501.0, 501.0));
    let mut observed: Vec<usize> = grid.query(Vector2D::new(0.0, 0.0), Vector2D::new(64.0, 64.0));
    observed.sort_unstable();
    assert_eq!(observed, vec![1, 2], "only the two local bodies are found");
}

#[test]
fn negative_coordinates_map_into_negative_cells() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(64.0);
    grid.insert(9, Vector2D::new(-10.0, -10.0), Vector2D::new(-5.0, -5.0));
    let nearby: Vec<usize> = grid.query(Vector2D::new(-64.0, -64.0), Vector2D::new(0.0, 0.0));
    let far: Vec<usize> = grid.query(Vector2D::new(0.0, 0.0), Vector2D::new(64.0, 64.0));
    assert_eq!(nearby, vec![9], "negative space is addressable");
    assert!(far.is_empty(), "the positive side stays empty");
}

#[test]
fn a_body_just_past_a_cell_boundary_lands_in_the_next_cell() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(64.0);
    grid.insert(1, Vector2D::new(0.0, 0.0), Vector2D::new(5.0, 5.0));
    grid.insert(2, Vector2D::new(70.0, 0.0), Vector2D::new(75.0, 5.0));
    let first_cell: Vec<usize> = grid.query(Vector2D::new(0.0, 0.0), Vector2D::new(63.0, 63.0));
    let second_cell: Vec<usize> = grid.query(Vector2D::new(64.0, 0.0), Vector2D::new(127.0, 63.0));
    assert_eq!(
        first_cell,
        vec![1],
        "the body at the origin is in cell zero"
    );
    assert_eq!(
        second_cell,
        vec![2],
        "a body past the 64-unit boundary is not in cell zero"
    );
}

#[test]
fn a_negative_or_non_finite_cell_size_is_rejected_the_same_way() {
    let default_two: f64 = SpatialHashGrid2D::with_default_size().get_cell_size();
    let default_three: f64 = SpatialHashGrid3D::with_default_size().get_cell_size();
    for requested in [-1.0, f64::NAN, f64::INFINITY] {
        let grid: SpatialHashGrid2D = SpatialHashGrid2D::create(requested);
        assert_eq!(
            grid.get_cell_size(),
            default_two,
            "cell size {requested} must fall back to the default"
        );
        let three_dimensional: SpatialHashGrid3D = SpatialHashGrid3D::create(requested);
        assert_eq!(
            three_dimensional.get_cell_size(),
            default_three,
            "the 3D grid must reject cell size {requested} the same way"
        );
    }
}

#[test]
fn clear_empties_every_cell_of_the_2d_grid() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(64.0);
    grid.insert(1, Vector2D::new(0.0, 0.0), Vector2D::new(5.0, 5.0));
    grid.insert(2, Vector2D::new(6.0, 6.0), Vector2D::new(7.0, 7.0));
    grid.clear();
    let observed: Vec<usize> = grid.query(Vector2D::new(0.0, 0.0), Vector2D::new(64.0, 64.0));
    assert!(observed.is_empty(), "clear must drop every candidate");
}

#[test]
fn a_2d_grid_is_reusable_after_a_clear() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(64.0);
    grid.insert(1, Vector2D::new(0.0, 0.0), Vector2D::new(5.0, 5.0));
    grid.clear();
    grid.insert(2, Vector2D::new(0.0, 0.0), Vector2D::new(5.0, 5.0));
    let observed: Vec<usize> = grid.query(Vector2D::new(0.0, 0.0), Vector2D::new(64.0, 64.0));
    assert_eq!(
        observed,
        vec![2],
        "the fresh insert pass is all that is left"
    );
}

#[test]
fn query_into_matches_query_for_the_2d_grid() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(10.0);
    grid.insert(4, Vector2D::new(0.0, 0.0), Vector2D::new(25.0, 5.0));
    let allocated: Vec<usize> = grid.query(Vector2D::new(0.0, 0.0), Vector2D::new(30.0, 10.0));
    let mut out: Vec<usize> = Vec::new();
    let mut seen: HashSet<usize> = HashSet::new();
    grid.query_into(
        Vector2D::new(0.0, 0.0),
        Vector2D::new(30.0, 10.0),
        &mut out,
        &mut seen,
    );
    assert_eq!(
        out, allocated,
        "the reuse buffer matches the allocating path"
    );
}

#[test]
fn query_into_clears_the_buffers_it_is_handed() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(64.0);
    grid.insert(1, Vector2D::new(0.0, 0.0), Vector2D::new(5.0, 5.0));
    let mut out: Vec<usize> = vec![111, 222];
    let mut seen: HashSet<usize> = HashSet::from([333]);
    grid.query_into(
        Vector2D::new(500.0, 500.0),
        Vector2D::new(600.0, 600.0),
        &mut out,
        &mut seen,
    );
    assert!(out.is_empty(), "stale entries must be dropped first");
    assert!(seen.is_empty(), "the scratch set must be dropped first too");
}

#[test]
fn query_into_can_be_called_repeatedly_with_the_same_buffers() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(64.0);
    grid.insert(1, Vector2D::new(0.0, 0.0), Vector2D::new(5.0, 5.0));
    grid.insert(2, Vector2D::new(0.0, 0.0), Vector2D::new(6.0, 6.0));
    let mut out: Vec<usize> = Vec::new();
    let mut seen: HashSet<usize> = HashSet::new();
    for _ in 0..3 {
        grid.query_into(
            Vector2D::new(0.0, 0.0),
            Vector2D::new(64.0, 64.0),
            &mut out,
            &mut seen,
        );
        let mut sorted: Vec<usize> = out.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![1, 2], "every pass returns the same two bodies");
    }
}

#[test]
fn a_usable_cell_size_is_kept_exactly_as_requested() {
    let grid: SpatialHashGrid2D = SpatialHashGrid2D::create(12.5);
    assert_eq!(
        grid.get_cell_size(),
        12.5,
        "a valid cell size must be honoured without adjustment"
    );
}

#[test]
fn a_fresh_3d_grid_has_the_requested_cell_size() {
    let grid: SpatialHashGrid3D = SpatialHashGrid3D::create(16.0);
    assert_eq!(
        grid.get_cell_size(),
        16.0,
        "the cell size is stored as given"
    );
}

#[test]
fn grid_query_never_misses_a_body_a_linear_scan_would_overlap() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(24.0);
    let mut boxes: Vec<(Vector2D, Vector2D)> = Vec::new();
    for index in 0..60usize {
        let offset: f64 = index as f64 * 3.0;
        let (min, max) = box_at(offset, offset * 0.5);
        grid.insert(index, min, max);
        boxes.push((min, max));
    }
    let (query_min, query_max) = box_at(40.0, 20.0);
    let overlaps: Vec<usize> = boxes
        .iter()
        .enumerate()
        .filter(|(_, (min, max))| {
            min.get_x() <= query_max.get_x()
                && query_min.get_x() <= max.get_x()
                && min.get_y() <= query_max.get_y()
                && query_min.get_y() <= max.get_y()
        })
        .map(|(index, _)| index)
        .collect();
    let candidates: Vec<usize> = grid.query(query_min, query_max);
    assert!(
        !overlaps.is_empty(),
        "the fixture must actually produce overlapping bodies"
    );
    for index in overlaps {
        assert!(
            candidates.contains(&index),
            "body {index} overlaps the query box and must not be culled"
        );
    }
}

#[test]
fn grid_query_culls_bodies_in_a_distant_part_of_the_world() {
    let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::create(24.0);
    let (near_min, near_max) = box_at(0.0, 0.0);
    let (far_min, far_max) = box_at(100_000.0, 100_000.0);
    grid.insert(1, near_min, near_max);
    grid.insert(2, far_min, far_max);
    let found: Vec<usize> = grid.query(near_min, near_max);
    assert!(found.contains(&1), "the nearby body must be returned");
    assert!(
        !found.contains(&2),
        "the far away body must be culled by the broad phase"
    );
}

#[test]
fn a_zero_3d_cell_size_is_clamped_away_from_division_by_zero() {
    let grid: SpatialHashGrid3D = SpatialHashGrid3D::create(0.0);
    assert!(
        grid.get_cell_size() > 0.0,
        "a zero cell size must be lifted off zero"
    );
}

#[test]
fn the_default_3d_grid_uses_the_documented_default_cell_size() {
    let explicit: SpatialHashGrid3D = SpatialHashGrid3D::create(64.0);
    let default: SpatialHashGrid3D = SpatialHashGrid3D::with_default_size();
    assert_eq!(
        default.get_cell_size(),
        explicit.get_cell_size(),
        "the default constructor uses a 64-unit cell"
    );
}

#[test]
fn an_empty_3d_grid_finds_nothing() {
    let grid: SpatialHashGrid3D = SpatialHashGrid3D::create(64.0);
    let observed: Vec<usize> = grid.query(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(10.0, 10.0, 10.0),
    );
    assert!(
        observed.is_empty(),
        "a grid with no inserts returns nothing"
    );
}

#[test]
fn three_dimensional_grid_finds_a_body_along_the_z_axis() {
    let mut grid: SpatialHashGrid3D = SpatialHashGrid3D::create(32.0);
    let min: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let max: Vector3D = Vector3D::new(10.0, 10.0, 10.0);
    grid.insert(4, min, max);
    assert_eq!(
        grid.query(min, max),
        vec![4],
        "a 3D query over the inserted box must return that body"
    );
}

#[test]
fn a_3d_query_finds_a_body_in_the_same_cell() {
    let mut grid: SpatialHashGrid3D = SpatialHashGrid3D::create(64.0);
    grid.insert(
        5,
        Vector3D::new(10.0, 10.0, 10.0),
        Vector3D::new(20.0, 20.0, 20.0),
    );
    let observed: Vec<usize> = grid.query(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(64.0, 64.0, 64.0),
    );
    assert_eq!(observed, vec![5], "the inserted index comes back");
}

#[test]
fn a_3d_query_is_separated_along_the_z_axis() {
    let mut grid: SpatialHashGrid3D = SpatialHashGrid3D::create(64.0);
    grid.insert(
        1,
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(5.0, 5.0, 5.0),
    );
    let same_layer: Vec<usize> = grid.query(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(64.0, 64.0, 64.0),
    );
    let other_layer: Vec<usize> = grid.query(
        Vector3D::new(0.0, 0.0, 200.0),
        Vector3D::new(64.0, 64.0, 264.0),
    );
    assert_eq!(same_layer, vec![1], "the body lives on the low layer");
    assert!(
        other_layer.is_empty(),
        "a query on another z layer must not see it"
    );
}

#[test]
fn a_3d_body_spanning_many_cells_is_found_only_once_per_query() {
    let mut grid: SpatialHashGrid3D = SpatialHashGrid3D::create(10.0);
    grid.insert(
        2,
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(35.0, 5.0, 5.0),
    );
    let observed: Vec<usize> = grid.query(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(40.0, 10.0, 10.0),
    );
    assert_eq!(
        observed,
        vec![2],
        "a query spanning the body's cells must deduplicate it"
    );
}

#[test]
fn three_dimensional_grid_deduplicates_a_body_spanning_many_cells() {
    let mut grid: SpatialHashGrid3D = SpatialHashGrid3D::create(4.0);
    let min: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let max: Vector3D = Vector3D::new(30.0, 30.0, 30.0);
    grid.insert(6, min, max);
    assert_eq!(
        grid.query(min, max),
        vec![6],
        "a 3D body covering many cells must be deduplicated to one hit"
    );
}

#[test]
fn three_dimensional_grid_clear_empties_and_reclaims_cells() {
    let mut grid: SpatialHashGrid3D = SpatialHashGrid3D::create(4.0);
    let near_min: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let near_max: Vector3D = Vector3D::new(6.0, 6.0, 6.0);
    grid.insert(1, near_min, near_max);
    grid.clear();
    let retained: usize = grid.get_cells().len();
    let far_min: Vector3D = Vector3D::new(5000.0, 5000.0, 5000.0);
    let far_max: Vector3D = Vector3D::new(5006.0, 5006.0, 5006.0);
    grid.insert(2, far_min, far_max);
    grid.clear();
    assert_eq!(
        grid.get_cells().len(),
        retained,
        "abandoned 3D cells must be reclaimed rather than accumulated"
    );
    assert!(
        grid.query(far_min, far_max).is_empty(),
        "clear must empty the 3D grid"
    );
}

#[test]
fn clear_empties_every_cell_of_the_3d_grid() {
    let mut grid: SpatialHashGrid3D = SpatialHashGrid3D::create(64.0);
    grid.insert(
        1,
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(5.0, 5.0, 5.0),
    );
    grid.clear();
    let observed: Vec<usize> = grid.query(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(64.0, 64.0, 64.0),
    );
    assert!(observed.is_empty(), "clear must drop every candidate");
}

#[test]
fn a_3d_grid_is_reusable_after_a_clear() {
    let mut grid: SpatialHashGrid3D = SpatialHashGrid3D::create(64.0);
    grid.insert(
        1,
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(5.0, 5.0, 5.0),
    );
    grid.clear();
    grid.insert(
        2,
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(5.0, 5.0, 5.0),
    );
    let observed: Vec<usize> = grid.query(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(64.0, 64.0, 64.0),
    );
    assert_eq!(
        observed,
        vec![2],
        "the fresh insert pass is all that is left"
    );
}

#[test]
fn three_dimensional_query_into_resets_the_caller_buffers() {
    let mut grid: SpatialHashGrid3D = SpatialHashGrid3D::create(32.0);
    let min: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let max: Vector3D = Vector3D::new(8.0, 8.0, 8.0);
    grid.insert(3, min, max);
    let mut out: Vec<usize> = vec![42];
    let mut seen: HashSet<usize> = HashSet::new();
    seen.insert(4242);
    grid.query_into(min, max, &mut out, &mut seen);
    assert_eq!(out, vec![3], "query_into must overwrite the output buffer");
    assert!(
        !seen.contains(&4242),
        "query_into must clear the caller scratch set"
    );
}

#[test]
fn boxes_overlap_is_true_for_overlapping_touching_and_contained_boxes() {
    let a: AABB3D = AABB3D::new(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(10.0, 10.0, 10.0),
    );
    let overlapping: AABB3D = AABB3D::new(
        Vector3D::new(5.0, 0.0, 0.0),
        Vector3D::new(15.0, 10.0, 10.0),
    );
    let touching: AABB3D = AABB3D::new(
        Vector3D::new(10.0, 0.0, 0.0),
        Vector3D::new(20.0, 10.0, 10.0),
    );
    let contained: AABB3D = AABB3D::new(Vector3D::new(2.0, 2.0, 2.0), Vector3D::new(4.0, 4.0, 4.0));
    assert!(
        AABB3D::broad_phase(a, overlapping),
        "partial overlap counts"
    );
    assert!(
        AABB3D::broad_phase(a, touching),
        "flush faces count as touching"
    );
    assert!(
        AABB3D::broad_phase(a, contained),
        "a box inside another counts"
    );
    assert!(
        AABB3D::broad_phase(contained, a),
        "and the order does not matter"
    );
}

#[test]
fn query_into_matches_query_for_the_3d_grid() {
    let mut grid: SpatialHashGrid3D = SpatialHashGrid3D::create(10.0);
    grid.insert(
        4,
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(25.0, 5.0, 5.0),
    );
    let allocated: Vec<usize> = grid.query(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(30.0, 10.0, 10.0),
    );
    let mut out: Vec<usize> = Vec::new();
    let mut seen: HashSet<usize> = HashSet::new();
    grid.query_into(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(30.0, 10.0, 10.0),
        &mut out,
        &mut seen,
    );
    assert_eq!(
        out, allocated,
        "the reuse buffer matches the allocating path"
    );
}

#[test]
fn boxes_overlap_is_false_for_disjoint_and_merely_adjacent_faces() {
    let a: AABB3D = AABB3D::new(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(10.0, 10.0, 10.0),
    );
    let apart: AABB3D = AABB3D::new(
        Vector3D::new(50.0, 0.0, 0.0),
        Vector3D::new(60.0, 10.0, 10.0),
    );
    let past: AABB3D = AABB3D::new(
        Vector3D::new(11.0, 0.0, 0.0),
        Vector3D::new(20.0, 10.0, 10.0),
    );
    assert!(
        !AABB3D::broad_phase(a, apart),
        "well separated boxes do not"
    );
    assert!(
        !AABB3D::broad_phase(a, past),
        "a gap of even one unit culls the pair, and it is not symmetric about the diagonal either"
    );
    let touching: AABB3D = AABB3D::new(
        Vector3D::new(10.0, 0.0, 0.0),
        Vector3D::new(20.0, 10.0, 10.0),
    );
    assert!(
        AABB3D::broad_phase(a, touching),
        "a shared face still counts"
    );
}

#[test]
fn a_fresh_quadtree_reports_the_box_it_was_bounded_by() {
    let tree: QuadTree2D =
        QuadTree2D::create(Vector2D::new(-10.0, -10.0), Vector2D::new(10.0, 10.0), 4, 8);

    let (min, max): (Vector2D, Vector2D) = tree.bounds();

    assert_eq!(
        (min.get_x(), min.get_y()),
        (-10.0, -10.0),
        "the lower corner of the region the tree was built for"
    );
    assert!(
        max.get_x() > 10.0 && max.get_y() > 10.0,
        "the upper corner is nudged past the requested maximum by a small epsilon, because a \
         zero-width region would make the split test compare equal on both sides forever"
    );
}

#[test]
fn with_half_extent_squares_the_region_around_the_origin() {
    let tree: QuadTree2D = QuadTree2D::with_half_extent(256.0);
    let (min, max) = tree.bounds();
    assert!(
        epsilon(min.get_x(), -256.0) && epsilon(min.get_y(), -256.0),
        "min is -half extent"
    );
    assert!(
        (max.get_x() - 256.0).abs() < 1e-5 && (max.get_y() - 256.0).abs() < 1e-5,
        "max is +half extent, widened by EPSILON so subdivision always makes progress, got {max:?}"
    );
    let width: f64 = max.get_x() - min.get_x();
    assert!(
        (width - 512.0).abs() < 1e-5,
        "the region is twice the half extent wide plus the EPSILON widening, got {width}"
    );
}

#[test]
fn with_half_extent_absolutes_its_argument_and_survives_a_degenerate_one() {
    let positive: QuadTree2D = QuadTree2D::with_half_extent(-100.0);
    let (min, _max) = positive.bounds();
    assert!(
        min.get_x() <= 0.0,
        "a negative half extent is made positive, got {}",
        min.get_x()
    );
    for degenerate in [0.0, f64::NAN] {
        let tree: QuadTree2D = QuadTree2D::with_half_extent(degenerate);
        let (low, high) = tree.bounds();
        assert!(
            high.get_x() > low.get_x(),
            "a degenerate half extent must still yield a non-empty region, got {low:?}..{high:?}"
        );
    }
}

#[test]
fn a_quadtree_entry_round_trips_its_index_and_box() {
    let entry: QuadTreeEntry2D =
        QuadTreeEntry2D::new(5, Vector2D::new(1.0, 2.0), Vector2D::new(3.0, 4.0));
    assert_eq!(entry.get_index(), 5, "the body index round-trips");
    assert!(
        epsilon(entry.get_min().get_x(), 1.0),
        "the min corner round-trips"
    );
    assert!(
        epsilon(entry.get_max().get_y(), 4.0),
        "the max corner round-trips"
    );
}

#[test]
fn quad_tree_boxes_overlap_is_true_for_a_shared_region() {
    assert!(
        QuadTree2D::boxes_overlap(
            Vector2D::zero(),
            Vector2D::new(10.0, 10.0),
            Vector2D::new(5.0, 5.0),
            Vector2D::new(15.0, 15.0),
        ),
        "two boxes sharing a corner-sized region overlap"
    );
}

#[test]
fn quad_tree_boxes_overlap_is_true_when_the_edges_merely_touch() {
    assert!(
        QuadTree2D::boxes_overlap(
            Vector2D::zero(),
            Vector2D::new(10.0, 10.0),
            Vector2D::new(10.0, 0.0),
            Vector2D::new(20.0, 10.0),
        ),
        "the comparison is inclusive, so a shared edge counts as overlap"
    );
    assert!(
        QuadTree2D::boxes_overlap(
            Vector2D::zero(),
            Vector2D::new(10.0, 10.0),
            Vector2D::new(10.0, 10.0),
            Vector2D::new(20.0, 20.0),
        ),
        "a single shared corner counts too"
    );
    assert!(
        QuadTree2D::boxes_overlap(
            Vector2D::new(10.0, 0.0),
            Vector2D::new(20.0, 10.0),
            Vector2D::new(0.0, 0.0),
            Vector2D::new(10.0, 10.0),
        ),
        "the other side of the x comparison is inclusive too: here the first box's \
         minimum edge is exactly the second box's maximum edge"
    );
}

#[test]
fn quad_tree_boxes_overlap_is_false_when_a_gap_remains_on_any_axis() {
    assert!(
        !QuadTree2D::boxes_overlap(
            Vector2D::zero(),
            Vector2D::new(10.0, 10.0),
            Vector2D::new(11.0, 0.0),
            Vector2D::new(20.0, 10.0),
        ),
        "separated along x"
    );
    assert!(
        !QuadTree2D::boxes_overlap(
            Vector2D::zero(),
            Vector2D::new(10.0, 10.0),
            Vector2D::new(0.0, -12.0),
            Vector2D::new(10.0, -1.0),
        ),
        "separated along y by a gap of one unit"
    );
    assert!(
        QuadTree2D::boxes_overlap(
            Vector2D::zero(),
            Vector2D::new(10.0, 10.0),
            Vector2D::new(0.0, -11.0),
            Vector2D::new(10.0, 0.0),
        ),
        "the same boxes meeting exactly at y=0 still overlap, because the \
         comparison is inclusive on every axis"
    );
}

#[test]
fn quad_tree_boxes_overlap_is_true_when_one_box_swallows_the_other() {
    assert!(
        QuadTree2D::boxes_overlap(
            Vector2D::new(-100.0, -100.0),
            Vector2D::new(100.0, 100.0),
            Vector2D::new(1.0, 1.0),
            Vector2D::new(2.0, 2.0),
        ),
        "containment is overlap"
    );
}

#[test]
fn a_quad_tree_node_can_be_built_directly_with_its_bounds_depth_and_child_slots() {
    let children: QuadTreeChildren2D = [usize::MAX, usize::MAX, usize::MAX, usize::MAX];
    let node: QuadTreeNode2D = QuadTreeNode2D::new(
        Vector2D::new(-10.0, -10.0),
        Vector2D::new(10.0, 10.0),
        0,
        children,
        true,
        false,
        Vec::new(),
    );
    assert_eq!(node.get_min().get_x(), -10.0);
    assert_eq!(node.get_max().get_y(), 10.0);
    assert_eq!(node.get_depth(), 0);
    assert_eq!(
        node.get_children(),
        &children,
        "the unset sentinel round-trips"
    );
    assert!(node.get_leaf());
    assert!(!node.get_loose());
}

#[test]
fn a_quad_tree_node_is_marked_loose_when_it_holds_an_entry_it_does_not_contain() {
    let empty: QuadTreeChildren2D = [usize::MAX, usize::MAX, usize::MAX, usize::MAX];
    let tight: QuadTreeNode2D = QuadTreeNode2D::new(
        Vector2D::zero(),
        Vector2D::new(4.0, 4.0),
        0,
        empty,
        true,
        false,
        Vec::new(),
    );
    let loose: QuadTreeNode2D = QuadTreeNode2D::new(
        Vector2D::zero(),
        Vector2D::new(4.0, 4.0),
        0,
        empty,
        true,
        true,
        Vec::new(),
    );
    assert_ne!(
        tight, loose,
        "the loose flag is part of the node's identity"
    );
    assert!(
        !tight.get_loose(),
        "no straddling entry, so the region prune stays sound"
    );
    assert!(
        loose.get_loose(),
        "a straddling entry forces the exact box test"
    );
}

#[test]
fn two_quad_tree_nodes_with_the_same_fields_compare_equal() {
    let first: QuadTreeNode2D = QuadTreeNode2D::new(
        Vector2D::zero(),
        Vector2D::new(4.0, 4.0),
        2,
        [0, 0, 0, 0],
        true,
        false,
        Vec::new(),
    );
    let same: QuadTreeNode2D = QuadTreeNode2D::new(
        Vector2D::zero(),
        Vector2D::new(4.0, 4.0),
        2,
        [0, 0, 0, 0],
        true,
        false,
        Vec::new(),
    );
    let deeper: QuadTreeNode2D = QuadTreeNode2D::new(
        Vector2D::zero(),
        Vector2D::new(4.0, 4.0),
        3,
        [0, 0, 0, 0],
        true,
        false,
        Vec::new(),
    );
    assert_eq!(first, same, "the node is a plain value type");
    assert_ne!(first, deeper, "depth is part of the identity");
}

#[test]
fn a_quadtree_built_from_swapped_corners_still_reports_min_before_max() {
    let tree: QuadTree2D =
        QuadTree2D::create(Vector2D::new(10.0, 10.0), Vector2D::new(-10.0, -10.0), 4, 8);

    let (min, max): (Vector2D, Vector2D) = tree.bounds();

    assert!(
        min.get_x() < max.get_x() && min.get_y() < max.get_y(),
        "the two corners are ordered on construction, so a caller that passed them the wrong \
         way round still gets a usable box instead of an inverted one that rejects every insert"
    );
}
