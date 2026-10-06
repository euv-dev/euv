use super::*;

const EPSILON: f64 = 1e-9;

#[test]
fn step_applies_torque_to_3d_angular_velocity() {
    let mut world: PhysicsWorld3D = PhysicsWorld3D::default();
    let mut body: RigidBody3D = RigidBody3D::new_dynamic(1, Vector3D::new(0.0, 0.0, 0.0));
    body.apply_torque(Vector3D::new(0.0, 0.0, 2.0));
    world.add_body(body);
    world.step(1.0);
    let omega: Vector3D = world.get_body(1).unwrap().get_angular_velocity();
    assert!(
        omega.get_x().abs() < EPSILON,
        "unexpected x angular velocity: {}",
        omega.get_x(),
    );
    assert!(
        omega.get_y().abs() < EPSILON,
        "unexpected y angular velocity: {}",
        omega.get_y(),
    );
    let expected_z: f64 = 2.0;
    assert!(
        (omega.get_z() - expected_z).abs() < EPSILON,
        "expected z angular velocity {}, got {}",
        expected_z,
        omega.get_z(),
    );
    world.step(1.0);
    let omega_after: Vector3D = world.get_body(1).unwrap().get_angular_velocity();
    assert!(
        (omega_after.get_z() - expected_z).abs() < EPSILON,
        "torque_accumulator leaked into a second step: z = {}",
        omega_after.get_z(),
    );
}

#[test]
fn step_static_3d_body_ignores_torque() {
    let mut world: PhysicsWorld3D = PhysicsWorld3D::default();
    let mut body: RigidBody3D = RigidBody3D::new_static(1, Vector3D::new(0.0, 0.0, 0.0));
    body.apply_torque(Vector3D::new(1.0, 0.0, 0.0));
    world.add_body(body);
    world.step(1.0);
    let omega: Vector3D = world.get_body(1).unwrap().get_angular_velocity();
    assert!(
        omega.get_x().abs() < EPSILON
            && omega.get_y().abs() < EPSILON
            && omega.get_z().abs() < EPSILON,
        "static body must remain rotationally inert, got ({}, {}, {})",
        omega.get_x(),
        omega.get_y(),
        omega.get_z(),
    );
}

#[test]
fn step_torque_accumulates_over_multiple_steps() {
    let mut world: PhysicsWorld3D = PhysicsWorld3D::default();
    let body: RigidBody3D = RigidBody3D::new_dynamic(1, Vector3D::new(0.0, 0.0, 0.0));
    world.add_body(body);
    for _ in 0..4 {
        world
            .get_body_mut(1)
            .unwrap()
            .apply_torque(Vector3D::new(0.0, 1.0, 0.0));
        world.step(1.0);
    }
    let omega: Vector3D = world.get_body(1).unwrap().get_angular_velocity();
    assert!(
        (omega.get_y() - 4.0).abs() < EPSILON,
        "expected cumulative angular velocity 4.0 on y axis, got {}",
        omega.get_y(),
    );
}
#[test]
fn step_2d_angular_velocity_unchanged() {
    let mut world: PhysicsWorld2D = PhysicsWorld2D::default();
    let body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::new(0.0, 0.0));
    world.add_body(body);
    world.step(1.0);
    let omega: f64 = world.get_body(1).unwrap().get_angular_velocity();
    assert!(
        omega.abs() < EPSILON,
        "2D angular velocity should remain 0 with no input, got {}",
        omega,
    );
}

#[test]
fn friction_2d_slows_tangential_slide_to_rest() {
    let config: PhysicsConfig = PhysicsConfig::new(
        Vector2D::zero(),
        DEFAULT_LINEAR_DAMPING,
        DEFAULT_ANGULAR_DAMPING,
    );
    let mut world: PhysicsWorld2D = PhysicsWorld2D::with_config(config);
    let mut ground: RigidBody2D = RigidBody2D::new_static(1, Vector2D::new(0.0, -5.0));
    ground.update_collider(BodyCollider::Aabb(AabbCollider::new(Rect::new(
        -50.0, 0.0, 100.0, 100.0,
    ))));
    world.add_body(ground);
    let mut slider: RigidBody2D = RigidBody2D::new_dynamic(2, Vector2D::new(0.0, 0.19));
    slider.update_collider(BodyCollider::Circle(CircleCollider::from_center(
        Vector2D::zero(),
        0.2,
    )));
    world.add_body(slider);
    world
        .get_body_mut(2)
        .unwrap()
        .set_velocity(Vector2D::new(2.0, 0.0));
    let mut previous: f64 = 2.0;
    let mut monotonic: bool = true;
    for _ in 0..900 {
        world.step(1.0 / 60.0);
        let tangent: f64 = world.get_body(2).unwrap().get_velocity().get_x();
        if tangent > previous + 1e-9 {
            monotonic = false;
        }
        previous = tangent;
    }
    assert!(
        monotonic,
        "tangential speed must never increase while sliding",
    );
    let final_tangent: f64 = world.get_body(2).unwrap().get_velocity().get_x();
    assert!(
        final_tangent < 0.5,
        "friction should bleed off most tangential speed, got {}",
        final_tangent,
    );
}

