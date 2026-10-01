use euv_engine::*;

use std::collections::HashSet;

fn sorted(items: &[usize]) -> Vec<usize> {
    let mut copy: Vec<usize> = items.to_vec();
    copy.sort_unstable();
    copy
}

fn tree(bounds: f64, capacity: usize, depth: usize) -> QuadTree2D {
    QuadTree2D::create(
        Vector2D::new(-bounds, -bounds),
        Vector2D::new(bounds, bounds),
        capacity,
        depth,
    )
}

fn brute_force(rects: &[(Vector2D, Vector2D)], min: Vector2D, max: Vector2D) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    for (index, (a, b)) in rects.iter().enumerate() {
        let hit: bool = a.get_x() <= max.get_x()
            && min.get_x() <= b.get_x()
            && a.get_y() <= max.get_y()
            && min.get_y() <= b.get_y();
        if hit {
            out.push(index);
        }
    }
    sorted(&out)
}

#[test]
fn empty_tree_query_returns_nothing() {
    let tree: QuadTree2D = tree(64.0, 4, 6);
    let hit: Vec<usize> = tree.query(Vector2D::new(-10.0, -10.0), Vector2D::new(10.0, 10.0));
    assert!(
        hit.is_empty(),
        "an empty tree must return no candidates, got {:?}",
        hit
    );
    assert_eq!(tree.len(), 0, "an empty tree must report zero length");
    assert!(tree.is_empty(), "an empty tree must report is_empty");
}

#[test]
fn default_tree_is_empty_and_queryable() {
    let mut tree: QuadTree2D = QuadTree2D::default();
    assert!(tree.is_empty());
    tree.insert(7, Vector2D::new(-1.0, -1.0), Vector2D::new(1.0, 1.0));
    assert_eq!(tree.len(), 1);
    let hit: Vec<usize> = tree.query(Vector2D::new(-2.0, -2.0), Vector2D::new(2.0, 2.0));
    assert_eq!(sorted(&hit), vec![7], "default tree must be usable");
}

#[test]
fn single_body_found_by_containing_query_and_missed_by_distant_query() {
    let mut tree: QuadTree2D = tree(64.0, 4, 6);
    tree.insert(3, Vector2D::new(10.0, 20.0), Vector2D::new(12.0, 24.0));
    let inside: Vec<usize> = tree.query(Vector2D::new(0.0, 0.0), Vector2D::new(50.0, 50.0));
    assert_eq!(
        sorted(&inside),
        vec![3],
        "a query fully containing the body must return it"
    );
    let exact: Vec<usize> = tree.query(Vector2D::new(10.0, 20.0), Vector2D::new(12.0, 24.0));
    assert_eq!(sorted(&exact), vec![3], "a query equal to the box must hit");
    let away: Vec<usize> = tree.query(Vector2D::new(-60.0, -60.0), Vector2D::new(-50.0, -50.0));
    assert!(
        away.is_empty(),
        "a query far from the body must return nothing, got {:?}",
        away
    );
}

#[test]
fn four_unit_bodies_in_a_known_layout_return_exactly_their_quadrant() {
    let mut tree: QuadTree2D = tree(64.0, 1, 8);
    tree.insert(0, Vector2D::new(0.0, 0.0), Vector2D::new(1.0, 1.0));
    tree.insert(1, Vector2D::new(10.0, 0.0), Vector2D::new(11.0, 1.0));
    tree.insert(2, Vector2D::new(0.0, 10.0), Vector2D::new(1.0, 11.0));
    tree.insert(3, Vector2D::new(10.0, 10.0), Vector2D::new(11.0, 11.0));
    assert_eq!(tree.len(), 4);
    let all: Vec<usize> = tree.query(Vector2D::new(-64.0, -64.0), Vector2D::new(64.0, 64.0));
    assert_eq!(
        sorted(&all),
        vec![0, 1, 2, 3],
        "querying the whole root must return all four bodies"
    );
    let quadrant: Vec<usize> = tree.query(Vector2D::new(-1.0, -1.0), Vector2D::new(2.0, 2.0));
    assert_eq!(
        sorted(&quadrant),
        vec![0],
        "the low-low quadrant must return only the body at (0,0), got {:?}",
        quadrant
    );
    let opposite: Vec<usize> = tree.query(Vector2D::new(9.5, 9.5), Vector2D::new(11.5, 11.5));
    assert_eq!(
        sorted(&opposite),
        vec![3],
        "the high-high quadrant must return only the body at (10,10), got {:?}",
        opposite
    );
    let strip: Vec<usize> = tree.query(Vector2D::new(9.0, -1.0), Vector2D::new(12.0, 1.0));
    assert_eq!(
        sorted(&strip),
        vec![1],
        "a query beside the low row must return only the (10,0) body, got {:?}",
        strip
    );
}

