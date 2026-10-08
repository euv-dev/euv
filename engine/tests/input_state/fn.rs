use super::*;

fn state() -> InputState {
    InputState::default()
}

#[test]
fn a_fresh_input_state_reports_nothing() {
    let input: InputState = state();
    assert!(
        !input.is_key_pressed("KeyA"),
        "nothing is pressed on the first frame"
    );
    assert!(!input.is_key_held("KeyA"), "nothing is held either");
    assert!(!input.is_key_released("KeyA"), "and nothing was released");
}

#[test]
fn a_fresh_input_state_has_no_mouse_buttons() {
    let input: InputState = state();
    assert!(
        !input.is_mouse_button_pressed(MouseButton::Left),
        "no button starts pressed"
    );
    assert!(
        !input.is_mouse_button_held(MouseButton::Left),
        "and none starts held"
    );
}

#[test]
fn a_newly_pressed_key_is_both_pressed_and_held() {
    let mut input: InputState = state();
    input.press_key(String::from("KeyA"));
    assert!(
        input.is_key_pressed("KeyA"),
        "the first frame is the press edge"
    );
    assert!(input.is_key_held("KeyA"), "and the key is down");
    assert!(!input.is_key_released("KeyA"), "a press is not a release");
}

#[test]
fn a_key_pressed_again_while_held_keeps_the_same_single_edge() {
    let mut input: InputState = state();
    input.press_key(String::from("KeyA"));
    input.press_key(String::from("KeyA"));
    assert!(
        input.is_key_pressed("KeyA"),
        "the edge from the first press is still in force for this frame"
    );
    assert!(input.is_key_held("KeyA"), "and the key is down");
    input.end_frame();
    assert!(
        !input.is_key_pressed("KeyA"),
        "the second press must not have queued a second edge for the next frame"
    );
}

#[test]
fn releasing_a_key_reports_the_release_edge_and_clears_held() {
    let mut input: InputState = state();
    input.press_key(String::from("KeyA"));
    input.release_key(String::from("KeyA"));
    assert!(
        input.is_key_released("KeyA"),
        "the release is an edge of its own"
    );
    assert!(!input.is_key_held("KeyA"), "and the key is no longer down");
}

#[test]
fn a_released_key_reports_both_edges_in_the_same_frame() {
    let mut input: InputState = state();
    input.press_key(String::from("KeyA"));
    input.release_key(String::from("KeyA"));
    assert!(
        input.is_key_pressed("KeyA") && input.is_key_released("KeyA"),
        "a key tapped and let go inside one frame shows both edges"
    );
}

#[test]
fn releasing_a_key_that_was_never_held_still_reports_a_release() {
    let mut input: InputState = state();
    input.release_key(String::from("KeyZ"));
    assert!(
        input.is_key_released("KeyZ"),
        "a release is reported even without a press"
    );
    assert!(!input.is_key_held("KeyZ"), "but the key is not held");
}

#[test]
fn keys_are_tracked_independently() {
    let mut input: InputState = state();
    input.press_key(String::from("KeyA"));
    input.press_key(String::from("KeyB"));
    input.release_key(String::from("KeyA"));
    assert!(!input.is_key_held("KeyA"), "the released key comes up");
    assert!(input.is_key_held("KeyB"), "while the other stays down");
    assert!(
        input.is_key_released("KeyA"),
        "and only the released one is an edge"
    );
    assert!(!input.is_key_released("KeyB"), "the held one is not");
}

#[test]
fn end_frame_clears_the_key_edges_but_keeps_what_is_held() {
    let mut input: InputState = state();
    input.press_key(String::from("KeyA"));
    input.press_key(String::from("KeyB"));
    input.release_key(String::from("KeyB"));
    input.end_frame();
    assert!(
        !input.is_key_pressed("KeyA"),
        "press edges do not survive a frame"
    );
    assert!(
        !input.is_key_released("KeyB"),
        "release edges do not either"
    );
    assert!(
        input.is_key_held("KeyA"),
        "but a key still physically down must stay held"
    );
    assert!(
        !input.is_key_held("KeyB"),
        "while a released one does not come back"
    );
}

