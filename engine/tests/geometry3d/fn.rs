use super::*;

const HALF_PI: f64 = FRAC_PI_2;
fn close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-9
}

fn vec3_close(left: Vector3D, right: Vector3D) -> bool {
    close(left.get_x(), right.get_x())
        && close(left.get_y(), right.get_y())
        && close(left.get_z(), right.get_z())
}

fn aabb(center: Vector3D, size: Vector3D) -> AABB3D {
    AABB3D::from_center(center, size.get_x(), size.get_y(), size.get_z())
}

fn sphere(center: Vector3D, radius: f64) -> Sphere {
    Sphere::new(center, radius)
}

fn ray(origin: Vector3D, direction: Vector3D) -> Ray3D {
    Ray3D::new(origin, direction)
}

#[test]
fn a_box_from_a_centre_reports_that_centre_back() {
    let observed: Vector3D =
        aabb(Vector3D::new(1.0, 2.0, 3.0), Vector3D::new(2.0, 2.0, 2.0)).center();
    assert!(
        vec3_close(observed, Vector3D::new(1.0, 2.0, 3.0)),
        "the centre round-trips, got {observed:?}"
    );
}

#[test]
fn a_box_from_a_centre_reports_that_size_back() {
    let observed: Vector3D = aabb(Vector3D::zero(), Vector3D::new(2.0, 4.0, 6.0)).size();
    assert!(
        vec3_close(observed, Vector3D::new(2.0, 4.0, 6.0)),
        "the dimensions round-trip, got {observed:?}"
    );
}

#[test]
fn a_box_contains_a_point_inside_it() {
    let observed: bool = aabb(Vector3D::zero(), Vector3D::new(4.0, 4.0, 4.0))
        .contains(Vector3D::new(1.0, -1.0, 2.0));
    assert!(observed, "a point inside the extents is contained");
}

#[test]
fn a_box_does_not_contain_a_point_outside_it() {
    let observed: bool =
        aabb(Vector3D::zero(), Vector3D::new(4.0, 4.0, 4.0)).contains(Vector3D::new(5.0, 0.0, 0.0));
    assert!(!observed, "a point past an extent is outside");
}

#[test]
fn two_overlapping_boxes_intersect() {
    let a: AABB3D = aabb(Vector3D::zero(), Vector3D::new(4.0, 4.0, 4.0));
    let b: AABB3D = aabb(Vector3D::new(2.0, 0.0, 0.0), Vector3D::new(4.0, 4.0, 4.0));
    assert!(a.intersects(b), "boxes sharing a slab intersect");
}

#[test]
fn two_separated_boxes_do_not_intersect() {
    let a: AABB3D = aabb(Vector3D::zero(), Vector3D::new(4.0, 4.0, 4.0));
    let b: AABB3D = aabb(Vector3D::new(0.0, 0.0, 50.0), Vector3D::new(4.0, 4.0, 4.0));
    assert!(
        !a.intersects(b),
        "boxes stacked apart on z do not intersect"
    );
}

#[test]
fn box_intersection_is_symmetric() {
    let a: AABB3D = aabb(Vector3D::zero(), Vector3D::new(4.0, 4.0, 4.0));
    let b: AABB3D = aabb(Vector3D::new(1.0, 1.0, 1.0), Vector3D::new(4.0, 4.0, 4.0));
    assert_eq!(
        a.intersects(b),
        b.intersects(a),
        "overlap does not depend on the order"
    );
}

#[test]
fn a_sphere_contains_a_point_inside_it() {
    let observed: bool = sphere(Vector3D::zero(), 5.0).contains(Vector3D::new(3.0, 0.0, 4.0));
    assert!(observed, "a 3-4-5 point is on the rim");
}

#[test]
fn a_sphere_does_not_contain_a_point_outside_it() {
    let observed: bool = sphere(Vector3D::zero(), 5.0).contains(Vector3D::new(6.0, 0.0, 0.0));
    assert!(!observed, "a point past the radius is outside");
}

#[test]
fn two_overlapping_spheres_intersect() {
    let observed: bool =
        sphere(Vector3D::zero(), 5.0).intersects(sphere(Vector3D::new(6.0, 0.0, 0.0), 5.0));
    assert!(observed, "centres six apart with radius five overlap");
}

