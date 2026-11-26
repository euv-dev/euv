//! euv
//!
//! A high-performance cross-platform native UI framework for Rust targeting Web,
//! Android, iOS, iPadOS, macOS, Windows, and Linux with direct system APIs,
//! fine-grained reactivity, zero-VDOM macros, and multi-mode rendering.

pub use {euv_core::*, euv_macros::*};

#[cfg(target_arch = "wasm32")]
pub use {console_error_panic_hook, js_sys, wasm_bindgen, wasm_bindgen_futures, web_sys};
