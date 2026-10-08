use super::*;

const HALF_PI: f64 = FRAC_PI_2;
fn close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-9
}

fn vec2_close(left: Vector2D, right: Vector2D) -> bool {
    close(left.get_x(), right.get_x()) && close(left.get_y(), right.get_y())
}

#[test]
fn the_zero_vector_is_the_origin() {
    let observed: Vector2D = Vector2D::zero();
    assert_eq!(observed, Vector2D::new(0.0, 0.0), "zero sits at the origin");
}

#[test]
fn the_right_vector_points_along_positive_x() {
    let observed: Vector2D = Vector2D::right();
    assert_eq!(observed, Vector2D::new(1.0, 0.0), "right is +x");
}

#[test]
fn the_up_vector_points_along_negative_y() {
    let observed: Vector2D = Vector2D::up();
    assert_eq!(
        observed,
        Vector2D::new(0.0, -1.0),
        "up is -y, screen coordinates"
    );
}

#[test]
fn from_angle_produces_a_unit_vector() {
    let observed: Vector2D = Vector2D::from_angle(0.0);
    assert!(
        vec2_close(observed, Vector2D::new(1.0, 0.0)),
        "zero radians is +x"
    );
}

#[test]
fn from_angle_turns_counter_clockwise_with_increasing_angle() {
    let quarter: Vector2D = Vector2D::from_angle(HALF_PI);
    assert!(
        vec2_close(quarter, Vector2D::new(0.0, 1.0)),
        "from_angle uses the math convention: a quarter turn is +y, got {quarter:?}"
    );
}

#[test]
fn magnitude_is_the_euclidean_length() {
    let observed: f64 = Vector2D::new(3.0, 4.0).magnitude();
    assert!(
        close(observed, 5.0),
        "a 3-4 vector is 5 long, got {observed}"
    );
}

#[test]
fn magnitude_of_the_zero_vector_is_zero() {
    let observed: f64 = Vector2D::zero().magnitude();
    assert_eq!(observed, 0.0, "the origin has no length");
}

#[test]
fn magnitude_squared_is_the_square_of_magnitude() {
    let vector: Vector2D = Vector2D::new(2.0, 3.0);
    let squared: f64 = vector.magnitude_squared();
    let plain: f64 = vector.magnitude();
    assert!(close(squared, 13.0), "2 squared plus 3 squared is 13");
    assert!(close(squared, plain * plain), "the two forms must agree");
}

#[test]
fn normalized_rescales_to_unit_length() {
    let observed: Vector2D = Vector2D::new(0.0, 5.0).normalized();
    assert!(
        close(observed.magnitude(), 1.0),
        "a normalized vector is unit length"
    );
}

#[test]
fn normalized_keeps_the_direction() {
    let observed: Vector2D = Vector2D::new(3.0, 4.0).normalized();
    assert!(
        vec2_close(observed, Vector2D::new(0.6, 0.8)),
        "the 3-4-5 direction is preserved, got {observed:?}"
    );
}

#[test]
fn normalized_of_the_zero_vector_stays_the_zero_vector() {
    let observed: Vector2D = Vector2D::zero().normalized();
    assert_eq!(observed, Vector2D::zero(), "there is no direction to keep");
}

#[test]
fn normalize_rewrites_the_vector_in_place() {
    let mut vector: Vector2D = Vector2D::new(0.0, 10.0);
    vector.normalize();
    assert!(
        close(vector.magnitude(), 1.0),
        "the same vector is now unit length"
    );
}

#[test]
fn dot_measures_parallelism() {
    let observed: f64 = Vector2D::new(1.0, 0.0).dot(Vector2D::new(1.0, 0.0));
    assert!(
        close(observed, 1.0),
        "a vector dotted with itself is its length squared"
    );
}

#[test]
fn dot_of_perpendicular_vectors_is_zero() {
    let observed: f64 = Vector2D::right().dot(Vector2D::up());
    assert!(
        close(observed, 0.0),
        "perpendicular vectors are orthogonal, got {observed}"
    );
}

#[test]
fn dot_of_opposite_vectors_is_negative() {
    let observed: f64 = Vector2D::new(2.0, 0.0).dot(Vector2D::new(-3.0, 0.0));
    assert!(
        close(observed, -6.0),
        "opposing vectors dot negative, got {observed}"
    );
}