#[test]
fn two_distant_spheres_do_not_intersect() {
    let observed: bool =
        sphere(Vector3D::zero(), 1.0).intersects(sphere(Vector3D::new(50.0, 0.0, 0.0), 1.0));
    assert!(!observed, "far-apart spheres are disjoint");
}

#[test]
fn a_sphere_volume_is_four_thirds_pi_r_cubed() {
    let observed: f64 = sphere(Vector3D::zero(), 3.0).volume();
    let expected: f64 = (4.0 / 3.0) * PI * 27.0;
    assert!(
        close(observed, expected),
        "four thirds pi r cubed, got {observed}"
    );
}

#[test]
fn a_sphere_surface_area_is_four_pi_r_squared() {
    let observed: f64 = sphere(Vector3D::zero(), 2.0).surface_area();
    assert!(
        close(observed, 4.0 * PI * 4.0),
        "four pi r squared, got {observed}"
    );
}

#[test]
fn a_sphere_of_radius_zero_has_no_volume() {
    let observed: f64 = sphere(Vector3D::zero(), 0.0).volume();
    assert_eq!(observed, 0.0, "a degenerate sphere encloses nothing");
}

#[test]
fn a_plane_through_the_origin_is_at_zero_distance_from_it() {
    let observed: f64 =
        Plane::from_normal_and_point(Vector3D::new(0.0, 1.0, 0.0), Vector3D::zero())
            .distance_to_point(Vector3D::zero());
    assert!(
        close(observed, 0.0),
        "a point on the plane is at distance zero"
    );
}

#[test]
fn a_plane_measures_a_signed_distance() {
    let observed: f64 =
        Plane::from_normal_and_point(Vector3D::new(0.0, 1.0, 0.0), Vector3D::zero())
            .distance_to_point(Vector3D::new(0.0, 5.0, 0.0));
    assert!(
        close(observed, 5.0),
        "a point five along the normal is five away"
    );
}

#[test]
fn a_plane_measures_the_opposite_side_as_negative() {
    let observed: f64 =
        Plane::from_normal_and_point(Vector3D::new(0.0, 1.0, 0.0), Vector3D::zero())
            .distance_to_point(Vector3D::new(0.0, -5.0, 0.0));
    assert!(
        close(observed, -5.0),
        "the far side of the normal is negative, got {observed}"
    );
}

#[test]
fn a_plane_normalises_its_own_normal() {
    let mut plane: Plane = Plane::new(Vector3D::new(0.0, 4.0, 0.0), 0.0);
    plane.normalize();
    let point: Vector3D = Vector3D::new(0.0, 2.0, 0.0);
    let observed: f64 = plane.distance_to_point(point);
    assert!(
        close(observed, 2.0),
        "after normalising, the distance is the true one, got {observed}"
    );
}

#[test]
fn normalising_a_degenerate_plane_leaves_it_alone() {
    let mut plane: Plane = Plane::new(Vector3D::zero(), 3.0);
    plane.normalize();
    let observed: f64 = plane.distance_to_point(Vector3D::new(1.0, 1.0, 1.0));
    assert!(
        close(observed, 3.0),
        "a zero normal cannot be scaled, so the plane is untouched, got {observed}"
    );
}

#[test]
fn a_ray_starts_at_its_origin() {
    let observed: Vector3D =
        ray(Vector3D::new(1.0, 2.0, 3.0), Vector3D::new(0.0, 0.0, 1.0)).point_at(0.0);
    assert!(
        vec3_close(observed, Vector3D::new(1.0, 2.0, 3.0)),
        "t = 0 is the origin, got {observed:?}"
    );
}

#[test]
fn a_ray_walks_along_its_direction() {
    let observed: Vector3D = ray(Vector3D::zero(), Vector3D::new(1.0, 0.0, 0.0)).point_at(4.0);
    assert!(
        vec3_close(observed, Vector3D::new(4.0, 0.0, 0.0)),
        "t scales the direction, got {observed:?}"
    );
}

