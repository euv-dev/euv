use super::*;

/// Metadata queries for `PlatformKind`.
impl PlatformKind {
    /// Returns the canonical lowercase name of this platform.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform kind.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Canonical platform name string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Web => PLATFORM_NAME_WEB,
            Self::Android => PLATFORM_NAME_ANDROID,
            Self::Ios => PLATFORM_NAME_IOS,
            Self::IpadOs => PLATFORM_NAME_IPADOS,
            Self::MacOs => PLATFORM_NAME_MACOS,
            Self::Windows => PLATFORM_NAME_WINDOWS,
            Self::Linux => PLATFORM_NAME_LINUX,
            Self::Headless => PLATFORM_NAME_HEADLESS,
        }
    }

    /// Returns the default `DeviceFormFactor` associated with this platform.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform kind.
    ///
    /// # Returns
    ///
    /// - `DeviceFormFactor` - Device form factor.
    #[must_use]
    pub const fn form_factor(&self) -> DeviceFormFactor {
        match *self {
            Self::MacOs | Self::Windows | Self::Linux => DeviceFormFactor::Desktop,
            Self::IpadOs => DeviceFormFactor::Tablet,
            Self::Ios | Self::Android => DeviceFormFactor::Phone,
            Self::Web => DeviceFormFactor::Browser,
            Self::Headless => DeviceFormFactor::Headless,
        }
    }

    /// Returns true for mobile or tablet touch-first platforms (`Android`, `Ios`, `IpadOs`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Platform kind.
    ///
    /// # Returns
    ///
    /// - `bool` - True for mobile/tablet targets.
    #[must_use]
    pub const fn is_mobile_or_tablet(&self) -> bool {
        matches!(*self, Self::Android | Self::Ios | Self::IpadOs)
    }

    /// Returns the list of all 7 production platforms supported by euv.
    ///
    /// # Returns
    ///
    /// - `[Self; 7]` - Array containing Web, Android, iOS, iPadOS, macOS, Windows, and Linux.
    #[must_use]
    pub const fn all_targets() -> [Self; 7] {
        [
            Self::Web,
            Self::Android,
            Self::Ios,
            Self::IpadOs,
            Self::MacOs,
            Self::Windows,
            Self::Linux,
        ]
    }
}

