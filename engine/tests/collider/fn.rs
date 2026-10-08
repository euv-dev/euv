use super::*;

const EPSILON: f64 = 1e-9;
#[test]
fn aabb_collider_reports_the_aabb_shape() {
    let collider: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let observed: ColliderShape = collider.shape();
    assert_eq!(
        observed,
        ColliderShape::Aabb,
        "a box collider reports the Aabb shape"
    );
}

#[test]
fn aabb_collider_center_is_the_centre_it_was_built_from() {
    let collider: AabbCollider = AabbCollider::from_center(Vector2D::new(3.0, -2.0), 4.0, 6.0);
    let observed: Vector2D = collider.center();
    assert_eq!(
        observed,
        Vector2D::new(3.0, -2.0),
        "the centre is preserved"
    );
}

#[test]
fn aabb_collider_bounding_box_spans_the_requested_size() {
    let collider: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 4.0, 6.0);
    let observed: Rect = collider.bounding_box();
    assert_eq!(observed.get_width(), 4.0, "the box keeps its width");
    assert_eq!(observed.get_height(), 6.0, "the box keeps its height");
}

#[test]
fn aabb_collider_contains_a_point_inside_and_rejects_one_outside() {
    let collider: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let inside: bool = collider.contains_point(Vector2D::new(1.0, 1.0));
    let outside: bool = collider.contains_point(Vector2D::new(5.0, 0.0));
    assert!(inside, "a point well inside the box is contained");
    assert!(!outside, "a point past the edge is not contained");
}

#[test]
fn aabb_colliders_that_overlap_report_a_collision() {
    let a: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let b: AabbCollider = AabbCollider::from_center(Vector2D::new(2.0, 0.0), 4.0, 4.0);
    let observed: Option<CollisionResult> = a.collide_with_aabb(&b);
    assert!(observed.is_some(), "boxes sharing two units must collide");
}

#[test]
fn overlapping_aabb_colliders_report_the_overlap_as_the_depth() {
    let a: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let b: AabbCollider = AabbCollider::from_center(Vector2D::new(2.0, 0.0), 4.0, 4.0);
    let result: CollisionResult = a.collide_with_aabb(&b).expect("the boxes overlap");
    assert!(
        (result.get_depth() - 2.0).abs() < EPSILON,
        "two-unit overlap means a depth of 2, got {}",
        result.get_depth()
    );
}

#[test]
fn aabb_colliders_pointing_left_report_a_negative_x_normal() {
    let left: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let right: AabbCollider = AabbCollider::from_center(Vector2D::new(2.0, 0.0), 4.0, 4.0);
    let result: CollisionResult = left.collide_with_aabb(&right).expect("the boxes overlap");
    let normal: Vector2D = result.get_normal();
    assert!(
        (normal.get_x() + 1.0).abs() < EPSILON && normal.get_y().abs() < EPSILON,
        "the left box is pushed along minus x, got {normal:?}"
    );
}

#[test]
fn separated_aabb_colliders_report_no_collision() {
    let a: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let b: AabbCollider = AabbCollider::from_center(Vector2D::new(20.0, 0.0), 4.0, 4.0);
    let observed: Option<CollisionResult> = a.collide_with_aabb(&b);
    assert_eq!(observed, None, "distant boxes must not collide");
}

#[test]
fn aabb_colliders_that_only_touch_edges_report_no_collision() {
    let a: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let b: AabbCollider = AabbCollider::from_center(Vector2D::new(4.0, 0.0), 4.0, 4.0);
    let observed: Option<CollisionResult> = a.collide_with_aabb(&b);
    assert_eq!(
        observed, None,
        "edge-to-edge contact is not penetration, so no collision"
    );
}

#[test]
fn aabb_collider_reports_a_collision_against_an_overlapping_circle() {
    let box_collider: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 10.0, 10.0);
    let circle: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 2.0);
    let observed: Option<CollisionResult> = box_collider.collide_with_circle(&circle);
    assert!(
        observed.is_some(),
        "a circle inside the box must register a collision"
    );
}

#[test]
fn aabb_collider_reports_no_collision_against_a_distant_circle() {
    let box_collider: AabbCollider = AabbCollider::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let circle: CircleCollider = CircleCollider::from_center(Vector2D::new(50.0, 0.0), 1.0);
    let observed: Option<CollisionResult> = box_collider.collide_with_circle(&circle);
    assert_eq!(observed, None, "a far-away circle must not register");
}

