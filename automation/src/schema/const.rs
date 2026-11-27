/// Version identifier for the euv AI Runtime & Layout Introspection Schema v1 protocol.
pub const AI_SCHEMA_PROTOCOL_VERSION: &str = "euv.ai.schema.v1";

/// Version identifier for the euv AI Runtime & Layout Introspection Schema v2 protocol.
pub const AI_SCHEMA_V2_PROTOCOL_VERSION: &str = "euv.ai.schema.v2";

/// Canonical redaction marker replacing sensitive input or secret-keyed node content in AI snapshots.
pub const AI_SCHEMA_REDACTED_VALUE: &str = "[REDACTED]";

/// Formal JSON Schema specification describing `AiSchemaRequest`, `AiSchemaResponse`, and `AiRuntimeSnapshot`.
pub const AI_SCHEMA_JSON_SPEC: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "euv AI Runtime & Layout Schema v1/v2",
  "protocol": "euv.ai.schema.v1",
  "supportedProtocols": ["euv.ai.schema.v1", "euv.ai.schema.v2"],
  "devOnly": true,
  "httpEndpoints": [
    "GET /schema",
    "GET /ai/schema",
    "GET /snapshot",
    "GET /ai/snapshot",
    "GET /ai/snapshot/v2",
    "GET /ai/a11y",
    "GET /ai/health",
    "POST /query",
    "POST /ai/query",
    "POST /ai/action",
    "POST /ai/script",
    "POST /ai/e2e"
  ],
  "requests": [
    { "op": "DescribeSchema" },
    { "op": "Snapshot", "include_layout_tree": true },
    { "op": "SnapshotV2", "include_layout_tree": true, "include_rgba_hash": false },
    { "op": "AccessibilityTree" },
    { "op": "QueryNodes", "selector": { "type": "ByText", "value": "Overview" } },
    { "op": "HitTest", "x": 100.0, "y": 108.0 },
    { "op": "ExecuteAction", "action": { "kind": "Click", "selector": { "type": "ByText", "value": "Forms" } } },
    { "op": "RunTestScript", "script": { "name": "smoke", "steps": [] } }
  ],
  "selectors": [
    "ByText",
    "ByTextContains",
    "ByPlaceholder",
    "ByInputValue",
    "ByA11yLabel",
    "ByRole",
    "ByKind",
    "ByNodeIndex",
    "ByTestId",
    "ByStableKey",
    "Interactive",
    "Nth",
    "All"
  ],
  "actions": [
    "Click",
    "ClickAt",
    "DoubleClick",
    "Hover",
    "Focus",
    "TypeText",
    "ClearInput",
    "Drag",
    "DragPoints",
    "Gesture",
    "Scroll",
    "PressKey",
    "Resize",
    "StepFrames",
    "StepGame",
    "SetFpsLimit",
    "Navigate",
    "SetTheme",
    "GameControl"
  ],
  "snapshotFields": [
    "protocol_version",
    "schema_version",
    "frame_id",
    "scene_revision",
    "redacted_field_count",
    "window",
    "performance",
    "reactive",
    "interaction",
    "runtime",
    "game_stats",
    "accessibility_tree",
    "layout_tree",
    "anomalies"
  ]
}"#;