/// Capability matrix factory for `PlatformCapabilities`.
impl PlatformCapabilities {
    /// Returns the canonical capability matrix for `platform`.
    ///
    /// # Arguments
    ///
    /// - `PlatformKind` - Target platform.
    ///
    /// # Returns
    ///
    /// - `Self` - Normalized platform capabilities.
    #[must_use]
    pub fn for_platform(platform: PlatformKind) -> Self {
        match platform {
            PlatformKind::MacOs => Self {
                multi_window: true,
                hover_pointer: true,
                touch: false,
                stylus: false,
                hardware_keyboard: true,
                safe_area_insets: false,
                hardware_gpu: true,
                software_blit: true,
                clipboard: true,
                ime: true,
                dynamic_island: false,
                live_activities: false,
                system_tray: true,
                biometrics: true,
                haptics: true,
                protocol_handler: true,
                accessibility_bridge: true,
            },
            PlatformKind::Windows => Self {
                multi_window: true,
                hover_pointer: true,
                touch: false,
                stylus: false,
                hardware_keyboard: true,
                safe_area_insets: false,
                hardware_gpu: true,
                software_blit: true,
                clipboard: true,
                ime: true,
                dynamic_island: false,
                live_activities: false,
                system_tray: true,
                biometrics: true,
                haptics: false,
                protocol_handler: true,
                accessibility_bridge: true,
            },
            PlatformKind::Linux => Self {
                multi_window: true,
                hover_pointer: true,
                touch: false,
                stylus: false,
                hardware_keyboard: true,
                safe_area_insets: false,
                hardware_gpu: true,
                software_blit: true,
                clipboard: true,
                ime: true,
                dynamic_island: false,
                live_activities: false,
                system_tray: true,
                biometrics: false,
                haptics: false,
                protocol_handler: true,
                accessibility_bridge: true,
            },
            PlatformKind::IpadOs => Self {
                multi_window: true,
                hover_pointer: true,
                touch: true,
                stylus: true,
                hardware_keyboard: true,
                safe_area_insets: true,
                hardware_gpu: true,
                software_blit: true,
                clipboard: true,
                ime: true,
                dynamic_island: false,
                live_activities: true,
                system_tray: false,
                biometrics: true,
                haptics: true,
                protocol_handler: true,
                accessibility_bridge: true,
            },
            PlatformKind::Ios => Self {
                multi_window: false,
                hover_pointer: false,
                touch: true,
                stylus: true,
                hardware_keyboard: false,
                safe_area_insets: true,
                hardware_gpu: true,
                software_blit: true,
                clipboard: true,
                ime: true,
                dynamic_island: true,
                live_activities: true,
                system_tray: false,
                biometrics: true,
                haptics: true,
                protocol_handler: true,
                accessibility_bridge: true,
            },
            PlatformKind::Android => Self {
                multi_window: false,
                hover_pointer: false,
                touch: true,
                stylus: true,
                hardware_keyboard: false,
                safe_area_insets: true,
                hardware_gpu: true,
                software_blit: true,
                clipboard: true,
                ime: true,
                dynamic_island: false,
                live_activities: true,
                system_tray: false,
                biometrics: true,
                haptics: true,
                protocol_handler: true,
                accessibility_bridge: true,
            },
            PlatformKind::Web => Self {
                multi_window: false,
                hover_pointer: true,
                touch: true,
                stylus: true,
                hardware_keyboard: true,
                safe_area_insets: true,
                hardware_gpu: true,
                software_blit: true,
                clipboard: true,
                ime: true,
                dynamic_island: false,
                live_activities: false,
                system_tray: false,
                biometrics: true,
                haptics: true,
                protocol_handler: true,
                accessibility_bridge: true,
            },
            PlatformKind::Headless => Self {
                multi_window: true,
                hover_pointer: true,
                touch: true,
                stylus: false,
                hardware_keyboard: true,
                safe_area_insets: true,
                hardware_gpu: false,
                software_blit: true,
                clipboard: true,
                ime: true,
                dynamic_island: false,
                live_activities: false,
                system_tray: true,
                biometrics: false,
                haptics: false,
                protocol_handler: true,
                accessibility_bridge: true,
            },
        }
    }
}

/// Public accessors for `DisplayModeInfo`.
impl DisplayModeInfo {
    /// Returns the OS mode identifier.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Display mode info.
    ///
    /// # Returns
    ///
    /// - `i32` - Mode identifier.
    #[must_use]
    pub fn mode_id(&self) -> i32 {
        self.get_mode_id()
    }

    /// Returns the physical resolution `(width, height)` in pixels.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Display mode info.
    ///
    /// # Returns
    ///
    /// - `(u32, u32)` - Physical width and height.
    #[must_use]
    pub fn physical_size(&self) -> (u32, u32) {
        (self.get_physical_width(), self.get_physical_height())
    }

    /// Returns the hardware refresh rate in Hertz.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Display mode info.
    ///
    /// # Returns
    ///
    /// - `f32` - Refresh rate in Hertz.
    #[must_use]
    pub fn refresh_rate_hz(&self) -> f32 {
        self.get_refresh_rate_hz()
    }
}

/// Public accessors for `FramePacingConfig`.
impl FramePacingConfig {
    /// Returns the minimum adaptive refresh rate in Hertz.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Frame pacing config.
    ///
    /// # Returns
    ///
    /// - `f32` - Minimum refresh rate in Hertz.
    #[must_use]
    pub fn min_hz(&self) -> f32 {
        self.get_min_hz()
    }

    /// Returns the maximum adaptive refresh rate in Hertz.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Frame pacing config.
    ///
    /// # Returns
    ///
    /// - `f32` - Maximum refresh rate in Hertz.
    #[must_use]
    pub fn max_hz(&self) -> f32 {
        self.get_max_hz()
    }

    /// Returns the preferred target refresh rate in Hertz.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Frame pacing config.
    ///
    /// # Returns
    ///
    /// - `f32` - Preferred refresh rate in Hertz.
    #[must_use]
    pub fn preferred_hz(&self) -> f32 {
        self.get_preferred_hz()
    }

