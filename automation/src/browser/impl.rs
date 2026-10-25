use super::*;

/// Browser operations and state-based assertions over a real WebDriver session.
impl Browser {
    /// Opens an isolated headless Chrome session.
    ///
    /// # Arguments
    ///
    /// - `&str` - Loopback driver endpoint.
    /// - `Option<&Path>` - Optional Chrome executable override.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<Self>` - Browser with a driver-issued session identifier.
    pub fn connect(endpoint: &str, chrome: Option<&Path>) -> AutomationResult<Self> {
        let mut options: Value =
            json!({(JSON_ARGS): [CHROME_HEADLESS, CHROME_SHARED_MEMORY, CHROME_WINDOW]});
        if let Some(binary) = chrome {
            options[JSON_BINARY] = json!(binary.to_string_lossy());
        }
        let capabilities: Value = json!({(JSON_CAPABILITIES): {(JSON_ALWAYS_MATCH): {(JSON_BROWSER_NAME): CHROME_NAME, (JSON_INSECURE_CERTS): false, (JSON_CHROME_OPTIONS): options, (JSON_LOGGING): {(BROWSER_LOG): LOG_LEVEL}}}});
        let value: Value =
            webdriver_request(endpoint, SESSION_PATH, HTTP_POST, Some(&capabilities))?;
        let session: String = value
            .get(JSON_SESSION_ID)
            .and_then(Value::as_str)
            .filter(|value: &&str| !value.is_empty())
            .ok_or_else(|| format!("driver returned no session identifier: {value}"))?
            .to_string();
        let metadata: Value = value
            .get(JSON_CAPABILITIES)
            .cloned()
            .ok_or_else(|| format!("driver returned no browser capabilities: {value}"))?;
        Ok(Self::new(
            endpoint.to_string(),
            session,
            Duration::from_secs(5),
            metadata,
        ))
    }

    /// Executes one session-scoped command.
    ///
    /// # Arguments
    ///
    /// - `&str` - HTTP method and command suffix.
    /// - `Option<&Value>` - Optional command body.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<Value>` - Validated command result.
    pub(crate) fn command(
        &self,
        method: &str,
        suffix: &str,
        payload: Option<&Value>,
    ) -> AutomationResult<Value> {
        let path: String = if suffix.is_empty() {
            format!("/session/{}", self.get_session())
        } else {
            format!("/session/{}/{suffix}", self.get_session())
        };
        webdriver_request(self.get_endpoint(), &path, method, payload)
    }

    /// Navigates to an actual page and waits for its normal load event.
    ///
    /// # Arguments
    ///
    /// - `&str` - URL to navigate to.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<()>` - Navigation result.
    pub fn navigate(&self, url: &str) -> AutomationResult<()> {
        self.command(HTTP_POST, URL_COMMAND, Some(&json!({(JSON_URL): url})))
            .map(|_: Value| ())
    }

    /// Locates a displayed-page element by stable CSS selector.
    ///
    /// # Arguments
    ///
    /// - `&str` - CSS selector.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<String>` - Driver-issued element identifier.
    pub fn find(&self, selector: &str) -> AutomationResult<String> {
        let value: Value = self.command(
            HTTP_POST,
            ELEMENT_COMMAND,
            Some(&json!({(JSON_USING): CSS_SELECTOR, (JSON_VALUE): selector})),
        )?;
        value
            .get(ELEMENT_KEY)
            .and_then(Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| format!("no element identifier for {selector}: {value}"))
    }

    /// Clicks through WebDriver's real element interaction command.
    ///
    /// # Arguments
    ///
    /// - `&str` - Target selector.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<()>` - Click result.
    pub fn click(&self, selector: &str) -> AutomationResult<()> {
        let element: String = self.find(selector)?;
        self.command(
            HTTP_POST,
            &format!("element/{element}/click"),
            Some(&json!({})),
        )
        .map(|_: Value| ())
    }

