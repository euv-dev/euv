use super::*;

/// Invokes a snapshot of system event callbacks outside any hub or scene borrow.
///
/// # Arguments
///
/// - `Vec<SystemEventCallback>` - Ordered listener callbacks to invoke.
/// - `&SystemEvent` - Normalized system event payload.
fn invoke_system_callbacks(callbacks: Vec<SystemEventCallback>, event: &SystemEvent) {
    for callback in callbacks.into_iter() {
        if let Ok(mut func) = callback.try_borrow_mut() {
            func(event);
        }
    }
}

/// Dispatches `event` synchronously to all active (non-frozen) scoped system event listeners.
///
/// # Arguments
///
/// - `&SystemEvent` - Normalized system event payload.
pub(crate) fn dispatch_system_event_listeners(event: &SystemEvent) {
    let callbacks: Vec<SystemEventCallback> = SYSTEM_EVENT_HUB
        .try_with(|cell: &RefCell<SystemEventHub>| {
            cell.try_borrow()
                .map(|hub: Ref<'_, SystemEventHub>| hub.snapshot_active())
                .unwrap_or_default()
        })
        .unwrap_or_default();
    invoke_system_callbacks(callbacks, event);
}

/// Dispatches `event` synchronously to listeners owned by any reactive scope within `scopes`.
///
/// # Arguments
///
/// - `&[ScopeId]` - Root scope identifiers defining the target page or component subtree.
/// - `&SystemEvent` - System event payload (`Appear` or `Disappear`).
pub(crate) fn dispatch_system_event_to_scopes(scopes: &[ScopeId], event: &SystemEvent) {
    let callbacks: Vec<SystemEventCallback> = SYSTEM_EVENT_HUB
        .try_with(|cell: &RefCell<SystemEventHub>| {
            cell.try_borrow()
                .map(|hub: Ref<'_, SystemEventHub>| hub.snapshot_for_scopes(scopes))
                .unwrap_or_default()
        })
        .unwrap_or_default();
    invoke_system_callbacks(callbacks, event);
}

/// Clears all registered thread-local system event listeners when resetting the reactive runtime.
pub(crate) fn reset_system_event_hub() {
    let _: Result<(), _> = SYSTEM_EVENT_HUB.try_with(|cell: &RefCell<SystemEventHub>| {
        if let Ok(mut hub) = cell.try_borrow_mut() {
            hub.clear();
        }
    });
}

/// Subscribes a scoped listener invoked for every normalized cross-platform `SystemEvent`.
///
/// Automatically unregisters when the enclosing reactive `Scope` disposes.
///
/// # Arguments
///
/// - `F` - Handler closure receiving `&SystemEvent`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Copyable handle supporting optional early `.unsubscribe()`.
pub fn on_system_event<F>(handler: F) -> SystemEventSubscription
where
    F: FnMut(&SystemEvent) + 'static,
{
    let owner_scope: Option<ScopeId> =
        with_runtime(|rt: &mut ReactiveRuntime| rt.current_scope_id(), None);
    let callback: SystemEventCallback = Rc::new(RefCell::new(handler));
    let id: u64 = SYSTEM_EVENT_HUB
        .try_with(|cell: &RefCell<SystemEventHub>| {
            cell.try_borrow_mut()
                .map(|mut hub: RefMut<'_, SystemEventHub>| hub.register(owner_scope, callback))
                .unwrap_or(0)
        })
        .unwrap_or(0);
    let sub: SystemEventSubscription = SystemEventSubscription::new(id);
    if owner_scope.is_some() && id != 0 {
        on_cleanup(move || {
            sub.unsubscribe();
        });
    }
    sub
}

/// Subscribes a scoped handler invoked when the application window or current routed page appears.
///
/// # Arguments
///
/// - `F` - Zero-argument callback invoked on `SystemEvent::Appear`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_appear<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut() + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if matches!(event, SystemEvent::Appear) {
            handler();
        }
    })
}

/// Subscribes a scoped handler invoked when the application window or current routed page disappears.
///
/// # Arguments
///
/// - `F` - Zero-argument callback invoked on `SystemEvent::Disappear`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_disappear<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut() + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if matches!(event, SystemEvent::Disappear) {
            handler();
        }
    })
}

/// Subscribes a scoped handler invoked when the application enters active foreground execution.
///
/// # Arguments
///
/// - `F` - Zero-argument callback invoked on `SystemEvent::Foreground`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_foreground<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut() + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if matches!(event, SystemEvent::Foreground) {
            handler();
        }
    })
}

/// Subscribes a scoped handler invoked when the application enters background execution.
///
/// # Arguments
///
/// - `F` - Zero-argument callback invoked on `SystemEvent::Background`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_background<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut() + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if matches!(event, SystemEvent::Background) {
            handler();
        }
    })
}

/// Subscribes a scoped handler invoked when the application transitions to inactive status.
///
/// # Arguments
///
/// - `F` - Zero-argument callback invoked on `SystemEvent::Inactive`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_inactive<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut() + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if matches!(event, SystemEvent::Inactive) {
            handler();
        }
    })
}

/// Subscribes a scoped handler invoked when the application is terminating gracefully.
///
/// # Arguments
///
/// - `F` - Zero-argument callback invoked on `SystemEvent::Terminate`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_terminate<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut() + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if matches!(event, SystemEvent::Terminate) {
            handler();
        }
    })
}

/// Subscribes a scoped handler invoked whenever window focus changes.
///
/// # Arguments
///
/// - `F` - Callback receiving `true` on focus gain and `false` on focus loss.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_focus_change<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut(bool) + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if let SystemEvent::FocusChanged(focused) = event {
            handler(*focused);
        }
    })
}

