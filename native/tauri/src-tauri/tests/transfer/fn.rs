use super::*;

fn runtime() -> Runtime {
    Builder::new_current_thread().enable_all().build().unwrap()
}

fn scratch_directory(label: &str) -> PathBuf {
    let directory: PathBuf = env::temp_dir().join(format!("euv-app-{label}-{}", id()));
    if directory.exists() {
        fs::remove_dir_all(&directory).unwrap();
    }
    fs::create_dir_all(&directory).unwrap();
    directory
}

fn temporary_file(target: &Path) -> PathBuf {
    target.with_extension(format!("{}.{}", DOWNLOAD_TEMP_SUFFIX, id()))
}

async fn receive_request(stream: &mut TcpStream) -> String {
    let mut request: Vec<u8> = Vec::new();
    let mut buffer: [u8; 1024] = [0_u8; 1024];
    loop {
        let count: usize = stream.read(&mut buffer).await.unwrap();
        if count == 0 {
            break;
        }
        request.extend_from_slice(&buffer[..count]);
        if request
            .windows(4)
            .any(|window: &[u8]| window == b"\r\n\r\n")
        {
            break;
        }
    }
    String::from_utf8_lossy(&request)
        .split_whitespace()
        .nth(1)
        .unwrap()
        .to_string()
}

async fn write_response(stream: &mut TcpStream, body: &[u8]) {
    let header: String = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes()).await.unwrap();
    stream.write_all(body).await.unwrap();
    stream.flush().await.unwrap();
}

async fn serve_timed_request(
    mut stream: TcpStream,
    active: Arc<AtomicUsize>,
    peak: Arc<AtomicUsize>,
) {
    let path: String = receive_request(&mut stream).await;
    let current: usize = active.fetch_add(1, Ordering::SeqCst) + 1;
    peak.fetch_max(current, Ordering::SeqCst);
    tokio::time::sleep(Duration::from_millis(if path == "/slow" {
        250
    } else {
        20
    }))
    .await;
    write_response(&mut stream, b"x").await;
    active.fetch_sub(1, Ordering::SeqCst);
}

async fn start_timed_server(request_count: usize) -> (String, Arc<AtomicUsize>, JoinHandle<()>) {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address: SocketAddr = listener.local_addr().unwrap();
    let active: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let peak: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let server: JoinHandle<()> = {
        let active: Arc<AtomicUsize> = Arc::clone(&active);
        let peak: Arc<AtomicUsize> = Arc::clone(&peak);
        tokio::spawn(async move {
            let mut requests: JoinSet<()> = JoinSet::new();
            for _ in 0..request_count {
                let (stream, _): (TcpStream, SocketAddr) = listener.accept().await.unwrap();
                requests.spawn(serve_timed_request(
                    stream,
                    Arc::clone(&active),
                    Arc::clone(&peak),
                ));
            }
            while let Some(result) = requests.join_next().await {
                result.unwrap();
            }
        })
    };
    (format!("http://{address}"), peak, server)
}

async fn download_in_batches(
    client: Client,
    pending: VecDeque<(String, String)>,
    directory: PathBuf,
) -> Vec<DownloadedResource> {
    let pending: Vec<(String, String)> = pending.into_iter().collect();
    let mut downloaded: Vec<DownloadedResource> = Vec::new();
    for batch in pending.chunks(DOWNLOAD_CONCURRENCY) {
        let mut tasks: JoinSet<Result<DownloadedResource, ResourceError>> = JoinSet::new();
        for (url, clean) in batch {
            let target: PathBuf = directory.join(clean);
            let client: Client = client.clone();
            let clean: String = clean.clone();
            let url: String = url.clone();
            tasks.spawn(async move {
                let (final_url, source): (String, String) =
                    download_resource(client, url, target, false).await?;
                Ok::<(String, String, String), ResourceError>((clean, final_url, source))
            });
        }
        while let Some(result) = tasks.join_next().await {
            downloaded.push(result.unwrap().unwrap());
        }
    }
    downloaded
}

async fn scheduler_fixture(batched: bool) -> (u128, usize) {
    let request_count: usize = 31;
    let (base_url, peak, server): (String, Arc<AtomicUsize>, JoinHandle<()>) =
        start_timed_server(request_count).await;
    let root: PathBuf = scratch_directory(if batched {
        "batched"
    } else {
        "work-conserving"
    });
    let mut pending: VecDeque<(String, String)> = VecDeque::new();
    pending.push_back((format!("{base_url}/slow"), "slow.bin".to_string()));
    for index in 0..request_count - 1 {
        let clean: String = format!("fast-{index}.bin");
        pending.push_back((format!("{base_url}/{clean}"), clean));
    }
    let client: Client = Client::new();
    let started: Instant = Instant::now();
    let downloaded: Vec<DownloadedResource> = if batched {
        download_in_batches(client, pending, root.clone()).await
    } else {
        download_resources(client, pending, root.clone())
            .await
            .unwrap()
    };
    let elapsed: u128 = started.elapsed().as_millis();
    server.await.unwrap();
    assert_eq!(downloaded.len(), request_count);
    assert!(peak.load(Ordering::SeqCst) <= DOWNLOAD_CONCURRENCY);
    for (clean, _, source) in downloaded {
        assert!(source.is_empty());
        assert_eq!(fs::metadata(root.join(clean)).unwrap().len(), 1);
    }
    fs::remove_dir_all(root).unwrap();
    (elapsed, peak.load(Ordering::SeqCst))
}