#[test]
fn cross_measures_the_signed_parallelogram_area() {
    let observed: f64 = Vector2D::right().cross(Vector2D::up());
    assert!(
        close(observed, -1.0),
        "right x up is the -z component, got {observed}"
    );
}

#[test]
fn cross_of_parallel_vectors_is_zero() {
    let observed: f64 = Vector2D::new(2.0, 4.0).cross(Vector2D::new(1.0, 2.0));
    assert!(
        close(observed, 0.0),
        "collinear vectors enclose no area, got {observed}"
    );
}

#[test]
fn cross_of_swapped_vectors_negates() {
    let a: Vector2D = Vector2D::new(3.0, 1.0);
    let b: Vector2D = Vector2D::new(2.0, 5.0);
    let forward: f64 = a.cross(b);
    let backward: f64 = b.cross(a);
    assert!(
        close(forward, -backward),
        "the cross product is antisymmetric"
    );
}

#[test]
fn perp_rotates_ninety_degrees_counter_clockwise() {
    let observed: Vector2D = Vector2D::right().perp();
    assert!(
        vec2_close(observed, Vector2D::new(0.0, 1.0)),
        "perp is counter-clockwise, so right becomes +y, got {observed:?}"
    );
}

#[test]
fn perp_of_perp_returns_the_original_flipped() {
    let original: Vector2D = Vector2D::new(1.0, 0.0);
    let observed: Vector2D = original.perp().perp();
    assert!(
        vec2_close(observed, Vector2D::new(-1.0, 0.0)),
        "two quarter turns is a half turn, got {observed:?}"
    );
}

#[test]
fn angle_of_right_is_zero() {
    let observed: f64 = Vector2D::right().angle();
    assert!(
        close(observed, 0.0),
        "the +x axis is the zero angle, got {observed}"
    );
}

#[test]
fn angle_of_up_is_a_negative_quarter_turn() {
    let observed: f64 = Vector2D::up().angle();
    assert!(
        close(observed, -HALF_PI),
        "up is -y, so its bearing is a negative quarter turn, got {observed}"
    );
}

#[test]
fn angle_to_returns_the_bearing_to_the_other_vector_not_a_turn() {
    let observed: f64 = Vector2D::right().angle_to(Vector2D::up());
    assert!(
        close(observed, -0.75 * PI),
        "angle_to is atan2 of the difference vector, so the two positions \
         leak into the result: right to up measures atan2(-1, -1), not a quarter turn. Got {observed}"
    );
}

#[test]
fn angle_to_of_a_point_lone_target_is_its_bearing() {
    let observed: f64 = Vector2D::new(5.0, 5.0).angle_to(Vector2D::new(5.0, 4.0));
    assert!(
        close(observed, -HALF_PI),
        "straight down is a negative quarter turn, got {observed}"
    );
}

#[test]
fn angle_to_is_antisymmetric() {
    let a: Vector2D = Vector2D::new(1.0, 1.0);
    let b: Vector2D = Vector2D::new(1.0, -1.0);
    let forward: f64 = a.angle_to(b);
    let backward: f64 = b.angle_to(a);
    assert!(close(forward, -backward), "the turn back undoes the turn");
}

#[test]
fn rotated_returns_a_new_vector_and_leaves_the_original() {
    let original: Vector2D = Vector2D::right();
    let observed: Vector2D = original.rotated(HALF_PI);
    assert_eq!(original, Vector2D::right(), "rotated does not mutate");
    assert!(
        vec2_close(observed, Vector2D::new(0.0, 1.0)),
        "rotated is counter-clockwise, so right becomes +y, got {observed:?}"
    );
}

#[test]
fn rotate_mutates_in_place() {
    let mut vector: Vector2D = Vector2D::right();
    vector.rotate(HALF_PI);
    assert!(
        vec2_close(vector, Vector2D::new(0.0, 1.0)),
        "rotate turns the vector counter-clockwise, got {vector:?}"
    );
}

#[test]
fn distance_to_measures_the_gap_between_points() {
    let observed: f64 = Vector2D::zero().distance_to(Vector2D::new(3.0, 4.0));
    assert!(close(observed, 5.0), "the distance is 5, got {observed}");
}

#[test]
fn distance_to_is_symmetric() {
    let a: Vector2D = Vector2D::new(1.0, 2.0);
    let b: Vector2D = Vector2D::new(-3.0, 8.0);
    assert!(
        close(a.distance_to(b), b.distance_to(a)),
        "distance does not depend on the order"
    );
}

