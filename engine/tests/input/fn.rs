use super::*;

fn epsilon(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 1e-9
}

#[test]
fn engine_handle_starts_without_input_cell() {
    let config: EngineConfig = EngineConfig::default();
    let handle: EngineHandle = EngineHandle::new(config, None, None, None, None);
    let cell: &Option<InputStateCell> = handle.try_get_input_cell();
    assert!(cell.is_none());
}

#[test]
fn input_state_frame_lifecycle() {
    let mut state: InputState = InputState::new();
    state.press_key("KeyW".to_string());
    state.press_mouse_button(MouseButton::Left, Vector2D::new(10.0, 20.0));
    state.update_mouse_position(Vector2D::new(12.0, 24.0));
    state.start_touch(1, Vector2D::new(5.0, 6.0));
    assert!(state.get_keys_pressed().contains("KeyW"));
    assert!(state.get_keys_held().contains("KeyW"));
    assert!(
        state
            .get_mouse_buttons_pressed()
            .contains(&MouseButton::Left)
    );
    assert!(state.get_mouse_buttons_held().contains(&MouseButton::Left));
    assert_eq!(state.get_mouse_position().get_x(), 12.0);
    assert_eq!(state.get_mouse_position().get_y(), 24.0);
    assert!(state.get_touch_points().contains_key(&1));
    assert!(state.get_touch_started().contains(&1));
    state.end_frame();
    assert!(state.get_keys_pressed().is_empty());
    assert!(state.get_keys_held().contains("KeyW"));
    assert!(state.get_mouse_buttons_pressed().is_empty());
    assert!(state.get_mouse_buttons_held().contains(&MouseButton::Left));
    assert!(!state.get_mouse_moved());
    assert!(state.get_touch_started().is_empty());
    assert!(state.get_touch_points().contains_key(&1));
    state.release_key("KeyW".to_string());
    state.release_mouse_button(MouseButton::Left);
    state.end_touch(1);
    assert!(state.get_keys_released().contains("KeyW"));
    assert!(!state.get_keys_held().contains("KeyW"));
    assert!(!state.get_mouse_buttons_held().contains(&MouseButton::Left));
    assert!(!state.get_touch_points().contains_key(&1));
    assert!(state.get_touch_ended().contains(&1));
}

#[test]
fn scheduler_tick_clears_edge_state_after_render() {
    let cell: InputStateCell = Rc::new(EngineCell::new(InputState::default()));
    let state: &mut InputState = cell.get_mut();
    state.press_key(String::from("KeyW"));
    state.press_mouse_button(MouseButton::Left, Vector2D::new(10.0, 20.0));
    state.update_mouse_position(Vector2D::new(14.0, 26.0));
    state.start_touch(1, Vector2D::new(5.0, 6.0));
    assert!(state.get_keys_pressed().contains("KeyW"));
    assert!(
        state
            .get_mouse_buttons_pressed()
            .contains(&MouseButton::Left)
    );
    let (handler, updates, renders): (TickHandlerRc, Rc<Cell<u32>>, Rc<Cell<u32>>) =
        CountingHandler::spawn();
    let config: SchedulerConfig = SchedulerConfig::default();
    let mut scheduler: SchedulerState = SchedulerState::default();
    scheduler.tick(&config, &handler, None, Some(&cell));
    let state: &InputState = cell.get();
    assert!(state.get_keys_pressed().is_empty());
    assert!(state.get_keys_released().is_empty());
    assert!(state.get_mouse_buttons_pressed().is_empty());
    assert!(state.get_mouse_buttons_released().is_empty());
    assert!(state.get_touch_started().is_empty());
    assert!(state.get_touch_ended().is_empty());
    assert!(!state.get_mouse_moved());
    assert!(state.get_mouse_delta().magnitude() < 1e-9);
    assert!(state.get_keys_held().contains("KeyW"));
    assert!(state.get_mouse_buttons_held().contains(&MouseButton::Left));
    assert!(state.get_touch_points().contains_key(&1));
    assert!(state.get_mouse_position().get_x() == 14.0);
    assert_eq!(updates.get(), 1);
    assert_eq!(renders.get(), 1);
}

