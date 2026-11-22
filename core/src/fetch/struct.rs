use super::*;

/// The result of a completed HTTP request.
///
/// The body is kept as the raw bytes the server sent, and the status is the
/// one the server answered with, so a caller can decide what a non-2xx
/// answer means instead of having it thrown away.
///
/// The fields are crate-visible and the accessors are public, so a
/// consumer crate reads the answer through `get_status()` and
/// `get_body()` and cannot freeze the field layout into the published
/// API.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct FetchResponse {
    /// The HTTP status the server answered with.
    #[get(type(copy))]
    pub(crate) status: u16,
    /// The response body as raw bytes.
    #[get(type(clone))]
    pub(crate) body: Vec<u8>,
}
