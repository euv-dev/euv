use super::*;

fn descriptor(version: &str, files: &[(&str, &[u8])]) -> Vec<u8> {
    let entries: Vec<Value> = files.iter().map(|(path, bytes): &(&str, &[u8])| json!({"path": path, "sha256": format!("{:x}", Sha256::digest(bytes)), "size": bytes.len()})).collect();
    serde_json::to_vec(&json!({"format": 1, "version": version, "files": entries})).unwrap()
}

#[test]
fn release_inventory_rejects_downgrades_and_unsafe_paths() {
    assert_eq!(RELEASE_BASE, "https://ltpp.vip/pages/euv/");
    let bytes: Vec<u8> = descriptor("0.35.40", &[("index.html", b"old")]);
    assert!(parse_release(&bytes, "0.35.91").is_err());
    assert!(parse_release(&descriptor("0.35.93", &[("index.html", b"new")]), "0.35.94").is_err());
    for path in [
        "../x",
        "/x",
        "https://evil/x",
        "x%2fy",
        "x\\y",
        "x//y",
        "x\nx",
        "active",
        "euv-release.json",
    ] {
        assert!(!safe_release_path(path));
    }
    let duplicate: Vec<u8> = descriptor("0.35.93", &[("index.html", b"a"), ("index.html", b"b")]);
    assert!(parse_release(&duplicate, "0.35.91").is_err());
}

async fn release_server(files: ReleaseResources) -> (String, JoinHandle<()>) {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address: SocketAddr = listener.local_addr().unwrap();
    let task: JoinHandle<()> = tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let mut request: [u8; 8192] = [0; 8192];
            let count: usize = stream.read(&mut request).await.unwrap();
            let source: String = String::from_utf8_lossy(&request[..count]).into_owned();
            let path: &str = source
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .trim_start_matches('/');
            let found: Option<&(String, Vec<u8>)> = files
                .iter()
                .find(|(name, _): &&(String, Vec<u8>)| name == path);
            let (status, bytes): (&str, &[u8]) = match found {
                Some((_, bytes)) => ("200 OK", bytes),
                None => ("404 Not Found", b"missing"),
            };
            let header: String = format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                bytes.len()
            );
            stream.write_all(header.as_bytes()).await.unwrap();
            stream.write_all(bytes).await.unwrap();
        }
    });
    (format!("http://{address}/"), task)
}

#[test]
fn release_http_failures_preserve_active_and_valid_release_contains_every_theme() {
    let runtime: Runtime = Builder::new_current_thread().enable_all().build().unwrap();
    runtime.block_on(async {
        let root: PathBuf = env::temp_dir().join(format!("euv-release-regression-{}", id()));
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("active"), "v_original").unwrap();
        let resources: &[(&str, &[u8])] = &[
            ("index.html", b"root"),
            ("themes/default/index.html", b"default"),
            ("themes/theme-shadcn/index.html", b"shadcn"),
            ("themes/theme-shadcn/pkg/app.wasm", b"wasm bytes"),
            ("images/logo.svg", b"svg"),
        ];
        let client: Client = Client::new();
        let required: Vec<String> = resources
            .iter()
            .map(|(name, _): &(&str, &[u8])| name.to_string())
            .collect();
        for mode in [
            "older",
            "hash",
            "size",
            "malformed",
            "incomplete",
            "missing",
            "valid",
        ] {
            let version: &str = if mode == "older" {
                "0.35.40"
            } else {
                "0.35.93"
            };
            let mut served: ReleaseResources = resources
                .iter()
                .map(|(name, bytes): &(&str, &[u8])| (name.to_string(), bytes.to_vec()))
                .collect();
            if mode != "missing" {
                served.push((RELEASE_NAME.to_string(), descriptor(version, resources)));
            }
            if mode == "malformed" {
                served.last_mut().unwrap().1 = b"{bad json".to_vec();
            }
            if mode == "incomplete" {
                served.last_mut().unwrap().1 = descriptor(version, &resources[..1]);
            }
            if mode == "size" {
                served[2].1 = b"x".to_vec();
            }
            if mode == "hash" {
                served[2].1 = b"xxxxxx".to_vec();
            }
            let (url, task): (String, JoinHandle<()>) = release_server(served).await;
            let name: String = format!("v_{mode}");
            let staging: PathBuf = root.join(&name);
            let result: Result<(), String> =
                activate_release(&client, &url, &root, &name, "0.35.91", &required).await;
            if mode == "valid" {
                result.unwrap();
                assert_eq!(
                    verified_release(&staging, "0.35.91").await.unwrap(),
                    "0.35.93"
                );
                for (name, bytes) in resources {
                    assert_eq!(fs::read(staging.join(name)).unwrap(), *bytes);
                }
            } else {
                assert!(result.is_err(), "{mode}");
            }
            assert_eq!(
                fs::read_to_string(root.join("active")).unwrap(),
                if mode == "valid" {
                    "v_valid"
                } else {
                    "v_original"
                }
            );
            if mode != "valid" {
                assert!(!staging.exists());
            }
            task.abort();
        }
        assert!(verified_release(&root, "0.35.91").await.is_err());
        fs::remove_dir_all(root).unwrap();
    });
}