    /// Returns the target frame interval in seconds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Frame pacing config.
    ///
    /// # Returns
    ///
    /// - `f32` - Frame duration in seconds.
    #[must_use]
    pub fn frame_interval_secs(&self) -> f32 {
        self.get_frame_interval_secs()
    }

    /// Returns the target frame interval in nanoseconds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Frame pacing config.
    ///
    /// # Returns
    ///
    /// - `u64` - Frame duration in nanoseconds.
    #[must_use]
    pub fn frame_interval_nanos(&self) -> u64 {
        self.get_frame_interval_nanos()
    }
}

/// Multi-touch pointer lifecycle tracking and cancellation helpers for `TouchPointerTracker`.
impl TouchPointerTracker {
    /// Returns the currently active mapped pointer identifiers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Tracker reference.
    ///
    /// # Returns
    ///
    /// - `&[u64]` - Slice of active pointer IDs.
    #[must_use]
    pub fn active_ids(&self) -> &[u64] {
        self.get_active_pointers().as_slice()
    }

    /// Returns the count of currently active touch pointers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Tracker reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Active pointer count.
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.get_active_pointers().len()
    }

    /// Maps an Android `MotionEvent` raw pointer ID (`>= 0`) to a non-zero `euv` pointer identifier (`>= 1`).
    ///
    /// # Arguments
    ///
    /// - `i32` - Android pointer identifier.
    ///
    /// # Returns
    ///
    /// - `u64` - Non-zero mapped pointer identifier.
    #[must_use]
    pub fn map_android_pointer_id(raw_id: i32) -> u64 {
        (raw_id.max(0) as u64).saturating_add(1)
    }

    /// Inserts `pointer_id` into the active contact set if not already present.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable tracker.
    /// - `u64` - Mapped pointer identifier.
    fn track_pointer(&mut self, pointer_id: u64) {
        if pointer_id != 0 && !self.get_active_pointers().contains(&pointer_id) {
            self.get_mut_active_pointers().push(pointer_id);
        }
    }

    /// Removes `pointer_id` from the active contact set.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable tracker.
    /// - `u64` - Mapped pointer identifier.
    fn untrack_pointer(&mut self, pointer_id: u64) {
        self.get_mut_active_pointers()
            .retain(|id: &u64| *id != pointer_id);
    }

    /// Translates an Android `MotionEvent` (`action_masked`, `action_index`, `pointers`) into normalized `WindowEvent`s.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable tracker.
    /// - `WindowId` - Target window handle.
    /// - `i32` - Masked Android motion action (`ACTION_DOWN`, `ACTION_POINTER_DOWN`, `ACTION_MOVE`, `ACTION_POINTER_UP`, `ACTION_UP`, `ACTION_CANCEL`).
    /// - `usize` - Pointer index reported by `MotionEvent.getActionIndex()`.
    /// - `&[(i32, f32, f32)]` - Active pointer tuples `(raw_pointer_id, logical_x, logical_y)` in index order.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Ordered events to dispatch to `App`.
    pub fn handle_android_motion(
        &mut self,
        wid: WindowId,
        action_masked: i32,
        action_index: usize,
        pointers: &[(i32, f32, f32)],
    ) -> Vec<WindowEvent> {
        let mut events: Vec<WindowEvent> = Vec::new();
        match action_masked {
            ANDROID_ACTION_DOWN => {
                if !self.get_active_pointers().is_empty() {
                    let stale: Vec<u64> = take(self.get_mut_active_pointers());
                    for id in stale.into_iter() {
                        events.push(WindowEvent::PointerCancel(wid, id));
                    }
                    events.push(WindowEvent::PointerCancel(wid, 0));
                }
                if let Some(&(raw_id, x, y)) = pointers.get(action_index) {
                    let pid: u64 = Self::map_android_pointer_id(raw_id);
                    self.track_pointer(pid);
                    events.push(WindowEvent::PointerDown(
                        wid,
                        PointerEvent::touch(pid, x, y),
                    ));
                }
            }
            ANDROID_ACTION_POINTER_DOWN => {
                if let Some(&(raw_id, x, y)) = pointers.get(action_index) {
                    let pid: u64 = Self::map_android_pointer_id(raw_id);
                    self.track_pointer(pid);
                    events.push(WindowEvent::PointerDown(
                        wid,
                        PointerEvent::touch(pid, x, y),
                    ));
                }
            }
            ANDROID_ACTION_MOVE => {
                for &(raw_id, x, y) in pointers.iter() {
                    let pid: u64 = Self::map_android_pointer_id(raw_id);
                    self.track_pointer(pid);
                    events.push(WindowEvent::PointerMove(
                        wid,
                        PointerEvent::touch(pid, x, y),
                    ));
                }
            }
            ANDROID_ACTION_POINTER_UP => {
                if let Some(&(raw_id, x, y)) = pointers.get(action_index) {
                    let pid: u64 = Self::map_android_pointer_id(raw_id);
                    self.untrack_pointer(pid);
                    events.push(WindowEvent::PointerUp(wid, PointerEvent::touch(pid, x, y)));
                }
            }
            ANDROID_ACTION_UP => {
                if let Some(&(raw_id, x, y)) = pointers.get(action_index) {
                    let pid: u64 = Self::map_android_pointer_id(raw_id);
                    self.untrack_pointer(pid);
                    events.push(WindowEvent::PointerUp(wid, PointerEvent::touch(pid, x, y)));
                }
                self.get_mut_active_pointers().clear();
            }
            ANDROID_ACTION_CANCEL => {
                let stale: Vec<u64> = take(self.get_mut_active_pointers());
                for id in stale.into_iter() {
                    events.push(WindowEvent::PointerCancel(wid, id));
                }
                events.push(WindowEvent::PointerCancel(wid, 0));
            }
            _ => {}
        }
        events
    }

    /// Records iOS `touchesBegan` contacts and produces corresponding `PointerDown` events.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable tracker.
    /// - `WindowId` - Target window handle.
    /// - `&[(u64, f32, f32)]` - Touch tuples `(pointer_id, logical_x, logical_y)`.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - `PointerDown` events.
    pub fn handle_ios_touches_began(
        &mut self,
        wid: WindowId,
        touches: &[(u64, f32, f32)],
    ) -> Vec<WindowEvent> {
        let mut events: Vec<WindowEvent> = Vec::with_capacity(touches.len());
        for &(raw_id, x, y) in touches.iter() {
            let pid: u64 = raw_id.max(1);
            self.track_pointer(pid);
            events.push(WindowEvent::PointerDown(
                wid,
                PointerEvent::touch(pid, x, y),
            ));
        }
        events
    }

    /// Handles iOS `touchesCancelled`, emitting `PointerCancel` for every cancelled and stale contact.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable tracker.
    /// - `WindowId` - Target window handle.
    /// - `&[u64]` - Cancelled touch pointer identifiers.
    /// - `&[u64]` - Remaining active touch pointer identifiers from `UIEvent.allTouches`.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Ordered `PointerCancel` events.
    pub fn handle_ios_touches_cancelled(
        &mut self,
        wid: WindowId,
        cancelled_ids: &[u64],
        remaining_live_ids: &[u64],
    ) -> Vec<WindowEvent> {
        let mut events: Vec<WindowEvent> = Vec::new();
        for &raw_id in cancelled_ids.iter() {
            let pid: u64 = raw_id.max(1);
            self.untrack_pointer(pid);
            events.push(WindowEvent::PointerCancel(wid, pid));
        }
        let stale: Vec<u64> = self
            .get_active_pointers()
            .iter()
            .copied()
            .filter(|id: &u64| !remaining_live_ids.contains(id))
            .collect();
        for id in stale.into_iter() {
            self.untrack_pointer(id);
            events.push(WindowEvent::PointerCancel(wid, id));
        }
        if self.get_active_pointers().is_empty() {
            events.push(WindowEvent::PointerCancel(wid, 0));
        }
        events
    }

    /// Cancels all active touches on focus loss, pause, or window detachment so input never wedges.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable tracker.
    /// - `WindowId` - Target window handle.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Per-pointer and global `PointerCancel` events (empty if no contacts were active).
    pub fn cancel_all_on_focus_loss(&mut self, wid: WindowId) -> Vec<WindowEvent> {
        let active: Vec<u64> = take(self.get_mut_active_pointers());
        if active.is_empty() {
            return Vec::new();
        }
        let mut events: Vec<WindowEvent> = Vec::with_capacity(active.len().saturating_add(1));
        for id in active.into_iter() {
            events.push(WindowEvent::PointerCancel(wid, id));
        }
        events.push(WindowEvent::PointerCancel(wid, 0));
        events
    }
}

