use super::*;

/// Metadata for persisted resources without retaining non-JavaScript bodies.
pub(crate) type FetchResult = Vec<FetchedResource>;

/// The recoverable JavaScript parser error collected during dependency discovery.
pub(crate) type ParserError = swc_ecma_parser::error::Error;
