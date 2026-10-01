use super::*;

/// Implements static event extraction methods on the `Input` namespace struct.
impl Input {
    /// Extracts the key code string from a keyboard event.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The keyboard event.
    ///
    /// # Returns
    ///
    /// - `String` - The key code string (e.g., `"KeyA"`, `"Space"`, `"ArrowLeft"`).
    pub fn extract_key_code(event: &Event) -> String {
        // OPT 39: typed web-sys getter avoids the JS Reflect::get crossing.
        event.unchecked_ref::<KeyboardEvent>().code()
    }

    /// Extracts the mouse button enum from a mouse event.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The mouse event.
    ///
    /// # Returns
    ///
    /// - `MouseButton` - The mouse button that was pressed or released.
    pub fn extract_mouse_button(event: &Event) -> MouseButton {
        // OPT 39: typed web-sys `MouseEvent::button` getter instead of
        // Reflect::get + as_f64 cast.
        let button_value: i16 = event.unchecked_ref::<MouseEvent>().button();
        match button_value {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            3 => MouseButton::Button4,
            4 => MouseButton::Button5,
            _ => MouseButton::Left,
        }
    }

    /// Extracts the client (viewport) coordinates from a mouse event.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The mouse event.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The `(x, y)` client coordinates.
    pub fn extract_mouse_position(event: &Event) -> Vector2D {
        // OPT 39: typed web-sys `MouseEvent::client_x` / `client_y` getters
        // instead of two Reflect::get + as_f64 casts per mouse event.
        let mouse_event: &MouseEvent = event.unchecked_ref::<MouseEvent>();
        let client_x: f64 = f64::from(mouse_event.client_x());
        let client_y: f64 = f64::from(mouse_event.client_y());
        Vector2D::new(client_x, client_y)
    }
}

/// Implements DOM event listener registration on the `Input` namespace struct.
///
/// This is the wiring layer that routes DOM events into [`InputState`]:
/// keyboard events bind to `window` (a `<canvas>` is not focusable by
/// default), while mouse and touch events bind to the canvas element so
/// hit-testing coordinates stay canvas-local. Registered closures are
/// `.forget()`-ed and stay alive for the lifetime of the document,
/// matching the engine's mount-only convention.
impl Input {
    /// Attaches all input listeners and returns the shared state cell.
    ///
    /// # Arguments
    ///
    /// - `InputStateCell` - The shared input state to mutate from event handlers.
    /// - `&Window` - The global window, receiving keyboard events.
    /// - `&EventTarget` - The pointer target (typically the canvas element).
    ///
    /// # Returns
    ///
    /// - `InputStateCell` - The same cell passed in, for convenient chaining.
    pub fn attach(
        state_cell: InputStateCell,
        window: &Window,
        pointer_target: &EventTarget,
    ) -> InputStateCell {
        Self::attach_keyboard(&state_cell, window);
        Self::attach_pointer(&state_cell, pointer_target);
        state_cell
    }

