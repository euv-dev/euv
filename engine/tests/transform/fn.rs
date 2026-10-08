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

fn quat_close(left: Quaternion, right: Quaternion) -> bool {
    let a: String = format!("{:?}", left);
    let b: String = format!("{:?}", right);
    let numbers: fn(&str) -> Vec<f64> = |text: &str| -> Vec<f64> {
        text.replace(['{', '}'], "")
            .split(", ")
            .filter_map(|part: &str| part.split(": ").nth(1)?.parse::<f64>().ok())
            .collect()
    };
    let left_values: Vec<f64> = numbers(&a);
    let right_values: Vec<f64> = numbers(&b);
    left_values.len() == right_values.len()
        && left_values
            .iter()
            .zip(right_values.iter())
            .all(|pair: (&f64, &f64)| close(*pair.0, *pair.1))
}

#[test]
fn from_rgb_scales_each_channel_into_the_unit_range() {
    let observed: Color = Color::from_rgb(255, 128, 0);
    assert_eq!(
        observed.to_css_rgba(),
        "rgba(255, 128, 0, 1)",
        "255 maps to one, 128 rounds to 128, and the alpha starts opaque"
    );
}

#[test]
fn from_rgb_of_zero_is_opaque_black() {
    let observed: Color = Color::from_rgb(0, 0, 0);
    assert_eq!(observed, Color::black(), "all channels at zero is black");
}

#[test]
fn from_rgb_of_full_is_opaque_white() {
    let observed: Color = Color::from_rgb(255, 255, 255);
    assert_eq!(observed, Color::white(), "all channels at full is white");
}

#[test]
fn black_renders_as_opaque_black() {
    let observed: String = Color::black().to_css_rgba();
    assert_eq!(observed, "rgba(0, 0, 0, 1)", "black is opaque");
}

#[test]
fn white_renders_as_opaque_white() {
    let observed: String = Color::white().to_css_rgba();
    assert_eq!(observed, "rgba(255, 255, 255, 1)", "white is opaque");
}

#[test]
fn transparent_renders_as_see_through_black() {
    let observed: String = Color::transparent().to_css_rgba();
    assert_eq!(
        observed, "rgba(0, 0, 0, 0)",
        "transparent keeps the channels but drops the alpha"
    );
}

#[test]
fn transparent_differs_from_black_only_in_alpha() {
    let black: Color = Color::black();
    let clear: Color = Color::transparent();
    assert_ne!(black, clear, "black and transparent only differ in alpha");
    assert_eq!(
        black
            .to_css_rgba()
            .replace("rgba(0, 0, 0, 1)", "rgba(0, 0, 0, 0)"),
        clear.to_css_rgba(),
        "only the trailing alpha differs"
    );
}

#[test]
fn a_fractional_alpha_is_printed_as_a_fraction() {
    let observed: String = Color::new(1.0, 1.0, 1.0, 0.5).to_css_rgba();
    assert_eq!(
        observed, "rgba(255, 255, 255, 0.5)",
        "the alpha keeps its fractional value, got {observed}"
    );
}

#[test]
fn write_css_rgba_appends_to_an_existing_buffer() {
    let mut buffer: String = String::from("prefix:");
    let _: fmt::Result = write!(buffer, "{}", Color::white().to_css_rgba());
    assert!(
        buffer.starts_with("prefix:"),
        "the call appends rather than replacing, got {buffer}"
    );
}

#[test]
fn color_lerp_takes_the_midpoint_of_two_colors() {
    let observed: Color = Color::black().lerp(Color::white(), 0.5);
    assert_eq!(
        observed.to_css_rgba(),
        "rgba(128, 128, 128, 1)",
        "halfway between black and white is mid grey, got {}",
        observed.to_css_rgba()
    );
}

#[test]
fn color_lerp_at_the_endpoints_lands_on_each_end() {
    assert_eq!(Color::black().lerp(Color::white(), 0.0), Color::black());
    assert_eq!(Color::black().lerp(Color::white(), 1.0), Color::white());
}

#[test]
fn a_default_quaternion_is_the_identity() {
    let observed: Quaternion = Quaternion::identity();
    assert_eq!(
        observed,
        Quaternion::new(0.0, 0.0, 0.0, 1.0),
        "identity is w = 1"
    );
}

