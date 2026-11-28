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
            refresh_rate_hz: FALLBACK_REFRESH_RATE_HZ,
            windows: HashMap::new(),
            pending_events: Vec::new(),
            clipboard_cache: None,
            pump_paused: false,
        }
    }
}

/// Android NDK lifecycle binding methods for `AndroidPlatform`.
impl AndroidPlatform {
    /// Attaches a live `ANativeWindow*` pointer received from `NativeActivity::onNativeWindowCreated` and hints the target refresh rate.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Android platform.
    /// - `usize` - Raw `ANativeWindow*` pointer.
    /// - `f32` - Device display density scale factor.
    pub fn attach_native_window(&mut self, native_window: usize, density_scale: f32) {
        let hz: f32 = self.get_refresh_rate_hz();
        self.set_attached_native_window(native_window);
        self.set_density_scale(density_scale.max(0.75));
        for rec in self.get_mut_windows().values_mut() {
            rec.set_native_window(native_window);
            rec.set_surface_width(0);
            rec.set_surface_height(0);
        }
        let _: bool = android_set_native_window_frame_rate(native_window, hz);
    }

    /// Detaches the active `ANativeWindow*` pointer on `APP_CMD_TERM_WINDOW` and cancels dangling touches so backgrounded apps never wedge input or touch destroyed surfaces.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Android platform.
    pub fn detach_native_window(&mut self) {
        self.set_attached_native_window(0);
        let mut ids: Vec<WindowId> = Vec::with_capacity(self.get_windows().len());
        for (&wid, rec) in self.get_mut_windows().iter_mut() {
            rec.set_native_window(0);
            rec.set_surface_width(0);
            rec.set_surface_height(0);
            ids.push(wid);
        }
        for wid in ids.into_iter() {
            self.get_mut_pending_events()
                .push(WindowEvent::PointerCancel(wid, 0));
        }
    }

    /// Resets transient per-activity session state on same-process re-entry while preserving detected display metrics.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Android platform.
    pub fn reset_session(&mut self) {
        self.set_attached_native_window(0);
        self.set_pump_paused(false);
        self.get_mut_windows().clear();
        self.get_mut_pending_events().clear();
    }

    /// Updates the detected Android hardware display refresh rate (e.g. `90`/`120`/`144` Hz) and hints the attached surface.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Android platform.
    /// - `f32` - Active display refresh rate in Hertz.
    pub fn update_refresh_rate_hz(&mut self, hz: f32) {
        let pacing: FramePacingConfig = compute_frame_pacing(hz);
        let preferred_hz: f32 = pacing.get_preferred_hz();
        self.set_refresh_rate_hz(preferred_hz);
        let rounded_hz: u32 = preferred_hz.round() as u32;
        for rec in self.get_mut_windows().values_mut() {
            rec.get_mut_state()
                .get_mut_display()
                .set_refresh_rate_hz(rounded_hz);
        }
        let nw: usize = self.get_attached_native_window();
        let _: bool = android_set_native_window_frame_rate(nw, preferred_hz);
    }

    /// Updates live safe-area insets across open Android windows and enqueues `WindowEvent::SafeAreaChanged`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Android platform.
    /// - `SafeAreaInsets` - Resolved safe-area insets in logical dp.
    pub fn update_safe_area(&mut self, insets: SafeAreaInsets) {
        let mut ids: Vec<WindowId> = Vec::with_capacity(self.get_windows().len());
        for (&wid, rec) in self.get_mut_windows().iter_mut() {
            rec.get_mut_state().set_safe_area(insets);
            ids.push(wid);
        }
        for wid in ids.into_iter() {
            self.get_mut_pending_events()
                .push(WindowEvent::SafeAreaChanged(wid, insets));
        }
    }

    /// Enqueues a normalized Android `SystemEvent` to be drained on the next `poll_events()` tick.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Android platform backend.
    /// - `SystemEvent` - System event forwarded from `Activity` / `Application` JNI hooks.
    pub fn push_system_event(&mut self, event: SystemEvent) {
        match &event {
            SystemEvent::Background | SystemEvent::Terminate => {
                self.set_pump_paused(true);
            }
            SystemEvent::Appear | SystemEvent::Foreground => {
                self.set_pump_paused(false);
            }
            _ => {}
        }
        self.get_mut_pending_events()
            .push(WindowEvent::System(event));
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
            WindowCommand::SetFullscreen(id, mode) => {
                let rec: &mut AndroidWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                rec.get_mut_config().set_fullscreen(mode);
                rec.get_mut_state().set_fullscreen(mode);
                let arg: &str = if mode == FullscreenMode::Windowed {
                    FULLSCREEN_ACTION_OFF
                } else {
                    FULLSCREEN_ACTION_ON
                };
                let _: bool = dispatch_native_action(NATIVE_ACTION_FULLSCREEN, arg, "");
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

    /// Binds a window record to the attached `ANativeWindow*` with Android safe-area cutouts and active refresh rate.
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
        let hz: f32 = self.get_refresh_rate_hz();
        let mut state: WindowState = WindowState::from_config(id, config, scale);
        state
            .get_mut_display()
            .set_refresh_rate_hz(hz.round() as u32);
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
        let _: bool = android_set_native_window_frame_rate(nw, hz);
        self.get_mut_windows().insert(
            id,
            AndroidWindowRecord {
                config: config.clone(),
                state,
                native_window: nw,
                surface_width: 0,
                surface_height: 0,
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

    /// Pauses or resumes the Android Choreographer / NDK frame pump flag.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Android platform.
    /// - `bool` - `true` when backgrounded; `false` when resumed.
    fn set_frame_pump_paused(&mut self, paused: bool) {
        self.set_pump_paused(paused);
    }

    /// Returns whether the Android frame pump is currently paused for background freeze.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when paused in background.
    fn is_frame_pump_paused(&self) -> bool {
        self.get_pump_paused()
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
        let rec: &mut AndroidWindowRecord = self
            .get_mut_windows()
            .get_mut(&id)
            .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
        let nw: usize = rec.get_native_window();
        let mut surf_w: u32 = rec.get_surface_width();
        let mut surf_h: u32 = rec.get_surface_height();
        let res: Result<(), PlatformError> =
            android_present_native_window(nw, framebuffer, &mut surf_w, &mut surf_h);
        rec.set_surface_width(surf_w);
        rec.set_surface_height(surf_h);
        res
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
        let _: bool = write_system_clipboard(text);
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

    /// Returns the detected Android hardware display refresh rate in Hertz (`90.0`/`120.0`/`144.0`+).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `f32` - Active refresh rate in Hertz.
    fn display_refresh_rate_hz(&self) -> f32 {
        self.get_refresh_rate_hz()
    }
}
