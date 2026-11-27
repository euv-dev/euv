use super::*;

/// Background non-blocking HTTP/JSON worker implementation isolating all network I/O and serialization off the UI thread.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
impl BridgeWorkerState {
    /// Runs the cooperative non-blocking accept, read, dispatch, and response-write loop until `shutdown` is raised.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable worker state.
    fn run_loop(&mut self) {
        let shutdown: Arc<AtomicBool> = {
            let Self { shutdown, .. } = self;
            Arc::clone(shutdown)
        };
        while !shutdown.load(Ordering::Acquire) {
            let mut progressed: bool = false;
            if self.accept_pending() {
                progressed = true;
            }
            if self.poll_reading_clients() {
                progressed = true;
            }
            if self.poll_waiting_replies() {
                progressed = true;
            }
            if !progressed {
                sleep(Duration::from_micros(BRIDGE_IDLE_SLEEP_US));
            }
        }
        self.drain_on_shutdown();
    }

    /// Accepts all queued non-blocking TCP client connections, rejecting excess sockets with HTTP `429` backpressure.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable worker state.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when at least one connection was accepted.
    fn accept_pending(&mut self) -> bool {
        let mut accepted: bool = false;
        let Self {
            listener,
            reading,
            waiting,
            backpressure_rejected,
            ..
        } = self;
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    accepted = true;
                    if reading.len().saturating_add(waiting.len()) >= HTTP_MAX_CONCURRENT_SOCKETS {
                        let _: u64 = backpressure_rejected.fetch_add(1, Ordering::AcqRel);
                        let resp: AiSchemaResponse = AiSchemaResponse::failure_with_code(
                            DEV_BRIDGE_ERR_BACKPRESSURE,
                            ERR_QUEUE_SATURATED.to_string(),
                        );
                        Self::write_json_response(
                            &mut stream,
                            HTTP_STATUS_TOO_MANY_REQUESTS,
                            &resp,
                        );
                        continue;
                    }
                    let _: io::Result<()> = stream.set_nonblocking(true);
                    reading.push(PendingClientRead {
                        stream,
                        raw_bytes: Vec::with_capacity(4096),
                        header_end: None,
                        content_length: 0,
                        deadline: Instant::now() + Duration::from_millis(HTTP_READ_TIMEOUT_MS),
                    });
                }
                Err(err) if err.kind() == ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }
        accepted
    }

    /// Polls all non-blocking reading sockets, dispatching complete HTTP requests off-thread or into the bounded UI channel.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable worker state.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when any socket read bytes or transitioned to waiting/completed.
    fn poll_reading_clients(&mut self) -> bool {
        let mut progressed: bool = false;
        let now: Instant = Instant::now();
        let mut remaining: Vec<PendingClientRead> = Vec::new();
        let Self {
            reading,
            waiting,
            ui_tx,
            requests_served,
            backpressure_rejected,
            pending_count,
            ..
        } = self;
        let active_reads: Vec<PendingClientRead> = take(reading);
        for mut client in active_reads.into_iter() {
            let mut chunk: [u8; 4096] = [0; 4096];
            let mut eof: bool = false;
            let mut fatal: bool = false;
            loop {
                match client.stream.read(&mut chunk) {
                    Ok(0) => {
                        eof = true;
                        break;
                    }
                    Ok(n) => {
                        progressed = true;
                        client.raw_bytes.extend_from_slice(&chunk[..n]);
                        if client.header_end.is_none()
                            && let Some(pos) = client
                                .raw_bytes
                                .windows(HTTP_HEADER_SEP_BYTES.len())
                                .position(|w: &[u8]| w == HTTP_HEADER_SEP_BYTES)
                        {
                            let body_start: usize = pos + HTTP_HEADER_SEP_BYTES.len();
                            client.header_end = Some(body_start);
                            let head_str: String =
                                String::from_utf8_lossy(&client.raw_bytes[..pos]).into_owned();
                            for line in head_str.lines().skip(1) {
                                if let Some((k, v)) = line.split_once(':')
                                    && k.trim().eq_ignore_ascii_case(HTTP_HEADER_CONTENT_LENGTH)
                                {
                                    client.content_length = v
                                        .trim()
                                        .parse::<usize>()
                                        .unwrap_or(0)
                                        .min(HTTP_MAX_BODY_BYTES);
                                }
                            }
                        }
                        if let Some(body_start) = client.header_end
                            && client.raw_bytes.len() >= body_start + client.content_length
                        {
                            break;
                        }
                        if client.raw_bytes.len() >= HTTP_MAX_BODY_BYTES + 16_384 {
                            break;
                        }
                    }
                    Err(err) if err.kind() == ErrorKind::WouldBlock => break,
                    Err(_) => {
                        fatal = true;
                        break;
                    }
                }
            }
            if fatal {
                continue;
            }
            let ready: bool = client.header_end.is_some_and(|body_start: usize| {
                client.raw_bytes.len() >= body_start + client.content_length
            }) || client.raw_bytes.len() >= HTTP_MAX_BODY_BYTES + 16_384
                || eof
                || now >= client.deadline;
            if !ready {
                remaining.push(client);
                continue;
            }
            progressed = true;
            let Some((method, path, body)) = Self::parse_http_buffer(
                &client.raw_bytes,
                client.header_end,
                client.content_length,
            ) else {
                continue;
            };
            let served_now: u64 = requests_served.load(Ordering::Acquire);
            let rejected_now: u64 = backpressure_rejected.load(Ordering::Acquire);
            let depth_now: usize = pending_count.load(Ordering::Acquire);
            match Self::route_http_request(
                &method,
                &path,
                &body,
                served_now,
                rejected_now,
                depth_now,
            ) {
                BridgeWorkerRoute::Immediate {
                    raw_http,
                    count_served,
                } => {
                    Self::write_raw_http(&mut client.stream, &raw_http);
                    if count_served {
                        let _: u64 = requests_served.fetch_add(1, Ordering::AcqRel);
                    }
                }
                BridgeWorkerRoute::UiRequest(request) => {
                    let (reply_tx, reply_rx): (
                        SyncSender<AiSchemaResponse>,
                        Receiver<AiSchemaResponse>,
                    ) = sync_channel(1);
                    let cmd: BridgeUiCommand = BridgeUiCommand {
                        request: *request,
                        reply_tx,
                    };
                    match ui_tx.try_send(cmd) {
                        Ok(()) => {
                            let _: usize = pending_count.fetch_add(1, Ordering::AcqRel);
                            waiting.push(WaitingClientReply {
                                stream: client.stream,
                                reply_rx,
                                deadline: Instant::now()
                                    + Duration::from_millis(HTTP_UI_REPLY_TIMEOUT_MS),
                            });
                        }
                        Err(TrySendError::Full(_)) => {
                            let _: u64 = backpressure_rejected.fetch_add(1, Ordering::AcqRel);
                            let resp: AiSchemaResponse = AiSchemaResponse::failure_with_code(
                                DEV_BRIDGE_ERR_BACKPRESSURE,
                                ERR_QUEUE_SATURATED.to_string(),
                            );
                            Self::write_json_response(
                                &mut client.stream,
                                HTTP_STATUS_TOO_MANY_REQUESTS,
                                &resp,
                            );
                        }
                        Err(TrySendError::Disconnected(_)) => {
                            let resp: AiSchemaResponse = AiSchemaResponse::failure_with_code(
                                ERR_CODE_BRIDGE_SHUTDOWN,
                                ERR_BRIDGE_SHUTDOWN.to_string(),
                            );
                            Self::write_json_response(
                                &mut client.stream,
                                HTTP_STATUS_SERVICE_UNAVAILABLE,
                                &resp,
                            );
                        }
                    }
                }
            }
        }
        *reading = remaining;
        progressed
    }

    /// Polls all sockets awaiting UI-thread results, serializes JSON responses off-thread, and writes HTTP replies.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable worker state.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when at least one waiting client completed or timed out.
    fn poll_waiting_replies(&mut self) -> bool {
        let mut progressed: bool = false;
        let now: Instant = Instant::now();
        let mut remaining: Vec<WaitingClientReply> = Vec::new();
        let Self {
            waiting,
            requests_served,
            ..
        } = self;
        let active_waiters: Vec<WaitingClientReply> = take(waiting);
        for mut waiter in active_waiters.into_iter() {
            match waiter.reply_rx.try_recv() {
                Ok(resp) => {
                    progressed = true;
                    Self::write_json_response(&mut waiter.stream, HTTP_STATUS_OK, &resp);
                    let _: u64 = requests_served.fetch_add(1, Ordering::AcqRel);
                }
                Err(TryRecvError::Disconnected) => {
                    progressed = true;
                    let resp: AiSchemaResponse = AiSchemaResponse::failure_with_code(
                        ERR_CODE_BRIDGE_SHUTDOWN,
                        ERR_BRIDGE_SHUTDOWN.to_string(),
                    );
                    Self::write_json_response(
                        &mut waiter.stream,
                        HTTP_STATUS_SERVICE_UNAVAILABLE,
                        &resp,
                    );
                }
                Err(TryRecvError::Empty) => {
                    if now >= waiter.deadline {
                        progressed = true;
                        let resp: AiSchemaResponse = AiSchemaResponse::failure_with_code(
                            ERR_CODE_UI_TIMEOUT,
                            ERR_UI_TIMEOUT.to_string(),
                        );
                        Self::write_json_response(
                            &mut waiter.stream,
                            HTTP_STATUS_GATEWAY_TIMEOUT,
                            &resp,
                        );
                    } else {
                        remaining.push(waiter);
                    }
                }
            }
        }
        *waiting = remaining;
        progressed
    }

    /// Responds cleanly with `503 Service Unavailable` to any sockets still waiting when the bridge shuts down.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable worker state.
    fn drain_on_shutdown(&mut self) {
        let Self {
            waiting, reading, ..
        } = self;
        let resp: AiSchemaResponse = AiSchemaResponse::failure_with_code(
            ERR_CODE_BRIDGE_SHUTDOWN,
            ERR_BRIDGE_SHUTDOWN.to_string(),
        );
        for mut waiter in waiting.drain(..) {
            Self::write_json_response(&mut waiter.stream, HTTP_STATUS_SERVICE_UNAVAILABLE, &resp);
        }
        reading.clear();
    }

    /// Extracts `(method, path, body)` from a buffered HTTP/1.1 request payload.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Raw accumulated request bytes.
    /// - `Option<usize>` - Parsed start offset of the HTTP body.
    /// - `usize` - Parsed `Content-Length`.
    ///
    /// # Returns
    ///
    /// - `Option<(String, String, String)>` - Parsed `(method, path, body)` tuple when non-empty.
    #[must_use]
    fn parse_http_buffer(
        raw_bytes: &[u8],
        header_end: Option<usize>,
        content_length: usize,
    ) -> Option<(String, String, String)> {
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

    /// Routes a parsed HTTP request either directly on the worker thread or into a decoded `AiSchemaRequest` for the UI thread.
    ///
    /// # Arguments
    ///
    /// - `&str` - HTTP method string.
    /// - `&str` - Normalized HTTP path without query parameters.
    /// - `&str` - Trimmed UTF-8 request body.
    /// - `u64` - Snapshot of served requests count.
    /// - `u64` - Snapshot of backpressure-rejected count.
    /// - `usize` - Snapshot of pending queue depth.
    ///
    /// # Returns
    ///
    /// - `BridgeWorkerRoute` - Immediate off-thread HTTP reply or decoded UI-thread request.
    #[must_use]
    fn route_http_request(
        method: &str,
        path: &str,
        body: &str,
        served_now: u64,
        rejected_now: u64,
        depth_now: usize,
    ) -> BridgeWorkerRoute {
        if method.eq_ignore_ascii_case(HTTP_METHOD_OPTIONS) {
            return BridgeWorkerRoute::Immediate {
                raw_http: HTTP_OPTIONS_RESPONSE.to_string(),
                count_served: true,
            };
        }
        if path == HTTP_PATH_AI_HEALTH {
            let payload: DevBridgeHealthPayload = DevBridgeHealthPayload::new(
                HEALTH_STATUS_OK.to_string(),
                served_now,
                rejected_now,
                depth_now,
                AI_SCHEMA_PROTOCOL_VERSION.to_string(),
                vec![
                    AI_SCHEMA_PROTOCOL_VERSION.to_string(),
                    AI_SCHEMA_V2_PROTOCOL_VERSION.to_string(),
                ],
            );
            let data: Value = to_value(payload).unwrap_or(Value::Null);
            let resp: AiSchemaResponse = AiSchemaResponse::success(data);
            return BridgeWorkerRoute::Immediate {
                raw_http: Self::format_json_http(HTTP_STATUS_OK, &resp),
                count_served: true,
            };
        }
        if path == HTTP_PATH_SCHEMA || path == HTTP_PATH_AI_SCHEMA {
            return BridgeWorkerRoute::UiRequest(Box::new(AiSchemaRequest::DescribeSchema));
        }
        if path == HTTP_PATH_SNAPSHOT || path == HTTP_PATH_AI_SNAPSHOT {
            return BridgeWorkerRoute::UiRequest(Box::new(AiSchemaRequest::Snapshot {
                include_layout_tree: true,
            }));
        }
        if path == HTTP_PATH_AI_SNAPSHOT_V2 {
            return BridgeWorkerRoute::UiRequest(Box::new(AiSchemaRequest::SnapshotV2 {
                include_layout_tree: true,
                include_rgba_hash: true,
            }));
        }
        if path == HTTP_PATH_AI_A11Y {
            return BridgeWorkerRoute::UiRequest(Box::new(AiSchemaRequest::AccessibilityTree));
        }
        if (path == HTTP_PATH_QUERY || path == HTTP_PATH_AI_QUERY) && !body.is_empty() {
            return match from_str::<AiSchemaRequest>(body) {
                Ok(req) => BridgeWorkerRoute::UiRequest(Box::new(req)),
                Err(err) => {
                    let resp: AiSchemaResponse = AiSchemaResponse::failure_with_code(
                        ERR_CODE_INVALID_JSON,
                        format!("{ERR_INVALID_REQUEST_JSON}: {err}"),
                    );
                    BridgeWorkerRoute::Immediate {
                        raw_http: Self::format_json_http(HTTP_STATUS_BAD_REQUEST, &resp),
                        count_served: false,
                    }
                }
            };
        }
        if path == HTTP_PATH_AI_ACTION && !body.is_empty() {
            return match from_str::<AiAction>(body) {
                Ok(action) => {
                    BridgeWorkerRoute::UiRequest(Box::new(AiSchemaRequest::ExecuteAction {
                        action,
                    }))
                }
                Err(err) => {
                    let resp: AiSchemaResponse = AiSchemaResponse::failure_with_code(
                        ERR_CODE_INVALID_JSON,
                        format!("{ERR_INVALID_ACTION_JSON}: {err}"),
                    );
                    BridgeWorkerRoute::Immediate {
                        raw_http: Self::format_json_http(HTTP_STATUS_BAD_REQUEST, &resp),
                        count_served: false,
                    }
                }
            };
        }
        if path == HTTP_PATH_AI_SCRIPT && !body.is_empty() {
            return match from_str::<UiTestScript>(body) {
                Ok(script) => {
                    BridgeWorkerRoute::UiRequest(Box::new(AiSchemaRequest::RunTestScript {
                        script,
                    }))
                }
                Err(err) => {
                    let resp: AiSchemaResponse = AiSchemaResponse::failure_with_code(
                        ERR_CODE_INVALID_JSON,
                        format!("{ERR_INVALID_SCRIPT_JSON}: {err}"),
                    );
                    BridgeWorkerRoute::Immediate {
                        raw_http: Self::format_json_http(HTTP_STATUS_BAD_REQUEST, &resp),
                        count_served: false,
                    }
                }
            };
        }
        if path == HTTP_PATH_AI_E2E && !body.is_empty() {
            return match from_str::<UiE2eFlowV1>(body) {
                Ok(flow) => BridgeWorkerRoute::UiRequest(Box::new(AiSchemaRequest::RunE2eFlow {
                    flow: Box::new(flow),
                    run_id: String::new(),
                    app_build: String::new(),
                })),
                Err(err) => {
                    let resp: AiSchemaResponse = AiSchemaResponse::failure_with_code(
                        ERR_CODE_INVALID_JSON,
                        format!("{ERR_INVALID_E2E_JSON}: {err}"),
                    );
                    BridgeWorkerRoute::Immediate {
                        raw_http: Self::format_json_http(HTTP_STATUS_BAD_REQUEST, &resp),
                        count_served: false,
                    }
                }
            };
        }
        BridgeWorkerRoute::UiRequest(Box::new(AiSchemaRequest::Snapshot {
            include_layout_tree: true,
        }))
    }

    /// Formats a JSON `AiSchemaResponse` into a complete HTTP/1.1 response string.
    ///
    /// # Arguments
    ///
    /// - `&str` - HTTP status line suffix (e.g. `"200 OK"`, `"429 Too Many Requests"`).
    /// - `&AiSchemaResponse` - Structured protocol response to encode off-thread.
    ///
    /// # Returns
    ///
    /// - `String` - Complete HTTP/1.1 response payload.
    #[must_use]
    fn format_json_http(status: &str, resp: &AiSchemaResponse) -> String {
        let json_body: String = to_string(resp).unwrap_or_default();
        format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            json_body.len(),
            json_body
        )
    }

    /// Encodes `resp` as JSON and writes the HTTP/1.1 reply on the background worker thread.
    ///
    /// # Arguments
    ///
    /// - `&mut TcpStream` - Connected client socket.
    /// - `&str` - HTTP status string.
    /// - `&AiSchemaResponse` - Response payload to serialize.
    fn write_json_response(stream: &mut TcpStream, status: &str, resp: &AiSchemaResponse) {
        let reply: String = Self::format_json_http(status, resp);
        Self::write_raw_http(stream, &reply);
    }

    /// Writes a pre-formatted HTTP/1.1 response payload with a bounded write timeout and flushes cleanly.
    ///
    /// # Arguments
    ///
    /// - `&mut TcpStream` - Connected client socket.
    /// - `&str` - Raw HTTP/1.1 response bytes.
    fn write_raw_http(stream: &mut TcpStream, raw_http: &str) {
        let _: io::Result<()> = stream.set_nonblocking(false);
        let _: io::Result<()> =
            stream.set_write_timeout(Some(Duration::from_millis(HTTP_WRITE_TIMEOUT_MS)));
        let _: io::Result<()> = stream.write_all(raw_http.as_bytes());
        let _: io::Result<()> = stream.flush();
    }
}

