use super::*;

/// The transfer state of one file in the [`euv_upload`] list.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvUploadStatus {
    /// Queued, not yet handed to the transport.
    #[default]
    Pending,
    /// In flight.
    Uploading,
    /// Finished successfully.
    Done,
    /// Finished with an error.
    Failed,
}
