use super::*;

/// The result of a completed HTTP request.
///
/// The body is kept as the raw bytes the server sent, and the status is the
/// one the server answered with, so a caller can decide what a non-2xx
/// answer means instead of having it thrown away.
#[derive(Clone, Debug, Eq, PartialEq, Data, New)]
pub struct FetchResponse {
    /// The HTTP status the server answered with.
    #[get(type(copy))]
    #[get(pub(crate))]
    status: u16,
    /// The response body as raw bytes.
    #[get(pub(crate))]
    body: Vec<u8>,
}
