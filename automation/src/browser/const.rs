/// Reads a browser command result.
pub(crate) const HTTP_GET: &str = "GET";

/// Streams JSON request bytes through standard input.
pub(crate) const CURL_DATA: &str = "--data-binary";

/// Executes a browser state-changing command.
pub(crate) const HTTP_POST: &str = "POST";

/// W3C protocol-error code member.
pub(crate) const JSON_ERROR: &str = "error";

/// W3C response envelope payload member.
pub(crate) const JSON_VALUE: &str = "value";

/// Sets the JSON request content type.
pub(crate) const CURL_HEADER: &str = "--header";

/// Suppresses transfer progress without hiding failures.
pub(crate) const CURL_SILENT: &str = "--silent";

/// W3C remote-element reference key.
pub(crate) const ELEMENT_KEY: &str = "element-6066-11e4-a52e-4f735466cecf";

/// Deletes the owned browser session.
pub(crate) const HTTP_DELETE: &str = "DELETE";

/// Chrome capability selecting a browser executable.
pub(crate) const JSON_BINARY: &str = "binary";

/// Navigates or reads the active URL.
pub(crate) const URL_COMMAND: &str = "url";

/// HTTP transport executable; invoked without a shell.
pub(crate) const CURL_PROGRAM: &str = "curl";

/// Sets the explicit HTTP method.
pub(crate) const CURL_REQUEST: &str = "--request";

/// Human-readable protocol-error member.
pub(crate) const JSON_MESSAGE: &str = "message";

/// Disables proxies for the explicit driver request.
pub(crate) const PROXY_BYPASS: &str = "*";

/// Sets the actual browser viewport rectangle.
pub(crate) const RECT_COMMAND: &str = "window/rect";

/// ChromeDriver extension used only for an actual emulated viewport.
pub(crate) const CDP_COMMAND: &str = "goog/cdp/execute";

/// Creates a browser-driver session.
pub(crate) const SESSION_PATH: &str = "/session";

/// Allows bounded cold browser startup without extending ordinary command deadlines.
pub(crate) const SESSION_REQUEST_TIMEOUT: &str = "60";

/// Makes curl read request bytes from standard input.
pub(crate) const STDIN_SOURCE: &str = "@-";

/// Bounds each HTTP request duration.
pub(crate) const CURL_MAX_TIME: &str = "--max-time";

/// Bypasses proxy configuration for the loopback driver.
pub(crate) const CURL_NO_PROXY: &str = "--noproxy";

/// Addresses the session itself for teardown.
pub(crate) const EMPTY_COMMAND: &str = "";

/// Treats non-success HTTP statuses as failures while retaining JSON.
pub(crate) const CURL_FAIL_BODY: &str = "--fail-with-body";

/// Binary-safe standard-library screenshot decoder.
pub(crate) const PYTHON_PROGRAM: &str = "python3";

/// Performs native browser input actions.
pub(crate) const ACTIONS_COMMAND: &str = "actions";

/// Retains transport diagnostics.
pub(crate) const CURL_SHOW_ERROR: &str = "--show-error";

/// Finds one page element.
pub(crate) const ELEMENT_COMMAND: &str = "element";

/// Runs synchronous browser observation code.
pub(crate) const EXECUTE_COMMAND: &str = "execute/sync";

/// Session identifier returned by the driver.
pub(crate) const JSON_SESSION_ID: &str = "sessionId";

/// Driver-reported browser capabilities and owned process identity.
pub(crate) const JSON_CAPABILITIES: &str = "capabilities";

/// Maximum seconds for one driver command.
pub(crate) const REQUEST_TIMEOUT: &str = "10";

/// Executes the fixed screenshot-decoder program.
pub(crate) const PYTHON_CODE_FLAG: &str = "-c";

/// Reads actual page state for failure diagnostics.
pub(crate) const DIAGNOSTIC_SCRIPT: &str = "return {url:location.href,title:document.title,body:document.body.innerText,html:document.documentElement.outerHTML,active:document.activeElement&&document.activeElement.id,visibility:document.visibilityState};";

/// JSON content-type header used by WebDriver.
pub(crate) const JSON_CONTENT_TYPE: &str = "Content-Type: application/json; charset=utf-8";

/// Captures the current browser viewport.
pub(crate) const SCREENSHOT_COMMAND: &str = "screenshot";

/// Reveals the wheel target before native scrolling.
pub(crate) const SCROLL_INTO_VIEW_SCRIPT: &str =
    "arguments[0].scrollIntoView({block:'center',inline:'nearest'}); return true;";

