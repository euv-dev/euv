use super::*;

/// Default initialization detecting iPhone (`Ios`) vs iPad (`IpadOs`) at runtime via UIKit.
impl Default for IosPlatform {
    /// Creates an initialized iOS / iPadOS UIKit platform backend.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured UIKit backend.
    fn default() -> Self {
        Self {
            resolved_kind: ios_detect_platform_kind(),
            next_id: 1,
            windows: HashMap::new(),
            pending_events: Vec::new(),
            clipboard_cache: None,
            display_link: 0,
            pump_paused: false,
        }
    }
}

/// UIKit system event injection and display-link lifecycle helpers for `IosPlatform`.
impl IosPlatform {
    /// Attaches a host `CADisplayLink*` pointer so background transitions pause it automatically with zero CPU/GPU wakeups.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable iOS/iPadOS platform backend.
    /// - `usize` - Raw `CADisplayLink*` pointer (`0` to detach).
    pub fn attach_display_link(&mut self, display_link: usize) {
        self.set_display_link(display_link);
        if display_link != 0 {
            uikit_set_display_link_paused(display_link, self.get_pump_paused());
        }
    }

    /// Enqueues a normalized UIKit `SystemEvent` to be drained on the next `poll_events()` tick.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable iOS/iPadOS platform backend.
    /// - `SystemEvent` - System event forwarded from `UIApplicationDelegate` / `UISceneDelegate`.
    pub fn push_system_event(&mut self, event: SystemEvent) {
        match &event {
            SystemEvent::Background => self.set_frame_pump_paused(true),
            SystemEvent::Foreground => self.set_frame_pump_paused(false),
            _ => {}
        }
        self.get_mut_pending_events()
            .push(WindowEvent::System(event));
    }
}

