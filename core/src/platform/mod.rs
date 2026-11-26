#[cfg(target_os = "android")]
mod android;
mod backend;
mod headless;
mod info;
#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "android")]
pub use android::*;
#[cfg(target_os = "ios")]
pub use ios::*;
#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(target_arch = "wasm32")]
pub use web::*;
#[cfg(target_os = "windows")]
pub use windows::*;
pub use {backend::*, headless::*, info::*};

use super::*;
