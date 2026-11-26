#[cfg(not(target_arch = "wasm32"))]
use super::*;

/// Process-wide persistent worker pool shared by `Renderer3D` across frames (zero per-frame `pthread_create`).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) static ENGINE3D_WORKER_POOL: OnceLock<Engine3dWorkerPool> = OnceLock::new();