#[test]
fn body_straddling_a_subdivision_boundary_is_found_from_either_side() {
    let mut tree: QuadTree2D = tree(64.0, 1, 8);
    tree.insert(0, Vector2D::new(-2.0, 1.0), Vector2D::new(2.0, 3.0));
    let left: Vec<usize> = tree.query(Vector2D::new(-2.0, 1.0), Vector2D::new(-1.0, 3.0));
    assert_eq!(
        sorted(&left),
        vec![0],
        "a query covering only the left half must still find the straddler, got {:?}",
        left
    );
    let right: Vec<usize> = tree.query(Vector2D::new(1.0, 1.0), Vector2D::new(2.0, 3.0));
    assert_eq!(
        sorted(&right),
        vec![0],
        "a query covering only the right half must still find the straddler, got {:?}",
        right
    );
    let above: Vec<usize> = tree.query(Vector2D::new(-2.0, 4.0), Vector2D::new(2.0, 5.0));
    assert!(
        above.is_empty(),
        "a query beyond the body's y range must not find it, got {:?}",
        above
    );
    let to_the_left: Vec<usize> = tree.query(Vector2D::new(-5.0, 1.0), Vector2D::new(-3.0, 3.0));
    assert!(
        to_the_left.is_empty(),
        "an exact box test must reject a query that stops short of the body, got {:?}",
        to_the_left
    );
}

#[test]
fn body_larger_than_the_root_region_is_inserted_and_found_by_every_overlapping_query() {
    let mut tree: QuadTree2D = tree(16.0, 1, 4);
    tree.insert(0, Vector2D::new(-8.0, -8.0), Vector2D::new(-7.0, -7.0));
    tree.insert(1, Vector2D::new(7.0, -8.0), Vector2D::new(8.0, -7.0));
    tree.insert(
        2,
        Vector2D::new(-1000.0, -1000.0),
        Vector2D::new(1000.0, 1000.0),
    );
    assert_eq!(tree.len(), 3, "an oversized body still counts as inserted");
    let far: Vec<usize> = tree.query(Vector2D::new(900.0, 900.0), Vector2D::new(950.0, 950.0));
    assert_eq!(
        sorted(&far),
        vec![2],
        "an oversized body must be returned by a far query that overlaps it, got {:?}",
        far
    );
    let other_corner: Vec<usize> =
        tree.query(Vector2D::new(-950.0, -950.0), Vector2D::new(-900.0, -900.0));
    assert_eq!(
        sorted(&other_corner),
        vec![2],
        "the oversized body must be found from the opposite corner too, got {:?}",
        other_corner
    );
    let centre: Vec<usize> = tree.query(Vector2D::new(-1.0, -1.0), Vector2D::new(1.0, 1.0));
    assert_eq!(
        sorted(&centre),
        vec![2],
        "the oversized body must be found by a central query, got {:?}",
        centre
    );
    let outside: Vec<usize> =
        tree.query(Vector2D::new(2000.0, 2000.0), Vector2D::new(2100.0, 2100.0));
    assert!(
        outside.is_empty(),
        "a query entirely outside the oversized body must return nothing, got {:?}",
        outside
    );
}

#[test]
fn many_small_bodies_match_a_brute_force_linear_scan_after_deep_subdivision() {
    let mut tree: QuadTree2D = tree(128.0, 1, 10);
    let mut rects: Vec<(Vector2D, Vector2D)> = Vec::new();
    for row in 0..16 {
        for col in 0..16 {
            let min: Vector2D =
                Vector2D::new(-128.0 + (col as f64) * 16.0, -128.0 + (row as f64) * 16.0);
            let max: Vector2D = Vector2D::new(min.get_x() + 1.0, min.get_y() + 1.0);
            let index: usize = rects.len();
            rects.push((min, max));
            tree.insert(index, min, max);
        }
    }
    assert_eq!(tree.len(), 256);
    assert!(
        tree.get_nodes().len() > 64,
        "256 tiny bodies at capacity 1 must force deep subdivision, got only {} nodes",
        tree.get_nodes().len()
    );
    let cells: Vec<(Vector2D, Vector2D)> = vec![
        (Vector2D::new(-128.0, -128.0), Vector2D::new(0.0, 0.0)),
        (Vector2D::new(0.0, 0.0), Vector2D::new(128.0, 128.0)),
        (Vector2D::new(-4.0, -4.0), Vector2D::new(4.0, 4.0)),
        (Vector2D::new(-1.0, -1.0), Vector2D::new(1.0, 1.0)),
        (Vector2D::new(-129.0, -129.0), Vector2D::new(129.0, 129.0)),
    ];
    for (min, max) in cells {
        let got: Vec<usize> = sorted(&tree.query(min, max));
        let want: Vec<usize> = brute_force(&rects, min, max);
        assert_eq!(
            got, want,
            "query {:?}..{:?} must match the brute-force scan",
            min, max
        );
    }
    let tile: Vec<usize> = tree.query(Vector2D::new(16.0, 16.0), Vector2D::new(32.0, 32.0));
    assert_eq!(
        sorted(&tile),
        vec![153, 154, 169, 170],
        "the 16..32 corner query must return the four unit bodies touching its four corners"
    );
    let inner: Vec<usize> = tree.query(Vector2D::new(17.5, 17.5), Vector2D::new(31.0, 31.0));
    assert!(
        inner.is_empty(),
        "a query strictly inside the gap between the corner bodies must return nothing, got {:?}",
        inner
    );
    let touching: Vec<usize> = tree.query(Vector2D::new(17.0, 17.0), Vector2D::new(31.0, 31.0));
    assert_eq!(
        sorted(&touching),
        vec![153],
        "a query whose corner exactly touches body 153 must still count as overlapping"
    );
    let column: Vec<usize> = tree.query(Vector2D::new(17.0, 16.0), Vector2D::new(32.0, 17.0));
    assert_eq!(
        sorted(&column),
        vec![153, 154],
        "a query along the 16..17 row must return exactly the two bodies at y=16"
    );
}

