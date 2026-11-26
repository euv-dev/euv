use super::*;

#[test]
fn cli_parses_all_subcommands_and_resolves_seven_platform_plans() {
    let parsed_build: Cli = Cli::try_parse_from([
        "euv",
        "build",
        "--target",
        "all",
        "--render",
        "hybrid",
        "--release",
        "--plan",
    ])
    .unwrap();
    let json: String = execute_cli(&parsed_build).unwrap();
    assert!(json.contains(TRIPLE_WEB));
    assert!(json.contains(TRIPLE_ANDROID));
    assert!(json.contains(TRIPLE_IOS));
    assert!(json.contains(target_triple_for_host(TargetArg::Macos)));
    assert!(json.contains(target_triple_for_host(TargetArg::Windows)));
    assert!(json.contains(target_triple_for_host(TargetArg::Linux)));
    let plans: Vec<BuildPlan> = resolve_build_plans(TargetArg::All, RenderArg::Hybrid, true);
    assert_eq!(plans.len(), 7);
    let check_steps: Vec<BuildStep> = resolve_check_steps(TargetArg::All, Some("euv-core"));
    assert_eq!(check_steps.len(), 6);
    let run_step: BuildStep = resolve_run_step(
        TargetArg::Macos,
        RenderArg::Software,
        false,
        Some("euv-example"),
    );
    assert!(
        run_step
            .get_args()
            .iter()
            .any(|a: &String| a == "--software")
    );
}

#[test]
fn cli_doctor_and_project_scaffolding_work_end_to_end() {
    let report: DoctorReport = collect_doctor_report();
    assert_eq!(report.get_targets().len(), 7);
    assert!(report.get_rustc_version().contains("rustc"));
    let formatted: String = format_doctor_report(&report);
    assert!(formatted.contains("macos"));
    let temp_root: PathBuf = env::temp_dir().join("euv_cli_scaffold_test");
    drop(fs::remove_dir_all(&temp_root));
    fs::create_dir_all(&temp_root).unwrap();
    let scaffold: ScaffoldReport = scaffold_project(&temp_root, "demo-app").unwrap();
    assert_eq!(scaffold.get_files().len(), 3);
    for file in scaffold.get_files().iter() {
        assert!(Path::new(file).exists());
    }
    let cargo_toml: String = fs::read_to_string(temp_root.join("demo-app/Cargo.toml")).unwrap();
    assert!(cargo_toml.contains(env!("CARGO_PKG_VERSION")));
    drop(fs::remove_dir_all(&temp_root));
}

#[test]
fn cli_web_bundler_emits_zero_js_files_and_valid_html_wasm_bundle() {
    let temp_dir: PathBuf = env::temp_dir().join("euv_cli_zero_js_web_test");
    drop(fs::remove_dir_all(&temp_dir));
    fs::create_dir_all(&temp_dir).unwrap();
    fs::write(temp_dir.join("app_bg.wasm"), b"\0asm\x01\0\0\0").unwrap();
    fs::write(
        temp_dir.join("app.js"),
        "export function helper() {}\nasync function __wbg_init(p) { const u = new URL(\"app_bg.wasm\", import.meta.url); return u; }\nexport { __wbg_init as default };\n",
    )
    .unwrap();
    fs::write(
        temp_dir.join("app.d.ts"),
        "export function helper(): void;\n",
    )
    .unwrap();
    fs::write(temp_dir.join("package.json"), "{}").unwrap();
    let index_path: PathBuf = package_zero_js_web_bundle(&temp_dir, "Zero <JS> App").unwrap();
    assert!(index_path.exists());
    let html: String = fs::read_to_string(&index_path).unwrap();
    assert!(html.contains("<title>Zero &lt;JS&gt; App</title>"));
    assert!(html.contains("__wbg_init({ module_or_path: './app_bg.wasm' })"));
    assert!(!html.contains("import.meta.url"));
    assert!(!html.contains("export {"));
    let mut files: Vec<String> = Vec::new();
    for entry_res in fs::read_dir(&temp_dir).unwrap() {
        let entry: DirEntry = entry_res.unwrap();
        files.push(entry.file_name().to_string_lossy().into_owned());
    }
    assert!(
        files
            .iter()
            .all(|f: &String| !f.ends_with(".js") && !f.ends_with(".d.ts"))
    );
    assert!(files.contains(&"index.html".to_string()));
    assert!(files.contains(&"app_bg.wasm".to_string()));
    assert_eq!(files.len(), 2);
    drop(fs::remove_dir_all(&temp_dir));
}

