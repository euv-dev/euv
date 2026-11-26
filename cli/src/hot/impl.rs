use super::*;

/// Default dev server configuration targeting local Web development with OTA enabled.
impl Default for DevServerConfig {
    /// Returns a sensible default `DevServerConfig` bound to `127.0.0.1:8080`.
    ///
    /// # Returns
    ///
    /// - `Self` - Default development server configuration.
    fn default() -> Self {
        Self {
            target: DevTargetArg::Web,
            render: RenderArg::Auto,
            release: false,
            src_dir: PathBuf::from("."),
            out_dir: PathBuf::from(OUT_DIR_WEB),
            title: None,
            base_url: String::new(),
            package: None,
            host: DEFAULT_DEV_HOST.to_string(),
            port: 8080,
            poll_ms: 250,
            enable_ota: true,
            once: false,
        }
    }
}

/// Change-set inspection helpers for `FileChangeSet`.
impl FileChangeSet {
    /// Returns `true` when no files were added, modified, or removed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - File change set reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the change set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.get_added().is_empty()
            && self.get_modified().is_empty()
            && self.get_removed().is_empty()
    }

    /// Returns a flattened list of every added, modified, and removed file path.
    ///
    /// # Arguments
    ///
    /// - `&Self` - File change set reference.
    ///
    /// # Returns
    ///
    /// - `Vec<String>` - Combined changed path list.
    #[must_use]
    pub fn all_paths(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::with_capacity(
            self.get_added().len() + self.get_modified().len() + self.get_removed().len(),
        );
        out.extend(self.get_added().iter().cloned());
        out.extend(self.get_modified().iter().cloned());
        out.extend(self.get_removed().iter().cloned());
        out
    }
}

/// Recursive directory scanning and deterministic diffing for `WatchSnapshot`.
impl WatchSnapshot {
    /// Scans `root` recursively collecting modification stamps for files matching `allowed_exts`.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Root project or documentation directory.
    /// - `&[&str]` - Allowed lowercase file extensions (for example `["md", "rs"]`).
    ///
    /// # Returns
    ///
    /// - `io::Result<Self>` - Collected directory snapshot or I/O error.
    pub fn scan(root: &Path, allowed_exts: &[&str]) -> io::Result<Self> {
        Self::scan_excluding(root, allowed_exts, None)
    }

    /// Scans `root` recursively while excluding `exclude_dir` so generated outputs never self-trigger rebuilds.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Root project or documentation directory.
    /// - `&[&str]` - Allowed lowercase file extensions.
    /// - `Option<&Path>` - Optional output directory excluded from watching.
    ///
    /// # Returns
    ///
    /// - `io::Result<Self>` - Collected directory snapshot or I/O error.
    pub fn scan_excluding(
        root: &Path,
        allowed_exts: &[&str],
        exclude_dir: Option<&Path>,
    ) -> io::Result<Self> {
        let mut entries: BTreeMap<PathBuf, FileStamp> = BTreeMap::new();
        let canonical_exclude: Option<PathBuf> =
            exclude_dir.map(|p: &Path| fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf()));
        if root.exists() {
            scan_dir_stamps(
                root,
                allowed_exts,
                canonical_exclude.as_deref(),
                &mut entries,
            )?;
        }
        Ok(Self { entries })
    }

    /// Computes added, modified, and removed paths comparing `self` (previous) against `next`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Previous snapshot.
    /// - `&Self` - Freshly scanned snapshot.
    ///
    /// # Returns
    ///
    /// - `FileChangeSet` - Categorized file differences.
    #[must_use]
    pub fn diff(&self, next: &Self) -> FileChangeSet {
        let mut added: Vec<String> = Vec::new();
        let mut modified: Vec<String> = Vec::new();
        let mut removed: Vec<String> = Vec::new();
        for (path, next_stamp) in next.get_entries().iter() {
            match self.get_entries().get(path) {
                None => added.push(path.display().to_string()),
                Some(prev_stamp) if prev_stamp != next_stamp => {
                    modified.push(path.display().to_string());
                }
                Some(_) => {}
            }
        }
        for path in self.get_entries().keys() {
            if !next.get_entries().contains_key(path) {
                removed.push(path.display().to_string());
            }
        }
        FileChangeSet::new(added, modified, removed)
    }
}

/// Thread-safe HMR broadcast and OTA bundle coordination for `HmrHub`.
impl HmrHub {
    /// Creates a new `HmrHub` initialized at revision `1` with `initial_ota`.
    ///
    /// # Arguments
    ///
    /// - `OtaBundleState` - Initial signed Dynamic WASM OTA artifact.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized hub ready for concurrent access.
    #[must_use]
    pub fn new(initial_ota: OtaBundleState) -> Self {
        Self {
            revision: AtomicU64::new(1),
            last_event: Mutex::new(Some(HmrEvent::new(
                1,
                HmrEventKind::Reload,
                MSG_INITIAL_BUILD_READY.to_string(),
                Vec::new(),
            ))),
            ota_state: Mutex::new(initial_ota),
            running: AtomicBool::new(true),
        }
    }

