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
}