    /// Clears and types into a real editable control.
    ///
    /// # Arguments
    ///
    /// - `&str` - Target selector and text to type.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<()>` - Input result.
    pub fn fill(&self, selector: &str, text: &str) -> AutomationResult<()> {
        let element: String = self.find(selector)?;
        self.command(
            HTTP_POST,
            &format!("element/{element}/clear"),
            Some(&json!({})),
        )?;
        self.press(selector, text)
    }

    /// Sends text or a WebDriver key code to a real focused element.
    ///
    /// # Arguments
    ///
    /// - `&str` - Target selector and key sequence.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<()>` - Keyboard result.
    pub fn press(&self, selector: &str, keys: &str) -> AutomationResult<()> {
        let element: String = self.find(selector)?;
        self.command(
            HTTP_POST,
            &format!("element/{element}/value"),
            Some(&json!({(JSON_TEXT): keys})),
        )
        .map(|_: Value| ())
    }

    /// Reads rendered element text through the browser driver.
    ///
    /// # Arguments
    ///
    /// - `&str` - Element selector.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<String>` - Actual rendered text.
    pub fn text(&self, selector: &str) -> AutomationResult<String> {
        let element: String = self.find(selector)?;
        self.command(HTTP_GET, &format!("element/{element}/text"), None)?
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| format!("non-string element text for {selector}"))
    }

    /// Waits for exact rendered text instead of using a fixed test delay.
    ///
    /// # Arguments
    ///
    /// - `&str` - Element selector and expected text.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<()>` - Matching text or a timeout with the observed value.
    pub fn assert_text(&self, selector: &str, expected: &str) -> AutomationResult<()> {
        let started: Instant = Instant::now();
        loop {
            let observed: AutomationResult<String> = self.text(selector);
            if observed.as_deref() == Ok(expected) {
                return Ok(());
            }
            if started.elapsed() >= self.get_timeout() {
                return Err(format!(
                    "text assertion failed for {selector}: expected {expected:?}, observed {observed:?}"
                ));
            }
            sleep(Duration::from_millis(25));
        }
    }

    /// Runs browser-side observation code for assertions and diagnostics.
    ///
    /// # Arguments
    ///
    /// - `&str` - JavaScript observation expression.
    /// - `Vec<Value>` - Explicit script arguments.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<Value>` - Browser-produced value.
    pub fn evaluate(&self, script: &str, arguments: Vec<Value>) -> AutomationResult<Value> {
        self.command(
            HTTP_POST,
            EXECUTE_COMMAND,
            Some(&json!({(JSON_SCRIPT): script, (JSON_ARGS): arguments})),
        )
    }

    /// Scrolls the real element using browser wheel input.
    ///
    /// # Arguments
    ///
    /// - `&str` - Scroll-container selector.
    /// - `i32` - Vertical wheel delta in CSS pixels.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<()>` - Wheel-action result.
    pub fn scroll(&self, selector: &str, delta: i32) -> AutomationResult<()> {
        let element: String = self.find(selector)?;
        let origin: Value = json!({(ELEMENT_KEY): element});
        self.evaluate(SCROLL_INTO_VIEW_SCRIPT, vec![origin.clone()])?;
        self.command(HTTP_POST, ACTIONS_COMMAND, Some(&json!({(JSON_ACTIONS): [{(JSON_TYPE): WHEEL_SOURCE, (JSON_ID): WHEEL_SOURCE, (JSON_ACTIONS): [{(JSON_TYPE): SCROLL_ACTION, (JSON_X): 0, (JSON_Y): 0, (JSON_DELTA_X): 0, (JSON_DELTA_Y): delta, (JSON_DURATION): 100, (JSON_ORIGIN): origin}]}]}))).map(|_: Value| ())
    }

    /// Changes the real browser viewport for responsive-layout checks.
    ///
    /// # Arguments
    ///
    /// - `&str` - Browser observation expression.
    /// - `Vec<Value>` - Explicit expression arguments.
    /// - `Value` - Expected browser-produced value.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<()>` - Matching observation or a timeout with evidence.
    pub fn assert_script(
        &self,
        script: &str,
        arguments: Vec<Value>,
        expected: Value,
    ) -> AutomationResult<()> {
        let started: Instant = Instant::now();
        loop {
            let observed: AutomationResult<Value> = self.evaluate(script, arguments.clone());
            if observed.as_ref() == Ok(&expected) {
                return Ok(());
            }
            if started.elapsed() >= self.get_timeout() {
                return Err(format!(
                    "browser-state assertion failed: expected {expected}, observed {observed:?}"
                ));
            }
            sleep(Duration::from_millis(25));
        }
    }

    /// Changes the real browser viewport for responsive-layout checks.
    ///
    /// # Arguments
    ///
    /// - `u32` - Requested width and height.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<()>` - Window-rectangle result.
    pub fn viewport(&self, width: u32, height: u32) -> AutomationResult<()> {
        self.command(
            HTTP_POST,
            RECT_COMMAND,
            Some(&json!({(JSON_WIDTH): width, (JSON_HEIGHT): height})),
        )?;
        self.command(HTTP_POST, CDP_COMMAND, Some(&json!({(JSON_COMMAND): VIEWPORT_METRICS, (JSON_PARAMETERS): {(JSON_WIDTH): width, (JSON_HEIGHT): height, (JSON_SCALE): 1, (JSON_MOBILE): false}}))).map(|_: Value| ())
    }

    /// Captures an actual browser screenshot and DOM diagnostic snapshot.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Artifact directory.
    /// - `&str` - File-name prefix for the current case.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<()>` - Diagnostic write result.
    pub fn capture(&self, directory: &Path, name: &str) -> AutomationResult<()> {
        fs::create_dir_all(directory).map_err(|error: io::Error| error.to_string())?;
        let screenshot: Value = self.command(HTTP_GET, SCREENSHOT_COMMAND, None)?;
        let encoded: &str = screenshot
            .as_str()
            .ok_or_else(|| ERROR_SCREENSHOT_TYPE.to_string())?;
        let mut process: Child = Command::new(PYTHON_PROGRAM)
            .args([PYTHON_CODE_FLAG, DECODE_SCREENSHOT_SCRIPT])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error: io::Error| error.to_string())?;
        {
            let Child { stdin, .. }: &mut Child = &mut process;
            let mut input: ChildStdin = stdin
                .take()
                .ok_or_else(|| ERROR_DECODER_STDIN.to_string())?;
            input
                .write_all(encoded.as_bytes())
                .map_err(|error: io::Error| error.to_string())?;
        }
        let Output {
            status,
            stdout,
            stderr,
        }: Output = process
            .wait_with_output()
            .map_err(|error: io::Error| error.to_string())?;
        if !status.success() || !stdout.starts_with(PNG_SIGNATURE) {
            return Err(format!(
                "invalid screenshot bytes: {}",
                String::from_utf8_lossy(&stderr)
            ));
        }
        fs::write(directory.join(format!("{name}.png")), stdout)
            .map_err(|error: io::Error| error.to_string())?;
        let diagnostic: Value = self.evaluate(DIAGNOSTIC_SCRIPT, Vec::new())?;
        fs::write(
            directory.join(format!("{name}.json")),
            to_string_pretty(&diagnostic).map_err(|error: Error| error.to_string())?,
        )
        .map_err(|error: io::Error| error.to_string())
    }

    /// Deletes the owned browser session and prevents duplicate teardown.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<()>` - Confirmed driver teardown result.
    pub fn close(mut self) -> AutomationResult<()> {
        self.command(HTTP_DELETE, EMPTY_COMMAND, None)?;
        self.set_session(String::new());
        Ok(())
    }
}

/// Attempts session cleanup on every early-return path.
impl Drop for Browser {
    /// Deletes only the session created by this browser handle.
    fn drop(&mut self) {
        if !self.get_session().is_empty()
            && let Err(error) = self.command(HTTP_DELETE, EMPTY_COMMAND, None)
        {
            eprintln!("browser cleanup failed: {error}");
        }
    }
}
