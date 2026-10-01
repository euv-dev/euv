use super::*;

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
fn a_usable_cell_size_is_kept_exactly_as_requested() {
    let grid: SpatialHashGrid2D = SpatialHashGrid2D::create(12.5);
    assert_eq!(
        grid.get_cell_size(),
        12.5,
        "a valid cell size must be honoured without adjustment"
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
