use super::*;

fn close(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 1e-9
}

fn v2(x: f64, y: f64) -> Vector2D {
    Vector2D::new(x, y)
}

fn v3(x: f64, y: f64, z: f64) -> Vector3D {
    Vector3D::new(x, y, z)
}

#[test]
fn clamp_bounds_a_value_between_min_and_max() {
    assert!(
        close(Numeric::clamp(5.0, 0.0, 10.0), 5.0),
        "inside the range passes through"
    );
    assert!(
        close(Numeric::clamp(-3.0, 0.0, 10.0), 0.0),
        "below the range clamps to min"
    );
    assert!(
        close(Numeric::clamp(42.0, 0.0, 10.0), 10.0),
        "above the range clamps to max"
    );
}

const TWO_PI: f64 = TAU;
const DEG_TO_RAD: f64 = PI / 180.0;
const RAD_TO_DEG: f64 = 180.0 / PI;
const HALF_PI: f64 = FRAC_PI_2;
const EPSILON: f64 = 1e-9;
#[test]
fn clamp_keeps_a_value_inside_the_bounds() {
    let observed: f64 = Numeric::clamp(0.5, 0.0, 1.0);
    assert_eq!(observed, 0.5, "a value between the bounds passes through");
}

#[test]
fn clamp_pulls_a_value_up_to_the_minimum() {
    let observed: f64 = Numeric::clamp(-3.0, 0.0, 1.0);
    assert_eq!(observed, 0.0, "a value below the range clamps to min");
}

#[test]
fn clamp_pulls_a_value_down_to_the_maximum() {
    let observed: f64 = Numeric::clamp(7.0, 0.0, 1.0);
    assert_eq!(observed, 1.0, "a value above the range clamps to max");
}

#[test]
fn clamp_handles_a_negative_range() {
    let observed: f64 = Numeric::clamp(-5.0, -2.0, -1.0);
    assert_eq!(observed, -2.0, "a negative window clamps the same way");
}

#[test]
fn lerp_returns_the_start_at_factor_zero() {
    let observed: f64 = Numeric::lerp(10.0, 20.0, 0.0);
    assert_eq!(observed, 10.0, "factor zero lands on the start value");
}

#[test]
fn lerp_returns_the_end_at_factor_one() {
    let observed: f64 = Numeric::lerp(10.0, 20.0, 1.0);
    assert_eq!(observed, 20.0, "factor one lands on the end value");
}

#[test]
fn lerp_returns_the_midpoint_at_the_halfway_factor() {
    let observed: f64 = Numeric::lerp(10.0, 20.0, 0.5);
    assert_eq!(observed, 15.0, "the halfway factor is the midpoint");
}

#[test]
fn lerp_works_on_a_descending_range() {
    let observed: f64 = Numeric::lerp(20.0, 10.0, 0.25);
    assert_eq!(observed, 17.5, "a descending range lerps the same way");
}

#[test]
fn deg_to_rad_converts_a_half_turn() {
    let observed: f64 = Numeric::deg_to_rad(180.0);
    assert!(
        (observed - PI).abs() < EPSILON,
        "180 degrees is pi radians, got {observed}"
    );
}

#[test]
fn lerp_interpolates_between_the_endpoints() {
    assert!(
        close(Numeric::lerp(0.0, 10.0, 0.0), 0.0),
        "factor zero is the start"
    );
    assert!(
        close(Numeric::lerp(0.0, 10.0, 1.0), 10.0),
        "factor one is the end"
    );
    assert!(
        close(Numeric::lerp(0.0, 10.0, 0.5), 5.0),
        "halfway is the midpoint"
    );
}

#[test]
fn deg_to_rad_converts_a_quarter_turn() {
    let observed: f64 = Numeric::deg_to_rad(90.0);
    assert!(
        (observed - HALF_PI).abs() < EPSILON,
        "90 degrees is half pi radians, got {observed}"
    );
}

#[test]
fn degree_and_radian_conversion_round_trips() {
    let radians: f64 = Numeric::deg_to_rad(180.0);
    assert!(close(radians, PI), "180 degrees is pi radians");
    assert!(
        close(Numeric::rad_to_deg(PI), 180.0),
        "pi radians is 180 degrees"
    );
    assert!(
        close(Numeric::deg_to_rad(90.0), FRAC_PI_2),
        "90 degrees is pi/2"
    );
}

#[test]
fn rad_to_deg_converts_a_half_turn() {
    let observed: f64 = Numeric::rad_to_deg(PI);
    assert!(
        (observed - 180.0).abs() < EPSILON,
        "pi radians is 180 degrees, got {observed}"
    );
}

#[test]
fn normalize_angle_folds_into_the_signed_range() {
    assert!(
        close(Numeric::normalize_angle(3.0 * FRAC_PI_2), -FRAC_PI_2),
        "3pi/2 folds down to -pi/2"
    );
    assert!(
        close(Numeric::normalize_angle(-3.0 * FRAC_PI_2), FRAC_PI_2),
        "-3pi/2 folds up to pi/2"
    );
    for probe in [-10.0, -3.0, -1.0, 0.0, 1.0, 3.0, 10.0] {
        let folded: f64 = Numeric::normalize_angle(probe);
        assert!(
            folded > -PI - 1e-9 && folded <= PI + 1e-9,
            "{probe} normalized to {folded}, outside (-pi, pi]"
        );
    }
    assert!(
        close(Numeric::normalize_angle(FRAC_PI_2), FRAC_PI_2),
        "an angle already in range is untouched"
    );
}

#[test]
fn deg_to_rad_and_rad_to_deg_round_trip() {
    for degrees in [-720.0_f64, -90.0, 0.0, 45.0, 180.0, 360.0, 1080.0] {
        let round_tripped: f64 = Numeric::rad_to_deg(Numeric::deg_to_rad(degrees));
        assert!(
            (round_tripped - degrees).abs() < EPSILON,
            "{degrees} degrees must survive the round trip, got {round_tripped}"
        );
    }
}

#[test]
fn normalize_angle_leaves_an_angle_inside_the_range_untouched() {
    let observed: f64 = Numeric::normalize_angle(1.0);
    assert_eq!(observed, 1.0, "an in-range angle is its own normal form");
}

#[test]
fn normalize_angle_folds_a_full_turn_onto_zero() {
    let observed: f64 = Numeric::normalize_angle(TWO_PI);
    assert!(
        observed.abs() < EPSILON,
        "a full turn folds onto zero, got {observed}"
    );
}

#[test]
fn angle_delta_reports_the_shortest_rotation() {
    let delta: f64 = Numeric::angle_delta(0.0, FRAC_PI_2);
    assert!(
        close(delta, FRAC_PI_2),
        "a quarter turn is pi/2, got {delta}"
    );
    let wrapped: f64 = Numeric::angle_delta(0.0, 3.0 * FRAC_PI_2);
    assert!(
        (wrapped - (-FRAC_PI_2)).abs() < 1e-9,
        "3pi/2 forward is -pi/2 the short way, got {wrapped}"
    );
}

#[test]
fn normalize_angle_folds_a_lagging_angle_into_the_negative_half() {
    let observed: f64 = Numeric::normalize_angle(1.75 * PI);
    assert!(
        (observed + 0.25 * PI).abs() < EPSILON,
        "7pi/4 must fold onto -pi/4, got {observed}"
    );
}

#[test]
fn lerp_angle_takes_the_short_way_round() {
    let halfway: f64 = Numeric::lerp_angle(0.0, 3.0 * FRAC_PI_2, 0.5);
    assert!(
        close(halfway, -FRAC_PI_2 / 2.0),
        "the short way from 0 to 3pi/2 runs backwards through -pi/2, so halfway is -pi/4, got {halfway}"
    );
    assert!(
        close(Numeric::lerp_angle(0.0, FRAC_PI_2, 0.5), FRAC_PI_2 / 2.0),
        "a forward quarter turn halfway is pi/8"
    );
}