async fn counted_release_fixture(name: &str, files: ReleaseResources) -> ReleaseHttpFixture {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address: SocketAddr = listener.local_addr().unwrap();
    let root: PathBuf = env::temp_dir().join(format!("euv-release-http-{}-{name}", id()));
    fs::create_dir(&root).unwrap();
    let files: Arc<Mutex<ReleaseResources>> = Arc::new(Mutex::new(files));
    let requests: Arc<Mutex<Vec<(String, usize)>>> = Arc::new(Mutex::new(Vec::new()));
    let after_metadata: Arc<Mutex<Option<ReleaseResources>>> = Arc::new(Mutex::new(None));
    let server_files: Arc<Mutex<ReleaseResources>> = Arc::clone(&files);
    let server_requests: Arc<Mutex<Vec<(String, usize)>>> = Arc::clone(&requests);
    let server_after_metadata: Arc<Mutex<Option<ReleaseResources>>> = Arc::clone(&after_metadata);
    let task: JoinHandle<()> = tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let mut request: Vec<u8> = Vec::new();
            let mut chunk: [u8; 8192] = [0; 8192];
            while !request.windows(4).any(|bytes: &[u8]| bytes == b"\r\n\r\n") {
                let count: usize = stream.read(&mut chunk).await.unwrap();
                assert!(count > 0 && request.len() < 65536);
                request.extend_from_slice(&chunk[..count]);
            }
            let source: String = String::from_utf8(request).unwrap();
            let path: String = source.split_whitespace().nth(1).unwrap().to_string();
            let relative: &str = path.strip_prefix("/release/").unwrap_or("");
            let found: Option<Vec<u8>> = server_files
                .lock()
                .await
                .iter()
                .find(|(name, _): &&(String, Vec<u8>)| name == relative)
                .map(|(_, bytes): &(String, Vec<u8>)| bytes.clone());
            let (status, bytes): (&str, Vec<u8>) = match found {
                Some(bytes) => ("200 OK", bytes),
                None => ("404 Not Found", b"missing".to_vec()),
            };
            if relative == RELEASE_NAME
                && let Some(replacement) = server_after_metadata.lock().await.take()
            {
                *server_files.lock().await = replacement;
            }
            let header: String = format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                bytes.len()
            );
            stream.write_all(header.as_bytes()).await.unwrap();
            stream.write_all(&bytes).await.unwrap();
            server_requests.lock().await.push((path, bytes.len()));
        }
    });
    ReleaseHttpFixture {
        root,
        base: format!("http://{address}/release/"),
        files,
        requests,
        after_metadata,
        task,
    }
}

