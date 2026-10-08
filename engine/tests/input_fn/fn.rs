use super::*;

#[test]
fn a_value_inside_the_deadzone_reads_as_zero() {
    let observed: f64 = apply_axis_deadzone(0.1);
    assert_eq!(
        observed, 0.0,
        "joystick drift below the deadzone is dropped"
    );
}

#[test]
fn the_deadzone_boundary_itself_reads_as_zero() {
    let observed: f64 = apply_axis_deadzone(0.15);
    assert_eq!(
        observed, 0.0,
        "the comparison is inclusive, so exactly at the deadzone is still zero"
    );
}

#[test]
fn a_value_just_outside_the_deadzone_rescales_upward() {
    let observed: f64 = apply_axis_deadzone(0.16);
    assert!(
        observed > 0.0,
        "crossing the deadzone must not stay at zero, got {observed}"
    );
}

#[test]
fn a_full_deflection_rescales_to_one() {
    let observed: f64 = apply_axis_deadzone(1.0);
    assert!(
        (observed - 1.0).abs() < 1e-9,
        "the post-deadzone span is rescaled onto the full range, got {observed}"
    );
}

#[test]
fn a_full_negative_deflection_rescales_to_minus_one() {
    let observed: f64 = apply_axis_deadzone(-1.0);
    assert!(
        (observed + 1.0).abs() < 1e-9,
        "the sign is preserved through the rescale, got {observed}"
    );
}

#[test]
fn the_deadzone_rescale_is_symmetric_about_zero() {
    let positive: f64 = apply_axis_deadzone(0.6);
    let negative: f64 = apply_axis_deadzone(-0.6);
    assert!(
        (positive + negative).abs() < 1e-9,
        "mirror inputs must give mirror outputs, got {positive} vs {negative}"
    );
}

#[test]
fn an_over_range_value_is_clamped_to_one() {
    let observed: f64 = apply_axis_deadzone(1.5);
    assert!(
        (observed - 1.0).abs() < 1e-9,
        "an out-of-range reading must not rescale past one, got {observed}"
    );
}

#[test]
fn an_over_range_negative_value_is_clamped_to_minus_one() {
    let observed: f64 = apply_axis_deadzone(-1.5);
    assert!(
        (observed + 1.0).abs() < 1e-9,
        "an out-of-range negative must not pass minus one, got {observed}"
    );
}

#[test]
fn exactly_zero_reads_as_zero() {
    let observed: f64 = apply_axis_deadzone(0.0);
    assert_eq!(observed, 0.0, "a centred stick is zero");
}

#[test]
fn a_button_at_the_press_threshold_counts_as_down() {
    let observed: bool = is_button_down(0.5);
    assert!(observed, "the threshold comparison is inclusive");
}

#[test]
fn a_button_just_below_the_threshold_is_up() {
    let observed: bool = is_button_down(0.49);
    assert!(!observed, "just under the threshold is still released");
}

#[test]
fn a_fully_pressed_button_is_down() {
    let observed: bool = is_button_down(1.0);
    assert!(observed, "full deflection is definitely down");
}

#[test]
fn a_released_button_is_up() {
    let observed: bool = is_button_down(0.0);
    assert!(!observed, "no deflection is up");
}

#[test]
fn compute_held_reports_every_button_currently_down() {
    let observed: GamepadButtonSet = compute_held_buttons(&[1.0, 0.0, 1.0]);
    assert_eq!(observed.len(), 2, "two of the three are down");
    assert!(
        observed.contains(&0) && observed.contains(&2),
        "the right two"
    );
}

#[test]
fn compute_held_of_no_buttons_is_empty() {
    let observed: GamepadButtonSet = compute_held_buttons(&[0.0, 0.0]);
    assert!(observed.is_empty(), "nothing down means nothing held");
}

#[test]
fn compute_pressed_reports_only_newly_down_buttons() {
    let mut previous: GamepadButtonSet = GamepadButtonSet::new();
    previous.insert(0);
    let observed: GamepadButtonSet = compute_pressed_buttons(&previous, &[1.0, 1.0]);
    assert_eq!(
        observed.len(),
        1,
        "only the newly pressed button is an edge"
    );
    assert!(observed.contains(&1), "the button that was not held before");
}

