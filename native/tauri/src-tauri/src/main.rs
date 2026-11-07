#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Starts the application with the platform-specific webview configuration.
///
/// # Returns
/// - `()` - Completes after the application event loop exits.
fn main() {
    #[cfg(target_os = "windows")]
    unsafe {
        std::env::set_var(
            "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
            "--enable-features=WebGPU --enable-unsafe-webgpu --ignore-gpu-blocklist",
        );
    }
    euv_lib::run();
}
