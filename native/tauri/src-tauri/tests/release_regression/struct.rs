use super::*;

pub(super) struct ReleaseHttpFixture {
    pub(super) root: PathBuf,
    pub(super) base: String,
    pub(super) files: Arc<Mutex<ReleaseResources>>,
    pub(super) requests: Arc<Mutex<Vec<(String, usize)>>>,
    pub(super) after_metadata: Arc<Mutex<Option<ReleaseResources>>>,
    pub(super) task: JoinHandle<()>,
}
