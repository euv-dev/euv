use super::*;

/// Dev-mode live non-blocking HTTP/JSON AI Schema inspector implementation.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
/// Inherent implementation of [`DevAutomationBridge`].
impl DevAutomationBridge {
    /// Returns `true` in dev builds (`debug_assertions` enabled) and `false` in release builds.
    ///
    /// # Returns
    ///
    /// - `bool` - Always `true` in dev builds.
    #[must_use]
    pub const fn is_enabled() -> bool {
        true
    }

    /// Starts a non-blocking localhost HTTP/JSON AI Schema inspector bound to `addr`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Loopback bind address (e.g. `"127.0.0.1:4174"` or `"127.0.0.1:0"`).
    ///
    /// # Returns
    ///
    /// - `Self` - Bound dev bridge (gracefully disabled if port is already occupied).
    #[must_use]
    pub fn start(addr: &str) -> Self {
        match TcpListener::bind(addr) {
            Ok(listener) => {
                drop(listener.set_nonblocking(true));
                let bound_addr: String = listener
                    .local_addr()
                    .map(|sa: SocketAddr| sa.to_string())
                    .unwrap_or_else(|_: Error| addr.to_string());
                Self {
                    listener: Some(listener),
                    bound_addr,
                    requests_served: 0,
                    hooks: None,
                }
            }
            Err(_) => Self {
                listener: None,
                bound_addr: String::new(),
                requests_served: 0,
                hooks: None,
            },
        }
    }

    /// Attaches host-application telemetry and interactive control hooks fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source dev bridge.
    /// - `AiAppHooks` - Host route/theme/game callbacks.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated dev bridge.
    #[must_use]
    pub fn with_hooks(mut self, hooks: AiAppHooks) -> Self {
        let Self { hooks: slot, .. } = &mut self;
        *slot = Some(hooks);
        self
    }

    /// Returns the actual bound loopback address if the dev inspector listener is active.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Bridge reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Bound address string.
    #[must_use]
    pub fn bound_address(&self) -> Option<&str> {
        let Self {
            listener,
            bound_addr,
            ..
        } = self;
        if listener.is_some() {
            Some(bound_addr.as_str())
        } else {
            None
        }
    }

