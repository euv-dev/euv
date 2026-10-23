use super::*;

/// Which browser storage a [`UseStorage`] handle reads and writes.
///
/// The two storages differ in lifetime, not in shape: both are plain
/// string maps on the window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UseStorageKind {
    /// `window.localStorage` — survives a reload and a browser restart.
    Local,
    /// `window.sessionStorage` — survives a reload, gone with the tab.
    Session,
}
