use super::*;

thread_local! {
    /// Reusable thread-local WASM linear memory buffer avoiding repeated 64 KiB page allocations and full-page zero fills.
    pub(crate) static WASM_LINEAR_MEMORY_POOL: RefCell<WasmMemoryPool> = const {
        RefCell::new(WasmMemoryPool { buffer: Vec::new() })
    };
}
