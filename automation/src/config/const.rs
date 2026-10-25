/// Runs against an already-served compatible fixture.
pub(crate) const FLAG_URL: &str = "--url";

/// Environment override for the Cargo executable.
pub(crate) const CARGO_ENV: &str = "CARGO";

/// Selects a healthy Cargo executable.
pub(crate) const FLAG_CARGO: &str = "--cargo";

/// Overrides the Chrome binary used by the driver.
pub(crate) const FLAG_CHROME: &str = "--chrome";

/// Selects the ChromeDriver executable.
pub(crate) const FLAG_DRIVER: &str = "--driver";

/// Selects the ignored report and fixture directory.
pub(crate) const FLAG_OUTPUT: &str = "--output";

/// Selects the matching wasm-bindgen executable.
pub(crate) const FLAG_BINDGEN: &str = "--bindgen";

/// Default Rust package manager executable.
pub(crate) const CARGO_PROGRAM: &str = "cargo";

/// Default browser-driver executable.
pub(crate) const DRIVER_PROGRAM: &str = "chromedriver";

/// Default matching WASM bridge generator.
pub(crate) const BINDGEN_PROGRAM: &str = "wasm-bindgen";

/// Explicitly reuses an existing fixture build.
pub(crate) const FLAG_SKIP_BUILD: &str = "--skip-build";

/// Default ignored artifact directory.
pub(crate) const RESULTS_DIRECTORY: &str = "results";

/// Adds a deliberate failing assertion for negative-control verification.
pub(crate) const FLAG_PROBE_FAILURE: &str = "--probe-failure";