#[test]
fn distance_squared_to_is_the_square_of_distance_to() {
    let a: Vector2D = Vector2D::new(0.0, 0.0);
    let b: Vector2D = Vector2D::new(1.0, 1.0);
    assert!(
        close(a.distance_squared_to(b), 2.0),
        "a diagonal unit offset squares to 2"
    );
    assert!(
        close(
            a.distance_squared_to(b),
            a.distance_to(b) * a.distance_to(b)
        ),
        "the squared form matches the square of the plain form"
    );
}

#[test]
fn direction_to_points_from_one_point_towards_another() {
    let observed: Vector2D = Vector2D::zero().direction_to(Vector2D::new(0.0, 10.0));
    assert!(
        vec2_close(observed, Vector2D::new(0.0, 1.0)),
        "the direction towards +y is +y, got {observed:?}"
    );
}

#[test]
fn direction_to_is_unit_length() {
    let observed: Vector2D = Vector2D::new(1.0, 1.0).direction_to(Vector2D::new(7.0, 9.0));
    assert!(
        close(observed.magnitude(), 1.0),
        "a direction is always normalized"
    );
}

#[test]
fn direction_to_itself_is_the_zero_vector() {
    let point: Vector2D = Vector2D::new(3.0, 4.0);
    let observed: Vector2D = point.direction_to(point);
    assert_eq!(
        observed,
        Vector2D::zero(),
        "there is no direction to the same point"
    );
}

#[test]
fn lerp_moves_between_two_vectors() {
    let from: Vector2D = Vector2D::new(0.0, 0.0);
    let to: Vector2D = Vector2D::new(10.0, 20.0);
    let observed: Vector2D = from.lerp(to, 0.5);
    assert!(
        vec2_close(observed, Vector2D::new(5.0, 10.0)),
        "halfway is the midpoint, got {observed:?}"
    );
}

#[test]
fn lerp_at_the_endpoints_lands_on_each_end() {
    let from: Vector2D = Vector2D::new(1.0, 1.0);
    let to: Vector2D = Vector2D::new(3.0, 5.0);
    assert_eq!(from.lerp(to, 0.0), from, "factor zero returns the start");
    assert_eq!(from.lerp(to, 1.0), to, "factor one returns the end");
}

#[test]
fn scaled_returns_a_scaled_copy_and_leaves_the_original() {
    let original: Vector2D = Vector2D::new(1.0, 2.0);
    let observed: Vector2D = original.scaled(3.0);
    assert_eq!(original, Vector2D::new(1.0, 2.0), "scaled does not mutate");
    assert!(
        vec2_close(observed, Vector2D::new(3.0, 6.0)),
        "each component is scaled, got {observed:?}"
    );
}

#[test]
fn scale_mutates_in_place() {
    let mut vector: Vector2D = Vector2D::new(1.0, 2.0);
    vector.scale(2.0);
    assert!(
        vec2_close(vector, Vector2D::new(2.0, 4.0)),
        "scale rewrites the vector, got {vector:?}"
    );
}

#[test]
fn adding_two_vectors_is_componentwise() {
    let observed: Vector2D = Vector2D::new(1.0, 2.0) + Vector2D::new(10.0, 20.0);
    assert!(
        vec2_close(observed, Vector2D::new(11.0, 22.0)),
        "the components add, got {observed:?}"
    );
}

#[test]
fn subtracting_two_vectors_is_componentwise() {
    let observed: Vector2D = Vector2D::new(10.0, 20.0) - Vector2D::new(1.0, 2.0);
    assert!(
        vec2_close(observed, Vector2D::new(9.0, 18.0)),
        "the components subtract, got {observed:?}"
    );
}

#[test]
fn multiplying_by_a_scalar_scales_the_vector() {
    let observed: Vector2D = Vector2D::new(1.0, 2.0) * 4.0;
    assert!(
        vec2_close(observed, Vector2D::new(4.0, 8.0)),
        "both components scale, got {observed:?}"
    );
}

#[test]
fn negating_a_vector_flips_both_components() {
    let observed: Vector2D = -Vector2D::new(1.0, -2.0);
    assert!(
        vec2_close(observed, Vector2D::new(-1.0, 2.0)),
        "negation flips the signs, got {observed:?}"
    );
}

#[test]
fn add_assign_accumulates_into_the_left_vector() {
    let mut vector: Vector2D = Vector2D::new(1.0, 1.0);
    vector += Vector2D::new(2.0, 3.0);
    assert!(
        vec2_close(vector, Vector2D::new(3.0, 4.0)),
        "the accumulator holds the sum, got {vector:?}"
    );
}

