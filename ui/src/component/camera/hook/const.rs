/// The CSS selector used to query the camera video element from the DOM.
pub(crate) const CAMERA_VIDEO_SELECTOR: &str = "#camera-video";

/// The facing mode constraint value for the user-facing (front) camera.
pub(crate) const CAMERA_FACING_MODE_USER: &str = "user";

/// The facing mode constraint value for the environment-facing (rear) camera.
pub(crate) const CAMERA_FACING_MODE_ENVIRONMENT: &str = "environment";

/// The interval in milliseconds between QR code scan attempts.
pub(crate) const CAMERA_SCAN_INTERVAL_MILLIS: i32 = 500;

/// The prefix used to detect URL strings in QR code scan results.
pub(crate) const CAMERA_URL_PREFIX_HTTP: &str = "http://";

/// The prefix used to detect secure URL strings in QR code scan results.
pub(crate) const CAMERA_URL_PREFIX_HTTPS: &str = "https://";

/// The hostname that represents the local loopback device.
pub(crate) const CAMERA_LOCALHOST_HOSTNAME: &str = "localhost";

/// The JS method name on a `BarcodeDetector` instance, cached across scans.
pub(crate) const CAMERA_DETECT_FN_KEY: &str = "detect";

/// The error returned when the page has no `window` to open a camera from.
pub(crate) const CAMERA_NO_WINDOW_ERROR: &str = "no global window exists";

/// The JS constraint key selecting which physical camera to open.
pub(crate) const CAMERA_FACING_MODE_KEY: &str = "facingMode";

/// The console prefix tagging every camera diagnostic message.
pub(crate) const CAMERA_LOG_PREFIX: &str = "[euv-camera]";

/// The no-op detector body used when the platform exposes no `detect` method.
pub(crate) const CAMERA_DETECT_FALLBACK_BODY: &str = "return Promise.resolve([])";

/// The JS global exposing the `BarcodeDetector` constructor.
pub(crate) const CAMERA_BARCODE_DETECTOR_KEY: &str = "BarcodeDetector";

/// The message set when the browser has no `BarcodeDetector` support.
pub(crate) const CAMERA_BARCODE_UNSUPPORTED_MESSAGE: &str =
    "BarcodeDetector API is not supported in this browser";

/// The only barcode format this scanner requests from `BarcodeDetector`.
pub(crate) const CAMERA_BARCODE_FORMAT_QR_CODE: &str = "qr_code";

/// The JS option key carrying the requested barcode formats.
pub(crate) const CAMERA_BARCODE_FORMATS_KEY: &str = "formats";

/// The JS result key holding a detected barcode's decoded text.
pub(crate) const CAMERA_BARCODE_RAW_VALUE_KEY: &str = "rawValue";

/// The JS method name opening a URL in a new browsing context.
pub(crate) const CAMERA_WINDOW_OPEN_KEY: &str = "open";
