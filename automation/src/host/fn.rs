use super::*;

/// Finds an available loopback port without claiming a pre-existing listener.
///
/// # Returns
///
/// - `AutomationResult<u16>` - Ephemeral port selected by the operating system.
pub(crate) fn available_port() -> AutomationResult<u16> {
    let listener: TcpListener =
        TcpListener::bind(LOOPBACK_EPHEMERAL).map_err(|error: io::Error| error.to_string())?;
    Ok(listener
        .local_addr()
        .map_err(|error: io::Error| error.to_string())?
        .port())
}

/// Executes a build prerequisite and requires an actual successful producer.
///
/// # Arguments
///
/// - `&mut Command` - Process configured without a shell.
///
/// # Returns
///
/// - `AutomationResult<Output>` - Complete successful process output.
pub(crate) fn checked_output(command: &mut Command) -> AutomationResult<Output> {
    let output: Output = command
        .output()
        .map_err(|error: io::Error| format!("cannot run {command:?}: {error}"))?;
    let Output {
        status,
        stdout,
        stderr,
    }: &Output = &output;
    if !status.success() {
        return Err(format!(
            "command {command:?} failed ({status}): {} {}",
            String::from_utf8_lossy(stderr),
            String::from_utf8_lossy(stdout)
        ));
    }
    Ok(output)
}

/// Checks for a nonempty version result so empty executables cannot fake success.
///
/// # Arguments
///
/// - `&Path` - Tool executable.
///
/// # Returns
///
/// - `AutomationResult<()>` - Verified executable or a prerequisite failure.
pub(crate) fn check_tool(program: &Path) -> AutomationResult<()> {
    let Output { stdout, stderr, .. }: Output =
        checked_output(Command::new(program).arg(VERSION_FLAG))?;
    if stdout.is_empty() && stderr.is_empty() {
        return Err(format!(
            "tool produced no version output: {}",
            program.display()
        ));
    }
    Ok(())
}

/// Compiles the real euv fixture and generates its matching WASM bridge.
///
/// # Arguments
///
/// - `&Settings` - Tool and artifact configuration.
///
/// # Returns
///
/// - `AutomationResult<PathBuf>` - Serving directory containing real compiled assets.
pub(crate) fn build_fixture(settings: &Settings) -> AutomationResult<PathBuf> {
    let site: PathBuf = settings.get_output().join(SITE_DIRECTORY);
    let package: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    fs::create_dir_all(site.join(PKG_DIRECTORY)).map_err(|error: io::Error| error.to_string())?;
    if !settings.get_skip_build() {
        check_tool(settings.get_cargo())?;
        check_tool(settings.get_bindgen())?;
        checked_output(
            Command::new(settings.get_cargo())
                .args([
                    BUILD_COMMAND,
                    PACKAGE_FLAG,
                    PACKAGE_NAME,
                    LIB_FLAG,
                    RELEASE_FLAG,
                    TARGET_FLAG,
                    WASM_TARGET,
                ])
                .current_dir(
                    package
                        .parent()
                        .ok_or_else(|| ERROR_WORKSPACE_PARENT.to_string())?,
                ),
        )?;
        let Output { stdout, .. }: Output = checked_output(
            Command::new(settings.get_cargo())
                .args([METADATA_COMMAND, NO_DEPS_FLAG, FORMAT_FLAG, FORMAT_VERSION])
                .current_dir(&package),
        )?;
        let metadata: Value = from_slice(&stdout).map_err(|error: Error| error.to_string())?;
        let target: &str = metadata
            .get(TARGET_DIRECTORY_KEY)
            .and_then(Value::as_str)
            .ok_or_else(|| ERROR_TARGET_DIRECTORY.to_string())?;
        let wasm: PathBuf = Path::new(target)
            .join(WASM_TARGET)
            .join(RELEASE_DIRECTORY)
            .join(RAW_WASM_FILE);
        if !wasm.is_file()
            || wasm
                .metadata()
                .map_err(|error: io::Error| error.to_string())?
                .len()
                == 0
        {
            return Err(format!("no compiled WASM fixture at {}", wasm.display()));
        }
        checked_output(
            Command::new(settings.get_bindgen())
                .arg(&wasm)
                .args([TARGET_FLAG, WEB_TARGET, OUT_DIR_FLAG])
                .arg(site.join(PKG_DIRECTORY))
                .args([OUT_NAME_FLAG, BRIDGE_NAME]),
        )?;
    }
    for file in [BRIDGE_FILE, WASM_FILE] {
        let path: PathBuf = site.join(PKG_DIRECTORY).join(file);
        if !path.is_file()
            || path
                .metadata()
                .map_err(|error: io::Error| error.to_string())?
                .len()
                == 0
        {
            return Err(format!(
                "fixture asset missing or empty: {}",
                path.display()
            ));
        }
    }
    fs::write(site.join(INDEX_FILE), INDEX_HTML).map_err(|error: io::Error| error.to_string())?;
    Ok(site)
}

/// Starts a helper with owned logs and an immediately installed process guard.
///
/// # Arguments
///
/// - `&mut Command` - Configured helper command.
/// - `&Path` - Diagnostic log file.
/// - `&'static str` - Human-readable helper identity.
///
/// # Returns
///
/// - `AutomationResult<OwnedProcess>` - Guard owning the started helper.
pub(crate) fn spawn_helper(
    command: &mut Command,
    log: &Path,
    name: &'static str,
) -> AutomationResult<OwnedProcess> {
    let output: File = File::create(log).map_err(|error: io::Error| error.to_string())?;
    let errors: File = output
        .try_clone()
        .map_err(|error: io::Error| error.to_string())?;
    let child: Child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::from(output))
        .stderr(Stdio::from(errors))
        .spawn()
        .map_err(|error: io::Error| format!("cannot start {name}: {error}"))?;
    Ok(OwnedProcess::new(child, name))
}

/// Serves the generated fixture until the owning parent closes standard input.
///
/// # Arguments
///
/// - `PathBuf` - Canonical serving root.
/// - `u16` - Loopback listening port.
///
/// # Returns
///
/// - `AutomationResult<()>` - Confirmed server startup and normal parent-directed exit.
pub(crate) async fn serve_fixture(root: PathBuf, port: u16) -> AutomationResult<()> {
    SERVING_ROOT
        .set(
            root.canonicalize()
                .map_err(|error: io::Error| error.to_string())?,
        )
        .map_err(|_: PathBuf| ERROR_SERVER_REINITIALIZED.to_string())?;
    let mut config: ServerConfig = ServerConfig::default();
    config.set_address(format!("127.0.0.1:{port}"));
    let mut server: Server = Server::default();
    server
        .server_config(config)
        .route::<FixtureFiles>(ROOT_ROUTE)
        .route::<FixtureFiles>(FILE_ROUTE);
    let control: ServerControlHook = server
        .run()
        .await
        .map_err(|error: Box<ServerError>| format!("fixture server failed: {error}"))?;
    let _: usize = spawn_blocking(|| {
        let mut bytes: Vec<u8> = Vec::new();
        io::stdin().read_to_end(&mut bytes)
    })
    .await
    .map_err(|error: JoinError| error.to_string())?
    .map_err(|error: io::Error| error.to_string())?;
    drop(control);
    Ok(())
}