#[test]
fn sub_assign_accumulates_into_the_left_vector() {
    let mut vector: Vector2D = Vector2D::new(5.0, 5.0);
    vector -= Vector2D::new(1.0, 2.0);
    assert!(
        vec2_close(vector, Vector2D::new(4.0, 3.0)),
        "the accumulator holds the difference, got {vector:?}"
    );
}

#[test]
fn mul_assign_scales_the_left_vector() {
    let mut vector: Vector2D = Vector2D::new(2.0, 3.0);
    vector *= 2.0;
    assert!(
        vec2_close(vector, Vector2D::new(4.0, 6.0)),
        "the accumulator is scaled, got {vector:?}"
    );
}

#[test]
fn the_three_dimensional_magnitude_is_the_space_diagonal() {
    let observed: f64 = Vector3D::new(2.0, 3.0, 6.0).magnitude();
    assert!(
        close(observed, 7.0),
        "a 2-3-6 diagonal is 7, got {observed}"
    );
}

#[test]
fn the_three_dimensional_dot_product_follows_the_same_rules() {
    let observed: f64 = Vector3D::new(1.0, 0.0, 0.0).dot(Vector3D::new(0.0, 1.0, 0.0));
    assert!(close(observed, 0.0), "perpendicular axes are orthogonal");
}

#[test]
fn the_three_dimensional_cross_product_is_perpendicular_to_both_inputs() {
    let a: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
    let b: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let observed: Vector3D = a.cross(b);
    assert!(
        close(a.dot(observed), 0.0) && close(b.dot(observed), 0.0),
        "x cross y is the z axis, got {observed:?}"
    );
}

#[test]
fn the_three_dimensional_normalized_vector_is_unit_length() {
    let observed: Vector3D = Vector3D::new(0.0, 3.0, 4.0).normalized();
    assert!(
        close(observed.magnitude(), 1.0),
        "a normalized vector is unit length"
    );
}

#[test]
fn three_dimensional_addition_is_componentwise() {
    let observed: Vector3D = Vector3D::new(1.0, 2.0, 3.0) + Vector3D::new(10.0, 20.0, 30.0);
    assert_eq!(
        observed,
        Vector3D::new(11.0, 22.0, 33.0),
        "the components add"
    );
}

#[test]
fn a_rect_from_center_keeps_its_centre() {
    let observed: Rect = Rect::from_center(Vector2D::new(10.0, 20.0), 4.0, 6.0);
    assert!(
        vec2_close(observed.center(), Vector2D::new(10.0, 20.0)),
        "the centre round-trips, got {:?}",
        observed.center()
    );
}

#[test]
fn a_rect_from_center_keeps_its_size() {
    let observed: Rect = Rect::from_center(Vector2D::new(0.0, 0.0), 4.0, 6.0);
    assert!(close(observed.get_width(), 4.0), "the width is stored");
    assert!(close(observed.get_height(), 6.0), "the height is stored");
}

#[test]
fn a_rect_reports_its_min_and_max_corners() {
    let observed: Rect = Rect::from_center(Vector2D::new(5.0, 5.0), 4.0, 4.0);
    assert!(
        vec2_close(observed.min(), Vector2D::new(3.0, 3.0)),
        "min is the top-left corner, got {:?}",
        observed.min()
    );
    assert!(
        vec2_close(observed.max(), Vector2D::new(7.0, 7.0)),
        "max is the bottom-right corner, got {:?}",
        observed.max()
    );
}

#[test]
fn a_rect_contains_a_point_inside_it() {
    let observed: Rect = Rect::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    assert!(
        observed.contains(Vector2D::new(1.0, 1.0)),
        "a point inside is contained"
    );
}

#[test]
fn a_rect_does_not_contain_a_point_outside_it() {
    let observed: Rect = Rect::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    assert!(
        !observed.contains(Vector2D::new(5.0, 0.0)),
        "a point past the edge is out"
    );
}

#[test]
fn overlapping_rects_intersect() {
    let a: Rect = Rect::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let b: Rect = Rect::from_center(Vector2D::new(2.0, 0.0), 4.0, 4.0);
    assert!(a.intersects(b), "rects sharing an edge region intersect");
}

