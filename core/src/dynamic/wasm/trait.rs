use super::*;

/// Pluggable transport abstraction for downloading `.wasm` binaries over HTTP/HTTPS, CDN, local disk, or test registries.
pub trait WasmFetcher {
    /// Fetches raw `.wasm` binary bytes for `request`, invoking `on_progress(received, total)` as chunks arrive.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Fetcher reference.
    /// - `&WasmDownloadRequest` - Target download descriptor.
    /// - `u32` - Zero-based retry attempt counter (`0` on initial attempt).
    /// - `&mut dyn FnMut(u64, u64)` - Streaming byte-progress callback.
    ///
    /// # Returns
    ///
    /// - `Result<WasmFetchedPayload, WasmDynamicError>` - Downloaded `.wasm` bytes or typed transport error.
    fn fetch(
        &self,
        request: &WasmDownloadRequest,
        attempt: u32,
        on_progress: &mut dyn FnMut(u64, u64),
    ) -> Result<WasmFetchedPayload, WasmDynamicError>;
}