#[test]
fn a_ray_going_backwards_walks_the_other_way() {
    let observed: Vector3D = ray(Vector3D::zero(), Vector3D::new(1.0, 0.0, 0.0)).point_at(-2.0);
    assert!(
        vec3_close(observed, Vector3D::new(-2.0, 0.0, 0.0)),
        "a negative t runs backwards, got {observed:?}"
    );
}

#[test]
fn a_ray_pointing_at_a_sphere_finds_the_near_intersection() {
    let observed: Option<f64> = ray(Vector3D::new(-10.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0))
        .intersect_sphere(sphere(Vector3D::zero(), 1.0));
    match observed {
        Some(t) => assert!(
            close(t, 9.0),
            "the near surface is nine units along, got {t}"
        ),
        None => panic!("a ray aimed at a sphere must hit it"),
    }
}

#[test]
fn a_ray_starting_inside_a_sphere_reports_the_far_surface() {
    let observed: Option<f64> = ray(Vector3D::zero(), Vector3D::new(1.0, 0.0, 0.0))
        .intersect_sphere(sphere(Vector3D::zero(), 2.0));
    match observed {
        Some(t) => assert!(
            close(t, 2.0),
            "the near root is behind the ray, so the far surface is reported, got {t}"
        ),
        None => panic!("a ray inside a sphere must hit it"),
    }
}

#[test]
fn a_ray_missing_a_sphere_reports_nothing() {
    let observed: Option<f64> = ray(
        Vector3D::new(-10.0, 10.0, 0.0),
        Vector3D::new(1.0, 0.0, 0.0),
    )
    .intersect_sphere(sphere(Vector3D::zero(), 1.0));
    assert_eq!(observed, None, "a ray passing wide misses entirely");
}

#[test]
fn a_ray_pointing_away_from_a_sphere_reports_nothing() {
    let observed: Option<f64> = ray(
        Vector3D::new(-10.0, 0.0, 0.0),
        Vector3D::new(-1.0, 0.0, 0.0),
    )
    .intersect_sphere(sphere(Vector3D::zero(), 1.0));
    assert_eq!(observed, None, "both roots lie behind the ray");
}

#[test]
fn a_ray_hitting_a_plane_reports_the_step() {
    let plane: Plane =
        Plane::from_normal_and_point(Vector3D::new(0.0, 0.0, 1.0), Vector3D::new(0.0, 0.0, 5.0));
    let observed: Option<f64> =
        ray(Vector3D::zero(), Vector3D::new(0.0, 0.0, 1.0)).intersect_plane(plane);
    match observed {
        Some(t) => assert!(close(t, 5.0), "the plane is five steps away, got {t}"),
        None => panic!("a ray aimed at a plane must hit it"),
    }
}

#[test]
fn a_ray_parallel_to_a_plane_reports_nothing() {
    let plane: Plane =
        Plane::from_normal_and_point(Vector3D::new(0.0, 0.0, 1.0), Vector3D::new(0.0, 0.0, 5.0));
    let observed: Option<f64> =
        ray(Vector3D::zero(), Vector3D::new(1.0, 0.0, 0.0)).intersect_plane(plane);
    assert_eq!(observed, None, "a ray parallel to a plane never reaches it");
}

#[test]
fn a_ray_facing_away_from_a_plane_reports_nothing() {
    let plane: Plane =
        Plane::from_normal_and_point(Vector3D::new(0.0, 0.0, 1.0), Vector3D::new(0.0, 0.0, 5.0));
    let observed: Option<f64> =
        ray(Vector3D::zero(), Vector3D::new(0.0, 0.0, -1.0)).intersect_plane(plane);
    assert_eq!(observed, None, "the plane lies behind the ray");
}

#[test]
fn the_identity_transform3d_leaves_a_point_where_it_is() {
    let point: Vector3D = Vector3D::new(3.0, -4.0, 5.0);
    let observed: Vector3D = Transform3D::identity().apply_to_point(point);
    assert!(
        vec3_close(observed, point),
        "identity is a no-op, got {observed:?}"
    );
}

