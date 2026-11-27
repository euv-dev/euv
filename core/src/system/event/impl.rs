use super::*;

/// Orientation classification helpers for `ScreenOrientation`.
impl ScreenOrientation {
    /// Derives canonical `Portrait` or `Landscape` orientation from logical viewport dimensions.
    ///
    /// # Arguments
    ///
    /// - `f32` - Logical viewport width.
    /// - `f32` - Logical viewport height.
    ///
    /// # Returns
    ///
    /// - `Self` - `Landscape` when `width > height`, otherwise `Portrait`.
    #[must_use]
    pub fn from_size(width: f32, height: f32) -> Self {
        if width > height {
            Self::Landscape
        } else {
            Self::Portrait
        }
    }

    /// Returns `true` for any landscape orientation variant.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Orientation reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether orientation is horizontal.
    #[must_use]
    pub fn is_landscape(&self) -> bool {
        matches!(
            self,
            Self::Landscape | Self::LandscapeLeft | Self::LandscapeRight
        )
    }

    /// Returns `true` for any portrait orientation variant.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Orientation reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether orientation is vertical.
    #[must_use]
    pub fn is_portrait(&self) -> bool {
        matches!(self, Self::Portrait | Self::PortraitUpsideDown)
    }
}

/// Reachability helper methods for `NetworkStatus`.
impl NetworkStatus {
    /// Returns `true` when network connectivity is available (`Online`, `Wifi`, or `Cellular`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Network status reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the host has an active online route.
    #[must_use]
    pub fn is_online(&self) -> bool {
        matches!(self, Self::Online | Self::Wifi | Self::Cellular)
    }
}

/// Marker trait enabling `SystemEvent` forwarding over `EventBus` when desired.
impl CustomEvent for SystemEvent {}

/// Manual early unregistration for `SystemEventSubscription`.
impl SystemEventSubscription {
    /// Unregisters the associated system event listener immediately.
    ///
    /// # Arguments
    ///
    /// - `Self` - Subscription handle to cancel.
    pub fn unsubscribe(self) {
        let id: u64 = self.get_id();
        let _: Result<(), _> = SYSTEM_EVENT_HUB.try_with(|cell: &RefCell<SystemEventHub>| {
            if let Ok(mut hub) = cell.try_borrow_mut() {
                hub.unregister(id);
            }
        });
    }
}

/// Default active-foreground initial state for `SystemEventNormalizer`.
impl Default for SystemEventNormalizer {
    /// Creates a `SystemEventNormalizer` initialized in `AppState::Active` and visible.
    ///
    /// # Returns
    ///
    /// - `Self` - Default normalizer state machine.
    fn default() -> Self {
        Self {
            state: AppState::Active,
            visible: true,
            last_focused: None,
            last_resize: None,
            last_safe_area: None,
            last_orientation: None,
            last_theme: None,
            last_locale: None,
            last_keyboard: None,
            last_low_power: None,
            last_network: None,
        }
    }
}

/// Canonical cross-platform lifecycle normalization and deduplication state machine.
impl SystemEventNormalizer {
    /// Creates a new `SystemEventNormalizer` starting in `AppState::Active`.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized state machine.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current canonical `AppState`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Normalizer reference.
    ///
    /// # Returns
    ///
    /// - `AppState` - Active normalized application state.
    #[must_use]
    pub fn state(&self) -> AppState {
        self.get_state()
    }

