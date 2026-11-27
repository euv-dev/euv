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
pub(crate) const OVERFLOW_SCROLL: &str = "Scroll";
pub(crate) const DISPLAY_NONE: &str = "None";
pub(crate) const TEST_ID_HINT_PREFIX: &str = "test_id:";
pub(crate) const KEY_HINT_PREFIX: &str = "key:";
pub(crate) const THEME_MODE_DARK: &str = "Dark";
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
