use super::*;

#[test]
fn two_overlapping_axis_aligned_boxes_report_a_collision() {
    let left: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 10.0, 10.0);
    let right: AabbCollider = AabbCollider::from_center(Vector2D::new(8.0, 0.0), 10.0, 10.0);
    let result: Option<CollisionResult> = left.collide_with_aabb(&right);
    assert!(
        result.is_some(),
        "overlapping boxes must report a collision"
    );
    let depth: f64 = result.unwrap().get_depth();
    assert!(
        depth > 0.0,
        "a reported collision must have positive depth, got {depth}"
    );
}

#[test]
fn two_separated_axis_aligned_boxes_report_no_collision() {
    let left: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 10.0, 10.0);
    let right: AabbCollider = AabbCollider::from_center(Vector2D::new(100.0, 0.0), 10.0, 10.0);
    let result: Option<CollisionResult> = left.collide_with_aabb(&right);
    assert!(result.is_none(), "boxes 100 units apart must not collide");
}

#[test]
fn boxes_that_almost_touch_still_overlap() {
    let left: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 10.0, 10.0);
    let right: AabbCollider = AabbCollider::from_center(Vector2D::new(9.9, 0.0), 10.0, 10.0);
    let result: Option<CollisionResult> = left.collide_with_aabb(&right);
    assert!(
        result.is_some(),
        "boxes with a sliver of overlap must report a collision"
    );
}

#[test]
fn collision_is_symmetric_between_two_boxes() {
    let left: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 10.0, 10.0);
    let right: AabbCollider = AabbCollider::from_center(Vector2D::new(6.0, 0.0), 10.0, 10.0);
    let forward: Option<CollisionResult> = left.collide_with_aabb(&right);
    let backward: Option<CollisionResult> = right.collide_with_aabb(&left);
    assert_eq!(
        forward.is_some(),
        backward.is_some(),
        "box overlap must not depend on argument order"
    );
    if let (Some(a), Some(b)) = (forward, backward) {
        assert!(
            (a.get_depth() - b.get_depth()).abs() < 1e-9,
            "the penetration depth must not depend on argument order"
        );
    }
}

#[test]
fn an_axis_aligned_box_overlapping_a_circle_reports_a_collision() {
    let box_shape: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 20.0, 20.0);
    let circle: CircleCollider = CircleCollider::from_center(Vector2D::new(8.0, 0.0), 2.0);
    let result: Option<CollisionResult> = box_shape.collide_with_circle(&circle);
    assert!(
        result.is_some(),
        "a circle inside the box must collide with it"
    );
    assert!(
        result.unwrap().get_depth() > 0.0,
        "a box-circle collision must have positive depth"
    );
}

#[test]
fn a_circle_outside_a_box_does_not_collide_with_it() {
    let box_shape: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let circle: CircleCollider = CircleCollider::from_center(Vector2D::new(100.0, 0.0), 2.0);
    let result: Option<CollisionResult> = box_shape.collide_with_circle(&circle);
    assert!(
        result.is_none(),
        "a distant circle must not collide with the box"
    );
}

#[test]
fn two_overlapping_circles_report_a_collision() {
    let left: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 5.0);
    let right: CircleCollider = CircleCollider::from_center(Vector2D::new(8.0, 0.0), 5.0);
    let result: Option<CollisionResult> = left.collide_with_circle(&right);
    assert!(
        result.is_some(),
        "circles closer than the sum of their radii must collide"
    );
}

#[test]
fn two_circles_with_a_sliver_of_overlap_collide() {
    let left: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 5.0);
    let right: CircleCollider = CircleCollider::from_center(Vector2D::new(9.9, 0.0), 5.0);
    let result: Option<CollisionResult> = left.collide_with_circle(&right);
    assert!(
        result.is_some(),
        "circles closer than the sum of their radii must collide"
    );
}

#[test]
fn two_separated_circles_report_no_collision() {
    let left: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 5.0);
    let right: CircleCollider = CircleCollider::from_center(Vector2D::new(20.0, 0.0), 5.0);
    let result: Option<CollisionResult> = left.collide_with_circle(&right);
    assert!(
        result.is_none(),
        "circles 20 units apart with radius 5 must not collide"
    );
}

