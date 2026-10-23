use super::*;

/// Inherent implementation of [`RayTraceBackendAdapter`].
impl RayTraceBackendAdapter {
    /// Attaches one explicitly requested backend.
    ///
    /// A backend that cannot be attached is an error; nothing is left
    /// half-built and no other backend is substituted.
    ///
    /// # Arguments
    ///
    /// - `&HtmlCanvasElement` - The canvas to draw into.
    /// - `RayTraceBackend` - The backend to attach.
    /// - `u32` - The surface width in pixels.
    ///
    /// # Returns
    ///
    /// - `Result<Self, JsValue>` - The live adapter, or the reason it
    ///   could not be built.
    pub(crate) async fn attach(
        canvas: &HtmlCanvasElement,
        backend: RayTraceBackend,
        width: u32,
        height: u32,
    ) -> Result<Self, JsValue> {
        match backend {
            RayTraceBackend::Canvas2D => Ok(Self::Canvas2D(CanvasRayTraceBackend::attach(
                canvas, width, height,
            )?)),
            RayTraceBackend::WebGl2 => Ok(Self::WebGl2(WebGlRayTraceBackend::attach(
                canvas, width, height,
            )?)),
            RayTraceBackend::WebGpu => {
                Ok(Self::WebGpu(WebGpuRayTraceBackend::attach(canvas).await?))
            }
        }
    }

    /// The largest pixel count this adapter accepts on one axis.
    ///
    /// The limit is per axis, never a byte total: it is what the driver
    /// enforces on one texture or viewport dimension, and comparing a
    /// surface's whole RGBA byte count against it would reject sizes the
    /// hardware renders perfectly well.
    ///
    /// # Returns
    ///
    /// - `Option<u32>` - The per-axis pixel ceiling, or `None` for the
    ///   backends that expose none to enforce.
    pub(crate) fn dimension_limit(&self) -> Option<u32> {
        match self {
            Self::Canvas2D(backend) => backend.dimension_limit(),
            Self::WebGl2(backend) => backend.dimension_limit(),
            Self::WebGpu(backend) => backend.dimension_limit(),
        }
    }

    /// Applies a new surface size to the attached adapter.
    ///
    /// # Arguments
    ///
    /// - `u32` - The surface width in pixels.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the new size is live.
    pub(crate) fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        match self {
            Self::Canvas2D(backend) => backend.resize(width, height),
            Self::WebGl2(backend) => backend.resize(width, height),
            Self::WebGpu(backend) => backend.resize(width, height),
        }
    }

    /// Uploads the packed scene block.
    ///
    /// # Arguments
    ///
    /// - `&[f32]` - The packed scene block.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the upload was issued, or
    ///   `Ok` for the Canvas path which traces on the CPU.
    pub(crate) fn upload_scene(&mut self, packed: &[f32]) -> Result<(), JsValue> {
        match self {
            Self::Canvas2D(_) => Ok(()),
            Self::WebGl2(backend) => backend.upload_scene(packed),
            Self::WebGpu(backend) => backend.upload_scene(packed),
        }
    }

    /// Issues the frame on the GPU paths.
    ///
    /// # Arguments
    ///
    /// - `&[f32]` - The packed frame block.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the frame is submitted, or
    ///   `Ok` for the Canvas path, which presents through
    ///   [`RayTraceBackendAdapter::present`].
    pub(crate) fn draw(&mut self, frame: &[f32]) -> Result<(), JsValue> {
        match self {
            Self::Canvas2D(_) => Ok(()),
            Self::WebGl2(backend) => backend.draw(frame),
            Self::WebGpu(backend) => backend.draw(frame),
        }
    }

    /// Presents the already-traced Canvas pixels.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The traced RGBA bytes.
    ///
    /// # Returns
    ///
    /// - `Result<(), JsValue>` - `Ok` once the blit was accepted, or
    ///   `Ok` for the GPU paths which present through their own draw.
    pub(crate) fn present(&mut self, pixels: &[u8]) -> Result<(), JsValue> {
        match self {
            Self::Canvas2D(backend) => backend.present(pixels),
            _ => Ok(()),
        }
    }

    /// Releases every resource this adapter owns.
    pub(crate) fn release(&mut self) {
        match self {
            Self::Canvas2D(backend) => backend.release(),
            Self::WebGl2(backend) => backend.release(),
            Self::WebGpu(backend) => backend.release(),
        }
    }
}