    /// Binds `keydown` / `keyup` listeners to `window`.
    ///
    /// Keyboard events must bind to `window` rather than the canvas: a
    /// `<canvas>` element is not focusable unless `tabindex` is set and the
    /// user clicks it, so canvas-bound key listeners would never fire.
    ///
    /// # Arguments
    ///
    /// - `&InputStateCell` - The shared input state.
    /// - `&Window` - The global window.
    pub fn attach_keyboard(state_cell: &InputStateCell, window: &Window) {
        let state_keydown: InputStateCell = state_cell.clone();
        let keydown_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let code: String = Input::extract_key_code(&event);
                if code.is_empty() {
                    return;
                }
                let state: &mut InputState = state_keydown.get_mut();
                state.press_key(code);
            }));
        Self::register_listener(window, INPUT_EVENT_KEYDOWN, keydown_closure);
        let state_keyup: InputStateCell = state_cell.clone();
        let keyup_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let code: String = Input::extract_key_code(&event);
                if code.is_empty() {
                    return;
                }
                let state: &mut InputState = state_keyup.get_mut();
                state.release_key(code);
            }));
        Self::register_listener(window, INPUT_EVENT_KEYUP, keyup_closure);
    }

    /// Binds mouse / touch / context-menu listeners to the pointer target.
    ///
    /// `touchstart` and `touchmove` call `prevent_default()` so the browser
    /// does not interpret touches as scroll/zoom gestures before the engine
    /// sees them. `contextmenu` is suppressed so right-click reaches the
    /// engine as `MouseButton::Right` instead of opening the browser menu.
    ///
    /// # Arguments
    ///
    /// - `&InputStateCell` - The shared input state.
    /// - `&EventTarget` - The pointer target (typically the canvas element).
    pub fn attach_pointer(state_cell: &InputStateCell, target: &EventTarget) {
        let state_mousedown: InputStateCell = state_cell.clone();
        let mousedown_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let button: MouseButton = Input::extract_mouse_button(&event);
                let position: Vector2D = Input::extract_mouse_position(&event);
                let state: &mut InputState = state_mousedown.get_mut();
                state.press_mouse_button(button, position);
            }));
        Self::register_listener(target, INPUT_EVENT_MOUSEDOWN, mousedown_closure);
        let state_mouseup: InputStateCell = state_cell.clone();
        let mouseup_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let button: MouseButton = Input::extract_mouse_button(&event);
                let state: &mut InputState = state_mouseup.get_mut();
                state.release_mouse_button(button);
            }));
        Self::register_listener(target, INPUT_EVENT_MOUSEUP, mouseup_closure);
        let state_mousemove: InputStateCell = state_cell.clone();
        let mousemove_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let position: Vector2D = Input::extract_mouse_position(&event);
                let state: &mut InputState = state_mousemove.get_mut();
                state.update_mouse_position(position);
            }));
        Self::register_listener(target, INPUT_EVENT_MOUSEMOVE, mousemove_closure);
        let state_mouseleave: InputStateCell = state_cell.clone();
        let mouseleave_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |_: Event| {
                let state: &mut InputState = state_mouseleave.get_mut();
                state.set_mouse_moved(false);
                state.set_mouse_delta(Vector2D::zero());
            }));
        Self::register_listener(target, INPUT_EVENT_MOUSELEAVE, mouseleave_closure);
        let state_touchstart: InputStateCell = state_cell.clone();
        let touchstart_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                event.prevent_default();
                let state: &mut InputState = state_touchstart.get_mut();
                for (identifier, position) in Input::extract_touch_positions(&event) {
                    state.start_touch(identifier, position);
                }
            }));
        Self::register_listener(target, INPUT_EVENT_TOUCHSTART, touchstart_closure);
        let state_touchmove: InputStateCell = state_cell.clone();
        let touchmove_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                event.prevent_default();
                let state: &mut InputState = state_touchmove.get_mut();
                for (identifier, position) in Input::extract_touch_positions(&event) {
                    state.update_touch(identifier, position);
                }
            }));
        Self::register_listener(target, INPUT_EVENT_TOUCHMOVE, touchmove_closure);
        let state_touchend: InputStateCell = state_cell.clone();
        let touchend_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let state: &mut InputState = state_touchend.get_mut();
                for identifier in Input::extract_touch_identifiers(&event) {
                    state.end_touch(identifier);
                }
            }));
        Self::register_listener(target, INPUT_EVENT_TOUCHEND, touchend_closure);
        let contextmenu_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                event.prevent_default();
            }));
        Self::register_listener(target, INPUT_EVENT_CONTEXTMENU, contextmenu_closure);
    }

    /// Extracts `(identifier, position)` pairs for every changed touch.
    ///
    /// A single touch event can carry multiple changed touches, so this
    /// iterates the whole `changedTouches` list rather than reading index 0.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<(i32, Vector2D)>` - The identifier and client position of each changed touch.
    fn extract_touch_positions(event: &Event) -> Vec<(i32, Vector2D)> {
        let touch_event: &TouchEvent = event.unchecked_ref();
        let touches: TouchList = touch_event.changed_touches();
        let length: u32 = touches.length();
        let mut out: Vec<(i32, Vector2D)> = Vec::with_capacity(length as usize);
        for index in 0..length {
            let Some(touch) = touches.get(index) else {
                continue;
            };
            let identifier: i32 = touch.identifier();
            let position: Vector2D =
                Vector2D::new(f64::from(touch.client_x()), f64::from(touch.client_y()));
            out.push((identifier, position));
        }
        out
    }

    /// Extracts the identifier of every changed touch (for `touchend`).
    ///
    /// # Arguments
    ///
    /// - `&Event` - The touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<i32>` - The identifier of each changed touch.
    fn extract_touch_identifiers(event: &Event) -> Vec<i32> {
        let touch_event: &TouchEvent = event.unchecked_ref();
        let touches: TouchList = touch_event.changed_touches();
        let length: u32 = touches.length();
        let mut out: Vec<i32> = Vec::with_capacity(length as usize);
        for index in 0..length {
            let Some(touch) = touches.get(index) else {
                continue;
            };
            out.push(touch.identifier());
        }
        out
    }

    /// Registers a closure on the target and leaks it for the document's lifetime.
    ///
    /// The engine follows the wasm single-page-mount convention: listeners
    /// are never detached, so the closure is `.forget()`-ed immediately
    /// after registration (its clone of the state cell keeps the cell
    /// reachable through the listeners even if the caller drops its `Rc`).
    ///
    /// # Arguments
    ///
    /// - `&EventTarget` - The DOM target to listen on.
    /// - `&str` - The DOM event name.
    /// - `Closure<dyn FnMut(Event)>` - The handler to register.
    fn register_listener(
        target: &EventTarget,
        event_name: &str,
        closure: Closure<dyn FnMut(Event)>,
    ) {
        let _: Result<(), JsValue> =
            target.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

/// Implements input state management for `InputState`.
impl InputState {
    /// Records a key press event, adding to `keys_pressed` and `keys_held`.
    ///
    /// # Arguments
    ///
    /// - `String` - The key code string (e.g., `"KeyA"`, `"Space"`).
    pub fn press_key(&mut self, key_code: String) {
        if !self.get_keys_held().contains(&key_code) {
            self.get_mut_keys_pressed().insert(key_code.clone());
        }
        self.get_mut_keys_held().insert(key_code);
    }

    /// Records a key release event, moving from `keys_held` to `keys_released`.
    ///
    /// # Arguments
    ///
    /// - `String` - The key code string.
    pub fn release_key(&mut self, key_code: String) {
        self.get_mut_keys_held().remove(&key_code);
        self.get_mut_keys_released().insert(key_code);
    }

    /// Tests whether a key was pressed during this frame.
    ///
    /// # Arguments
    ///
    /// - `K` - The key code string.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the key was pressed this frame.
    pub fn is_key_pressed<K>(&self, key_code: K) -> bool
    where
        K: AsRef<str>,
    {
        self.get_keys_pressed().contains(key_code.as_ref())
    }

    /// Tests whether a key is currently held down.
    ///
    /// # Arguments
    ///
    /// - `K` - The key code string.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the key is held.
    pub fn is_key_held<K>(&self, key_code: K) -> bool
    where
        K: AsRef<str>,
    {
        self.get_keys_held().contains(key_code.as_ref())
    }

    /// Tests whether a key was released during this frame.
    ///
    /// # Arguments
    ///
    /// - `K` - The key code string.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the key was released this frame.
    pub fn is_key_released<K>(&self, key_code: K) -> bool
    where
        K: AsRef<str>,
    {
        self.get_keys_released().contains(key_code.as_ref())
    }

    /// Records a mouse button press at the given position.
    ///
    /// # Arguments
    ///
    /// - `MouseButton` - The button that was pressed.
    /// - `Vector2D` - The mouse position.
    pub fn press_mouse_button(&mut self, button: MouseButton, position: Vector2D) {
        if !self.get_mouse_buttons_held().contains(&button) {
            self.get_mut_mouse_buttons_pressed().insert(button);
        }
        self.get_mut_mouse_buttons_held().insert(button);
        self.set_mouse_position(position);
    }

    /// Records a mouse button release.
    ///
    /// # Arguments
    ///
    /// - `MouseButton` - The button that was released.
    pub fn release_mouse_button(&mut self, button: MouseButton) {
        self.get_mut_mouse_buttons_held().remove(&button);
        self.get_mut_mouse_buttons_released().insert(button);
    }

    /// Updates the mouse position and computes the delta from the previous position.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The new mouse position.
    pub fn update_mouse_position(&mut self, position: Vector2D) {
        self.set_mouse_delta(position - self.get_mouse_position());
        self.set_mouse_position(position);
        self.set_mouse_moved(true);
    }

    /// Tests whether a mouse button was pressed during this frame.
    ///
    /// # Arguments
    ///
    /// - `MouseButton` - The button to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the button was pressed this frame.
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.get_mouse_buttons_pressed().contains(&button)
    }

    /// Tests whether a mouse button is currently held down.
    ///
    /// # Arguments
    ///
    /// - `MouseButton` - The button to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the button is held.
    pub fn is_mouse_button_held(&self, button: MouseButton) -> bool {
        self.get_mouse_buttons_held().contains(&button)
    }

    /// Adds or updates a touch point.
    ///
    /// # Arguments
    ///
    /// - `i32` - The touch identifier.
    /// - `Vector2D` - The touch position.
    pub fn update_touch(&mut self, identifier: i32, position: Vector2D) {
        self.get_mut_touch_points().insert(identifier, position);
    }

    /// Records a new touch point that started this frame.
    ///
    /// # Arguments
    ///
    /// - `i32` - The touch identifier.
    /// - `Vector2D` - The touch position.
    pub fn start_touch(&mut self, identifier: i32, position: Vector2D) {
        self.get_mut_touch_points().insert(identifier, position);
        self.get_mut_touch_started().insert(identifier);
    }

    /// Removes a touch point and marks it as ended this frame.
    ///
    /// # Arguments
    ///
    /// - `i32` - The touch identifier.
    pub fn end_touch(&mut self, identifier: i32) {
        self.get_mut_touch_points().remove(&identifier);
        self.get_mut_touch_ended().insert(identifier);
    }

    /// Returns the position of the lowest-identifier active touch point.
    ///
    /// Pointer-style consumers (the example pages' interactive demos) treat
    /// the primary touch like a mouse cursor: touch events never update
    /// `mouse_position`, so this accessor is the only public way to read a
    /// touch position.
    ///
    /// # Returns
    ///
    /// - `Option<Vector2D>` - The client-space position of the primary
    ///   touch, or `None` when no touch is active.
    pub fn primary_touch_position(&self) -> Option<Vector2D> {
        self.get_touch_points()
            .iter()
            .min_by_key(|(identifier, _): &(&i32, &Vector2D)| **identifier)
            .map(|(_, position): (&i32, &Vector2D)| *position)
    }

    /// Clears all per-frame input data (pressed, released, deltas).
    ///
    /// Should be called at the end of each game frame after all input has been processed.
    pub fn end_frame(&mut self) {
        self.get_mut_keys_pressed().clear();
        self.get_mut_keys_released().clear();
        self.get_mut_mouse_buttons_pressed().clear();
        self.get_mut_mouse_buttons_released().clear();
        self.set_mouse_delta(Vector2D::zero());
        self.set_mouse_moved(false);
        self.get_mut_touch_started().clear();
        self.get_mut_touch_ended().clear();
    }
}