#[test]
fn separated_rects_do_not_intersect() {
    let a: Rect = Rect::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let b: Rect = Rect::from_center(Vector2D::new(50.0, 0.0), 4.0, 4.0);
    assert!(!a.intersects(b), "distant rects do not intersect");
}

#[test]
fn a_rect_with_a_zero_size_still_answers_queries() {
    let observed: Rect = Rect::from_center(Vector2D::new(1.0, 1.0), 0.0, 0.0);
    assert!(
        close(observed.get_width(), 0.0),
        "a degenerate rect has no width"
    );
    let size: Vector2D = observed.size();
    assert!(
        close(size.get_x(), 0.0) && close(size.get_y(), 0.0),
        "and therefore a zero size vector"
    );
}

#[test]
fn a_rect_size_is_width_and_height_as_a_vector() {
    let observed: Vector2D = Rect::from_center(Vector2D::new(0.0, 0.0), 4.0, 6.0).size();
    assert!(
        vec2_close(observed, Vector2D::new(4.0, 6.0)),
        "size reports the dimensions, got {observed:?}"
    );
}

#[test]
fn two_overlapping_rects_intersect_in_their_shared_box() {
    let a: Rect = Rect::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let b: Rect = Rect::from_center(Vector2D::new(2.0, 0.0), 4.0, 4.0);
    let observed: Option<Rect> = a.intersection(b);
    match observed {
        Some(shared) => assert!(
            close(shared.get_width(), 2.0) && close(shared.get_height(), 4.0),
            "the shared box is two wide and four tall, got {shared:?}"
        ),
        None => panic!("overlapping rects must have an intersection"),
    }
}

#[test]
fn two_separated_rects_have_no_intersection() {
    let a: Rect = Rect::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let b: Rect = Rect::from_center(Vector2D::new(50.0, 0.0), 4.0, 4.0);
    let observed: Option<Rect> = a.intersection(b);
    assert!(observed.is_none(), "distant rects share no box at all");
}

#[test]
fn the_rect_broad_phase_alias_agrees_with_intersects() {
    let a: Rect = Rect::from_center(Vector2D::new(0.0, 0.0), 4.0, 4.0);
    let b: Rect = Rect::from_center(Vector2D::new(2.0, 0.0), 4.0, 4.0);
    let far: Rect = Rect::from_center(Vector2D::new(50.0, 0.0), 4.0, 4.0);
    assert_eq!(
        Rect::broad_phase_alias(a, b),
        a.intersects(b),
        "the alias and intersects must agree on an overlapping pair"
    );
    assert_eq!(
        Rect::broad_phase_alias(a, far),
        a.intersects(far),
        "and on a distant pair"
    );
}

#[test]
fn a_circle_keeps_the_centre_and_radius_it_was_built_from() {
    let observed: Circle = Circle::new(Vector2D::new(3.0, -4.0), 5.0);
    assert!(
        vec2_close(observed.get_center(), Vector2D::new(3.0, -4.0)),
        "the centre is stored"
    );
    assert!(close(observed.get_radius(), 5.0), "the radius is stored");
}

#[test]
fn a_circle_contains_a_point_inside_it() {
    let observed: Circle = Circle::new(Vector2D::new(0.0, 0.0), 5.0);
    assert!(
        observed.contains(Vector2D::new(3.0, 4.0)),
        "a 3-4-5 point is on the rim"
    );
}

#[test]
fn a_circle_does_not_contain_a_point_outside_it() {
    let observed: Circle = Circle::new(Vector2D::new(0.0, 0.0), 5.0);
    assert!(
        !observed.contains(Vector2D::new(6.0, 0.0)),
        "a point past the radius is out"
    );
}

#[test]
fn a_circle_area_is_pi_r_squared() {
    let observed: f64 = Circle::new(Vector2D::new(0.0, 0.0), 2.0).area();
    assert!(close(observed, PI * 4.0), "pi r squared, got {observed}");
}

#[test]
fn a_circle_of_radius_two_has_a_circumference_of_four_pi() {
    let observed: f64 = Circle::new(Vector2D::new(0.0, 0.0), 2.0).circumference();
    assert!(
        close(observed, PI * 4.0),
        "two pi r is four pi, got {observed}"
    );
}

#[test]
fn a_circle_at_the_origin_has_zero_area() {
    let observed: f64 = Circle::new(Vector2D::new(0.0, 0.0), 0.0).area();
    assert_eq!(observed, 0.0, "a degenerate circle encloses nothing");
}
