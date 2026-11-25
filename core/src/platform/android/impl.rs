use super::*;

/// Default initialization for `AndroidPlatform`.
impl Default for AndroidPlatform {
    /// Creates an Android NDK platform backend awaiting `ANativeWindow` attachment.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized Android backend.
    fn default() -> Self {
        Self {
            next_id: 1,
            attached_native_window: 0,
            density_scale: 2.5,
            windows: HashMap::new(),
            pending_events: Vec::new(),
            clipboard_cache: None,
        }
    }
}

/// Android NDK lifecycle binding methods for `AndroidPlatform`.
impl AndroidPlatform {
    /// Attaches a live `ANativeWindow*` pointer received from `NativeActivity::onNativeWindowCreated`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Android platform.
    /// - `usize` - Raw `ANativeWindow*` pointer.
    /// - `f32` - Device display density scale factor.
    pub fn attach_native_window(&mut self, native_window: usize, density_scale: f32) {
        self.set_attached_native_window(native_window);
        self.set_density_scale(density_scale.max(0.75));
        for rec in self.get_mut_windows().values_mut() {
            rec.set_native_window(native_window);
        }
    }
}

/// `WindowController` implementation for Android NDK.
impl WindowController for AndroidPlatform {
    /// Applies a `WindowCommand` to the active Android surface window.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Android platform.
    /// - `WindowCommand` - Command to apply.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Execution status.
    fn apply_command(&mut self, command: WindowCommand) -> Result<(), PlatformError> {
        match command {
            WindowCommand::SetTitle(id, title) => {
                let rec: &mut AndroidWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                rec.get_mut_config().set_title(title);
                Ok(())
            }
            WindowCommand::SetStatusBarStyle(id, style) => {
                let rec: &mut AndroidWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                rec.get_mut_config().set_status_bar(style);
                Ok(())
            }
            WindowCommand::SetRenderMode(id, mode) => {
                let rec: &mut AndroidWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                rec.get_mut_config().get_mut_render().set_mode(mode);
                Ok(())
            }
            WindowCommand::RequestRedraw(id) => {
                if !self.get_windows().contains_key(&id) {
                    return Err(PlatformError::WindowNotFound(id.get_value()));
                }
                self.get_mut_pending_events()
                    .push(WindowEvent::RedrawRequested(id));
                Ok(())
            }
            WindowCommand::Close(id) => {
                if self.get_mut_windows().remove(&id).is_none() {
                    return Err(PlatformError::WindowNotFound(id.get_value()));
                }
                Ok(())
            }
            WindowCommand::SetSize(id, _, _)
            | WindowCommand::SetPosition(id, _, _)
            | WindowCommand::SetResizable(id, _)
            | WindowCommand::SetDecorations(id, _)
            | WindowCommand::SetAlwaysOnTop(id, _)
            | WindowCommand::SetFullscreen(id, _)
            | WindowCommand::SetVisible(id, _)
            | WindowCommand::SetCursor(id, _)
            | WindowCommand::SetIme(id, _, _) => {
                if !self.get_windows().contains_key(&id) {
                    return Err(PlatformError::WindowNotFound(id.get_value()));
                }
                Ok(())
            }
        }
    }
}

/// `PlatformBackend` implementation for native Android NDK.
impl PlatformBackend for AndroidPlatform {
    /// Returns `PlatformKind::Android`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `PlatformKind` - `PlatformKind::Android`.
    fn kind(&self) -> PlatformKind {
        PlatformKind::Android
    }

    /// Returns Android mobile capabilities.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `PlatformCapabilities` - Android capability flags.
    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities::for_platform(PlatformKind::Android)
    }

    /// Binds a window record to the attached `ANativeWindow*` with Android safe-area cutouts.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Android platform.
    /// - `&WindowConfig` - Declarative window config.
    ///
    /// # Returns
    ///
    /// - `Result<WindowState, PlatformError>` - Initial window state.
    fn create_window(&mut self, config: &WindowConfig) -> Result<WindowState, PlatformError> {
        let id: WindowId = WindowId::new(self.get_next_id());
        self.set_next_id(self.get_next_id().wrapping_add(1));
        let scale: f32 = self.get_density_scale();
        let mut state: WindowState = WindowState::from_config(id, config, scale);
        if let Some((pw, ph)) = android_query_window_size(self.get_attached_native_window()) {
            state.set_physical_width(pw);
            state.set_physical_height(ph);
            state.set_logical_width((pw as f32) / scale);
            state.set_logical_height((ph as f32) / scale);
        }
        state.set_safe_area(SafeAreaInsets::new(
            ANDROID_DEFAULT_SAFE_TOP,
            0.0,
            ANDROID_DEFAULT_SAFE_BOTTOM,
            0.0,
        ));
        let nw: usize = self.get_attached_native_window();
        self.get_mut_windows().insert(
            id,
            AndroidWindowRecord {
                config: config.clone(),
                state,
                native_window: nw,
            },
        );
        Ok(state)
    }

    /// Queries live `WindowState` for `id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    /// - `WindowId` - Window handle.
    ///
    /// # Returns
    ///
    /// - `Option<WindowState>` - Window state if active.
    fn window_state(&self, id: WindowId) -> Option<WindowState> {
        self.get_windows()
            .get(&id)
            .map(AndroidWindowRecord::get_state)
    }

    /// Returns `RawWindowHandle::AndroidNdk` carrying `ANativeWindow*` for Vulkan/EGL binding.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    /// - `WindowId` - Window handle.
    ///
    /// # Returns
    ///
    /// - `RawWindowHandle` - Android NDK surface handle.
    fn raw_window_handle(&self, id: WindowId) -> RawWindowHandle {
        match self.get_windows().get(&id) {
            Some(rec) => RawWindowHandle::AndroidNdk {
                native_window: rec.get_native_window(),
            },
            None => RawWindowHandle::Headless,
        }
    }

    /// Drains queued Android NDK events.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Android platform.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Pending events.
    fn poll_events(&mut self) -> Vec<WindowEvent> {
        take(self.get_mut_pending_events())
    }

    /// Blits `framebuffer` directly into the locked `ANativeWindow` hardware buffer.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Android platform.
    /// - `WindowId` - Window handle.
    /// - `&Framebuffer` - Rendered RGBA8 framebuffer.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Presentation status.
    fn present_framebuffer(
        &mut self,
        id: WindowId,
        framebuffer: &Framebuffer,
    ) -> Result<(), PlatformError> {
        let nw: usize = self
            .get_windows()
            .get(&id)
            .map(AndroidWindowRecord::get_native_window)
            .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
        android_present_native_window(nw, framebuffer)
    }

    /// Reads cached clipboard text.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Clipboard contents.
    fn read_clipboard(&self) -> Option<String> {
        let Self {
            clipboard_cache, ..
        } = self;
        clipboard_cache.clone()
    }

    /// Writes `text` into clipboard cache.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable platform.
    /// - `&str` - Clipboard text.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Write status.
    fn write_clipboard(&mut self, text: &str) -> Result<(), PlatformError> {
        self.set_clipboard_cache(Some(text.to_string()));
        Ok(())
    }

    /// Returns active Android theme mode.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `ThemeMode` - Active appearance mode.
    fn system_theme(&self) -> ThemeMode {
        ThemeMode::Light
    }
}
