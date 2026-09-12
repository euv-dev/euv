use euv_engine::*;

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
