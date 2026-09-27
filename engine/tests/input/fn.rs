use super::*;

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
    scheduler.tick(&config, &handler, Some(&cell));
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
    scheduler.tick(&config, &handler, Some(&cell));
    scheduler.tick(&config, &handler, Some(&cell));
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
    scheduler.tick(&config, &handler, None);
    scheduler.tick(&config, &handler, None);
    assert!(updates.get() >= 1);
    assert_eq!(renders.get(), 2);
    assert!(scheduler.get_frame_count() >= 2);
    let _: u64 = scheduler.get_update_count();
}