/// Generation-stamped same-process session lifecycle helpers for `MobileSessionLifecycle`.
impl MobileSessionLifecycle {
    /// Registers a new host activity or scene session, resetting transient flags and returning its monotonic generation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable lifecycle coordinator.
    /// - `u64` - Unique identifier of the newly created host activity or scene.
    ///
    /// # Returns
    ///
    /// - `u64` - Monotonic session generation assigned to `owner_id`.
    pub fn begin_session(&mut self, owner_id: u64) -> u64 {
        let next_gen: u64 = self.get_session_generation().wrapping_add(1).max(1);
        self.set_session_generation(next_gen);
        self.set_active_owner_id(owner_id);
        self.set_surface_attached(false);
        self.set_loop_running(false);
        self.set_app_running(true);
        self.set_background_frozen(false);
        self.set_terminated(false);
        self.get_mut_touch_tracker()
            .get_mut_active_pointers()
            .clear();
        next_gen
    }

    /// Re-claims bridge ownership when `owner_id` resumes and is either the latest session or the previous owner cleared.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable lifecycle coordinator.
    /// - `u64` - Resuming host activity or scene identifier.
    /// - `u64` - Generation originally assigned to `owner_id`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `owner_id` holds active bridge ownership after the call.
    pub fn activate_session(&mut self, owner_id: u64, session_gen: u64) -> bool {
        let cur_gen: u64 = self.get_session_generation();
        if session_gen >= cur_gen || self.get_active_owner_id() == 0 {
            if session_gen > cur_gen {
                self.set_session_generation(session_gen);
            }
            self.set_active_owner_id(owner_id);
            return true;
        }
        self.get_active_owner_id() == owner_id && cur_gen == session_gen
    }

