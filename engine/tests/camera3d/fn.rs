use super::*;

const TOLERANCE: f64 = 1e-6;

fn camera() -> Camera3D {
    Camera3D::create(Vector3D::new(0.0, 0.0, 5.0), Vector3D::zero(), 800.0, 600.0)
}

fn view_space(camera: &Camera3D, world: Vector3D) -> Vector3D {
    camera.view_matrix().transform_point(world)
}

fn orbit_radius(camera: &Camera3D) -> f64 {
    let target_in_view: Vector3D = view_space(camera, Vector3D::zero());
    target_in_view.get_z().abs()
}

fn assert_near(observed: Vector3D, expected: Vector3D, what: &str) {
    assert!(
        (observed.get_x() - expected.get_x()).abs() < TOLERANCE
            && (observed.get_y() - expected.get_y()).abs() < TOLERANCE
            && (observed.get_z() - expected.get_z()).abs() < TOLERANCE,
        "{what}: expected {expected:?}, got {observed:?}"
    );
}

#[test]
fn the_view_matrix_puts_the_eye_at_the_view_origin() {
    let camera: Camera3D = camera();
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 5.0);
    assert_near(
        view_space(&camera, eye),
        Vector3D::zero(),
        "the eye of a look-at camera is always the view-space origin",
    );
}

#[test]
fn the_view_matrix_puts_the_target_straight_down_the_negative_z_axis() {
    let camera: Camera3D = camera();
    assert_near(
        view_space(&camera, Vector3D::zero()),
        Vector3D::new(0.0, 0.0, -5.0),
        "a camera 5 units back from the origin must see it 5 units ahead",
    );
}

#[test]
fn the_view_matrix_leaves_the_up_axis_vertical_for_a_level_camera() {
    let camera: Camera3D = camera();
    let up_in_view: Vector3D = view_space(&camera, Vector3D::up());
    assert!(
        (up_in_view.get_x()).abs() < TOLERANCE,
        "for a camera with no roll the world up axis must stay vertical, got {up_in_view:?}"
    );
    assert!(
        up_in_view.get_y() > 0.0,
        "a level camera keeps looking up as up, got {up_in_view:?}"
    );
}

#[test]
fn the_combined_matrix_puts_the_view_target_inside_normalised_device_coordinates() {
    let camera: Camera3D = camera();
    let clip: Vector3D = camera.view_proj_matrix().transform_point(Vector3D::zero());
    assert!(
        clip.get_x().abs() <= 1.0 && clip.get_y().abs() <= 1.0,
        "a point on the view axis must project to the centre of the frustum, got {clip:?}"
    );
    assert!(
        clip.get_z() >= -1.0 && clip.get_z() <= 1.0,
        "a point in front of the camera must project inside the depth range, got {clip:?}"
    );
}

#[test]
fn a_point_the_camera_looks_at_is_inside_its_frustum() {
    let camera: Camera3D = camera();
    assert!(
        camera.in_frustum(Vector3D::zero()),
        "the point a camera was pointed at is by definition visible"
    );
}

#[test]
fn a_point_behind_the_camera_is_outside_its_frustum() {
    let camera: Camera3D = camera();
    assert!(
        !camera.in_frustum(Vector3D::new(0.0, 0.0, 50.0)),
        "a point 45 units behind a camera at z=5 must not be reported as visible"
    );
}

#[test]
fn a_point_far_off_to_the_side_is_outside_the_frustum() {
    let camera: Camera3D = camera();
    assert!(
        !camera.in_frustum(Vector3D::new(1000.0, 0.0, 0.0)),
        "the horizontal field of view is narrow, so a distant lateral point must be culled"
    );
}

#[test]
fn orbiting_keeps_the_camera_the_same_distance_from_its_target() {
    let mut camera: Camera3D = camera();
    let before: f64 = orbit_radius(&camera);
    camera.orbit(PI / 3.0, 0.2);
    let after: f64 = orbit_radius(&camera);
    assert!(
        (before - after).abs() < TOLERANCE,
        "an orbit rotates around the target, it must not change the orbit radius: {before} -> {after}"
    );
}

#[test]
fn a_full_turn_of_yaw_returns_the_camera_to_its_original_side() {
    let mut camera: Camera3D = camera();
    let start: Vector3D = Vector3D::new(0.0, 0.0, 5.0);
    camera.orbit(TAU, 0.0);
    assert_near(
        view_space(&camera, start),
        Vector3D::zero(),
        "after a full revolution the starting eye point is the eye again",
    );
}

