use super::*;

/// Builds a complete static documentation site from all `.md` files beneath `src_dir` into `out_dir`.
///
/// # Arguments
///
/// - `&Path` - Project or Markdown source root directory.
/// - `&Path` - Destination output directory.
/// - `&DocsBuildOptions` - Site title, base URL, clean URL, asset copy, and optional dev HMR configuration.
///
/// # Returns
///
/// - `io::Result<GeneratedSiteReport>` - Summary report of generated pages or I/O error.
pub fn build_docs_site(
    src_dir: &Path,
    out_dir: &Path,
    options: &DocsBuildOptions,
) -> io::Result<GeneratedSiteReport> {
    build_docs_bundle(src_dir, out_dir, options)
}

/// Compiles a fresh signed Dynamic WASM OTA hot-update bundle for version `version`.
///
/// # Arguments
///
/// - `u64` - Monotonic OTA bundle version number.
/// - `&str` - Human-readable title embedded into the executable `.wasm` module.
/// - `Vec<String>` - Source file paths triggering this hot update.
///
/// # Returns
///
/// - `OtaBundleState` - Signed manifest and executable `.wasm` bytes.
#[must_use]
pub fn build_dev_ota_state(
    version: u64,
    title: &str,
    changed_files: Vec<String>,
) -> OtaBundleState {
    let accent_hex: u32 = if version.is_multiple_of(2) {
        0x2563EB
    } else {
        0x059669
    };
    let wasm_bytes: Vec<u8> = build_partial_widget_wasm(title, version, accent_hex);
    let checksum: u64 = compute_wasm_checksum(&wasm_bytes);
    let signature: String = compute_wasm_signature(&wasm_bytes, version, OTA_DEV_SIGNING_KEY);
    let manifest: OtaManifest = OtaManifest::new(
        version,
        OTA_SLOT_DEV.to_string(),
        OTA_MODULE_ENDPOINT.to_string(),
        checksum,
        signature,
        wasm_bytes.len(),
        changed_files,
    );
    OtaBundleState::new(manifest, wasm_bytes)
}

/// Injects the dev-only SSE hot-reload script right before `</body>` in an HTML document.
///
/// # Arguments
///
/// - `&str` - Original HTML source string.
/// - `&str` - SSE HMR endpoint URL path.
///
/// # Returns
///
/// - `String` - HTML document containing the live-reload client snippet.
#[must_use]
pub fn inject_dev_hmr_into_html(html: &str, hmr_endpoint: &str) -> String {
    if html.contains(hmr_endpoint) {
        return html.to_string();
    }
    let snippet: String = format!("{DEV_HMR_SCRIPT_PREFIX}{hmr_endpoint}{DEV_HMR_SCRIPT_SUFFIX}");
    if let Some(pos) = html.rfind(HTML_BODY_CLOSE_TAG) {
        let mut out: String = String::with_capacity(html.len() + snippet.len());
        out.push_str(&html[..pos]);
        out.push_str(&snippet);
        out.push_str(&html[pos..]);
        out
    } else {
        format!("{html}{snippet}")
    }
}

