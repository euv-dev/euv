use super::*;

/// Generates a random UUID v4 string using `Math::random`.
///
/// # Returns
///
/// - `String` - A randomly generated UUID v4 string.
fn generate_uuid() -> String {
    let hex: String = (0..32)
        .map(|_: i32| format!("{:x}", (Math::random() * 16.0) as u8 & 0x0f))
        .collect();
    format!(
        "{}-{}-4{}-{}-{}",
        &hex[0..8],
        &hex[8..12],
        &hex[12..15],
        &hex[15..19],
        &hex[19..32]
    )
}

/// Formats a millisecond timestamp into a human-readable time string.
///
/// Uses `Date` to convert the timestamp to local time and formats
/// it as `HH:MM:SS`.
///
/// # Arguments
///
/// - `f64` - The millisecond timestamp.
///
/// # Returns
///
/// - `String` - The formatted time string.
fn format_timestamp(timestamp_ms: f64) -> String {
    let date: Date = Date::new(&JsValue::from_f64(timestamp_ms));
    let hours: u32 = date.get_hours();
    let minutes: u32 = date.get_minutes();
    let seconds: u32 = date.get_seconds();
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

/// Extracts the `data` and `time` fields from a JSON string and returns
/// a `WsMessage`. Falls back to the raw string as data and empty time
/// if JSON deserialization fails.
///
/// # Arguments
///
/// - `&str` - The raw JSON string received from the WebSocket.
///
/// # Returns
///
/// - `WsMessage` - The parsed message with display text and formatted time.
fn parse_ws_message(raw: &str) -> WsMessage {
    serde_json::from_str::<WsServerMessage>(raw)
        .map(|server_message: WsServerMessage| WsMessage {
            data: server_message.data,
            time: format_timestamp(server_message.time),
        })
        .unwrap_or_else(|_: Error| WsMessage {
            data: raw.to_string(),
            time: String::new(),
        })
}

/// Builds the WebSocket URL by appending a random UUID to the default prefix.
///
/// # Returns
///
/// - `String` - The full WebSocket URL with a random UUID query parameter.
fn build_websocket_url() -> String {
    format!("{WEBSOCKET_DEFAULT_URL_PREFIX}{}", generate_uuid())
}

/// Creates WebSocket state signals wrapped in a `UseWebSocket` struct.
///
/// # Returns
///
/// - `UseWebSocket` - The WebSocket connection state.
pub(crate) fn use_websocket() -> UseWebSocket {
    UseWebSocket {
        url: App::use_signal(build_websocket_url),
        connected: App::use_signal(|| false),
        connecting: App::use_signal(|| false),
        message_input: App::use_signal(String::new),
        messages: App::use_signal(Vec::new),
        error: App::use_signal(String::new),
        ping_handle: App::use_signal(|| None),
    }
}

/// Starts the Ping keep-alive interval timer.
///
/// Sends a Ping message every `WEBSOCKET_PING_INTERVAL_MS` milliseconds
/// to prevent the WebSocket connection from being closed due to inactivity.
///
/// # Arguments
///
/// - `Signal<Option<IntervalHandle>>` - The signal to store the interval handle.
fn start_ping_timer(ping_handle_signal: Signal<Option<IntervalHandle>>) {
    let handle: IntervalHandle = App::use_interval(WEBSOCKET_PING_INTERVAL_MS, move || {
        WS_INSTANCE.with(|instance: &RefCell<Option<WebSocket>>| {
            if let Some(socket) = instance.borrow().as_ref() {
                let _: Result<(), JsValue> = socket.send_with_str(WEBSOCKET_PING_MESSAGE);
            }
        });
    });
    ping_handle_signal.set(Some(handle));
}

/// Stops the Ping keep-alive interval timer if it is running.
///
/// # Arguments
///
/// - `Signal<Option<IntervalHandle>>` - The signal holding the interval handle.
fn stop_ping_timer(ping_handle_signal: Signal<Option<IntervalHandle>>) {
    if let Some(handle) = ping_handle_signal.get() {
        handle.clear();
        ping_handle_signal.set(None);
    }
}

/// Opens a WebSocket connection to the URL stored in state.
///
/// Creates a browser `WebSocket` to the URL, and sets up `onopen`,
/// `onmessage`, `onclose`, and `onerror` callbacks. On successful
/// connection, starts a Ping keep-alive timer. The WebSocket instance
/// is stored in a thread-local so it can be closed later.
///
/// A `connection_id` is captured at creation time and checked inside
/// each callback to prevent stale callbacks from a previous connection
/// from corrupting the state of the current connection.
///
/// # Arguments
///
/// - `UseWebSocket` - The WebSocket connection state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to open the WebSocket connection.
pub(crate) fn websocket_on_connect(state: UseWebSocket) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if state.get_connecting().get() || state.get_connected().get() {
            return;
        }
        let url: String = state.get_url().get();
        if url.is_empty() {
            state
                .get_error()
                .set("Please enter a valid WebSocket URL".to_string());
            return;
        }
        ws_close_instance();
        stop_ping_timer(state.get_ping_handle());
        let connection_id: usize = WS_CONNECTION_ID.with(|id: &Cell<usize>| {
            let next: usize = id.get() + 1;
            id.set(next);
            next
        });
        state.get_connecting().set(true);
        state.get_error().set(String::new());
        state.get_messages().set(Vec::new());
        let socket: WebSocket = match WebSocket::new(&url) {
            Ok(ws) => ws,
            Err(_) => {
                state.get_connecting().set(false);
                state
                    .get_error()
                    .set("Failed to create WebSocket".to_string());
                return;
            }
        };
        WS_INSTANCE.with(|instance: &RefCell<Option<WebSocket>>| {
            *instance.borrow_mut() = Some(socket.clone());
        });
        let on_open: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new({
            let state: UseWebSocket = state;
            move |_: JsValue| {
                state.get_connected().set(true);
                state.get_connecting().set(false);
                start_ping_timer(state.get_ping_handle());
                let mut messages: Vec<WsMessage> = state.get_messages().get();
                messages.push(WsMessage {
                    data: "[System] Connection established".to_string(),
                    time: String::new(),
                });
                state.get_messages().set(messages);
            }
        }));
        socket.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        on_open.forget();
        let on_message: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new({
            let state: UseWebSocket = state;
            move |event_value: JsValue| {
                let message_event: MessageEvent = event_value.unchecked_into();
                let data: JsValue = message_event.data();
                let raw: String = if data.is_string() {
                    data.as_string().unwrap_or_default()
                } else {
                    format!("{data:?}")
                };
                let ws_message: WsMessage = parse_ws_message(&raw);
                let mut messages: Vec<WsMessage> = state.get_messages().get();
                messages.push(ws_message);
                if messages.len() > WEBSOCKET_MAX_MESSAGES {
                    messages.drain(0..messages.len() - WEBSOCKET_MAX_MESSAGES);
                }
                state.get_messages().set(messages);
            }
        }));
        socket.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();
        let on_close: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new({
            let state: UseWebSocket = state;
            move |_: JsValue| {
                let current_id: usize = WS_CONNECTION_ID.with(|id: &Cell<usize>| id.get());
                if current_id != connection_id {
                    return;
                }
                WS_INSTANCE.with(|instance: &RefCell<Option<WebSocket>>| {
                    instance.borrow_mut().take();
                });
                state.get_connected().set(false);
                state.get_connecting().set(false);
                stop_ping_timer(state.get_ping_handle());
                let mut messages: Vec<WsMessage> = state.get_messages().get();
                messages.push(WsMessage {
                    data: "[System] Connection closed".to_string(),
                    time: String::new(),
                });
                state.get_messages().set(messages);
            }
        }));
        socket.set_onclose(Some(on_close.as_ref().unchecked_ref()));
        on_close.forget();
        let on_error: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new({
            let state: UseWebSocket = state;
            move |_: JsValue| {
                let current_id: usize = WS_CONNECTION_ID.with(|id: &Cell<usize>| id.get());
                if current_id != connection_id {
                    return;
                }
                WS_INSTANCE.with(|instance: &RefCell<Option<WebSocket>>| {
                    instance.borrow_mut().take();
                });
                state.get_connected().set(false);
                state.get_connecting().set(false);
                stop_ping_timer(state.get_ping_handle());
                state
                    .get_error()
                    .set("WebSocket error occurred".to_string());
                let mut messages: Vec<WsMessage> = state.get_messages().get();
                messages.push(WsMessage {
                    data: "[System] Connection lost due to error".to_string(),
                    time: String::new(),
                });
                state.get_messages().set(messages);
            }
        }));
        socket.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        on_error.forget();
    }))
}

