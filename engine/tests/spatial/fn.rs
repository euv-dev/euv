use super::*;

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
fn a_fresh_3d_grid_has_the_requested_cell_size() {
    let grid: SpatialHashGrid3D = SpatialHashGrid3D::create(16.0);
    assert_eq!(
        grid.get_cell_size(),
        16.0,
        "the cell size is stored as given"
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