/// Starts a live HTTP, SSE hot-reload, file-watcher, and Dynamic WASM OTA dev server in background threads.
///
/// # Arguments
///
/// - `DevServerConfig` - Full dev server configuration.
///
/// # Returns
///
/// - `io::Result<DevServerHandle>` - Running server handle or startup error.
pub fn start_dev_server(config: DevServerConfig) -> io::Result<DevServerHandle> {
    if config.get_target() != DevTargetArg::Native {
        fs::create_dir_all(config.get_out_dir())?;
    }
    let mut initial_pages: usize = 0;
    let mode_label: String = match config.get_target() {
        DevTargetArg::Docs => {
            let docs_opts: DocsBuildOptions = docs_options_for_dev(&config);
            let report: GeneratedSiteReport =
                build_docs_site(config.get_src_dir(), config.get_out_dir(), &docs_opts)?;
            initial_pages = report.get_page_count();
            MODE_LABEL_DOCS.to_string()
        }
        DevTargetArg::Web => {
            rebuild_web_dev_bundle(&config)?;
            MODE_LABEL_WEB.to_string()
        }
        DevTargetArg::Native => MODE_LABEL_NATIVE.to_string(),
    };
    let initial_ota: OtaBundleState = build_dev_ota_state(1, OTA_INITIAL_TITLE, Vec::new());
    let hub: Arc<HmrHub> = Arc::new(HmrHub::new(initial_ota));
    let bind_addr: String = format!("{}:{}", config.get_host(), config.get_port());
    let listener: TcpListener = TcpListener::bind(&bind_addr)?;
    let address: SocketAddr = listener.local_addr()?;
    listener.set_nonblocking(true)?;
    let serve_ctx: HttpServeContext = HttpServeContext {
        out_dir: config.get_out_dir().clone(),
        base_url: config.get_base_url().clone(),
        enable_ota: config.get_enable_ota(),
        hub: Arc::clone(&hub),
    };
    let http_thread: JoinHandle<()> = thread::spawn(move || {
        run_http_accept_loop(listener, serve_ctx);
    });
    let watch_hub: Arc<HmrHub> = Arc::clone(&hub);
    let watch_cfg: DevServerConfig = config.clone();
    let dev_url: String = format!("http://{address}");
    let watch_thread: JoinHandle<()> = thread::spawn(move || {
        run_watch_rebuild_loop(watch_cfg, dev_url, watch_hub);
    });
    Ok(DevServerHandle {
        address,
        out_dir: config.get_out_dir().clone(),
        mode: mode_label,
        page_count: initial_pages,
        hub,
        http_thread: Some(http_thread),
        watch_thread: Some(watch_thread),
    })
}

/// Starts the dev server and either exits immediately (`once=true`) or serves continuously until interrupted.
///
/// # Arguments
///
/// - `DevServerConfig` - Dev server configuration.
///
/// # Returns
///
/// - `io::Result<DevServerReport>` - Server startup summary report or I/O error.
pub fn run_dev_blocking(config: DevServerConfig) -> io::Result<DevServerReport> {
    let once: bool = config.get_once();
    let mut handle: DevServerHandle = start_dev_server(config)?;
    let report: DevServerReport = handle.report();
    if once {
        handle.shutdown();
        return Ok(report);
    }
    eprintln!(
        "[euv-dev] mode={} url={} hmr={} ota={}",
        report.get_mode(),
        report.get_url(),
        report.get_hmr_url(),
        report.get_ota_manifest_url()
    );
    while handle.hub.is_running() {
        thread::sleep(Duration::from_millis(250));
    }
    handle.shutdown();
    Ok(report)
}

/// Constructs `DocsBuildOptions` wired for live dev hot reload from a `DevServerConfig`.
///
/// # Arguments
///
/// - `&DevServerConfig` - Active dev server configuration.
///
/// # Returns
///
/// - `DocsBuildOptions` - Populated docs build options with dev HMR endpoint.
fn docs_options_for_dev(config: &DevServerConfig) -> DocsBuildOptions {
    let mut opts: DocsBuildOptions = DocsBuildOptions::default();
    opts.set_title(config.try_get_title().clone())
        .set_base_url(config.get_base_url().clone())
        .set_hmr_endpoint(Some(HMR_ENDPOINT.to_string()))
        .set_assets_root(Some(config.get_src_dir().clone()))
        .set_clean_urls(true);
    opts
}