    /// Ends `owner_id`'s session only when `(owner_id, session_gen)` still owns the bridge, preventing delayed teardown races.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable lifecycle coordinator.
    /// - `u64` - Destroying host activity or scene identifier.
    /// - `u64` - Generation originally assigned to `owner_id`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when ownership belonged to `owner_id` and was cleared; `false` when a newer session already replaced it.
    pub fn end_session(&mut self, owner_id: u64, session_gen: u64) -> bool {
        if self.get_active_owner_id() != owner_id || self.get_session_generation() != session_gen {
            return false;
        }
        self.set_active_owner_id(0);
        self.set_surface_attached(false);
        self.set_loop_running(false);
        self.set_app_running(false);
        self.set_terminated(true);
        self.get_mut_touch_tracker()
            .get_mut_active_pointers()
            .clear();
        true
    }

    /// Returns `true` when `(owner_id, session_gen)` is currently the authoritative foreground bridge owner.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Lifecycle coordinator reference.
    /// - `u64` - Host activity or scene identifier.
    /// - `u64` - Generation assigned to `owner_id`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `owner_id` matches the active generation and owner.
    #[must_use]
    pub fn is_active_owner(&self, owner_id: u64, session_gen: u64) -> bool {
        owner_id != 0
            && self.get_active_owner_id() == owner_id
            && self.get_session_generation() == session_gen
    }

    /// Returns the current monotonic session generation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Lifecycle coordinator reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Current session generation.
    #[must_use]
    pub fn active_session_generation(&self) -> u64 {
        self.get_session_generation()
    }

    /// Returns the currently active bridge owner identifier (`0` when no activity or scene is active).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Lifecycle coordinator reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Active host owner identifier.
    #[must_use]
    pub fn active_owner_id(&self) -> u64 {
        self.get_active_owner_id()
    }

