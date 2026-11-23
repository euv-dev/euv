use super::*;

/// Identifies and executes real browser scenarios.
impl Scenario {
    /// Returns a stable artifact and report identifier.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Case identifier.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Counter => CASE_COUNTER,
            Self::Input => CASE_INPUT,
            Self::Keyboard => CASE_KEYBOARD,
            Self::Checkbox => CASE_CHECKBOX,
            Self::Modal => CASE_MODAL,
            Self::Scroll => CASE_SCROLL,
            Self::Responsive => CASE_RESPONSIVE,
            Self::CaretGap => CASE_CARET,
            Self::Negative => CASE_NEGATIVE,
        }
    }

    /// Runs the named case against the mounted framework fixture.
    ///
    /// # Arguments
    ///
    /// - `&Browser` - Actual browser session.
    /// - `&str` - Served fixture URL.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<()>` - Assertion outcome.
    pub(crate) fn execute(self, browser: &Browser, url: &str) -> AutomationResult<()> {
        browser.viewport(1280, 900)?;
        if self == Self::Responsive {
            browser.viewport(390, 844)?;
        }
        browser.navigate(url)?;
        browser.assert_text(READY_SELECTOR, READY_TEXT)?;
        match self {
            Self::Counter => {
                browser.assert_text(COUNT_SELECTOR, ZERO_TEXT)?;
                browser.click(INCREMENT_SELECTOR)?;
                browser.click(INCREMENT_SELECTOR)?;
                browser.assert_text(COUNT_SELECTOR, TWO_TEXT)?;
                browser.assert_text(TRUSTED_SELECTOR, TRUE_TEXT)?;
            }
            Self::Input => {
                browser.fill(INPUT_SELECTOR, INPUT_SAMPLE)?;
                browser.assert_text(TEXT_SELECTOR, INPUT_SAMPLE)?;
                browser.assert_text(TRUSTED_SELECTOR, TRUE_TEXT)?;
            }
            Self::Keyboard => {
                browser.press(INPUT_SELECTOR, WEBDRIVER_ENTER)?;
                browser.assert_text(KEYS_SELECTOR, ONE_TEXT)?;
                browser.press(INPUT_SELECTOR, WEBDRIVER_TAB)?;
                browser.assert_script(ACTIVE_ID_SCRIPT, Vec::new(), json!(CHECKBOX_ID))?;
            }
            Self::Checkbox => {
                browser.assert_text(CHECKED_SELECTOR, FALSE_TEXT)?;
                browser.click(CHECKBOX_SELECTOR)?;
                browser.assert_text(CHECKED_SELECTOR, TRUE_TEXT)?;
                browser.assert_script(
                    CHECKED_SCRIPT,
                    vec![json!(CHECKBOX_SELECTOR)],
                    json!(true),
                )?;
                browser.click(CHECKBOX_SELECTOR)?;
                browser.assert_text(CHECKED_SELECTOR, FALSE_TEXT)?;
            }
            Self::Modal => {
                browser.assert_script(
                    VISIBLE_SCRIPT,
                    vec![json!(DIALOG_CONTENT_SELECTOR)],
                    json!(false),
                )?;
                browser.click(OPEN_SELECTOR)?;
                browser.assert_text(DIALOG_CONTENT_SELECTOR, DIALOG_TEXT)?;
                browser.assert_script(
                    VISIBLE_SCRIPT,
                    vec![json!(DIALOG_CONTENT_SELECTOR)],
                    json!(true),
                )?;
                browser.click(DIALOG_CLOSE_SELECTOR)?;
                browser.assert_script(
                    VISIBLE_SCRIPT,
                    vec![json!(DIALOG_CONTENT_SELECTOR)],
                    json!(false),
                )?;
            }
            Self::Scroll => {
                browser.assert_text(SCROLL_SELECTOR, ZERO_TEXT)?;
                browser.scroll(SCROLLER_SELECTOR, 240)?;
                browser.assert_script(
                    SCROLLED_SCRIPT,
                    vec![json!(SCROLLER_SELECTOR), json!(SCROLL_SELECTOR)],
                    json!(true),
                )?;
            }
            Self::Responsive => {
                browser.click(INCREMENT_SELECTOR)?;
                browser.assert_text(COUNT_SELECTOR, ONE_TEXT)?;
                browser.assert_script(
                    RESPONSIVE_SCRIPT,
                    vec![json!(INPUT_SELECTOR)],
                    json!(true),
                )?;
            }
            Self::CaretGap => {
                browser.assert_script(
                    CARET_GAP_SCRIPT,
                    vec![
                        json!(CARET_LABEL_SELECTOR),
                        json!(CARET_ICON_SELECTOR),
                        json!(CARET_GAP_MINIMUM),
                    ],
                    json!(true),
                )?;
            }
            Self::Negative => browser.assert_text(COUNT_SELECTOR, WRONG_COUNT)?,
        }
        browser.assert_script(ERRORS_SCRIPT, Vec::new(), json!([]))
    }
}
