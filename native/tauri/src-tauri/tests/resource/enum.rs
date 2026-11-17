#[derive(Debug)]
pub(crate) enum ResourceError {
    Fetch(String),
    Write(String),
}
