use super::*;

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