#[test]
fn friction_2d_frictionless_body_keeps_sliding() {
    let config: PhysicsConfig = PhysicsConfig::new(
        Vector2D::zero(),
        DEFAULT_LINEAR_DAMPING,
        DEFAULT_ANGULAR_DAMPING,
    );
    let mut world: PhysicsWorld2D = PhysicsWorld2D::with_config(config);
    let mut ground: RigidBody2D = RigidBody2D::new_static(1, Vector2D::new(0.0, -5.0));
    ground.update_collider(BodyCollider::Aabb(AabbCollider::new(Rect::new(
        -50.0, 0.0, 100.0, 100.0,
    ))));
    world.add_body(ground);
    let mut slider: RigidBody2D = RigidBody2D::new_dynamic(2, Vector2D::new(0.0, 0.19));
    slider.update_collider(BodyCollider::Circle(CircleCollider::from_center(
        Vector2D::zero(),
        0.2,
    )));
    slider.set_friction(0.0);
    world.add_body(slider);
    world
        .get_body_mut(2)
        .unwrap()
        .set_velocity(Vector2D::new(2.0, 0.0));
    for _ in 0..900 {
        world.step(1.0 / 60.0);
    }
    let final_tangent: f64 = world.get_body(2).unwrap().get_velocity().get_x();
    assert!(
        final_tangent > 1.0,
        "a zero-friction body must keep sliding, got {}",
        final_tangent,
    );
}

#[test]
fn friction_3d_slows_tangential_slide() {
    let config: PhysicsConfig3D = PhysicsConfig3D::new(
        Vector3D::zero(),
        DEFAULT_LINEAR_DAMPING,
        DEFAULT_ANGULAR_DAMPING,
    );
    let mut world: PhysicsWorld3D = PhysicsWorld3D::with_config(config);
    let mut ground: RigidBody3D = RigidBody3D::new_static(1, Vector3D::zero());
    ground.update_collider(BodyCollider3D::Sphere(SphereCollider3D::from_center(
        Vector3D::zero(),
        1.0,
    )));
    world.add_body(ground);
    let mut slider: RigidBody3D = RigidBody3D::new_dynamic(2, Vector3D::new(0.0, 1.19, 0.0));
    slider.update_collider(BodyCollider3D::Sphere(SphereCollider3D::from_center(
        Vector3D::zero(),
        0.2,
    )));
    world.add_body(slider);
    world
        .get_body_mut(2)
        .unwrap()
        .set_velocity(Vector3D::new(2.0, 0.0, 0.0));
    let mut previous: f64 = 2.0;
    let mut monotonic: bool = true;
    for _ in 0..900 {
        world.step(1.0 / 60.0);
        let tangent: f64 = world.get_body(2).unwrap().get_velocity().get_x();
        if tangent > previous + 1e-9 {
            monotonic = false;
        }
        previous = tangent;
    }
    assert!(
        monotonic,
        "tangential speed must never increase while sliding",
    );
    let final_tangent: f64 = world.get_body(2).unwrap().get_velocity().get_x();
    assert!(
        final_tangent < 2.0,
        "friction must reduce tangential speed, got {}",
        final_tangent,
    );
}

fn at(x: f64, y: f64) -> Vector2D {
    Vector2D::new(x, y)
}

fn at3(x: f64, y: f64, z: f64) -> Vector3D {
    Vector3D::new(x, y, z)
}

fn close(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 1e-9
}

#[test]
fn a_dynamic_body_is_dynamic_and_a_static_one_is_not() {
    let dynamic: RigidBody2D = RigidBody2D::new_dynamic(1, at(0.0, 0.0));
    let fixed: RigidBody2D = RigidBody2D::new_static(2, at(0.0, 0.0));
    assert!(dynamic.is_dynamic(), "new_dynamic must report dynamic");
    assert!(!fixed.is_dynamic(), "new_static must not report dynamic");
    assert_eq!(dynamic.get_id(), 1, "the id round-trips");
    assert!(
        !dynamic.get_force_accumulator().get_x().is_nan(),
        "force starts as a real number"
    );
}