#[test]
fn cli_docs_command_builds_all_project_markdown_and_skips_ignored_dirs() {
    let base: PathBuf = env::temp_dir().join("euv_cli_docs_build_e2e");
    drop(fs::remove_dir_all(&base));
    let src: PathBuf = base.join("project");
    let out: PathBuf = base.join("out");
    fs::create_dir_all(src.join("guide")).unwrap();
    fs::create_dir_all(src.join("api")).unwrap();
    fs::create_dir_all(src.join("target")).unwrap();
    fs::create_dir_all(src.join("img")).unwrap();
    fs::write(
        src.join("README.md"),
        "---\nsite:\n  title: Acme Euv Docs\n  description: Zero-JS Rust docs\n---\n\n# Welcome\n\nRoot project docs.\n",
    )
    .unwrap();
    fs::write(
        src.join("guide/getting-started.md"),
        "# Getting Started\n\n> Install `euv` CLI first.\n\n| Command | Purpose |\n| --- | --- |\n| euv docs | Build all Markdown |\n",
    )
    .unwrap();
    fs::write(
        src.join("api/runtime.md"),
        "# Runtime API\n\n```rust\nlet sig: Signal<i32> = Signal::new(1);\n```\n",
    )
    .unwrap();
    fs::write(
        src.join("target/ignored.md"),
        "# Ignored Build Artifact\n\nMust never appear.\n",
    )
    .unwrap();
    fs::write(src.join("img/logo.svg"), "<svg></svg>").unwrap();
    let parsed: Cli = Cli::try_parse_from([
        "euv",
        "docs",
        "build",
        src.to_str().unwrap(),
        "--out",
        out.to_str().unwrap(),
    ])
    .unwrap();
    let json: String = execute_cli(&parsed).unwrap();
    assert!(json.contains("\"page_count\": 3"));
    assert!(out.join("index.html").exists());
    assert!(out.join("guide/getting-started/index.html").exists());
    assert!(out.join("api/runtime/index.html").exists());
    assert!(out.join("search-index.json").exists());
    assert!(out.join("sitemap.xml").exists());
    assert!(out.join("img/logo.svg").exists());
    assert!(!out.join("target/ignored.html").exists());
    let home_html: String = fs::read_to_string(out.join("index.html")).unwrap();
    assert!(home_html.contains("Acme Euv Docs"));
    assert!(home_html.contains("Getting Started"));
    assert!(home_html.contains("Runtime API"));
    assert!(!home_html.contains("Ignored Build Artifact"));
    assert!(!home_html.contains(HMR_ENDPOINT));
    let guide_html: String =
        fs::read_to_string(out.join("guide/getting-started/index.html")).unwrap();
    assert!(guide_html.contains("<table>"));
    assert!(guide_html.contains("<blockquote>"));
    assert!(guide_html.contains("<code>euv</code>"));
    let direct_once_cli: Cli = Cli::try_parse_from([
        "euv",
        "docs",
        src.to_str().unwrap(),
        "--out",
        out.to_str().unwrap(),
        "--watch",
        "--port",
        "0",
        "--once",
    ])
    .unwrap();
    let direct_once_json: String = execute_cli(&direct_once_cli).unwrap();
    assert!(direct_once_json.contains("\"mode\": \"docs\""));
    assert!(direct_once_json.contains("\"page_count\": 3"));
    drop(fs::remove_dir_all(&base));
}

