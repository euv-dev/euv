package com.euv

object PermissionGroups {
    private val CAMERA = arrayOf(
        "android.permission.CAMERA"
    )
    private val MICROPHONE = arrayOf(
        "android.permission.RECORD_AUDIO"
    )
    private val LOCATION = arrayOf(
        "android.permission.ACCESS_FINE_LOCATION",
        "android.permission.ACCESS_COARSE_LOCATION",
        "android.permission.ACCESS_BACKGROUND_LOCATION",
        "android.permission.ACCESS_MEDIA_LOCATION"
    )
    private val STORAGE = arrayOf(
        "android.permission.READ_EXTERNAL_STORAGE",
        "android.permission.WRITE_EXTERNAL_STORAGE",
        "android.permission.READ_MEDIA_IMAGES",
        "android.permission.READ_MEDIA_VIDEO",
        "android.permission.READ_MEDIA_AUDIO"
    )
    private val BLUETOOTH = arrayOf(
        "android.permission.BLUETOOTH_SCAN",
        "android.permission.BLUETOOTH_CONNECT",
        "android.permission.BLUETOOTH_ADVERTISE"
    )
    private val CONTACTS = arrayOf(
        "android.permission.READ_CONTACTS",
        "android.permission.WRITE_CONTACTS",
        "android.permission.GET_ACCOUNTS"
    )
    private val CALENDAR = arrayOf(
        "android.permission.READ_CALENDAR",
        "android.permission.WRITE_CALENDAR"
    )
    private val PHONE = arrayOf(
        "android.permission.READ_PHONE_STATE",
        "android.permission.CALL_PHONE",
        "android.permission.ACCEPT_HANDOVER",
        "android.permission.READ_CALL_LOG",
        "android.permission.WRITE_CALL_LOG",
        "android.permission.READ_PHONE_NUMBERS",
        "android.permission.PROCESS_OUTGOING_CALLS"
    )
    private val SMS = arrayOf(
        "android.permission.SEND_SMS",
        "android.permission.RECEIVE_SMS",
        "android.permission.READ_SMS",
        "android.permission.RECEIVE_WAP_PUSH"
    )
    private val SENSORS = arrayOf(
        "android.permission.BODY_SENSORS",
        "android.permission.BODY_SENSORS_BACKGROUND",
        "android.permission.ACTIVITY_RECOGNITION",
        "android.permission.HIGH_SAMPLING_RATE_SENSORS"
    )
    private val NFC = arrayOf(
        "android.permission.NFC"
    )
    private val NOTIFICATIONS = arrayOf(
        "android.permission.POST_NOTIFICATIONS"
    )
    private val ALL = CAMERA + MICROPHONE + LOCATION + STORAGE + BLUETOOTH +
            CONTACTS + CALENDAR + PHONE + SMS + SENSORS + NFC + NOTIFICATIONS

    fun getPermissions(group: String): Array<String> {
        return when (group.lowercase()) {
            "camera" -> CAMERA
            "microphone", "audio" -> MICROPHONE
            "location" -> LOCATION
            "storage" -> STORAGE
            "bluetooth" -> BLUETOOTH
            "contacts" -> CONTACTS
            "calendar" -> CALENDAR
            "phone" -> PHONE
            "sms" -> SMS
            "sensors" -> SENSORS
            "nfc" -> NFC
            "notifications" -> NOTIFICATIONS
            "all" -> ALL
            else -> emptyArray()
        }
    }
}