/// Implements `Default` for `InputState` as a fresh empty state.
impl Default for InputState {
    /// Constructs a default [`InputState`] value.
    ///
    /// # Returns
    ///
    /// - `InputState` - A default-constructed instance with the documented initial state.
    fn default() -> InputState {
        InputState::new()
    }
}

/// Implements the per-pad query surface on `GamepadState`.
impl GamepadState {
    /// Reads one analog axis with the deadzone applied.
    ///
    /// # Arguments
    ///
    /// - `u32` - The axis index to read.
    ///
    /// # Returns
    ///
    /// - `f64` - The deadzone-corrected reading, or `0.0` when the pad
    ///   has no such axis.
    pub fn axis(&self, axis: u32) -> f64 {
        apply_axis_deadzone(read_raw_axis(self.get_axes(), axis))
    }

    /// Reads one button's raw pressure without any deadzone.
    ///
    /// # Arguments
    ///
    /// - `u32` - The button index to read.
    ///
    /// # Returns
    ///
    /// - `f64` - The pressure in `[0.0, 1.0]`, or `0.0` when the pad
    ///   has no such button.
    pub fn button_value(&self, button: u32) -> f64 {
        read_raw_axis(self.get_button_values(), button)
    }

    /// Tests whether a button went down on this frame.
    ///
    /// # Arguments
    ///
    /// - `u32` - The button index to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the button was pressed this frame.
    pub fn is_button_pressed(&self, button: u32) -> bool {
        self.get_buttons_pressed().contains(&button)
    }