/// Dev-mode live non-blocking async HTTP/JSON AI Schema inspector implementation.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
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

    /// Starts an async non-blocking localhost HTTP/JSON AI Schema inspector bound to `addr` with the default bounded queue capacity (`16`).
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
        Self::start_with_capacity(addr, DEV_BRIDGE_QUEUE_CAPACITY)
    }

    /// Starts an async non-blocking localhost HTTP/JSON AI Schema inspector bound to `addr` with an explicit bounded control queue capacity.
    ///
    /// # Arguments
    ///
    /// - `&str` - Loopback bind address.
    /// - `usize` - Fixed bounded channel capacity (clamped to `>= 1`).
    ///
    /// # Returns
    ///
    /// - `Self` - Bound dev bridge with a resident background I/O worker thread.
    #[must_use]
    pub fn start_with_capacity(addr: &str, capacity: usize) -> Self {
        let queue_capacity: usize = capacity.max(1);
        let requests_served: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
        let backpressure_rejected: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
        let pending_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let shutdown: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        match TcpListener::bind(addr) {
            Ok(listener) => {
                let _: io::Result<()> = listener.set_nonblocking(true);
                let bound_addr: String = listener
                    .local_addr()
                    .map(|sa: SocketAddr| sa.to_string())
                    .unwrap_or_else(|_: io::Error| addr.to_string());
                let (ui_tx, ui_rx): (SyncSender<BridgeUiCommand>, Receiver<BridgeUiCommand>) =
                    sync_channel(queue_capacity);
                let mut worker_state: BridgeWorkerState = BridgeWorkerState {
                    listener,
                    ui_tx,
                    shutdown: Arc::clone(&shutdown),
                    requests_served: Arc::clone(&requests_served),
                    backpressure_rejected: Arc::clone(&backpressure_rejected),
                    pending_count: Arc::clone(&pending_count),
                    reading: Vec::new(),
                    waiting: Vec::new(),
                };
                let spawn_res: io::Result<JoinHandle<()>> = Builder::new()
                    .name(BRIDGE_WORKER_NAME.to_string())
                    .spawn(move || {
                        worker_state.run_loop();
                    });
                match spawn_res {
                    Ok(handle) => Self {
                        bound_addr,
                        queue_capacity,
                        active: true,
                        requests_served,
                        backpressure_rejected,
                        pending_count,
                        shutdown,
                        ui_rx: Some(ui_rx),
                        worker: Some(handle),
                        hooks: None,
                    },
                    Err(_) => Self {
                        bound_addr: String::new(),
                        queue_capacity,
                        active: false,
                        requests_served,
                        backpressure_rejected,
                        pending_count,
                        shutdown,
                        ui_rx: None,
                        worker: None,
                        hooks: None,
                    },
                }
            }
            Err(_) => Self {
                bound_addr: String::new(),
                queue_capacity,
                active: false,
                requests_served,
                backpressure_rejected,
                pending_count,
                shutdown,
                ui_rx: None,
                worker: None,
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
            active, bound_addr, ..
        } = self;
        if *active {
            Some(bound_addr.as_str())
        } else {
            None
        }
    }

    /// Returns total requests completed by the dev automation bridge.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Bridge reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Completed request count.
    #[must_use]
    pub fn requests_served(&self) -> u64 {
        let Self {
            requests_served, ..
        } = self;
        requests_served.load(Ordering::Acquire)
    }

    /// Returns total requests rejected with HTTP `429` / `BACKPRESSURE` due to bounded queue saturation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Bridge reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Backpressure rejection count.
    #[must_use]
    pub fn backpressure_rejected(&self) -> u64 {
        let Self {
            backpressure_rejected,
            ..
        } = self;
        backpressure_rejected.load(Ordering::Acquire)
    }

    /// Returns the fixed bounded control queue capacity configured for this bridge.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Bridge reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Bounded control channel capacity.
    #[must_use]
    pub fn queue_capacity(&self) -> usize {
        let Self { queue_capacity, .. } = self;
        *queue_capacity
    }

    /// Non-blockingly drains pending decoded UI commands strictly within `budget_us` microseconds without ever performing network or disk I/O on the UI thread.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable dev bridge.
    /// - `&mut App` - Active application runtime.
    /// - `u64` - Maximum UI-thread execution budget in microseconds.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of UI commands processed during this frame slice.
    pub fn poll_budgeted(&mut self, app: &mut App, budget_us: u64) -> usize {
        let Self {
            active,
            ui_rx,
            hooks,
            pending_count,
            ..
        } = self;
        if !*active {
            return 0;
        }
        let Some(rx) = ui_rx.as_ref() else {
            return 0;
        };
        let started: Instant = Instant::now();
        let budget_limit: u128 = budget_us as u128;
        let mut served: usize = 0;
        while served < 8 {
            if served > 0 && started.elapsed().as_micros() >= budget_limit {
                break;
            }
            match rx.try_recv() {
                Ok(cmd) => {
                    let _: usize = pending_count.fetch_sub(1, Ordering::AcqRel);
                    let resp: AiSchemaResponse =
                        AiSchemaProtocol::handle_with_hooks(app, cmd.request, hooks.as_ref());
                    let _: Result<(), TrySendError<AiSchemaResponse>> = cmd.reply_tx.try_send(resp);
                    served += 1;
                    if budget_limit == 0 || started.elapsed().as_micros() >= budget_limit {
                        break;
                    }
                }
                Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => break,
            }
        }
        served
    }

    /// Non-blockingly drains pending AI Schema UI commands against `app` within the default frame budget (`2000us`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable dev bridge.
    /// - `&mut App` - Active application runtime.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of UI commands processed on this tick.
    pub fn poll(&mut self, app: &mut App) -> usize {
        self.poll_budgeted(app, DEV_BRIDGE_DEFAULT_BUDGET_US)
    }
}

