use super::*;

/// Canonical serving root initialized only in the dedicated server process.
pub(crate) static SERVING_ROOT: OnceLock<PathBuf> = OnceLock::new();
