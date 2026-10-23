/// The rendering backend a chart is running on.
///
/// The backend is chosen at run time: the chart asks for WebGPU, falls
/// back to WebGL2, and falls back again to Canvas2D. From the outside the
/// three look identical, because they all replay the same display list.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BackendKind {
    /// WebGPU: the fastest backend, and the one the chart prefers.
    ///
    /// Vector geometry is tessellated into triangle batches and uploaded
    /// to the GPU once; a steady-state frame issues a handful of draw
    /// calls and touches no JavaScript.
    WebGpu,
    /// WebGL2: the GPU fallback for browsers without WebGPU.
    ///
    /// The same batches as WebGPU, drawn through WebGL2 instancing.
    WebGl2,
    /// Canvas2D: the universal fallback.
    ///
    /// The display list is replayed as immediate-mode canvas calls. Slower
    /// than the GPU backends, but it works everywhere.
    #[default]
    Canvas2D,
}
