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

/// One row of the [`euv_upload`] file list.
///
/// The view never touches the DOM `File` object: the caller turns each picked
/// file into a name / size / status triple, so the list is driven by plain
/// data and the row is reproducible from these fields alone.
#[derive(Clone, Copy, CustomDebug, Data, Default, New, PartialEq)]
pub struct EuvUploadFile {
    /// The display name of the file.
    #[get(type(copy))]
    pub name: &'static str,
    /// The pre-formatted size label (e.g. `"1.2 MB"`).
    #[get(type(copy))]
    pub size: &'static str,
    /// The transfer state driving the status badge.
    #[get(type(copy))]
    pub status: EuvUploadStatus,
}

/// Props for the [`euv_upload`] component.
///
/// The file list and the drag state are caller-owned signals so the caller can
/// feed real upload progress back in; the component only presents them.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvUploadProps {
    /// The `accept` filter applied to the underlying file input.
    pub accept: &'static str,
    /// Whether the file input allows selecting more than one file.
    pub multiple: Signal<bool>,
    /// The rows rendered in the file list.
    pub files: Signal<Vec<EuvUploadFile>>,
    /// Whether a drag is currently over the drop zone.
    pub drag_active: Signal<bool>,
    /// Optional callback receiving a display string for the picked files.
    #[debug(skip)]
    pub on_files: Option<Rc<dyn Fn(&'static str)>>,
}