#[test]
fn scheduler_tick_second_frame_does_not_replay_stale_edges() {
    let cell: InputStateCell = Rc::new(EngineCell::new(InputState::default()));
    let (handler, _updates, _renders): (TickHandlerRc, Rc<Cell<u32>>, Rc<Cell<u32>>) =
        CountingHandler::spawn();
    let config: SchedulerConfig = SchedulerConfig::default();
    let mut scheduler: SchedulerState = SchedulerState::default();
    cell.get_mut().press_key(String::from("Space"));
    scheduler.tick(&config, &handler, None, Some(&cell));
    scheduler.tick(&config, &handler, None, Some(&cell));
    let state: &InputState = cell.get();
    assert!(state.get_keys_pressed().is_empty());
    assert!(state.get_keys_held().contains("Space"));
}

#[test]
fn scheduler_tick_without_input_cell_still_runs_both_callbacks() {
    let (handler, updates, renders): (TickHandlerRc, Rc<Cell<u32>>, Rc<Cell<u32>>) =
        CountingHandler::spawn();
    let config: SchedulerConfig = SchedulerConfig::default();
    let mut scheduler: SchedulerState = SchedulerState::default();
    scheduler.tick(&config, &handler, None, None);
    scheduler.tick(&config, &handler, None, None);
    assert!(updates.get() >= 1);
    assert_eq!(renders.get(), 2);
    assert!(scheduler.get_frame_count() >= 2);
    let _: u64 = scheduler.get_update_count();
}

#[test]
fn a_key_press_is_both_pressed_this_frame_and_held() {
    let mut state: InputState = InputState::default();
    assert!(
        !state.is_key_held("KeyA"),
        "nothing is held on a fresh state"
    );
    state.press_key(String::from("KeyA"));
    assert!(
        state.is_key_pressed("KeyA"),
        "the press is an edge this frame"
    );
    assert!(state.is_key_held("KeyA"), "and the key is now down");
    assert!(!state.is_key_released("KeyA"), "a press is not a release");
}

#[test]
fn a_key_release_clears_held_and_raises_the_release_edge() {
    let mut state: InputState = InputState::default();
    state.press_key(String::from("KeyA"));
    state.release_key(String::from("KeyA"));
    assert!(!state.is_key_held("KeyA"), "the key is up again");
    assert!(state.is_key_released("KeyA"), "and the release is an edge");
    assert!(
        state.is_key_pressed("KeyA"),
        "a press AND release inside one frame reports both edges, since release_key only moves the key out of held"
    );
    state.end_frame();
    assert!(
        !state.is_key_pressed("KeyA"),
        "and both edges clear on the frame boundary"
    );
}

#[test]
fn releasing_a_key_that_was_never_pressed_is_still_reported() {
    let mut state: InputState = InputState::default();
    state.release_key(String::from("KeyZ"));
    assert!(
        state.is_key_released("KeyZ"),
        "the release edge is recorded"
    );
    assert!(!state.is_key_held("KeyZ"), "and the key is not held");
}

#[test]
fn end_frame_clears_the_edges_but_keeps_what_is_still_down() {
    let mut state: InputState = InputState::default();
    state.press_key(String::from("KeyA"));
    state.press_key(String::from("KeyB"));
    state.release_key(String::from("KeyB"));
    state.end_frame();
    assert!(
        !state.is_key_pressed("KeyA"),
        "the press edge lasted one frame"
    );
    assert!(
        !state.is_key_released("KeyB"),
        "the release edge lasted one frame"
    );
    assert!(
        state.is_key_held("KeyA"),
        "but the key is still physically down"
    );
    assert!(!state.is_key_held("KeyB"), "and the released one is not");
}

#[test]
fn a_second_frame_after_end_frame_reports_no_edges_at_all() {
    let mut state: InputState = InputState::default();
    state.press_key(String::from("KeyA"));
    state.end_frame();
    state.end_frame();
    assert!(!state.is_key_pressed("KeyA"), "edges are not re-raised");
    assert!(
        state.is_key_held("KeyA"),
        "and holding persists across frames"
    );
}

#[test]
fn several_keys_are_tracked_independently() {
    let mut state: InputState = InputState::default();
    state.press_key(String::from("KeyW"));
    state.press_key(String::from("ShiftLeft"));
    assert!(state.is_key_held("KeyW"), "the first key is down");
    assert!(state.is_key_held("ShiftLeft"), "the second key is down");
    assert!(!state.is_key_held("KeyA"), "an untouched key is not");
    state.release_key(String::from("KeyW"));
    assert!(
        !state.is_key_held("KeyW"),
        "releasing one leaves the other alone"
    );
    assert!(
        state.is_key_held("ShiftLeft"),
        "the second key is untouched"
    );
}