    /// Tests whether a button is currently held down.
    ///
    /// # Arguments
    ///
    /// - `u32` - The button index to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the button is held.
    pub fn is_button_held(&self, button: u32) -> bool {
        self.get_buttons_held().contains(&button)
    }

    /// Tests whether a button came up on this frame.
    ///
    /// # Arguments
    ///
    /// - `u32` - The button index to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the button was released this frame.
    pub fn is_button_released(&self, button: u32) -> bool {
        self.get_buttons_released().contains(&button)
    }

    /// Resolves a button's current state into the enum form.
    ///
    /// # Arguments
    ///
    /// - `u32` - The button index to check.
    ///
    /// # Returns
    ///
    /// - `InputAction` - The edge state of the button, `Pressed` taking
    ///   precedence over `Released` when a connect and a release land on
    ///   the same frame.
    pub fn button_action(&self, button: u32) -> InputAction {
        if self.is_button_pressed(button) {
            return InputAction::Pressed;
        }
        if self.is_button_released(button) {
            return InputAction::Released;
        }
        if self.is_button_held(button) {
            return InputAction::Held;
        }
        InputAction::Idle
    }

    /// Tests whether a direction pair reads as a positive deflection.
    ///
    /// # Arguments
    ///
    /// - `u32` - The index of the axis to read.
    /// - `f64` - The deflection that counts as a press.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the corrected axis reading reaches
    ///   `threshold`.
    pub fn axis_pressed(&self, axis: u32, threshold: f64) -> bool {
        self.axis(axis) >= threshold
    }

