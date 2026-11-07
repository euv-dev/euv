use super::*;

/// Implements conversion from `BridgeGroup` to a slice of permission strings.
impl BridgeGroup {
    /// Returns the list of permission strings for this group.
    ///
    /// # Returns
    ///
    /// - `&[&str]` - The permission strings belonging to this group.
    pub(crate) fn permissions(&self) -> &[&str] {
        match self {
            BridgeGroup::Camera => BRIDGE_GROUP_CAMERA,
            BridgeGroup::Microphone => BRIDGE_GROUP_MICROPHONE,
            BridgeGroup::Location => BRIDGE_GROUP_LOCATION,
            BridgeGroup::Storage => BRIDGE_GROUP_STORAGE,
            BridgeGroup::Bluetooth => BRIDGE_GROUP_BLUETOOTH,
            BridgeGroup::Contacts => BRIDGE_GROUP_CONTACTS,
            BridgeGroup::Calendar => BRIDGE_GROUP_CALENDAR,
            BridgeGroup::Phone => BRIDGE_GROUP_PHONE,
            BridgeGroup::Sms => BRIDGE_GROUP_SMS,
            BridgeGroup::Sensors => BRIDGE_GROUP_SENSORS,
            BridgeGroup::Nfc => BRIDGE_GROUP_NFC,
            BridgeGroup::Notifications => BRIDGE_GROUP_NOTIFICATIONS,
            BridgeGroup::All => BRIDGE_DANGEROUS_PERMISSIONS,
        }
    }
}

/// Implements `fmt::Display` for `BridgeGroup`.
impl fmt::Display for BridgeGroup {
    /// Formats the [`BridgeGroup`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - A `fmt::Result` value.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: &str = match self {
            BridgeGroup::Camera => BRIDGE_NAME_CAMERA,
            BridgeGroup::Microphone => BRIDGE_NAME_MICROPHONE,
            BridgeGroup::Location => BRIDGE_NAME_LOCATION,
            BridgeGroup::Storage => BRIDGE_NAME_STORAGE,
            BridgeGroup::Bluetooth => BRIDGE_NAME_BLUETOOTH,
            BridgeGroup::Contacts => BRIDGE_NAME_CONTACTS,
            BridgeGroup::Calendar => BRIDGE_NAME_CALENDAR,
            BridgeGroup::Phone => BRIDGE_NAME_PHONE,
            BridgeGroup::Sms => BRIDGE_NAME_SMS,
            BridgeGroup::Sensors => BRIDGE_NAME_SENSORS,
            BridgeGroup::Nfc => BRIDGE_NAME_NFC,
            BridgeGroup::Notifications => BRIDGE_NAME_NOTIFICATIONS,
            BridgeGroup::All => BRIDGE_NAME_ALL,
        };
        write!(f, "{name}")
    }
}

/// Implements `FromStr` for `BridgeGroup` to parse from string input.
impl FromStr for BridgeGroup {
    type Err = String;

    /// Implements the `from_str` operation on [`BridgeGroup`].
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - Result of the operation; an `Err` variant on failure.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            BRIDGE_NAME_CAMERA => Ok(BridgeGroup::Camera),
            BRIDGE_NAME_MICROPHONE | BRIDGE_NAME_AUDIO => Ok(BridgeGroup::Microphone),
            BRIDGE_NAME_LOCATION => Ok(BridgeGroup::Location),
            BRIDGE_NAME_STORAGE => Ok(BridgeGroup::Storage),
            BRIDGE_NAME_BLUETOOTH => Ok(BridgeGroup::Bluetooth),
            BRIDGE_NAME_CONTACTS => Ok(BridgeGroup::Contacts),
            BRIDGE_NAME_CALENDAR => Ok(BridgeGroup::Calendar),
            BRIDGE_NAME_PHONE => Ok(BridgeGroup::Phone),
            BRIDGE_NAME_SMS => Ok(BridgeGroup::Sms),
            BRIDGE_NAME_SENSORS => Ok(BridgeGroup::Sensors),
            BRIDGE_NAME_NFC => Ok(BridgeGroup::Nfc),
            BRIDGE_NAME_NOTIFICATIONS => Ok(BridgeGroup::Notifications),
            BRIDGE_NAME_ALL => Ok(BridgeGroup::All),
            other => Err(format!("Unknown bridge group: {other}")),
        }
    }
}
