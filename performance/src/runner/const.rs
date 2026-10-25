/// Compiler executable queried for provenance information.
pub(crate) const COMPILER: &str = "rustc";

/// Report marker for a build without allocator instrumentation.
pub(crate) const MODE_TIMING: &str = "timing";

/// Report marker for an allocation-instrumented build.
pub(crate) const MODE_ALLOCATIONS: &str = "allocations";

/// Requests verbose compiler version and target information.
pub(crate) const COMPILER_FLAG: &str = "-Vv";

/// Benchmark package directory embedded when the harness is compiled.
pub(crate) const SOURCE_ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// Documents the measurement scope and requested-heap-byte semantics.
pub(crate) const LIMITATIONS: &str = "CPU-only native workloads; text uses approximate metrics. No browser DOM, GPU submission, GPU memory, process RSS, or end-to-end FPS is measured. Heap counters are per-thread requested bytes through a System allocator wrapper. Signal slabs intentionally retain stale-handle payloads until thread exit. Peak extra bytes exclude the fixture already live at the start of the phase. Reallocation traffic counts the full new requested size.";
