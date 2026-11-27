---
title: Cross-Platform System Events & Lifecycle Hooks
order: 5
section: guide
tagline: Unified AppState machine, 20 auto-cleaned scoped hooks, reactive system signals, #[component] lifecycle callbacks, and EuvRouterOutlet page Appear/Disappear dispatch across all 7 platforms.
---

# Cross-Platform System Events & Lifecycle Hooks

`euv` normalizes OS-specific lifecycle, window, display, and environment signals across **macOS, Windows, Linux (X11), iOS, iPadOS, Android, WebAssembly, and Headless** into one deterministic `SystemEvent` stream and `AppState` state machine.

---

## Canonical `AppState` & Event Normalization

`SystemEventNormalizer` deduplicates duplicate platform callbacks and guarantees identical event ordering on every OS:

- **Entering background** from `Active`: emits `SystemEvent::Inactive` -> `SystemEvent::Disappear` -> `SystemEvent::Background` (`AppState::Background`).
- **Returning to foreground** from `Background`: emits `SystemEvent::Appear` -> `SystemEvent::Foreground` (`AppState::Active`).
- **Post-termination suppression**: once `SystemEvent::Terminate` fires (`AppState::Terminated`), subsequent redundant lifecycle events are ignored.
- **Orientation derivation**: viewport `Resize` events crossing portrait/landscape boundaries automatically emit `SystemEvent::OrientationChanged`.

| Category            | Variants                                                                                                                        |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------ |
| Lifecycle           | `Appear`, `Disappear`, `Foreground`, `Background`, `Inactive`, `Terminate`                                                      |
| Window & Display    | `FocusChanged(bool)`, `Resize { width, height, scale_factor }`, `SafeAreaChanged`, `OrientationChanged`, `ThemeChanged`         |
| Input & Environment | `LocaleChanged(String)`, `KeyboardVisibilityChanged { visible, height }`, `DeepLink(String)`, `ClipboardChanged`, `BackPressed` |
| System Resources    | `MemoryWarning(MemoryPressure)`, `LowPowerModeChanged(bool)`, `NetworkStatusChanged(NetworkStatus)`                             |

---

## Reactive System Signals & Scoped Hooks

Every hook registered inside a reactive `Scope` (including routed pages) automatically unregisters on `Scope::dispose()` and suspends while frozen (`PageRetention::Freeze`):

```rust
let app_state: Signal<AppState> = use_app_state();
let sys_theme: Signal<SystemTheme> = use_system_theme();
let network: Signal<NetworkStatus> = use_network_status();
let keyboard_h: Signal<f32> = use_keyboard_height();
let safe_area: Signal<SafeAreaInsets> = use_safe_area();

let _sub_all: SystemEventSubscription = on_system_event(move |ev: &SystemEvent| {
    let _: &SystemEvent = ev;
});
let _sub_bg: SystemEventSubscription = on_background(move || {
    // Flush draft state or pause background telemetry
});
let _sub_net: SystemEventSubscription = on_network_change(move |status: NetworkStatus| {
    let _: bool = status.is_online();
});
```

Available typed hooks: `on_system_event`, `on_appear`, `on_disappear`, `on_foreground`, `on_background`, `on_inactive`, `on_terminate`, `on_focus_change`, `on_resize`, `on_safe_area_change`, `on_orientation_change`, `on_theme_change`, `on_locale_change`, `on_keyboard_change`, `on_memory_warning`, `on_low_power_change`, `on_network_change`, `on_system_back`, `on_deep_link`, and `on_clipboard_change`.

---

## `#[component]` Lifecycle Hooks & `EuvRouterOutlet`

Components annotated with `#[component]` can implement optional system and visibility callbacks directly:

```rust
#[component(retention = Freeze)]
impl AnalyticsPage {
    fn on_appear(&self) {
        // Fires on initial mount, Frozen resume, and OS foreground return
    }

    fn on_disappear(&self) {
        // Fires when leaving the route or entering OS background
    }

    fn on_background(&self) {
        // Persist in-flight drafts before OS suspension
    }

    fn on_system_event(&self, _event: &SystemEvent) {}

    fn render(&self) -> NodeId {
        let app_state: Signal<AppState> = use_app_state();
        let status_text = move || -> String { format!("State: {:?}", app_state.get()) };
        view! {
            column {
                text {
                    { status_text }
                }
            }
        }
    }
}
```

When navigating routes inside `EuvRouterOutlet`, outgoing pages automatically receive `ctx.emit_subtree_disappear(outgoing_wrapper)` before freezing or destroying, and incoming or resumed pages receive `ctx.emit_subtree_appear(incoming_wrapper)` right after activation.
