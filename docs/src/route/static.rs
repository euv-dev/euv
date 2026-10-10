use super::*;

/// The root locale every unmatched route falls back to.
pub(crate) static ROUTE_LOCALE_ROOT: RouteLocale = RouteLocale { prefix: ROUTE_ROOT };
