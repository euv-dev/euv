use super::*;

/// Builds the change handler for the hidden file input.
///
/// Mirrors the euv example's file page: the handler reads `FileList` off the
/// input and reduces it to a single display string, which is handed to
/// `on_files`. It deliberately does not build [`EuvUploadFile`] rows itself —
/// the caller owns the `files` signal and the real transfer, so the component
/// stays a presentation layer and the DOM `File` objects never leak into the
/// props type.
///
/// # Arguments
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The files callback receiving the
///   display string.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The change handler for the file input.
pub fn on_upload_files_change(
    on_files: Option<Rc<dyn Fn(&'static str)>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Some(callback) = &on_files else {
            return;
        };
        let Some(target) = event.target() else {
            return;
        };
        let Ok(input) = target.dyn_into::<HtmlInputElement>() else {
            return;
        };
        let Some(file_list) = input.files() else {
            callback(UPLOAD_NO_FILES_SELECTED_MESSAGE);
            return;
        };
        let count: u32 = file_list.length();
        if count == 0 {
            callback(UPLOAD_NO_FILES_SELECTED_MESSAGE);
            return;
        }
        let names: Vec<String> = (0..count)
            .filter_map(|index: u32| file_list.get(index).map(|file: File| file.name()))
            .collect();
        if names.is_empty() {
            callback(UPLOAD_NO_FILES_SELECTED_MESSAGE);
            return;
        }
        let summary: &'static str = Box::leak(
            format!("{} file(s) selected: {}", names.len(), names.join(", ")).into_boxed_str(),
        );
        callback(summary);
    }))
}

/// Builds the drag-over handler that marks the drop zone as active.
///
/// The drag-active state is caller-owned so the caller can also react to
/// `dragleave` / `drop` with the same signal.
///
/// # Arguments
///
/// - `Signal<bool>` - The caller-owned drag-active signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The drag-over handler.
pub fn on_upload_drag_over(drag_active: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        event.prevent_default();
        drag_active.set(true);
    }))
}

/// Builds the drag-leave handler that clears the drop zone highlight.
///
/// # Arguments
///
/// - `Signal<bool>` - The caller-owned drag-active signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The drag-leave handler.
pub fn on_upload_drag_leave(drag_active: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        event.prevent_default();
        drag_active.set(false);
    }))
}

/// Renders one file row of the upload list.
///
/// # Arguments
///
/// - `EuvUploadFile` - The file descriptor supplying name, size, and status.
///
/// # Returns
///
/// - `VirtualNode` - The rendered file row.
fn upload_file_row(file: EuvUploadFile) -> VirtualNode {
    let status_class: fn() -> &'static Css = match file.status {
        EuvUploadStatus::Pending => c_euv_upload_file_status_pending,
        EuvUploadStatus::Uploading => c_euv_upload_file_status_uploading,
        EuvUploadStatus::Done => c_euv_upload_file_status_done,
        EuvUploadStatus::Failed => c_euv_upload_file_status_failed,
    };
    let status_label: &'static str = match file.status {
        EuvUploadStatus::Pending => UPLOAD_STATUS_PENDING_LABEL,
        EuvUploadStatus::Uploading => UPLOAD_STATUS_UPLOADING_LABEL,
        EuvUploadStatus::Done => UPLOAD_STATUS_DONE_LABEL,
        EuvUploadStatus::Failed => UPLOAD_STATUS_FAILED_LABEL,
    };
    html! {
        div {
            class: c_euv_upload_file()
            key: file.name
            span {
                class: c_euv_upload_file_name()
                {
                    file.name
                }
            }
            span {
                class: c_euv_upload_file_size()
                {
                    file.size
                }
            }
            span {
                class: status_class()
                {
                    status_label
                }
            }
        }
    }
}

/// A drop zone with a hidden file input and a status-driven file list.
///
/// The real `<input type="file">` is visually hidden behind
/// `c_euv_upload_input` but stays in the accessibility tree, and the drop
/// zone is a `<label>` wrapping it — so clicking anywhere on the zone opens
/// the native picker with no programmatic-click plumbing. Drag state is a
/// reactive class swap on the zone (`c_euv_upload_drop_active`) rather than a
/// mount/unmount, so the highlight toggles without a re-layout of the input.
///
/// The rows are driven by the caller-owned `files` signal, which is what lets
/// the caller push real upload progress back in: this view only ever reads
/// the signal and maps each entry's status onto a badge class.
///
/// # Arguments
///
/// - `VirtualNode<EuvUploadProps>` - The props node containing the accept
///   filter, the multiple flag, the file rows, the drag state and the files
///   callback.
///
/// # Returns
///
/// - `VirtualNode` - The upload drop zone and file list.
#[component]
pub fn euv_upload(node: VirtualNode<EuvUploadProps>) -> VirtualNode {
    let EuvUploadProps {
        accept,
        multiple,
        files,
        drag_active,
        on_files,
    }: EuvUploadProps = node.try_get_props().unwrap_or_default();
    let file_rows: Vec<EuvUploadFile> = files.get();
    let row_nodes: Vec<VirtualNode> = file_rows
        .iter()
        .map(|file: &EuvUploadFile| upload_file_row(*file))
        .collect();
    html! {
        div {
            class: c_euv_upload()
            label {
                class: if { drag_active.get() } {
                    c_euv_upload_drop_active()
                } else {
                    c_euv_upload()
                }
                ondragover: on_upload_drag_over(drag_active)
                ondragleave: on_upload_drag_leave(drag_active)
                input {
                    type: "file"
                    class: c_euv_upload_input()
                    accept: accept
                    multiple: multiple.get()
                    onchange: on_upload_files_change(on_files)
                }
                span {
                    {
                        "Drop files here or click to browse"
                    }
                }
            }
            div {
                class: c_euv_upload_list()
                row_nodes
            }
        }
    }
}