/// Clean shutdown of the resident background worker thread when `DevAutomationBridge` is dropped.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
impl Drop for DevAutomationBridge {
    /// Signals the background HTTP worker to exit and joins its thread cleanly.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable dev bridge being dropped.
    fn drop(&mut self) {
        let Self {
            shutdown,
            ui_rx,
            worker,
            ..
        } = self;
        shutdown.store(true, Ordering::Release);
        *ui_rx = None;
        if let Some(handle) = worker.take() {
            let _: thread::Result<()> = handle.join();
        }
    }
}

/// Zero-cost release/WASM no-op implementation of `DevAutomationBridge`.
#[cfg(not(all(debug_assertions, not(target_arch = "wasm32"))))]
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

    /// Zero-cost no-op bounded constructor in release builds.
    ///
    /// # Arguments
    ///
    /// - `&str` - Ignored address string.
    /// - `usize` - Ignored queue capacity.
    ///
    /// # Returns
    ///
    /// - `Self` - Zero-sized bridge stub.
    #[must_use]
    pub const fn start_with_capacity(_addr: &str, _capacity: usize) -> Self {
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

    /// Always returns `0` in release builds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Bridge stub reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Always `0`.
    #[must_use]
    pub const fn requests_served(&self) -> u64 {
        0
    }

    /// Always returns `0` in release builds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Bridge stub reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Always `0`.
    #[must_use]
    pub const fn backpressure_rejected(&self) -> u64 {
        0
    }

    /// Always returns `0` in release builds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Bridge stub reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Always `0`.
    #[must_use]
    pub const fn queue_capacity(&self) -> usize {
        0
    }

    /// Zero-cost no-op budgeted poll in release builds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable bridge stub.
    /// - `&mut App` - Ignored application reference.
    /// - `u64` - Ignored microsecond budget.
    ///
    /// # Returns
    ///
    /// - `usize` - Always `0`.
    pub fn poll_budgeted(&mut self, _app: &mut App, _budget_us: u64) -> usize {
        0
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