#[test]
fn a_body_keeps_the_position_it_was_built_with() {
    let body: RigidBody2D = RigidBody2D::new_dynamic(7, at(3.0, -4.0));
    assert_eq!(body.get_position().get_x(), 3.0, "x round-trips");
    assert_eq!(body.get_position().get_y(), -4.0, "y round-trips");
}

#[test]
fn apply_force_accumulates_and_a_step_consumes_it() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, at(0.0, 0.0));
    body.apply_force(at(10.0, 0.0));
    assert!(
        close(body.get_force_accumulator().get_x(), 10.0),
        "the force is held until the next step"
    );
    let mut world: PhysicsWorld2D = PhysicsWorld2D::with_config(PhysicsConfig::default());
    world.add_body(body);
    world.step(1.0);
    let moved: Option<&RigidBody2D> = world.get_body(1);
    let after: &RigidBody2D = moved.expect("the body must still be in the world");
    assert!(
        after.get_position().get_x() > 0.0,
        "the force accelerated it along +x"
    );
    assert!(
        close(after.get_force_accumulator().get_x(), 0.0),
        "a step must clear the accumulator"
    );
}

#[test]
fn apply_impulse_changes_velocity_directly() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, at(0.0, 0.0));
    let before: f64 = body.get_velocity().get_x();
    body.apply_impulse(at(5.0, 0.0));
    let after: f64 = body.get_velocity().get_x();
    assert!(
        after > before,
        "an impulse must raise the velocity immediately, {before} -> {after}"
    );
    assert!(after > 0.0, "and it must point along the impulse");
}

#[test]
fn a_zero_mass_body_becomes_immovable() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, at(0.0, 0.0));
    body.update_mass(0.0);
    assert!(
        close(body.get_inverse_mass(), 0.0),
        "an infinite mass has a zero inverse mass"
    );
    let before: f64 = body.get_position().get_x();
    body.apply_force(at(1000.0, 0.0));
    let mut world: PhysicsWorld2D = PhysicsWorld2D::with_config(PhysicsConfig::default());
    world.add_body(body);
    world.step(1.0);
    let after: f64 = world
        .get_body(1)
        .expect("body present")
        .get_position()
        .get_x();
    assert!(
        close(before, after),
        "an immovable body must not be accelerated, {before} -> {after}"
    );
}

#[test]
fn bounding_box_is_none_without_a_collider_and_some_with_one() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, at(0.0, 0.0));
    assert!(
        body.bounding_box().is_none(),
        "a body with no collider has no box"
    );
    body.update_collider(BodyCollider::Aabb(AabbCollider::from_center(
        at(0.0, 0.0),
        2.0,
        4.0,
    )));
    let box_rect: Rect = body.bounding_box().expect("a collider yields a box");
    assert!(
        close(box_rect.size().get_x(), 2.0),
        "the box is twice the half width"
    );
    assert!(
        close(box_rect.size().get_y(), 4.0),
        "and twice the half height"
    );
}

#[test]
fn three_dimensional_bodies_cover_the_same_ground() {
    let mut body: RigidBody3D = RigidBody3D::new_dynamic(1, at3(0.0, 0.0, 0.0));
    let fixed: RigidBody3D = RigidBody3D::new_static(2, at3(0.0, 0.0, 0.0));
    assert!(body.is_dynamic(), "new_dynamic reports dynamic");
    assert!(!fixed.is_dynamic(), "new_static does not");
    let before: f64 = body.get_angular_velocity().get_x();
    body.apply_torque(at3(0.0, 0.0, 4.0));
    assert!(
        close(body.get_torque_accumulator().get_z(), 4.0),
        "a torque is accumulated, not applied instantly"
    );
    assert!(
        close(body.get_angular_velocity().get_x(), before),
        "and the angular velocity does not move until a step"
    );
    let mut world: PhysicsWorld3D = PhysicsWorld3D::with_config(PhysicsConfig3D::default());
    world.add_body(body);
    world.step(1.0);
    let stepped: &RigidBody3D = world.get_body(1).expect("body 1 must be present");
    assert!(
        stepped.get_angular_velocity().get_z() > 0.0,
        "the step turns the accumulated torque into spin, got {:?}",
        stepped.get_angular_velocity()
    );
    assert!(
        close(stepped.get_torque_accumulator().get_z(), 0.0),
        "and clears the accumulator"
    );
}

#[test]
fn the_world_looks_bodies_up_and_forgets_them_by_id() {
    let mut world: PhysicsWorld2D = PhysicsWorld2D::with_config(PhysicsConfig::default());
    world.add_body(RigidBody2D::new_dynamic(1, at(0.0, 0.0)));
    world.add_body(RigidBody2D::new_dynamic(2, at(10.0, 0.0)));
    assert!(world.get_body(1).is_some(), "body 1 was added");
    assert!(world.get_body(2).is_some(), "body 2 was added");
    assert!(world.get_body(3).is_none(), "body 3 was never added");
    world.remove_body(1);
    assert!(world.get_body(1).is_none(), "body 1 was removed");
    assert!(
        world.get_body(2).is_some(),
        "removing one body keeps the other"
    );
}

