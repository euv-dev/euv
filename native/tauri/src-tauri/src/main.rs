#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Starts the application with the platform-specific webview configuration.
///
/// # Returns
/// - `()` - Completes after the application event loop exits.
fn main() {
    euv_lib::run();
}
