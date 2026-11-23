/// Native UI automation command synopsis.
pub(crate) const USAGE: &str = "euv-automation [--driver PATH] [--bindgen PATH] [--cargo PATH] [--chrome PATH] [--output DIR] [--url URL] [--skip-build] [--probe-failure]";

/// Numeric signal text after one observed event.
pub(crate) const ONE_TEXT: &str = "1";

/// Counter text expected after two real clicks.
pub(crate) const TWO_TEXT: &str = "2";

/// Prints available runner options without starting resources.
pub(crate) const HELP_FLAG: &str = "--help";

/// Retains browser startup diagnostics when session creation fails in CI.
pub(crate) const DRIVER_VERBOSE: &str = "--verbose";

/// Failure when the internal server mode receives an invalid argument shape.
pub(crate) const ERROR_SERVER_ARGUMENTS: &str = "invalid internal server arguments";

/// Rendered true state of a reactive boolean.
pub(crate) const TRUE_TEXT: &str = "true";

/// Initial numeric signal text expected from the fixture.
pub(crate) const ZERO_TEXT: &str = "0";

/// Native text-entry case identifier.
pub(crate) const CASE_INPUT: &str = "input";

/// Modal open-and-close case identifier.
pub(crate) const CASE_MODAL: &str = "modal";

/// Printed result of a failed browser assertion case.
pub(crate) const FAIL_LABEL: &str = "FAIL";

/// Rendered false state of a reactive boolean.
pub(crate) const FALSE_TEXT: &str = "false";

/// Driver readiness payload member.
pub(crate) const JSON_READY: &str = "ready";

/// Printed result of a successful browser assertion case.
pub(crate) const PASS_LABEL: &str = "PASS";

/// Native wheel and reactive-scroll case identifier.
pub(crate) const CASE_SCROLL: &str = "scroll";

/// Identity of the task-owned static-file process.
pub(crate) const SERVER_NAME: &str = "fixture-server";

/// Reads actual driver readiness.
pub(crate) const STATUS_PATH: &str = "/status";

/// Intentionally incorrect value used only by the negative-control mode.
pub(crate) const WRONG_COUNT: &str = "999";

/// Trusted click and reactive counter case identifier.
pub(crate) const CASE_COUNTER: &str = "counter";

/// Text sent through the browser input command.
pub(crate) const INPUT_SAMPLE: &str = "Native input value";

/// Two-way checkbox binding case identifier.
pub(crate) const CASE_CHECKBOX: &str = "checkbox";

/// Native keyboard and focus case identifier.
pub(crate) const CASE_KEYBOARD: &str = "keyboard";

/// Deliberately failing assertion case identifier.
pub(crate) const CASE_NEGATIVE: &str = "negative-control";

/// Reads actual uncaught browser errors recorded by the fixture bootstrap.
pub(crate) const ERRORS_SCRIPT: &str = "return window.__EUV_AUTOMATION_ERRORS__ || [];";

/// WebDriver key value representing Tab.
pub(crate) const WEBDRIVER_TAB: &str = "";

/// Observes the real checkbox DOM property.
pub(crate) const CHECKED_SCRIPT: &str = "return document.querySelector(arguments[0]).checked;";

/// Observes actual rendered dimensions and visibility.
pub(crate) const VISIBLE_SCRIPT: &str = "const el=document.querySelector(arguments[0]); if(!el)return false; const r=el.getBoundingClientRect(),s=getComputedStyle(el); return r.width>0&&r.height>0&&s.visibility!=='hidden'&&s.display!=='none';";

/// Narrow viewport usability and geometry case identifier.
pub(crate) const CASE_RESPONSIVE: &str = "responsive";

/// Requires both real browser scroll and euv event propagation.
pub(crate) const SCROLLED_SCRIPT: &str = "return document.querySelector(arguments[0]).scrollTop>0 && Number(document.querySelector(arguments[1]).textContent)>0;";

/// WebDriver key value representing Enter.
pub(crate) const WEBDRIVER_ENTER: &str = "";

/// Observes actual native keyboard focus.
pub(crate) const ACTIVE_ID_SCRIPT: &str =
    "return document.activeElement ? document.activeElement.id : null;";

/// Prevents remote access to the task-owned browser driver.
pub(crate) const DRIVER_LOCAL_ONLY: &str = "--allowed-ips=127.0.0.1";

/// Checks actual narrow viewport and in-bounds input geometry.
pub(crate) const RESPONSIVE_SCRIPT: &str = "const r=document.querySelector(arguments[0]).getBoundingClientRect(); return innerWidth<=420 && r.left>=0 && r.right<=innerWidth;";

/// Directory containing actual PNG and DOM captures.
pub(crate) const SCREENSHOT_DIRECTORY: &str = "screenshots";

/// Bounds each fixture-readiness HTTP request.
pub(crate) const SERVER_REQUEST_TIMEOUT: &str = "2";

/// Caret-gap geometry case identifier.
pub(crate) const CASE_CARET: &str = "caret-gap";

/// Observes whether the real distance between a trigger's label and its caret
/// reaches the shared gap token. The measurement reads the rendered boxes, so
/// it proves the gap in the browser rather than in the stylesheet.
pub(crate) const CARET_GAP_SCRIPT: &str = "const l=document.querySelector(arguments[0]).getBoundingClientRect(),c=document.querySelector(arguments[1]).getBoundingClientRect();return c.left-l.right>=arguments[2];";

/// The minimum label-to-caret distance the shared gap token promises.
pub(crate) const CARET_GAP_MINIMUM: f64 = 8.0;
