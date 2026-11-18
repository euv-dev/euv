use super::*;

impl fmt::Display for ResourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceError::Fetch(message) => write!(formatter, "Fetch: {message}"),
            ResourceError::Write(message) => write!(formatter, "Write: {message}"),
        }
    }
}

impl error::Error for ResourceError {}
