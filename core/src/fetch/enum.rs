/// The method of an HTTP request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FetchMethod {
    /// The `GET` method.
    Get,
    /// The `POST` method.
    Post,
    /// The `PUT` method.
    Put,
    /// The `DELETE` method.
    Delete,
}

/// A failure of an HTTP request.
///
/// The variants name the stage the request failed at, so a caller can tell a
/// transport failure from a body the server refused.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FetchError {
    /// The request could not be started or was cut off before it completed.
    Transport(String),
    /// The response body could not be read.
    Body(String),
}