    /// Non-blockingly drains pending AI Schema HTTP/JSON requests against `app` during dev frames.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable dev bridge.
    /// - `&mut App` - Active application runtime.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of requests served on this tick.
    pub fn poll(&mut self, app: &mut App) -> usize {
        let Self {
            listener,
            requests_served,
            hooks,
            ..
        } = self;
        let Some(tcp) = listener.as_ref() else {
            return 0;
        };
        let mut served: usize = 0;
        while served < 8 {
            match tcp.accept() {
                Ok((mut stream, _)) => {
                    Self::handle_stream(&mut stream, app, hooks.as_ref(), *requests_served);
                    *requests_served = requests_served.saturating_add(1);
                    served += 1;
                }
                Err(err) if err.kind() == ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }
        served
    }

    /// Reads a complete HTTP/1.1 request honoring `Content-Length` framing cleanly beyond 8 KiB.
    ///
    /// # Arguments
    ///
    /// - `&mut TcpStream` - Connected client socket.
    ///
    /// # Returns
    ///
    /// - `Option<(String, String, String)>` - `(method, path, body)` tuple when valid.
    fn read_http_request(stream: &mut TcpStream) -> Option<(String, String, String)> {
        let mut raw_bytes: Vec<u8> = Vec::with_capacity(4096);
        let mut chunk: [u8; 4096] = [0; 4096];
        let mut header_end: Option<usize> = None;
        let mut content_length: usize = 0;
        loop {
            match stream.read(&mut chunk) {
                Ok(0) => break,
                Ok(n) => {
                    raw_bytes.extend_from_slice(&chunk[..n]);
                    if header_end.is_none()
                        && let Some(pos) = raw_bytes
                            .windows(HTTP_HEADER_SEP_BYTES.len())
                            .position(|w: &[u8]| w == HTTP_HEADER_SEP_BYTES)
                    {
                        let body_start: usize = pos + HTTP_HEADER_SEP_BYTES.len();
                        header_end = Some(body_start);
                        let head_str: String =
                            String::from_utf8_lossy(&raw_bytes[..pos]).into_owned();
                        for line in head_str.lines().skip(1) {
                            if let Some((k, v)) = line.split_once(':')
                                && k.trim().eq_ignore_ascii_case(HTTP_HEADER_CONTENT_LENGTH)
                            {
                                content_length = v
                                    .trim()
                                    .parse::<usize>()
                                    .unwrap_or(0)
                                    .min(HTTP_MAX_BODY_BYTES);
                            }
                        }
                    }
                    if let Some(body_start) = header_end
                        && raw_bytes.len() >= body_start + content_length
                    {
                        break;
                    }
                    if raw_bytes.len() >= HTTP_MAX_BODY_BYTES + 16_384 {
                        break;
                    }
                }
                Err(err)
                    if err.kind() == ErrorKind::WouldBlock || err.kind() == ErrorKind::TimedOut =>
                {
                    break;
                }
                Err(_) => return None,
            }
        }
        if raw_bytes.is_empty() {
            return None;
        }
        let body_start: usize = header_end.unwrap_or(raw_bytes.len());
        let header_slice: &[u8] = &raw_bytes[..body_start.min(raw_bytes.len())];
        let head_text: String = String::from_utf8_lossy(header_slice).into_owned();
        let first_line: &str = head_text.lines().next().unwrap_or("");
        let mut parts: SplitWhitespace<'_> = first_line.split_whitespace();
        let method: String = parts.next().unwrap_or("GET").to_string();
        let raw_path: &str = parts.next().unwrap_or("/");
        let path: String = raw_path
            .split_once('?')
            .map(|(p, _): (&str, &str)| p)
            .unwrap_or(raw_path)
            .to_string();
        let body_end: usize = (body_start + content_length).min(raw_bytes.len());
        let body: String = if body_start < raw_bytes.len() {
            let slice: &[u8] = if content_length > 0 {
                &raw_bytes[body_start..body_end]
            } else {
                &raw_bytes[body_start..]
            };
            String::from_utf8_lossy(slice).trim().to_string()
        } else {
            String::new()
        };
        Some((method, path, body))
    }

    /// Handles a single incoming HTTP/JSON AI Schema connection on loopback.
    ///
    /// # Arguments
    ///
    /// - `&mut TcpStream` - Connected client socket.
    /// - `&mut App` - Active application runtime.
    /// - `Option<&AiAppHooks>` - Optional host telemetry/control hooks.
    /// - `u64` - Requests served prior to this connection.
    fn handle_stream(
        stream: &mut TcpStream,
        app: &mut App,
        hooks: Option<&AiAppHooks>,
        requests_served: u64,
    ) {
        drop(stream.set_nonblocking(false));
        drop(stream.set_read_timeout(Some(Duration::from_millis(200))));
        let Some((method, path, body)) = Self::read_http_request(stream) else {
            return;
        };
        if method.eq_ignore_ascii_case(HTTP_METHOD_OPTIONS) {
            drop(stream.write_all(HTTP_OPTIONS_RESPONSE.as_bytes()));
            drop(stream.flush());
            return;
        }
        let json_body: String = if path == HTTP_PATH_SCHEMA || path == HTTP_PATH_AI_SCHEMA {
            let resp: AiSchemaResponse =
                AiSchemaProtocol::handle_with_hooks(app, AiSchemaRequest::DescribeSchema, hooks);
            to_string(&resp).unwrap_or_default()
        } else if path == HTTP_PATH_SNAPSHOT || path == HTTP_PATH_AI_SNAPSHOT {
            let resp: AiSchemaResponse = AiSchemaProtocol::handle_with_hooks(
                app,
                AiSchemaRequest::Snapshot {
                    include_layout_tree: true,
                },
                hooks,
            );
            to_string(&resp).unwrap_or_default()
        } else if path == HTTP_PATH_AI_A11Y {
            let resp: AiSchemaResponse =
                AiSchemaProtocol::handle_with_hooks(app, AiSchemaRequest::AccessibilityTree, hooks);
            to_string(&resp).unwrap_or_default()
        } else if path == HTTP_PATH_AI_HEALTH {
            let resp: AiSchemaResponse = AiSchemaResponse::success(json!({
                "status": "ok",
                "requests_served": requests_served,
                "protocol": AI_SCHEMA_PROTOCOL_VERSION,
            }));
            to_string(&resp).unwrap_or_default()
        } else if (path == HTTP_PATH_QUERY || path == HTTP_PATH_AI_QUERY) && !body.is_empty() {
            AiSchemaProtocol::handle_json_with_hooks(app, &body, hooks)
        } else if path == HTTP_PATH_AI_ACTION && !body.is_empty() {
            let resp: AiSchemaResponse = match from_str::<AiAction>(&body) {
                Ok(action) => AiSchemaProtocol::handle_with_hooks(
                    app,
                    AiSchemaRequest::ExecuteAction { action },
                    hooks,
                ),
                Err(err) => AiSchemaResponse::failure(format!("{ERR_INVALID_ACTION_JSON}: {err}")),
            };
            to_string(&resp).unwrap_or_default()
        } else if path == HTTP_PATH_AI_SCRIPT && !body.is_empty() {
            let resp: AiSchemaResponse = match from_str::<UiTestScript>(&body) {
                Ok(script) => AiSchemaProtocol::handle_with_hooks(
                    app,
                    AiSchemaRequest::RunTestScript { script },
                    hooks,
                ),
                Err(err) => AiSchemaResponse::failure(format!("{ERR_INVALID_SCRIPT_JSON}: {err}")),
            };
            to_string(&resp).unwrap_or_default()
        } else {
            let resp: AiSchemaResponse = AiSchemaProtocol::handle_with_hooks(
                app,
                AiSchemaRequest::Snapshot {
                    include_layout_tree: true,
                },
                hooks,
            );
            to_string(&resp).unwrap_or_default()
        };
        let reply: String = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            json_body.len(),
            json_body
        );
        drop(stream.write_all(reply.as_bytes()));
        drop(stream.flush());
    }
}