#[test]
fn work_conserving_scheduler_outperforms_batch_waiting() {
    runtime().block_on(async {
        let (batched_ms, batched_peak): (u128, usize) = scheduler_fixture(true).await;
        let (optimized_ms, optimized_peak): (u128, usize) = scheduler_fixture(false).await;
        println!(
            "native-scheduler-benchmark batched_ms={batched_ms} work_conserving_ms={optimized_ms} batched_peak={batched_peak} work_conserving_peak={optimized_peak}"
        );
        assert!(optimized_ms < batched_ms);
    });
}

#[test]
fn binary_response_reaches_disk_before_the_response_finishes() {
    runtime().block_on(async {
        let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url: String = format!("http://{}/payload.bin", listener.local_addr().unwrap());
        let body_size: usize = 8 * 1024 * 1024;
        let chunk_size: usize = 1024 * 1024;
        let (first_send, first_receive): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        let (release_send, release_receive): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        let server: JoinHandle<()> = tokio::spawn(async move {
            let (mut stream, _): (TcpStream, SocketAddr) = listener.accept().await.unwrap();
            receive_request(&mut stream).await;
            let header: String = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {body_size}\r\nConnection: close\r\n\r\n"
            );
            stream.write_all(header.as_bytes()).await.unwrap();
            let body: Vec<u8> = vec![b'x'; body_size];
            stream.write_all(&body[..chunk_size]).await.unwrap();
            stream.flush().await.unwrap();
            first_send.send(()).unwrap();
            release_receive.await.unwrap();
            stream.write_all(&body[chunk_size..]).await.unwrap();
        });
        let root: PathBuf = scratch_directory("streaming");
        let target: PathBuf = root.join("payload.bin");
        let temporary: PathBuf = temporary_file(&target);
        let download: JoinHandle<Result<(String, String), ResourceError>> = tokio::spawn(
            download_resource(Client::new(), url.clone(), target.clone(), false),
        );
        first_receive.await.unwrap();
        tokio::time::timeout(Duration::from_secs(2), async {
            loop {
                if fs::metadata(&temporary)
                    .is_ok_and(|metadata: fs::Metadata| metadata.len() >= chunk_size as u64)
                {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await
        .unwrap();
        assert!(!target.exists());
        release_send.send(()).unwrap();
        let (final_url, source): (String, String) = download.await.unwrap().unwrap();
        assert_eq!(final_url, url);
        assert!(source.is_empty());
        assert_eq!(fs::metadata(&target).unwrap().len(), body_size as u64);
        assert!(!temporary.exists());
        server.await.unwrap();
        fs::remove_dir_all(root).unwrap();
    });
}

#[test]
fn javascript_source_preserves_utf8_across_network_chunks() {
    runtime().block_on(async {
        let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url: String = format!("http://{}/app.js", listener.local_addr().unwrap());
        let expected: &str = "export const value = \"\u{4f60}\u{597d}\";";
        let server: JoinHandle<()> = tokio::spawn(async move {
            let (mut stream, _): (TcpStream, SocketAddr) = listener.accept().await.unwrap();
            receive_request(&mut stream).await;
            let body: &[u8] = expected.as_bytes();
            let split: usize = body.iter().position(|byte: &u8| !byte.is_ascii()).unwrap() + 1;
            let header: String = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            stream.write_all(header.as_bytes()).await.unwrap();
            stream.write_all(&body[..split]).await.unwrap();
            stream.flush().await.unwrap();
            tokio::time::sleep(Duration::from_millis(5)).await;
            stream.write_all(&body[split..]).await.unwrap();
        });
        let root: PathBuf = scratch_directory("utf8");
        let target: PathBuf = root.join("app.js");
        let (_, source): (String, String) =
            download_resource(Client::new(), url, target.clone(), true)
                .await
                .unwrap();
        assert_eq!(source, expected);
        assert_eq!(fs::read_to_string(&target).unwrap(), expected);
        server.await.unwrap();
        fs::remove_dir_all(root).unwrap();
    });
}

#[test]
fn failed_and_empty_responses_leave_no_published_file() {
    runtime().block_on(async {
        let listener: TcpListener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url: String = format!("http://{}", listener.local_addr().unwrap());
        let server: JoinHandle<()> = tokio::spawn(async move {
            for index in 0..2 {
                let (mut stream, _): (TcpStream, SocketAddr) = listener.accept().await.unwrap();
                receive_request(&mut stream).await;
                if index == 0 {
                    stream
                        .write_all(
                            b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\npartial",
                        )
                        .await
                        .unwrap();
                } else {
                    write_response(&mut stream, b"").await;
                }
            }
        });
        let root: PathBuf = scratch_directory("failed-download");
        for name in ["truncated.bin", "empty.bin"] {
            let target: PathBuf = root.join(name);
            let error: ResourceError = download_resource(
                Client::new(),
                format!("{url}/{name}"),
                target.clone(),
                false,
            )
            .await
            .unwrap_err();
            assert!(matches!(error, ResourceError::Fetch(_)));
            assert!(!target.exists());
            assert!(!temporary_file(&target).exists());
        }
        server.await.unwrap();
        fs::remove_dir_all(root).unwrap();
    });
}