#[test]
fn normalize_angle_folds_a_leading_angle_up_into_the_positive_half() {
    let observed: f64 = Numeric::normalize_angle(-1.75 * PI);
    assert!(
        (observed - 0.25 * PI).abs() < EPSILON,
        "-7pi/4 is -315 degrees, which normalises to +pi/4, got {observed}"
    );
}

#[test]
fn distance_matches_the_pythagorean_form() {
    let a: Vector2D = v2(0.0, 0.0);
    let b: Vector2D = v2(3.0, 4.0);
    assert!(
        close(Numeric::distance(a, b), 5.0),
        "a 3-4-5 triangle is 5 long"
    );
    assert!(
        close(Numeric::distance_squared(a, b), 25.0),
        "the squared form avoids the square root"
    );
    assert!(
        close(
            Numeric::distance(a, b).powi(2),
            Numeric::distance_squared(a, b)
        ),
        "the two forms must agree"
    );
}

#[test]
fn normalize_angle_always_lands_inside_plus_minus_pi() {
    for radians in [-100.0_f64, -7.0, -1.0, 0.0, 1.0, 7.0, 100.0] {
        let observed: f64 = Numeric::normalize_angle(radians);
        assert!(
            (-PI - EPSILON..=PI + EPSILON).contains(&observed),
            "normalize_angle({radians}) = {observed} escaped the -pi..pi window"
        );
    }
}

#[test]
fn angle_delta_measures_a_forward_turn() {
    let observed: f64 = Numeric::angle_delta(0.0, HALF_PI);
    assert!(
        (observed - HALF_PI).abs() < EPSILON,
        "a quarter turn forward is half pi, got {observed}"
    );
}

#[test]
fn smoothstep_is_flat_at_both_ends_and_halfway_in_the_middle() {
    assert!(
        close(Numeric::smoothstep(0.0, 1.0, 0.0), 0.0),
        "at the low edge it is zero"
    );
    assert!(
        close(Numeric::smoothstep(0.0, 1.0, 1.0), 1.0),
        "at the high edge it is one"
    );
    assert!(
        close(Numeric::smoothstep(0.0, 1.0, 0.5), 0.5),
        "the midpoint of a symmetric smoothstep is one half"
    );
}

#[test]
fn angle_delta_measures_a_backward_turn_as_negative() {
    let observed: f64 = Numeric::angle_delta(HALF_PI, 0.0);
    assert!(
        (observed + HALF_PI).abs() < EPSILON,
        "a quarter turn back is minus half pi, got {observed}"
    );
}

#[test]
fn smoothstep_clamps_outside_its_edges() {
    assert!(
        close(Numeric::smoothstep(0.0, 1.0, -5.0), 0.0),
        "below the low edge it is zero"
    );
    assert!(
        close(Numeric::smoothstep(0.0, 1.0, 9.0), 1.0),
        "above the high edge it is one"
    );
}

#[test]
fn angle_delta_takes_the_short_path_across_the_seam() {
    let observed: f64 = Numeric::angle_delta(0.0, 1.5 * PI);
    assert!(
        (observed + HALF_PI).abs() < EPSILON,
        "reaching 3pi/2 from zero is a backwards quarter turn, got {observed}"
    );
}

#[test]
fn approach_moves_by_at_most_max_delta() {
    assert!(
        close(Numeric::approach(0.0, 10.0, 3.0), 3.0),
        "approach takes one capped step"
    );
    assert!(
        close(Numeric::approach(0.0, 1.0, 3.0), 1.0),
        "a target inside the cap is reached exactly, not overshot"
    );
    assert!(
        close(Numeric::approach(5.0, 5.0, 1.0), 5.0),
        "already at the target it does not move"
    );
}

#[test]
fn angle_delta_is_zero_for_the_same_angle() {
    let observed: f64 = Numeric::angle_delta(1.25, 1.25);
    assert_eq!(observed, 0.0, "the delta to itself is zero");
}

#[test]
fn lerp_angle_takes_the_short_way_round_the_circle() {
    let observed: f64 = Numeric::lerp_angle(0.0, 1.5 * PI, 0.5);
    assert!(
        (observed + 0.25 * PI).abs() < EPSILON,
        "the short way from zero to 3pi/2 is a quarter turn back, so halfway is -pi/4, got {observed}"
    );
}

#[test]
fn sign_reports_direction_and_sign_or_positive_never_goes_negative() {
    assert!(
        close(Numeric::sign(7.0), 1.0),
        "a positive value is plus one"
    );
    assert!(
        close(Numeric::sign(-7.0), -1.0),
        "a negative value is minus one"
    );
    assert!(close(Numeric::sign(0.0), 0.0), "zero is its own sign");
    assert!(
        close(Numeric::sign_or_positive(-7.0), -1.0),
        "the documented contract is 1.0 for non-negative and -1.0 otherwise"
    );
    assert!(
        close(Numeric::sign_or_positive(0.0), 1.0),
        "zero counts as non-negative, so it maps to 1.0"
    );
    assert!(
        close(Numeric::sign_or_positive(7.0), 1.0),
        "a positive value is unchanged"
    );
}

#[test]
fn lerp_angle_returns_its_endpoints_at_zero_and_one() {
    let start: f64 = Numeric::lerp_angle(0.5, 2.0, 0.0);
    let end: f64 = Numeric::lerp_angle(0.5, 2.0, 1.0);
    assert_eq!(start, 0.5, "factor zero returns the source angle");
    assert!(
        (end - 2.0).abs() < EPSILON,
        "factor one returns the target angle, got {end}"
    );
}

#[test]
fn wrap_folds_a_value_into_the_half_open_range() {
    let folded: f64 = Numeric::wrap(7.0, 5.0);
    assert!(
        (0.0..5.0).contains(&folded),
        "wrap must land in [0, max), got {folded}"
    );
    assert!(close(folded, 2.0), "7 wraps to 2 over a period of 5");
    assert!(
        close(Numeric::wrap(-1.0, 5.0), 4.0),
        "a negative value wraps up"
    );
}

#[test]
fn three_dimensional_distance_matches_the_squared_form() {
    let a: Vector3D = v3(0.0, 0.0, 0.0);
    let b: Vector3D = v3(2.0, 3.0, 6.0);
    assert!(
        close(Numeric::distance_3d(a, b), 7.0),
        "a 2-3-6 box is 7 across"
    );
    assert!(
        close(Numeric::distance_squared_3d(a, b), 49.0),
        "the squared form matches"
    );
}

#[test]
fn vector2d_basis_helpers_return_the_unit_axes() {
    assert_eq!(Vector2D::zero(), v2(0.0, 0.0), "zero is the origin");
    assert_eq!(Vector2D::right(), v2(1.0, 0.0), "right is +x");
    assert_eq!(
        Vector2D::up(),
        v2(0.0, -1.0),
        "Vector2D is screen space, so up is -y"
    );
}

#[test]
fn from_angle_produces_a_unit_vector_at_that_bearing() {
    let v: Vector2D = Vector2D::from_angle(0.0);
    assert!(
        close(v.get_x(), 1.0) && close(v.get_y(), 0.0),
        "angle zero is +x"
    );
    let quarter: Vector2D = Vector2D::from_angle(FRAC_PI_2);
    assert!(
        close(quarter.get_x(), 0.0) && close(quarter.get_y(), 1.0),
        "a quarter turn is +y"
    );
    assert!(
        close(Vector2D::from_angle(1.0).magnitude(), 1.0),
        "the result is always unit length"
    );
}

