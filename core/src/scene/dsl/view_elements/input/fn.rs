/// Schema anchor for built-in `view!` element `input` — Editable single-line or multiline native text input leaf node.
pub const fn element() {}

/// Sets initial text value (`impl Into<String>`).
pub const fn value() {}

/// Sets placeholder hint text shown when empty (`impl Into<String>`).
pub const fn placeholder() {}

/// Enables password masking (`bool`).
pub const fn password() {}

/// Enables multi-line editing mode (`bool`).
pub const fn multiline() {}

/// Disables user text editing (`bool`).
pub const fn disabled() {}

/// Sets initial byte caret position (`usize`).
pub const fn caret() {}
