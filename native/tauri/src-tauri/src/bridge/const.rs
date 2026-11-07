/// The list of all dangerous Android runtime permissions that this app may request.
pub(crate) const BRIDGE_DANGEROUS_PERMISSIONS: &[&str] = &[
    "android.permission.CAMERA",
    "android.permission.RECORD_AUDIO",
    "android.permission.READ_EXTERNAL_STORAGE",
    "android.permission.WRITE_EXTERNAL_STORAGE",
    "android.permission.READ_MEDIA_IMAGES",
    "android.permission.READ_MEDIA_VIDEO",
    "android.permission.READ_MEDIA_AUDIO",
    "android.permission.ACCESS_FINE_LOCATION",
    "android.permission.ACCESS_COARSE_LOCATION",
    "android.permission.ACCESS_BACKGROUND_LOCATION",
    "android.permission.ACCESS_MEDIA_LOCATION",
    "android.permission.BLUETOOTH_SCAN",
    "android.permission.BLUETOOTH_CONNECT",
    "android.permission.BLUETOOTH_ADVERTISE",
    "android.permission.READ_CONTACTS",
    "android.permission.WRITE_CONTACTS",
    "android.permission.GET_ACCOUNTS",
    "android.permission.READ_CALENDAR",
    "android.permission.WRITE_CALENDAR",
    "android.permission.READ_PHONE_STATE",
    "android.permission.CALL_PHONE",
    "android.permission.ACCEPT_HANDOVER",
    "android.permission.READ_CALL_LOG",
    "android.permission.WRITE_CALL_LOG",
    "android.permission.READ_PHONE_NUMBERS",
    "android.permission.SEND_SMS",
    "android.permission.RECEIVE_SMS",
    "android.permission.READ_SMS",
    "android.permission.RECEIVE_WAP_PUSH",
    "android.permission.BODY_SENSORS",
    "android.permission.BODY_SENSORS_BACKGROUND",
    "android.permission.ACTIVITY_RECOGNITION",
    "android.permission.HIGH_SAMPLING_RATE_SENSORS",
    "android.permission.POST_NOTIFICATIONS",
    "android.permission.NFC",
    "android.permission.PROCESS_OUTGOING_CALLS",
];

/// Camera group permissions.
pub(crate) const BRIDGE_GROUP_CAMERA: &[&str] = &["android.permission.CAMERA"];

/// Microphone group permissions.
pub(crate) const BRIDGE_GROUP_MICROPHONE: &[&str] = &["android.permission.RECORD_AUDIO"];

/// Location group permissions.
pub(crate) const BRIDGE_GROUP_LOCATION: &[&str] = &[
    "android.permission.ACCESS_FINE_LOCATION",
    "android.permission.ACCESS_COARSE_LOCATION",
    "android.permission.ACCESS_BACKGROUND_LOCATION",
    "android.permission.ACCESS_MEDIA_LOCATION",
];

/// Storage group permissions.
pub(crate) const BRIDGE_GROUP_STORAGE: &[&str] = &[
    "android.permission.READ_EXTERNAL_STORAGE",
    "android.permission.WRITE_EXTERNAL_STORAGE",
    "android.permission.READ_MEDIA_IMAGES",
    "android.permission.READ_MEDIA_VIDEO",
    "android.permission.READ_MEDIA_AUDIO",
];

/// Bluetooth group permissions.
pub(crate) const BRIDGE_GROUP_BLUETOOTH: &[&str] = &[
    "android.permission.BLUETOOTH_SCAN",
    "android.permission.BLUETOOTH_CONNECT",
    "android.permission.BLUETOOTH_ADVERTISE",
];

/// Contacts group permissions.
pub(crate) const BRIDGE_GROUP_CONTACTS: &[&str] = &[
    "android.permission.READ_CONTACTS",
    "android.permission.WRITE_CONTACTS",
    "android.permission.GET_ACCOUNTS",
];

/// Calendar group permissions.
pub(crate) const BRIDGE_GROUP_CALENDAR: &[&str] = &[
    "android.permission.READ_CALENDAR",
    "android.permission.WRITE_CALENDAR",
];

/// Phone group permissions.
pub(crate) const BRIDGE_GROUP_PHONE: &[&str] = &[
    "android.permission.READ_PHONE_STATE",
    "android.permission.CALL_PHONE",
    "android.permission.ACCEPT_HANDOVER",
    "android.permission.READ_CALL_LOG",
    "android.permission.WRITE_CALL_LOG",
    "android.permission.READ_PHONE_NUMBERS",
    "android.permission.PROCESS_OUTGOING_CALLS",
];

/// SMS group permissions.
pub(crate) const BRIDGE_GROUP_SMS: &[&str] = &[
    "android.permission.SEND_SMS",
    "android.permission.RECEIVE_SMS",
    "android.permission.READ_SMS",
    "android.permission.RECEIVE_WAP_PUSH",
];

/// Sensor group permissions.
pub(crate) const BRIDGE_GROUP_SENSORS: &[&str] = &[
    "android.permission.BODY_SENSORS",
    "android.permission.BODY_SENSORS_BACKGROUND",
    "android.permission.ACTIVITY_RECOGNITION",
    "android.permission.HIGH_SAMPLING_RATE_SENSORS",
];

/// NFC group permissions.
pub(crate) const BRIDGE_GROUP_NFC: &[&str] = &["android.permission.NFC"];

/// Notification group permissions.
pub(crate) const BRIDGE_GROUP_NOTIFICATIONS: &[&str] = &["android.permission.POST_NOTIFICATIONS"];

/// The wire name of the camera bridge permission group.
pub(crate) const BRIDGE_NAME_CAMERA: &str = "camera";

/// The wire name of the microphone bridge permission group.
pub(crate) const BRIDGE_NAME_MICROPHONE: &str = "microphone";

/// The wire name of the location bridge permission group.
pub(crate) const BRIDGE_NAME_LOCATION: &str = "location";

/// The wire name of the storage bridge permission group.
pub(crate) const BRIDGE_NAME_STORAGE: &str = "storage";

/// The wire name of the bluetooth bridge permission group.
pub(crate) const BRIDGE_NAME_BLUETOOTH: &str = "bluetooth";

/// The wire name of the contacts bridge permission group.
pub(crate) const BRIDGE_NAME_CONTACTS: &str = "contacts";

/// The wire name of the calendar bridge permission group.
pub(crate) const BRIDGE_NAME_CALENDAR: &str = "calendar";

/// The wire name of the phone bridge permission group.
pub(crate) const BRIDGE_NAME_PHONE: &str = "phone";

/// The wire name of the sms bridge permission group.
pub(crate) const BRIDGE_NAME_SMS: &str = "sms";

/// The wire name of the sensors bridge permission group.
pub(crate) const BRIDGE_NAME_SENSORS: &str = "sensors";

/// The wire name of the nfc bridge permission group.
pub(crate) const BRIDGE_NAME_NFC: &str = "nfc";

/// The wire name of the notifications bridge permission group.
pub(crate) const BRIDGE_NAME_NOTIFICATIONS: &str = "notifications";

/// The wire name of the all bridge permission group.
pub(crate) const BRIDGE_NAME_ALL: &str = "all";

/// The wire name of the audio bridge permission group.
pub(crate) const BRIDGE_NAME_AUDIO: &str = "audio";
