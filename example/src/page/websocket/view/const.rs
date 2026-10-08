/// The title text for the header of the WebSocket chat page.
pub(crate) const WEBSOCKET_HEADER_TITLE: &str = "WebSocket Chat";

/// The subtitle text for the header of the WebSocket chat page.
pub(crate) const WEBSOCKET_HEADER_SUBTITLE: &str = "Connect to a WebSocket chat server with automatic UUID assignment and Ping keep-alive. Send and receive messages in real time.";

/// The card heading for the connection controls.
pub(crate) const WEBSOCKET_CONNECTION_CARD_TITLE: &str = "Connection";

/// The explanatory paragraph for the connection card.
pub(crate) const WEBSOCKET_CONNECTION_CARD_DESCRIPTION: &str = "A random UUID is generated for each session. Click Connect to establish a real-time bidirectional connection. Ping messages are sent automatically to keep the connection alive.";

/// The button label shown while the connection attempt is in flight.
pub(crate) const WEBSOCKET_WAIT_LABEL: &str = "Wait";

/// The button label that closes the open connection.
pub(crate) const WEBSOCKET_DISCONNECT_LABEL: &str = "Close";

/// The button label that opens the connection.
pub(crate) const WEBSOCKET_CONNECT_LABEL: &str = "Connect";

/// The card heading for the send-message input.
pub(crate) const WEBSOCKET_SEND_CARD_TITLE: &str = "Send Message";

/// The button label that sends the buffered message.
pub(crate) const WEBSOCKET_SEND_LABEL: &str = "Send";

/// The card heading for the received messages list.
pub(crate) const WEBSOCKET_MESSAGES_CARD_TITLE: &str = "Messages";

/// The placeholder text shown in the messages list before anything arrives.
pub(crate) const WEBSOCKET_MESSAGES_EMPTY_TEXT: &str =
    "No messages yet. Connect to start receiving.";
