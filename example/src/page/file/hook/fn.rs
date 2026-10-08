use super::*;

/// Creates file upload state signals wrapped in a `UseFileUpload` struct.
///
/// # Returns
///
/// - `UseFileUpload` - The file upload state.
pub(crate) fn use_file_upload() -> UseFileUpload {
    UseFileUpload::new(
        App::use_signal(Vec::new),
        App::use_signal(Vec::new),
        App::use_signal(Vec::new),
        App::use_signal(|| false),
        App::use_signal(String::new),
        App::use_signal(|| FILE_UPLOAD_STATUS_NO_FILES.to_string()),
    )
}

/// Creates a change event handler that reads file information from a file input.
///
/// Uses `input.files()` to access the `FileList` API, which correctly returns
/// all selected files when the "Allow multiple files" option is enabled.
///
/// # Arguments
///
/// - `UseFileUpload` - The file upload state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A change handler for the file input.
pub(crate) fn file_upload_on_change(state: UseFileUpload) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            let file_list: Option<FileList> = input.files();
            let names: Vec<String>;
            let sizes: Vec<f64>;
            let types: Vec<String>;
            match file_list {
                Some(files) => {
                    let count: u32 = files.length();
                    names = (0..count)
                        .filter_map(|index: u32| files.get(index).map(|file: File| file.name()))
                        .collect();
                    sizes = (0..count)
                        .filter_map(|index: u32| files.get(index).map(|file: File| file.size()))
                        .collect();
                    types = (0..count)
                        .filter_map(|index: u32| files.get(index).map(|file: File| file.type_()))
                        .collect();
                }
                None => {
                    names = Vec::new();
                    sizes = Vec::new();
                    types = Vec::new();
                }
            }
            state.get_file_names().set(names.clone());
            state.get_file_sizes().set(sizes);
            state.get_file_types().set(types);
            if names.is_empty() {
                state
                    .get_status()
                    .set(FILE_UPLOAD_STATUS_NO_FILES.to_string());
            } else {
                let count: usize = names.len();
                state.get_status().set(format!("{count} file(s) selected"));
                Console::log(format!("Files selected: {names:?}"));
            }
        }
    }))
}

/// Creates a click event handler that programmatically triggers the hidden file input.
///
/// Defers the programmatic click via `setTimeout` to avoid recursive
/// closure invocation from the delegated event system.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler for the custom file button.
pub(crate) fn file_upload_on_select() -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let Some(window_value): Option<Window> = window() else {
            return;
        };
        let Some(document): Option<Document> = window_value.document() else {
            return;
        };
        if let Some(input) = document.get_element_by_id(FILE_UPLOAD_ID)
            && let Ok(html_input) = input.dyn_into::<HtmlInputElement>()
        {
            let html_input_clone: HtmlInputElement = html_input.clone();
            let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                html_input_clone.click();
            }));
            let Some(window): Option<Window> = window() else {
                return;
            };
            let _: Result<i32, JsValue> = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    0,
                );
            closure.forget();
        }
    }))
}

/// Creates a click event handler that clears all selected files.
///
/// # Arguments
///
/// - `UseFileUpload` - The file upload state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to clear files.
pub(crate) fn file_upload_on_clear(state: UseFileUpload) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_file_names().set(Vec::new());
        state.get_file_sizes().set(Vec::new());
        state.get_file_types().set(Vec::new());
        state
            .get_status()
            .set(FILE_UPLOAD_STATUS_NO_FILES.to_string());
    }))
}
