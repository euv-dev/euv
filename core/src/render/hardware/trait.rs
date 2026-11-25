use super::*;

/// Hardware GPU backend driver contract implemented per graphics API (Metal, Vulkan, D3D, WebGPU, WebGL2, Headless).
pub trait GpuDriver {
    /// Returns the concrete `GraphicsBackend` served by this driver.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Driver reference.
    ///
    /// # Returns
    ///
    /// - `GraphicsBackend` - Active backend identifier.
    fn backend(&self) -> GraphicsBackend;

    /// Returns the native shader source compiled by this GPU backend.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Driver reference.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Shader program source string.
    fn shader_source(&self) -> &'static str;

    /// Submits compiled instanced GPU batches for presentation on the active surface.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable GPU driver.
    /// - `u32` - Physical surface width.
    /// - `u32` - Physical surface height.
    /// - `Color` - Surface clear color.
    /// - `&[GpuBatch]` - Ordered draw batches.
    ///
    /// # Returns
    ///
    /// - `Result<usize, PlatformError>` - Number of submitted GPU draw calls.
    fn submit_batches(
        &mut self,
        width: u32,
        height: u32,
        clear_color: Color,
        batches: &[GpuBatch],
    ) -> Result<usize, PlatformError>;
}
