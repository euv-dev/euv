use super::*;

/// Rejects malformed envelopes and WebDriver errors rather than reporting success.
///
/// # Arguments
///
/// - `Value` - JSON received from the driver.
///
/// # Returns
///
/// - `AutomationResult<Value>` - The W3C value payload or the protocol error.
pub(crate) fn response_value(document: Value) -> AutomationResult<Value> {
    let value: &Value = document
        .get(JSON_VALUE)
        .ok_or_else(|| format!("WebDriver response lacks value: {document}"))?;
    if let Some(error) = value.get(JSON_ERROR) {
        return Err(format!(
            "WebDriver {error}: {}",
            value.get(JSON_MESSAGE).unwrap_or(&Value::Null)
        ));
    }
    Ok(value.clone())
}

/// Sends one JSON WebDriver request with bounded I/O and no shell interpolation.
///
/// # Arguments
///
/// - `&str` - Driver endpoint, relative command path and HTTP method.
/// - `Option<&Value>` - Optional JSON request body.
///
/// # Returns
///
/// - `AutomationResult<Value>` - Validated protocol payload.
pub(crate) fn webdriver_request(
    endpoint: &str,
    path: &str,
    method: &str,
    payload: Option<&Value>,
) -> AutomationResult<Value> {
    let request_timeout: &str = if method == HTTP_POST && path == SESSION_PATH {
        SESSION_REQUEST_TIMEOUT
    } else {
        REQUEST_TIMEOUT
    };
    let mut command: Command = Command::new(CURL_PROGRAM);
    command.args([
        CURL_SILENT,
        CURL_SHOW_ERROR,
        CURL_FAIL_BODY,
        CURL_NO_PROXY,
        PROXY_BYPASS,
        CURL_MAX_TIME,
        request_timeout,
        CURL_REQUEST,
        method,
        CURL_HEADER,
        JSON_CONTENT_TYPE,
    ]);
    command.arg(format!("{endpoint}{path}"));
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let encoded: Option<Vec<u8>> = payload
        .map(to_vec)
        .transpose()
        .map_err(|error: Error| error.to_string())?;
    if encoded.is_some() {
        command
            .args([CURL_DATA, STDIN_SOURCE])
            .stdin(Stdio::piped());
    }
    let mut process: Child = command
        .spawn()
        .map_err(|error: io::Error| format!("cannot start curl: {error}"))?;
    if let Some(bytes) = encoded {
        let Child { stdin, .. }: &mut Child = &mut process;
        let mut input: ChildStdin = stdin.take().ok_or_else(|| ERROR_CURL_STDIN.to_string())?;
        input
            .write_all(&bytes)
            .map_err(|error: io::Error| format!("cannot write WebDriver request: {error}"))?;
    }
    let Output {
        status,
        stdout,
        stderr,
    }: Output = process
        .wait_with_output()
        .map_err(|error: io::Error| error.to_string())?;
    if !status.success() {
        return Err(format!(
            "WebDriver HTTP {method} {path} failed: {} {}",
            String::from_utf8_lossy(&stderr),
            String::from_utf8_lossy(&stdout)
        ));
    }
    let document: Value =
        from_slice(&stdout).map_err(|error: Error| format!("invalid WebDriver JSON: {error}"))?;
    response_value(document)
}