fn transfer_release_files() -> (Option<PathBuf>, ReleaseResources) {
    let distribution: Option<PathBuf> =
        env::var_os("EUV_RELEASE_REGRESSION_DIST").map(PathBuf::from);
    if let Some(directory) = &distribution {
        let bytes: Vec<u8> = fs::read(directory.join(RELEASE_NAME)).unwrap();
        let manifest: ReleaseManifest = parse_release(&bytes, "0.0.0").unwrap();
        let mut files: ReleaseResources = manifest
            .files
            .iter()
            .map(|file: &ReleaseFile| {
                let data: Vec<u8> = fs::read(directory.join(&file.path)).unwrap();
                verify_release_file(file, &data).unwrap();
                (file.path.clone(), data)
            })
            .collect();
        files.push((RELEASE_NAME.to_string(), bytes));
        return (distribution, files);
    }
    let resources: &[(&str, &[u8])] = &[
        ("index.html", b"<html>release</html>"),
        ("themes/default/index.html", b"<html>default</html>"),
        ("themes/theme-shadcn/index.html", b"<html>shadcn</html>"),
        (
            "themes/theme-shadcn/pkg/app.wasm",
            b"\0asm\x01\0\0\0\0\x02\x01x",
        ),
        ("images/logo.svg", b"<svg></svg>"),
    ];
    let mut files: ReleaseResources = resources
        .iter()
        .map(|(path, bytes): &(&str, &[u8])| (path.to_string(), bytes.to_vec()))
        .collect();
    files.push((RELEASE_NAME.to_string(), descriptor("0.36.2", resources)));
    (None, files)
}

fn replace_release_descriptor(files: &mut [(String, Vec<u8>)], version: &str) {
    let resources: Vec<(&str, &[u8])> = files
        .iter()
        .filter(|(name, _): &&(String, Vec<u8>)| name != RELEASE_NAME)
        .map(|(name, bytes): &(String, Vec<u8>)| (name.as_str(), bytes.as_slice()))
        .collect();
    let bytes: Vec<u8> = descriptor(version, &resources);
    files
        .iter_mut()
        .find(|(name, _): &&mut (String, Vec<u8>)| name == RELEASE_NAME)
        .unwrap()
        .1 = bytes;
}