#[test]
fn a_default_transform3d_is_the_identity() {
    let point: Vector3D = Vector3D::new(1.0, 2.0, 3.0);
    let observed: Vector3D = Transform3D::default().apply_to_point(point);
    assert!(
        vec3_close(observed, point),
        "the derive default is the identity"
    );
}

#[test]
fn translating_a_transform3d_shifts_a_point() {
    let mut transform: Transform3D = Transform3D::identity();
    transform.translate(Vector3D::new(1.0, 2.0, 3.0));
    let observed: Vector3D = transform.apply_to_point(Vector3D::new(10.0, 20.0, 30.0));
    assert!(
        vec3_close(observed, Vector3D::new(11.0, 22.0, 33.0)),
        "the point moves by the offset, got {observed:?}"
    );
}

#[test]
fn scaling_a_transform3d_multiplies_each_axis() {
    let mut transform: Transform3D = Transform3D::identity();
    transform.scale_by(Vector3D::new(2.0, 3.0, 4.0));
    let observed: Vector3D = transform.apply_to_point(Vector3D::new(1.0, 1.0, 1.0));
    assert!(
        vec3_close(observed, Vector3D::new(2.0, 3.0, 4.0)),
        "each axis scales on its own, got {observed:?}"
    );
}

#[test]
fn a_transform3d_to_matrix_matches_applying_it_to_a_point() {
    let mut transform: Transform3D = Transform3D::identity();
    transform.translate(Vector3D::new(5.0, 0.0, 0.0));
    let point: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
    let direct: Vector3D = transform.apply_to_point(point);
    let via_matrix: Vector3D = transform.to_matrix().transform_point(point);
    assert!(
        vec3_close(direct, via_matrix),
        "the matrix form and the point form must agree, got {direct:?} vs {via_matrix:?}"
    );
}

#[test]
fn an_identity_transform3d_produces_the_identity_matrix() {
    let observed: Matrix4x4 = Transform3D::identity().to_matrix();
    assert_eq!(
        observed,
        Matrix4x4::identity(),
        "an untouched transform is the identity"
    );
}

#[test]
fn a_translated_transform3d_produces_a_translation_matrix() {
    let mut transform: Transform3D = Transform3D::identity();
    transform.translate(Vector3D::new(1.0, 2.0, 3.0));
    let observed: Matrix4x4 = transform.to_matrix();
    let point: Vector3D = transform.apply_to_point(Vector3D::zero());
    assert_eq!(
        observed,
        Matrix4x4::translation(Vector3D::new(1.0, 2.0, 3.0)),
        "translation is the only thing applied so far"
    );
    assert!(
        vec3_close(point, Vector3D::new(1.0, 2.0, 3.0)),
        "and the origin lands on the offset, got {point:?}"
    );
}

#[test]
fn rotating_a_vector_by_the_identity_leaves_it_where_it_is() {
    let observed: Vector3D = Vector3D::new(1.0, 2.0, 3.0).rotated_by(Quaternion::identity());
    assert!(
        vec3_close(observed, Vector3D::new(1.0, 2.0, 3.0)),
        "rotating by identity is a no-op, got {observed:?}"
    );
}

#[test]
fn rotating_a_vector_keeps_its_length() {
    let v: Vector3D = Vector3D::new(1.0, 2.0, 3.0);
    let q: Quaternion = Quaternion::from_euler(0.4, 0.9, -0.2);
    let observed: Vector3D = v.rotated_by(q);
    assert!(
        (observed.magnitude() - v.magnitude()).abs() < 1e-9,
        "a rotation preserves magnitude, got {} vs {}",
        observed.magnitude(),
        v.magnitude()
    );
}

#[test]
fn a_quarter_turn_about_z_maps_x_onto_y() {
    let q: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), HALF_PI);
    let observed: Vector3D = Vector3D::new(1.0, 0.0, 0.0).rotated_by(q);
    assert!(
        close(observed.get_x(), 0.0) && close(observed.get_y(), 1.0),
        "a quarter turn about z carries +x to +y, got {observed:?}"
    );
}