/// Runs the non-blocking HTTP accept loop dispatching static files, SSE `/__euv_hmr`, and OTA endpoints.
///
/// # Arguments
///
/// - `TcpListener` - Bound non-blocking TCP listener.
/// - `HttpServeContext` - Output directory, base prefix, OTA gate, and shared HMR hub.
fn run_http_accept_loop(listener: TcpListener, ctx: HttpServeContext) {
    while ctx.get_hub().is_running() {
        match listener.accept() {
            Ok((stream, _)) => {
                if !ctx.get_hub().is_running() {
                    break;
                }
                let conn_ctx: HttpServeContext = ctx.clone();
                drop(thread::spawn(move || {
                    serve_http_connection(stream, &conn_ctx);
                }));
            }
            Err(err) if err.kind() == ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(20));
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
}

/// Runs the recursive file watcher loop rebuilding docs, web bundles, or native binaries on file changes.
///
/// # Arguments
///
/// - `DevServerConfig` - Active dev server configuration.
/// - `String` - Base HTTP URL of the active dev server forwarded to native child processes.
/// - `Arc<HmrHub>` - Shared HMR and OTA hub.
fn run_watch_rebuild_loop(config: DevServerConfig, dev_url: String, hub: Arc<HmrHub>) {
    let allowed_exts: &[&str] = match config.get_target() {
        DevTargetArg::Docs => &WATCH_EXTS_DOCS,
        DevTargetArg::Web | DevTargetArg::Native => &WATCH_EXTS_CODE,
    };
    let exclude_out: Option<&Path> = if config.get_target() == DevTargetArg::Native {
        None
    } else {
        Some(config.get_out_dir().as_path())
    };
    let mut prev_snap: WatchSnapshot =
        WatchSnapshot::scan_excluding(config.get_src_dir(), allowed_exts, exclude_out)
            .unwrap_or_default();
    let mut native_child: Option<Child> =
        if config.get_target() == DevTargetArg::Native && !config.get_once() {
            spawn_native_app_child(&config, &dev_url).ok()
        } else {
            None
        };
    let poll_dur: Duration = Duration::from_millis(config.get_poll_ms().clamp(50, 5000));
    while hub.is_running() {
        thread::sleep(poll_dur);
        if !hub.is_running() {
            break;
        }
        let Ok(next_snap) =
            WatchSnapshot::scan_excluding(config.get_src_dir(), allowed_exts, exclude_out)
        else {
            continue;
        };
        let changes: FileChangeSet = prev_snap.diff(&next_snap);
        if changes.is_empty() {
            continue;
        }
        prev_snap = next_snap;
        let changed_files: Vec<String> = changes.all_paths();
        match config.get_target() {
            DevTargetArg::Docs => {
                let docs_opts: DocsBuildOptions = docs_options_for_dev(&config);
                match build_docs_site(config.get_src_dir(), config.get_out_dir(), &docs_opts) {
                    Ok(report) => {
                        let next_ver: u64 = hub.revision() + 1;
                        if config.get_enable_ota() {
                            let ota_title: String = format!(
                                "Docs Hot Update v{next_ver} ({} pages)",
                                report.get_page_count()
                            );
                            hub.set_ota_state(build_dev_ota_state(
                                next_ver,
                                &ota_title,
                                changed_files.clone(),
                            ));
                        }
                        let _: u64 = hub.publish(
                            HmrEventKind::Reload,
                            format!("{MSG_DOCS_REBUILT} ({} pages)", report.get_page_count()),
                            changed_files,
                        );
                    }
                    Err(err) => {
                        let _: u64 =
                            hub.publish(HmrEventKind::BuildError, err.to_string(), changed_files);
                    }
                }
            }
            DevTargetArg::Web => match rebuild_web_dev_bundle(&config) {
                Ok(()) => {
                    let next_ver: u64 = hub.revision() + 1;
                    if config.get_enable_ota() {
                        let ota_title: String = format!("Web WASM Hot Update v{next_ver}");
                        hub.set_ota_state(build_dev_ota_state(
                            next_ver,
                            &ota_title,
                            changed_files.clone(),
                        ));
                    }
                    let _: u64 = hub.publish(
                        HmrEventKind::Reload,
                        MSG_WEB_REBUILT.to_string(),
                        changed_files,
                    );
                }
                Err(err) => {
                    let _: u64 =
                        hub.publish(HmrEventKind::BuildError, err.to_string(), changed_files);
                }
            },
            DevTargetArg::Native => {
                if let Some(mut old_child) = native_child.take() {
                    drop(old_child.kill());
                    drop(old_child.wait());
                }
                match spawn_native_app_child(&config, &dev_url) {
                    Ok(child) => {
                        native_child = Some(child);
                        let next_ver: u64 = hub.revision() + 1;
                        if config.get_enable_ota() {
                            let ota_title: String = format!("Native OTA Hot Update v{next_ver}");
                            hub.set_ota_state(build_dev_ota_state(
                                next_ver,
                                &ota_title,
                                changed_files.clone(),
                            ));
                        }
                        let _: u64 = hub.publish(
                            HmrEventKind::OtaUpdate,
                            MSG_NATIVE_RESTARTED.to_string(),
                            changed_files,
                        );
                    }
                    Err(err) => {
                        let _: u64 =
                            hub.publish(HmrEventKind::BuildError, err.to_string(), changed_files);
                    }
                }
            }
        }
    }
    if let Some(mut child) = native_child.take() {
        drop(child.kill());
        drop(child.wait());
    }
}

/// Rebuilds the WebAssembly bundle into `config.out_dir` and injects the dev SSE hot-reload script.
///
/// # Arguments
///
/// - `&DevServerConfig` - Active web dev configuration.
///
/// # Returns
///
/// - `io::Result<()>` - Build result.
fn rebuild_web_dev_bundle(config: &DevServerConfig) -> io::Result<()> {
    let out_dir: &Path = config.get_out_dir();
    let mut wp_args: Vec<String> = vec![
        ARG_BUILD.to_string(),
        ARG_TARGET.to_string(),
        ARG_TARGET_WEB.to_string(),
        ARG_OUT_DIR.to_string(),
        out_dir.display().to_string(),
    ];
    if config.get_release() {
        wp_args.push(ARG_RELEASE.to_string());
    }
    let output: Output = Command::new(BIN_WASM_PACK)
        .args(&wp_args)
        .current_dir(config.get_src_dir())
        .output()?;
    if !output.status.success() {
        let stderr: String = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(Error::other(format!("{ERR_STEP_FAILED}: {stderr}")));
    }
    if out_dir.exists() {
        let index_path: PathBuf = package_zero_js_web_bundle(out_dir, WEB_BUNDLE_TITLE)?;
        let html: String = fs::read_to_string(&index_path)?;
        let with_hmr: String = inject_dev_hmr_into_html(&html, HMR_ENDPOINT);
        fs::write(&index_path, with_hmr)?;
    }
    Ok(())
}

/// Spawns a fresh native `cargo run` child process with live HMR/OTA endpoint env vars.
///
/// # Arguments
///
/// - `&DevServerConfig` - Active native dev configuration.
/// - `&str` - Base HTTP URL of the active dev server.
///
/// # Returns
///
/// - `io::Result<Child>` - Spawned child process handle or error.
fn spawn_native_app_child(config: &DevServerConfig, dev_url: &str) -> io::Result<Child> {
    let step: BuildStep = resolve_run_step(
        TargetArg::Native,
        config.get_render(),
        config.get_release(),
        config.try_get_package().as_deref(),
    );
    let mut cmd: Command = Command::new(step.get_program());
    let _: &mut Command = cmd
        .args(step.get_args())
        .current_dir(config.get_src_dir())
        .stdin(Stdio::null())
        .env(ENV_DEV_URL, dev_url)
        .env(ENV_HMR_URL, format!("{dev_url}{HMR_ENDPOINT}"));
    if config.get_enable_ota() {
        let _: &mut Command = cmd
            .env(
                ENV_OTA_MANIFEST_URL,
                format!("{dev_url}{OTA_MANIFEST_ENDPOINT}"),
            )
            .env(
                ENV_OTA_MODULE_URL,
                format!("{dev_url}{OTA_MODULE_ENDPOINT}"),
            );
    }
    cmd.spawn()
}

/// Handles a single incoming HTTP/1.1 connection for static files, SSE `/__euv_hmr`, or Dynamic WASM OTA.
///
/// # Arguments
///
/// - `TcpStream` - Connected client socket.
/// - `&HttpServeContext` - Output directory, base prefix, OTA gate, and shared HMR hub.
fn serve_http_connection(mut stream: TcpStream, ctx: &HttpServeContext) {
    drop(stream.set_nonblocking(false));
    drop(stream.set_read_timeout(Some(Duration::from_secs(3))));
    let req_line: String = {
        let mut reader: BufReader<&TcpStream> = BufReader::new(&stream);
        let mut line: String = String::new();
        if reader.read_line(&mut line).is_err() || line.is_empty() {
            return;
        }
        let mut header_count: usize = 0;
        while header_count < 64 {
            let mut hdr: String = String::new();
            match reader.read_line(&mut hdr) {
                Ok(0) => break,
                Ok(_) if hdr == "\r\n" || hdr == "\n" => break,
                Ok(_) => header_count += 1,
                Err(_) => break,
            }
        }
        line
    };
    let mut parts: SplitWhitespace<'_> = req_line.split_whitespace();
    let method: &str = parts.next().unwrap_or("");
    let raw_target: &str = parts.next().unwrap_or("/");
    if method.eq_ignore_ascii_case(HTTP_METHOD_OPTIONS) {
        drop(stream.write_all(HTTP_204_OPTIONS_RESP.as_bytes()));
        drop(stream.flush());
        return;
    }
    let head_only: bool = method.eq_ignore_ascii_case(HTTP_METHOD_HEAD);
    let path_only: &str = raw_target.split('?').next().unwrap_or("/");
    let hub: &Arc<HmrHub> = ctx.get_hub();
    if path_only == HMR_ENDPOINT {
        serve_sse_stream(stream, hub);
        return;
    }
    if path_only == OTA_MANIFEST_ENDPOINT {
        if ctx.get_enable_ota()
            && let Some(ota) = hub.ota_snapshot()
            && let Ok(json) = to_string_pretty(ota.get_manifest())
        {
            write_http_response(&mut stream, MIME_JSON, json.as_bytes(), head_only);
            return;
        }
        drop(stream.write_all(HTTP_404_RESP.as_bytes()));
        return;
    }
    if path_only == OTA_MODULE_ENDPOINT {
        if ctx.get_enable_ota()
            && let Some(ota) = hub.ota_snapshot()
        {
            write_http_response(
                &mut stream,
                MIME_WASM,
                ota.get_wasm_bytes().as_slice(),
                head_only,
            );
            return;
        }
        drop(stream.write_all(HTTP_404_RESP.as_bytes()));
        return;
    }
    if path_only == DEV_STATUS_ENDPOINT {
        let payload: String = match hub.current_event() {
            Some(ev) => to_string_pretty(&ev).unwrap_or_else(|_| EMPTY_JSON_OBJECT.to_string()),
            None => format!("{{\"revision\":{}}}", hub.revision()),
        };
        write_http_response(&mut stream, MIME_JSON, payload.as_bytes(), head_only);
        return;
    }
    let decoded_path: String = percent_decode_path(path_only);
    let stripped_path: &str = strip_base_prefix(&decoded_path, ctx.get_base_url());
    if let Some(file_path) = resolve_static_file(ctx.get_out_dir(), stripped_path)
        && let Ok(bytes) = fs::read(&file_path)
    {
        let mime: &str = mime_type_for_path(&file_path);
        write_http_response(&mut stream, mime, &bytes, head_only);
        return;
    }
    drop(stream.write_all(HTTP_404_RESP.as_bytes()));
}

/// Streams live Server-Sent Events (`event: reload` / `ota` / `error`) whenever `hub` increments revision.
///
/// # Arguments
///
/// - `TcpStream` - Long-lived client TCP stream.
/// - `&Arc<HmrHub>` - Shared HMR hub.
fn serve_sse_stream(mut stream: TcpStream, hub: &Arc<HmrHub>) {
    if stream.write_all(HTTP_SSE_HEADERS.as_bytes()).is_err() || stream.flush().is_err() {
        return;
    }
    let mut last_sent_rev: u64 = hub.revision();
    let mut idle_ticks: u32 = 0;
    while hub.is_running() {
        thread::sleep(Duration::from_millis(50));
        let cur_rev: u64 = hub.revision();
        if cur_rev != last_sent_rev {
            last_sent_rev = cur_rev;
            idle_ticks = 0;
            if let Some(ev) = hub.current_event() {
                let ev_name: &str = match ev.get_kind() {
                    HmrEventKind::Reload => HMR_EVENT_RELOAD,
                    HmrEventKind::OtaUpdate => HMR_EVENT_OTA,
                    HmrEventKind::BuildError => HMR_EVENT_ERROR,
                };
                let data: String = to_string_pretty(&ev)
                    .unwrap_or_else(|_| format!("{{\"revision\":{cur_rev}}}"))
                    .replace('\n', " ");
                let frame: String = format!("event: {ev_name}\ndata: {data}\n\n");
                if stream.write_all(frame.as_bytes()).is_err() || stream.flush().is_err() {
                    return;
                }
            }
        } else {
            idle_ticks += 1;
            if idle_ticks >= 100 {
                idle_ticks = 0;
                if stream.write_all(SSE_PING_LINE.as_bytes()).is_err() || stream.flush().is_err() {
                    return;
                }
            }
        }
    }
}

/// Writes an HTTP/1.1 200 response with CORS and no-cache headers, omitting the body on `HEAD` requests.
///
/// # Arguments
///
/// - `&mut TcpStream` - Client socket.
/// - `&str` - Content-Type header value.
/// - `&[u8]` - Response payload bytes.
/// - `bool` - When `true`, writes headers only (`HEAD`).
fn write_http_response(stream: &mut TcpStream, mime: &str, body: &[u8], head_only: bool) {
    let header: String = format!(
        "{HTTP_200_PREFIX}{mime}{HTTP_CORS_HEADERS}{}{HTTP_CONN_CLOSE_SEP}",
        body.len()
    );
    drop(stream.write_all(header.as_bytes()));
    if !head_only {
        drop(stream.write_all(body));
    }
    drop(stream.flush());
}

/// Strips a configured `base_url` prefix from an incoming request path when present.
///
/// # Arguments
///
/// - `&'a str` - Percent-decoded HTTP request path.
/// - `&str` - Configured base URL prefix (for example `""` or `"/docs"`).
///
/// # Returns
///
/// - `&'a str` - Relative path rooted at the output directory.
fn strip_base_prefix<'a>(path: &'a str, base_url: &str) -> &'a str {
    let normalized: &str = base_url.trim_matches('/');
    if normalized.is_empty() {
        return path;
    }
    let trimmed: &str = path.trim_start_matches('/');
    if let Some(rest) = trimmed.strip_prefix(normalized)
        && (rest.is_empty() || rest.starts_with('/'))
    {
        return rest;
    }
    path
}

