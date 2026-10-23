use super::*;

/// Inherent implementation of [`BackendKind`].
impl BackendKind {
    /// Returns the name of the backend.
    ///
    /// # Returns
    ///
    /// - `&'static str` - A `'static str` value.
    pub fn name(&self) -> &'static str {
        match self {
            BackendKind::WebGpu => "webgpu",
            BackendKind::WebGl2 => "webgl2",
            BackendKind::Canvas2D => "canvas2d",
        }
    }

    /// Returns `true` when the backend runs on the GPU.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_gpu(&self) -> bool {
        matches!(self, BackendKind::WebGpu | BackendKind::WebGl2)
    }
}

/// Inherent implementation of [`BackendFactory`].
impl BackendFactory {
    /// Returns a factory.
    pub fn new() -> Self {
        Self
    }

    /// Creates the best backend for the given canvas.
    ///
    /// `preferred` is the backend the caller would like; the factory
    /// falls back to the next one down when the host cannot provide it.
    /// The fallback chain is always WebGPU, WebGL2, Canvas2D.
    ///
    /// # Arguments
    ///
    /// - `&HtmlCanvasElement` - The `&HtmlCanvasElement` value.
    /// - `BackendKind` - The `BackendKind` value.
    ///
    /// # Returns
    ///
    /// - `Option<Box<dyn ChartBackend>>` - The `Option<Box<dyn ChartBackend>>` value.
    pub async fn create(
        canvas: &HtmlCanvasElement,
        preferred: BackendKind,
    ) -> Option<Box<dyn ChartBackend>> {
        let order: &[BackendKind] = match preferred {
            BackendKind::WebGpu => &[
                BackendKind::WebGpu,
                BackendKind::WebGl2,
                BackendKind::Canvas2D,
            ],
            BackendKind::WebGl2 => &[BackendKind::WebGl2, BackendKind::Canvas2D],
            BackendKind::Canvas2D => &[BackendKind::Canvas2D],
        };

        for kind in order {
            match kind {
                BackendKind::WebGpu => {
                    if let Some(backend) = webgpu::WebGpuChartBackend::create(canvas).await {
                        return Some(Box::new(backend));
                    }
                }
                BackendKind::WebGl2 => {
                    if let Some(backend) = webgl::WebGl2ChartBackend::create(canvas) {
                        return Some(Box::new(backend));
                    }
                }
                BackendKind::Canvas2D => {
                    if let Some(backend) = canvas2d::Canvas2dChartBackend::create(canvas) {
                        return Some(Box::new(backend));
                    }
                }
            }
        }
        None
    }

    /// Returns the backend the host would pick, without creating it.
    ///
    /// The check is cheap and synchronous, so a chart can report the
    /// backend it is about to use before the asynchronous creation
    /// finishes.
    ///
    /// # Arguments
    ///
    /// - `BackendKind` - A `BackendKind` parameter.
    ///
    /// # Returns
    ///
    /// - `BackendKind` - A `BackendKind` value.
    pub fn detect(preferred: BackendKind) -> BackendKind {
        if preferred == BackendKind::WebGpu && !webgpu::WebGpuChartBackend::is_supported() {
            return if webgl::WebGl2ChartBackend::is_supported() {
                BackendKind::WebGl2
            } else {
                BackendKind::Canvas2D
            };
        }
        if preferred == BackendKind::WebGl2 && !webgl::WebGl2ChartBackend::is_supported() {
            return BackendKind::Canvas2D;
        }
        preferred
    }
}
