use super::*;

/// Default `HeadlessPlatform` at 1.0x scale and Light theme.
impl Default for HeadlessPlatform {
    /// Creates a clean offscreen headless platform backend.
    ///
    /// # Returns
    ///
    /// - `Self` - Default `HeadlessPlatform`.
    fn default() -> Self {
        Self {
            emulated_kind: PlatformKind::Headless,
            scale_factor: 1.0,
            safe_area: SafeAreaInsets::default(),
            theme: ThemeMode::Light,
            next_id: 1,
            windows: HashMap::new(),
            pending_events: Vec::new(),
            clipboard: None,
        }
    }
}

/// Configuration and event injection helpers for `HeadlessPlatform`.
impl HeadlessPlatform {
    /// Creates a headless platform emulating a specific target platform's form factor and DPI.
    ///
    /// # Arguments
    ///
    /// - `PlatformKind` - Target platform to emulate.
    /// - `f32` - Simulated DPI scale factor.
    /// - `SafeAreaInsets` - Simulated safe-area insets.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured headless backend.
    #[must_use]
    pub fn emulating(platform: PlatformKind, scale_factor: f32, safe_area: SafeAreaInsets) -> Self {
        let mut backend: Self = Self::default();
        backend.set_emulated_kind(platform);
        backend.set_scale_factor(scale_factor.max(0.5));
        backend.set_safe_area(safe_area);
        backend
    }

    /// Enqueues a synthetic `WindowEvent` to be consumed on the next `poll_events()` call.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable headless platform.
    /// - `WindowEvent` - Synthetic event to inject.
    pub fn push_event(&mut self, event: WindowEvent) {
        self.get_mut_pending_events().push(event);
    }

    /// Returns a reference to the last presented `Framebuffer` for `id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Headless platform reference.
    /// - `WindowId` - Window identifier.
    ///
    /// # Returns
    ///
    /// - `Option<&Framebuffer>` - Most recently presented pixel buffer.
    #[must_use]
    pub fn last_framebuffer(&self, id: WindowId) -> Option<&Framebuffer> {
        self.get_windows()
            .get(&id)
            .map(HeadlessWindowRecord::get_last_frame)
    }
}

/// `WindowController` implementation for `HeadlessPlatform`.
impl WindowController for HeadlessPlatform {
    /// Applies a `WindowCommand` against the in-memory headless window record.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable headless platform.
    /// - `WindowCommand` - Command to apply.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Success or `WindowNotFound`.
    fn apply_command(&mut self, command: WindowCommand) -> Result<(), PlatformError> {
        match command {
            WindowCommand::SetTitle(id, title) => {
                let record: &mut HeadlessWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.get_mut_config().set_title(title);
                Ok(())
            }
            WindowCommand::SetSize(id, width, height) => {
                let scale: f32 = {
                    let record: &mut HeadlessWindowRecord = self
                        .get_mut_windows()
                        .get_mut(&id)
                        .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                    let s: f32 = record.get_state().get_scale_factor();
                    record.get_mut_config().set_width(width.max(1));
                    record.get_mut_config().set_height(height.max(1));
                    record
                        .get_mut_state()
                        .set_logical_width(width.max(1) as f32);
                    record
                        .get_mut_state()
                        .set_logical_height(height.max(1) as f32);
                    record
                        .get_mut_state()
                        .set_physical_width(((width.max(1) as f32) * s).round() as u32);
                    record
                        .get_mut_state()
                        .set_physical_height(((height.max(1) as f32) * s).round() as u32);
                    s
                };
                self.push_event(WindowEvent::Resized(id, width.max(1), height.max(1), scale));
                Ok(())
            }
            WindowCommand::SetPosition(id, x, y) => {
                let record: &mut HeadlessWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.get_mut_config().set_position(Some((x, y)));
                Ok(())
            }
            WindowCommand::SetResizable(id, resizable) => {
                let record: &mut HeadlessWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.get_mut_config().set_resizable(resizable);
                Ok(())
            }
            WindowCommand::SetDecorations(id, decorations) => {
                let record: &mut HeadlessWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.get_mut_config().set_decorations(decorations);
                Ok(())
            }
            WindowCommand::SetAlwaysOnTop(id, on_top) => {
                let record: &mut HeadlessWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.get_mut_config().set_always_on_top(on_top);
                Ok(())
            }
            WindowCommand::SetFullscreen(id, mode) => {
                let record: &mut HeadlessWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.get_mut_config().set_fullscreen(mode);
                record.get_mut_state().set_fullscreen(mode);
                Ok(())
            }
            WindowCommand::SetVisible(id, visible) => {
                let record: &mut HeadlessWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.get_mut_config().set_visible(visible);
                Ok(())
            }
            WindowCommand::SetCursor(id, cursor) => {
                let record: &mut HeadlessWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.set_cursor(cursor);
                Ok(())
            }
            WindowCommand::SetIme(id, enabled, rect) => {
                let record: &mut HeadlessWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.set_ime_active(enabled);
                record.set_ime_rect(rect);
                Ok(())
            }
            WindowCommand::SetStatusBarStyle(id, style) => {
                let record: &mut HeadlessWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.get_mut_config().set_status_bar(style);
                Ok(())
            }
            WindowCommand::RequestRedraw(id) => {
                if !self.get_windows().contains_key(&id) {
                    return Err(PlatformError::WindowNotFound(id.get_value()));
                }
                self.push_event(WindowEvent::RedrawRequested(id));
                Ok(())
            }
            WindowCommand::SetRenderMode(id, mode) => {
                let record: &mut HeadlessWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.get_mut_config().get_mut_render().set_mode(mode);
                Ok(())
            }
            WindowCommand::Close(id) => {
                if self.get_mut_windows().remove(&id).is_none() {
                    return Err(PlatformError::WindowNotFound(id.get_value()));
                }
                Ok(())
            }
        }
    }
}