#[test]
fn the_identity_quaternion_is_unit_length() {
    let observed: f64 = Quaternion::identity().magnitude();
    assert!(
        close(observed, 1.0),
        "a unit quaternion is 1 long, got {observed}"
    );
}

#[test]
fn a_zero_angle_rotation_about_any_axis_is_the_identity() {
    let observed: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), 0.0);
    assert_eq!(
        observed,
        Quaternion::identity(),
        "a zero turn changes nothing"
    );
}

#[test]
fn an_axis_angle_quaternion_is_unit_length() {
    let observed: f64 = Quaternion::from_axis_angle(Vector3D::new(0.0, 1.0, 0.0), 0.7).magnitude();
    assert!(
        close(observed, 1.0),
        "an axis-angle rotation is always a unit quaternion, got {observed}"
    );
}

#[test]
fn a_half_turn_about_z_puts_the_vector_on_the_negative_side() {
    let observed: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), PI);
    assert!(
        quat_close(observed, Quaternion::new(0.0, 0.0, 1.0, 0.0)),
        "a half turn about z is a pure z sine with zero cosine, got {observed:?}"
    );
}

#[test]
fn a_quaternion_from_euler_angles_is_unit_length() {
    let observed: f64 = Quaternion::from_euler(0.3, -0.2, 1.1).magnitude();
    assert!(
        close(observed, 1.0),
        "euler angles compose to a unit, got {observed}"
    );
}

#[test]
fn euler_angles_of_zero_are_the_identity() {
    let observed: Quaternion = Quaternion::from_euler(0.0, 0.0, 0.0);
    assert_eq!(observed, Quaternion::identity(), "no rotation is identity");
}

#[test]
fn the_conjugate_of_the_identity_is_the_identity() {
    let observed: Quaternion = Quaternion::identity().conjugate();
    assert_eq!(
        observed,
        Quaternion::identity(),
        "identity is its own conjugate"
    );
}

#[test]
fn a_quaternion_dotted_with_itself_is_its_length_squared() {
    let q: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), 0.5);
    let observed: f64 = q.dot(q);
    assert!(
        close(observed, 1.0),
        "a unit quaternion dots itself to one, got {observed}"
    );
}

#[test]
fn normalizing_a_quaternion_brings_it_back_to_unit_length() {
    let q: Quaternion = Quaternion::new(0.0, 0.0, 0.0, 4.0);
    let observed: f64 = q.normalized().magnitude();
    assert!(
        close(observed, 1.0),
        "normalizing rescales to one, got {observed}"
    );
}

#[test]
fn normalizing_keeps_the_direction_of_the_rotation() {
    let q: Quaternion = Quaternion::new(0.0, 0.0, 0.0, 4.0);
    assert_eq!(
        q.normalized(),
        Quaternion::identity(),
        "scaling the same rotation by four normalises back to identity"
    );
}

#[test]
fn multiplying_by_the_identity_on_the_left_changes_nothing() {
    let q: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), 0.4);
    let observed: Quaternion = Quaternion::identity() * q;
    assert_eq!(
        observed, q,
        "identity is the left unit of the quaternion group"
    );
}

#[test]
fn multiplying_by_the_identity_on_the_right_changes_nothing() {
    let q: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), 0.4);
    let observed: Quaternion = q * Quaternion::identity();
    assert_eq!(observed, q, "identity is the right unit too");
}

#[test]
fn slerp_towards_itself_returns_the_same_rotation() {
    let q: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), 0.6);
    let observed: Quaternion = q.slerp(q, 0.5);
    assert_eq!(observed, q, "slerping to the same rotation is a no-op");
}

#[test]
fn slerp_at_the_endpoints_lands_on_each_end() {
    let a: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), 0.0);
    let b: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), 1.0);
    assert_eq!(a.slerp(b, 0.0), a, "factor zero returns the start");
    assert_eq!(a.slerp(b, 1.0), b, "factor one returns the end");
}

#[test]
fn slerp_halfway_is_still_a_unit_quaternion() {
    let a: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), 0.0);
    let b: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), 1.2);
    let observed: f64 = a.slerp(b, 0.5).magnitude();
    assert!(
        close(observed, 1.0),
        "slerp preserves the unit sphere, got {observed}"
    );
}