    /// Returns the current VSync loop generation counter.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Lifecycle coordinator reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Active VSync callback generation.
    #[must_use]
    pub fn loop_generation(&self) -> u64 {
        self.get_loop_generation()
    }

    /// Returns whether the VSync render loop is currently active.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Lifecycle coordinator reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the VSync frame pump is running.
    #[must_use]
    pub fn is_loop_running(&self) -> bool {
        self.get_loop_running()
    }

    /// Returns whether a native rendering surface is currently attached.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Lifecycle coordinator reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the native surface is valid and bound.
    #[must_use]
    pub fn is_surface_attached(&self) -> bool {
        self.get_surface_attached()
    }

    /// Returns `true` when an existing activity resumed after `Terminate` or exit request and must recreate its native `App` handle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Lifecycle coordinator reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the native handle is no longer running or was terminated.
    #[must_use]
    pub fn should_recreate_handle(&self) -> bool {
        self.get_terminated() || !self.get_app_running()
    }

    /// Marks the active session's surface attached and recreates the live running state if needed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable lifecycle coordinator.
    pub fn on_surface_created(&mut self) {
        self.set_surface_attached(true);
        if self.should_recreate_handle() {
            self.set_app_running(true);
            self.set_terminated(false);
            self.set_background_frozen(false);
        }
    }

    /// Marks the active surface detached and emits `PointerCancel` events for any active contacts.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable lifecycle coordinator.
    /// - `WindowId` - Window whose surface detached.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Cancellation events for any dangling touch contacts.
    pub fn on_surface_destroyed(&mut self, wid: WindowId) -> Vec<WindowEvent> {
        self.set_surface_attached(false);
        self.get_mut_touch_tracker().cancel_all_on_focus_loss(wid)
    }

    /// Transitions the active session to resumed foreground state and returns the unique VSync loop generation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable lifecycle coordinator.
    ///
    /// # Returns
    ///
    /// - `u64` - Unique VSync loop generation preventing duplicate frame callbacks.
    pub fn on_resume(&mut self) -> u64 {
        self.set_background_frozen(false);
        if self.should_recreate_handle() {
            self.set_app_running(true);
            self.set_terminated(false);
        }
        let next_loop: u64 = self.get_loop_generation().wrapping_add(1).max(1);
        self.set_loop_generation(next_loop);
        self.set_loop_running(true);
        next_loop
    }

    /// Pauses the active VSync loop, advances the loop generation to invalidate in-flight callbacks, and cancels active touches.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable lifecycle coordinator.
    /// - `WindowId` - Target window handle.
    ///
    /// # Returns
    ///
    /// - `Vec<WindowEvent>` - Cancellation events for any active touch contacts.
    pub fn on_pause(&mut self, wid: WindowId) -> Vec<WindowEvent> {
        self.set_loop_running(false);
        let next_loop: u64 = self.get_loop_generation().wrapping_add(1).max(1);
        self.set_loop_generation(next_loop);
        self.get_mut_touch_tracker().cancel_all_on_focus_loss(wid)
    }

    /// Marks the active session background-frozen with the VSync loop halted.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable lifecycle coordinator.
    pub fn on_background(&mut self) {
        self.set_background_frozen(true);
        self.set_loop_running(false);
    }

    /// Records a double-back exit request or OS termination so subsequent re-entry recreates a fresh native handle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable lifecycle coordinator.
    pub fn on_terminate(&mut self) {
        self.set_app_running(false);
        self.set_terminated(true);
        self.set_loop_running(false);
    }

    /// Returns an immutable reference to the session's `TouchPointerTracker`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Lifecycle coordinator reference.
    ///
    /// # Returns
    ///
    /// - `&TouchPointerTracker` - Active touch tracker.
    #[must_use]
    pub fn touch_tracker(&self) -> &TouchPointerTracker {
        self.get_touch_tracker()
    }

    /// Returns a mutable reference to the session's `TouchPointerTracker`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable lifecycle coordinator.
    ///
    /// # Returns
    ///
    /// - `&mut TouchPointerTracker` - Mutable touch tracker.
    pub fn touch_tracker_mut(&mut self) -> &mut TouchPointerTracker {
        self.get_mut_touch_tracker()
    }
}
