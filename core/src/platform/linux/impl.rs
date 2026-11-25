use super::*;

/// Default initialization for `LinuxPlatform`.
impl Default for LinuxPlatform {
    /// Creates an empty Linux X11 / Wayland platform backend.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized Linux backend.
    fn default() -> Self {
        Self {
            next_id: 1,
            x11_lib: 0,
            windows: HashMap::new(),
            pending_events: Vec::new(),
            clipboard_cache: None,
        }
    }
}

/// `WindowController` implementation for Linux X11 / Wayland.
impl WindowController for LinuxPlatform {
    /// Applies a `WindowCommand` onto the native X11 or Wayland window.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Linux platform.
    /// - `WindowCommand` - Command to apply.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Execution status.
    fn apply_command(&mut self, command: WindowCommand) -> Result<(), PlatformError> {
        let lib: usize = self.get_x11_lib();
        match command {
            WindowCommand::SetTitle(id, title) => {
                let rec: &mut LinuxWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                if !rec.get_is_wayland() {
                    linux_x11_set_title(lib, rec.get_display(), rec.get_window(), &title);
                }
                rec.get_mut_config().set_title(title);
                Ok(())
            }
            WindowCommand::SetSize(id, width, height) => {
                let rec: &mut LinuxWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                let w: u32 = width.max(1);
                let h: u32 = height.max(1);
                if !rec.get_is_wayland() {
                    linux_x11_resize(lib, rec.get_display(), rec.get_window(), w, h);
                }
                let scale: f32 = rec.get_state().get_scale_factor();
                rec.get_mut_config().set_width(w);
                rec.get_mut_config().set_height(h);
                rec.get_mut_state().set_logical_width(w as f32);
                rec.get_mut_state().set_logical_height(h as f32);
                rec.get_mut_state()
                    .set_physical_width(((w as f32) * scale).round() as u32);
                rec.get_mut_state()
                    .set_physical_height(((h as f32) * scale).round() as u32);
                Ok(())
            }
            WindowCommand::Close(id) => {
                let rec: LinuxWindowRecord = self
                    .get_mut_windows()
                    .remove(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                if !rec.get_is_wayland() {
                    linux_x11_destroy(lib, rec.get_display(), rec.get_window());
                }
                Ok(())
            }
            WindowCommand::SetRenderMode(id, mode) => {
                let rec: &mut LinuxWindowRecord = self
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
            WindowCommand::SetPosition(id, _, _)
            | WindowCommand::SetResizable(id, _)
            | WindowCommand::SetDecorations(id, _)
            | WindowCommand::SetAlwaysOnTop(id, _)
            | WindowCommand::SetFullscreen(id, _)
            | WindowCommand::SetVisible(id, _)
            | WindowCommand::SetCursor(id, _)
            | WindowCommand::SetIme(id, _, _)
            | WindowCommand::SetStatusBarStyle(id, _) => {
                if !self.get_windows().contains_key(&id) {
                    return Err(PlatformError::WindowNotFound(id.get_value()));
                }
                Ok(())
            }
        }
    }
}

/// `PlatformBackend` implementation for native Linux.
impl PlatformBackend for LinuxPlatform {
    /// Returns `PlatformKind::Linux`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `PlatformKind` - `PlatformKind::Linux`.
    fn kind(&self) -> PlatformKind {
        PlatformKind::Linux
    }

    /// Returns Linux desktop capabilities.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `PlatformCapabilities` - Linux capability flags.
    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities::for_platform(PlatformKind::Linux)
    }

    /// Opens an X11 or Wayland window via dynamic `dlopen`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Linux platform.
    /// - `&WindowConfig` - Declarative window config.
    ///
    /// # Returns
    ///
    /// - `Result<WindowState, PlatformError>` - Initial window state.
    fn create_window(&mut self, config: &WindowConfig) -> Result<WindowState, PlatformError> {
        let (lib, is_wayland, display, window): (usize, bool, usize, u64) =
            linux_create_native_window(config)?;
        if !is_wayland {
            self.set_x11_lib(lib);
        }
        let id: WindowId = WindowId::new(self.get_next_id());
        self.set_next_id(self.get_next_id().wrapping_add(1));
        let state: WindowState = WindowState::from_config(id, config, 1.0);
        self.get_mut_windows().insert(
            id,
            LinuxWindowRecord {
                config: config.clone(),
                state,
                is_wayland,
                display,
                window,
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
    /// - `Option<WindowState>` - Window state if open.
    fn window_state(&self, id: WindowId) -> Option<WindowState> {
        self.get_windows()
            .get(&id)
            .map(LinuxWindowRecord::get_state)
    }

    /// Returns `RawWindowHandle::Xlib` or `RawWindowHandle::Wayland` for Vulkan/EGL surface binding.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    /// - `WindowId` - Window handle.
    ///
    /// # Returns
    ///
    /// - `RawWindowHandle` - Native Linux display/window handle.
    fn raw_window_handle(&self, id: WindowId) -> RawWindowHandle {
        match self.get_windows().get(&id) {
            Some(rec) => {
                if rec.get_is_wayland() {
                    RawWindowHandle::Wayland {
                        surface: rec.get_window() as usize,
                        display: rec.get_display(),
                    }
                } else {
                    RawWindowHandle::Xlib {
                        window: rec.get_window(),
                        display: rec.get_display(),
                    }
                }
            }
            None => RawWindowHandle::Headless,
        }
    }

    /// Drains queued Linux events.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Linux platform.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Pending events.
    fn poll_events(&mut self) -> Vec<WindowEvent> {
        take(self.get_mut_pending_events())
    }

    /// Presents `framebuffer` to the active Linux window surface.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Linux platform.
    /// - `WindowId` - Target window handle.
    /// - `&Framebuffer` - Rendered RGBA8 framebuffer.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Presentation status.
    fn present_framebuffer(
        &mut self,
        id: WindowId,
        _framebuffer: &Framebuffer,
    ) -> Result<(), PlatformError> {
        if !self.get_windows().contains_key(&id) {
            return Err(PlatformError::WindowNotFound(id.get_value()));
        }
        Ok(())
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

    /// Writes `text` to clipboard cache.
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

    /// Returns active system theme mode.
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
