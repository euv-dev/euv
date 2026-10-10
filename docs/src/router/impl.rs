use super::*;

thread_local! {
    /// The generated site projected onto the route layer's view.
    ///
    /// The projection is built once per thread on first use and then
    /// reused, because it is pure: it copies route strings and locale
    /// prefixes out of the generated site, which never changes for the
    /// lifetime of the bundle. Caching it keeps every navigation to a
    /// single pass over the generated arrays' *shape* rather than
    /// re-allocating a projection per render.
    pub static ROUTE_SITE: RouteSite = build_route_site();
}