    /// Clears this frame's per-frame data, keeping the held set.
    ///
    /// Mirrors [`InputState::end_frame`]: the pressed and released
    /// sets are per-frame and are dropped, while the held set survives
    /// so a button that is still down stays held on the next frame.
    pub fn end_frame(&mut self) {
        self.get_mut_buttons_pressed().clear();
        self.get_mut_buttons_released().clear();
    }
}

/// Implements the manager query surface on `GamepadManager`.
impl GamepadManager {
    /// Looks up the state of one gamepad by index.
    ///
    /// # Arguments
    ///
    /// - `u32` - The browser-assigned gamepad index.
    ///
    /// # Returns
    ///
    /// - `Option<&GamepadState>` - The stored state, or `None` when
    ///   this index has never been seen.
    pub fn state(&self, index: u32) -> Option<&GamepadState> {
        self.get_states().get(&index)
    }

    /// Looks up the state of one gamepad by index, mutably.
    ///
    /// # Arguments
    ///
    /// - `u32` - The browser-assigned gamepad index.
    ///
    /// # Returns
    ///
    /// - `Option<&mut GamepadState>` - The stored state, or `None` when
    ///   this index has never been seen.
    pub fn state_mut(&mut self, index: u32) -> Option<&mut GamepadState> {
        self.get_mut_states().get_mut(&index)
    }

    /// Tests whether a gamepad index is currently connected.
    ///
    /// # Arguments
    ///
    /// - `u32` - The browser-assigned gamepad index.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the pad is connected.
    pub fn is_connected(&self, index: u32) -> bool {
        self.state(index)
            .map(|pad: &GamepadState| pad.get_connected())
            == Some(true)
    }

