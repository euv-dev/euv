use super::*;

/// Hybrid renderer combining CPU dirty-tile rasterization with GPU textured quad compositing.
#[derive(Clone, Data, Debug, Default)]
pub struct HybridRenderer {
    /// CPU tile rasterizer producing crisp pixel-identical surfaces.
    pub(crate) software: SoftwareRenderer,
    /// GPU compositor driver presenting uploaded dirty tiles.
    pub(crate) compositor: RecordingGpuDriver,
}