    /// Returns `true` when the active surface is currently visible.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Normalizer reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether `Appear` is active without a subsequent `Disappear`.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.get_visible()
    }

    /// Normalizes a raw platform `SystemEvent`, deduplicating redundant notifications and synthesizing missing transition edges.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable normalizer state machine.
    /// - `SystemEvent` - Raw platform or synthetic system event.
    ///
    /// # Returns
    ///
    /// - `Vec<SystemEvent>` - Zero or more canonical events in strict cross-platform order.
    pub fn normalize(&mut self, event: SystemEvent) -> Vec<SystemEvent> {
        if self.get_state() == AppState::Terminated {
            return Vec::new();
        }
        let mut out: Vec<SystemEvent> = Vec::with_capacity(3);
        match event {
            SystemEvent::Inactive => {
                if self.get_state() == AppState::Active {
                    self.set_state(AppState::Inactive);
                    out.push(SystemEvent::Inactive);
                }
            }
            SystemEvent::Disappear => {
                if self.get_visible() {
                    if self.get_state() == AppState::Active {
                        self.set_state(AppState::Inactive);
                        out.push(SystemEvent::Inactive);
                    }
                    self.set_visible(false);
                    out.push(SystemEvent::Disappear);
                }
            }
            SystemEvent::Background => {
                if self.get_state() != AppState::Background {
                    if self.get_state() == AppState::Active {
                        self.set_state(AppState::Inactive);
                        out.push(SystemEvent::Inactive);
                    }
                    if self.get_visible() {
                        self.set_visible(false);
                        out.push(SystemEvent::Disappear);
                    }
                    self.set_state(AppState::Background);
                    out.push(SystemEvent::Background);
                }
            }
            SystemEvent::Appear => {
                if !self.get_visible() {
                    self.set_visible(true);
                    if self.get_state() == AppState::Background {
                        self.set_state(AppState::Inactive);
                    }
                    out.push(SystemEvent::Appear);
                }
            }
            SystemEvent::Foreground => {
                if self.get_state() != AppState::Active || !self.get_visible() {
                    if !self.get_visible() {
                        self.set_visible(true);
                        out.push(SystemEvent::Appear);
                    }
                    self.set_state(AppState::Active);
                    out.push(SystemEvent::Foreground);
                }
            }
            SystemEvent::Terminate => {
                if self.get_state() == AppState::Active {
                    out.push(SystemEvent::Inactive);
                }
                if self.get_visible() {
                    self.set_visible(false);
                    out.push(SystemEvent::Disappear);
                }
                self.set_state(AppState::Terminated);
                out.push(SystemEvent::Terminate);
            }
            SystemEvent::FocusChanged(focused) => {
                let prev: Option<bool> = {
                    let Self { last_focused, .. } = self;
                    *last_focused
                };
                if prev != Some(focused) {
                    self.set_last_focused(Some(focused));
                    out.push(SystemEvent::FocusChanged(focused));
                }
            }
            SystemEvent::Resize {
                width,
                height,
                scale_factor,
            } => {
                let w: f32 = width.max(1.0);
                let h: f32 = height.max(1.0);
                let s: f32 = if scale_factor.is_finite() {
                    scale_factor.clamp(0.5, 4.5)
                } else {
                    1.0
                };
                let prev: Option<(f32, f32, f32)> = {
                    let Self { last_resize, .. } = self;
                    *last_resize
                };
                let changed: bool = match prev {
                    Some((pw, ph, ps)) => {
                        (pw - w).abs() > RESIZE_DEDUP_EPSILON
                            || (ph - h).abs() > RESIZE_DEDUP_EPSILON
                            || (ps - s).abs() > RESIZE_DEDUP_EPSILON
                    }
                    None => true,
                };
                if changed {
                    self.set_last_resize(Some((w, h, s)));
                    out.push(SystemEvent::Resize {
                        width: w,
                        height: h,
                        scale_factor: s,
                    });
                    let derived_orient: ScreenOrientation = ScreenOrientation::from_size(w, h);
                    let prev_orient: Option<ScreenOrientation> = {
                        let Self {
                            last_orientation, ..
                        } = self;
                        *last_orientation
                    };
                    let orient_changed: bool = match prev_orient {
                        Some(po) => po.is_landscape() != derived_orient.is_landscape(),
                        None => false,
                    };
                    if prev_orient.is_none() {
                        self.set_last_orientation(Some(derived_orient));
                    } else if orient_changed {
                        self.set_last_orientation(Some(derived_orient));
                        out.push(SystemEvent::OrientationChanged(derived_orient));
                    }
                }
            }
            SystemEvent::SafeAreaChanged(insets) => {
                let prev: Option<SafeAreaInsets> = {
                    let Self { last_safe_area, .. } = self;
                    *last_safe_area
                };
                if prev != Some(insets) {
                    self.set_last_safe_area(Some(insets));
                    out.push(SystemEvent::SafeAreaChanged(insets));
                }
            }
            SystemEvent::OrientationChanged(orient) => {
                let prev: Option<ScreenOrientation> = {
                    let Self {
                        last_orientation, ..
                    } = self;
                    *last_orientation
                };
                if prev != Some(orient) {
                    self.set_last_orientation(Some(orient));
                    out.push(SystemEvent::OrientationChanged(orient));
                }
            }
            SystemEvent::ThemeChanged(theme) => {
                let prev: Option<SystemTheme> = {
                    let Self { last_theme, .. } = self;
                    *last_theme
                };
                if prev != Some(theme) {
                    self.set_last_theme(Some(theme));
                    out.push(SystemEvent::ThemeChanged(theme));
                }
            }
            SystemEvent::LocaleChanged(locale) => {
                let trimmed: String = locale.trim().to_string();
                if !trimmed.is_empty() {
                    let same: bool = {
                        let Self { last_locale, .. } = self;
                        last_locale.as_deref() == Some(trimmed.as_str())
                    };
                    if !same {
                        self.set_last_locale(Some(trimmed.clone()));
                        out.push(SystemEvent::LocaleChanged(trimmed));
                    }
                }
            }
            SystemEvent::KeyboardVisibilityChanged { visible, height } => {
                let norm_h: f32 = if visible && height.is_finite() {
                    height.max(0.0)
                } else {
                    0.0
                };
                let prev: Option<(bool, f32)> = {
                    let Self { last_keyboard, .. } = self;
                    *last_keyboard
                };
                let changed: bool = match prev {
                    Some((pv, ph)) => {
                        pv != visible || (ph - norm_h).abs() > KEYBOARD_HEIGHT_EPSILON
                    }
                    None => true,
                };
                if changed {
                    self.set_last_keyboard(Some((visible, norm_h)));
                    out.push(SystemEvent::KeyboardVisibilityChanged {
                        visible,
                        height: norm_h,
                    });
                }
            }
            SystemEvent::LowPowerModeChanged(enabled) => {
                let prev: Option<bool> = {
                    let Self { last_low_power, .. } = self;
                    *last_low_power
                };
                if prev != Some(enabled) {
                    self.set_last_low_power(Some(enabled));
                    out.push(SystemEvent::LowPowerModeChanged(enabled));
                }
            }
            SystemEvent::NetworkStatusChanged(status) => {
                let prev: Option<NetworkStatus> = {
                    let Self { last_network, .. } = self;
                    *last_network
                };
                if prev != Some(status) {
                    self.set_last_network(Some(status));
                    out.push(SystemEvent::NetworkStatusChanged(status));
                }
            }
            SystemEvent::MemoryWarning(pressure) => {
                out.push(SystemEvent::MemoryWarning(pressure));
            }
            SystemEvent::BackPressed => {
                out.push(SystemEvent::BackPressed);
            }
            SystemEvent::DeepLink(uri) => {
                if !uri.is_empty() {
                    out.push(SystemEvent::DeepLink(uri));
                }
            }
            SystemEvent::ClipboardChanged => {
                out.push(SystemEvent::ClipboardChanged);
            }
        }
        out
    }
}

