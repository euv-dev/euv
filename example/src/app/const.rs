//! String constants for the crate root (rust-standards §1.3c).
//!
//! `lib.rs` is the crate root, so this module is declared from it as
//! `mod r#const;` — the `r#` prefix is required because `const` is a Rust
//! keyword. Every string literal that the §1.3c verifier flags in a
//! function body lives here, verbatim and byte-identical to its original
//! inline form.

/// The CSS selector of the DOM element the example app mounts into.
///
/// Must match the container element declared by the host page's markup.
pub(crate) const APP_MOUNT_SELECTOR: &str = "#app";
