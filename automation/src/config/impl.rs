use super::*;

/// Defaults to a fresh fixture build and isolated headless browser.
impl Default for Settings {
    /// Constructs native runner defaults without starting external resources.
    fn default() -> Self {
        Self::new(
            Path::new(env!("CARGO_MANIFEST_DIR")).join(RESULTS_DIRECTORY),
            PathBuf::from(DRIVER_PROGRAM),
            PathBuf::from(BINDGEN_PROGRAM),
            PathBuf::from(
                env::var(CARGO_ENV).unwrap_or_else(|_: env::VarError| CARGO_PROGRAM.to_string()),
            ),
        )
    }
}

/// Validates explicit native runner options.
impl Settings {
    /// Parses the command line without silently accepting unknown options.
    ///
    /// # Arguments
    ///
    /// - `I` - Runner arguments excluding the executable name.
    ///
    /// # Returns
    ///
    /// - `AutomationResult<Self>` - Validated settings or the invalid option.
    pub fn parse<I>(arguments: I) -> AutomationResult<Self>
    where
        I: IntoIterator<Item = String>,
    {
        let mut settings: Self = Self::default();
        let mut arguments: I::IntoIter = arguments.into_iter();
        while let Some(flag) = arguments.next() {
            if flag == FLAG_SKIP_BUILD {
                settings.set_skip_build(true);
                continue;
            }
            if flag == FLAG_PROBE_FAILURE {
                settings.set_probe_failure(true);
                continue;
            }
            let value: String = arguments
                .next()
                .ok_or_else(|| format!("missing value for {flag}"))?;
            match flag.as_str() {
                FLAG_OUTPUT => {
                    settings.set_output(PathBuf::from(value));
                }
                FLAG_DRIVER => {
                    settings.set_driver(PathBuf::from(value));
                }
                FLAG_BINDGEN => {
                    settings.set_bindgen(PathBuf::from(value));
                }
                FLAG_CARGO => {
                    settings.set_cargo(PathBuf::from(value));
                }
                FLAG_CHROME => {
                    settings.set_chrome(Some(PathBuf::from(value)));
                }
                FLAG_URL => {
                    settings.set_url(Some(value));
                }
                _ => return Err(format!("unknown option: {flag}")),
            }
        }
        Ok(settings)
    }
}