#[test]
fn magnitude_and_magnitude_squared_agree() {
    let v: Vector2D = v2(3.0, 4.0);
    assert!(close(v.magnitude(), 5.0), "a 3-4 vector is 5 long");
    assert!(close(v.magnitude_squared(), 25.0), "the squared form is 25");
    assert!(
        close(
            v.magnitude_squared(),
            v.get_x() * v.get_x() + v.get_y() * v.get_y()
        ),
        "magnitude_squared is the sum of the component squares"
    );
}

#[test]
fn normalized_produces_a_unit_vector_without_touching_the_original() {
    let v: Vector2D = v2(3.0, 4.0);
    let unit: Vector2D = v.normalized();
    assert!(close(unit.magnitude(), 1.0), "the result is unit length");
    assert_eq!(v, v2(3.0, 4.0), "normalized must not consume the receiver");
}

#[test]
fn normalize_mutates_in_place_and_leaves_a_zero_vector_alone() {
    let mut v: Vector2D = v2(3.0, 4.0);
    v.normalize();
    assert!(close(v.magnitude(), 1.0), "the receiver is now unit length");
    let mut zero: Vector2D = Vector2D::zero();
    zero.normalize();
    assert_eq!(
        zero,
        Vector2D::zero(),
        "a zero vector has no direction to keep"
    );
}

#[test]
fn dot_and_cross_use_the_two_dimensional_conventions() {
    let a: Vector2D = v2(1.0, 0.0);
    let b: Vector2D = v2(0.0, 1.0);
    assert!(
        close(a.dot(b), 0.0),
        "perpendicular vectors have no dot product"
    );
    assert!(
        close(a.dot(v2(2.0, 0.0)), 2.0),
        "a parallel dot scales by length"
    );
    assert!(
        close(a.cross(b), 1.0),
        "cross of +x into +y is positive one, the 2D scalar z component"
    );
    assert!(close(b.cross(a), -1.0), "cross is antisymmetric");
}

#[test]
fn perp_rotates_ninety_degrees_without_changing_length() {
    let v: Vector2D = v2(1.0, 0.0);
    let rotated: Vector2D = v.perp();
    assert!(
        close(rotated.get_x(), 0.0) && close(rotated.get_y(), 1.0),
        "perp turns +x into +y"
    );
    assert!(
        close(rotated.magnitude(), v.magnitude()),
        "perp preserves length"
    );
}

#[test]
fn angle_reports_the_bearing_of_a_vector() {
    assert!(close(Vector2D::right().angle(), 0.0), "+x is bearing zero");
    assert!(
        close(Vector2D::up().angle(), -FRAC_PI_2),
        "in screen space up is -y, a negative quarter turn"
    );
    assert!(
        close(Vector2D::from_angle(FRAC_PI_2).angle(), FRAC_PI_2),
        "from_angle follows the trig convention, independent of up()"
    );
}

#[test]
fn angle_to_measures_the_rotation_from_one_vector_to_another() {
    let a: Vector2D = v2(1.0, 0.0);
    let b: Vector2D = v2(0.0, 1.0);
    let delta: Vector2D = b - a;
    assert!(
        close(a.angle_to(b), delta.angle()),
        "angle_to reports the bearing of the difference vector"
    );
    assert!(
        close(a.angle_to(a), 0.0),
        "the angle to a point from itself has no difference to point along"
    );
}

#[test]
fn rotated_returns_a_new_vector_and_leaves_the_original_alone() {
    let v: Vector2D = Vector2D::right();
    let turned: Vector2D = v.rotated(FRAC_PI_2);
    assert!(close(turned.get_y(), 1.0), "a quarter turn lands on +y");
    assert_eq!(
        v,
        Vector2D::right(),
        "rotated must not consume the receiver"
    );
}

#[test]
fn rotate_mutates_in_place() {
    let mut v: Vector2D = Vector2D::right();
    v.rotate(FRAC_PI_2);
    assert!(close(v.get_y(), 1.0), "the receiver itself turned");
}

#[test]
fn distance_helpers_on_the_vector_agree_with_the_free_functions() {
    let a: Vector2D = v2(0.0, 0.0);
    let b: Vector2D = v2(3.0, 4.0);
    assert!(
        close(a.distance_to(b), 5.0),
        "distance_to is the same length"
    );
    assert!(
        close(a.distance_squared_to(b), 25.0),
        "so is the squared form"
    );
    let unit: Vector2D = a.direction_to(b);
    assert!(
        close(unit.magnitude(), 1.0),
        "direction_to yields a unit vector"
    );
    assert!(
        close(unit.get_x(), 0.6) && close(unit.get_y(), 0.8),
        "pointing at the 3-4 target"
    );
}

#[test]
fn vector_lerp_and_scaled_do_not_consume_the_receiver() {
    let a: Vector2D = v2(0.0, 0.0);
    let b: Vector2D = v2(4.0, 8.0);
    let mid: Vector2D = a.lerp(b, 0.5);
    assert!(
        close(mid.get_x(), 2.0) && close(mid.get_y(), 4.0),
        "halfway to the target"
    );
    let doubled: Vector2D = a.scaled(3.0);
    assert_eq!(doubled, v2(0.0, 0.0), "scaling zero is still zero");
    assert!(
        close(b.scaled(2.0).get_x(), 8.0),
        "scaled returns a new vector"
    );
    assert_eq!(b, v2(4.0, 8.0), "scaled must not consume the receiver");
}

#[test]
fn scale_mutates_in_place() {
    let mut v: Vector2D = v2(2.0, 3.0);
    v.scale(2.0);
    assert_eq!(v, v2(4.0, 6.0), "scale applied in place");
}

#[test]
fn vector3d_basis_helpers_and_from_the_plane() {
    assert_eq!(Vector3D::zero(), v3(0.0, 0.0, 0.0), "zero is the origin");
    assert_eq!(Vector3D::right(), v3(1.0, 0.0, 0.0), "right is +x");
    assert_eq!(Vector3D::up(), v3(0.0, 1.0, 0.0), "up is +y");
    assert_eq!(
        Vector3D::forward(),
        v3(0.0, 0.0, -1.0),
        "a right-handed frame puts forward at -z"
    );
}

#[test]
fn vector3d_dot_and_cross_follow_the_right_hand_rule() {
    let x: Vector3D = Vector3D::right();
    let y: Vector3D = Vector3D::up();
    let z: Vector3D = Vector3D::forward();
    assert!(close(x.dot(y), 0.0), "the basis axes are orthogonal");
    let crossed: Vector3D = x.cross(y);
    assert_eq!(crossed, v3(0.0, 0.0, 1.0), "x cross y is +z");
    assert_eq!(z, v3(0.0, 0.0, -1.0), "and forward() is its opposite, -z");
    assert_eq!(y.cross(x), v3(0.0, 0.0, -1.0), "y cross x is -z");
}

#[test]
fn vector3d_magnitude_normalization_and_lerp() {
    let v: Vector3D = v3(2.0, 3.0, 6.0);
    assert!(close(v.magnitude(), 7.0), "a 2-3-6 vector is 7 long");
    assert!(
        close(v.magnitude_squared(), 49.0),
        "the squared form matches"
    );
    assert!(
        close(v.normalized().magnitude(), 1.0),
        "normalized is unit length"
    );
    let mid: Vector3D = Vector3D::zero().lerp(v, 0.5);
    assert!(
        close(mid.get_x(), 1.0) && close(mid.get_z(), 3.0),
        "lerp walks the segment"
    );
    assert!(
        close(v.scaled(2.0).magnitude(), 14.0),
        "scaled doubles the length"
    );
    assert_eq!(v, v3(2.0, 3.0, 6.0), "scaled must not consume the receiver");
}

#[test]
fn vector3d_normalize_leaves_a_zero_vector_at_zero() {
    let mut v: Vector3D = v3(0.0, 0.0, 0.0);
    v.normalize();
    assert_eq!(
        v,
        Vector3D::zero(),
        "a zero vector has no direction to keep"
    );
}