    /// Reads one analog axis of one gamepad with the deadzone applied.
    ///
    /// # Arguments
    ///
    /// - `u32` - The browser-assigned gamepad index.
    /// - `u32` - The axis index to read.
    ///
    /// # Returns
    ///
    /// - `Option<f64>` - The deadzone-corrected reading, or `None` when
    ///   the index is unknown.
    pub fn axis(&self, index: u32, axis: u32) -> Option<f64> {
        self.state(index).map(|pad: &GamepadState| pad.axis(axis))
    }

    /// Reads one button's raw pressure on one gamepad.
    ///
    /// # Arguments
    ///
    /// - `u32` - The browser-assigned gamepad index.
    /// - `u32` - The button index to read.
    ///
    /// # Returns
    ///
    /// - `Option<f64>` - The pressure, or `None` when the index is
    ///   unknown.
    pub fn button_value(&self, index: u32, button: u32) -> Option<f64> {
        self.state(index)
            .map(|pad: &GamepadState| pad.button_value(button))
    }

    /// Tests whether a button on one gamepad went down on this frame.
    ///
    /// # Arguments
    ///
    /// - `u32` - The browser-assigned gamepad index.
    /// - `u32` - The button index to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the button was pressed this frame, false
    ///   when the index is unknown.
    pub fn is_button_pressed(&self, index: u32, button: u32) -> bool {
        self.state(index)
            .map(|pad: &GamepadState| pad.is_button_pressed(button))
            == Some(true)
    }

    /// Tests whether a button on one gamepad is currently held down.
    ///
    /// # Arguments
    ///
    /// - `u32` - The browser-assigned gamepad index.
    /// - `u32` - The button index to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the button is held, false when the index is
    ///   unknown.
    pub fn is_button_held(&self, index: u32, button: u32) -> bool {
        self.state(index)
            .map(|pad: &GamepadState| pad.is_button_held(button))
            == Some(true)
    }

    /// Tests whether a button on one gamepad came up on this frame.
    ///
    /// # Arguments
    ///
    /// - `u32` - The browser-assigned gamepad index.
    /// - `u32` - The button index to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the button was released this frame, false
    ///   when the index is unknown.
    pub fn is_button_released(&self, index: u32, button: u32) -> bool {
        self.state(index)
            .map(|pad: &GamepadState| pad.is_button_released(button))
            == Some(true)
    }

    /// Resolves a button's current state on one gamepad into enum form.
    ///
    /// # Arguments
    ///
    /// - `u32` - The browser-assigned gamepad index.
    /// - `u32` - The button index to check.
    ///
    /// # Returns
    ///
    /// - `InputAction` - The edge state, `Idle` when the index is
    ///   unknown.
    pub fn button_action(&self, index: u32, button: u32) -> InputAction {
        self.state(index)
            .map(|pad: &GamepadState| pad.button_action(button))
            .unwrap_or_default()
    }

    /// Counts the gamepads that are currently connected.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of connected gamepads.
    pub fn connected_count(&self) -> usize {
        self.get_states()
            .values()
            .filter(|pad: &&GamepadState| pad.get_connected())
            .count()
    }

    /// Clears every pad's per-frame data and the manager's edge sets.
    ///
    /// Mirrors [`InputState::end_frame`]: pressed and released are
    /// per-frame and are dropped, while held sets and the connected
    /// flags survive.
    pub fn end_frame(&mut self) {
        for pad in self.get_mut_states().values_mut() {
            pad.end_frame();
        }
        self.get_mut_connected().clear();
        self.get_mut_disconnected().clear();
    }
}

