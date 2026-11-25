use super::*;

/// Normalized hardware and OS capability flags reported by every platform backend.
#[derive(Clone, Copy, Debug, Default, Eq, Getter, Hash, PartialEq)]
pub struct PlatformCapabilities {
    /// Supports opening and managing multiple independent OS windows simultaneously.
    #[get(type(copy))]
    pub(crate) multi_window: bool,
    /// Supports hover pointer tracking (mouse, trackpad, Apple Pencil hover).
    #[get(type(copy))]
    pub(crate) hover_pointer: bool,
    /// Supports multi-touch gestures.
    #[get(type(copy))]
    pub(crate) touch: bool,
    /// Supports stylus / Apple Pencil pressure input.
    #[get(type(copy))]
    pub(crate) stylus: bool,
    /// Supports physical hardware keyboard input.
    #[get(type(copy))]
    pub(crate) hardware_keyboard: bool,
    /// Reports non-trivial OS safe-area cutouts or gesture bars.
    #[get(type(copy))]
    pub(crate) safe_area_insets: bool,
    /// Supports native GPU hardware acceleration (Metal / Vulkan / D3D / WebGPU / WebGL2).
    #[get(type(copy))]
    pub(crate) hardware_gpu: bool,
    /// Supports direct CPU framebuffer presentation.
    #[get(type(copy))]
    pub(crate) software_blit: bool,
    /// Supports native system clipboard read/write.
    #[get(type(copy))]
    pub(crate) clipboard: bool,
    /// Supports native IME / soft keyboard composition.
    #[get(type(copy))]
    pub(crate) ime: bool,
}
