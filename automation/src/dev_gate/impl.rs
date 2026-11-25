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
                }
            }
            Err(_) => Self {
                listener: None,
                bound_addr: String::new(),
                requests_served: 0,
            },
        }
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
            ..
        } = self;
        let Some(tcp) = listener.as_ref() else {
            return 0;
        };
        let mut served: usize = 0;
        while served < 8 {
            match tcp.accept() {
                Ok((mut stream, _)) => {
                    Self::handle_stream(&mut stream, app);
                    *requests_served = requests_served.saturating_add(1);
                    served += 1;
                }
                Err(err) if err.kind() == ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }
        served
    }

    /// Handles a single incoming HTTP/JSON AI Schema connection on loopback.
    ///
    /// # Arguments
    ///
    /// - `&mut TcpStream` - Connected client socket.
    /// - `&mut App` - Active application runtime.
    fn handle_stream(stream: &mut TcpStream, app: &mut App) {
        drop(stream.set_nonblocking(false));
        drop(stream.set_read_timeout(Some(Duration::from_millis(50))));
        let mut buf: [u8; 8192] = [0; 8192];
        let Ok(n) = stream.read(&mut buf) else {
            return;
        };
        if n == 0 {
            return;
        }
        let raw: String = String::from_utf8_lossy(&buf[..n]).into_owned();
        let first_line: &str = raw.lines().next().unwrap_or("");
        let path: &str = first_line.split_whitespace().nth(1).unwrap_or("/");
        let body: &str = raw
            .split_once(HTTP_HEADER_SEP)
            .map(|(_, b): (&str, &str)| b.trim())
            .unwrap_or("");
        let json_body: String = if path == HTTP_PATH_SCHEMA {
            let resp: AiSchemaResponse =
                AiSchemaProtocol::handle(app, AiSchemaRequest::DescribeSchema);
            to_string(&resp).unwrap_or_default()
        } else if path == HTTP_PATH_SNAPSHOT {
            let resp: AiSchemaResponse = AiSchemaProtocol::handle(
                app,
                AiSchemaRequest::Snapshot {
                    include_layout_tree: true,
                },
            );
            to_string(&resp).unwrap_or_default()
        } else if path == HTTP_PATH_QUERY && !body.is_empty() {
            AiSchemaProtocol::handle_json(app, body)
        } else {
            let resp: AiSchemaResponse = AiSchemaProtocol::handle(
                app,
                AiSchemaRequest::Snapshot {
                    include_layout_tree: true,
                },
            );
            to_string(&resp).unwrap_or_default()
        };
        let reply: String = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
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