/// Implements the DOM wiring layer on `GamepadManager`.
///
/// The Gamepad API is a polling API: the browser only mutates
/// `navigator.get_gamepads()` snapshots and never emits per-button
/// events, so button edges must be derived by diffing one frame's
/// snapshot against the last. The two connect / disconnect events are
/// still bound so a device that appears or vanishes between two polls
/// is reflected in `connected` / `disconnected` on the same frame
/// rather than one frame late; a poll alone would also detect both, so
/// the events are a latency optimisation and the poll is the
/// authority.
impl GamepadManager {
    /// Re-reads every gamepad from the DOM and recomputes this
    /// frame's button edges.
    ///
    /// A pad that the browser no longer lists is marked disconnected
    /// and its edge sets flushed, so a yanked controller cannot keep
    /// reporting held buttons forever.
    ///
    /// # Arguments
    ///
    /// - `&Window` - The global window, used to reach `navigator`.
    pub fn poll(&mut self, window: &Window) {
        let pads: Array = match Self::read_gamepads(window) {
            Ok(pads) => pads,
            Err(_) => return,
        };
        let length: u32 = pads.length();
        // Phase 1: fold every live pad into the stored state.
        let mut live: GamepadIndexSet = GamepadIndexSet::new();
        for slot in 0..length {
            let value: JsValue = pads.get(slot);
            if value.is_null() || value.is_undefined() {
                continue;
            }
            let pad: &Gamepad = value.unchecked_ref();
            let index: u32 = pad.index();
            live.insert(index);
            self.sync_pad(index, pad);
        }
        // Phase 2: flush every pad the browser stopped listing.
        let stale: Vec<u32> = self
            .get_states()
            .keys()
            .filter(|index: &&u32| !live.contains(index))
            .copied()
            .collect();
        for index in stale {
            self.release_pad(index);
        }
    }

    /// Attaches the gamepad listeners and returns the shared cell.
    ///
    /// # Arguments
    ///
    /// - `GamepadManagerCell` - The shared manager to mutate.
    /// - `&Window` - The global window, receiving the connect and
    ///   disconnect events.
    ///
    /// # Returns
    ///
    /// - `GamepadManagerCell` - The same cell passed in, for
    ///   convenient chaining.
    pub fn attach(manager_cell: GamepadManagerCell, window: &Window) -> GamepadManagerCell {
        Self::attach_gamepad(&manager_cell, window);
        manager_cell
    }