/// Decodes `%XX` percent-encoded byte sequences in a URL path into UTF-8 text.
///
/// # Arguments
///
/// - `&str` - Raw URL path component.
///
/// # Returns
///
/// - `String` - Decoded UTF-8 URL path.
fn percent_decode_path(input: &str) -> String {
    if !input.contains('%') {
        return input.to_string();
    }
    let bytes: &[u8] = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i: usize = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let (Some(hi), Some(lo)) = (hex_nibble(bytes[i + 1]), hex_nibble(bytes[i + 2]))
        {
            out.push((hi << 4) | lo);
            i += 3;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8(out).unwrap_or_else(|_| input.to_string())
}

/// Converts a single ASCII hexadecimal byte into its 4-bit value.
///
/// # Arguments
///
/// - `u8` - Candidate ASCII hex byte.
///
/// # Returns
///
/// - `Option<u8>` - Numeric nibble (`0..=15`) when valid.
fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// Resolves an HTTP request path safely within `out_dir` supporting clean URLs and `index.html` fallbacks.
///
/// # Arguments
///
/// - `&Path` - Served output directory root.
/// - `&str` - Sanitized URL path component.
///
/// # Returns
///
/// - `Option<PathBuf>` - Existing file path inside `out_dir`, if found.
fn resolve_static_file(out_dir: &Path, url_path: &str) -> Option<PathBuf> {
    let trimmed: &str = url_path.trim_matches('/');
    let mut safe_rel: PathBuf = PathBuf::new();
    for seg in trimmed.split('/') {
        if seg.is_empty() || seg == "." || seg == ".." {
            continue;
        }
        safe_rel.push(seg);
    }
    if safe_rel.as_os_str().is_empty() {
        let root_idx: PathBuf = out_dir.join(INDEX_HTML_NAME);
        return root_idx.is_file().then_some(root_idx);
    }
    let direct: PathBuf = out_dir.join(&safe_rel);
    if direct.is_file() {
        return Some(direct);
    }
    let clean_idx: PathBuf = direct.join(INDEX_HTML_NAME);
    if clean_idx.is_file() {
        return Some(clean_idx);
    }
    let html_file: PathBuf = out_dir.join(format!("{}.html", safe_rel.display()));
    if html_file.is_file() {
        return Some(html_file);
    }
    None
}

/// Determines the HTTP `Content-Type` header for a static file path by extension.
///
/// # Arguments
///
/// - `&Path` - File path.
///
/// # Returns
///
/// - `&'static str` - Appropriate MIME type string.
#[must_use]
pub fn mime_type_for_path(path: &Path) -> &'static str {
    let Some(ext) = path.extension().and_then(|s: &OsStr| s.to_str()) else {
        return MIME_OCTET;
    };
    if ext.eq_ignore_ascii_case(EXT_HTML) {
        MIME_HTML
    } else if ext.eq_ignore_ascii_case(EXT_JSON) {
        MIME_JSON
    } else if ext.eq_ignore_ascii_case(EXT_XML) {
        MIME_XML
    } else if ext.eq_ignore_ascii_case(EXT_CSS) {
        MIME_CSS
    } else if ext.eq_ignore_ascii_case(EXT_WASM) {
        MIME_WASM
    } else if ext.eq_ignore_ascii_case(EXT_SVG) {
        MIME_SVG
    } else if ext.eq_ignore_ascii_case(EXT_PNG) {
        MIME_PNG
    } else if ext.eq_ignore_ascii_case(EXT_JPG) || ext.eq_ignore_ascii_case(EXT_JPEG) {
        MIME_JPEG
    } else if ext.eq_ignore_ascii_case(EXT_WEBP) {
        MIME_WEBP
    } else {
        MIME_OCTET
    }
}

