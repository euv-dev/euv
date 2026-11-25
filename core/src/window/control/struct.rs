use super::*;

/// Thread-local command queue allowing UI handlers to issue window mutations safely.
#[derive(Clone, Data, Debug, Default)]
pub struct WindowCommandQueue {
    /// Queued commands awaiting dispatch to the active `PlatformBackend`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commands: Vec<WindowCommand>,
}
