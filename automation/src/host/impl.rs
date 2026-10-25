use super::*;

/// Bounded teardown of task-owned helpers.
impl OwnedProcess {
    /// Requests stdin-based shutdown, then kills only the owned surviving child.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<()>` - Reaped process or a cleanup error.
    pub(crate) fn stop(&mut self) -> AutomationResult<()> {
        let Child { stdin, .. }: &mut Child = self.get_mut_child();
        drop(stdin.take());
        let started: Instant = Instant::now();
        loop {
            if self
                .get_mut_child()
                .try_wait()
                .map_err(|error: io::Error| error.to_string())?
                .is_some()
            {
                return Ok(());
            }
            if started.elapsed() >= Duration::from_secs(2) {
                break;
            }
            sleep(Duration::from_millis(20));
        }
        self.get_mut_child()
            .kill()
            .map_err(|error: io::Error| format!("cannot stop {}: {error}", self.get_name()))?;
        self.get_mut_child()
            .wait()
            .map_err(|error: io::Error| error.to_string())?;
        Ok(())
    }
}

/// Cleans up helper processes on every early return.
impl Drop for OwnedProcess {
    /// Reaps only the child process owned by this handle.
    fn drop(&mut self) {
        if let Err(error) = self.stop() {
            eprintln!("helper cleanup failed: {error}");
        }
    }
}

/// Implements a direct-send static-file hook without hidden response middleware.
impl ServerHook for FixtureFiles {
    /// Creates a stateless route hook.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Self` - Static-file hook.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Sends the exact built fixture bytes with their browser MIME type.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Connection used for the response.
    /// - `&mut Context` - Parsed route context.
    ///
    /// # Returns
    ///
    /// - `Status` - Stops the pipeline after sending exactly one response.
    async fn handle(self, stream: &mut Stream, context: &mut Context) -> Status {
        let relative: String = context
            .try_get_route_param(ROUTE_PARAMETER)
            .unwrap_or_default();
        let selected: &str = if relative.is_empty() {
            INDEX_FILE
        } else {
            &relative
        };
        let root: Option<&PathBuf> = SERVING_ROOT.get();
        let file: Option<PathBuf> = root
            .and_then(|root: &PathBuf| root.join(selected).canonicalize().ok())
            .filter(|path: &PathBuf| root.is_some_and(|root: &PathBuf| path.starts_with(root)));
        let body: Option<Vec<u8>> = file.as_ref().and_then(|path: &PathBuf| fs::read(path).ok());
        let content_type: &str = if selected.ends_with(WASM_EXTENSION) {
            WASM_MIME
        } else if selected.ends_with(JS_EXTENSION) {
            JS_MIME
        } else {
            HTML_MIME
        };
        let response: &mut Response = context.get_mut_response();
        response.set_status_code(if body.is_some() { 200 } else { 404 });
        response
            .set_header(CONTENT_TYPE_HEADER, content_type)
            .set_header(CACHE_HEADER, CACHE_DISABLED);
        response.set_body(body.unwrap_or_else(|| NOT_FOUND_BODY.as_bytes().to_vec()));
        let bytes: Vec<u8> = response.build();
        if let Err(error) = stream.try_send(&bytes).await {
            eprintln!("fixture send failed: {error}");
        }
        Status::Reject
    }
}
