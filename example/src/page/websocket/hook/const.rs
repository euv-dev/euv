/// The default WebSocket server URL prefix used for the demo.
pub(crate) const WEBSOCKET_DEFAULT_URL_PREFIX: &str = "wss://ltpp.vip/api/chat?uuid=";

/// The maximum number of WebSocket messages to keep in the display list.
pub(crate) const WEBSOCKET_MAX_MESSAGES: usize = 100;

/// The interval in milliseconds for sending Ping messages to keep the connection alive.
pub(crate) const WEBSOCKET_PING_INTERVAL_MS: i32 = 30000;

/// The JSON body for sending a Ping message through the WebSocket.
pub(crate) const WEBSOCKET_PING_MESSAGE: &str = r#"{"type":"Ping","data":""}"#;