#[test]
fn circle_collider_reports_the_circle_shape() {
    let collider: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 3.0);
    let observed: ColliderShape = collider.shape();
    assert_eq!(
        observed,
        ColliderShape::Circle,
        "a round collider reports the Circle shape"
    );
}

#[test]
fn circle_collider_center_is_the_centre_it_was_built_from() {
    let collider: CircleCollider = CircleCollider::from_center(Vector2D::new(-4.0, 7.0), 3.0);
    let observed: Vector2D = collider.center();
    assert_eq!(
        observed,
        Vector2D::new(-4.0, 7.0),
        "the centre is preserved"
    );
}

#[test]
fn circle_collider_bounding_box_is_the_diameter_square() {
    let collider: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 3.0);
    let observed: Rect = collider.bounding_box();
    assert!(
        (observed.get_width() - 6.0).abs() < EPSILON,
        "a radius of 3 bounds a 6-wide box, got {}",
        observed.get_width()
    );
    assert!(
        (observed.get_height() - 6.0).abs() < EPSILON,
        "a radius of 3 bounds a 6-tall box, got {}",
        observed.get_height()
    );
}

#[test]
fn circle_collider_contains_a_near_point_and_rejects_a_far_one() {
    let collider: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 5.0);
    let near: bool = collider.contains_point(Vector2D::new(3.0, 0.0));
    let far: bool = collider.contains_point(Vector2D::new(6.0, 0.0));
    assert!(near, "a point inside the radius is contained");
    assert!(!far, "a point outside the radius is not contained");
}

#[test]
fn overlapping_circle_colliders_report_a_collision() {
    let a: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 5.0);
    let b: CircleCollider = CircleCollider::from_center(Vector2D::new(6.0, 0.0), 5.0);
    let observed: Option<CollisionResult> = a.collide_with_circle(&b);
    assert!(
        observed.is_some(),
        "centres six apart with radius five overlap"
    );
}

#[test]
fn overlapping_circle_colliders_report_the_penetration_as_the_depth() {
    let a: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 5.0);
    let b: CircleCollider = CircleCollider::from_center(Vector2D::new(6.0, 0.0), 5.0);
    let result: CollisionResult = a.collide_with_circle(&b).expect("the circles overlap");
    assert!(
        (result.get_depth() - 4.0).abs() < EPSILON,
        "radius sum 10 minus distance 6 leaves a depth of 4, got {}",
        result.get_depth()
    );
}

#[test]
fn concentric_circle_colliders_pick_right_as_the_normal() {
    let a: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 5.0);
    let b: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 2.0);
    let result: CollisionResult = a.collide_with_circle(&b).expect("they overlap");
    let normal: Vector2D = result.get_normal();
    assert_eq!(
        normal,
        Vector2D::right(),
        "a zero-length delta falls back to the right vector"
    );
}

#[test]
fn circle_colliders_exactly_tangent_report_no_collision() {
    let a: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 5.0);
    let b: CircleCollider = CircleCollider::from_center(Vector2D::new(10.0, 0.0), 5.0);
    let observed: Option<CollisionResult> = a.collide_with_circle(&b);
    assert_eq!(
        observed, None,
        "distance equal to the radius sum is tangency, not overlap"
    );
}

#[test]
fn circle_colliders_reach_each_other_from_either_side() {
    let a: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 5.0);
    let b: CircleCollider = CircleCollider::from_center(Vector2D::new(6.0, 0.0), 5.0);
    let forward: Option<CollisionResult> = a.collide_with_circle(&b);
    let backward: Option<CollisionResult> = b.collide_with_circle(&a);
    assert_eq!(
        forward.is_some(),
        backward.is_some(),
        "collision detection must be symmetric"
    );
}

#[test]
fn aabb_collider_3d_reports_the_aabb_shape() {
    let collider: AabbCollider3D =
        AabbCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0);
    let observed: ColliderShape3D = collider.shape();
    assert_eq!(
        observed,
        ColliderShape3D::Aabb,
        "a 3D box collider reports the Aabb shape"
    );
}

#[test]
fn sphere_collider_3d_reports_the_sphere_shape() {
    let collider: SphereCollider3D =
        SphereCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 2.0);
    let observed: ColliderShape3D = collider.shape();
    assert_eq!(
        observed,
        ColliderShape3D::Sphere,
        "a 3D round collider reports the Sphere shape"
    );
}

