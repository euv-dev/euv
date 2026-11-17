use super::*;

pub(crate) async fn download_resource(
    client: Client,
    url: String,
    target: PathBuf,
    keep_source: bool,
) -> Result<(String, String), ResourceError> {
    let mut response: Response = client
        .get(&url)
        .send()
        .await
        .map_err(|error: Error| ResourceError::Fetch(error.to_string()))?;
    if !response.status().is_success() {
        return Err(ResourceError::Fetch(format!("HTTP {}", response.status())));
    }
    if let Some(parent) = target.parent() {
        create_dir_all(parent)
            .await
            .map_err(|error: io::Error| ResourceError::Write(error.to_string()))?;
    }
    let temporary: PathBuf = target.with_extension(format!("{}.{}", DOWNLOAD_TEMP_SUFFIX, id()));
    let mut file: File = File::create(&temporary)
        .await
        .map_err(|error: io::Error| ResourceError::Write(error.to_string()))?;
    let result: Result<(String, String), ResourceError> = async {
        let final_url: String = response.url().to_string();
        let mut source_bytes: Option<Vec<u8>> = keep_source.then(Vec::new);
        let mut received_bytes: usize = 0;
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|error: Error| ResourceError::Fetch(error.to_string()))?
        {
            received_bytes += chunk.len();
            if let Some(bytes) = source_bytes.as_mut() {
                bytes.extend_from_slice(&chunk);
            }
            file.write_all(&chunk)
                .await
                .map_err(|error: io::Error| ResourceError::Write(error.to_string()))?;
        }
        if received_bytes == 0 {
            return Err(ResourceError::Fetch(EMPTY_RESOURCE_ERROR.to_string()));
        }
        file.flush()
            .await
            .map_err(|error: io::Error| ResourceError::Write(error.to_string()))?;
        drop(file);
        rename(&temporary, &target)
            .await
            .map_err(|error: io::Error| ResourceError::Write(error.to_string()))?;
        let source: String = match source_bytes {
            Some(bytes) => match String::from_utf8(bytes) {
                Ok(text) => text,
                Err(error) => String::from_utf8_lossy(error.as_bytes()).into_owned(),
            },
            None => String::new(),
        };
        Ok((final_url, source))
    }
    .await;
    if result.is_err() {
        remove_file(&temporary).await.ok();
    }
    result
}

pub(crate) async fn download_resources(
    client: Client,
    mut pending: VecDeque<(String, String)>,
    version_dir: PathBuf,
) -> Result<Vec<DownloadedResource>, ResourceError> {
    let mut tasks: JoinSet<Result<DownloadedResource, ResourceError>> = tokio::task::JoinSet::new();
    let mut results: Vec<DownloadedResource> = Vec::with_capacity(pending.len());
    let mut failure: Option<ResourceError> = None;
    loop {
        while failure.is_none() && tasks.len() < DOWNLOAD_CONCURRENCY {
            let Some((url, clean)) = pending.pop_front() else {
                break;
            };
            let target: PathBuf = version_dir.join(&clean);
            let keep_source: bool = clean.ends_with(JS_EXTENSION)
                || clean.ends_with(MJS_EXTENSION)
                || clean.ends_with(HTML_EXTENSION);
            let client: Client = client.clone();
            tasks.spawn(async move {
                let (final_url, source): (String, String) =
                    download_resource(client, url, target, keep_source).await?;
                Ok((clean, final_url, source))
            });
        }
        let Some(result) = tasks.join_next().await else {
            break;
        };
        match result {
            Ok(Ok(resource)) => results.push(resource),
            Ok(Err(error)) => failure = Some(error),
            Err(error) => failure = Some(ResourceError::Fetch(error.to_string())),
        }
    }
    if let Some(error) = failure {
        return Err(error);
    }
    Ok(results)
}