/// Decodes driver PNG bytes without text conversion or shell interpolation.
pub(crate) const DECODE_SCREENSHOT_SCRIPT: &str = "import base64,sys; sys.stdout.buffer.write(base64.b64decode(sys.stdin.buffer.read(),validate=True))";

/// Magic bytes required for every captured PNG artifact.
pub(crate) const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";

/// Failure when an HTTP request pipe was not created.
pub(crate) const ERROR_CURL_STDIN: &str = "curl stdin unavailable";

/// Failure when the driver returns a non-text screenshot payload.
pub(crate) const ERROR_SCREENSHOT_TYPE: &str = "driver screenshot was not base64 text";

/// Failure when a screenshot decoder pipe was not created.
pub(crate) const ERROR_DECODER_STDIN: &str = "decoder stdin unavailable";

/// Chrome capability arguments member.
pub(crate) const JSON_ARGS: &str = "args";

/// Headless mode keeps test input independent of the user's desktop.
pub(crate) const CHROME_HEADLESS: &str = "--headless=new";

/// Avoids constrained shared-memory mounts on CI hosts.
pub(crate) const CHROME_SHARED_MEMORY: &str = "--disable-dev-shm-usage";

/// Initial real browser window dimensions.
pub(crate) const CHROME_WINDOW: &str = "--window-size=1280,900";

/// Required browser capability set.
pub(crate) const JSON_ALWAYS_MATCH: &str = "alwaysMatch";

/// Standard browser-name capability.
pub(crate) const JSON_BROWSER_NAME: &str = "browserName";

/// Chrome browser name used by the suite.
pub(crate) const CHROME_NAME: &str = "chrome";

/// Capability controlling invalid-certificate acceptance.
pub(crate) const JSON_INSECURE_CERTS: &str = "acceptInsecureCerts";

/// Vendor capability containing Chrome launch options.
pub(crate) const JSON_CHROME_OPTIONS: &str = "goog:chromeOptions";

/// Vendor capability enabling diagnostic browser logs.
pub(crate) const JSON_LOGGING: &str = "goog:loggingPrefs";

/// Browser console-log category.
pub(crate) const BROWSER_LOG: &str = "browser";

/// Requests all browser log severities.
pub(crate) const LOG_LEVEL: &str = "ALL";

/// Navigation payload member.
pub(crate) const JSON_URL: &str = "url";

/// Element-locator strategy member.
pub(crate) const JSON_USING: &str = "using";

/// Stable CSS-selector lookup strategy.
pub(crate) const CSS_SELECTOR: &str = "css selector";

/// Element key-input payload member.
pub(crate) const JSON_TEXT: &str = "text";

/// Observation script payload member.
pub(crate) const JSON_SCRIPT: &str = "script";

/// Native input action-sequence member.
pub(crate) const JSON_ACTIONS: &str = "actions";

/// Input source and action discriminator.
pub(crate) const JSON_TYPE: &str = "type";

/// Native wheel source and identifier.
pub(crate) const WHEEL_SOURCE: &str = "wheel";

/// Input-source identifier member.
pub(crate) const JSON_ID: &str = "id";

/// Native scroll action discriminator.
pub(crate) const SCROLL_ACTION: &str = "scroll";

/// Horizontal action coordinate.
pub(crate) const JSON_X: &str = "x";

/// Vertical action coordinate.
pub(crate) const JSON_Y: &str = "y";

/// Horizontal wheel displacement member.
pub(crate) const JSON_DELTA_X: &str = "deltaX";

/// Vertical wheel displacement member.
pub(crate) const JSON_DELTA_Y: &str = "deltaY";

/// Native-action duration member.
pub(crate) const JSON_DURATION: &str = "duration";

/// Action origin member containing a real element reference.
pub(crate) const JSON_ORIGIN: &str = "origin";

/// Browser viewport width member.
pub(crate) const JSON_WIDTH: &str = "width";

/// Browser viewport height member.
pub(crate) const JSON_HEIGHT: &str = "height";

/// Chrome protocol command member.
pub(crate) const JSON_COMMAND: &str = "cmd";

/// Chrome protocol parameter object member.
pub(crate) const JSON_PARAMETERS: &str = "params";

/// Sets actual emulated viewport metrics below Chrome's physical window minimum.
pub(crate) const VIEWPORT_METRICS: &str = "Emulation.setDeviceMetricsOverride";

/// Actual browser device-scale-factor member.
pub(crate) const JSON_SCALE: &str = "deviceScaleFactor";

/// Actual browser mobile-emulation member.
pub(crate) const JSON_MOBILE: &str = "mobile";