/// Subscribes a scoped handler invoked whenever logical viewport size or DPI scale changes.
///
/// # Arguments
///
/// - `F` - Callback receiving `(width, height, scale_factor)`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_resize<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut(f32, f32, f32) + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if let SystemEvent::Resize {
            width,
            height,
            scale_factor,
        } = event
        {
            handler(*width, *height, *scale_factor);
        }
    })
}

/// Subscribes a scoped handler invoked whenever safe-area insets change.
///
/// # Arguments
///
/// - `F` - Callback receiving updated `SafeAreaInsets`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_safe_area_change<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut(SafeAreaInsets) + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if let SystemEvent::SafeAreaChanged(insets) = event {
            handler(*insets);
        }
    })
}

/// Subscribes a scoped handler invoked whenever screen orientation changes.
///
/// # Arguments
///
/// - `F` - Callback receiving updated `ScreenOrientation`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_orientation_change<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut(ScreenOrientation) + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if let SystemEvent::OrientationChanged(orient) = event {
            handler(*orient);
        }
    })
}

/// Subscribes a scoped handler invoked whenever host OS/browser light or dark theme changes.
///
/// # Arguments
///
/// - `F` - Callback receiving updated `SystemTheme`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_theme_change<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut(SystemTheme) + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if let SystemEvent::ThemeChanged(theme) = event {
            handler(*theme);
        }
    })
}

/// Subscribes a scoped handler invoked whenever host OS/browser preferred locale changes.
///
/// # Arguments
///
/// - `F` - Callback receiving the updated BCP-47 locale tag slice.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_locale_change<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut(&str) + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if let SystemEvent::LocaleChanged(locale) = event {
            handler(locale.as_str());
        }
    })
}

/// Subscribes a scoped handler invoked whenever soft keyboard visibility or height changes.
///
/// # Arguments
///
/// - `F` - Callback receiving `(visible, height_dp)`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_keyboard_change<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut(bool, f32) + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if let SystemEvent::KeyboardVisibilityChanged { visible, height } = event {
            handler(*visible, *height);
        }
    })
}

/// Subscribes a scoped handler invoked whenever the OS signals memory pressure.
///
/// # Arguments
///
/// - `F` - Callback receiving `MemoryPressure`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_memory_warning<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut(MemoryPressure) + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if let SystemEvent::MemoryWarning(pressure) = event {
            handler(*pressure);
        }
    })
}

/// Subscribes a scoped handler invoked whenever OS battery saver / low-power mode toggles.
///
/// # Arguments
///
/// - `F` - Callback receiving `true` when low-power mode is enabled.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_low_power_change<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut(bool) + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if let SystemEvent::LowPowerModeChanged(enabled) = event {
            handler(*enabled);
        }
    })
}

/// Subscribes a scoped handler invoked whenever host network connectivity status changes.
///
/// # Arguments
///
/// - `F` - Callback receiving updated `NetworkStatus`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_network_change<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut(NetworkStatus) + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if let SystemEvent::NetworkStatusChanged(status) = event {
            handler(*status);
        }
    })
}

/// Subscribes a scoped handler invoked whenever a system back gesture or key press occurs.
///
/// # Arguments
///
/// - `F` - Zero-argument callback invoked on `SystemEvent::BackPressed`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_system_back<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut() + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if matches!(event, SystemEvent::BackPressed) {
            handler();
        }
    })
}

/// Subscribes a scoped handler invoked whenever the host OS delivers a deep link URI.
///
/// # Arguments
///
/// - `F` - Callback receiving the deep link URI slice.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_deep_link<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut(&str) + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if let SystemEvent::DeepLink(uri) = event {
            handler(uri.as_str());
        }
    })
}

/// Subscribes a scoped handler invoked whenever the system clipboard changes.
///
/// # Arguments
///
/// - `F` - Zero-argument callback invoked on `SystemEvent::ClipboardChanged`.
///
/// # Returns
///
/// - `SystemEventSubscription` - Subscription handle auto-cleaned on `Scope` disposal.
pub fn on_clipboard_change<F>(mut handler: F) -> SystemEventSubscription
where
    F: FnMut() + 'static,
{
    on_system_event(move |event: &SystemEvent| {
        if matches!(event, SystemEvent::ClipboardChanged) {
            handler();
        }
    })
}

/// Returns the reactive `Signal<AppState>` from the active ambient `Context`.
///
/// # Returns
///
/// - `Signal<AppState>` - Reactive application lifecycle state signal.
#[must_use]
pub fn use_app_state() -> Signal<AppState> {
    current_context().app_state_signal()
}

/// Returns the reactive `Signal<SystemTheme>` from the active ambient `Context`.
///
/// # Returns
///
/// - `Signal<SystemTheme>` - Reactive OS light/dark theme signal.
#[must_use]
pub fn use_system_theme() -> Signal<SystemTheme> {
    current_context().get_theme_mode()
}

/// Returns the reactive `Signal<NetworkStatus>` from the active ambient `Context`.
///
/// # Returns
///
/// - `Signal<NetworkStatus>` - Reactive network connectivity status signal.
#[must_use]
pub fn use_network_status() -> Signal<NetworkStatus> {
    current_context().get_network_status()
}

/// Returns the reactive `Signal<f32>` carrying current soft keyboard height in logical pixels.
///
/// # Returns
///
/// - `Signal<f32>` - Reactive keyboard height signal (`0.0` when hidden).
#[must_use]
pub fn use_keyboard_height() -> Signal<f32> {
    current_context().get_keyboard_height()
}

/// Returns the reactive `Signal<SafeAreaInsets>` from the active ambient `Context`.
///
/// # Returns
///
/// - `Signal<SafeAreaInsets>` - Reactive display safe-area insets signal.
#[must_use]
pub fn use_safe_area() -> Signal<SafeAreaInsets> {
    current_context().get_safe_area()
}