async fn observed_release_poll(
    fixture: &ReleaseHttpFixture,
    client: &Client,
    name: &str,
    minimum: &str,
    required: &[String],
) -> (Result<(), String>, Value) {
    fixture.requests.lock().await.clear();
    let before: String = fs::read_to_string(fixture.root.join(RELEASE_POINTER)).unwrap_or_default();
    let outcome: Result<(), String> = activate_release(
        client,
        &fixture.base,
        &fixture.root,
        name,
        minimum,
        required,
    )
    .await;
    let after: String = fs::read_to_string(fixture.root.join(RELEASE_POINTER)).unwrap_or_default();
    let requests: Vec<(String, usize)> = fixture.requests.lock().await.clone();
    let metadata_path: String = format!("/release/{RELEASE_NAME}");
    let metadata_requests: usize = requests
        .iter()
        .filter(|(path, _): &&(String, usize)| path == &metadata_path)
        .count();
    let metadata_bytes: usize = requests
        .iter()
        .filter(|(path, _): &&(String, usize)| path == &metadata_path)
        .map(|(_, count): &(String, usize)| *count)
        .sum();
    let asset_bytes: usize = requests
        .iter()
        .filter(|(path, _): &&(String, usize)| path != &metadata_path)
        .map(|(_, count): &(String, usize)| *count)
        .sum();
    let mut entries: Vec<String> = fs::read_dir(&fixture.root)
        .unwrap()
        .map(|entry: io::Result<fs::DirEntry>| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    entries.sort();
    let observation: Value = json!({
        "attempt": name,
        "minimum": minimum,
        "pointer_before": before,
        "pointer_after": after,
        "attempt_directory_exists": fixture.root.join(name).exists(),
        "root_entries": entries,
        "metadata_requests": metadata_requests,
        "metadata_body_bytes": metadata_bytes,
        "asset_requests": requests.len() - metadata_requests,
        "asset_body_bytes": asset_bytes,
        "requests": requests.iter().map(|(path, count): &(String, usize)| json!({"path": path, "body_bytes": count})).collect::<Vec<Value>>(),
        "error": outcome.as_ref().err(),
    });
    (outcome, observation)
}

fn assert_release_transfer(observation: &Value, assets: usize, bytes: usize) {
    assert_eq!(observation["metadata_requests"], 1, "{observation}");
    assert!(observation["metadata_body_bytes"].as_u64().unwrap() > 0);
    assert_eq!(observation["asset_requests"], assets, "{observation}");
    assert_eq!(observation["asset_body_bytes"], bytes, "{observation}");
    assert_eq!(
        observation["requests"].as_array().unwrap().len(),
        assets + 1
    );
    assert_eq!(
        observation["requests"][0]["path"],
        "/release/euv-release.json"
    );
}

async fn stop_release_fixture(fixture: &mut ReleaseHttpFixture) {
    fixture.task.abort();
    assert!((&mut fixture.task).await.unwrap_err().is_cancelled());
    let address: String = Url::parse(&fixture.base)
        .unwrap()
        .socket_addrs(|| None)
        .unwrap()[0]
        .to_string();
    assert!(TcpStream::connect(&address).await.is_err());
}

#[test]
fn release_http_full_transfer_integrity_and_offline_regression() {
    let runtime: Runtime = Builder::new_current_thread().enable_all().build().unwrap();
    runtime.block_on(async {
        let (distribution, files): (Option<PathBuf>, ReleaseResources) = transfer_release_files();
        let manifest_bytes: Vec<u8> = files.iter().find(|(path, _): &&(String, Vec<u8>)| path == RELEASE_NAME).unwrap().1.clone();
        let manifest: ReleaseManifest = parse_release(&manifest_bytes, "0.0.0").unwrap();
        let asset_total: usize = manifest.files.iter().map(|file: &ReleaseFile| file.size as usize).sum();
        let required: Vec<String> = manifest.files.iter().map(|file: &ReleaseFile| file.path.clone()).collect();
        let expected_paths: HashSet<String> = manifest.files.iter().map(|file: &ReleaseFile| format!("/release/{}", file.path)).collect();
        let wasm_path: String = manifest.files.iter().find(|file: &&ReleaseFile| file.path.ends_with(".wasm")).unwrap().path.clone();
        let client: Client = Client::builder().redirect(reqwest::redirect::Policy::none()).timeout(Duration::from_secs(30)).build().unwrap();
        let mut fixture: ReleaseHttpFixture = counted_release_fixture("full", files).await;
        let mut observations: Vec<Value> = Vec::new();
        let session: CacheSession = CacheSession::default();
        for name in ["v_first", "v_repeated"] {
            let (result, observation): (Result<(), String>, Value) = observed_release_poll(&fixture, &client, name, &manifest.version, &required).await;
            result.unwrap();
            assert_release_transfer(&observation, manifest.files.len(), asset_total);
            let actual_paths: HashSet<String> = observation["requests"].as_array().unwrap().iter().skip(1).map(|entry: &Value| entry["path"].as_str().unwrap().to_string()).collect();
            assert_eq!(actual_paths, expected_paths);
            assert_eq!(observation["pointer_after"], name);
            assert!(fixture.root.join(name).is_dir());
            assert_eq!(verified_release(&fixture.root.join(name), &manifest.version).await.unwrap(), manifest.version);
            if name == "v_first" {
                assert!(session.publish(fixture.root.join(name)));
            } else {
                assert!(!session.publish(fixture.root.join(name)));
                assert_eq!(observation["pointer_before"], "v_first");
            }
            observations.push(observation);
        }
        let original_wasm: Vec<u8> = fs::read(fixture.root.join("v_repeated").join(&wasm_path)).unwrap();
        {
            let mut files: MutexGuard<'_, ReleaseResources> = fixture.files.lock().await;
            let bytes: &mut Vec<u8> = &mut files.iter_mut().find(|(path, _): &&mut (String, Vec<u8>)| path == &wasm_path).unwrap().1;
            *bytes.last_mut().unwrap() ^= 1;
        }
        let (result, observation): (Result<(), String>, Value) = observed_release_poll(&fixture, &client, "v_tampered", &manifest.version, &required).await;
        assert!(result.unwrap_err().contains("ReleaseIntegrityMismatch"));
        assert_eq!(observation["pointer_before"], "v_repeated");
        assert_eq!(observation["pointer_after"], "v_repeated");
        assert_eq!(observation["attempt_directory_exists"], false);
        assert_eq!(fs::read(fixture.root.join("v_repeated").join(&wasm_path)).unwrap(), original_wasm);
        observations.push(observation);
        {
            let mut files: MutexGuard<'_, ReleaseResources> = fixture.files.lock().await;
            replace_release_descriptor(&mut files, "0.0.0");
        }
        let (result, observation): (Result<(), String>, Value) = observed_release_poll(&fixture, &client, "v_older", &manifest.version, &required).await;
        assert_eq!(result.unwrap_err(), OLDER_RELEASE);
        assert_release_transfer(&observation, 0, 0);
        assert_eq!(observation["pointer_after"], "v_repeated");
        assert_eq!(observation["attempt_directory_exists"], false);
        observations.push(observation);
        {
            let mut files: MutexGuard<'_, ReleaseResources> = fixture.files.lock().await;
            replace_release_descriptor(&mut files, &manifest.version);
            let mut replacement: ReleaseResources = files.clone();
            *replacement.iter_mut().find(|(path, _): &&mut (String, Vec<u8>)| path == &wasm_path).unwrap().1.last_mut().unwrap() ^= 2;
            *fixture.after_metadata.lock().await = Some(replacement);
        }
        let (result, observation): (Result<(), String>, Value) = observed_release_poll(&fixture, &client, "v_origin_changed", &manifest.version, &required).await;
        assert!(result.unwrap_err().contains("ReleaseIntegrityMismatch"));
        assert_eq!(observation["pointer_after"], "v_repeated");
        assert_eq!(observation["attempt_directory_exists"], false);
        observations.push(observation);
        stop_release_fixture(&mut fixture).await;
        let (result, observation): (Result<(), String>, Value) = observed_release_poll(&fixture, &client, "v_offline", &manifest.version, &required).await;
        assert!(result.unwrap_err().contains("MetadataMissingOrUnavailable"));
        assert_eq!(observation["pointer_after"], "v_repeated");
        assert_eq!(observation["asset_requests"], 0);
        assert_eq!(observation["attempt_directory_exists"], false);
        assert_eq!(verified_release(&fixture.root.join("v_repeated"), &manifest.version).await.unwrap(), manifest.version);
        assert_eq!(session.directory().unwrap().as_ref(), fixture.root.join("v_first"));
        assert_eq!(fs::read(session.directory().unwrap().join(&wasm_path)).unwrap(), original_wasm);
        observations.push(observation);
        let evidence: Value = json!({"distribution": distribution, "manifest_sha256": format!("{:x}", Sha256::digest(&manifest_bytes)), "version": manifest.version, "asset_count": manifest.files.len(), "full_transfer_asset_bytes": asset_total, "pinned_session_preserved_offline": true, "server_stopped_and_port_closed": true, "phases": observations});
        if let Some(path) = env::var_os("EUV_RELEASE_REGRESSION_EVIDENCE") {
            fs::write(PathBuf::from(path), serde_json::to_vec_pretty(&evidence).unwrap()).unwrap();
        }
        println!("{}", serde_json::to_string(&evidence).unwrap());
    });
}

fn small_release_files() -> ReleaseResources {
    let resources: &[(&str, &[u8])] = &[
        ("index.html", b"original index"),
        ("pkg/app.wasm", b"original wasm"),
    ];
    let mut files: ReleaseResources = resources
        .iter()
        .map(|(path, bytes): &(&str, &[u8])| (path.to_string(), bytes.to_vec()))
        .collect();
    files.push((RELEASE_NAME.to_string(), descriptor("0.36.2", resources)));
    files
}

#[cfg(unix)]
#[test]
fn release_http_pointer_temporary_symlink_cannot_modify_original_snapshot() {
    let runtime: Runtime = Builder::new_current_thread().enable_all().build().unwrap();
    runtime.block_on(async {
        let mut fixture: ReleaseHttpFixture =
            counted_release_fixture("pointer_symlink", small_release_files()).await;
        let client: Client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let required: Vec<String> = vec![RELEASE_INDEX.to_string()];
        activate_release(
            &client,
            &fixture.base,
            &fixture.root,
            "v_first",
            "0.36.2",
            &required,
        )
        .await
        .unwrap();
        fixture
            .files
            .lock()
            .await
            .iter_mut()
            .find(|(path, _): &&mut (String, Vec<u8>)| path == RELEASE_NAME)
            .unwrap()
            .1
            .push(b'\n');
        let temporary: PathBuf = fixture.root.join(".release_pointer_v_blocked");
        tokio::fs::symlink(fixture.root.join("v_first/index.html"), &temporary)
            .await
            .unwrap();
        let (result, observation): (Result<(), String>, Value) =
            observed_release_poll(&fixture, &client, "v_blocked", "0.36.2", &required).await;
        assert!(result.is_err());
        assert_release_transfer(
            &observation,
            2,
            b"original index".len() + b"original wasm".len(),
        );
        assert_eq!(observation["pointer_before"], "v_first");
        assert_eq!(observation["pointer_after"], "v_first");
        assert_eq!(observation["attempt_directory_exists"], false);
        assert_eq!(
            fs::read(fixture.root.join("v_first/index.html")).unwrap(),
            b"original index"
        );
        assert!(
            fs::symlink_metadata(&temporary)
                .unwrap()
                .file_type()
                .is_symlink()
        );
        fs::remove_file(temporary).unwrap();
        verified_release(&fixture.root.join("v_first"), "0.36.2")
            .await
            .unwrap();
        stop_release_fixture(&mut fixture).await;
    });
}

async fn redirect_release_server(
    location: String,
    requests: Arc<AtomicUsize>,
) -> (String, JoinHandle<()>) {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address: SocketAddr = listener.local_addr().unwrap();
    let task: JoinHandle<()> = tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let mut request: [u8; 8192] = [0; 8192];
            assert!(stream.read(&mut request).await.unwrap() > 0);
            requests.fetch_add(1, Ordering::SeqCst);
            let header: String = format!(
                "HTTP/1.1 302 Found\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
            stream.write_all(header.as_bytes()).await.unwrap();
        }
    });
    (format!("http://{address}/"), task)
}

