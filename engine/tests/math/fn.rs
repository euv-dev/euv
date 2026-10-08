use super::*;

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
fn deg_to_rad_converts_a_quarter_turn() {
    let observed: f64 = Numeric::deg_to_rad(90.0);
    assert!(
        (observed - HALF_PI).abs() < EPSILON,
        "90 degrees is half pi radians, got {observed}"
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
fn normalize_angle_folds_a_lagging_angle_into_the_negative_half() {
    let observed: f64 = Numeric::normalize_angle(1.75 * PI);
    assert!(
        (observed + 0.25 * PI).abs() < EPSILON,
        "7pi/4 must fold onto -pi/4, got {observed}"
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
fn angle_delta_measures_a_backward_turn_as_negative() {
    let observed: f64 = Numeric::angle_delta(HALF_PI, 0.0);
    assert!(
        (observed + HALF_PI).abs() < EPSILON,
        "a quarter turn back is minus half pi, got {observed}"
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
