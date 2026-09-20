use super::*;

/// Implements `ServerHook` for `RequestMiddleware` to inject cache-control headers.
impl ServerHook for RequestMiddleware {
    /// Creates a new `RequestMiddleware` instance.
    ///
    /// # Arguments
    /// - `&mut Stream` - The connection stream (unused).
    /// - `&mut Context` - The request context (unused).
    ///
    /// # Returns
    /// - `RequestMiddleware` - A new instance with no internal state.
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    /// Injects cache-control headers to prevent stale WASM assets during development.
    ///
    /// Sets `Cache-Control: no-cache, no-store, must-revalidate`,
    /// `Pragma: no-cache`, and `Expires: 0` on the response.
    ///
    /// # Arguments
    /// - `self` - The consumed middleware instance.
    /// - `&mut Stream` - The connection stream (unused).
    /// - `&mut Context` - The request context used to set response headers.
    ///
    /// # Returns
    /// - `Status` - The hook processing result.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CACHE_CONTROL, NO_CACHE_NO_STORE_MUST_REVALIDATE)
            .set_header(PRAGMA, NO_CACHE)
            .set_header(EXPIRES, EXPIRES_DISABLED);
        Status::Continue
    }
}

/// Implements `ServerHook` for `ResponseMiddleware` to serialize and send the response.
impl ServerHook for ResponseMiddleware {
    /// Creates a new `ResponseMiddleware` instance.
    ///
    /// # Arguments
    /// - `&mut Stream` - The connection stream (unused).
    /// - `&mut Context` - The request context (unused).
    ///
    /// # Returns
    /// - `ResponseMiddleware` - A new instance with no internal state.
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    /// Builds the HTTP response bytes and sends them through the connection stream.
    ///
    /// If the send fails, marks the stream as closed and rejects the request.
    ///
    /// # Arguments
    /// - `self` - The consumed middleware instance.
    /// - `&mut Stream` - The connection stream used to send the response bytes.
    /// - `&mut Context` - The request context used to build the response.
    ///
    /// # Returns
    /// - `Status` - The hook processing result.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let response: Vec<u8> = ctx.get_mut_response().build();
        if stream.try_send(&response).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        Status::Continue
    }
}

/// Implements `ServerHook` for `RootRoute` to serve the development HTML at `/`.
///
/// The bare root path is what most browsers show when a user types
/// `http://host:port` without a path component. Without this route the
/// `IndexRoute` (registered at `{serving_route_prefix}/{path:.*}`) does
/// not match and hyperlane's default empty 200 is returned — the
/// "white screen" symptom. This route always serves the in-memory
/// development HTML (same as `IndexRoute`'s `index.html` case) and
/// applies the same 500 fallback when the global state is missing.
impl ServerHook for RootRoute {
    /// Creates a new `RootRoute` instance.
    ///
    /// # Arguments
    /// - `&mut Stream` - The connection stream (unused).
    /// - `&mut Context` - The request context (unused).
    ///
    /// # Returns
    /// - `RootRoute` - A new instance with no internal state.
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    /// Handles requests for `/` and `/index.html` by serving the
    /// in-memory development HTML. Returns 500 if the global app state
    /// has not been initialized.
    ///
    /// # Arguments
    /// - `self` - The consumed route instance.
    /// - `&mut Stream` - The connection stream (unused).
    /// - `&mut Context` - The request context used to write the response.
    ///
    /// # Returns
    /// - `Status` - The hook processing result.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let state: Arc<AppState> = match get_global_state() {
            Some(state) => state,
            None => {
                ctx.get_mut_response().set_status_code(500);
                return Status::Continue;
            }
        };
        let html: String = state.get_html_content().read().await.clone();
        ctx.get_mut_response()
            .set_body(&html)
            .set_header(CONTENT_TYPE, TEXT_HTML);
        Status::Continue
    }
}