#[test]
fn mouse_buttons_follow_the_same_press_release_contract() {
    let mut state: InputState = InputState::default();
    state.press_mouse_button(MouseButton::Left, Vector2D::new(4.0, 9.0));
    assert!(
        state.is_mouse_button_pressed(MouseButton::Left),
        "the press is an edge"
    );
    assert!(
        state.is_mouse_button_held(MouseButton::Left),
        "and it is down"
    );
    assert!(
        !state.is_mouse_button_pressed(MouseButton::Right),
        "the other button is not"
    );
    state.release_mouse_button(MouseButton::Left);
    assert!(
        !state.is_mouse_button_held(MouseButton::Left),
        "releasing clears held"
    );
}

#[test]
fn the_mouse_position_is_stored_where_it_was_last_put() {
    let mut state: InputState = InputState::default();
    state.update_mouse_position(Vector2D::new(120.0, 340.0));
    assert!(
        epsilon(state.get_mouse_position().get_x(), 120.0)
            && epsilon(state.get_mouse_position().get_y(), 340.0),
        "the position round-trips, got {:?}",
        state.get_mouse_position()
    );
}

#[test]
fn touches_are_tracked_per_identifier() {
    let mut state: InputState = InputState::default();
    assert!(
        state.primary_touch_position().is_none(),
        "no touch is active yet"
    );
    state.start_touch(1, Vector2D::new(10.0, 20.0));
    state.update_touch(1, Vector2D::new(15.0, 25.0));
    let active: Option<Vector2D> = state.primary_touch_position();
    let position: Vector2D = active.expect("touch 1 is active");
    assert!(
        epsilon(position.get_x(), 15.0) && epsilon(position.get_y(), 25.0),
        "the tracked position is the latest one, got {position:?}"
    );
    state.end_touch(1);
    assert!(
        state.primary_touch_position().is_none(),
        "ending the touch clears it"
    );
}

#[test]
fn the_axis_deadzone_flattens_a_centre_resting_stick() {
    assert!(
        epsilon(apply_axis_deadzone(0.0), 0.0),
        "an axis at rest reads as zero"
    );
    assert!(
        epsilon(apply_axis_deadzone(0.05), 0.0),
        "and so does drift inside the deadzone, got {}",
        apply_axis_deadzone(0.05)
    );
    assert!(
        epsilon(apply_axis_deadzone(-0.05), 0.0),
        "on the negative side too"
    );
}

#[test]
fn the_deadzone_rescales_the_remaining_span_back_onto_the_full_range() {
    let full: f64 = apply_axis_deadzone(1.0);
    assert!(
        epsilon(full, 1.0),
        "a fully deflected stick still reads one"
    );
    let half: f64 = apply_axis_deadzone(0.5);
    assert!(
        half > 0.05 && half < 1.0,
        "a half deflection must survive the deadzone as something between the raw value and full scale, got {half}"
    );
    assert!(
        apply_axis_deadzone(0.9) > half,
        "a larger deflection must read larger after rescaling"
    );
}

#[test]
fn the_deadzone_is_sign_preserving() {
    assert!(
        apply_axis_deadzone(-1.0) < 0.0,
        "a fully negative stick stays negative"
    );
    assert!(
        epsilon(apply_axis_deadzone(-1.0), -apply_axis_deadzone(1.0)),
        "and the mapping is symmetric about zero"
    );
    assert!(
        apply_axis_deadzone(-2.0).abs() <= 1.0,
        "an over-range reading is clamped rather than amplified past one"
    );
}

#[test]
fn a_button_reading_crosses_the_press_threshold() {
    assert!(is_button_down(1.0), "a fully pressed button is down");
    assert!(is_button_down(0.5), "so is one exactly at the threshold");
    assert!(!is_button_down(0.4), "and one just below it is not");
    assert!(!is_button_down(0.0), "a resting button is not");
}

#[test]
fn held_buttons_are_derived_from_this_frames_readings_alone() {
    let values: Vec<f64> = vec![0.0, 1.0, 0.0, 0.9];
    let held: GamepadButtonSet = compute_held_buttons(&values);
    assert_eq!(
        held.len(),
        2,
        "two readings crossed the threshold, got {held:?}"
    );
    assert!(held.contains(&1), "button 1 is held");
    assert!(held.contains(&3), "button 3 is held");
    assert!(!held.contains(&0), "a resting button is not held");
    assert!(!held.contains(&2), "and neither is a released one");
}