#[test]
fn vector3d_distance_helpers_match_the_free_functions() {
    let a: Vector3D = Vector3D::zero();
    let b: Vector3D = v3(2.0, 3.0, 6.0);
    assert!(close(a.distance_to(b), 7.0), "distance_to agrees");
    assert!(
        close(a.distance_squared_to(b), 49.0),
        "so does the squared form"
    );
    assert!(
        close(a.direction_to(b).magnitude(), 1.0),
        "direction_to is unit length"
    );
}

#[test]
fn quaternion_identity_is_neutral_under_rotation() {
    let identity: Quaternion = Quaternion::identity();
    assert!(
        close(identity.magnitude(), 1.0),
        "the identity quaternion is unit length"
    );
    let v: Vector3D = v3(1.0, 2.0, 3.0);
    let turned: Vector3D = v.rotated_by(identity);
    assert!(
        close(turned.magnitude(), v.magnitude()),
        "rotating by identity preserves length"
    );
    assert!(
        (turned.get_x() - v.get_x()).abs() < 1e-9,
        "rotating by identity is the identity transform"
    );
}

#[test]
fn quaternion_from_axis_angle_keeps_unit_length_and_normalizes() {
    let q: Quaternion = Quaternion::from_axis_angle(Vector3D::up(), FRAC_PI_2);
    assert!(
        close(q.magnitude(), 1.0),
        "a rotation quaternion is unit length"
    );
    assert!(
        close(q.normalized().magnitude(), 1.0),
        "normalized stays unit length"
    );
    assert!(
        close(q.conjugate().magnitude(), q.magnitude()),
        "conjugate preserves length"
    );
}

#[test]
fn quaternion_from_euler_is_a_valid_rotation() {
    let q: Quaternion = Quaternion::from_euler(0.3, 0.2, 0.1);
    assert!(
        (q.magnitude() - 1.0).abs() < 1e-6,
        "an euler-built rotation is unit length, got {}",
        q.magnitude()
    );
}

#[test]
fn quaternion_dot_and_slerp_stay_on_the_unit_sphere() {
    let a: Quaternion = Quaternion::identity();
    let b: Quaternion = Quaternion::from_axis_angle(Vector3D::up(), 0.5);
    assert!(
        close(a.dot(b), b.get_w()),
        "orthogonal part does not contribute to the dot"
    );
    let mid: Quaternion = a.slerp(b, 0.5);
    assert!(
        (mid.magnitude() - 1.0).abs() < 1e-6,
        "slerp must stay on the unit sphere, got {}",
        mid.magnitude()
    );
    let start: Quaternion = a.slerp(b, 0.0);
    let end: Quaternion = a.slerp(b, 1.0);
    assert!(
        close(start.magnitude(), 1.0) && close(end.magnitude(), 1.0),
        "endpoints are unit"
    );
}

#[test]
fn matrix_identity_leaves_a_point_untouched() {
    let m: Matrix4x4 = Matrix4x4::identity();
    let p: Vector3D = v3(1.0, 2.0, 3.0);
    let out: Vector3D = m.transform_point(p);
    assert!(
        (out.get_x() - 1.0).abs() < 1e-9
            && (out.get_y() - 2.0).abs() < 1e-9
            && (out.get_z() - 3.0).abs() < 1e-9,
        "the identity matrix must be a no-op, got {out:?}"
    );
}

#[test]
fn matrix_translation_moves_a_point_by_the_offset() {
    let m: Matrix4x4 = Matrix4x4::translation(v3(10.0, 0.0, 0.0));
    let out: Vector3D = m.transform_point(Vector3D::zero());
    assert!(
        (out.get_x() - 10.0).abs() < 1e-9 && out.get_y().abs() < 1e-9,
        "the point moved by the translation, got {out:?}"
    );
}

#[test]
fn matrix_scaling_multiplies_each_axis() {
    let m: Matrix4x4 = Matrix4x4::scaling(v3(2.0, 3.0, 4.0));
    let out: Vector3D = m.transform_point(v3(1.0, 1.0, 1.0));
    assert!(
        (out.get_x() - 2.0).abs() < 1e-9
            && (out.get_y() - 3.0).abs() < 1e-9
            && (out.get_z() - 4.0).abs() < 1e-9,
        "each axis scaled by its own factor, got {out:?}"
    );
}

#[test]
fn matrix_multiply_composes_left_to_right() {
    let translate: Matrix4x4 = Matrix4x4::translation(v3(5.0, 0.0, 0.0));
    let scale: Matrix4x4 = Matrix4x4::scaling(v3(2.0, 2.0, 2.0));
    let composed: Matrix4x4 = translate.multiply(scale);
    let out: Vector3D = composed.transform_point(v3(1.0, 0.0, 0.0));
    assert!(
        (out.get_x() - 7.0).abs() < 1e-9,
        "the right operand is applied first, so 1*2 + 5 = 7, got {out:?}"
    );
    let reversed: Matrix4x4 = scale.multiply(translate);
    let other: Vector3D = reversed.transform_point(v3(1.0, 0.0, 0.0));
    assert!(
        (other.get_x() - 12.0).abs() < 1e-9,
        "reversing the operands reverses the order, (1+5)*2 = 12, got {other:?}"
    );
}

#[test]
fn matrix_rotation_preserves_length() {
    let q: Quaternion = Quaternion::from_axis_angle(Vector3D::up(), 0.7);
    let m: Matrix4x4 = Matrix4x4::rotation(q);
    let p: Vector3D = v3(1.0, 2.0, 3.0);
    let out: Vector3D = m.transform_point(p);
    assert!(
        (out.magnitude() - p.magnitude()).abs() < 1e-6,
        "a rotation must not change length, {} vs {}",
        out.magnitude(),
        p.magnitude()
    );
}

#[test]
fn perspective_and_orthographic_projections_are_constructible() {
    let perspective: Matrix4x4 = Matrix4x4::perspective(FRAC_PI_2, 16.0 / 9.0, 0.1, 100.0);
    let near: Vector3D = perspective.transform_point(v3(0.0, 0.0, -0.1));
    let far: Vector3D = perspective.transform_point(v3(0.0, 0.0, -100.0));
    assert!(
        near.get_z() <= far.get_z(),
        "a point on the far plane must project deeper than one on the near plane"
    );
    let ortho: Matrix4x4 = Matrix4x4::orthographic(-1.0, 1.0, -1.0, 1.0, 0.1, 100.0);
    let inside: Vector3D = ortho.transform_point(v3(0.0, 0.0, -1.0));
    assert!(
        inside.get_z() > -1.0,
        "the origin stays inside the ortho box"
    );
}

#[test]
fn look_at_places_the_eye_at_the_origin() {
    let eye: Vector3D = v3(0.0, 0.0, 10.0);
    let m: Matrix4x4 = Matrix4x4::look_at(eye, Vector3D::zero(), Vector3D::up());
    let view_eye: Vector3D = m.transform_point(eye);
    assert!(
        view_eye.magnitude() < 1e-6,
        "the view matrix must map the eye to the origin, got {view_eye:?}"
    );
}

#[test]
fn transform2d_identity_translate_and_rotate_compose() {
    let mut t: Transform2D = Transform2D::identity();
    assert_eq!(
        t.apply_to_point(v2(3.0, 4.0)),
        v2(3.0, 4.0),
        "identity is a no-op"
    );
    t.translate(v2(10.0, 0.0));
    let moved: Vector2D = t.apply_to_point(v2(0.0, 0.0));
    assert!(close(moved.get_x(), 10.0), "the translate was applied");
    t.rotate(FRAC_PI_2);
    let turned: Vector2D = t.apply_to_point(v2(0.0, 0.0));
    assert!(
        close(turned.get_x(), 10.0) && close(turned.get_y(), 0.0),
        "Transform2D is decomposed translate-rotate-scale, so the position offset is applied last and does not turn with the rotation, got {turned:?}"
    );
}

