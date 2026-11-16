use super::*;

/// Executes the automation runner or its private loopback-serving mode.
///
/// # Returns
///
/// - `AutomationResult<()>` - Complete suite result, including cleanup.
pub async fn run() -> AutomationResult<()> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments
        .first()
        .is_some_and(|value: &String| value == HELP_FLAG)
    {
        println!("{USAGE}");
        return Ok(());
    }
    if arguments
        .first()
        .is_some_and(|value: &String| value == SERVE_FLAG)
    {
        if arguments.len() != 3 {
            return Err(ERROR_SERVER_ARGUMENTS.to_string());
        }
        let port: u16 = arguments[2]
            .parse()
            .map_err(|error: ParseIntError| error.to_string())?;
        return serve_fixture(PathBuf::from(&arguments[1]), port).await;
    }
    let settings: Settings = Settings::parse(arguments)?;
    execute_suite(settings)
}

/// Waits for the actual driver while also proving the owned child is alive.
///
/// # Arguments
///
/// - `&mut OwnedProcess` - Newly created browser-driver process.
/// - `&str` - Driver endpoint.
///
/// # Returns
///
/// - `AutomationResult<()>` - Ready driver or a bounded startup failure.
fn wait_driver(process: &mut OwnedProcess, endpoint: &str) -> AutomationResult<()> {
    let started: Instant = Instant::now();
    loop {
        if let Some(status) = process
            .get_mut_child()
            .try_wait()
            .map_err(|error: io::Error| error.to_string())?
        {
            return Err(format!(
                "owned ChromeDriver exited before readiness: {status}"
            ));
        }
        if webdriver_request(endpoint, STATUS_PATH, HTTP_GET, None)
            .is_ok_and(|value: Value| value.get(JSON_READY).and_then(Value::as_bool) == Some(true))
        {
            return Ok(());
        }
        if started.elapsed() >= Duration::from_secs(15) {
            return Err(format!("ChromeDriver did not become ready at {endpoint}"));
        }
        sleep(Duration::from_millis(50));
    }
}

/// Waits for actual fixture bytes while checking the owned server process.
///
/// # Arguments
///
/// - `&mut OwnedProcess` - Newly created static server.
/// - `&str` - Fixture URL.
///
/// # Returns
///
/// - `AutomationResult<()>` - Ready serving path or a startup failure.
fn wait_server(process: &mut OwnedProcess, url: &str) -> AutomationResult<()> {
    let started: Instant = Instant::now();
    loop {
        if let Some(status) = process
            .get_mut_child()
            .try_wait()
            .map_err(|error: io::Error| error.to_string())?
        {
            return Err(format!(
                "owned fixture server exited before readiness: {status}"
            ));
        }
        let output: Result<Output, io::Error> = Command::new(CURL_PROGRAM)
            .args([
                CURL_SILENT,
                CURL_SHOW_ERROR,
                CURL_FAIL_BODY,
                CURL_NO_PROXY,
                PROXY_BYPASS,
                CURL_MAX_TIME,
                SERVER_REQUEST_TIMEOUT,
                url,
            ])
            .output();
        if let Ok(Output { status, stdout, .. }) = output
            && status.success()
            && String::from_utf8_lossy(&stdout).contains(FIXTURE_TITLE)
        {
            return Ok(());
        }
        if started.elapsed() >= Duration::from_secs(10) {
            return Err(format!("fixture did not become ready at {url}"));
        }
        sleep(Duration::from_millis(40));
    }
}

