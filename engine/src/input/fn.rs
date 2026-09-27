use super::*;

/// Applies the analog deadzone to a single raw axis reading.
///
/// A reading at or inside the deadzone band reports `0.0`; a reading
/// beyond it is rescaled so the usable range still reaches `1.0` at
/// full deflection. Sign is preserved, so a stick pushed left stays
/// negative.
///
/// # Arguments
///
/// - `f64` - The raw axis reading as reported by the browser.
///
/// # Returns
///
/// - `f64` - The deadzone-corrected reading, centered inside the band.
pub fn apply_axis_deadzone(value: f64) -> f64 {
    let magnitude: f64 = value.abs();
    if magnitude <= INPUT_GAMEPAD_AXIS_DEADZONE {
        return 0.0;
    }
    // Phase 1: rescale the post-deadzone span back onto the full range.
    let span: f64 = 1.0 - INPUT_GAMEPAD_AXIS_DEADZONE;
    let normalized: f64 = (magnitude - INPUT_GAMEPAD_AXIS_DEADZONE) / span;
    value.signum() * normalized.min(1.0)
}

/// Folds one frame of raw gamepad readings into the per-pad edge sets.
///
/// This is the DOM-free half of a poll: it takes the previous frame's
/// held set and this frame's raw button pressures, and recomputes
/// pressed / held / released exactly the way [`InputState::press_key`]
/// and [`InputState::release_key`] do for keys. A button counts as
/// down when its pressure is at least half, which makes digital
/// buttons (`0.0` / `1.0`) and analog triggers agree.
///
/// # Arguments
///
/// - `&GamepadButtonSet` - The previous frame's held button indices.
/// - `&[f64]` - This frame's raw button pressure per button index.
///
/// # Returns
///
/// - `GamepadButtonSet` - The buttons newly pressed on this frame.
pub fn compute_pressed_buttons(
    previous_held: &GamepadButtonSet,
    values: &[f64],
) -> GamepadButtonSet {
    let mut pressed: GamepadButtonSet = GamepadButtonSet::new();
    for (index, value) in values.iter().enumerate() {
        let button: u32 = index as u32;
        if is_button_down(*value) && !previous_held.contains(&button) {
            pressed.insert(button);
        }
    }
    pressed
}

/// Folds one frame of raw gamepad readings into the per-pad edge sets.
///
/// The held set this frame is every button at or above half pressure;
/// the released set is every button that was held last frame and is
/// no longer down, which is what a caller polls with to fire a
/// one-shot action on button-up.
///
/// # Arguments
///
/// - `&GamepadButtonSet` - The previous frame's held button indices.
/// - `&[f64]` - This frame's raw button pressure per button index.
///
/// # Returns
///
/// - `GamepadButtonSet` - The buttons that came up on this frame.
pub fn compute_released_buttons(
    previous_held: &GamepadButtonSet,
    values: &[f64],
) -> GamepadButtonSet {
    let mut released: GamepadButtonSet = GamepadButtonSet::new();
    for button in previous_held.iter() {
        let slot: usize = *button as usize;
        let value: f64 = values.get(slot).copied().unwrap_or(0.0);
        if !is_button_down(value) {
            released.insert(*button);
        }
    }
    released
}

/// Folds one frame of raw gamepad readings into the per-pad edge sets.
///
/// # Arguments
///
/// - `&[f64]` - This frame's raw button pressure per button index.
///
/// # Returns
///
/// - `GamepadButtonSet` - Every button currently down this frame.
pub fn compute_held_buttons(values: &[f64]) -> GamepadButtonSet {
    let mut held: GamepadButtonSet = GamepadButtonSet::new();
    for (index, value) in values.iter().enumerate() {
        if is_button_down(*value) {
            held.insert(index as u32);
        }
    }
    held
}

/// Tests whether one raw button pressure counts as the button being down.
///
/// The threshold is one half so that a digital button reporting
/// `1.0` and an analog trigger crossing its midpoint agree.
///
/// # Arguments
///
/// - `f64` - The raw button pressure as reported by the browser.
///
/// # Returns
///
/// - `bool` - True when the pressure reaches the press threshold.
pub fn is_button_down(value: f64) -> bool {
    value >= GAMEPAD_BUTTON_PRESS_THRESHOLD
}

/// Reads the nth analog axis out of a raw axis reading list.
///
/// # Arguments
///
/// - `&[f64]` - The raw axis readings as reported by the browser.
/// - `u32` - The axis index to read.
///
/// # Returns
///
/// - `f64` - The raw reading, or `0.0` when the pad has no such axis.
pub fn read_raw_axis(axes: &[f64], axis: u32) -> f64 {
    axes.get(axis as usize).copied().unwrap_or(0.0)
}