#[test]
fn transform2d_scale_by_factors_each_axis() {
    let mut t: Transform2D = Transform2D::identity();
    t.scale_by(v2(2.0, 5.0));
    let out: Vector2D = t.apply_to_point(v2(1.0, 1.0));
    assert!(
        close(out.get_x(), 2.0) && close(out.get_y(), 5.0),
        "per-axis scaling, got {out:?}"
    );
}

#[test]
fn transform3d_round_trips_through_a_matrix() {
    let mut t: Transform3D = Transform3D::identity();
    assert_eq!(
        t.apply_to_point(v3(1.0, 2.0, 3.0)),
        v3(1.0, 2.0, 3.0),
        "identity is a no-op"
    );
    t.translate(v3(1.0, 1.0, 1.0));
    let moved: Vector3D = t.apply_to_point(Vector3D::zero());
    assert!(
        (moved.get_x() - 1.0).abs() < 1e-9,
        "the translate was applied, got {moved:?}"
    );
    let as_matrix: Matrix4x4 = t.to_matrix();
    let via_matrix: Vector3D = as_matrix.transform_point(Vector3D::zero());
    assert!(
        (via_matrix.get_x() - moved.get_x()).abs() < 1e-9,
        "to_matrix must agree with apply_to_point"
    );
}

#[test]
fn rect_from_center_reports_its_own_geometry_back() {
    let r: Rect = Rect::from_center(v2(5.0, 5.0), 4.0, 6.0);
    assert_eq!(r.center(), v2(5.0, 5.0), "the centre round-trips");
    assert_eq!(r.size(), v2(4.0, 6.0), "the size round-trips");
    assert_eq!(r.min(), v2(3.0, 2.0), "min is centre minus half size");
    assert_eq!(r.max(), v2(7.0, 8.0), "max is centre plus half size");
}

#[test]
fn rect_contains_and_intersects() {
    let r: Rect = Rect::from_center(v2(0.0, 0.0), 10.0, 10.0);
    assert!(r.contains(v2(0.0, 0.0)), "the centre is inside");
    assert!(!r.contains(v2(100.0, 0.0)), "a far point is outside");
    let other: Rect = Rect::from_center(v2(5.0, 0.0), 10.0, 10.0);
    assert!(r.intersects(other), "overlapping rects intersect");
    assert!(
        Rect::broad_phase_alias(r, other),
        "the free-function form agrees with the method"
    );
    let apart: Rect = Rect::from_center(v2(500.0, 0.0), 10.0, 10.0);
    assert!(!r.intersects(apart), "distant rects do not");
}

#[test]
fn rect_intersection_returns_the_overlap_region() {
    let a: Rect = Rect::from_center(v2(0.0, 0.0), 10.0, 10.0);
    let b: Rect = Rect::from_center(v2(5.0, 0.0), 10.0, 10.0);
    let overlap: Option<Rect> = a.intersection(b);
    let rect: Rect = overlap.expect("overlapping rects must yield an overlap");
    assert_eq!(
        rect.size(),
        v2(5.0, 10.0),
        "the overlap is the smaller strip"
    );
    let apart: Rect = Rect::from_center(v2(500.0, 0.0), 10.0, 10.0);
    assert!(
        a.intersection(apart).is_none(),
        "disjoint rects have no overlap"
    );
}

#[test]
fn circle_geometry() {
    let c: Circle = Circle::new(v2(0.0, 0.0), 2.0);
    assert!(close(c.area(), PI * 4.0), "area is pi r squared");
    assert!(
        close(c.circumference(), TAU * 2.0),
        "circumference is 2 pi r"
    );
    assert!(
        c.contains(v2(1.0, 0.0)),
        "a point inside the radius is contained"
    );
    assert!(
        !c.contains(v2(3.0, 0.0)),
        "a point outside the radius is not"
    );
    assert!(
        c.intersects(Circle::new(v2(3.0, 0.0), 2.0)),
        "touching circles intersect"
    );
    assert!(
        !c.intersects(Circle::new(v2(10.0, 0.0), 2.0)),
        "distant circles do not"
    );
}

#[test]
fn color_construction_and_css_output() {
    let c: Color = Color::from_rgb(255, 0, 128);
    assert!(close(c.get_red(), 1.0), "channels are normalized into 0..1");
    let css: String = c.to_css_rgba();
    assert!(
        css.starts_with("rgba("),
        "the CSS form is rgba(), got {css}"
    );
    let mut buffer: String = String::new();
    c.write_css_rgba(&mut buffer);
    assert_eq!(buffer, css, "write_css_rgba and to_css_rgba must agree");
}

#[test]
fn color_named_constructors() {
    assert!(close(Color::black().get_red(), 0.0), "black has no red");
    assert_eq!(Color::black().get_alpha(), 1.0, "black is opaque");
    assert!(
        close(Color::white().get_red(), 1.0),
        "white pins the red channel to one"
    );
    assert_eq!(
        Color::transparent().get_alpha(),
        0.0,
        "transparent has zero alpha"
    );
}

#[test]
fn color_lerp_mixes_towards_the_target() {
    let black: Color = Color::black();
    let white: Color = Color::white();
    let mid: Color = black.lerp(white, 0.5);
    assert!(
        close(mid.get_red(), 0.5),
        "halfway between black and white is 0.5"
    );
    assert!(
        close(black.lerp(white, 0.0).get_red(), 0.0),
        "factor zero is the receiver"
    );
    assert!(
        close(black.lerp(white, 1.0).get_red(), 1.0),
        "factor one is the target"
    );
}

#[test]
fn aabb3d_geometry() {
    let box3: AABB3D = AABB3D::from_center(v3(0.0, 0.0, 0.0), 2.0, 4.0, 6.0);
    assert_eq!(box3.center(), v3(0.0, 0.0, 0.0), "the centre round-trips");
    assert_eq!(box3.size(), v3(2.0, 4.0, 6.0), "the size round-trips");
    assert!(
        close(
            box3.size().get_x() * box3.size().get_y() * box3.size().get_z(),
            48.0
        ),
        "the side lengths multiply out to 48"
    );
    assert!(
        box3.contains(v3(0.5, 0.5, 0.5)),
        "an interior point is contained"
    );
    assert!(
        !box3.contains(v3(5.0, 0.0, 0.0)),
        "an exterior point is not"
    );
    assert!(
        box3.intersects(AABB3D::from_center(v3(1.0, 0.0, 0.0), 2.0, 2.0, 2.0)),
        "overlap"
    );
    assert!(
        !box3.intersects(AABB3D::from_center(v3(50.0, 0.0, 0.0), 2.0, 2.0, 2.0)),
        "disjoint"
    );
}

#[test]
fn sphere_geometry() {
    let s: Sphere = Sphere::new(v3(0.0, 0.0, 0.0), 3.0);
    assert!(close(s.volume(), 4.0 / 3.0 * PI * 27.0), "sphere volume");
    assert!(
        close(s.surface_area(), 4.0 * PI * 9.0),
        "sphere surface area"
    );
    assert!(s.contains(v3(1.0, 0.0, 0.0)), "a point inside is contained");
    assert!(!s.contains(v3(9.0, 0.0, 0.0)), "a point outside is not");
    assert!(
        s.intersects(Sphere::new(v3(5.0, 0.0, 0.0), 3.0)),
        "overlapping spheres intersect"
    );
    assert!(
        !s.intersects(Sphere::new(v3(50.0, 0.0, 0.0), 3.0)),
        "distant ones do not"
    );
}