/// Recursively populates `entries` with modification stamps for all watched files under `dir`.
///
/// # Arguments
///
/// - `&Path` - Current directory to scan.
/// - `&[&str]` - Allowed lowercase file extensions.
/// - `Option<&Path>` - Optional canonical output directory excluded from watching.
/// - `Option<&Path>` - Canonical path of `dir` when `exclude_dir` lies within `dir`.
/// - `bool` - Whether non-symlink child directories must be tested against `exclude_dir`.
/// - `&mut BTreeMap<PathBuf, FileStamp>` - Accumulator map.
///
/// # Returns
///
/// - `io::Result<()>` - Scan result.
pub(crate) fn scan_dir_stamps(
    dir: &Path,
    allowed_exts: &[&str],
    exclude_dir: Option<&Path>,
    canon_dir: Option<&Path>,
    check_real_exclude: bool,
    entries: &mut BTreeMap<PathBuf, FileStamp>,
) -> io::Result<()> {
    for entry_res in fs::read_dir(dir)? {
        let entry: DirEntry = entry_res?;
        let path: PathBuf = entry.path();
        if let Ok(ft) = entry.file_type() {
            if ft.is_dir() {
                let file_name: OsString = entry.file_name();
                let name: Cow<'_, str> = file_name.to_string_lossy();
                if should_skip_watch_dir(&name) {
                    continue;
                }
                if check_real_exclude && let Some(excluded) = exclude_dir {
                    if let Some(cur_canon) = canon_dir {
                        let child_canon: PathBuf = cur_canon.join(&file_name);
                        if child_canon == excluded || child_canon.starts_with(excluded) {
                            continue;
                        }
                        let recurse_check: bool = excluded.starts_with(&child_canon);
                        let next_canon: Option<&Path> =
                            recurse_check.then_some(child_canon.as_path());
                        scan_dir_stamps(
                            &path,
                            allowed_exts,
                            exclude_dir,
                            next_canon,
                            recurse_check,
                            entries,
                        )?;
                        continue;
                    }
                    let canon: PathBuf = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
                    if canon == excluded || canon.starts_with(excluded) {
                        continue;
                    }
                }
                scan_dir_stamps(
                    &path,
                    allowed_exts,
                    exclude_dir,
                    None,
                    check_real_exclude,
                    entries,
                )?;
                continue;
            }
            if ft.is_file() {
                if has_allowed_extension(&path, allowed_exts)
                    && let Ok(meta) = entry.metadata()
                {
                    let _: Option<FileStamp> = entries.insert(path, stamp_from_metadata(&meta));
                }
                continue;
            }
        }
        let Ok(meta) = fs::metadata(&path) else {
            continue;
        };
        if meta.is_dir() {
            let file_name: OsString = entry.file_name();
            let name: Cow<'_, str> = file_name.to_string_lossy();
            if should_skip_watch_dir(&name) {
                continue;
            }
            if let Some(excluded) = exclude_dir {
                let canon: PathBuf = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
                if canon == excluded || canon.starts_with(excluded) {
                    continue;
                }
                let recurse_check: bool = excluded.starts_with(&canon);
                let next_canon: Option<&Path> = recurse_check.then_some(canon.as_path());
                scan_dir_stamps(
                    &path,
                    allowed_exts,
                    exclude_dir,
                    next_canon,
                    recurse_check,
                    entries,
                )?;
                continue;
            }
            scan_dir_stamps(&path, allowed_exts, None, None, false, entries)?;
        } else if meta.is_file() && has_allowed_extension(&path, allowed_exts) {
            let _: Option<FileStamp> = entries.insert(path, stamp_from_metadata(&meta));
        }
    }
    Ok(())
}

