use super::*;

/// Default initialization for `WindowsPlatform`.
impl Default for WindowsPlatform {
    /// Creates an empty Windows Win32 platform backend.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized Windows platform.
    fn default() -> Self {
        Self {
            next_id: 1,
            class_registered: false,
            windows: HashMap::new(),
            pending_events: Vec::new(),
            clipboard_cache: None,
        }
    }
}

/// `WindowController` implementation translating commands directly to Win32 APIs.
impl WindowController for WindowsPlatform {
    /// Applies a `WindowCommand` onto the target `HWND`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Windows platform.
    /// - `WindowCommand` - Command to apply.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Execution status.
    fn apply_command(&mut self, command: WindowCommand) -> Result<(), PlatformError> {
        match command {
            WindowCommand::SetTitle(id, title) => {
                let rec: &mut WindowsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                win32_set_title(rec.get_hwnd(), &title);
                rec.get_mut_config().set_title(title);
                Ok(())
            }
            WindowCommand::SetSize(id, width, height) => {
                let rec: &mut WindowsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                let w: u32 = width.max(1);
                let h: u32 = height.max(1);
                win32_set_size(rec.get_hwnd(), w, h);
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
            WindowCommand::SetAlwaysOnTop(id, on_top) => {
                let rec: &mut WindowsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                win32_set_topmost(rec.get_hwnd(), on_top);
                rec.get_mut_config().set_always_on_top(on_top);
                Ok(())
            }
            WindowCommand::SetVisible(id, visible) => {
                let rec: &mut WindowsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                win32_set_visible(rec.get_hwnd(), visible);
                rec.get_mut_config().set_visible(visible);
                Ok(())
            }
            WindowCommand::Close(id) => {
                let rec: WindowsWindowRecord = self
                    .get_mut_windows()
                    .remove(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                win32_destroy(rec.get_hwnd());
                Ok(())
            }
            WindowCommand::SetRenderMode(id, mode) => {
                let rec: &mut WindowsWindowRecord = self
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
            | WindowCommand::SetFullscreen(id, _)
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

/// `PlatformBackend` implementation for native Windows.
impl PlatformBackend for WindowsPlatform {
    /// Returns `PlatformKind::Windows`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `PlatformKind` - `PlatformKind::Windows`.
    fn kind(&self) -> PlatformKind {
        PlatformKind::Windows
    }

    /// Returns Windows desktop capabilities.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `PlatformCapabilities` - Windows capability flags.
    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities::for_platform(PlatformKind::Windows)
    }

    /// Creates a real Win32 `HWND`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Windows platform.
    /// - `&WindowConfig` - Declarative window config.
    ///
    /// # Returns
    ///
    /// - `Result<WindowState, PlatformError>` - Initial window state.
    fn create_window(&mut self, config: &WindowConfig) -> Result<WindowState, PlatformError> {
        let mut registered: bool = self.get_class_registered();
        let (hwnd, hinstance, scale): (usize, usize, f32) =
            win32_create_window(config, &mut registered)?;
        self.set_class_registered(registered);
        let id: WindowId = WindowId::new(self.get_next_id());
        self.set_next_id(self.get_next_id().wrapping_add(1));
        let state: WindowState = WindowState::from_config(id, config, scale);
        self.get_mut_windows().insert(
            id,
            WindowsWindowRecord {
                config: config.clone(),
                state,
                hwnd,
                hinstance,
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
            .map(WindowsWindowRecord::get_state)
    }

    /// Returns `RawWindowHandle::Win32` for Direct3D / Vulkan surface binding.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    /// - `WindowId` - Window handle.
    ///
    /// # Returns
    ///
    /// - `RawWindowHandle` - Win32 handle pair.
    fn raw_window_handle(&self, id: WindowId) -> RawWindowHandle {
        match self.get_windows().get(&id) {
            Some(rec) => RawWindowHandle::Win32 {
                hwnd: rec.get_hwnd(),
                hinstance: rec.get_hinstance(),
            },
            None => RawWindowHandle::Headless,
        }
    }

    /// Pumps Win32 messages and drains pending events.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Windows platform.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Pending events.
    fn poll_events(&mut self) -> Vec<WindowEvent> {
        let Self {
            windows,
            pending_events,
            ..
        } = self;
        win32_pump_messages(windows, pending_events);
        take(pending_events)
    }

    /// Blits `framebuffer` directly to the Win32 `HWND` via GDI `StretchDIBits`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable Windows platform.
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
        let hwnd: usize = self
            .get_windows()
            .get(&id)
            .map(WindowsWindowRecord::get_hwnd)
            .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
        win32_present_dib(hwnd, framebuffer)
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

    /// Returns default system theme mode.
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