#[test]
fn plane_reports_signed_distance_to_a_point() {
    let plane: Plane = Plane::from_normal_and_point(Vector3D::up(), Vector3D::zero());
    let above: f64 = plane.distance_to_point(v3(0.0, 5.0, 0.0));
    assert!(
        close(above, 5.0),
        "the distance along the normal, got {above}"
    );
    assert!(
        close(plane.distance_to_point(Vector3D::zero()), 0.0),
        "the plane contains its point"
    );
}

#[test]
fn ray_intersections_report_the_parameter_along_the_ray() {
    let ray: Ray3D = Ray3D::new(Vector3D::zero(), v3(0.0, 0.0, 1.0));
    let hit: f64 = ray
        .intersect_sphere(Sphere::new(v3(0.0, 0.0, 10.0), 1.0))
        .expect("a ray pointing at a sphere must hit it");
    assert!(
        hit > 0.0 && hit < 11.0,
        "the hit lies in front of the origin, got {hit}"
    );
    let on_plane: Option<f64> = ray.intersect_plane(Plane::from_normal_and_point(
        v3(0.0, 0.0, 1.0),
        v3(0.0, 0.0, 5.0),
    ));
    assert!(
        on_plane.is_some(),
        "a ray perpendicular to a plane must hit it"
    );
    let behind: Option<f64> = ray.intersect_sphere(Sphere::new(v3(0.0, 0.0, -10.0), 1.0));
    assert!(behind.is_none(), "a sphere behind the origin is not hit");
}

#[test]
fn ray_point_at_walks_along_the_direction() {
    let ray: Ray3D = Ray3D::new(v3(1.0, 0.0, 0.0), v3(0.0, 0.0, 1.0));
    let p: Vector3D = ray.point_at(3.0);
    assert!(
        close(p.get_z(), 3.0) && close(p.get_x(), 1.0),
        "point_at walks from the origin along the direction, got {p:?}"
    );
    let backwards: Ray3D = Ray3D::new(Vector3D::zero(), Vector3D::forward());
    assert!(
        close(backwards.point_at(3.0).get_z(), -3.0),
        "forward() is -z, so three units along it is -3"
    );
}

#[test]
fn ray_intersect_aabb_reports_a_forward_hit() {
    let ray: Ray3D = Ray3D::new(v3(-10.0, 0.0, 0.0), Vector3D::right());
    let target: AABB3D = AABB3D::from_center(Vector3D::zero(), 2.0, 2.0, 2.0);
    assert!(
        ray.intersect_aabb(target).is_some(),
        "a ray aimed at the box must hit it"
    );
    let away: Ray3D = Ray3D::new(v3(-10.0, 0.0, 0.0), v3(-1.0, 0.0, 0.0));
    assert!(
        away.intersect_aabb(target).is_none(),
        "a ray pointing away must miss"
    );
}

fn epsilon(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 1e-9
}

#[test]
fn the_vector_trait_is_implemented_for_both_vector_types() {
    assert_eq!(
        <Vector2D as Vector>::zero(),
        Vector2D::new(0.0, 0.0),
        "2D zero"
    );
    assert_eq!(
        <Vector3D as Vector>::zero(),
        Vector3D::new(0.0, 0.0, 0.0),
        "3D zero"
    );
    let two: Vector2D = Vector2D::new(3.0, 4.0);
    let three: Vector3D = Vector3D::new(2.0, 3.0, 6.0);
    assert!(
        epsilon(two.magnitude(), 5.0),
        "2D magnitude through the trait"
    );
    assert!(
        epsilon(three.magnitude(), 7.0),
        "3D magnitude through the trait"
    );
    assert!(
        epsilon(<Vector2D as Vector>::magnitude_squared(&two), 25.0),
        "the squared form comes from the trait too"
    );
    assert!(
        epsilon(two.normalized().magnitude(), 1.0),
        "normalized is unit length"
    );
    assert_eq!(
        two.scaled(2.0),
        Vector2D::new(6.0, 8.0),
        "scaled through the trait"
    );
}

fn generic_dot<V: Vector>(a: V, b: V) -> f64 {
    a.dot(b)
}

#[test]
fn the_vector_trait_is_what_lets_a_dot_call_be_generic() {
    let a: Vector2D = Vector2D::new(1.0, 2.0);
    let b: Vector2D = Vector2D::new(3.0, 4.0);
    assert!(
        epsilon(generic_dot(a, b), 11.0),
        "1*3 + 2*4 = 11, reached through the trait bound rather than a concrete type"
    );
    let c: Vector3D = Vector3D::new(1.0, 2.0, 3.0);
    let d: Vector3D = Vector3D::new(4.0, 5.0, 6.0);
    assert!(
        epsilon(generic_dot(c, d), 32.0),
        "and the same generic call works in 3D: 4 + 10 + 18"
    );
}

#[test]
fn the_vector_trait_lerp_agrees_with_the_inherent_method() {
    let a: Vector2D = Vector2D::new(0.0, 0.0);
    let b: Vector2D = Vector2D::new(8.0, 4.0);
    let half: Vector2D = a.lerp(b, 0.5);
    assert_eq!(half, Vector2D::new(4.0, 2.0), "halfway through the trait");
    assert_eq!(
        half,
        a.lerp(b, 0.5),
        "and it agrees with the inherent method"
    );
}

#[test]
fn the_interpolable_trait_is_implemented_for_scalars_and_colors() {
    let a: Color = Color::black();
    let b: Color = Color::white();
    let mid: Color = a.lerp(b, 0.5);
    assert!(
        epsilon(mid.get_red(), 0.5),
        "a colour interpolates the same way a scalar would"
    );
    assert_eq!(a.lerp(b, 0.0), a, "factor zero returns the receiver");
    assert_eq!(a.lerp(b, 1.0), b, "factor one returns the target");
}

#[test]
fn a_ray2d_carries_its_origin_and_direction() {
    let ray: Ray2D = Ray2D::new(Vector2D::new(1.0, 2.0), Vector2D::new(0.0, 1.0));
    assert!(
        epsilon(ray.get_origin().get_x(), 1.0) && epsilon(ray.get_origin().get_y(), 2.0),
        "the origin round-trips"
    );
    assert_eq!(
        ray.get_direction(),
        Vector2D::new(0.0, 1.0),
        "the direction round-trips"
    );
}

#[test]
fn distance_measures_a_three_four_five_triangle() {
    let a: Vector2D = Vector2D::new(0.0, 0.0);
    let b: Vector2D = Vector2D::new(3.0, 4.0);
    let observed: f64 = Numeric::distance(a, b);
    assert!(
        (observed - 5.0).abs() < EPSILON,
        "the hypotenuse of a 3-4-5 triangle is 5, got {observed}"
    );
}

#[test]
fn distance_is_zero_for_the_same_point() {
    let a: Vector2D = Vector2D::new(-2.0, 7.5);
    let observed: f64 = Numeric::distance(a, a);
    assert_eq!(observed, 0.0, "a point is zero away from itself");
}

#[test]
fn distance_is_symmetric_in_its_arguments() {
    let a: Vector2D = Vector2D::new(1.0, 2.0);
    let b: Vector2D = Vector2D::new(-4.0, 9.0);
    let forward: f64 = Numeric::distance(a, b);
    let backward: f64 = Numeric::distance(b, a);
    assert_eq!(forward, backward, "distance must not depend on the order");
}

#[test]
fn distance_squared_is_the_square_of_distance() {
    let a: Vector2D = Vector2D::new(1.0, 1.0);
    let b: Vector2D = Vector2D::new(4.0, 5.0);
    let squared: f64 = Numeric::distance_squared(a, b);
    let plain: f64 = Numeric::distance(a, b);
    assert_eq!(squared, 25.0, "a 3-4 offset squares to 25");
    assert!(
        (squared - plain * plain).abs() < EPSILON,
        "the squared form must be the square of the plain form"
    );
}