#[test]
fn release_http_rejects_redirects_and_unconfined_base_without_cross_origin_requests() {
    let runtime: Runtime = Builder::new_current_thread().enable_all().build().unwrap();
    runtime.block_on(async {
        let mut fixture: ReleaseHttpFixture =
            counted_release_fixture("redirects", small_release_files()).await;
        let client: Client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        let required: Vec<String> = vec![RELEASE_INDEX.to_string()];
        activate_release(
            &client,
            &fixture.base,
            &fixture.root,
            "v_first",
            "0.36.2",
            &required,
        )
        .await
        .unwrap();
        fixture.requests.lock().await.clear();
        let requests: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let (base, mut redirect): (String, JoinHandle<()>) = redirect_release_server(
            format!("{}{}", fixture.base, RELEASE_NAME),
            Arc::clone(&requests),
        )
        .await;
        let result: Result<(), String> = activate_release(
            &client,
            &base,
            &fixture.root,
            "v_redirect",
            "0.36.2",
            &required,
        )
        .await;
        assert!(result.unwrap_err().contains("ReleaseFetchRejected"));
        assert_eq!(requests.load(Ordering::SeqCst), 1);
        assert!(fixture.requests.lock().await.is_empty());
        assert!(!fixture.root.join("v_redirect").exists());
        assert_eq!(
            fs::read_to_string(fixture.root.join(RELEASE_POINTER)).unwrap(),
            "v_first"
        );
        redirect.abort();
        assert!((&mut redirect).await.unwrap_err().is_cancelled());
        assert!(
            TcpStream::connect(Url::parse(&base).unwrap().socket_addrs(|| None).unwrap()[0])
                .await
                .is_err()
        );
        assert_eq!(
            activate_release(
                &client,
                fixture.base.trim_end_matches('/'),
                &fixture.root,
                "v_unconfined",
                "0.36.2",
                &required
            )
            .await
            .unwrap_err(),
            RELEASE_ORIGIN_MISMATCH
        );
        assert!(fixture.requests.lock().await.is_empty());
        assert!(!fixture.root.join("v_unconfined").exists());
        verified_release(&fixture.root.join("v_first"), "0.36.2")
            .await
            .unwrap();
        stop_release_fixture(&mut fixture).await;
    });
}
