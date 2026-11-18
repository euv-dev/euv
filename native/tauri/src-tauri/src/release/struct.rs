use super::*;

/// Website release identity and complete immutable file inventory.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReleaseManifest {
    pub(crate) format: u32,
    pub(crate) version: String,
    pub(crate) files: Vec<ReleaseFile>,
}

/// Expected bytes for one safe relative resource path.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReleaseFile {
    pub(crate) path: String,
    pub(crate) sha256: String,
    pub(crate) size: u64,
}
