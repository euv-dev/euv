use super::*;

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-9
}

fn vec2_close(left: Vector2D, right: Vector2D) -> bool {
    close(left.get_x(), right.get_x()) && close(left.get_y(), right.get_y())
}

fn vec3_close(left: Vector3D, right: Vector3D) -> bool {
    close(left.get_x(), right.get_x())
        && close(left.get_y(), right.get_y())
        && close(left.get_z(), right.get_z())
}

fn dynamic_2d() -> RigidBody2D {
    RigidBody2D::new_dynamic(1, Vector2D::new(0.0, 0.0))
}

fn dynamic_3d() -> RigidBody3D {
    RigidBody3D::new_dynamic(1, Vector3D::new(0.0, 0.0, 0.0))
}

#[test]
fn a_dynamic_body_keeps_the_id_and_position_it_was_built_with() {
    let body: RigidBody2D = RigidBody2D::new_dynamic(42, Vector2D::new(3.0, 4.0));
    assert_eq!(body.get_id(), 42, "the id is stored verbatim");
    assert!(
        vec2_close(body.get_position(), Vector2D::new(3.0, 4.0)),
        "the position is stored verbatim"
    );
}

#[test]
fn a_dynamic_body_starts_at_rest() {
    let body: RigidBody2D = dynamic_2d();
    assert_eq!(
        body.get_velocity(),
        Vector2D::zero(),
        "a fresh body does not move"
    );
}

#[test]
fn a_dynamic_body_reports_itself_as_dynamic() {
    let body: RigidBody2D = dynamic_2d();
    assert!(body.is_dynamic(), "new_dynamic produces a dynamic body");
}

#[test]
fn a_static_body_does_not_report_itself_as_dynamic() {
    let body: RigidBody2D = RigidBody2D::new_static(1, Vector2D::zero());
    assert!(!body.is_dynamic(), "new_static produces a static body");
}

#[test]
fn applying_a_force_accumulates_rather_than_replacing() {
    let mut body: RigidBody2D = dynamic_2d();
    body.apply_force(Vector2D::new(1.0, 0.0));
    body.apply_force(Vector2D::new(2.0, 0.0));
    assert!(
        vec2_close(body.get_force_accumulator(), Vector2D::new(3.0, 0.0)),
        "two forces sum in the accumulator, got {:?}",
        body.get_force_accumulator()
    );
}

#[test]
fn an_impulse_scales_by_the_inverse_mass() {
    let mut body: RigidBody2D = dynamic_2d();
    body.update_mass(2.0);
    body.apply_impulse(Vector2D::new(10.0, 0.0));
    assert!(
        vec2_close(body.get_velocity(), Vector2D::new(5.0, 0.0)),
        "an impulse of ten on a mass of two moves at five, got {:?}",
        body.get_velocity()
    );
}

#[test]
fn an_impulse_accumulates_across_calls() {
    let mut body: RigidBody2D = dynamic_2d();
    body.update_mass(1.0);
    body.apply_impulse(Vector2D::new(1.0, 0.0));
    body.apply_impulse(Vector2D::new(2.0, 0.0));
    assert!(
        vec2_close(body.get_velocity(), Vector2D::new(3.0, 0.0)),
        "impulses add into the velocity, got {:?}",
        body.get_velocity()
    );
}

#[test]
fn an_impulse_on_a_heavier_body_moves_it_less() {
    let mut light: RigidBody2D = dynamic_2d();
    let mut heavy: RigidBody2D = dynamic_2d();
    light.update_mass(1.0);
    heavy.update_mass(10.0);
    light.apply_impulse(Vector2D::new(10.0, 0.0));
    heavy.apply_impulse(Vector2D::new(10.0, 0.0));
    assert!(
        light.get_velocity().get_x() > heavy.get_velocity().get_x(),
        "mass damps the response"
    );
}

#[test]
fn an_impulse_on_a_static_body_is_ignored() {
    let mut body: RigidBody2D = RigidBody2D::new_static(1, Vector2D::zero());
    body.apply_impulse(Vector2D::new(100.0, 0.0));
    assert_eq!(
        body.get_velocity(),
        Vector2D::zero(),
        "a static body has infinite mass, so it never moves"
    );
}

#[test]
fn update_mass_keeps_mass_and_inverse_mass_consistent() {
    let mut body: RigidBody2D = dynamic_2d();
    body.update_mass(4.0);
    assert!(close(body.get_mass(), 4.0), "the mass is stored as given");
    assert!(
        close(body.get_inverse_mass(), 0.25),
        "the inverse mass is recomputed, got {}",
        body.get_inverse_mass()
    );
}

