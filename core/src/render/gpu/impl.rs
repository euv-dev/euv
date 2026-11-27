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
            staging_buffer_handle: 0,
            staging_contents_ptr: 0,
            staging_buffer_capacity: 0,
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
        let Self {
            device_handle,
            queue_handle,
            staging_buffer_handle,
            staging_contents_ptr,
            staging_buffer_capacity,
            submitted_frames,
            uploaded_instances,
            fallback,
            ..
        } = self;
        let mut staging: (usize, usize, usize) = (
            *staging_buffer_handle,
            *staging_contents_ptr,
            *staging_buffer_capacity,
        );
        let uploaded: usize =
            submit_native_gpu_batches_staged(*device_handle, *queue_handle, &mut staging, batches);
        (
            *staging_buffer_handle,
            *staging_contents_ptr,
            *staging_buffer_capacity,
        ) = staging;
        *submitted_frames = submitted_frames.wrapping_add(1);
        *uploaded_instances = uploaded_instances.wrapping_add(uploaded as u64);
        fallback.submit_batches(width, height, clear_color, batches)
    }
}

/// Automatic native GPU handle cleanup when `NativeGpuDriver` is dropped.
impl Drop for NativeGpuDriver {
    /// Releases retained OS GPU staging buffer, device, command queue, and shader library handles.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Dropping driver.
    fn drop(&mut self) {
        let Self {
            device_handle,
            queue_handle,
            library_handle,
            staging_buffer_handle,
            staging_contents_ptr,
            staging_buffer_capacity,
            ..
        } = self;
        release_native_gpu_staging(*staging_buffer_handle);
        *staging_buffer_handle = 0;
        *staging_contents_ptr = 0;
        *staging_buffer_capacity = 0;
        release_native_gpu_handles(*device_handle, *queue_handle, *library_handle);
    }
}
