use super::*;

#[cfg(any(target_arch = "wasm32", test))]
/// Implements [`Drop`] for [`SheetScrollLease`].
impl Drop for SheetScrollLease {
    /// Restores only the properties owned by this lease, exactly once.
    fn drop(&mut self) {
        if let Some(cleanup) = self.get_mut_cleanup().take() {
            cleanup();
        }
    }
}