#[test]
fn a_mutable_borrow_lets_the_caller_move_a_body_in_place() {
    let mut world: PhysicsWorld2D = PhysicsWorld2D::with_config(PhysicsConfig::default());
    world.add_body(RigidBody2D::new_dynamic(1, at(0.0, 0.0)));
    let borrowed: Option<&mut RigidBody2D> = world.get_body_mut(1);
    let body: &mut RigidBody2D = borrowed.expect("body 1 must be present");
    body.set_position(at(42.0, 7.0));
    assert_eq!(
        world.get_body(1).expect("body 1").get_position().get_x(),
        42.0,
        "the write through the mutable borrow stuck"
    );
}

#[test]
fn stepping_an_empty_or_singleton_world_is_harmless() {
    let mut empty: PhysicsWorld2D = PhysicsWorld2D::with_config(PhysicsConfig::default());
    empty.step(1.0);
    empty.step(0.016);
    let mut single: PhysicsWorld2D = PhysicsWorld2D::with_config(PhysicsConfig::default());
    single.add_body(RigidBody2D::new_dynamic(1, at(0.0, 0.0)));
    single.step(0.016);
    assert!(
        single.get_body(1).is_some(),
        "a single body needs no pair resolution and must survive"
    );
}

#[test]
fn a_static_body_is_not_integrated_by_a_step() {
    let mut world: PhysicsWorld2D = PhysicsWorld2D::with_config(PhysicsConfig::default());
    world.add_body(RigidBody2D::new_static(1, at(5.0, 5.0)));
    world.step(1.0);
    let body: &RigidBody2D = world.get_body(1).expect("body 1 must be present");
    assert!(
        close(body.get_position().get_x(), 5.0),
        "a static body must not drift, got {:?}",
        body.get_position()
    );
}

#[test]
fn gravity_pulls_a_free_body_towards_the_configured_direction() {
    let mut config: PhysicsConfig = PhysicsConfig::default();
    config.set_gravity(at(0.0, -10.0));
    let mut world: PhysicsWorld2D = PhysicsWorld2D::with_config(config);
    world.add_body(RigidBody2D::new_dynamic(1, at(0.0, 0.0)));
    world.step(0.5);
    let body: &RigidBody2D = world.get_body(1).expect("body 1 must be present");
    assert!(
        body.get_position().get_y() < 0.0,
        "downward gravity must move it to negative y, got {:?}",
        body.get_position()
    );
}

#[test]
fn zero_inertia_makes_a_three_dimensional_body_immovable_about_every_axis() {
    let mut body: RigidBody3D = RigidBody3D::new_dynamic(1, at3(0.0, 0.0, 0.0));
    body.update_inertia(0.0);
    assert!(
        close(body.get_inverse_inertia(), 0.0),
        "zero inertia gives zero inverse inertia"
    );
    body.apply_torque(at3(0.0, 0.0, 4.0));
    let mut world: PhysicsWorld3D = PhysicsWorld3D::with_config(PhysicsConfig3D::default());
    world.add_body(body);
    world.step(1.0);
    let stepped: &RigidBody3D = world.get_body(1).expect("body 1 must be present");
    assert!(
        close(stepped.get_angular_velocity().get_z(), 0.0),
        "a body with no inertia cannot be spun, got {:?}",
        stepped.get_angular_velocity()
    );
}

#[test]
fn a_dynamic_body_reports_the_dynamic_body_type() {
    let body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::new(4.0, 5.0));
    assert_eq!(body.get_body_type(), BodyType::Dynamic);
    assert!(body.is_dynamic(), "a dynamic body takes part in simulation");
}

#[test]
fn a_static_body_reports_the_static_body_type_and_stays_put() {
    let body: RigidBody2D = RigidBody2D::new_static(2, Vector2D::new(4.0, 5.0));
    assert_eq!(body.get_body_type(), BodyType::Static);
    assert!(!body.is_dynamic(), "a static body is never moved by forces");
}

#[test]
fn the_three_dimensional_constructors_set_the_same_two_body_types() {
    assert_eq!(
        RigidBody3D::new_dynamic(3, Vector3D::zero()).get_body_type(),
        BodyType::Dynamic
    );
    assert_eq!(
        RigidBody3D::new_static(4, Vector3D::zero()).get_body_type(),
        BodyType::Static
    );
}