#[test]
fn holding_the_same_button_again_produces_no_press_edge() {
    let mut previous: GamepadButtonSet = GamepadButtonSet::new();
    previous.insert(0);
    previous.insert(1);
    let observed: GamepadButtonSet = compute_pressed_buttons(&previous, &[1.0, 1.0]);
    assert!(observed.is_empty(), "a sustained hold is not a new press");
}

#[test]
fn compute_pressed_from_nothing_holds_reports_everything_down() {
    let previous: GamepadButtonSet = GamepadButtonSet::new();
    let observed: GamepadButtonSet = compute_pressed_buttons(&previous, &[1.0, 1.0, 1.0]);
    assert_eq!(observed.len(), 3, "the first frame of a press is an edge");
}

#[test]
fn compute_released_reports_only_newly_up_buttons() {
    let mut previous: GamepadButtonSet = GamepadButtonSet::new();
    previous.insert(0);
    previous.insert(1);
    let observed: GamepadButtonSet = compute_released_buttons(&previous, &[0.0, 1.0]);
    assert_eq!(observed.len(), 1, "only the button that came up");
    assert!(observed.contains(&0), "the released one, not the held one");
}

#[test]
fn a_sustained_hold_produces_no_release_edge() {
    let mut previous: GamepadButtonSet = GamepadButtonSet::new();
    previous.insert(0);
    let observed: GamepadButtonSet = compute_released_buttons(&previous, &[1.0]);
    assert!(observed.is_empty(), "a button still down is not released");
}

#[test]
fn a_button_whose_slot_vanished_counts_as_released() {
    let mut previous: GamepadButtonSet = GamepadButtonSet::new();
    previous.insert(5);
    let observed: GamepadButtonSet = compute_released_buttons(&previous, &[]);
    assert!(
        observed.contains(&5),
        "a previously held button that is no longer reported is released, not stuck"
    );
}

#[test]
fn press_and_release_edges_never_land_on_the_same_button() {
    let mut previous: GamepadButtonSet = GamepadButtonSet::new();
    previous.insert(0);
    let values: [f64; 2] = [0.0, 1.0];
    let pressed: GamepadButtonSet = compute_pressed_buttons(&previous, &values);
    let released: GamepadButtonSet = compute_released_buttons(&previous, &values);
    assert_eq!(pressed.len(), 1, "button 1 came down");
    assert_eq!(released.len(), 1, "button 0 came up");
    assert!(
        pressed.contains(&1) && released.contains(&0),
        "the two edges belong to different buttons"
    );
}

#[test]
fn a_frame_with_no_changes_produces_no_edges_at_all() {
    let mut previous: GamepadButtonSet = GamepadButtonSet::new();
    previous.insert(0);
    previous.insert(1);
    let values: [f64; 2] = [1.0, 1.0];
    let pressed: GamepadButtonSet = compute_pressed_buttons(&previous, &values);
    let released: GamepadButtonSet = compute_released_buttons(&previous, &values);
    assert!(pressed.is_empty(), "nothing came down");
    assert!(released.is_empty(), "nothing came up");
}

#[test]
fn read_raw_axis_returns_the_value_at_the_index() {
    let observed: f64 = read_raw_axis(&[0.1, 0.2, 0.3], 1);
    assert!(
        (observed - 0.2).abs() < 1e-9,
        "the axis is read positionally, got {observed}"
    );
}

#[test]
fn read_raw_axis_beyond_the_slice_reads_as_zero() {
    let observed: f64 = read_raw_axis(&[0.1], 9);
    assert_eq!(
        observed, 0.0,
        "a missing axis reads as centred, not a panic"
    );
}

#[test]
fn read_raw_axis_from_an_empty_slice_reads_as_zero() {
    let observed: f64 = read_raw_axis(&[], 0);
    assert_eq!(observed, 0.0, "no axes reported means centred");
}

#[test]
fn read_raw_axis_index_zero_is_the_first_axis() {
    let observed: f64 = read_raw_axis(&[0.75, 0.0], 0);
    assert!(
        (observed - 0.75).abs() < 1e-9,
        "axis zero is the first slot, got {observed}"
    );
}
