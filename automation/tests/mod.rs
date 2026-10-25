#[cfg(not(target_arch = "wasm32"))]
mod contract;

#[cfg(not(target_arch = "wasm32"))]
use super::*;
