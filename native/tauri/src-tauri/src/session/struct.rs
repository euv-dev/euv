use super::*;

/// Holds the immutable resource directory and serializes snapshot updates.
#[derive(Data)]
pub(crate) struct CacheSession {
    /// The first complete snapshot published during this application session.
    #[get(pub(crate))]
    pub(crate) ready: Sender<Option<CacheReadiness>>,
    /// The guard shared by initialization and background snapshot updates.
    #[get(pub(crate))]
    pub(crate) update: Mutex<()>,
}