#[test]
fn orbiting_yaw_moves_the_eye_off_its_starting_side() {
    let mut camera: Camera3D = camera();
    let start: Vector3D = Vector3D::new(0.0, 0.0, 5.0);
    camera.orbit(PI, 0.0);
    let moved: Vector3D = view_space(&camera, start);
    assert!(
        moved.get_z() < -1.0,
        "a half turn swings the camera to the far side of the target, so the old eye ends up \
         two radii straight ahead of it, got {moved:?}"
    );
    assert_near(
        moved,
        Vector3D::new(0.0, 0.0, -10.0),
        "a half turn from radius 5 puts the old eye exactly 10 units ahead",
    );
}

#[test]
fn orbiting_saturates_at_the_pitch_clamp_instead_of_tumbling_over_the_pole() {
    let mut camera: Camera3D = camera();
    let radius: f64 = orbit_radius(&camera);
    camera.orbit(0.0, PI);
    let saturated: f64 = orbit_radius(&camera);
    assert!(
        (radius - saturated).abs() < TOLERANCE,
        "clamping the pitch must still keep the camera on the orbit sphere: {radius} -> {saturated}"
    );
    let repeated: f64 = {
        camera.orbit(0.0, PI);
        orbit_radius(&camera)
    };
    assert!(
        (saturated - repeated).abs() < TOLERANCE,
        "the pitch clamp is symmetric, so a second oversized pitch must not drift further"
    );
    assert!(
        camera.in_frustum(Vector3D::new(0.0, -5.0, 0.0)),
        "clamping an upward pitch must leave the camera above the target looking down, \
         so a point below the target is in front of it"
    );
}

#[test]
fn the_target_stays_centred_in_the_viewport_whatever_the_orbit() {
    let mut camera: Camera3D = camera();
    camera.orbit(1.1, -0.4);
    let projected: Vector3D = camera.world_to_screen(Vector3D::zero());
    assert!(
        (projected.get_x() - 400.0).abs() < 1e-3 && (projected.get_y() - 300.0).abs() < 1e-3,
        "orbiting turns the camera around its target, so the target must stay in the centre, got {projected:?}"
    );
}

#[test]
fn zooming_moves_the_camera_toward_the_target_rather_than_along_a_world_axis() {
    let target: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let mut camera: Camera3D =
        Camera3D::create(Vector3D::new(0.0, 0.0, 10.0), target, 800.0, 600.0);
    let before: f64 = (camera.get_position() - target).magnitude();

    camera.zoom(2.5);

    let after: f64 = (camera.get_position() - target).magnitude();
    assert!(
        after < before,
        "zoom is a step along the camera's own forward vector, so it has to shorten the \
         distance to the target — a step along a fixed world axis would leave it unchanged \
         for any other viewing angle"
    );
    assert!(
        (before - after - 2.5).abs() < 1e-9,
        "and by exactly the distance asked for, because forward is a normalized vector"
    );
    assert_eq!(
        camera.get_position().get_x(),
        0.0,
        "this target is straight ahead on the z axis, so x cannot drift"
    );
}

#[test]
fn zooming_twice_moves_twice_as_far_as_zooming_once() {
    let mut camera: Camera3D = Camera3D::create(
        Vector3D::new(0.0, 0.0, 10.0),
        Vector3D::new(0.0, 0.0, 0.0),
        800.0,
        600.0,
    );
    let start: Vector3D = camera.get_position();
    let one_step: Vector3D = camera.get_position() + camera.forward().scaled(3.0);

    camera.zoom(3.0);
    let after_one: Vector3D = camera.get_position();
    camera.zoom(3.0);
    let after_two: Vector3D = camera.get_position();

    let first_move: Vector3D = after_one - start;
    let second_move: Vector3D = after_two - after_one;
    assert!(
        (second_move.get_z() - first_move.get_z()).abs() < 1e-9,
        "each zoom is a plain translation along the current forward vector, so the second step \
         is the same length as the first; anything else means the distance is being scaled \
         rather than stepped, which compounds differently on every call"
    );
    assert!(
        (after_one.get_z() - one_step.get_z()).abs() < 1e-9,
        "and one step lands where position + forward * distance says it should"
    );
}