/// `PlatformBackend` implementation for `HeadlessPlatform`.
impl PlatformBackend for HeadlessPlatform {
    /// Returns the emulated `PlatformKind`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `PlatformKind` - Emulated platform kind.
    fn kind(&self) -> PlatformKind {
        self.get_emulated_kind()
    }

    /// Returns normalized capabilities for the emulated platform.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `PlatformCapabilities` - Capability matrix.
    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities::for_platform(self.get_emulated_kind())
    }

    /// Allocates an offscreen window record with simulated DPI and safe-area insets.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable headless platform.
    /// - `&WindowConfig` - Window configuration.
    ///
    /// # Returns
    ///
    /// - `Result<WindowState, PlatformError>` - Created window state.
    fn create_window(&mut self, config: &WindowConfig) -> Result<WindowState, PlatformError> {
        let id: WindowId = WindowId::new(self.get_next_id());
        self.set_next_id(self.get_next_id().wrapping_add(1));
        let mut state: WindowState = WindowState::from_config(id, config, self.get_scale_factor());
        state.set_safe_area(self.get_safe_area());
        state.set_system_theme(self.get_theme());
        let record: HeadlessWindowRecord = HeadlessWindowRecord {
            config: config.clone(),
            state,
            cursor: CursorIcon::Default,
            last_frame: Framebuffer::allocate(
                state.get_physical_width(),
                state.get_physical_height(),
            ),
            present_count: 0,
            ime_active: false,
            ime_rect: Rect::default(),
        };
        self.get_mut_windows().insert(id, record);
        Ok(state)
    }

    /// Queries current `WindowState` for `id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    /// - `WindowId` - Window handle.
    ///
    /// # Returns
    ///
    /// - `Option<WindowState>` - Window state if alive.
    fn window_state(&self, id: WindowId) -> Option<WindowState> {
        self.get_windows()
            .get(&id)
            .map(HeadlessWindowRecord::get_state)
    }

    /// Returns `RawWindowHandle::Headless`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    /// - `WindowId` - Window handle.
    ///
    /// # Returns
    ///
    /// - `RawWindowHandle` - Headless handle.
    fn raw_window_handle(&self, _id: WindowId) -> RawWindowHandle {
        RawWindowHandle::Headless
    }

    /// Drains injected synthetic events.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable headless platform.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Pending events.
    fn poll_events(&mut self) -> Vec<WindowEvent> {
        take(self.get_mut_pending_events())
    }

    /// Stores a copy of `framebuffer` inside the headless window record for pixel inspection.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable headless platform.
    /// - `WindowId` - Target window handle.
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
        let record: &mut HeadlessWindowRecord = self
            .get_mut_windows()
            .get_mut(&id)
            .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
        record.set_last_frame(framebuffer.clone());
        record.set_present_count(record.get_present_count().wrapping_add(1));
        Ok(())
    }

    /// Reads the simulated clipboard string.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Stored clipboard contents.
    fn read_clipboard(&self) -> Option<String> {
        let Self { clipboard, .. } = self;
        clipboard.clone()
    }

    /// Writes `text` into the simulated clipboard.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable platform.
    /// - `&str` - Clipboard text.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Always succeeds.
    fn write_clipboard(&mut self, text: &str) -> Result<(), PlatformError> {
        self.set_clipboard(Some(text.to_string()));
        Ok(())
    }

    /// Returns the simulated OS theme mode.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `ThemeMode` - Simulated theme mode.
    fn system_theme(&self) -> ThemeMode {
        self.get_theme()
    }
}
