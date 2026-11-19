use super::*;

impl Drop for ReleaseHttpFixture {
    fn drop(&mut self) {
        self.task.abort();
        fs::remove_dir_all(&self.root).ok();
    }
}
