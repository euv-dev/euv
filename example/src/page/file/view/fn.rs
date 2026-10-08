use super::*;

/// Builds the file list display node from selected file names.
///
/// # Arguments
///
/// - `UseFileUpload` - The file upload state.
///
/// # Returns
///
/// - `VirtualNode` - A list of file name items, or `VirtualNode::Empty`.
fn build_file_list(state: UseFileUpload) -> VirtualNode {
    let names: Vec<String> = state.get_file_names().get();
    if names.is_empty() {
        return VirtualNode::Empty;
    }
    let items: Vec<VirtualNode> = names
        .iter()
        .enumerate()
        .map(|tuple: (usize, &String)| {
            let (index, name) = tuple;
            let display_name: String = name.clone();
            html! {
                div {
                    class: c_file_upload_item()
                    span {
                        class: c_file_upload_item_index()
                        format!("#{}", index + 1)
                    }
                    span {
                        class: c_file_upload_item_name()
                        display_name
                    }
                }
            }
        })
        .collect();
    VirtualNode::Fragment(items)
}

/// A file upload demo page showcasing file selection and file list display.
/// # Arguments
///
/// - `VirtualNode<PageFileUploadProps>` - The component props node.
///
/// # Returns
///
/// - `VirtualNode` - The file upload demo page virtual DOM tree.
#[component]
pub(crate) fn page_file_upload(node: VirtualNode<PageFileUploadProps>) -> VirtualNode {
    let PageFileUploadProps: PageFileUploadProps = node.try_get_props().unwrap_or_default();
    let state: UseFileUpload = use_file_upload();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "📁"
                title: "File Upload"
                subtitle: "File input with configurable multiple and accept filter options. Displays selected file names and supports clearing."
            }
            euv_card {
                title: "File Input"
                input {
                    id: FILE_UPLOAD_ID
                    name: FILE_UPLOAD_NAME
                    type: FILE_INPUT_TYPE
                    autocomplete: FILE_AUTOCOMPLETE_OFF
                    class: c_file_upload_input_hidden()
                    accept: state.get_accept()
                    multiple: state.get_multiple()
                    onchange: file_upload_on_change(state)
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Browse"
                        onclick: file_upload_on_select()
                    }
                }
                div {
                    class: c_file_upload_options()
                    euv_checkbox {
                        id: FILE_MULTIPLE_ID
                        name: FILE_MULTIPLE_NAME
                        autocomplete: FILE_AUTOCOMPLETE_OFF
                        checked: state.get_multiple()
                        label: "Allow multiple files"
                    }
                    euv_field {
                        id: FILE_ACCEPT_ID
                        name: FILE_ACCEPT_NAME
                        label: "Accept filter"
                        input_type: FILE_TEXT_TYPE
                        placeholder: FILE_ACCEPT_PLACEHOLDER
                        autocomplete: FILE_AUTOCOMPLETE_OFF
                        value: state.get_accept()
                        error: None
                    }
                }
                p {
                    class: c_event_result()
                    "Status: "
                    span {
                        class: c_event_highlight()
                        state.get_status()
                    }
                }
                build_file_list(state)
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Clear"
                        onclick: file_upload_on_clear(state)
                    }
                }
            }
        }
    }
}
