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

async fn release_server(files: Vec<(String, Vec<u8>)>) -> (String, JoinHandle<()>) {
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
            let mut served: Vec<(String, Vec<u8>)> = resources
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
