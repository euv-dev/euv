/// Categorizes Android permissions into logical groups for batch operations.
pub(crate) enum BridgeGroup {
    /// Camera-related permissions.
    Camera,
    /// Microphone and audio recording permissions.
    Microphone,
    /// Location permissions (fine, coarse, background).
    Location,
    /// Storage and media permissions.
    Storage,
    /// Bluetooth permissions.
    Bluetooth,
    /// Contact-related permissions.
    Contacts,
    /// Calendar permissions.
    Calendar,
    /// Phone and call-related permissions.
    Phone,
    /// SMS permissions.
    Sms,
    /// Sensor permissions.
    Sensors,
    /// NFC permissions.
    Nfc,
    /// Notification permissions.
    Notifications,
    /// All dangerous runtime permissions combined.
    All,
}