/// `WindowController` implementation for iOS and iPadOS.
impl WindowController for IosPlatform {
    /// Applies a `WindowCommand` against the active UIKit window scene.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable iOS/iPadOS platform.
    /// - `WindowCommand` - Command to execute.
    ///
    /// # Returns
    ///
    /// - `Result<(), PlatformError>` - Command execution result.
    fn apply_command(&mut self, command: WindowCommand) -> Result<(), PlatformError> {
        match command {
            WindowCommand::SetTitle(id, title) => {
                let rec: &mut IosWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                rec.get_mut_config().set_title(title);
                Ok(())
            }
            WindowCommand::SetSize(id, width, height) => {
                let rec: &mut IosWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                let scale: f32 = rec.get_state().get_scale_factor();
                rec.get_mut_state().set_logical_width(width.max(1) as f32);
                rec.get_mut_state().set_logical_height(height.max(1) as f32);
                rec.get_mut_state()
                    .set_physical_width(((width.max(1) as f32) * scale).round() as u32);
                rec.get_mut_state()
                    .set_physical_height(((height.max(1) as f32) * scale).round() as u32);
                Ok(())
            }
            WindowCommand::SetStatusBarStyle(id, style) => {
                let rec: &mut IosWindowRecord = self
                    .get_mut_windows()
                    .get_mut(&id)
                    .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
                rec.get_mut_config().set_status_bar(style);
                Ok(())
            }
            WindowCommand::SetRenderMode(id, mode) => {
                let rec: &mut IosWindowRecord = self
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
            WindowCommand::SetPosition(id, _, _)
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

/// `PlatformBackend` implementation covering both iOS Phone and iPadOS Tablet scenes.
impl PlatformBackend for IosPlatform {
    /// Returns `PlatformKind::IpadOs` on iPad or `PlatformKind::Ios` on iPhone.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform backend reference.
    ///
    /// # Returns
    ///
    /// - `PlatformKind` - Detected Apple mobile/tablet platform.
    fn kind(&self) -> PlatformKind {
        self.get_resolved_kind()
    }

    /// Returns capability flags reflecting iPadOS (multi-window, hover, keyboard, Apple Pencil) vs iOS.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform backend reference.
    ///
    /// # Returns
    ///
    /// - `PlatformCapabilities` - Normalized capability flags.
    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities::for_platform(self.get_resolved_kind())
    }

    /// Creates a UIKit window scene with safe-area insets tailored to iPhone vs iPadOS.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable iOS/iPadOS platform.
    /// - `&WindowConfig` - Declarative window config.
    ///
    /// # Returns
    ///
    /// - `Result<WindowState, PlatformError>` - Initial window state.
    fn create_window(&mut self, config: &WindowConfig) -> Result<WindowState, PlatformError> {
        let (ui_window, ui_view, metal_layer): (usize, usize, usize) = ios_create_native_handles();
        let is_ipad: bool = self.get_resolved_kind() == PlatformKind::IpadOs;
        let id: WindowId = WindowId::new(self.get_next_id());
        self.set_next_id(self.get_next_id().wrapping_add(1));
        let scale: f32 = ios_screen_scale();
        let mut state: WindowState = WindowState::from_config(id, config, scale);
        state.set_system_theme(ios_system_theme());
        let safe_area: SafeAreaInsets = if is_ipad {
            SafeAreaInsets::new(
                IPADOS_DEFAULT_SAFE_TOP,
                0.0,
                IPADOS_DEFAULT_SAFE_BOTTOM,
                0.0,
            )
        } else {
            SafeAreaInsets::new(IOS_DEFAULT_SAFE_TOP, 0.0, IOS_DEFAULT_SAFE_BOTTOM, 0.0)
        };
        state.set_safe_area(safe_area);
        self.get_mut_windows().insert(
            id,
            IosWindowRecord {
                config: config.clone(),
                state,
                ui_window,
                ui_view,
                metal_layer,
                is_ipad,
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
    /// - `Option<WindowState>` - Live window state.
    fn window_state(&self, id: WindowId) -> Option<WindowState> {
        self.get_windows().get(&id).map(IosWindowRecord::get_state)
    }

    /// Returns `RawWindowHandle::UiKit` carrying `UIWindow`, `UIView`, `CAMetalLayer`, and `is_ipad`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    /// - `WindowId` - Window handle.
    ///
    /// # Returns
    ///
    /// - `RawWindowHandle` - UIKit surface handles.
    fn raw_window_handle(&self, id: WindowId) -> RawWindowHandle {
        match self.get_windows().get(&id) {
            Some(rec) => RawWindowHandle::UiKit {
                ui_window: rec.get_ui_window(),
                ui_view: rec.get_ui_view(),
                metal_layer: rec.get_metal_layer(),
                is_ipad: rec.get_is_ipad(),
            },
            None => RawWindowHandle::Headless,
        }
    }

    /// Drains queued UIKit touch, Apple Pencil, keyboard, and lifecycle events.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable platform backend.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Pending events.
    fn poll_events(&mut self) -> Vec<WindowEvent> {
        take(self.get_mut_pending_events())
    }

    /// Pauses or resumes the attached UIKit `CADisplayLink` frame pump.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable iOS/iPadOS platform backend.
    /// - `bool` - `true` to pause frame ticks in background; `false` to resume.
    fn set_frame_pump_paused(&mut self, paused: bool) {
        self.set_pump_paused(paused);
        uikit_set_display_link_paused(self.get_display_link(), paused);
    }

    /// Returns whether the UIKit frame pump is currently paused for background freeze.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform backend reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when paused in background.
    fn is_frame_pump_paused(&self) -> bool {
        self.get_pump_paused()
    }

    /// Presents `framebuffer` directly onto the UIKit `CALayer` / `CAMetalLayer`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable platform backend.
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
        let layer_ptr: usize = self
            .get_windows()
            .get(&id)
            .map(IosWindowRecord::get_metal_layer)
            .ok_or(PlatformError::WindowNotFound(id.get_value()))?;
        ios_present_cg_image(layer_ptr, framebuffer)
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

    /// Stores `text` in the clipboard cache.
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

    /// Queries the live UIKit light/dark interface style.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform reference.
    ///
    /// # Returns
    ///
    /// - `ThemeMode` - Active system appearance.
    fn system_theme(&self) -> ThemeMode {
        ios_system_theme()
    }
}
