use super::*;

/// Default construction for [`CacheSession`].
impl Default for CacheSession {
    /// Creates an uninitialized cache session without a disk lookup.
    ///
    /// # Returns
    /// - `Self` - An empty notification channel and update guard.
    fn default() -> Self {
        let (ready, _): (
            Sender<Option<CacheReadiness>>,
            Receiver<Option<CacheReadiness>>,
        ) = watch::channel(None);
        Self {
            ready,
            update: Mutex::new(()),
        }
    }
}

/// Inherent implementation of [`CacheSession`].
impl CacheSession {
    /// Reads the pinned resource directory without touching the filesystem.
    ///
    /// # Returns
    /// - `Option<Arc<Path>>` - A shared reference to the complete snapshot.
    pub(crate) fn directory(&self) -> Option<Arc<Path>> {
        match self.get_ready().borrow().as_ref() {
            Some(Ok(directory)) => Some(Arc::clone(directory)),
            _ => None,
        }
    }

    /// Pins the first complete snapshot and wakes every readiness subscriber.
    ///
    /// # Arguments
    /// - `PathBuf` - A directory that already contains a complete snapshot.
    ///
    /// # Returns
    /// - `bool` - Whether this call selected the session's snapshot.
    pub(crate) fn publish(&self, directory: PathBuf) -> bool {
        self.get_ready()
            .send_if_modified(|current: &mut Option<CacheReadiness>| {
                if matches!(current, Some(Ok(_))) {
                    return false;
                }
                *current = Some(Ok(Arc::from(directory)));
                true
            })
    }

    /// Wakes readiness subscribers with an initialization error instead of hanging.
    ///
    /// # Arguments
    /// - `String` - A terminal cache-directory or snapshot-initialization error.
    ///
    /// # Returns
    /// - `bool` - Whether this call completed the readiness state.
    pub(crate) fn fail(&self, message: String) -> bool {
        self.get_ready()
            .send_if_modified(|current: &mut Option<CacheReadiness>| {
                if current.is_some() {
                    return false;
                }
                *current = Some(Err(message));
                true
            })
    }

    /// Awaits readiness without polling disk or missing an earlier publication.
    ///
    /// # Returns
    /// - `Result<Arc<Path>, String>` - The shared directory or an initialization error.
    pub(crate) async fn wait(&self) -> Result<Arc<Path>, String> {
        let mut receiver: Receiver<Option<CacheReadiness>> = self.get_ready().subscribe();
        loop {
            let readiness: Option<CacheReadiness> = receiver.borrow_and_update().clone();
            if let Some(readiness) = readiness {
                return readiness;
            }
            receiver
                .changed()
                .await
                .map_err(|error: RecvError| error.to_string())?;
        }
    }
}
