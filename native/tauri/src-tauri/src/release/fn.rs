use super::*;

/// Converts a concrete parser, transport or filesystem failure to a diagnostic.
///
/// # Arguments
/// - `T` - The original displayable error.
///
/// # Returns
/// - `String` - The unchanged readable error message.
fn release_error<T>(error: T) -> String
where
    T: fmt::Display,
{
    error.to_string()
}

/// Parses a canonical numeric release version; ambiguous versions fail closed.
///
/// # Arguments
/// - `&str` - Canonical version identity.
///
/// # Returns
/// - `Result<Vec<u64>, String>` - Validated result, or failure without publishing an incomplete release.
pub(crate) fn release_version(value: &str) -> Result<Vec<u64>, String> {
    let parts: Vec<&str> = value.split('.').collect();
    if parts.len() != 3 {
        return Err(INVALID_RELEASE_VERSION.to_string());
    }
    for part in &parts {
        if part.is_empty() {
            return Err(INVALID_RELEASE_VERSION.to_string());
        }
        if part.len() > 1 && part.starts_with('0') {
            return Err(INVALID_RELEASE_VERSION.to_string());
        }
        if !part.bytes().all(|byte: u8| byte.is_ascii_digit()) {
            return Err(INVALID_RELEASE_VERSION.to_string());
        }
    }
    parts
        .iter()
        .map(|part: &&str| {
            part.parse::<u64>()
                .map_err(|_| INVALID_RELEASE_VERSION.to_string())
        })
        .collect()
}

/// Rejects traversal, encoded separators, native markers and URL-shaped paths.
///
/// # Arguments
/// - `&str` - Relative inventory path.
///
/// # Returns
/// - `bool` - Validated result, or failure without publishing an incomplete release.
pub(crate) fn safe_release_path(path: &str) -> bool {
    !path.is_empty()
        && !path
            .chars()
            .any(|value: char| value.is_control() || "\\:%?#".contains(value))
        && path.split('/').all(|part: &str| {
            ![
                "",
                ".",
                "..",
                RELEASE_NAME,
                "__euv_local_bundle__.json",
                "_manifest.json",
                ".empty",
                RELEASE_POINTER,
            ]
            .contains(&part)
        })
}

/// Validates descriptor bytes and the minimum acceptable release identity.
///
/// # Arguments
/// - `&[u8]` - Descriptor bytes and minimum release version.
/// - `&str` - Descriptor bytes and minimum release version.
///
/// # Returns
/// - `Result<ReleaseManifest, String>` - Validated result, or failure without publishing an incomplete release.
pub(crate) fn parse_release(bytes: &[u8], minimum: &str) -> Result<ReleaseManifest, String> {
    let manifest: ReleaseManifest = serde_json::from_slice(bytes).map_err(release_error)?;
    if manifest.format != 1 || release_version(&manifest.version)? < release_version(minimum)? {
        return Err(OLDER_RELEASE.to_string());
    }
    let mut paths: HashSet<String> = HashSet::new();
    for file in &manifest.files {
        let digest_valid: bool = file.sha256.len() == 64
            && file.sha256.bytes().all(|byte: u8| {
                byte.is_ascii_digit()
                    || (RELEASE_HEX_LETTER_START..=RELEASE_HEX_LETTER_END).contains(&byte)
            });
        if !safe_release_path(&file.path) || !paths.insert(file.path.clone()) || !digest_valid {
            return Err(INVALID_RELEASE_INVENTORY.to_string());
        }
    }
    if !manifest
        .files
        .iter()
        .any(|file: &ReleaseFile| file.path == RELEASE_INDEX && file.size > 0)
    {
        return Err(INCOMPLETE_RELEASE.to_string());
    }
    Ok(manifest)
}

/// Checks exact original bytes before they can become a native snapshot.
///
/// # Arguments
/// - `&ReleaseFile` - Advertised identity and actual resource bytes.
/// - `&[u8]` - Advertised identity and actual resource bytes.
///
/// # Returns
/// - `Result<(), String>` - Validated result, or failure without publishing an incomplete release.
pub(crate) fn verify_release_file(file: &ReleaseFile, bytes: &[u8]) -> Result<(), String> {
    let digest: String = format!("{:x}", Sha256::digest(bytes));
    if bytes.len() as u64 != file.size || digest != file.sha256 {
        return Err(format!("ReleaseIntegrityMismatch: {}", file.path));
    }
    Ok(())
}

/// Verifies a persisted snapshot, including every manifest-listed resource.
///
/// # Arguments
/// - `&Path` - Persisted snapshot directory and minimum version.
/// - `&str` - Persisted snapshot directory and minimum version.
///
/// # Returns
/// - `Result<String, String>` - Validated result, or failure without publishing an incomplete release.
pub(crate) async fn verified_release(directory: &Path, minimum: &str) -> Result<String, String> {
    for path in [directory.to_path_buf(), directory.join(RELEASE_NAME)] {
        if tokio::fs::symlink_metadata(path)
            .await
            .map_err(|_| METADATA_MISSING.to_string())?
            .file_type()
            .is_symlink()
        {
            return Err(RELEASE_SYMLINK.to_string());
        }
    }
    let bytes: Vec<u8> = read(directory.join(RELEASE_NAME))
        .await
        .map_err(|_| METADATA_MISSING.to_string())?;
    let manifest: ReleaseManifest = parse_release(&bytes, minimum)?;
    for file in &manifest.files {
        let target: PathBuf = directory.join(&file.path);
        let mut ancestor: PathBuf = directory.to_path_buf();
        for part in file.path.split('/') {
            ancestor.push(part);
            if tokio::fs::symlink_metadata(&ancestor)
                .await
                .map_err(release_error)?
                .file_type()
                .is_symlink()
            {
                return Err(RELEASE_SYMLINK.to_string());
            }
        }
        let data: Vec<u8> = read(target).await.map_err(release_error)?;
        verify_release_file(file, &data)?;
    }
    Ok(manifest.version)
}