/// Zero-cost release/WASM no-op implementation of `DevAutomationBridge`.
#[cfg(not(all(debug_assertions, not(target_arch = "wasm32"))))]
/// Inherent implementation of [`DevAutomationBridge`].
impl DevAutomationBridge {
    /// Returns `true` when `debug_assertions` is enabled (`false` in release builds).
    ///
    /// # Returns
    ///
    /// - `bool` - Whether dev automation is active.
    #[must_use]
    pub const fn is_enabled() -> bool {
        cfg!(debug_assertions)
    }

    /// Zero-cost no-op constructor in release builds.
    ///
    /// # Arguments
    ///
    /// - `&str` - Ignored address string.
    ///
    /// # Returns
    ///
    /// - `Self` - Zero-sized bridge stub.
    #[must_use]
    pub const fn start(_addr: &str) -> Self {
        Self
    }

    /// Zero-cost no-op hook attachment in release builds.
    ///
    /// # Arguments
    ///
    /// - `Self` - Bridge stub.
    /// - `AiAppHooks` - Ignored hooks.
    ///
    /// # Returns
    ///
    /// - `Self` - Zero-sized bridge stub.
    #[must_use]
    pub fn with_hooks(self, _hooks: AiAppHooks) -> Self {
        self
    }

    /// Always returns `None` in release builds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Bridge stub reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Always `None`.
    #[must_use]
    pub const fn bound_address(&self) -> Option<&str> {
        None
    }

    /// Zero-cost no-op poll in release builds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable bridge stub.
    /// - `&mut App` - Ignored application reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Always `0`.
    pub fn poll(&mut self, _app: &mut App) -> usize {
        0
    }
}