    /// Returns the current monotonic rebuild revision counter.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Current revision number.
    #[must_use]
    pub fn revision(&self) -> u64 {
        let Self { revision, .. } = self;
        revision.load(Ordering::Acquire)
    }

    /// Increments the revision counter and publishes a new `HmrEvent` to all SSE subscribers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `HmrEventKind` - Event variant (`Reload`, `OtaUpdate`, or `BuildError`).
    /// - `String` - Human-readable event description.
    /// - `Vec<String>` - List of changed source paths.
    ///
    /// # Returns
    ///
    /// - `u64` - Newly assigned revision number.
    pub fn publish(&self, kind: HmrEventKind, message: String, changed_files: Vec<String>) -> u64 {
        let Self {
            revision,
            last_event,
            ..
        } = self;
        let next_rev: u64 = revision.fetch_add(1, Ordering::AcqRel) + 1;
        let ev: HmrEvent = HmrEvent::new(next_rev, kind, message, changed_files);
        if let Ok(mut guard) = last_event.lock() {
            *guard = Some(ev);
        }
        next_rev
    }

    /// Replaces the active signed Dynamic WASM OTA bundle served over HTTP.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `OtaBundleState` - Freshly compiled and signed OTA artifact.
    pub fn set_ota_state(&self, next_state: OtaBundleState) {
        let Self { ota_state, .. } = self;
        if let Ok(mut guard) = ota_state.lock() {
            *guard = next_state;
        }
    }

    /// Returns a cloned copy of the latest published `HmrEvent`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    ///
    /// # Returns
    ///
    /// - `Option<HmrEvent>` - Latest event snapshot.
    #[must_use]
    pub fn current_event(&self) -> Option<HmrEvent> {
        let Self { last_event, .. } = self;
        last_event
            .lock()
            .ok()
            .and_then(|g: MutexGuard<'_, Option<HmrEvent>>| g.clone())
    }

    /// Returns a cloned snapshot of the active OTA manifest and `.wasm` bytes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    ///
    /// # Returns
    ///
    /// - `Option<OtaBundleState>` - Active OTA artifact if lock is unpoisoned.
    #[must_use]
    pub fn ota_snapshot(&self) -> Option<OtaBundleState> {
        let Self { ota_state, .. } = self;
        ota_state
            .lock()
            .ok()
            .map(|g: MutexGuard<'_, OtaBundleState>| g.clone())
    }

    /// Signals all background HTTP and watcher threads to exit cleanly.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    pub fn stop(&self) {
        let Self { running, .. } = self;
        running.store(false, Ordering::Release);
    }

    /// Returns `true` while the dev server has not been shut down.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Running state flag.
    #[must_use]
    pub fn is_running(&self) -> bool {
        let Self { running, .. } = self;
        running.load(Ordering::Acquire)
    }
}

/// Runtime inspection and graceful shutdown operations for `DevServerHandle`.
impl DevServerHandle {
    /// Returns the bound local TCP socket address (`ip:port`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Server handle reference.
    ///
    /// # Returns
    ///
    /// - `SocketAddr` - Bound address.
    #[must_use]
    pub fn address(&self) -> SocketAddr {
        let Self { address, .. } = self;
        *address
    }

    /// Returns the base HTTP URL for this running dev server.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Server handle reference.
    ///
    /// # Returns
    ///
    /// - `String` - Formatted `http://<ip>:<port>` URL.
    #[must_use]
    pub fn url(&self) -> String {
        format!("http://{}", self.address())
    }

    /// Returns the current live rebuild revision counter.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Server handle reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Active revision number.
    #[must_use]
    pub fn revision(&self) -> u64 {
        let Self { hub, .. } = self;
        hub.revision()
    }

    /// Builds a structured `DevServerReport` describing this active server.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Server handle reference.
    ///
    /// # Returns
    ///
    /// - `DevServerReport` - Serializable report snapshot.
    #[must_use]
    pub fn report(&self) -> DevServerReport {
        let Self {
            out_dir,
            mode,
            page_count,
            hub,
            ..
        } = self;
        let base_url: String = self.url();
        DevServerReport::new(
            mode.clone(),
            base_url.clone(),
            format!("{base_url}{HMR_ENDPOINT}"),
            format!("{base_url}{OTA_MANIFEST_ENDPOINT}"),
            out_dir.display().to_string(),
            hub.revision(),
            *page_count,
        )
    }

    /// Gracefully stops the HTTP listener and file watcher threads and joins them.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable server handle.
    pub fn shutdown(&mut self) {
        let Self {
            address,
            hub,
            http_thread,
            watch_thread,
            ..
        } = self;
        hub.stop();
        drop(TcpStream::connect(*address));
        if let Some(handle) = watch_thread.take() {
            drop(handle.join());
        }
        if let Some(handle) = http_thread.take() {
            drop(handle.join());
        }
    }
}

/// Ensures background server threads are cleanly stopped when `DevServerHandle` is dropped.
impl Drop for DevServerHandle {
    /// Signals shutdown and joins background threads on drop.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Handle being dropped.
    fn drop(&mut self) {
        self.shutdown();
    }
}