#[test]
fn slerp_halfway_turns_half_as_far_as_the_ends() {
    let a: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), 0.0);
    let b: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), 2.0);
    let half: Quaternion = a.slerp(b, 0.5);
    let expected: Quaternion = Quaternion::from_axis_angle(Vector3D::new(0.0, 0.0, 1.0), 1.0);
    assert!(
        quat_close(half, expected),
        "slerp travels the great-circle arc at a constant angular rate, got {half:?}"
    );
}

#[test]
fn the_identity_matrix_leaves_a_point_where_it_is() {
    let point: Vector3D = Vector3D::new(3.0, -4.0, 5.0);
    let observed: Vector3D = Matrix4x4::identity().transform_point(point);
    assert!(
        vec3_close(observed, point),
        "identity is a no-op, got {observed:?}"
    );
}

#[test]
fn a_translation_matrix_shifts_a_point() {
    let point: Vector3D = Vector3D::new(1.0, 2.0, 3.0);
    let offset: Vector3D = Vector3D::new(10.0, 20.0, 30.0);
    let observed: Vector3D = Matrix4x4::translation(offset).transform_point(point);
    assert!(
        vec3_close(observed, Vector3D::new(11.0, 22.0, 33.0)),
        "the point moves by the offset, got {observed:?}"
    );
}

#[test]
fn a_scaling_matrix_scales_each_axis_independently() {
    let point: Vector3D = Vector3D::new(1.0, 2.0, 3.0);
    let factors: Vector3D = Vector3D::new(2.0, 0.5, 10.0);
    let observed: Vector3D = Matrix4x4::scaling(factors).transform_point(point);
    assert!(
        vec3_close(observed, Vector3D::new(2.0, 1.0, 30.0)),
        "each axis scales on its own, got {observed:?}"
    );
}

#[test]
fn a_translation_by_zero_is_the_identity() {
    let observed: Matrix4x4 = Matrix4x4::translation(Vector3D::zero());
    assert_eq!(
        observed,
        Matrix4x4::identity(),
        "translating by nothing leaves the identity matrix"
    );
}

#[test]
fn a_scaling_by_one_is_the_identity() {
    let observed: Matrix4x4 = Matrix4x4::scaling(Vector3D::new(1.0, 1.0, 1.0));
    assert_eq!(
        observed,
        Matrix4x4::identity(),
        "scaling by one on every axis is the identity"
    );
}

#[test]
fn rotating_by_the_identity_quaternion_is_the_identity_matrix() {
    let observed: Matrix4x4 = Matrix4x4::rotation(Quaternion::identity());
    assert_eq!(observed, Matrix4x4::identity(), "no rotation is identity");
}

#[test]
fn multiplying_by_the_identity_on_the_right_is_a_no_op() {
    let matrix: Matrix4x4 = Matrix4x4::translation(Vector3D::new(1.0, 2.0, 3.0));
    assert_eq!(
        matrix.multiply(Matrix4x4::identity()),
        matrix,
        "identity is the right unit of matrix composition"
    );
}

#[test]
fn multiplying_by_the_identity_on_the_left_is_a_no_op() {
    let matrix: Matrix4x4 = Matrix4x4::translation(Vector3D::new(1.0, 2.0, 3.0));
    assert_eq!(
        Matrix4x4::identity().multiply(matrix),
        matrix,
        "identity is the left unit too"
    );
}

#[test]
fn two_translations_compose_into_one() {
    let a: Matrix4x4 = Matrix4x4::translation(Vector3D::new(1.0, 0.0, 0.0));
    let b: Matrix4x4 = Matrix4x4::translation(Vector3D::new(0.0, 2.0, 0.0));
    let observed: Vector3D = a.multiply(b).transform_point(Vector3D::zero());
    assert!(
        vec3_close(observed, Vector3D::new(1.0, 2.0, 0.0)),
        "composing two moves is one move by their sum, got {observed:?}"
    );
}

