use super::*;

/// Default initialization for `MacOsPlatform`.
impl Default for MacOsPlatform {
    /// Creates an empty macOS native platform backend.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized macOS backend.
    fn default() -> Self {
        let initial_theme: ThemeMode = macos_system_theme();
        let initial_low_power: bool = macos_low_power_mode();
        let initial_pb: isize = macos_pasteboard_change_count();
        Self {
            next_id: 1,
            windows: HashMap::new(),
            pending_events: Vec::new(),
            clipboard_cache: None,
            cached_color_space: 0,
            last_app_active: true,
            last_miniaturized: false,
            last_low_power: initial_low_power,
            last_pasteboard_count: initial_pb,
            last_theme: initial_theme,
        }
    }
}

/// AppKit system event injection helpers for `MacOsPlatform`.
impl MacOsPlatform {
    /// Enqueues a normalized macOS `SystemEvent` to be drained on the next `poll_events()` tick.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable macOS platform backend.
    /// - `SystemEvent` - System event to enqueue.
    pub fn push_system_event(&mut self, event: SystemEvent) {
        self.get_mut_pending_events()
            .push(WindowEvent::System(event));
    }
}

/// `WindowController` implementation translating commands directly to AppKit calls.
impl WindowController for MacOsPlatform {
    /// Applies `command` directly onto the native `NSWindow` via `objc_msgSend`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable macOS platform backend.
    /// - `WindowCommand` - Command to execute.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Execution status.
    fn apply_command(&mut self, command: WindowCommand) -> Result<(), PlatformError> {
        match command {
            WindowCommand::SetTitle(id, title) => {
                let record: &mut MacOsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                let ns_title: *mut c_void = macos_ns_string(&title);
                if record.get_ns_window() != 0 && !ns_title.is_null() {
                    let _: *mut c_void = objc_send_ptr1(
                        record.get_ns_window() as *mut c_void,
                        objc_sel(OBJC_SEL_SET_TITLE),
                        ns_title,
                    );
                }
                record.get_mut_config().set_title(title);
                Ok(())
            }
            WindowCommand::SetSize(id, width, height) => {
                let record: &mut MacOsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                let w: u32 = width.max(1);
                let h: u32 = height.max(1);
                if record.get_ns_window() != 0 {
                    objc_send_void_size(
                        record.get_ns_window() as *mut c_void,
                        objc_sel(OBJC_SEL_SET_CONTENT_SIZE),
                        CgSize {
                            width: w as f64,
                            height: h as f64,
                        },
                    );
                }
                let scale: f32 = record.get_state().get_scale_factor();
                record.get_mut_config().set_width(w);
                record.get_mut_config().set_height(h);
                record.get_mut_state().set_logical_width(w as f32);
                record.get_mut_state().set_logical_height(h as f32);
                record
                    .get_mut_state()
                    .set_physical_width(((w as f32) * scale).round() as u32);
                record
                    .get_mut_state()
                    .set_physical_height(((h as f32) * scale).round() as u32);
                Ok(())
            }
            WindowCommand::SetAlwaysOnTop(id, on_top) => {
                let record: &mut MacOsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                if record.get_ns_window() != 0 {
                    let level: isize = if on_top {
                        NS_FLOATING_WINDOW_LEVEL
                    } else {
                        NS_NORMAL_WINDOW_LEVEL
                    };
                    objc_send_void_isize(
                        record.get_ns_window() as *mut c_void,
                        objc_sel(OBJC_SEL_SET_LEVEL),
                        level,
                    );
                }
                record.get_mut_config().set_always_on_top(on_top);
                Ok(())
            }
            WindowCommand::SetVisible(id, visible) => {
                let record: &mut MacOsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                if record.get_ns_window() != 0 {
                    let sel_name: &'static str = if visible {
                        OBJC_SEL_MAKE_KEY_FRONT
                    } else {
                        OBJC_SEL_ORDER_OUT
                    };
                    let _: *mut c_void = objc_send_ptr1(
                        record.get_ns_window() as *mut c_void,
                        objc_sel(sel_name),
                        null_mut(),
                    );
                }
                record.get_mut_config().set_visible(visible);
                Ok(())
            }
            WindowCommand::SetFullscreen(id, mode) => {
                let record: &mut MacOsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                let was_fs: bool = record.get_state().get_fullscreen() != FullscreenMode::Windowed;
                let want_fs: bool = mode != FullscreenMode::Windowed;
                if record.get_ns_window() != 0 && was_fs != want_fs {
                    let _: *mut c_void = objc_send_ptr1(
                        record.get_ns_window() as *mut c_void,
                        objc_sel(OBJC_SEL_TOGGLE_FULLSCREEN),
                        null_mut(),
                    );
                }
                record.get_mut_config().set_fullscreen(mode);
                record.get_mut_state().set_fullscreen(mode);
                Ok(())
            }
            WindowCommand::Close(id) => {
                let record: MacOsWindowRecord = self
                    .get_mut_windows()
                    .remove(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                if record.get_ns_window() != 0 {
                    let _: *mut c_void = objc_send_ptr0(
                        record.get_ns_window() as *mut c_void,
                        objc_sel(OBJC_SEL_CLOSE),
                    );
                }
                Ok(())
            }
            WindowCommand::SetDecorations(id, decorations) => {
                let record: &mut MacOsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                let mask: isize = if decorations {
                    (NS_WINDOW_STYLE_TITLED
                        | NS_WINDOW_STYLE_CLOSABLE
                        | NS_WINDOW_STYLE_MINIATURIZABLE
                        | NS_WINDOW_STYLE_RESIZABLE) as isize
                } else {
                    NS_WINDOW_STYLE_BORDERLESS as isize
                };
                if record.get_ns_window() != 0 {
                    objc_send_void_isize(
                        record.get_ns_window() as *mut c_void,
                        objc_sel(OBJC_SEL_SET_STYLE_MASK),
                        mask,
                    );
                }
                record.get_mut_config().set_decorations(decorations);
                Ok(())
            }
            WindowCommand::SetPosition(id, x, y) => {
                let record: &mut MacOsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.get_mut_config().set_position(Some((x, y)));
                Ok(())
            }
            WindowCommand::SetResizable(id, resizable) => {
                let record: &mut MacOsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.get_mut_config().set_resizable(resizable);
                Ok(())
            }
            WindowCommand::SetRenderMode(id, mode) => {
                let record: &mut MacOsWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                record.get_mut_config().get_mut_render().set_mode(mode);
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
            WindowCommand::SetCursor(id, cursor) => {
                if !self.get_windows().contains_key(&id) {
                    return Err(PlatformError::WindowNotFound(id.get_value()));
                }
                let cls: *mut c_void = objc_class(OBJC_CLS_NS_CURSOR);
                if !cls.is_null() {
                    let sel_name: &'static str = match cursor {
                        CursorIcon::Pointer => OBJC_SEL_POINTING_HAND_CURSOR,
                        CursorIcon::Text => OBJC_SEL_IBEAM_CURSOR,
                        CursorIcon::Grab => OBJC_SEL_OPEN_HAND_CURSOR,
                        CursorIcon::Grabbing => OBJC_SEL_CLOSED_HAND_CURSOR,
                        CursorIcon::NotAllowed => OBJC_SEL_NOT_ALLOWED_CURSOR,
                        CursorIcon::Default
                        | CursorIcon::Crosshair
                        | CursorIcon::Move
                        | CursorIcon::EwResize
                        | CursorIcon::NsResize => OBJC_SEL_ARROW_CURSOR,
                    };
                    let ns_cursor: *mut c_void = objc_send_ptr0(cls, objc_sel(sel_name));
                    let _: *mut c_void = objc_send_ptr0(ns_cursor, objc_sel(OBJC_SEL_SET_CURSOR));
                }
                Ok(())
            }
            WindowCommand::SetIme(id, _, _) | WindowCommand::SetStatusBarStyle(id, _) => {
                if !self.get_windows().contains_key(&id) {
                    return Err(PlatformError::WindowNotFound(id.get_value()));
                }
                Ok(())
            }
        }
    }
}

