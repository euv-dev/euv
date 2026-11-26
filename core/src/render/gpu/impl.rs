use super::*;

/// Default `NativeGpuDriver` targets headless recording fallback.
impl Default for NativeGpuDriver {
    /// Creates a headless `NativeGpuDriver`.
    ///
    /// # Returns
    ///
    /// - `Self` - Headless GPU driver.
    fn default() -> Self {
        Self::probe(GraphicsBackend::Headless)
    }
}

/// Construction helpers for `NativeGpuDriver`.
impl NativeGpuDriver {
    /// Probes the active OS GPU hardware, compiles native shaders, and returns a live `NativeGpuDriver`.
    ///
    /// # Arguments
    ///
    /// - `GraphicsBackend` - Requested graphics backend.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized native GPU driver with automatic headless fallback.
    #[must_use]
    pub fn probe(backend: GraphicsBackend) -> Self {
        let (adapter, device_handle, queue_handle, library_handle): (
            GpuAdapterInfo,
            usize,
            usize,
            usize,
        ) = probe_native_gpu(backend);
        let mut fallback: RecordingGpuDriver = RecordingGpuDriver::default();
        fallback.set_backend_kind(backend);
        Self {
            adapter,
            device_handle,
            queue_handle,
            library_handle,
            submitted_frames: 0,
            uploaded_instances: 0,
            fallback,
        }
    }

    /// Re-initializes the GPU driver for `backend`, releasing any previously retained OS handles.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable GPU driver.
    /// - `GraphicsBackend` - Target graphics backend.
    pub fn switch_backend(&mut self, backend: GraphicsBackend) {
        if self.get_adapter().get_backend() == backend {
            return;
        }
        *self = Self::probe(backend);
    }
}

/// `GpuDriver` implementation uploading instanced quad buffers to the active physical OS GPU.
impl GpuDriver for NativeGpuDriver {
    /// Returns the resolved `GraphicsBackend`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Driver reference.
    ///
    /// # Returns
    ///
    /// - `GraphicsBackend` - Active backend tag.
    fn backend(&self) -> GraphicsBackend {
        self.get_adapter().get_backend()
    }

    /// Returns the native shader source compiled by this GPU backend.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Driver reference.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Native shader program source.
    fn shader_source(&self) -> &'static str {
        self.get_fallback().shader_source()
    }

    /// Uploads instanced batches to physical GPU memory and records draw telemetry.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable GPU driver.
    /// - `u32` - Surface width.
    /// - `u32` - Surface height.
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
    ) -> Result<usize, PlatformError> {
        let uploaded: usize =
            submit_native_gpu_batches(self.get_device_handle(), self.get_queue_handle(), batches);
        let Self {
            submitted_frames,
            uploaded_instances,
            fallback,
            ..
        } = self;
        *submitted_frames = submitted_frames.wrapping_add(1);
        *uploaded_instances = uploaded_instances.wrapping_add(uploaded as u64);
        fallback.submit_batches(width, height, clear_color, batches)
    }
}

/// Automatic native GPU handle cleanup when `NativeGpuDriver` is dropped.
impl Drop for NativeGpuDriver {
    /// Releases retained OS GPU device, command queue, and shader library handles.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Dropping driver.
    fn drop(&mut self) {
        release_native_gpu_handles(
            self.get_device_handle(),
            self.get_queue_handle(),
            self.get_library_handle(),
        );
    }
}