#[test]
fn a_degenerate_zero_radius_circle_still_participates_in_queries() {
    let point: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 0.0);
    let overlapping: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 1.0);
    let result: Option<CollisionResult> = point.collide_with_circle(&overlapping);
    assert!(
        result.is_some(),
        "a zero radius circle at a contained point must still collide"
    );
}

#[test]
fn two_overlapping_axis_aligned_boxes_in_three_dimensions_report_a_collision() {
    let left: AabbCollider3D =
        AabbCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 10.0, 10.0, 10.0);
    let right: AabbCollider3D =
        AabbCollider3D::from_center(Vector3D::new(5.0, 0.0, 0.0), 10.0, 10.0, 10.0);
    let result: Option<CollisionResult3D> = left.collide_with_aabb(&right);
    assert!(
        result.is_some(),
        "overlapping 3D boxes must report a collision"
    );
    assert!(
        result.unwrap().get_depth() > 0.0,
        "a 3D collision must have positive depth"
    );
}

#[test]
fn three_dimensional_boxes_separated_along_z_do_not_collide() {
    let left: AabbCollider3D =
        AabbCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 10.0, 10.0, 10.0);
    let right: AabbCollider3D =
        AabbCollider3D::from_center(Vector3D::new(0.0, 0.0, 50.0), 10.0, 10.0, 10.0);
    let result: Option<CollisionResult3D> = left.collide_with_aabb(&right);
    assert!(
        result.is_none(),
        "boxes separated only along the z axis must not collide"
    );
}

#[test]
fn an_axis_aligned_3d_box_overlapping_a_sphere_reports_a_collision() {
    let box_shape: AabbCollider3D =
        AabbCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 20.0, 20.0, 20.0);
    let sphere: SphereCollider3D = SphereCollider3D::from_center(Vector3D::new(9.0, 0.0, 0.0), 2.0);
    let result: Option<CollisionResult3D> = box_shape.collide_with_sphere(&sphere);
    assert!(
        result.is_some(),
        "a sphere inside the 3D box must collide with it"
    );
}

#[test]
fn two_separated_spheres_report_no_collision() {
    let left: SphereCollider3D = SphereCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 3.0);
    let right: SphereCollider3D = SphereCollider3D::from_center(Vector3D::new(50.0, 0.0, 0.0), 3.0);
    let result: Option<CollisionResult3D> = left.collide_with_sphere(&right);
    assert!(result.is_none(), "distant spheres must not collide");
}

#[test]
fn broad_phase_accepts_overlapping_boxes_and_rejects_separated_ones() {
    let a: AABB3D = AABB3D::new(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(10.0, 10.0, 10.0),
    );
    let overlapping: AABB3D = AABB3D::new(
        Vector3D::new(5.0, 0.0, 0.0),
        Vector3D::new(15.0, 10.0, 10.0),
    );
    let separated: AABB3D = AABB3D::new(
        Vector3D::new(100.0, 0.0, 0.0),
        Vector3D::new(110.0, 10.0, 10.0),
    );
    assert!(
        AABB3D::broad_phase(a, overlapping),
        "overlapping boxes must pass the broad phase"
    );
    assert!(
        !AABB3D::broad_phase(a, separated),
        "separated boxes must be culled by the broad phase"
    );
}

#[test]
fn the_collider_shape_enums_expose_matchable_variants() {
    let two_dimensional: ColliderShape = ColliderShape::Aabb;
    let three_dimensional: ColliderShape3D = ColliderShape3D::Sphere;
    let label_two: &str = match two_dimensional {
        ColliderShape::Aabb => "aabb",
        ColliderShape::Circle => "circle",
    };
    let label_three: &str = match three_dimensional {
        ColliderShape3D::Aabb => "aabb",
        ColliderShape3D::Sphere => "sphere",
    };
    assert_eq!(label_two, "aabb", "the 2D shape enum must be matchable");
    assert_eq!(label_three, "sphere", "the 3D shape enum must be matchable");
}