/// Closes the WebSocket connection and resets state signals.
///
/// Stops the Ping keep-alive timer before closing the connection.
/// Increments `WS_CONNECTION_ID` to invalidate any in-flight
/// callbacks from the old connection.
///
/// # Arguments
///
/// - `UseWebSocket` - The WebSocket connection state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to disconnect the WebSocket.
pub(crate) fn websocket_on_disconnect(state: UseWebSocket) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        WS_CONNECTION_ID.with(|id: &Cell<usize>| {
            id.set(id.get() + 1);
        });
        stop_ping_timer(state.get_ping_handle());
        ws_close_instance();
        state.get_connected().set(false);
        state.get_connecting().set(false);
        state.get_error().set(String::new());
    }))
}

/// Sends a text message through the active WebSocket connection.
///
/// Serializes a `WsClientTextMessage` struct into JSON before sending.
///
/// # Arguments
///
/// - `UseWebSocket` - The WebSocket connection state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to send the message.
pub(crate) fn websocket_on_send(state: UseWebSocket) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let text: String = state.get_message_input().get();
        if text.is_empty() {
            return;
        }
        let message: WsClientTextMessage = WsClientTextMessage {
            r#type: "Text",
            data: text,
        };
        let body: String = serde_json::to_string(&message).unwrap_or_default();
        WS_INSTANCE.with(|instance: &RefCell<Option<WebSocket>>| {
            if let Some(socket) = instance.borrow().as_ref() {
                if socket.ready_state() == WebSocket::OPEN {
                    let _: Result<(), JsValue> = socket.send_with_str(&body);
                } else {
                    state.get_connected().set(false);
                    state
                        .get_error()
                        .set("Connection lost, please reconnect".to_string());
                }
            }
        });
        state.get_message_input().set(String::new());
    }))
}

/// Registers a cleanup callback that closes the WebSocket connection and stops
/// the Ping keep-alive timer when the component unmounts or the page route
/// switches away.
///
/// # Arguments
///
/// - `UseWebSocket` - The WebSocket connection state.
pub(crate) fn ws_cleanup(state: UseWebSocket) {
    App::use_cleanup(move || {
        stop_ping_timer(state.get_ping_handle());
        ws_close_instance();
        state.get_connected().set(false);
        state.get_connecting().set(false);
        state.get_error().set(String::new());
    });
}

/// Closes and removes the stored WebSocket from thread-local storage.
fn ws_close_instance() {
    WS_INSTANCE.with(|instance: &RefCell<Option<WebSocket>>| {
        if let Some(socket) = instance.borrow_mut().take() {
            let _: Result<(), JsValue> = socket.close();
        }
    });
}