#[test]
fn a_translation_then_a_scale_compose_in_that_order() {
    let move_it: Matrix4x4 = Matrix4x4::translation(Vector3D::new(10.0, 0.0, 0.0));
    let grow: Matrix4x4 = Matrix4x4::scaling(Vector3D::new(2.0, 2.0, 2.0));
    let observed: Vector3D = move_it
        .multiply(grow)
        .transform_point(Vector3D::new(1.0, 0.0, 0.0));
    assert!(
        vec3_close(observed, Vector3D::new(12.0, 0.0, 0.0)),
        "the rightmost matrix applies first, so the point grows then moves, got {observed:?}"
    );
}

#[test]
fn a_perspective_matrix_is_not_the_identity() {
    let observed: Matrix4x4 = Matrix4x4::perspective(PI / 4.0, 1.0, 0.1, 100.0);
    assert_ne!(
        observed,
        Matrix4x4::identity(),
        "a projection always changes the mapping"
    );
}

#[test]
fn an_orthographic_matrix_is_not_the_identity() {
    let observed: Matrix4x4 = Matrix4x4::orthographic(-1.0, 1.0, 1.0, -1.0, 0.1, 100.0);
    assert_ne!(
        observed,
        Matrix4x4::identity(),
        "an orthographic projection always changes the mapping"
    );
}

#[test]
fn a_look_at_matrix_points_the_eye_at_its_target() {
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 5.0);
    let target: Vector3D = Vector3D::zero();
    let up: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let observed: Vector3D = Matrix4x4::look_at(eye, target, up).transform_point(eye);
    assert!(
        close(observed.get_z(), 0.0),
        "the view matrix maps the eye onto the near plane at z = 0, got {observed:?}"
    );
}

#[test]
fn the_identity_transform_leaves_a_point_where_it_is() {
    let point: Vector2D = Vector2D::new(3.0, -4.0);
    let observed: Vector2D = Transform2D::identity().apply_to_point(point);
    assert_eq!(observed, point, "identity is a no-op, got {observed:?}");
}

#[test]
fn a_default_transform_is_the_identity() {
    let observed: Transform2D = Transform2D::default();
    let point: Vector2D = Vector2D::new(1.0, 2.0);
    assert_eq!(
        observed.apply_to_point(point),
        point,
        "the derive default is the identity transform"
    );
}

#[test]
fn translating_a_transform_shifts_a_point() {
    let mut transform: Transform2D = Transform2D::identity();
    transform.translate(Vector2D::new(10.0, 20.0));
    let observed: Vector2D = transform.apply_to_point(Vector2D::new(1.0, 2.0));
    assert_eq!(
        observed,
        Vector2D::new(11.0, 22.0),
        "the point moves, got {observed:?}"
    );
}

#[test]
fn scaling_a_transform_multiplies_a_point() {
    let mut transform: Transform2D = Transform2D::identity();
    transform.scale_by(Vector2D::new(2.0, 3.0));
    let observed: Vector2D = transform.apply_to_point(Vector2D::new(1.0, 2.0));
    assert_eq!(
        observed,
        Vector2D::new(2.0, 6.0),
        "each axis scales, got {observed:?}"
    );
}

#[test]
fn rotating_a_transform_turns_a_point() {
    let mut transform: Transform2D = Transform2D::identity();
    transform.rotate(HALF_PI);
    let observed: Vector2D = transform.apply_to_point(Vector2D::new(1.0, 0.0));
    assert!(
        close(observed.get_x(), 0.0) && close(observed.get_y(), 1.0),
        "a quarter turn sends +x to +y, got {observed:?}"
    );
}

#[test]
fn a_rotation_by_zero_turns_nothing() {
    let mut transform: Transform2D = Transform2D::identity();
    transform.rotate(0.0);
    let observed: Vector2D = transform.apply_to_point(Vector2D::new(3.0, 4.0));
    assert_eq!(observed, Vector2D::new(3.0, 4.0), "a zero turn is a no-op");
}

#[test]
fn transform_operations_accumulate_rather_than_replace() {
    let mut transform: Transform2D = Transform2D::identity();
    transform.translate(Vector2D::new(1.0, 0.0));
    transform.scale_by(Vector2D::new(2.0, 2.0));
    let observed: Vector2D = transform.apply_to_point(Vector2D::new(1.0, 0.0));
    assert_eq!(
        observed,
        Vector2D::new(3.0, 0.0),
        "the point scales first and then shifts, got {observed:?}"
    );
}