/// Implements `ServerHook` for `IndexRoute` to serve the development HTML and static assets.
///
/// When the request targets `index.html`, returns the in-memory HTML
/// that has the live-reload script injected. For all other files the
/// handler reads the content from disk with path-traversal protection.
impl ServerHook for IndexRoute {
    /// Creates a new `IndexRoute` instance.
    ///
    /// # Arguments
    /// - `&mut Stream` - The connection stream (unused).
    /// - `&mut Context` - The request context (unused).
    ///
    /// # Returns
    /// - `IndexRoute` - A new instance with no internal state.
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    /// Handles requests for the index page and static assets with path-traversal protection.
    ///
    /// - Empty path or `index.html` - serves the in-memory HTML with live-reload script.
    /// - Other paths: reads the file from disk, validates the canonical path is within
    ///   the www directory, and sets the appropriate `Content-Type` header.
    ///
    /// # Arguments
    /// - `self` - The consumed route instance.
    /// - `&mut Stream` - The connection stream (unused).
    /// - `&mut Context` - The request context used to read route params and write the response.
    ///
    /// # Returns
    /// - `Status` - The hook processing result.
    ///
    /// # Panics
    ///
    /// Does not panic; all error cases set an appropriate HTTP status code.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let path_opt: Option<String> = ctx.try_get_route_param("path");
        let path: String = path_opt.unwrap_or_default();
        if path.contains("..") || path.starts_with('/') || path.starts_with('\\') {
            ctx.get_mut_response().set_status_code(403);
            return Status::Continue;
        }
        let state: Arc<AppState> = match get_global_state() {
            Some(state) => state,
            None => {
                ctx.get_mut_response().set_status_code(500);
                return Status::Continue;
            }
        };
        if path.is_empty() || path == INDEX_HTML_FILE_NAME {
            let html: String = state.get_html_content().read().await.clone();
            ctx.get_mut_response()
                .set_body(&html)
                .set_header(CONTENT_TYPE, TEXT_HTML);
            return Status::Continue;
        }
        let serving_root: PathBuf = resolve_serving_root(state.get_args()).await;
        let file_path: PathBuf = match resolve_file_in_base(&serving_root, &path).await {
            Some(resolved) => resolved,
            None => {
                ctx.get_mut_response().set_status_code(404);
                return Status::Continue;
            }
        };
        match read(&file_path).await {
            Ok(content) => {
                let extension: String = FileExtension::get_extension_name(&path);
                let content_type: &'static str =
                    FileExtension::parse(&extension).get_content_type();
                ctx.get_mut_response()
                    .set_body(&content)
                    .set_header(CONTENT_TYPE, content_type);
            }
            Err(_) => {
                ctx.get_mut_response().set_status_code(404);
            }
        }
        Status::Continue
    }
}

/// Implements `ServerHook` for `ReloadRoute` to provide long-polling reload notifications.
///
/// Uses long-polling: holds the connection open until a reload event
/// is broadcast, then returns a single JSON response so the client can
/// distinguish between a successful rebuild and an error.
impl ServerHook for ReloadRoute {
    /// Creates a new `ReloadRoute` instance.
    ///
    /// # Arguments
    /// - `&mut Stream` - The connection stream (unused).
    /// - `&mut Context` - The request context (unused).
    ///
    /// # Returns
    /// - `ReloadRoute` - A new instance with no internal state.
    async fn new(_: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    /// Waits for a reload event and returns it as a JSON response.
    ///
    /// Subscribes to the broadcast channel and blocks until a `ReloadEvent`
    /// is received, then serializes it as the response body.
    ///
    /// # Arguments
    /// - `self` - The consumed route instance.
    /// - `&mut Stream` - The connection stream (unused).
    /// - `&mut Context` - The request context used to write the response.
    ///
    /// # Returns
    /// - `Status` - The hook processing result.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let state: Arc<AppState> = match get_global_state() {
            Some(state) => state,
            None => {
                ctx.get_mut_response()
                    .set_status_code(500)
                    .set_body(ERROR_SERVER_NOT_READY);
                return Status::Continue;
            }
        };
        let mut rx: broadcast::Receiver<ReloadEvent> = state.get_reload_tx().subscribe();
        let event: ReloadEvent = match rx.recv().await {
            Ok(event) => event,
            Err(_) => {
                ctx.get_mut_response().set_status_code(503);
                return Status::Continue;
            }
        };
        let body: String = match serde_json::to_string(&event) {
            Ok(json) => json,
            Err(_) => {
                ctx.get_mut_response().set_status_code(500);
                return Status::Continue;
            }
        };
        ctx.get_mut_response().set_body(&body);
        Status::Continue
    }
}
