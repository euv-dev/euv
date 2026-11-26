use super::*;

/// Declarative content payload for iOS Dynamic Island, Live Activities, and cross-platform live status pills.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct LiveActivityContent {
    /// Expanded and Lock Screen primary title.
    pub(crate) title: String,
    /// Expanded and Lock Screen supporting status text.
    pub(crate) subtitle: String,
    /// Dynamic Island compact leading slot text or icon tag.
    #[new(skip)]
    pub(crate) compact_leading: String,
    /// Dynamic Island compact trailing slot metric or countdown text.
    #[new(skip)]
    pub(crate) compact_trailing: String,
    /// Dynamic Island minimal circular badge text.
    #[new(skip)]
    pub(crate) minimal_badge: String,
    /// Optional completion progress in `0..=100`.
    #[new(skip)]
    #[get(skip)]
    pub(crate) progress_percent: Option<u8>,
    /// Accent color RGB hex (`0xRRGGBB`) for tinting the island or progress ring.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) accent_hex: u32,
}

/// Active handle tracking a running Dynamic Island / Live Activity session.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct LiveActivityHandle {
    /// Unique application-scoped activity identifier.
    pub(crate) activity_id: String,
    /// Platform hosting the live activity.
    #[get(type(copy))]
    pub(crate) platform: PlatformKind,
    /// Resolved native presentation surface (`Compact`, `LockScreen`, `FallbackStatusBar`, etc.).
    #[get(type(copy))]
    pub(crate) presentation: IslandPresentationMode,
    /// Current rendered live activity content.
    pub(crate) content: LiveActivityContent,
    /// Whether the live activity is currently active.
    #[get(type(copy))]
    pub(crate) active: bool,
}

/// Cross-platform native system notification payload.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct SystemNotification {
    /// Stable notification identifier for replacement or dismissal.
    pub(crate) id: String,
    /// Notification heading.
    pub(crate) title: String,
    /// Notification message body.
    pub(crate) body: String,
    /// Optional app icon badge count.
    #[new(skip)]
    #[get(skip)]
    pub(crate) badge_count: Option<u32>,
    /// Suppresses alert sound when `true`.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) silent: bool,
}

/// Desktop system tray / macOS menu bar icon configuration.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct SystemTrayConfig {
    /// Hover tooltip text.
    pub(crate) tooltip: String,
    /// Optional inline title shown in macOS menu bar or Linux tray.
    #[new(skip)]
    pub(crate) title: String,
    /// Ordered context menu action labels.
    #[new(skip)]
    pub(crate) menu_items: Vec<String>,
}

/// Cross-platform native OS share sheet payload (`UIActivityViewController`, `NSSharingServicePicker`, Android `Intent`, Web Share).
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct SharePayload {
    /// Share dialog title.
    pub(crate) title: String,
    /// Body text to share.
    pub(crate) text: String,
    /// Optional URL to attach.
    #[new(skip)]
    pub(crate) url: String,
}

/// Unified per-app hub coordinating permissions, protocols, Dynamic Island / Live Activities, haptics, tray, and A11y announcements.
#[derive(Clone, Data, Debug, Default, New)]
pub struct PlatformServiceHub {
    /// Active platform kind.
    #[get(type(copy))]
    pub(crate) platform: PlatformKind,
    /// Unified system permission manager.
    #[new(skip)]
    pub(crate) permissions: PermissionManager,
    /// Custom URL protocol & deep link registry.
    #[new(skip)]
    pub(crate) protocols: ProtocolRegistry,
    /// Running and ended Dynamic Island / Live Activities.
    #[new(skip)]
    pub(crate) live_activities: Vec<LiveActivityHandle>,
    /// Dispatched system notifications.
    #[new(skip)]
    pub(crate) notifications: Vec<SystemNotification>,
    /// Queued screen reader live announcements.
    #[new(skip)]
    pub(crate) announcements: Vec<A11yAnnouncement>,
    /// Active desktop system tray configuration, if installed.
    #[new(skip)]
    #[get(skip)]
    pub(crate) tray: Option<SystemTrayConfig>,
    /// Total haptic pulses emitted.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) haptic_count: u64,
}