#[test]
fn smoothstep_pins_below_the_lower_edge() {
    let observed: f64 = Numeric::smoothstep(0.0, 1.0, -1.0);
    assert_eq!(observed, 0.0, "a value under the window stays at zero");
}

#[test]
fn smoothstep_pins_above_the_upper_edge() {
    let observed: f64 = Numeric::smoothstep(0.0, 1.0, 2.0);
    assert_eq!(observed, 1.0, "a value over the window stays at one");
}

#[test]
fn smoothstep_is_one_halfway_across_the_window() {
    let observed: f64 = Numeric::smoothstep(0.0, 1.0, 0.5);
    assert!(
        (observed - 0.5).abs() < EPSILON,
        "the smoothstep midpoint is 0.5, got {observed}"
    );
}

#[test]
fn smoothstep_respects_a_shifted_window() {
    let observed: f64 = Numeric::smoothstep(10.0, 20.0, 15.0);
    assert!(
        (observed - 0.5).abs() < EPSILON,
        "a shifted window is still centred on its own range, got {observed}"
    );
}

#[test]
fn approach_steps_by_at_most_the_maximum_delta() {
    let observed: f64 = Numeric::approach(0.0, 10.0, 3.0);
    assert_eq!(observed, 3.0, "the step is capped at max_delta");
}

#[test]
fn approach_snaps_onto_the_target_when_it_is_within_reach() {
    let observed: f64 = Numeric::approach(0.0, 2.0, 3.0);
    assert_eq!(observed, 2.0, "a reachable target is reached, not overshot");
}

#[test]
fn approach_steps_downwards_towards_a_lower_target() {
    let observed: f64 = Numeric::approach(10.0, 0.0, 3.0);
    assert_eq!(
        observed, 7.0,
        "the step is capped in the negative direction"
    );
}

#[test]
fn approach_on_a_zero_delta_swall_below_itself() {
    let observed: f64 = Numeric::approach(5.0, 5.0, 1.0);
    assert_eq!(observed, 5.0, "an already-met target returns unchanged");
}

#[test]
fn approach_treats_a_negative_max_delta_as_a_magnitude() {
    let observed: f64 = Numeric::approach(0.0, 10.0, -3.0);
    assert_eq!(observed, 3.0, "the step magnitude is positive either way");
}

#[test]
fn approach_never_crosses_the_target() {
    let mut value: f64 = 0.0;
    for _ in 0..10 {
        value = Numeric::approach(value, 5.0, 2.0);
    }
    assert_eq!(value, 5.0, "repeated steps must land exactly on the target");
}

#[test]
fn sign_reports_positive_negative_and_zero() {
    let positive: f64 = Numeric::sign(2.5);
    let negative: f64 = Numeric::sign(-2.5);
    let zero: f64 = Numeric::sign(0.0);
    assert_eq!(positive, 1.0, "a positive value signs as one");
    assert_eq!(negative, -1.0, "a negative value signs as minus one");
    assert_eq!(zero, 0.0, "zero signs as zero, not as one");
}

#[test]
fn wrap_folds_a_value_above_the_bound() {
    let observed: f64 = Numeric::wrap(7.0, 5.0);
    assert_eq!(observed, 2.0, "7 wraps into 0..5 as 2");
}

#[test]
fn wrap_folds_a_negative_value_up_into_range() {
    let observed: f64 = Numeric::wrap(-1.0, 5.0);
    assert_eq!(observed, 4.0, "-1 wraps into 0..5 as 4");
}

#[test]
fn wrap_folds_the_bound_itself_onto_zero() {
    let observed: f64 = Numeric::wrap(5.0, 5.0);
    assert_eq!(observed, 0.0, "the exclusive upper bound wraps to zero");
}

#[test]
fn wrap_always_lands_inside_the_range() {
    for value in [-13.0_f64, -5.0, -0.5, 0.0, 0.5, 5.0, 13.0] {
        let observed: f64 = Numeric::wrap(value, 5.0);
        assert!(
            (0.0..5.0).contains(&observed),
            "wrap({value}, 5.0) = {observed} escaped 0..5"
        );
    }
}

#[test]
fn sign_or_positive_maps_zero_onto_one() {
    let observed: f64 = Numeric::sign_or_positive(0.0);
    assert_eq!(observed, 1.0, "zero is treated as positive here");
}

#[test]
fn sign_or_positive_only_negative_values_map_to_minus_one() {
    let negative: f64 = Numeric::sign_or_positive(-0.001);
    let positive: f64 = Numeric::sign_or_positive(0.001);
    assert_eq!(negative, -1.0, "a negative value maps to minus one");
    assert_eq!(positive, 1.0, "a positive value maps to one");
}

#[test]
fn distance_3d_measures_a_space_diagonal() {
    let a: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let b: Vector3D = Vector3D::new(2.0, 3.0, 6.0);
    let observed: f64 = Numeric::distance_3d(a, b);
    assert!(
        (observed - 7.0).abs() < EPSILON,
        "the diagonal of a 2-3-6 box is 7, got {observed}"
    );
}

#[test]
fn distance_3d_is_zero_for_the_same_point() {
    let a: Vector3D = Vector3D::new(4.0, -1.0, 8.0);
    let observed: f64 = Numeric::distance_3d(a, a);
    assert_eq!(observed, 0.0, "a point is zero away from itself in 3D");
}

#[test]
fn distance_squared_3d_is_the_square_of_distance_3d() {
    let a: Vector3D = Vector3D::new(1.0, 2.0, 3.0);
    let b: Vector3D = Vector3D::new(-1.0, -2.0, -3.0);
    let squared: f64 = Numeric::distance_squared_3d(a, b);
    let plain: f64 = Numeric::distance_3d(a, b);
    assert_eq!(squared, 56.0, "a 2-4-6 offset squares to 56");
    assert!(
        (squared - plain * plain).abs() < EPSILON,
        "the squared form must be the square of the plain form"
    );
}

fn lerp_of<T: Interpolable>(from: T, to: T, factor: f64) -> T {
    from.lerp(to, factor)
}

fn magnitude_of<T: Vector>(value: T) -> f64 {
    value.magnitude()
}

fn dot_of<T: Vector>(left: T, right: T) -> f64 {
    left.dot(right)
}

#[test]
fn a_two_dimensional_vector_lerps_towards_its_target() {
    let observed: Vector2D = lerp_of(Vector2D::new(0.0, 0.0), Vector2D::new(10.0, 20.0), 0.5);
    assert!(
        (observed.get_x() - 5.0).abs() < EPSILON,
        "halfway in x, got {}",
        observed.get_x()
    );
    assert!(
        (observed.get_y() - 10.0).abs() < EPSILON,
        "halfway in y, got {}",
        observed.get_y()
    );
}

#[test]
fn a_lerp_at_the_ends_returns_the_endpoints_unchanged() {
    let a: Vector2D = Vector2D::new(1.0, 2.0);
    let b: Vector2D = Vector2D::new(9.0, 8.0);
    assert_eq!(lerp_of(a, b, 0.0), a, "factor 0 must return self exactly");
    assert_eq!(lerp_of(a, b, 1.0), b, "factor 1 must return other exactly");
}

#[test]
fn a_lerp_beyond_the_unit_range_keeps_extrapolating() {
    let a: Vector3D = Vector3D::zero();
    let b: Vector3D = Vector3D::new(2.0, 0.0, 0.0);
    let past: Vector3D = lerp_of(a, b, 2.0);
    assert!(
        (past.get_x() - 4.0).abs() < EPSILON,
        "factor 2 is extrapolation, not clamping, got {}",
        past.get_x()
    );
}