pub(crate) const KIND_VIEW: &str = "View";
pub(crate) const KIND_TEXT: &str = "Text";
pub(crate) const KIND_INPUT: &str = "Input";
pub(crate) const KIND_IMAGE: &str = "Image";
pub(crate) const KIND_SCROLL_VIEW: &str = "ScrollView";
pub(crate) const KIND_CANVAS: &str = "Canvas";
pub(crate) const KIND_PORTAL: &str = "Portal";
pub(crate) const KIND_FRAGMENT: &str = "Fragment";
pub(crate) const ROLE_BUTTON: &str = "Button";
pub(crate) const ROLE_TEXT_FIELD: &str = "TextField";
pub(crate) const ROLE_TEXT: &str = "Text";
pub(crate) const ROLE_SCROLL_CONTAINER: &str = "ScrollContainer";
pub(crate) const ROLE_CANVAS: &str = "Canvas";
pub(crate) const ROLE_DIALOG: &str = "Dialog";
pub(crate) const ROLE_IMAGE: &str = "Image";
pub(crate) const ROLE_CONTAINER: &str = "Container";
pub(crate) const A11Y_ROLE_GROUP: &str = "Group";
pub(crate) const A11Y_ROLE_WINDOW: &str = "Window";
pub(crate) const A11Y_ROLE_REGION: &str = "Region";
pub(crate) const A11Y_ROLE_NAVIGATION: &str = "Navigation";
pub(crate) const A11Y_ROLE_HEADING: &str = "Heading";
pub(crate) const A11Y_ROLE_TEXT_INPUT: &str = "TextInput";
pub(crate) const A11Y_ROLE_CHECKBOX: &str = "Checkbox";
pub(crate) const A11Y_ROLE_SWITCH: &str = "Switch";
pub(crate) const A11Y_ROLE_SLIDER: &str = "Slider";
pub(crate) const A11Y_ROLE_TAB: &str = "Tab";
pub(crate) const A11Y_ROLE_TAB_LIST: &str = "TabList";
pub(crate) const A11Y_ROLE_LIST: &str = "List";
pub(crate) const A11Y_ROLE_LIST_ITEM: &str = "ListItem";
pub(crate) const A11Y_ROLE_ALERT: &str = "Alert";
pub(crate) const A11Y_ROLE_PROGRESS_BAR: &str = "ProgressBar";
pub(crate) const A11Y_ROLE_SCROLL_VIEW: &str = "ScrollView";
pub(crate) const A11Y_ROLE_LINK: &str = "Link";
pub(crate) const A11Y_ROLE_MENU: &str = "Menu";
pub(crate) const A11Y_ROLE_MENU_ITEM: &str = "MenuItem";
pub(crate) const DISPLAY_FLEX: &str = "Flex";
pub(crate) const DISPLAY_BLOCK: &str = "Block";
pub(crate) const DISPLAY_GRID: &str = "Grid";
pub(crate) const DISPLAY_NONE: &str = "None";
pub(crate) const POSITION_RELATIVE: &str = "Relative";
pub(crate) const POSITION_ABSOLUTE: &str = "Absolute";
pub(crate) const POSITION_FIXED: &str = "Fixed";
pub(crate) const OVERFLOW_VISIBLE: &str = "Visible";
pub(crate) const OVERFLOW_HIDDEN: &str = "Hidden";
pub(crate) const OVERFLOW_SCROLL: &str = "Scroll";
pub(crate) const WEIGHT_LIGHT: &str = "Light";
pub(crate) const WEIGHT_NORMAL: &str = "Normal";
pub(crate) const WEIGHT_MEDIUM: &str = "Medium";
pub(crate) const WEIGHT_SEMI_BOLD: &str = "SemiBold";
pub(crate) const WEIGHT_BOLD: &str = "Bold";
pub(crate) const CURSOR_DEFAULT: &str = "Default";
pub(crate) const CURSOR_POINTER: &str = "Pointer";
pub(crate) const CURSOR_TEXT: &str = "Text";
pub(crate) const CURSOR_CROSSHAIR: &str = "Crosshair";
pub(crate) const CURSOR_MOVE: &str = "Move";
pub(crate) const CURSOR_NOT_ALLOWED: &str = "NotAllowed";
pub(crate) const CURSOR_EW_RESIZE: &str = "EwResize";
pub(crate) const CURSOR_NS_RESIZE: &str = "NsResize";
pub(crate) const CURSOR_GRAB: &str = "Grab";
pub(crate) const CURSOR_GRABBING: &str = "Grabbing";
pub(crate) const BREAKPOINT_COMPACT: &str = "Compact";
pub(crate) const BREAKPOINT_MEDIUM: &str = "Medium";
pub(crate) const BREAKPOINT_EXPANDED: &str = "Expanded";
pub(crate) const THEME_MODE_LIGHT: &str = "Light";
pub(crate) const THEME_MODE_DARK: &str = "Dark";
pub(crate) const RENDER_MODE_AUTO: &str = "Auto";
pub(crate) const RENDER_MODE_SOFTWARE: &str = "Software";
pub(crate) const RENDER_MODE_HARDWARE: &str = "Hardware";
pub(crate) const RENDER_MODE_HYBRID: &str = "Hybrid";
pub(crate) const BACKEND_SOFTWARE_CPU: &str = "SoftwareCpu";
pub(crate) const BACKEND_METAL: &str = "Metal";
pub(crate) const BACKEND_VULKAN: &str = "Vulkan";
pub(crate) const BACKEND_DIRECT3D12: &str = "Direct3D12";
pub(crate) const BACKEND_DIRECT3D11: &str = "Direct3D11";
pub(crate) const BACKEND_OPEN_GL_ES: &str = "OpenGlEs";
pub(crate) const BACKEND_WEB_GPU: &str = "WebGpu";
pub(crate) const BACKEND_WEB_GL2: &str = "WebGl2";
pub(crate) const BACKEND_CANVAS2D: &str = "Canvas2d";
pub(crate) const BACKEND_HEADLESS: &str = "Headless";
pub(crate) const PLATFORM_WEB: &str = "Web";
pub(crate) const PLATFORM_ANDROID: &str = "Android";
pub(crate) const PLATFORM_IOS: &str = "Ios";
pub(crate) const PLATFORM_IPAD_OS: &str = "IpadOs";
pub(crate) const PLATFORM_MAC_OS: &str = "MacOs";
pub(crate) const PLATFORM_WINDOWS: &str = "Windows";
pub(crate) const PLATFORM_LINUX: &str = "Linux";
pub(crate) const PLATFORM_HEADLESS: &str = "Headless";
pub(crate) const HEX_LOWER_DIGITS: &[u8; 16] = b"0123456789abcdef";
pub(crate) const TEST_ID_HINT_PREFIX: &str = "test_id:";
pub(crate) const KEY_HINT_PREFIX: &str = "key:";
pub(crate) const CONTROL_RESULT_FIELD: &str = "control_result";
pub(crate) const CAPTURE_SOURCE_SOFTWARE: &str = "software-framebuffer";
pub(crate) const CAPTURE_SOURCE_HYBRID: &str = "hybrid-framebuffer";
pub(crate) const CAPTURE_SOURCE_HARDWARE: &str = "hardware-readback";
pub(crate) const CAPTURE_SOURCE_NONE: &str = "none";
pub(crate) const DEFAULT_E2E_RUN_ID: &str = "dev-bridge-e2e";
pub(crate) const DEFAULT_E2E_APP_BUILD: &str = "dev-local";
pub(crate) const SENSITIVE_KEY_TOKENS: &[&str] = &[
    "password",
    "passwd",
    "secret",
    "token",
    "api_key",
    "apikey",
    "private_key",
    "credential",
    "ssn",
    "cvv",
];
pub(crate) const ERR_INVALID_REQUEST_JSON: &str = "Invalid AI schema request JSON";
pub(crate) const ERR_NO_NAVIGATE_HOOK: &str = "Navigate hook is not registered on this runtime";
pub(crate) const ERR_NO_THEME_HOOK: &str = "Theme hook is not registered on this runtime";
pub(crate) const ERR_NO_GAME_HOOK: &str = "GameControl hook is not registered on this runtime";
