use super::*;

fn directory(value: &str) -> Arc<Path> {
    Arc::from(PathBuf::from(value))
}

fn runtime() -> Runtime {
    Builder::new_current_thread().enable_time().build().unwrap()
}

#[test]
fn session_starts_without_a_directory() {
    assert!(CacheSession::default().directory().is_none());
}

#[test]
fn publication_before_wait_is_retained() {
    runtime().block_on(async {
        let session: CacheSession = CacheSession::default();
        assert!(session.publish(PathBuf::from("v_first")));
        assert_eq!(session.wait().await.unwrap(), directory("v_first"));
    });
}

#[test]
fn every_subscriber_wakes_after_publication() {
    runtime().block_on(async {
        let session: Arc<CacheSession> = Arc::new(CacheSession::default());
        let mut waiters: Vec<JoinHandle<Arc<Path>>> = Vec::new();
        for _ in 0..16 {
            let session: Arc<CacheSession> = Arc::clone(&session);
            waiters.push(tokio::spawn(async move { session.wait().await.unwrap() }));
        }
        tokio::task::yield_now().await;
        assert!(session.publish(PathBuf::from("v_complete")));
        for waiter in waiters {
            let published: Arc<Path> = tokio::time::timeout(Duration::from_secs(1), waiter)
                .await
                .unwrap()
                .unwrap();
            assert_eq!(published, directory("v_complete"));
        }
    });
}

#[test]
fn later_snapshots_do_not_change_the_session_directory() {
    runtime().block_on(async {
        let session: CacheSession = CacheSession::default();
        assert!(session.publish(PathBuf::from("v_first")));
        assert!(!session.publish(PathBuf::from("v_next")));
        assert_eq!(session.directory(), Some(directory("v_first")));
        assert_eq!(session.wait().await.unwrap(), directory("v_first"));
    });
}

#[test]
fn cancelling_a_wait_does_not_lose_future_readiness() {
    runtime().block_on(async {
        let session: CacheSession = CacheSession::default();
        assert!(
            tokio::time::timeout(Duration::from_millis(1), session.wait())
                .await
                .is_err()
        );
        assert!(session.publish(PathBuf::from("v_complete")));
        assert_eq!(session.wait().await.unwrap(), directory("v_complete"));
    });
}

#[test]
fn snapshot_updates_share_one_exclusive_guard() {
    let session: CacheSession = CacheSession::default();
    let guard: MutexGuard<'_, ()> = session.get_update().try_lock().unwrap();
    assert!(session.get_update().try_lock().is_err());
    drop(guard);
    assert!(session.get_update().try_lock().is_ok());
}

#[test]
fn initialization_failure_wakes_waiters_and_success_can_recover() {
    runtime().block_on(async {
        let session: Arc<CacheSession> = Arc::new(CacheSession::default());
        let waiting: JoinHandle<Result<Arc<Path>, String>> = {
            let session: Arc<CacheSession> = Arc::clone(&session);
            tokio::spawn(async move { session.wait().await })
        };
        tokio::task::yield_now().await;
        assert!(session.fail("cache unavailable".to_string()));
        assert_eq!(waiting.await.unwrap().unwrap_err(), "cache unavailable");
        assert!(session.publish(PathBuf::from("v_recovered")));
        assert_eq!(session.wait().await.unwrap(), directory("v_recovered"));
        assert!(!session.fail("late failure".to_string()));
        assert!(session.wait().await.is_ok());
    });
}