/// Returns `true` when directory `name` should be excluded from file watching.
///
/// # Arguments
///
/// - `&str` - Directory name.
///
/// # Returns
///
/// - `bool` - Whether the watcher skips this directory.
fn should_skip_watch_dir(name: &str) -> bool {
    name.starts_with('.')
        || WATCH_IGNORED_DIRS
            .iter()
            .any(|ignored: &&str| name.eq_ignore_ascii_case(ignored))
}

/// Returns `true` when `path` has an extension listed in `allowed_exts`.
///
/// # Arguments
///
/// - `&Path` - Candidate file path.
/// - `&[&str]` - Allowed lowercase extensions.
///
/// # Returns
///
/// - `bool` - Whether the file should be watched.
fn has_allowed_extension(path: &Path, allowed_exts: &[&str]) -> bool {
    path.extension()
        .and_then(|s: &OsStr| s.to_str())
        .is_some_and(|ext: &str| {
            allowed_exts
                .iter()
                .any(|allowed: &&str| ext.eq_ignore_ascii_case(allowed))
        })
}

/// Extracts a deterministic `FileStamp` from filesystem `Metadata`.
///
/// # Arguments
///
/// - `&Metadata` - File metadata.
///
/// # Returns
///
/// - `FileStamp` - Modification timestamp and byte size pair.
fn stamp_from_metadata(meta: &Metadata) -> FileStamp {
    let nanos: u128 = meta
        .modified()
        .ok()
        .and_then(|t: SystemTime| t.duration_since(UNIX_EPOCH).ok())
        .map(|d: Duration| d.as_nanos())
        .unwrap_or(0);
    FileStamp::new(nanos, meta.len())
}
