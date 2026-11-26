use super::*;

/// Default initialization for `WebPlatform`.
impl Default for WebPlatform {
    /// Creates an empty browser WebAssembly platform backend.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized web platform.
    fn default() -> Self {
        Self {
            next_id: 1,
            windows: HashMap::new(),
            pending_events: Vec::new(),
            clipboard_cache: None,
        }
    }
}

/// `WindowController` implementation for Browser WebAssembly.
impl WindowController for WebPlatform {
    /// Applies a `WindowCommand` against the browser document and `HTMLCanvasElement`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable web platform.
    /// - `WindowCommand` - Command to apply.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Execution status.
    fn apply_command(&mut self, command: WindowCommand) -> Result<(), PlatformError> {
        match command {
            WindowCommand::SetTitle(id, title) => {
                let rec: &mut WebWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                if let Some(doc) = window().and_then(|w: Window| w.document()) {
                    doc.set_title(&title);
                }
                rec.get_mut_config().set_title(title);
                Ok(())
            }
            WindowCommand::SetSize(id, width, height) => {
                let rec: &mut WebWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                let w: u32 = width.max(1);
                let h: u32 = height.max(1);
                let live_dpr: f32 = window()
                    .map(|bw: Window| (bw.device_pixel_ratio() as f32).clamp(0.5, 4.5))
                    .unwrap_or_else(|| rec.get_state().get_scale_factor().max(1.0));
                rec.get_mut_state().set_scale_factor(live_dpr);
                let dpr_f64: f64 = live_dpr as f64;
                let (css_w, css_h): (f64, f64) = {
                    let WebWindowRecord { canvas, .. } = rec;
                    if let Some(el) = canvas.as_ref() {
                        let rect: DomRect = el.get_bounding_client_rect();
                        if rect.width() >= 1.0
                            && rect.height() >= 1.0
                            && (rect.width() - (w as f64)).abs() <= 1.5
                            && (rect.height() - (h as f64)).abs() <= 1.5
                        {
                            (rect.width(), rect.height())
                        } else {
                            (w as f64, h as f64)
                        }
                    } else {
                        (w as f64, h as f64)
                    }
                };
                let phys_w: u32 = (css_w * dpr_f64).round().max(1.0) as u32;
                let phys_h: u32 = (css_h * dpr_f64).round().max(1.0) as u32;
                let WebWindowRecord { canvas, .. } = rec;
                if let Some(el) = canvas.as_ref() {
                    if el.width() != phys_w {
                        el.set_width(phys_w);
                    }
                    if el.height() != phys_h {
                        el.set_height(phys_h);
                    }
                    let css_style: String = format!(
                        "{WEB_CANVAS_STYLE_PREFIX}{WEB_CURSOR_POINTER}{WEB_CANVAS_STYLE_SUFFIX}"
                    );
                    drop(el.set_attribute(WEB_ATTR_STYLE, &css_style));
                }
                rec.get_mut_config().set_width(w);
                rec.get_mut_config().set_height(h);
                rec.get_mut_state()
                    .set_logical_width((phys_w as f32) / live_dpr);
                rec.get_mut_state()
                    .set_logical_height((phys_h as f32) / live_dpr);
                rec.get_mut_state().set_physical_width(phys_w);
                rec.get_mut_state().set_physical_height(phys_h);
                Ok(())
            }
            WindowCommand::SetRenderMode(id, mode) => {
                let rec: &mut WebWindowRecord = self
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
                let rec: WebWindowRecord = self
                    .get_mut_windows()
                    .remove(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                let WebWindowRecord { canvas, .. } = rec;
                if let Some(el) = canvas {
                    el.remove();
                }
                Ok(())
            }
            WindowCommand::SetCursor(id, cursor) => {
                let rec: &WebWindowRecord = self
                    .get_windows()
                    .get(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                let css_cursor: &str = match cursor {
                    CursorIcon::Default => WEB_CURSOR_DEFAULT,
                    CursorIcon::Pointer => WEB_CURSOR_POINTER,
                    CursorIcon::Text => WEB_CURSOR_TEXT,
                    CursorIcon::Crosshair => WEB_CURSOR_CROSSHAIR,
                    CursorIcon::NotAllowed => WEB_CURSOR_NOT_ALLOWED,
                    CursorIcon::Grab => WEB_CURSOR_GRAB,
                    CursorIcon::Grabbing => WEB_CURSOR_GRABBING,
                    CursorIcon::Move => WEB_CURSOR_MOVE,
                    CursorIcon::EwResize => WEB_CURSOR_EW_RESIZE,
                    CursorIcon::NsResize => WEB_CURSOR_NS_RESIZE,
                };
                let WebWindowRecord { canvas, .. } = rec;
                if let Some(el) = canvas.as_ref() {
                    let css_style: String =
                        format!("{WEB_CANVAS_STYLE_PREFIX}{css_cursor}{WEB_CANVAS_STYLE_SUFFIX}");
                    drop(el.set_attribute(WEB_ATTR_STYLE, &css_style));
                }
                Ok(())
            }
            WindowCommand::SetPosition(id, _, _)
            | WindowCommand::SetResizable(id, _)
            | WindowCommand::SetDecorations(id, _)
            | WindowCommand::SetAlwaysOnTop(id, _)
            | WindowCommand::SetFullscreen(id, _)
            | WindowCommand::SetVisible(id, _)
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

/// `PlatformBackend` implementation for Browser WebAssembly.
impl PlatformBackend for WebPlatform {
    /// Returns `PlatformKind::Web`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `PlatformKind` - `PlatformKind::Web`.
    fn kind(&self) -> PlatformKind {
        PlatformKind::Web
    }

    /// Returns browser capability flags.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `PlatformCapabilities` - Web capability flags.
    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities::for_platform(PlatformKind::Web)
    }

    /// Creates a browser canvas surface configured with device pixel ratio and dark mode detection.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable web platform.
    /// - `&WindowConfig` - Declarative window config.
    ///
    /// # Returns
    ///
    /// - `Result<WindowState, PlatformError>` - Initial window state.
    fn create_window(&mut self, config: &WindowConfig) -> Result<WindowState, PlatformError> {
        let dpr: f32 = web_device_pixel_ratio();
        let (canvas, ctx2d): (HtmlCanvasElement, CanvasRenderingContext2d) =
            web_create_canvas(config, dpr)?;
        let raw_id: u64 = self.get_next_id();
        let id: WindowId = WindowId::new(raw_id);
        self.set_next_id(raw_id.wrapping_add(1));
        let mut state: WindowState = WindowState::from_config(id, config, dpr);
        let phys_w: u32 = canvas.width().max(1);
        let phys_h: u32 = canvas.height().max(1);
        state.set_physical_width(phys_w);
        state.set_physical_height(phys_h);
        state.set_logical_width((phys_w as f32) / dpr);
        state.set_logical_height((phys_h as f32) / dpr);
        state.set_system_theme(web_system_theme());
        self.get_mut_windows().insert(
            id,
            WebWindowRecord {
                config: config.clone(),
                state,
                canvas_id: raw_id as u32,
                canvas: Some(canvas),
                ctx2d: Some(ctx2d),
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
        self.get_windows().get(&id).map(WebWindowRecord::get_state)
    }

    /// Returns `RawWindowHandle::WebCanvas` for WebGPU / WebGL2 surface binding.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    /// - `WindowId` - Window handle.
    ///
    /// # Returns
    ///
    /// - `RawWindowHandle` - Web canvas handle.
    fn raw_window_handle(&self, id: WindowId) -> RawWindowHandle {
        match self.get_windows().get(&id) {
            Some(rec) => RawWindowHandle::WebCanvas {
                canvas_id: rec.get_canvas_id(),
            },
            None => RawWindowHandle::Headless,
        }
    }

    /// Drains queued web events.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable web platform.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Pending events.
    fn poll_events(&mut self) -> Vec<WindowEvent> {
        take(self.get_mut_pending_events())
    }

    /// Blits `framebuffer` directly onto the browser `CanvasRenderingContext2d`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable web platform.
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
        let rec: &WebWindowRecord = self
            .get_windows()
            .get(&id)
            .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
        let WebWindowRecord { ctx2d, .. } = rec;
        if let Some(ctx) = ctx2d.as_ref() {
            web_present_image_data(ctx, framebuffer)?;
        }
        Ok(())
    }

    /// Blits only the damaged physical rectangle of `framebuffer` onto the browser `CanvasRenderingContext2d`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable web platform.
    /// - `WindowId` - Window handle.
    /// - `&Framebuffer` - Rendered RGBA8 framebuffer.
    /// - `DamageRegion` - Damaged physical pixel region.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Presentation status.
    fn present_framebuffer_region(
        &mut self,
        id: WindowId,
        framebuffer: &Framebuffer,
        damage: DamageRegion,
    ) -> Result<(), PlatformError> {
        let rec: &WebWindowRecord = self
            .get_windows()
            .get(&id)
            .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
        let WebWindowRecord { ctx2d, .. } = rec;
        if let Some(ctx) = ctx2d.as_ref() {
            web_present_image_data_region(ctx, framebuffer, damage)?;
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

    /// Writes `text` into clipboard cache.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable web platform.
    /// - `&str` - Clipboard text.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Write status.
    fn write_clipboard(&mut self, text: &str) -> Result<(), PlatformError> {
        self.set_clipboard_cache(Some(text.to_string()));
        Ok(())
    }

    /// Queries the browser's live dark/light appearance preference.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `ThemeMode` - Active appearance mode.
    fn system_theme(&self) -> ThemeMode {
        web_system_theme()
    }
}