#[test]
fn overlapping_aabb_colliders_3d_report_a_collision() {
    let a: AabbCollider3D =
        AabbCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let b: AabbCollider3D =
        AabbCollider3D::from_center(Vector3D::new(2.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let observed: Option<CollisionResult3D> = a.collide_with_aabb(&b);
    assert!(observed.is_some(), "boxes sharing two units must collide");
}

#[test]
fn separated_aabb_colliders_3d_report_no_collision() {
    let a: AabbCollider3D =
        AabbCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let b: AabbCollider3D =
        AabbCollider3D::from_center(Vector3D::new(0.0, 0.0, 30.0), 4.0, 4.0, 4.0);
    let observed: Option<CollisionResult3D> = a.collide_with_aabb(&b);
    assert_eq!(observed, None, "boxes stacked apart on z must not collide");
}

#[test]
fn aabb_collider_3d_reports_a_collision_against_an_overlapping_sphere() {
    let box_collider: AabbCollider3D =
        AabbCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 10.0, 10.0, 10.0);
    let sphere: SphereCollider3D = SphereCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 2.0);
    let observed: Option<CollisionResult3D> = box_collider.collide_with_sphere(&sphere);
    assert!(
        observed.is_some(),
        "a sphere inside the box must register a collision"
    );
}

#[test]
fn aabb_collider_3d_reports_no_collision_against_a_distant_sphere() {
    let box_collider: AabbCollider3D =
        AabbCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let sphere: SphereCollider3D =
        SphereCollider3D::from_center(Vector3D::new(50.0, 0.0, 0.0), 1.0);
    let observed: Option<CollisionResult3D> = box_collider.collide_with_sphere(&sphere);
    assert_eq!(observed, None, "a far-away sphere must not register");
}

#[test]
fn overlapping_sphere_colliders_3d_report_the_penetration_as_the_depth() {
    let a: SphereCollider3D = SphereCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 5.0);
    let b: SphereCollider3D = SphereCollider3D::from_center(Vector3D::new(0.0, 6.0, 0.0), 5.0);
    let result: CollisionResult3D = a.collide_with_sphere(&b).expect("the spheres overlap");
    assert!(
        (result.get_depth() - 4.0).abs() < EPSILON,
        "radius sum 10 minus distance 6 leaves a depth of 4, got {}",
        result.get_depth()
    );
}

#[test]
fn concentric_sphere_colliders_3d_pick_right_as_the_normal() {
    let a: SphereCollider3D = SphereCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 5.0);
    let b: SphereCollider3D = SphereCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 2.0);
    let result: CollisionResult3D = a.collide_with_sphere(&b).expect("they overlap");
    let normal: Vector3D = result.get_normal();
    assert_eq!(
        normal,
        Vector3D::right(),
        "a zero-length delta falls back to the right vector"
    );
}

#[test]
fn sphere_colliders_3d_exactly_tangent_report_no_collision() {
    let a: SphereCollider3D = SphereCollider3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 5.0);
    let b: SphereCollider3D = SphereCollider3D::from_center(Vector3D::new(10.0, 0.0, 0.0), 5.0);
    let observed: Option<CollisionResult3D> = a.collide_with_sphere(&b);
    assert_eq!(
        observed, None,
        "distance equal to the radius sum is tangency, not overlap"
    );
}

#[test]
fn broad_phase_accepts_overlapping_boxes() {
    let a: AABB3D = AABB3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let b: AABB3D = AABB3D::from_center(Vector3D::new(1.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let observed: bool = AABB3D::broad_phase(a, b);
    assert!(observed, "overlapping boxes pass the broad phase");
}

#[test]
fn broad_phase_rejects_separated_boxes() {
    let a: AABB3D = AABB3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let b: AABB3D = AABB3D::from_center(Vector3D::new(100.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let observed: bool = AABB3D::broad_phase(a, b);
    assert!(!observed, "distant boxes are culled by the broad phase");
}

#[test]
fn broad_phase_is_symmetric_in_its_arguments() {
    let a: AABB3D = AABB3D::from_center(Vector3D::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let b: AABB3D = AABB3D::from_center(Vector3D::new(1.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let forward: bool = AABB3D::broad_phase(a, b);
    let backward: bool = AABB3D::broad_phase(b, a);
    assert_eq!(
        forward, backward,
        "the broad phase must not depend on order"
    );
}
