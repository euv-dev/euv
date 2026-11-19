/// Website-owned release descriptor name.
pub(crate) const RELEASE_NAME: &str = "euv-release.json";

/// Authoritative website update origin and deployment path.
pub(crate) const RELEASE_BASE: &str = "https://ltpp.vip/pages/euv/";
/// Release protocol constant.
pub(crate) const INVALID_RELEASE_VERSION: &str = "InvalidReleaseVersion";

/// Release protocol constant.
pub(crate) const OLDER_RELEASE: &str = "UnsupportedOrOlderRelease";

/// Release protocol constant.
pub(crate) const INVALID_RELEASE_INVENTORY: &str = "InvalidReleaseInventory";

/// Release protocol constant.
pub(crate) const INCOMPLETE_RELEASE: &str = "IncompleteRelease";

/// Release protocol constant.
pub(crate) const METADATA_MISSING: &str = "MetadataMissing";

/// Release protocol constant.
pub(crate) const RELEASE_SYMLINK: &str = "ReleaseSymlink";

/// Release protocol constant.
pub(crate) const INCOMPLETE_RELEASE_INVENTORY: &str = "IncompleteReleaseInventory";

/// Release protocol constant.
pub(crate) const RELEASE_ORIGIN_MISMATCH: &str = "ReleaseOriginMismatch";

/// Release protocol constant.
pub(crate) const INVALID_SNAPSHOT_NAME: &str = "InvalidSnapshotName";

/// Release protocol constant.
pub(crate) const RELEASE_POINTER: &str = "active";

/// Release protocol constant.
pub(crate) const RELEASE_PREFIX: &str = "v_";

/// Release protocol constant.
pub(crate) const RELEASE_INDEX: &str = "index.html";

/// First lowercase letter accepted in a SHA-256 digest.
pub(crate) const RELEASE_HEX_LETTER_START: u8 = b'a';

/// Last lowercase letter accepted in a SHA-256 digest.
pub(crate) const RELEASE_HEX_LETTER_END: u8 = b'f';

/// Separator accepted in a staged snapshot directory name.
pub(crate) const RELEASE_SNAPSHOT_SEPARATOR: u8 = b'_';