#[test]
fn clear_empties_the_tree_and_leaves_it_reusable() {
    let mut tree: QuadTree2D = tree(64.0, 2, 6);
    for index in 0..12 {
        let offset: f64 = (index as f64) * 3.0;
        tree.insert(
            index,
            Vector2D::new(offset, offset),
            Vector2D::new(offset + 1.0, offset + 1.0),
        );
    }
    assert_eq!(tree.len(), 12);
    tree.clear();
    assert!(tree.is_empty(), "clear must reset the body count");
    assert_eq!(tree.len(), 0, "clear must leave the tree empty");
    let stale: Vec<usize> = tree.query(Vector2D::new(-64.0, -64.0), Vector2D::new(64.0, 64.0));
    assert!(
        stale.is_empty(),
        "a query after clear must return nothing, got {:?}",
        stale
    );
    assert_eq!(
        tree.get_nodes().len(),
        1,
        "clear must collapse the arena back to the single root node"
    );
    tree.insert(99, Vector2D::new(-2.0, -2.0), Vector2D::new(2.0, 2.0));
    let reused: Vec<usize> = tree.query(Vector2D::new(-3.0, -3.0), Vector2D::new(3.0, 3.0));
    assert_eq!(
        sorted(&reused),
        vec![99],
        "the tree must accept fresh inserts after clear, got {:?}",
        reused
    );
}

#[test]
fn query_into_matches_query_and_clears_the_caller_buffers() {
    let mut tree: QuadTree2D = tree(64.0, 1, 8);
    tree.insert(0, Vector2D::new(0.0, 0.0), Vector2D::new(2.0, 2.0));
    tree.insert(1, Vector2D::new(8.0, 8.0), Vector2D::new(10.0, 10.0));
    tree.insert(2, Vector2D::new(-8.0, -8.0), Vector2D::new(-6.0, -6.0));
    let mut out: Vec<usize> = vec![42, 43];
    let mut seen: HashSet<usize> = HashSet::from([42, 43]);
    let min: Vector2D = Vector2D::new(-1.0, -1.0);
    let max: Vector2D = Vector2D::new(3.0, 3.0);
    tree.query_into(min, max, &mut out, &mut seen);
    let want: Vec<usize> = sorted(&tree.query(min, max));
    assert_eq!(
        sorted(&out),
        want,
        "query_into must agree with query for {:?}..{:?}",
        min,
        max
    );
    assert_eq!(sorted(&out), vec![0], "stale entries must be cleared first");
    assert_eq!(
        seen.len(),
        1,
        "the dedup set must be cleared and hold exactly the returned bodies, got {:?}",
        seen
    );
    let miss: Vec<usize> = tree.query(Vector2D::new(50.0, 50.0), Vector2D::new(60.0, 60.0));
    assert!(miss.is_empty());
    tree.query_into(
        Vector2D::new(50.0, 50.0),
        Vector2D::new(60.0, 60.0),
        &mut out,
        &mut seen,
    );
    assert!(
        out.is_empty(),
        "a missing query must leave the output buffer empty, got {:?}",
        out
    );
    assert!(
        seen.is_empty(),
        "a missing query must leave the dedup set empty, got {:?}",
        seen
    );
}

#[test]
fn quadtree_and_spatial_hash_grid_expose_the_same_broad_phase_interface() {
    let _: fn(&mut QuadTree2D, usize, Vector2D, Vector2D) = QuadTree2D::insert;
    let _: fn(&mut SpatialHashGrid2D, usize, Vector2D, Vector2D) = SpatialHashGrid2D::insert;
    let _: fn(&QuadTree2D, Vector2D, Vector2D) -> Vec<usize> = QuadTree2D::query;
    let _: fn(&SpatialHashGrid2D, Vector2D, Vector2D) -> Vec<usize> = SpatialHashGrid2D::query;
    let _: fn(&mut QuadTree2D) = QuadTree2D::clear;
    let _: fn(&mut SpatialHashGrid2D) = SpatialHashGrid2D::clear;
    let _: fn(&QuadTree2D, Vector2D, Vector2D, &mut Vec<usize>, &mut HashSet<usize>) =
        QuadTree2D::query_into;
    let _: fn(&SpatialHashGrid2D, Vector2D, Vector2D, &mut Vec<usize>, &mut HashSet<usize>) =
        SpatialHashGrid2D::query_into;
}
