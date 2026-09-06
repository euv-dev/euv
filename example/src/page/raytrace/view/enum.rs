use super::*;

/// Represents the available rendering backend tabs on the RayTrace page.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum RayTraceTab {
    /// The Canvas 2D software rendering backend tab.
    #[default]
    Canvas2D,
    /// The WebGL 2 rendering backend tab.
    WebGl,
    /// The WebGPU rendering backend tab.
    WebGpu,
}
