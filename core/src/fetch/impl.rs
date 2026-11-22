use super::*;

/// Inherent implementation of [`FetchMethod`].
impl FetchMethod {
    /// The method name of a request, as it goes on the wire.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The uppercase HTTP method name.
    pub fn as_str(&self) -> &'static str {
        match self {
            FetchMethod::Get => METHOD_GET,
            FetchMethod::Post => METHOD_POST,
            FetchMethod::Put => METHOD_PUT,
            FetchMethod::Delete => METHOD_DELETE,
        }
    }
}

/// Inherent implementation of [`FetchResponse`].
impl FetchResponse {
    /// The HTTP status the server answered with.
    ///
    /// # Returns
    ///
    /// - `u16` - The status code.
    pub fn status(&self) -> u16 {
        self.get_status()
    }

    /// Whether the server accepted the request.
    ///
    /// A 2xx status means the server answered with the resource the
    /// caller asked for; anything else is a refusal or a failure, and the
    /// caller decides what that means rather than having it thrown away.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the status is in the 2xx range.
    pub fn is_ok(&self) -> bool {
        (STATUS_OK_LOW..=STATUS_OK_HIGH).contains(&self.get_status())
    }

    /// The response body as raw bytes.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The bytes the server sent.
    pub fn bytes(&self) -> Vec<u8> {
        self.get_body().clone()
    }

    /// The response body as text.
    ///
    /// A body that is not valid UTF-8 has no text form, so the caller
    /// receives `None` instead of a lossy replacement character that
    /// would silently corrupt what it renders.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The body as text, or `None` when the body is
    ///   not UTF-8.
    pub fn text(&self) -> Option<String> {
        from_utf8(&self.get_body().clone()).ok().map(str::to_string)
    }
}

/// Display implementation of [`FetchError`].
///
/// The message names the stage the request failed at, so a log line
/// says whether the request never reached the server or whether the
/// answer was cut off.
impl fmt::Display for FetchError {
    /// Renders the failure.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter the message is written to.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The result of the write.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::Transport(message) => write!(formatter, "transport: {message}"),
            FetchError::Body(message) => write!(formatter, "body: {message}"),
        }
    }
}