/// Scoped listener registration, unregistration, and snapshot dispatch for `SystemEventHub`.
impl SystemEventHub {
    /// Registers a listener callback owned by `owner_scope` and returns its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable system event hub.
    /// - `Option<ScopeId>` - Enclosing reactive scope identifier, if any.
    /// - `SystemEventCallback` - Type-erased callback handle.
    ///
    /// # Returns
    ///
    /// - `u64` - Unique listener identifier.
    pub(crate) fn register(
        &mut self,
        owner_scope: Option<ScopeId>,
        callback: SystemEventCallback,
    ) -> u64 {
        let Self { next_id, listeners } = self;
        *next_id = next_id.wrapping_add(1).max(1);
        let assigned: u64 = *next_id;
        listeners.push(SystemEventListener {
            id: assigned,
            owner_scope,
            callback,
        });
        assigned
    }

    /// Removes the listener matching `id`, if still present.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable system event hub.
    /// - `u64` - Listener identifier to remove.
    pub(crate) fn unregister(&mut self, id: u64) {
        let Self { listeners, .. } = self;
        listeners.retain(|entry: &SystemEventListener| entry.id != id);
    }

    /// Removes all registered listeners and resets the identifier counter.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable system event hub.
    pub(crate) fn clear(&mut self) {
        let Self { next_id, listeners } = self;
        listeners.clear();
        *next_id = 0;
    }

    /// Clones callbacks for all non-frozen listeners so invocation happens outside any hub borrow.
    ///
    /// # Arguments
    ///
    /// - `&Self` - System event hub reference.
    ///
    /// # Returns
    ///
    /// - `Vec<SystemEventCallback>` - Ordered active callbacks ready to execute.
    #[must_use]
    pub(crate) fn snapshot_active(&self) -> Vec<SystemEventCallback> {
        let Self { listeners, .. } = self;
        if listeners.is_empty() {
            return Vec::new();
        }
        with_runtime(
            |rt: &mut ReactiveRuntime| {
                let mut out: Vec<SystemEventCallback> = Vec::with_capacity(listeners.len());
                for entry in listeners.iter() {
                    let is_frozen: bool = entry
                        .owner_scope
                        .map(|sid: ScopeId| rt.is_scope_effectively_frozen(sid))
                        .unwrap_or(false);
                    if !is_frozen {
                        out.push(Rc::clone(&entry.callback));
                    }
                }
                out
            },
            Vec::new(),
        )
    }

    /// Clones callbacks for listeners owned by any scope in `scopes` (or descendant scopes thereof).
    ///
    /// # Arguments
    ///
    /// - `&Self` - System event hub reference.
    /// - `&[ScopeId]` - Target subtree root scope identifiers.
    ///
    /// # Returns
    ///
    /// - `Vec<SystemEventCallback>` - Matching callbacks in registration order.
    #[must_use]
    pub(crate) fn snapshot_for_scopes(&self, scopes: &[ScopeId]) -> Vec<SystemEventCallback> {
        let Self { listeners, .. } = self;
        if listeners.is_empty() || scopes.is_empty() {
            return Vec::new();
        }
        with_runtime(
            |rt: &mut ReactiveRuntime| {
                let mut out: Vec<SystemEventCallback> = Vec::new();
                for entry in listeners.iter() {
                    if let Some(owner) = entry.owner_scope
                        && scopes
                            .iter()
                            .any(|root_scope: &ScopeId| rt.is_scope_in_subtree(owner, *root_scope))
                    {
                        out.push(Rc::clone(&entry.callback));
                    }
                }
                out
            },
            Vec::new(),
        )
    }
}
