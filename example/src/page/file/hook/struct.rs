use super::*;

/// Reactive state for a file upload feature.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub(crate) struct UseFileUpload {
    /// The names of selected files.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_names: Signal<Vec<String>>,
    /// The sizes of selected files in bytes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_sizes: Signal<Vec<f64>>,
    /// The MIME types of selected files.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_types: Signal<Vec<String>>,
    /// Whether multiple file selection is enabled.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) multiple: Signal<bool>,
    /// The accepted file types filter string.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) accept: Signal<String>,
    /// The status message.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) status: Signal<String>,
}
