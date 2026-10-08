/// The page title shown in the SSE demo header.
pub(crate) const SSE_DEMO_TITLE: &str = "Server-Sent Events";

/// The page subtitle shown under the SSE demo header.
pub(crate) const SSE_DEMO_SUBTITLE: &str = "Connect to an SSE endpoint and receive real-time streaming events from the server. Events appear in the list below as they arrive.";

/// The card title for the SSE connection controls.
pub(crate) const SSE_CONNECTION_CARD_TITLE: &str = "Connection";

/// The description text introducing the SSE endpoint URL input.
pub(crate) const SSE_CONNECTION_DESCRIPTION: &str =
    "Enter the SSE endpoint URL and click Connect to start receiving server-sent events.";

/// The label of the button shown while the SSE connection is being established.
pub(crate) const SSE_CONNECTING_BUTTON_LABEL: &str = "Wait";

/// The label of the button that closes an active SSE connection.
pub(crate) const SSE_DISCONNECT_BUTTON_LABEL: &str = "Close";

/// The label of the button that opens an SSE connection.
pub(crate) const SSE_CONNECT_BUTTON_LABEL: &str = "Connect";

/// The card title for the received SSE messages list.
pub(crate) const SSE_MESSAGES_CARD_TITLE: &str = "Messages";

/// The placeholder text shown before any SSE message has been received.
pub(crate) const SSE_MESSAGES_EMPTY_TEXT: &str =
    "No messages received yet. Connect to an SSE endpoint to start receiving events.";
