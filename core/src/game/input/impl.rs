use super::*;

/// Per-frame input ingestion and real-time polling helpers for `InputState`.
impl InputState {
    /// Clears transient per-frame edge states (`just_pressed`, `just_released`, deltas) after a frame step.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable input state.
    pub fn begin_frame(&mut self) {
        self.get_mut_keys_just_pressed().clear();
        self.get_mut_keys_just_released().clear();
        self.set_pointer_delta(Point::new(0.0, 0.0));
        self.set_scroll_delta(Point::new(0.0, 0.0));
    }

    /// Updates continuous and edge-triggered states from a platform `WindowEvent`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable input state.
    /// - `&WindowEvent` - Incoming OS or test event.
    pub fn ingest(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyDown(_, key) => {
                let code: KeyCode = key.get_code();
                if self.get_mut_keys_down().insert(code) {
                    let _: bool = self.get_mut_keys_just_pressed().insert(code);
                }
            }
            WindowEvent::KeyUp(_, key) => {
                let code: KeyCode = key.get_code();
                if self.get_mut_keys_down().remove(&code) {
                    let _: bool = self.get_mut_keys_just_released().insert(code);
                }
            }
            WindowEvent::PointerMove(_, ptr) => {
                let prev: Point = self.get_pointer_position();
                let next: Point = ptr.get_position();
                let delta: Point = self.get_pointer_delta();
                self.set_pointer_delta(Point::new(
                    delta.get_x() + (next.get_x() - prev.get_x()),
                    delta.get_y() + (next.get_y() - prev.get_y()),
                ));
                self.set_pointer_position(next);
            }
            WindowEvent::PointerDown(_, ptr) => {
                self.set_pointer_position(ptr.get_position());
                let _: bool = self.get_mut_buttons_down().insert(ptr.get_button());
            }
            WindowEvent::PointerUp(_, ptr) => {
                self.set_pointer_position(ptr.get_position());
                let _: bool = self.get_mut_buttons_down().remove(&ptr.get_button());
            }
            WindowEvent::Scroll(_, delta) => {
                let cur: Point = self.get_scroll_delta();
                self.set_scroll_delta(Point::new(
                    cur.get_x() + delta.get_x(),
                    cur.get_y() + delta.get_y(),
                ));
            }
            WindowEvent::Focused(_, false) => {
                self.get_mut_keys_down().clear();
                self.get_mut_buttons_down().clear();
            }
            _ => {}
        }
    }

    /// Returns `true` while `key` is held down.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Input state reference.
    /// - `KeyCode` - Key to query.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the key is held.
    #[must_use]
    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.get_keys_down().contains(&key)
    }

    /// Returns `true` if `key` transitioned from up to down during the current frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Input state reference.
    /// - `KeyCode` - Key to query.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the key was just pressed.
    #[must_use]
    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.get_keys_just_pressed().contains(&key)
    }

    /// Returns `true` while pointer `button` is held down.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Input state reference.
    /// - `PointerButton` - Button to query.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the button is held.
    #[must_use]
    pub fn is_button_down(&self, button: PointerButton) -> bool {
        self.get_buttons_down().contains(&button)
    }

    /// Computes a normalized `-1.0..=1.0` movement axis from negative and positive key bindings.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Input state reference.
    /// - `KeyCode` - Negative direction key.
    /// - `KeyCode` - Positive direction key.
    ///
    /// # Returns
    ///
    /// - `f32` - Axis value in `{-1.0, 0.0, 1.0}`.
    #[must_use]
    pub fn axis(&self, negative: KeyCode, positive: KeyCode) -> f32 {
        let mut value: f32 = 0.0;
        if self.is_key_down(negative) {
            value -= 1.0;
        }
        if self.is_key_down(positive) {
            value += 1.0;
        }
        value
    }

    /// Sets the normalized analog virtual stick vector `(dx, dy)` for mobile touch joysticks or gamepads.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable input state.
    /// - `f32` - Horizontal axis in `[-1.0, 1.0]`.
    /// - `f32` - Vertical axis in `[-1.0, 1.0]`.
    pub fn set_virtual_stick(&mut self, dx: f32, dy: f32) {
        self.set_virtual_axis(Point::new(dx.clamp(-1.0, 1.0), dy.clamp(-1.0, 1.0)));
    }

    /// Resets the virtual analog stick to neutral `(0.0, 0.0)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable input state.
    pub fn clear_virtual_stick(&mut self) {
        self.set_virtual_axis(Point::new(0.0, 0.0));
    }
}