#[test]
fn pressed_is_the_difference_between_this_frame_and_the_last() {
    let values: Vec<f64> = vec![0.0, 1.0, 0.0];
    let mut previous: GamepadButtonSet = GamepadButtonSet::new();
    previous.insert(2);
    previous.insert(1);
    let pressed: GamepadButtonSet = compute_pressed_buttons(&previous, &values);
    assert!(pressed.is_empty(), "nothing newly crossed the threshold");
    let mut fresh: GamepadButtonSet = GamepadButtonSet::new();
    fresh.insert(2);
    let newly: GamepadButtonSet = compute_pressed_buttons(&fresh, &values);
    assert!(
        newly.contains(&1),
        "button 1 was up last frame and is down now"
    );
    assert!(
        !newly.contains(&2),
        "button 2 was already down, so it is not a new press"
    );
}

#[test]
fn released_is_the_mirror_of_pressed() {
    let values: Vec<f64> = vec![0.0, 0.0, 0.0];
    let mut previous: GamepadButtonSet = GamepadButtonSet::new();
    previous.insert(0);
    previous.insert(5);
    let released: GamepadButtonSet = compute_released_buttons(&previous, &values);
    assert_eq!(
        released.len(),
        2,
        "both previously-held buttons let go, got {released:?}"
    );
    assert!(
        released.contains(&0) && released.contains(&5),
        "both are reported"
    );
    let values_still_held: Vec<f64> = vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    let none: GamepadButtonSet = compute_released_buttons(&previous, &values_still_held);
    assert!(none.is_empty(), "a button still down is not a release");
}

#[test]
fn the_three_edge_sets_are_mutually_consistent_for_one_frame() {
    let first_frame: Vec<f64> = vec![0.0, 1.0];
    let mut previous: GamepadButtonSet = GamepadButtonSet::new();
    let pressed: GamepadButtonSet = compute_pressed_buttons(&previous, &first_frame);
    let held: GamepadButtonSet = compute_held_buttons(&first_frame);
    let released: GamepadButtonSet = compute_released_buttons(&previous, &first_frame);
    assert!(
        pressed.contains(&1) && held.contains(&1),
        "a fresh press is also held"
    );
    assert!(
        released.is_empty(),
        "and nothing can be released on the first frame"
    );
    previous = held;
    let second_frame: Vec<f64> = vec![0.0, 0.0];
    let pressed_again: GamepadButtonSet = compute_pressed_buttons(&previous, &second_frame);
    let released_again: GamepadButtonSet = compute_released_buttons(&previous, &second_frame);
    assert!(
        pressed_again.is_empty(),
        "a held-then-still-held button is not re-pressed"
    );
    assert!(released_again.contains(&1), "but letting go is a release");
}

#[test]
fn reading_a_raw_axis_past_the_end_of_the_list_is_zero_not_a_panic() {
    let axes: Vec<f64> = vec![0.0, 1.0, -1.0];
    assert!(
        epsilon(read_raw_axis(&axes, 0), 0.0),
        "index zero reads the first entry"
    );
    assert!(
        epsilon(read_raw_axis(&axes, 1), 1.0),
        "index one reads the second"
    );
    assert!(
        epsilon(read_raw_axis(&axes, 9), 0.0),
        "an index past the end reads zero"
    );
    assert!(
        epsilon(read_raw_axis(&[], 0), 0.0),
        "and an empty list reads zero rather than panicking"
    );
}

#[test]
fn a_pad_with_no_readings_reports_nothing_down() {
    let state: GamepadState = GamepadState::default();
    assert!(
        epsilon(state.button_value(0), 0.0),
        "a pad with no readings reports zero, not a panic"
    );
    assert!(!state.is_button_pressed(0), "and nothing is pressed");
    assert!(!state.is_button_held(0), "nor held");
    assert!(!state.is_button_released(0), "nor released");
    assert_eq!(
        state.button_action(0),
        InputAction::Idle,
        "a button with no edges and no hold resolves to Idle"
    );
    assert!(epsilon(state.axis(0), 0.0), "and every axis reads zero");
    assert!(
        !state.axis_pressed(0, 0.1),
        "so no axis can read as pressed at a real threshold"
    );
}

#[test]
fn the_input_action_enum_covers_the_four_states_a_button_can_be_in() {
    let states: Vec<InputAction> = vec![
        InputAction::Pressed,
        InputAction::Held,
        InputAction::Released,
        InputAction::Idle,
    ];
    assert_eq!(states.len(), 4, "a button has exactly four states");
    for (index, state) in states.iter().enumerate() {
        for other in states.iter().skip(index + 1) {
            assert_ne!(state, other, "the states must be distinguishable");
        }
    }
}