#[test]
fn update_mass_of_zero_makes_the_body_immovable() {
    let mut body: RigidBody2D = dynamic_2d();
    body.update_mass(0.0);
    assert_eq!(
        body.get_inverse_mass(),
        0.0,
        "zero mass is an infinite-mass body, not a division by zero"
    );
}

#[test]
fn update_mass_of_a_negative_value_makes_the_body_immovable() {
    let mut body: RigidBody2D = dynamic_2d();
    body.update_mass(-5.0);
    assert_eq!(
        body.get_inverse_mass(),
        0.0,
        "a negative mass is clamped to an immovable body rather than inverting its sign"
    );
}

#[test]
fn a_dynamic_body_3d_keeps_its_id_and_position() {
    let body: RigidBody3D = RigidBody3D::new_dynamic(7, Vector3D::new(1.0, 2.0, 3.0));
    assert_eq!(body.get_id(), 7, "the id is stored verbatim");
    assert!(
        vec3_close(body.get_position(), Vector3D::new(1.0, 2.0, 3.0)),
        "the position is stored verbatim"
    );
}

#[test]
fn a_dynamic_body_3d_starts_at_rest_and_is_dynamic() {
    let body: RigidBody3D = dynamic_3d();
    assert_eq!(
        body.get_velocity(),
        Vector3D::zero(),
        "a fresh body does not move"
    );
    assert!(body.is_dynamic(), "new_dynamic produces a dynamic body");
}

#[test]
fn a_static_body_3d_does_not_report_itself_as_dynamic() {
    let body: RigidBody3D = RigidBody3D::new_static(1, Vector3D::zero());
    assert!(!body.is_dynamic(), "new_static produces a static body");
}

#[test]
fn a_torque_accumulates_on_a_3d_body() {
    let mut body: RigidBody3D = dynamic_3d();
    body.apply_torque(Vector3D::new(0.0, 1.0, 0.0));
    body.apply_torque(Vector3D::new(0.0, 2.0, 0.0));
    assert!(
        vec3_close(body.get_torque_accumulator(), Vector3D::new(0.0, 3.0, 0.0)),
        "two torques sum, got {:?}",
        body.get_torque_accumulator()
    );
}

#[test]
fn a_force_on_a_3d_body_accumulates() {
    let mut body: RigidBody3D = dynamic_3d();
    body.apply_force(Vector3D::new(1.0, 0.0, 0.0));
    body.apply_force(Vector3D::new(2.0, 0.0, 0.0));
    assert!(
        vec3_close(body.get_force_accumulator(), Vector3D::new(3.0, 0.0, 0.0)),
        "two forces sum, got {:?}",
        body.get_force_accumulator()
    );
}

#[test]
fn an_impulse_on_a_3d_body_scales_by_the_inverse_mass() {
    let mut body: RigidBody3D = dynamic_3d();
    body.update_mass(2.0);
    body.apply_impulse(Vector3D::new(0.0, 10.0, 0.0));
    assert!(
        vec3_close(body.get_velocity(), Vector3D::new(0.0, 5.0, 0.0)),
        "an impulse of ten on a mass of two moves at five, got {:?}",
        body.get_velocity()
    );
}

#[test]
fn an_impulse_on_a_static_3d_body_is_ignored() {
    let mut body: RigidBody3D = RigidBody3D::new_static(1, Vector3D::zero());
    body.apply_impulse(Vector3D::new(100.0, 0.0, 0.0));
    assert_eq!(
        body.get_velocity(),
        Vector3D::zero(),
        "a static body has infinite mass, so it never moves"
    );
}

#[test]
fn update_mass_of_a_3d_body_keeps_both_sides_consistent() {
    let mut body: RigidBody3D = dynamic_3d();
    body.update_mass(8.0);
    assert!(close(body.get_mass(), 8.0), "the mass is stored as given");
    assert!(
        close(body.get_inverse_mass(), 0.125),
        "the inverse mass is recomputed, got {}",
        body.get_inverse_mass()
    );
}

#[test]
fn update_inertia_recomputes_only_the_inverse() {
    let mut body: RigidBody3D = dynamic_3d();
    body.update_inertia(4.0);
    assert!(
        close(body.get_inverse_inertia(), 0.25),
        "the inverse inertia is recomputed, got {}",
        body.get_inverse_inertia()
    );
}

#[test]
fn update_inertia_of_zero_makes_rotation_immovable() {
    let mut body: RigidBody3D = dynamic_3d();
    body.update_inertia(0.0);
    assert_eq!(
        body.get_inverse_inertia(),
        0.0,
        "zero inertia is an immovable rotation, not a division by zero"
    );
}