#[test]
fn a_press_after_a_release_and_a_frame_boundary_is_a_fresh_edge() {
    let mut input: InputState = state();
    input.press_key(String::from("KeyA"));
    input.end_frame();
    input.release_key(String::from("KeyA"));
    input.end_frame();
    input.press_key(String::from("KeyA"));
    assert!(
        input.is_key_pressed("KeyA"),
        "once the key is genuinely up, pressing it again is a real press"
    );
}

#[test]
fn a_key_still_down_across_frames_never_re_arms_its_edge() {
    let mut input: InputState = state();
    input.press_key(String::from("KeyA"));
    for _ in 0..5 {
        input.end_frame();
        input.press_key(String::from("KeyA"));
    }
    assert!(
        !input.is_key_pressed("KeyA"),
        "a key held down for many frames must not fire an edge every frame"
    );
}

#[test]
fn a_pressed_mouse_button_is_both_pressed_and_held() {
    let mut input: InputState = state();
    input.press_mouse_button(MouseButton::Left, Vector2D::new(10.0, 20.0));
    assert!(
        input.is_mouse_button_pressed(MouseButton::Left),
        "the first frame is the press edge"
    );
    assert!(
        input.is_mouse_button_held(MouseButton::Left),
        "and the button is down"
    );
}

#[test]
fn a_mouse_button_pressed_again_while_held_keeps_the_same_single_edge() {
    let mut input: InputState = state();
    input.press_mouse_button(MouseButton::Left, Vector2D::zero());
    input.press_mouse_button(MouseButton::Left, Vector2D::zero());
    assert!(
        input.is_mouse_button_pressed(MouseButton::Left),
        "the edge from the first press still stands for this frame"
    );
    assert!(
        input.is_mouse_button_held(MouseButton::Left),
        "and the button is down"
    );
    input.end_frame();
    assert!(
        !input.is_mouse_button_pressed(MouseButton::Left),
        "a repeat must not queue a second edge"
    );
}

#[test]
fn releasing_a_mouse_button_clears_held_but_the_press_edge_survives_the_frame() {
    let mut input: InputState = state();
    input.press_mouse_button(MouseButton::Left, Vector2D::zero());
    input.release_mouse_button(MouseButton::Left);
    assert!(
        !input.is_mouse_button_held(MouseButton::Left),
        "the button comes up at once"
    );
    assert!(
        input.is_mouse_button_pressed(MouseButton::Left),
        "the press edge is still in force for the rest of the frame"
    );
    input.end_frame();
    assert!(
        !input.is_mouse_button_pressed(MouseButton::Left),
        "and then it clears"
    );
}

#[test]
fn mouse_buttons_are_tracked_independently() {
    let mut input: InputState = state();
    input.press_mouse_button(MouseButton::Left, Vector2D::zero());
    input.press_mouse_button(MouseButton::Right, Vector2D::zero());
    input.release_mouse_button(MouseButton::Left);
    assert!(
        !input.is_mouse_button_held(MouseButton::Left),
        "the released one is up"
    );
    assert!(
        input.is_mouse_button_held(MouseButton::Right),
        "the other stays down"
    );
}

#[test]
fn a_press_records_where_the_mouse_was() {
    let mut input: InputState = state();
    input.press_mouse_button(MouseButton::Left, Vector2D::new(42.0, 24.0));
    let observed: Vector2D = input.get_mouse_position();
    assert_eq!(
        observed,
        Vector2D::new(42.0, 24.0),
        "the press position becomes the current mouse position"
    );
}

#[test]
fn moving_the_mouse_updates_the_position() {
    let mut input: InputState = state();
    input.update_mouse_position(Vector2D::new(5.0, 6.0));
    let observed: Vector2D = input.get_mouse_position();
    assert_eq!(
        observed,
        Vector2D::new(5.0, 6.0),
        "the new position is recorded"
    );
}