/// Runs every positive case, records failures, and verifies resource cleanup.
///
/// # Arguments
///
/// - `Settings` - Explicit runner configuration.
///
/// # Returns
///
/// - `AutomationResult<()>` - Success only when cases, artifacts and cleanup succeed.
fn execute_suite(settings: Settings) -> AutomationResult<()> {
    fs::create_dir_all(settings.get_output()).map_err(|error: io::Error| error.to_string())?;
    check_tool(settings.get_driver())?;
    check_tool(Path::new(CURL_PROGRAM))?;
    check_tool(Path::new(PYTHON_PROGRAM))?;
    let mut server: Option<OwnedProcess> = None;
    let mut server_port: Option<u16> = None;
    let url: String = if let Some(url) = settings.try_get_url() {
        url.clone()
    } else {
        let site: PathBuf = build_fixture(&settings)?;
        let port: u16 = available_port()?;
        let mut command: Command =
            Command::new(env::current_exe().map_err(|error: io::Error| error.to_string())?);
        command.arg(SERVE_FLAG).arg(&site).arg(port.to_string());
        let mut process: OwnedProcess = spawn_helper(
            &mut command,
            &settings.get_output().join(SERVER_LOG),
            SERVER_NAME,
        )?;
        let url: String = format!("http://127.0.0.1:{port}/");
        wait_server(&mut process, &url)?;
        server_port = Some(port);
        server = Some(process);
        url
    };
    let driver_port: u16 = available_port()?;
    let endpoint: String = format!("http://127.0.0.1:{driver_port}");
    let mut command: Command = Command::new(settings.get_driver());
    command
        .arg(format!("--port={driver_port}"))
        .arg(DRIVER_LOCAL_ONLY)
        .arg(DRIVER_VERBOSE);
    let mut driver: OwnedProcess = spawn_helper(
        &mut command,
        &settings.get_output().join(DRIVER_LOG),
        DRIVER_PROGRAM,
    )?;
    wait_driver(&mut driver, &endpoint)?;
    let browser: Browser = Browser::connect(&endpoint, settings.try_get_chrome().as_deref())?;
    let session: String = browser.get_session().clone();
    let capabilities: Value = browser.get_capabilities().clone();
    let mut scenarios: Vec<Scenario> = vec![
        Scenario::Counter,
        Scenario::Input,
        Scenario::Keyboard,
        Scenario::Checkbox,
        Scenario::Modal,
        Scenario::Scroll,
        Scenario::Responsive,
    ];
    if settings.get_probe_failure() {
        scenarios.push(Scenario::Negative);
    }
    let mut cases: Vec<CaseResult> = Vec::with_capacity(scenarios.len());
    for scenario in scenarios {
        let started: Instant = Instant::now();
        let outcome: AutomationResult<()> = scenario.execute(&browser, &url);
        let capture_error: Option<String> = browser
            .capture(
                &settings.get_output().join(SCREENSHOT_DIRECTORY),
                scenario.name(),
            )
            .err();
        let passed: bool = outcome.is_ok() && capture_error.is_none();
        println!(
            "{}: {}",
            scenario.name(),
            if passed { PASS_LABEL } else { FAIL_LABEL }
        );
        cases.push(CaseResult::new(
            scenario.name().to_string(),
            passed,
            outcome.err(),
            started.elapsed().as_millis(),
            capture_error,
        ));
    }
    let session_cleanup: AutomationResult<()> = browser.close();
    let driver_cleanup: AutomationResult<()> = driver.stop();
    let server_pid: Option<u32> = server
        .as_ref()
        .map(|process: &OwnedProcess| process.get_child().id());
    let server_cleanup: AutomationResult<()> = if let Some(process) = server.as_mut() {
        process.stop()
    } else {
        Ok(())
    };
    let cleaned_up: bool =
        session_cleanup.is_ok() && driver_cleanup.is_ok() && server_cleanup.is_ok();
    let resources: Resources = Resources::new(
        driver.get_child().id(),
        driver_port,
        server_pid,
        server_port,
        session,
        cleaned_up,
        capabilities,
    );
    let report: SuiteReport = SuiteReport::new(1, url, cases, resources);
    let path: PathBuf = settings.get_output().join(RESULT_FILE);
    fs::write(
        &path,
        to_string_pretty(&report).map_err(|error: Error| error.to_string())?,
    )
    .map_err(|error: io::Error| error.to_string())?;
    let failures: usize = report
        .get_cases()
        .iter()
        .filter(|case: &&CaseResult| !case.get_passed())
        .count();
    println!(
        "report={} cases={} failures={} cleaned_up={cleaned_up}",
        path.display(),
        report.get_cases().len(),
        failures
    );
    session_cleanup?;
    driver_cleanup?;
    server_cleanup?;
    if failures != 0 {
        return Err(format!(
            "{failures} UI automation case(s) failed; see {}",
            path.display()
        ));
    }
    Ok(())
}