/// Downloads exactly one URL and rejects redirects away from its identity.
///
/// # Arguments
/// - `&Client` - HTTP client and exact permitted resource URL.
/// - `Url` - HTTP client and exact permitted resource URL.
///
/// # Returns
/// - `Result<Vec<u8>, String>` - Validated result, or failure without publishing an incomplete release.
async fn release_bytes(client: &Client, url: Url) -> Result<Vec<u8>, String> {
    let response: Response = client
        .get(url.clone())
        .send()
        .await
        .map_err(release_error)?;
    if response.url() != &url || !response.status().is_success() {
        return Err(format!(
            "ReleaseFetchRejected: {url} ({})",
            response.status()
        ));
    }
    Ok(response.bytes().await.map_err(release_error)?.to_vec())
}

/// Stages all website bytes; caller alone owns the atomic active-pointer commit.
///
/// # Arguments
/// - `&Client` - Client, release base, staging directory, minimum version and required documents.
/// - `&str` - Client, release base, staging directory, minimum version and required documents.
/// - `&Path` - Client, release base, staging directory, minimum version and required documents.
/// - `&str` - Client, release base, staging directory, minimum version and required documents.
/// - `&[String]` - Client, release base, staging directory, minimum version and required documents.
///
/// # Returns
/// - `Result<String, String>` - Validated result, or failure without publishing an incomplete release.
pub(crate) async fn stage_release(
    client: &Client,
    base: &str,
    directory: &Path,
    minimum: &str,
    required: &[String],
) -> Result<String, String> {
    let base: Url = Url::parse(base).map_err(release_error)?;
    if !base.path().ends_with('/') {
        return Err(RELEASE_ORIGIN_MISMATCH.to_string());
    }
    let descriptor: Url = base.join(RELEASE_NAME).map_err(release_error)?;
    let bytes: Vec<u8> = release_bytes(client, descriptor)
        .await
        .map_err(|error: String| format!("MetadataMissingOrUnavailable: {error}"))?;
    let manifest: ReleaseManifest = parse_release(&bytes, minimum)?;
    if required.iter().any(|path: &String| {
        !manifest
            .files
            .iter()
            .any(|file: &ReleaseFile| &file.path == path)
    }) {
        return Err(INCOMPLETE_RELEASE_INVENTORY.to_string());
    }
    for file in &manifest.files {
        let url: Url = base.join(&file.path).map_err(release_error)?;
        if url.origin() != base.origin() || !url.path().starts_with(base.path()) {
            return Err(RELEASE_ORIGIN_MISMATCH.to_string());
        }
        let data: Vec<u8> = release_bytes(client, url).await?;
        verify_release_file(file, &data)?;
        let target: PathBuf = directory.join(&file.path);
        if let Some(parent) = target.parent() {
            create_dir_all(parent).await.map_err(release_error)?;
        }
        write(target, data).await.map_err(release_error)?;
    }
    write(directory.join(RELEASE_NAME), bytes)
        .await
        .map_err(release_error)?;
    Ok(manifest.version)
}
/// Commits a release pointer only after every advertised byte has been verified.
///
/// # Arguments
/// - `&Client` - Client, release base, cache root, new snapshot name, minimum version and required documents.
/// - `&str` - Client, release base, cache root, new snapshot name, minimum version and required documents.
/// - `&Path` - Client, release base, cache root, new snapshot name, minimum version and required documents.
/// - `&str` - Client, release base, cache root, new snapshot name, minimum version and required documents.
/// - `&str` - Client, release base, cache root, new snapshot name, minimum version and required documents.
/// - `&[String]` - Client, release base, cache root, new snapshot name, minimum version and required documents.
///
/// # Returns
/// - `Result<(), String>` - Validated result, or failure without publishing an incomplete release.
pub(crate) async fn activate_release(
    client: &Client,
    base: &str,
    root: &Path,
    name: &str,
    minimum: &str,
    required: &[String],
) -> Result<(), String> {
    if !name.starts_with(RELEASE_PREFIX)
        || !name
            .bytes()
            .all(|byte: u8| byte.is_ascii_alphanumeric() || byte == RELEASE_SNAPSHOT_SEPARATOR)
    {
        return Err(INVALID_SNAPSHOT_NAME.to_string());
    }
    let directory: PathBuf = root.join(name);
    tokio::fs::create_dir(&directory)
        .await
        .map_err(release_error)?;
    let temporary: PathBuf = root.join(format!(".release_pointer_{name}"));
    let mut pointer_created: bool = false;
    let outcome: Result<(), String> = async {
        stage_release(client, base, &directory, minimum, required).await?;
        verified_release(&directory, minimum).await?;
        let pointer: File = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .await
            .map_err(release_error)?;
        pointer_created = true;
        io::Write::write_all(&mut pointer.into_std().await, name.as_bytes())
            .map_err(release_error)?;
        tokio::fs::rename(&temporary, root.join(RELEASE_POINTER))
            .await
            .map_err(release_error)?;
        Ok(())
    }
    .await;
    if outcome.is_err() {
        tokio::fs::remove_dir_all(&directory).await.ok();
        if pointer_created {
            tokio::fs::remove_file(&temporary).await.ok();
        }
    }
    outcome
}