#[test]
fn end_frame_resets_the_mouse_delta() {
    let mut input: InputState = state();
    input.update_mouse_position(Vector2D::new(10.0, 10.0));
    input.update_mouse_position(Vector2D::new(20.0, 10.0));
    input.end_frame();
    let observed: Vector2D = input.get_mouse_delta();
    assert_eq!(
        observed,
        Vector2D::zero(),
        "the delta is a per-frame quantity"
    );
}

#[test]
fn end_frame_clears_the_mouse_button_edges() {
    let mut input: InputState = state();
    input.press_mouse_button(MouseButton::Left, Vector2D::zero());
    input.end_frame();
    assert!(
        !input.is_mouse_button_pressed(MouseButton::Left),
        "a button edge does not survive the frame it happened in"
    );
    assert!(
        input.is_mouse_button_held(MouseButton::Left),
        "while a still-down button remains held"
    );
}

#[test]
fn there_is_no_primary_touch_before_any_touch_starts() {
    let input: InputState = state();
    assert_eq!(
        input.primary_touch_position(),
        None,
        "a finger has to land before there is a position to report"
    );
}

#[test]
fn a_started_touch_reports_its_position() {
    let mut input: InputState = state();
    input.start_touch(0, Vector2D::new(100.0, 200.0));
    let observed: Option<Vector2D> = input.primary_touch_position();
    assert_eq!(
        observed,
        Some(Vector2D::new(100.0, 200.0)),
        "the position comes back"
    );
}

#[test]
fn the_lowest_identifier_is_the_primary_touch() {
    let mut input: InputState = state();
    input.start_touch(2, Vector2D::new(0.0, 0.0));
    input.start_touch(0, Vector2D::new(7.0, 8.0));
    input.start_touch(1, Vector2D::new(3.0, 4.0));
    let observed: Option<Vector2D> = input.primary_touch_position();
    assert_eq!(
        observed,
        Some(Vector2D::new(7.0, 8.0)),
        "identifier order decides which finger is primary, not insertion order"
    );
}

#[test]
fn a_moved_touch_keeps_its_identity() {
    let mut input: InputState = state();
    input.start_touch(0, Vector2D::new(1.0, 1.0));
    input.update_touch(0, Vector2D::new(9.0, 9.0));
    let observed: Option<Vector2D> = input.primary_touch_position();
    assert_eq!(
        observed,
        Some(Vector2D::new(9.0, 9.0)),
        "a moving finger keeps its identifier, so it stays primary"
    );
}

#[test]
fn ending_the_primary_touch_promotes_the_next_one() {
    let mut input: InputState = state();
    input.start_touch(0, Vector2D::new(1.0, 1.0));
    input.start_touch(1, Vector2D::new(5.0, 5.0));
    input.end_touch(0);
    let observed: Option<Vector2D> = input.primary_touch_position();
    assert_eq!(
        observed,
        Some(Vector2D::new(5.0, 5.0)),
        "once the lowest identifier lifts, the next one is primary"
    );
}

#[test]
fn ending_every_touch_leaves_no_primary() {
    let mut input: InputState = state();
    input.start_touch(0, Vector2D::new(1.0, 1.0));
    input.end_touch(0);
    let observed: Option<Vector2D> = input.primary_touch_position();
    assert_eq!(observed, None, "no fingers means no primary touch");
}

#[test]
fn ending_a_touch_that_never_started_is_harmless() {
    let mut input: InputState = state();
    input.end_touch(9);
    let observed: Option<Vector2D> = input.primary_touch_position();
    assert_eq!(observed, None, "lifting an unknown finger changes nothing");
}

#[test]
fn end_frame_clears_the_touch_edges() {
    let mut input: InputState = state();
    input.start_touch(0, Vector2D::zero());
    input.end_frame();
    input.start_touch(0, Vector2D::zero());
    input.end_frame();
    let observed: Option<Vector2D> = input.primary_touch_position();
    assert_eq!(
        observed,
        Some(Vector2D::zero()),
        "a touch that is still down survives the frame boundary"
    );
}

#[test]
fn the_default_mouse_button_is_the_left_one() {
    let observed: MouseButton = MouseButton::default();
    assert_eq!(observed, MouseButton::Left, "button zero is the default");
}