    /// Binds the connect / disconnect listeners to `window`.
    ///
    /// # Arguments
    ///
    /// - `&GamepadManagerCell` - The shared gamepad manager.
    /// - `&Window` - The global window.
    pub fn attach_gamepad(manager_cell: &GamepadManagerCell, window: &Window) {
        let cell_connected: GamepadManagerCell = manager_cell.clone();
        let connected_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let maybe_pad: Option<Gamepad> = event.unchecked_ref::<GamepadEvent>().gamepad();
                let Some(pad) = maybe_pad else {
                    return;
                };
                let manager: &mut GamepadManager = cell_connected.get_mut();
                manager.sync_pad(pad.index(), &pad);
                manager.get_mut_connected().insert(pad.index());
            }));
        Self::register_listener(window, INPUT_EVENT_GAMEPADCONNECTED, connected_closure);
        let cell_disconnected: GamepadManagerCell = manager_cell.clone();
        let disconnected_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let maybe_pad: Option<Gamepad> = event.unchecked_ref::<GamepadEvent>().gamepad();
                let Some(pad) = maybe_pad else {
                    return;
                };
                let index: u32 = pad.index();
                let manager: &mut GamepadManager = cell_disconnected.get_mut();
                manager.release_pad(index);
            }));
        Self::register_listener(
            window,
            INPUT_EVENT_GAMEPADDISCONNECTED,
            disconnected_closure,
        );
    }

    /// Registers a closure on the target and leaks it for the
    /// document's lifetime.
    ///
    /// # Arguments
    ///
    /// - `&EventTarget` - The DOM target to listen on.
    /// - `&str` - The DOM event name.
    /// - `Closure<dyn FnMut(Event)>` - The handler to register.
    fn register_listener(
        target: &EventTarget,
        event_name: &str,
        closure: Closure<dyn FnMut(Event)>,
    ) {
        let _: Result<(), JsValue> =
            target.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
        closure.forget();
    }

    /// Reads the browser's gamepad snapshot, mapping a thrown error
    /// onto `None` so a sandboxed document degrades to "no gamepads".
    ///
    /// # Arguments
    ///
    /// - `&Window` - The global window, used to reach `navigator`.
    ///
    /// # Returns
    ///
    /// - `Result<Array, JsValue>` - The snapshot, or the thrown value.
    fn read_gamepads(window: &Window) -> Result<Array, JsValue> {
        window.navigator().get_gamepads()
    }

    /// Folds one pad's raw DOM reading into its stored state,
    /// computing this frame's button edges from the previous frame's
    /// held set.
    ///
    /// # Arguments
    ///
    /// - `u32` - The browser-assigned gamepad index.
    /// - `&Gamepad` - The live pad to read.
    fn sync_pad(&mut self, index: u32, pad: &Gamepad) {
        let axes: Vec<f64> = Self::extract_axis_values(&pad.axes());
        let values: Vec<f64> = Self::extract_button_values(&pad.buttons());
        let previous_held: GamepadButtonSet = self
            .state(index)
            .map(|state: &GamepadState| state.get_buttons_held().clone())
            .unwrap_or_default();
        let pressed: GamepadButtonSet = compute_pressed_buttons(&previous_held, &values);
        let released: GamepadButtonSet = compute_released_buttons(&previous_held, &values);
        let held: GamepadButtonSet = compute_held_buttons(&values);
        let id: String = pad.id();
        self.get_mut_states().entry(index).or_default();
        let Some(state) = self.state_mut(index) else {
            return;
        };
        state.set_connected(true);
        state.set_id(id);
        state.set_axes(axes);
        state.set_button_values(values);
        *state.get_mut_buttons_pressed() = pressed;
        *state.get_mut_buttons_released() = released;
        *state.get_mut_buttons_held() = held;
    }

    /// Marks a pad as disconnected and releases every held button.
    ///
    /// A yanked controller stops appearing in the browser snapshot
    /// while its buttons are still physically down, so the held set
    /// is flushed and the flush is reported as a release: that is what
    /// stops a character from running forever after the pad is
    /// unplugged.
    ///
    /// # Arguments
    ///
    /// - `u32` - The browser-assigned gamepad index.
    fn release_pad(&mut self, index: u32) {
        let was_connected: bool = self.is_connected(index);
        let held: GamepadButtonSet = self
            .state(index)
            .map(|state: &GamepadState| state.get_buttons_held().clone())
            .unwrap_or_default();
        if let Some(state) = self.state_mut(index) {
            state.set_connected(false);
            *state.get_mut_buttons_pressed() = GamepadButtonSet::new();
            *state.get_mut_buttons_released() = held;
            state.get_mut_buttons_held().clear();
        }
        if was_connected {
            self.get_mut_disconnected().insert(index);
        }
    }

    /// Extracts the analog readings out of a `Gamepad.axes` list.
    ///
    /// # Arguments
    ///
    /// - `&Array` - The live DOM `axes` list.
    ///
    /// # Returns
    ///
    /// - `Vec<f64>` - The readings, one per axis.
    fn extract_axis_values(axes: &Array) -> Vec<f64> {
        let length: u32 = axes.length();
        let mut out: Vec<f64> = Vec::with_capacity(length as usize);
        for index in 0..length {
            let value: JsValue = axes.get(index);
            out.push(value.as_f64().unwrap_or(0.0));
        }
        out
    }

    /// Extracts the pressures out of a `Gamepad.buttons` list.
    ///
    /// # Arguments
    ///
    /// - `&Array` - The live DOM `buttons` list.
    ///
    /// # Returns
    ///
    /// - `Vec<f64>` - The pressures, one per button.
    fn extract_button_values(buttons: &Array) -> Vec<f64> {
        let length: u32 = buttons.length();
        let mut out: Vec<f64> = Vec::with_capacity(length as usize);
        for index in 0..length {
            let value: JsValue = buttons.get(index);
            if value.is_null() || value.is_undefined() {
                out.push(0.0);
                continue;
            }
            out.push(value.unchecked_ref::<GamepadButton>().value());
        }
        out
    }
}