#[test]
fn update_inertia_of_a_negative_value_makes_rotation_immovable() {
    let mut body: RigidBody3D = dynamic_3d();
    body.update_inertia(-2.0);
    assert_eq!(
        body.get_inverse_inertia(),
        0.0,
        "a negative inertia is clamped rather than flipping the sign"
    );
}

#[test]
fn the_two_dimensions_agree_on_the_mass_inverse_rule() {
    let mut flat: RigidBody2D = dynamic_2d();
    let mut solid: RigidBody3D = dynamic_3d();
    flat.update_mass(4.0);
    solid.update_mass(4.0);
    assert_eq!(
        flat.get_inverse_mass(),
        solid.get_inverse_mass(),
        "the mass rule is dimension independent"
    );
}

#[test]
fn the_two_dimensions_agree_on_impulse_scaling() {
    let mut flat: RigidBody2D = dynamic_2d();
    let mut solid: RigidBody3D = dynamic_3d();
    flat.update_mass(2.0);
    solid.update_mass(2.0);
    flat.apply_impulse(Vector2D::new(6.0, 0.0));
    solid.apply_impulse(Vector3D::new(6.0, 0.0, 0.0));
    assert_eq!(
        flat.get_velocity().get_x(),
        solid.get_velocity().get_x(),
        "the same impulse on the same mass gives the same speed in both dimensions"
    );
}

#[test]
fn a_body_without_a_collider_has_no_bounding_box() {
    let body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::new(5.0, 5.0));
    assert_eq!(
        body.bounding_box(),
        None,
        "a body is just a point until a collider is attached"
    );
}

#[test]
fn an_aabb_collider_keeps_its_own_dimensions() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::zero());
    body.update_collider(BodyCollider::Aabb(AabbCollider::from_center(
        Vector2D::zero(),
        40.0,
        20.0,
    )));
    match body.bounding_box() {
        Some(rect) => {
            assert!(
                close(rect.get_width(), 40.0),
                "an AABB must not shrink its width, got {rect:?}"
            );
            assert!(
                close(rect.get_height(), 20.0),
                "an AABB must not shrink its height, got {rect:?}"
            );
        }
        None => panic!("an AABB collider always bounds a box"),
    }
}

#[test]
fn an_aabb_of_forty_by_twenty_is_not_reported_as_twenty_by_ten() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::zero());
    body.update_collider(BodyCollider::Aabb(AabbCollider::from_center(
        Vector2D::zero(),
        40.0,
        20.0,
    )));
    match body.bounding_box() {
        Some(rect) => assert!(
            !(close(rect.get_width(), 20.0) && close(rect.get_height(), 10.0)),
            "the box came back half-size in both axes, which is what happens when the \
             centre offset is subtracted from an already-centred rect: {rect:?}"
        ),
        None => panic!("an AABB collider always bounds a box"),
    }
}

#[test]
fn an_aabb_follows_a_body_built_away_from_the_origin() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::new(25.0, 35.0));
    body.update_collider(BodyCollider::Aabb(AabbCollider::from_center(
        Vector2D::zero(),
        10.0,
        10.0,
    )));
    match body.bounding_box() {
        Some(rect) => {
            assert!(
                vec2_close(rect.center(), Vector2D::new(25.0, 35.0)),
                "the box must sit on the body, not at the origin, got {rect:?}"
            );
            assert!(
                close(rect.get_width(), 10.0) && close(rect.get_height(), 10.0),
                "translating a body must not resize its box, got {rect:?}"
            );
        }
        None => panic!("a collider is attached"),
    }
}

#[test]
fn a_body_at_a_negative_position_offsets_the_box_by_the_same_amount() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::new(-30.0, -15.0));
    body.update_collider(BodyCollider::Aabb(AabbCollider::from_center(
        Vector2D::zero(),
        20.0,
        20.0,
    )));
    match body.bounding_box() {
        Some(rect) => assert!(
            vec2_close(rect.center(), Vector2D::new(-30.0, -15.0)),
            "a negative position must offset the box, not be clamped away, got {rect:?}"
        ),
        None => panic!("a collider is attached"),
    }
}

#[test]
fn swapping_the_collider_replaces_the_bounding_box_shape() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::zero());
    body.update_collider(BodyCollider::Aabb(AabbCollider::from_center(
        Vector2D::zero(),
        40.0,
        20.0,
    )));
    body.update_collider(BodyCollider::Circle(CircleCollider::from_center(
        Vector2D::zero(),
        5.0,
    )));
    match body.bounding_box() {
        Some(rect) => {
            assert!(
                close(rect.get_width(), 10.0) && close(rect.get_height(), 10.0),
                "the circle must replace the wide AABB, got {rect:?}"
            );
        }
        None => panic!("a collider is attached"),
    }
}