/// `PlatformBackend` implementation for native macOS.
impl PlatformBackend for MacOsPlatform {
    /// Returns `PlatformKind::MacOs`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform backend reference.
    ///
    /// # Returns
    ///
    /// - `PlatformKind` - `PlatformKind::MacOs`.
    fn kind(&self) -> PlatformKind {
        PlatformKind::MacOs
    }

    /// Returns macOS desktop capability flags.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform backend reference.
    ///
    /// # Returns
    ///
    /// - `PlatformCapabilities` - macOS capabilities.
    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities::for_platform(PlatformKind::MacOs)
    }

    /// Creates a real AppKit `NSWindow` + `CAMetalLayer`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable macOS backend.
    /// - `&WindowConfig` - Declarative window settings.
    ///
    /// # Returns
    ///
    /// - `Result<WindowState, PlatformError>` - Initial window state.
    fn create_window(&mut self, config: &WindowConfig) -> Result<WindowState, PlatformError> {
        let (ns_window, ns_view, metal_layer): (usize, usize, usize) =
            macos_create_native_window(config)?;
        let id: WindowId = WindowId::new(self.get_next_id());
        self.set_next_id(self.get_next_id().wrapping_add(1));
        let scale: f32 = macos_backing_scale_factor();
        let mut state: WindowState = WindowState::from_config(id, config, scale);
        state.set_system_theme(macos_system_theme());
        if config.get_decorations() {
            state.set_safe_area(SafeAreaInsets::new(28.0, 0.0, 0.0, 0.0));
        }
        self.get_mut_windows().insert(
            id,
            MacOsWindowRecord {
                config: config.clone(),
                state,
                ns_window,
                ns_view,
                metal_layer,
            },
        );
        Ok(state)
    }

    /// Returns live `WindowState` for `id`.
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
            .map(MacOsWindowRecord::get_state)
    }

    /// Returns `RawWindowHandle::AppKit` carrying `NSWindow`, `NSView`, and `CAMetalLayer` pointers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    /// - `WindowId` - Window handle.
    ///
    /// # Returns
    ///
    /// - `RawWindowHandle` - Native AppKit pointers.
    fn raw_window_handle(&self, id: WindowId) -> RawWindowHandle {
        match self.get_windows().get(&id) {
            Some(rec) => RawWindowHandle::AppKit {
                ns_window: rec.get_ns_window(),
                ns_view: rec.get_ns_view(),
                metal_layer: rec.get_metal_layer(),
            },
            None => RawWindowHandle::Headless,
        }
    }

    /// Pumps pending AppKit events, detects closed windows, and drains queued macOS window events.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable macOS platform.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Pending events.
    fn poll_events(&mut self) -> Vec<WindowEvent> {
        let mut pumped: Vec<WindowEvent> = Vec::new();
        macos_pump_events(self.get_windows(), &mut pumped);
        let mut resized_events: Vec<(WindowId, u32, u32, f32)> = Vec::new();
        let mut focus_events: Vec<(WindowId, bool)> = Vec::new();
        let mut closed_ids: Vec<WindowId> = Vec::new();
        let mut any_miniaturized: bool = false;
        let mut primary_wid: Option<WindowId> = None;
        for (id, rec) in self.get_mut_windows().iter_mut() {
            if primary_wid.is_none() {
                primary_wid = Some(*id);
            }
            let win_ptr: *mut c_void = rec.get_ns_window() as *mut c_void;
            if !win_ptr.is_null() {
                let is_mini: bool = objc_send_bool0(win_ptr, objc_sel(OBJC_SEL_IS_MINIATURIZED));
                if is_mini {
                    any_miniaturized = true;
                    continue;
                }
                if rec.get_config().get_visible()
                    && !objc_send_bool0(win_ptr, objc_sel(OBJC_SEL_IS_VISIBLE))
                {
                    closed_ids.push(*id);
                    continue;
                }
                let is_key: bool = objc_send_bool0(win_ptr, objc_sel(OBJC_SEL_IS_KEY_WINDOW));
                if is_key != rec.get_state().get_focused() {
                    rec.get_mut_state().set_focused(is_key);
                    focus_events.push((*id, is_key));
                }
            }
            let view_ptr: *mut c_void = rec.get_ns_view() as *mut c_void;
            if let Some((w, h)) = macos_view_logical_size(view_ptr) {
                let prev_w: u32 = rec.get_state().get_logical_width().round() as u32;
                let prev_h: u32 = rec.get_state().get_logical_height().round() as u32;
                let prev_scale: f32 = rec.get_state().get_scale_factor();
                let scale: f32 = macos_window_backing_scale(win_ptr);
                if w != prev_w || h != prev_h || (scale - prev_scale).abs() > 0.01 {
                    let layer_ptr: *mut c_void = rec.get_metal_layer() as *mut c_void;
                    if !layer_ptr.is_null() && (scale - prev_scale).abs() > 0.01 {
                        objc_send_void_f64(
                            layer_ptr,
                            objc_sel(OBJC_SEL_SET_CONTENTS_SCALE),
                            scale as f64,
                        );
                    }
                    rec.get_mut_state().set_logical_width(w as f32);
                    rec.get_mut_state().set_logical_height(h as f32);
                    rec.get_mut_state().set_scale_factor(scale);
                    rec.get_mut_state()
                        .set_physical_width(((w as f32) * scale).round() as u32);
                    rec.get_mut_state()
                        .set_physical_height(((h as f32) * scale).round() as u32);
                    resized_events.push((*id, w, h, scale));
                }
            }
        }
        if any_miniaturized != self.get_last_miniaturized() {
            self.set_last_miniaturized(any_miniaturized);
            let lc_ev: SystemEvent = if any_miniaturized {
                SystemEvent::Background
            } else {
                SystemEvent::Foreground
            };
            self.get_mut_pending_events()
                .push(WindowEvent::System(lc_ev));
        }
        let app_active: bool = macos_app_is_active();
        if app_active != self.get_last_app_active() {
            self.set_last_app_active(app_active);
            if !any_miniaturized {
                let act_ev: SystemEvent = if app_active {
                    SystemEvent::Foreground
                } else {
                    SystemEvent::Inactive
                };
                self.get_mut_pending_events()
                    .push(WindowEvent::System(act_ev));
            }
        }
        let cur_theme: ThemeMode = macos_system_theme();
        if cur_theme != self.get_last_theme() {
            self.set_last_theme(cur_theme);
            if let Some(wid) = primary_wid {
                self.get_mut_pending_events()
                    .push(WindowEvent::ThemeChanged(wid, cur_theme));
            } else {
                self.get_mut_pending_events()
                    .push(WindowEvent::System(SystemEvent::ThemeChanged(cur_theme)));
            }
        }
        let cur_low_power: bool = macos_low_power_mode();
        if cur_low_power != self.get_last_low_power() {
            self.set_last_low_power(cur_low_power);
            self.get_mut_pending_events().push(WindowEvent::System(
                SystemEvent::LowPowerModeChanged(cur_low_power),
            ));
        }
        let cur_pb: isize = macos_pasteboard_change_count();
        let prev_pb: isize = self.get_last_pasteboard_count();
        if prev_pb >= 0 && cur_pb != prev_pb {
            self.get_mut_pending_events()
                .push(WindowEvent::System(SystemEvent::ClipboardChanged));
        }
        self.set_last_pasteboard_count(cur_pb);
        for (id, focused) in focus_events.into_iter() {
            self.get_mut_pending_events()
                .push(WindowEvent::Focused(id, focused));
        }
        for (id, w, h, scale) in resized_events.into_iter() {
            self.get_mut_pending_events()
                .push(WindowEvent::Resized(id, w, h, scale));
        }
        self.get_mut_pending_events().extend(pumped);
        for id in closed_ids.into_iter() {
            self.get_mut_pending_events()
                .push(WindowEvent::CloseRequested(id));
        }
        take(self.get_mut_pending_events())
    }

    /// Blits the RGBA8 `Framebuffer` directly onto the window's `CALayer` via CoreGraphics.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable macOS platform.
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
        let layer_ptr: usize = self
            .get_windows()
            .get(&id)
            .map(MacOsWindowRecord::get_metal_layer)
            .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
        let mut color_space: usize = self.get_cached_color_space();
        if color_space == 0 {
            color_space = macos_create_device_rgb_space();
            self.set_cached_color_space(color_space);
        }
        macos_present_cg_image(layer_ptr, color_space, framebuffer)
    }

    /// Reads cached clipboard text.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Clipboard text if present.
    fn read_clipboard(&self) -> Option<String> {
        let Self {
            clipboard_cache, ..
        } = self;
        clipboard_cache.clone()
    }

    /// Writes `text` into the clipboard cache.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable macOS platform.
    /// - `&str` - Text to store.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Write status.
    fn write_clipboard(&mut self, text: &str) -> Result<(), PlatformError> {
        self.set_clipboard_cache(Some(text.to_string()));
        let _: bool = write_system_clipboard(text);
        Ok(())
    }

    /// Queries macOS dark/light appearance directly from AppKit.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `ThemeMode` - Active macOS system appearance.
    fn system_theme(&self) -> ThemeMode {
        macos_system_theme()
    }

    /// Queries the native macOS display refresh rate via `NSScreen.maximumFramesPerSecond`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `f32` - Active display refresh rate in Hertz.
    fn display_refresh_rate_hz(&self) -> f32 {
        let win_ptr: *mut c_void = self
            .get_windows()
            .values()
            .next()
            .map(|rec: &MacOsWindowRecord| rec.get_ns_window() as *mut c_void)
            .unwrap_or(null_mut());
        macos_window_refresh_rate_hz(win_ptr)
    }
}