#[test]
fn a_scalar_and_a_colour_are_interpolable_too() {
    let scalar: f64 = lerp_of(0.0_f64, 10.0_f64, 0.25);
    assert!((scalar - 2.5).abs() < EPSILON, "got {scalar}");
    let colour: Color = lerp_of(
        Color::new(0.0, 0.0, 0.0, 0.0),
        Color::new(1.0, 1.0, 1.0, 1.0),
        0.5,
    );
    let rendered: String = format!("{colour:?}");
    for channel in ["red", "green", "blue", "alpha"] {
        let key: String = format!("{channel}: ");
        let start: usize = rendered
            .find(&key)
            .unwrap_or_else(|| panic!("{channel} missing from {rendered}"))
            + key.len();
        let rest: &str = &rendered[start..];
        let end: usize = rest
            .find(|c: char| [',', '}', ' '].contains(&c))
            .unwrap_or(rest.len());
        let value: f64 = rest[..end]
            .parse()
            .unwrap_or_else(|_| panic!("{channel} is not numeric: {rest}"));
        assert!(
            (value - 0.5).abs() < EPSILON,
            "every {channel} channel must be halfway, got {value}"
        );
    }
}

#[test]
fn both_vector_dimensions_satisfy_the_same_trait() {
    let two: f64 = magnitude_of(Vector2D::new(3.0, 4.0));
    assert!((two - 5.0).abs() < EPSILON, "the 3-4-5 triangle, got {two}");
    let three: f64 = magnitude_of(Vector3D::new(0.0, 3.0, 4.0));
    assert!(
        (three - 5.0).abs() < EPSILON,
        "the trait must work in 3D too, got {three}"
    );
}

#[test]
fn the_zero_of_every_vector_dimension_is_the_additive_identity() {
    let two: Vector2D = <Vector2D as Vector>::zero();
    let three: Vector3D = <Vector3D as Vector>::zero();
    assert_eq!(two, Vector2D::new(0.0, 0.0));
    assert_eq!(three, Vector3D::new(0.0, 0.0, 0.0));
    assert!((magnitude_of(two) - 0.0).abs() < EPSILON);
}

#[test]
fn the_dot_product_is_symmetric_through_the_trait() {
    let forward: f64 = dot_of(Vector2D::new(1.0, 2.0), Vector2D::new(3.0, 4.0));
    let backward: f64 = dot_of(Vector2D::new(3.0, 4.0), Vector2D::new(1.0, 2.0));
    assert!(
        (forward - 11.0).abs() < EPSILON,
        "1*3 + 2*4 = 11, got {forward}"
    );
    assert!(
        (forward - backward).abs() < EPSILON,
        "the dot product must not depend on the argument order"
    );
}

#[test]
fn a_default_ray_sits_at_the_origin_pointing_nowhere() {
    let ray: Ray2D = Ray2D::default();
    assert_eq!(
        format!("{ray:?}"),
        "Ray2D { origin: Vector2D { x: 0.0, y: 0.0 }, direction: Vector2D { x: 0.0, y: 0.0 } }",
        "a default ray must be a fully zeroed value"
    );
}

#[test]
fn a_ray_keeps_the_origin_and_direction_it_was_built_with() {
    let ray: Ray2D = Ray2D::new(Vector2D::new(1.0, 2.0), Vector2D::new(0.0, 1.0));
    assert!(
        format!("{ray:?}").contains("origin: Vector2D { x: 1.0, y: 2.0 }"),
        "the origin must survive, got {ray:?}"
    );
    assert!(
        format!("{ray:?}").contains("direction: Vector2D { x: 0.0, y: 1.0 }"),
        "the direction must survive, got {ray:?}"
    );
}

#[test]
fn interpolating_a_scalar_reaches_both_endpoints_exactly() {
    let start: f64 = 2.0;
    let end: f64 = 6.0;
    assert_eq!(start.lerp(end, 0.0), 2.0, "factor zero stays at self");
    assert_eq!(start.lerp(end, 1.0), 6.0, "factor one reaches other");
    assert_eq!(start.lerp(end, 0.5), 4.0, "the midpoint is halfway");
}

#[test]
fn interpolating_a_scalar_extrapolates_outside_the_unit_interval() {
    assert_eq!(0.0_f64.lerp(10.0, 2.0), 20.0, "the factor is not clamped");
    assert_eq!(
        0.0_f64.lerp(10.0, -1.0),
        -10.0,
        "a negative factor runs backwards"
    );
}

#[test]
fn interpolating_a_vector_moves_each_component_independently() {
    let from: Vector2D = Vector2D::new(0.0, 10.0);
    let to: Vector2D = Vector2D::new(10.0, 20.0);
    let mid: Vector2D = from.lerp(to, 0.5);
    assert_eq!(mid.get_x(), 5.0);
    assert_eq!(mid.get_y(), 15.0);
    assert_eq!(from.lerp(to, 0.0).get_x(), 0.0);
    assert_eq!(from.lerp(to, 1.0).get_y(), 20.0);
}

#[test]
fn interpolating_a_three_dimensional_vector_moves_each_component_independently() {
    let from: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let to: Vector3D = Vector3D::new(4.0, 8.0, 12.0);
    let mid: Vector3D = from.lerp(to, 0.25);
    assert_eq!(mid.get_x(), 1.0);
    assert_eq!(mid.get_y(), 2.0);
    assert_eq!(mid.get_z(), 3.0);
}

#[test]
fn interpolating_a_color_blends_the_rgba_channels_separately() {
    let black: Color = Color::new(0.0, 0.0, 0.0, 0.0);
    let white: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    let grey: Color = black.lerp(white, 0.5);
    assert_eq!(grey.get_red(), 0.5);
    assert_eq!(grey.get_green(), 0.5);
    assert_eq!(grey.get_blue(), 0.5);
    assert_eq!(
        grey.get_alpha(),
        0.5,
        "alpha blends with the rest, it is not special"
    );
    assert_eq!(black.lerp(white, 1.0).get_alpha(), 1.0);
}

fn halfway<T: Interpolable>(from: T, to: T) -> T {
    from.lerp(to, 0.5)
}

#[test]
fn the_interpolable_trait_is_usable_as_a_generic_bound() {
    assert_eq!(halfway(0.0_f64, 10.0), 5.0);
    assert_eq!(
        halfway(Vector2D::zero(), Vector2D::new(8.0, 2.0)).get_y(),
        1.0
    );
    assert_eq!(
        halfway(Vector3D::zero(), Vector3D::new(0.0, 0.0, 7.0)).get_z(),
        3.5
    );
    assert_eq!(
        halfway(
            Color::new(0.0, 0.0, 0.0, 0.0),
            Color::new(1.0, 1.0, 1.0, 1.0)
        )
        .get_red(),
        0.5
    );
}

#[test]
fn a_ray_is_copy_so_it_can_be_passed_to_several_intersection_tests() {
    let ray: Ray2D = Ray2D::new(Vector2D::new(0.0, 0.0), Vector2D::new(1.0, 0.0));
    let copied: Ray2D = ray;
    assert_eq!(copied, ray, "Ray2D is Copy, so a use must not move it");
}

#[test]
fn the_degree_and_radian_factors_are_exact_reciprocals() {
    let product: f64 = DEG_TO_RAD * RAD_TO_DEG;
    assert!(
        (product - 1.0).abs() < EPSILON,
        "the two conversion factors must undo each other, got {product}"
    );
    assert!((180.0 * DEG_TO_RAD - consts::PI).abs() < EPSILON);
    assert!(
        (HALF_PI * RAD_TO_DEG - 90.0).abs() < EPSILON,
        "a quarter turn is 90 degrees"
    );
    assert!(
        (TWO_PI * RAD_TO_DEG - 360.0).abs() < EPSILON,
        "a full turn is 360 degrees"
    );
}
