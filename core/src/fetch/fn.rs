use super::*;

/// The browser's `fetch`, exposed to the framework.
///
/// A page that has to show data it did not ship — a catalog of stored
/// projects, for instance — would otherwise have to reach for a script tag.
/// The framework owns the request so the content can be rendered in Rust.
///
/// The request is a browser fetch, so it carries the page's own credentials
/// rules and the server's CORS policy applies.
///
/// # Arguments
///
/// - `&str` - The absolute URL to request.
///
/// # Returns
///
/// - `Result<FetchResponse, FetchError>` - The response, or the failure that
///   stopped it.
pub async fn fetch(url: &str) -> Result<FetchResponse, FetchError> {
    fetch_with(url, FetchMethod::Get, None, &[]).await
}

/// Performs an HTTP request and reads the whole response body.
///
/// # Arguments
///
/// - `&str` - The absolute URL to request.
/// - `FetchMethod` - The HTTP method to use.
/// - `Option<Vec<u8>>` - The request body, when the method carries one.
/// - `&[(&str, &str)]` - The request headers, as name and value pairs.
///
/// # Returns
///
/// - `Result<FetchResponse, FetchError>` - The response, or the failure that
///   stopped it.
pub async fn fetch_with(
    url: &str,
    method: FetchMethod,
    body: Option<Vec<u8>>,
    headers: &[(&str, &str)],
) -> Result<FetchResponse, FetchError> {
    let window: Window = window().ok_or_else(|| FetchError::Transport(NOWINDOW.to_string()))?;
    let builder: RequestInit = RequestInit::new();
    builder.set_method(method.as_str());
    if !headers.is_empty() {
        let init: Object = Object::new();
        for (name, value) in headers {
            Reflect::set(&init, &JsValue::from_str(name), &JsValue::from_str(value))
                .map_err(|error: JsValue| FetchError::Transport(describe(&error)))?;
        }
        builder.set_headers(&init);
    }
    if let Some(bytes) = body {
        let array: Uint8Array = Uint8Array::new_with_length(bytes.len() as u32);
        array.copy_from(&bytes);
        let payload: &JsValue = array.as_ref();
        builder.set_body(payload);
    }
    let promise: Promise = window.fetch_with_str_and_init(url, &builder);
    let response: Response = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|error: JsValue| FetchError::Transport(describe(&error)))?
        .into();
    let status: u16 = response.status();
    let pending: Promise = response
        .array_buffer()
        .map_err(|error: JsValue| FetchError::Body(describe(&error)))?;
    let buffer: ArrayBuffer = wasm_bindgen_futures::JsFuture::from(pending)
        .await
        .map_err(|error: JsValue| FetchError::Body(describe(&error)))?
        .into();
    let bytes: Vec<u8> = Uint8Array::new(&buffer).to_vec();
    Ok(FetchResponse::new(status, bytes))
}

/// Renders a JavaScript value for an error message.
///
/// The message is written for a log, so it names the exception rather than
/// carrying its payload, which can hold anything the page passed in.
///
/// # Arguments
///
/// - `&JsValue` - The value the browser raised.
///
/// # Returns
///
/// - `String` - The name of the exception.
fn describe(error: &JsValue) -> String {
    match Reflect::get(error, &JsValue::from_str(ERROR_NAME)) {
        Ok(name) if name.is_string() => name.as_string().unwrap_or_default(),
        _ => UNKNOWN_ERROR.to_string(),
    }
}
