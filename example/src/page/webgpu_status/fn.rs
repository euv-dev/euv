use super::*;

/// Maps the WebGPU init state plus the engine's stable error code to
/// the banner text shown next to "Status: ".
///
/// This is the canonical mapping used by both the 2D and 3D WebGPU
/// demo tabs. The function is intentionally pure (no JS bridge calls,
/// no allocations) so the render path can call it every frame.
///
/// Decision tree:
///
/// - `loaded == false` - returns [`WEBGPU_STATUS_INITIALIZING`].
/// - `active == true` - returns [`WEBGPU_STATUS_ACTIVE`].
/// - `navigator.gpu` absent OR engine reports
///   [`WEBGPU_CODE_NAVIGATOR_GPU_MISSING`]: returns
///   [`WEBGPU_STATUS_NEEDS_HTTPS_OR_LOCALHOST`].
/// - adapter missing / timed out ([`WEBGPU_CODE_ADAPTER_UNAVAILABLE`]
///   or [`WEBGPU_CODE_ADAPTER_PROMISE`]): returns
///   [`WEBGPU_STATUS_ADAPTER_UNAVAILABLE`].
/// - device missing / timed out ([`WEBGPU_CODE_DEVICE_UNAVAILABLE`] or
///   [`WEBGPU_CODE_DEVICE_PROMISE`]): returns
///   [`WEBGPU_STATUS_DEVICE_UNAVAILABLE`].
/// - [`WEBGPU_CODE_CANVAS_NOT_FOUND`][] returns
///   [`WEBGPU_STATUS_CANVAS_NOT_FOUND`].
/// - [`WEBGPU_CODE_CANVAS_CONTEXT_UNAVAILABLE`][] returns
///   [`WEBGPU_STATUS_CANVAS_CONTEXT_UNAVAILABLE`].
/// - any code listed in [`WEBGPU_CODE_API_FAILURE`][] returns
///   [`WEBGPU_STATUS_INIT_FAILED_BROWSER_API`].
/// - anything else (empty or unknown code) returns
///   [`WEBGPU_STATUS_NOT_SUPPORTED`].
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `'static str` - A `'static str` value.
pub(crate) fn webgpu_status_text(
    loaded: bool,
    active: bool,
    init_error_code: &str,
) -> &'static str {
    if !loaded {
        return WEBGPU_STATUS_INITIALIZING;
    }
    if active {
        return WEBGPU_STATUS_ACTIVE;
    }
    if !WebGpuRenderer::is_available() || init_error_code == WEBGPU_CODE_NAVIGATOR_GPU_MISSING {
        return WEBGPU_STATUS_NEEDS_HTTPS_OR_LOCALHOST;
    }
    match init_error_code {
        WEBGPU_CODE_ADAPTER_UNAVAILABLE | WEBGPU_CODE_ADAPTER_PROMISE => {
            WEBGPU_STATUS_ADAPTER_UNAVAILABLE
        }
        WEBGPU_CODE_DEVICE_UNAVAILABLE | WEBGPU_CODE_DEVICE_PROMISE => {
            WEBGPU_STATUS_DEVICE_UNAVAILABLE
        }
        WEBGPU_CODE_CANVAS_NOT_FOUND => WEBGPU_STATUS_CANVAS_NOT_FOUND,
        WEBGPU_CODE_CANVAS_CONTEXT_UNAVAILABLE => WEBGPU_STATUS_CANVAS_CONTEXT_UNAVAILABLE,
        code if WEBGPU_CODE_API_FAILURE.contains(&code) => WEBGPU_STATUS_INIT_FAILED_BROWSER_API,
        _ => WEBGPU_STATUS_NOT_SUPPORTED,
    }
}