#[test]
fn a_half_turn_about_z_reverses_x() {
    let q: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), PI);
    let observed: Vector3D = Vector3D::new(1.0, 0.0, 0.0).rotated_by(q);
    assert!(
        close(observed.get_x(), -1.0) && close(observed.get_y(), 0.0),
        "a half turn about z sends +x to -x, got {observed:?}"
    );
}

#[test]
fn rotating_the_zero_vector_keeps_it_zero() {
    let q: Quaternion = Quaternion::from_euler(0.3, 0.3, 0.3);
    let observed: Vector3D = Vector3D::zero().rotated_by(q);
    assert_eq!(
        observed,
        Vector3D::zero(),
        "the zero vector has no direction to rotate"
    );
}

#[test]
fn a_ray_aimed_at_a_box_reports_the_near_face() {
    let box_bounds: AABB3D = aabb(Vector3D::zero(), Vector3D::new(2.0, 2.0, 2.0));
    let shooter: Ray3D = Ray3D::new(Vector3D::new(0.0, 0.0, -10.0), Vector3D::new(0.0, 0.0, 1.0));
    let observed: Option<f64> = shooter.intersect_aabb(box_bounds);
    match observed {
        Some(t) => assert!(
            close(t, 9.0),
            "the near face is nine units along a ray from z = -10, got {t}"
        ),
        None => panic!("a ray aimed straight at a box must hit it"),
    }
}

#[test]
fn a_ray_starting_inside_a_box_reports_the_way_out() {
    let box_bounds: AABB3D = aabb(Vector3D::zero(), Vector3D::new(2.0, 2.0, 2.0));
    let inside: Ray3D = Ray3D::new(Vector3D::zero(), Vector3D::new(0.0, 0.0, 1.0));
    let observed: Option<f64> = inside.intersect_aabb(box_bounds);
    match observed {
        Some(t) => assert!(
            close(t, 1.0),
            "from the centre the exit face is one unit away, got {t}"
        ),
        None => panic!("a ray inside a box must report an exit"),
    }
}

#[test]
fn a_ray_pointing_away_from_a_box_reports_nothing() {
    let box_bounds: AABB3D = aabb(Vector3D::zero(), Vector3D::new(2.0, 2.0, 2.0));
    let shooter: Ray3D = Ray3D::new(
        Vector3D::new(0.0, 0.0, -10.0),
        Vector3D::new(0.0, 0.0, -1.0),
    );
    let observed: Option<f64> = shooter.intersect_aabb(box_bounds);
    assert_eq!(observed, None, "both slab intersections lie behind the ray");
}

#[test]
fn a_ray_that_passes_beside_a_box_reports_nothing() {
    let box_bounds: AABB3D = aabb(Vector3D::zero(), Vector3D::new(2.0, 2.0, 2.0));
    let shooter: Ray3D = Ray3D::new(
        Vector3D::new(0.0, 50.0, -10.0),
        Vector3D::new(0.0, 0.0, 1.0),
    );
    let observed: Option<f64> = shooter.intersect_aabb(box_bounds);
    assert_eq!(
        observed, None,
        "a ray that misses sideways never reaches the box"
    );
}

#[test]
fn a_ray_parallel_to_a_slab_outside_it_reports_nothing() {
    let box_bounds: AABB3D = aabb(Vector3D::zero(), Vector3D::new(2.0, 2.0, 2.0));
    let parallel: Ray3D = Ray3D::new(Vector3D::new(0.0, 50.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
    let observed: Option<f64> = parallel.intersect_aabb(box_bounds);
    assert_eq!(
        observed, None,
        "a zero direction component cannot divide, so the slab is \
         rejected outright when the origin is outside it"
    );
}

#[test]
fn a_ray_parallel_to_a_slab_inside_it_still_hits() {
    let box_bounds: AABB3D = aabb(Vector3D::zero(), Vector3D::new(2.0, 2.0, 2.0));
    let inside_the_slab: Ray3D =
        Ray3D::new(Vector3D::new(0.0, 0.0, -10.0), Vector3D::new(0.0, 0.0, 1.0));
    let observed: Option<f64> = inside_the_slab.intersect_aabb(box_bounds);
    assert!(
        observed.is_some(),
        "a ray travelling parallel to a slab it lies inside must not be rejected"
    );
}