#[test]
fn cli_dev_server_supports_live_file_watch_sse_hot_reload_and_dynamic_wasm_ota() {
    let base: PathBuf = env::temp_dir().join("euv_cli_hot_reload_e2e");
    drop(fs::remove_dir_all(&base));
    let src: PathBuf = base.join("site");
    let out: PathBuf = base.join("dist");
    fs::create_dir_all(src.join("guide")).unwrap();
    fs::write(src.join("README.md"), "# Home\n\nInitial docs home.\n").unwrap();
    let hot_md: PathBuf = src.join("guide/hot.md");
    fs::write(&hot_md, "# Hot Reload\n\nVERSION_ALPHA_MARKER\n").unwrap();
    let dev_plan_cli: Cli =
        Cli::try_parse_from(["euv", "dev", "--target", "web", "--port", "9090", "--plan"]).unwrap();
    let dev_plan_json: String = execute_cli(&dev_plan_cli).unwrap();
    assert!(dev_plan_json.contains("\"port\": 9090"));
    assert!(dev_plan_json.contains("\"enable_ota\": true"));
    let dev_no_ota_cli: Cli =
        Cli::try_parse_from(["euv", "dev", "--target", "web", "--ota=false", "--plan"]).unwrap();
    let dev_no_ota_json: String = execute_cli(&dev_no_ota_cli).unwrap();
    assert!(dev_no_ota_json.contains("\"enable_ota\": false"));
    let run_watch_cli: Cli = Cli::try_parse_from([
        "euv",
        "run",
        "--release",
        "--watch",
        "--port",
        "9091",
        "--plan",
    ])
    .unwrap();
    let run_watch_json: String = execute_cli(&run_watch_cli).unwrap();
    assert!(run_watch_json.contains("\"port\": 9091"));
    assert!(run_watch_json.contains("\"release\": true"));
    let mut cfg: DevServerConfig = DevServerConfig::default();
    cfg.set_target(DevTargetArg::Docs)
        .set_src_dir(src.clone())
        .set_out_dir(out.clone())
        .set_title(Some("Hot Reload Docs".to_string()))
        .set_base_url("/docs".to_string())
        .set_port(0)
        .set_poll_ms(60)
        .set_enable_ota(true);
    let mut handle: DevServerHandle = start_dev_server(cfg).unwrap();
    let addr: SocketAddr = handle.address();
    let options_resp: String = http_request_string(addr, "OPTIONS", "/docs/guide/hot/");
    assert!(options_resp.contains("204 No Content"));
    let head_resp: String = http_request_string(addr, "HEAD", "/docs/guide/hot/");
    assert!(head_resp.contains("200 OK"));
    assert!(!head_resp.contains("VERSION_ALPHA_MARKER"));
    let initial_page: String = http_get_string(addr, "/docs/guide/hot/");
    assert!(initial_page.contains("200 OK"));
    assert!(initial_page.contains("VERSION_ALPHA_MARKER"));
    assert!(initial_page.contains(HMR_ENDPOINT));
    let ota_manifest_v1: String = http_get_string(addr, OTA_MANIFEST_ENDPOINT);
    assert!(ota_manifest_v1.contains("\"version\": 1"));
    let ota_wasm_resp: Vec<u8> = http_get_bytes(addr, OTA_MODULE_ENDPOINT);
    let body_pos: usize = ota_wasm_resp
        .windows(4)
        .position(|w: &[u8]| w == b"\r\n\r\n")
        .unwrap()
        + 4;
    let wasm_payload: &[u8] = &ota_wasm_resp[body_pos..];
    assert!(wasm_payload.starts_with(b"\0asm\x01\0\0\0"));
    let mut sse_stream: TcpStream = TcpStream::connect(addr).unwrap();
    sse_stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    sse_stream
        .write_all(format!("GET {HMR_ENDPOINT} HTTP/1.1\r\nHost: localhost\r\n\r\n").as_bytes())
        .unwrap();
    let mut sse_reader: BufReader<TcpStream> = BufReader::new(sse_stream);
    let mut header_line: String = String::new();
    let _: usize = sse_reader.read_line(&mut header_line).unwrap();
    assert!(header_line.contains("200 OK"));
    loop {
        let mut line: String = String::new();
        let n: usize = sse_reader.read_line(&mut line).unwrap();
        if n == 0 || line.starts_with(": connected") {
            break;
        }
    }
    thread::sleep(Duration::from_millis(80));
    fs::write(&hot_md, "# Hot Reload\n\nVERSION_BETA_HOT_UPDATED\n").unwrap();
    let mut saw_reload: bool = false;
    for _ in 0..40 {
        let mut line: String = String::new();
        match sse_reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if line.contains("event: reload") {
                    saw_reload = true;
                    break;
                }
            }
            Err(_) => break,
        }
    }
    assert!(saw_reload, "expected live SSE reload event after file edit");
    assert!(handle.revision() >= 2);
    let updated_page: String = http_get_string(addr, "/docs/guide/hot/");
    assert!(updated_page.contains("VERSION_BETA_HOT_UPDATED"));
    let ota_manifest_v2: String = http_get_string(addr, OTA_MANIFEST_ENDPOINT);
    assert!(ota_manifest_v2.contains("\"version\": 2"));
    handle.shutdown();
    drop(fs::remove_dir_all(&base));
}

fn http_request_bytes(addr: SocketAddr, method: &str, path: &str) -> Vec<u8> {
    let mut stream: TcpStream = TcpStream::connect(addr).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    stream
        .write_all(
            format!("{method} {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
                .as_bytes(),
        )
        .unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let _: usize = io::Read::read_to_end(&mut stream, &mut buf).unwrap();
    buf
}

fn http_request_string(addr: SocketAddr, method: &str, path: &str) -> String {
    String::from_utf8_lossy(&http_request_bytes(addr, method, path)).into_owned()
}

fn http_get_bytes(addr: SocketAddr, path: &str) -> Vec<u8> {
    http_request_bytes(addr, "GET", path)
}

fn http_get_string(addr: SocketAddr, path: &str) -> String {
    http_request_string(addr, "GET", path)
}