/// Default arcade action map binding WASD + Arrows, Space/Click fire, Shift/E dash, and P/Esc pause.
impl Default for ActionMap {
    /// Creates an `ActionMap` pre-populated with standard desktop/web arcade controls.
    ///
    /// # Returns
    ///
    /// - `Self` - Standard arcade action map.
    fn default() -> Self {
        let mut map: Self = Self {
            key_bindings: HashMap::new(),
            button_bindings: HashMap::new(),
        };
        map.bind_key(GameAction::MoveLeft, KeyCode::ArrowLeft);
        map.bind_key(GameAction::MoveLeft, KeyCode::Character('a'));
        map.bind_key(GameAction::MoveLeft, KeyCode::Character('A'));
        map.bind_key(GameAction::MoveRight, KeyCode::ArrowRight);
        map.bind_key(GameAction::MoveRight, KeyCode::Character('d'));
        map.bind_key(GameAction::MoveRight, KeyCode::Character('D'));
        map.bind_key(GameAction::MoveUp, KeyCode::ArrowUp);
        map.bind_key(GameAction::MoveUp, KeyCode::Character('w'));
        map.bind_key(GameAction::MoveUp, KeyCode::Character('W'));
        map.bind_key(GameAction::MoveDown, KeyCode::ArrowDown);
        map.bind_key(GameAction::MoveDown, KeyCode::Character('s'));
        map.bind_key(GameAction::MoveDown, KeyCode::Character('S'));
        map.bind_key(GameAction::Fire, KeyCode::Space);
        map.bind_key(GameAction::Fire, KeyCode::Character('j'));
        map.bind_button(GameAction::Fire, PointerButton::Primary);
        map.bind_key(GameAction::Dash, KeyCode::Character('e'));
        map.bind_key(GameAction::Dash, KeyCode::Character('E'));
        map.bind_key(GameAction::Jump, KeyCode::Space);
        map.bind_key(GameAction::Pause, KeyCode::Escape);
        map.bind_key(GameAction::Pause, KeyCode::Character('p'));
        map.bind_key(GameAction::ActionAlt, KeyCode::Character('r'));
        map.bind_key(GameAction::ActionAlt, KeyCode::Character('R'));
        map
    }
}

/// Binding registration and real-time input queries for `ActionMap`.
impl ActionMap {
    /// Binds a `KeyCode` to `action`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable action map.
    /// - `GameAction` - Semantic game action.
    /// - `KeyCode` - Keyboard key to bind.
    pub fn bind_key(&mut self, action: GameAction, key: KeyCode) {
        self.get_mut_key_bindings()
            .entry(action)
            .or_default()
            .push(key);
    }

    /// Binds a `PointerButton` to `action`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable action map.
    /// - `GameAction` - Semantic game action.
    /// - `PointerButton` - Mouse/touch button to bind.
    pub fn bind_button(&mut self, action: GameAction, button: PointerButton) {
        self.get_mut_button_bindings()
            .entry(action)
            .or_default()
            .push(button);
    }

    /// Returns `true` while any key or button bound to `action` is held in `input`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Action map reference.
    /// - `GameAction` - Action to query.
    /// - `&InputState` - Current polled input snapshot.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether `action` is currently active.
    #[must_use]
    pub fn is_active(&self, action: GameAction, input: &InputState) -> bool {
        if action == GameAction::Fire && input.get_virtual_fire() {
            return true;
        }
        if action == GameAction::Dash && input.get_virtual_dash() {
            return true;
        }
        let key_hit: bool = self
            .get_key_bindings()
            .get(&action)
            .map(|keys: &Vec<KeyCode>| keys.iter().copied().any(|k: KeyCode| input.is_key_down(k)))
            .unwrap_or(false);
        if key_hit {
            return true;
        }
        self.get_button_bindings()
            .get(&action)
            .map(|btns: &Vec<PointerButton>| {
                btns.iter()
                    .copied()
                    .any(|b: PointerButton| input.is_button_down(b))
            })
            .unwrap_or(false)
    }

    /// Returns `true` when any key bound to `action` transitioned down on this frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Action map reference.
    /// - `GameAction` - Action to query.
    /// - `&InputState` - Current polled input snapshot.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether `action` was just triggered.
    #[must_use]
    pub fn is_just_pressed(&self, action: GameAction, input: &InputState) -> bool {
        self.get_key_bindings()
            .get(&action)
            .map(|keys: &Vec<KeyCode>| {
                keys.iter()
                    .copied()
                    .any(|k: KeyCode| input.is_key_just_pressed(k))
            })
            .unwrap_or(false)
    }

    /// Computes a normalized 2D movement vector `(dx, dy)` from directional action bindings.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Action map reference.
    /// - `&InputState` - Current polled input snapshot.
    ///
    /// # Returns
    ///
    /// - `Point` - Normalized movement vector with length `<= 1.0`.
    #[must_use]
    pub fn movement_vector(&self, input: &InputState) -> Point {
        let mut dx: f32 = input.get_virtual_axis().get_x();
        let mut dy: f32 = input.get_virtual_axis().get_y();
        if self.is_active(GameAction::MoveLeft, input) {
            dx -= 1.0;
        }
        if self.is_active(GameAction::MoveRight, input) {
            dx += 1.0;
        }
        if self.is_active(GameAction::MoveUp, input) {
            dy -= 1.0;
        }
        if self.is_active(GameAction::MoveDown, input) {
            dy += 1.0;
        }
        let len: f32 = (dx * dx + dy * dy).sqrt();
        if len > 1.0 {
            Point::new(dx / len, dy / len)
        } else {
            Point::new(dx, dy)
        }
    }
}
