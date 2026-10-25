use super::*;

/// Default construction for [`Config`].
impl Default for Config {
    /// Constructs a default [`Config`] value.
    fn default() -> Self {
        Self::new(
            DEFAULT_SUITES.to_vec(),
            100,
            7,
            20,
            1,
            DEFAULT_LABEL.to_string(),
        )
    }
}

/// Inherent implementation of [`Config`].
impl Config {
    /// Parses explicit benchmark options without silently ignoring unknown inputs.
    ///
    /// # Arguments
    ///
    /// - `I` - Command-line arguments excluding the executable name.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Box<dyn Error>>` - Validated configuration or the invalid option.
    pub(crate) fn parse<I>(arguments: I) -> Result<Self, Box<dyn Error>>
    where
        I: IntoIterator<Item = String>,
    {
        let mut config: Self = Self::default();
        let mut arguments: I::IntoIter = arguments.into_iter();
        while let Some(flag) = arguments.next() {
            if flag == FLAG_HELP {
                config.set_help(true);
                continue;
            }
            let value: String = arguments
                .next()
                .ok_or_else(|| format!("missing value for {flag}"))?;
            match flag.as_str() {
                FLAG_TICKS => config.set_ticks(value.parse()?),
                FLAG_ROUNDS => config.set_rounds(value.parse()?),
                FLAG_WARMUP => config.set_warmup(value.parse()?),
                FLAG_SCALE => config.set_scale(value.parse()?),
                FLAG_LABEL => config.set_label(value),
                FLAG_OUTPUT => config.set_output(Some(PathBuf::from(value))),
                FLAG_SUITE => {
                    let suites: Vec<Suite> = match value.as_str() {
                        SUITE_ALL => DEFAULT_SUITES.to_vec(),
                        SUITE_CORE => vec![Suite::Core],
                        SUITE_ENGINE => vec![Suite::Engine],
                        SUITE_CHART => vec![Suite::Chart],
                        SUITE_MIXED => vec![Suite::Mixed],
                        _ => return Err(format!("unknown suite: {value}").into()),
                    };
                    config.set_suites(suites)
                }
                _ => return Err(format!("unknown option: {flag}").into()),
            };
        }
        if config.get_ticks() == 0
            || config.get_rounds() == 0
            || config.get_scale() == 0
            || config.get_scale() > 16
        {
            return Err(format!(
                "ticks and rounds must be positive; scale must be in 1..=16: {config:?}"
            )
            .into());
        }
        Ok(config)
    }
}
