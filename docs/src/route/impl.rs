use super::*;

/// Hand-written accessors (§17.11) for the route layer's site view.
///
/// Named `*_of` rather than the `get_*` spelling the Data derive would
/// produce, because these structs deliberately carry plain public fields
/// rather than lombok accessors: they are inert route strings, and the
/// whole point of the view is that a test can build one by hand.
impl RoutePage {
    /// Returns the page's route.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The route this page was generated at.
    pub fn get_route(&self) -> &'static str {
        self.route
    }
}

/// Hand-written accessors (§17.11) for the route layer's site view.
impl RouteLocale {
    /// Returns the locale's route prefix.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The prefix this locale is served at.
    pub fn get_prefix(&self) -> &'static str {
        self.prefix
    }
}

/// Implementation of the route layer's site view.
impl RouteSite {
    /// Returns the locale owning `route`, falling back to the root.
    ///
    /// # Arguments
    ///
    /// - `&str` - The route path.
    ///
    /// # Returns
    ///
    /// - `&'static RouteLocale` - The owning locale, or the root locale
    ///   when the route belongs to none.
    pub fn get_locale(&self, route: &str) -> &'static RouteLocale {
        self.locale_of(route).unwrap_or(&ROUTE_LOCALE_ROOT)
    }

    /// Returns the locales the site serves.
    ///
    /// # Returns
    ///
    /// - `&'static [RouteLocale]` - The declared locales, in order.
    pub fn get_locales(&self) -> &'static [RouteLocale] {
        self.locales
    }

    /// Returns the pages the site serves.
    ///
    /// # Returns
    ///
    /// - `&'static [RoutePage]` - Every generated page route.
    pub fn get_pages(&self) -> &'static [RoutePage] {
        self.pages
    }

    /// Builds a site view from its locale prefixes and page routes.
    ///
    /// # Arguments
    ///
    /// - `&'static [RouteLocale]` - The locales the site serves.
    /// - `&'static [RoutePage]` - The pages the site serves.
    ///
    /// # Returns
    ///
    /// - `RouteSite` - The assembled site view.
    pub fn new(locales: &'static [RouteLocale], pages: &'static [RoutePage]) -> RouteSite {
        RouteSite { locales, pages }
    }

    /// Returns the locale owning `route`, by longest declared prefix.
    ///
    /// The root locale is the fallback for any route no non-root prefix
    /// claims, and the first declared locale is the last resort, so a
    /// site declaring any locale always yields one for any route.
    ///
    /// # Arguments
    ///
    /// - `&str` - The route path.
    ///
    /// # Returns
    ///
    /// - `Option<&'static RouteLocale>` - The owning locale, or `None`
    ///   only when the site declares no locale at all.
    pub fn locale_of(&self, route: &str) -> Option<&'static RouteLocale> {
        let locales: &'static [RouteLocale] = self.get_locales();
        let mut owner: Option<&'static RouteLocale> = None;
        let mut owner_len: usize = 0;
        for locale in locales {
            let prefix: &str = locale.prefix;
            if prefix == ROUTE_ROOT || !route.starts_with(prefix) || prefix.len() <= owner_len {
                continue;
            }
            owner = Some(locale);
            owner_len = prefix.len();
        }
        owner.or_else(|| {
            locales
                .iter()
                .find(|locale: &&RouteLocale| locale.get_prefix() == ROUTE_ROOT)
                .or_else(|| locales.first())
        })
    }

    /// Looks up the page addressed by `request`.
    ///
    /// A route matching a generated page verbatim always wins, so a real
    /// page is never shadowed by a normalisation spelling. Only when no
    /// page claims the request as written are the candidate spellings
    /// from [`route_candidates`] tried — which is what lets a link
    /// written without `.html`, or with a duplicated separator, reach
    /// the page it obviously meant, while still refusing to merge two
    /// pages that genuinely differ only by suffix.
    ///
    /// # Arguments
    ///
    /// - `&str` - The requested route, before or after normalisation.
    ///
    /// # Returns
    ///
    /// - `Option<&'static RoutePage>` - The addressed page, or `None`
    ///   when no spelling of the request names one.
    pub fn find_page(&self, request: &str) -> Option<&'static RoutePage> {
        let pages: &'static [RoutePage] = self.get_pages();
        let normalized: String = normalize_route(request);
        if let Some(exact) = pages
            .iter()
            .find(|page: &&RoutePage| page.get_route() == normalized)
        {
            return Some(exact);
        }
        route_candidates(&normalized)
            .into_iter()
            .find_map(|candidate: String| {
                pages
                    .iter()
                    .find(|page: &&RoutePage| page.get_route() == candidate)
            })
    }

    /// Checks every spelling variant of `route` against the site.
    ///
    /// The variants are the mistakes a link author actually makes when
    /// hand-writing or assembling a hash route: dropping the `.html`
    /// suffix, adding or dropping a trailing separator, duplicating a
    /// separator, or appending a suffix twice. Each is fed through the
    /// same normalisation and lookup path the app uses, so the verdict
    /// is the app's own rather than a restatement of it.
    ///
    /// # Arguments
    ///
    /// - `&str` - A route the site serves.
    ///
    /// # Returns
    ///
    /// - `Vec<(String, bool)>` - One `(variant, resolves)` pair per
    ///   variant, in a stable order.
    pub fn audit_route_variants(&self, route: &str) -> Vec<(String, bool)> {
        route_variants(route)
            .into_iter()
            .map(|variant: String| {
                let resolved: bool = self.find_page(&variant).is_some();
                (variant, resolved)
            })
            .collect()
    }

    /// Checks whether the route spelled without its locale prefix still
    /// resolves.
    ///
    /// This is a probe rather than a normalisation rule: the page under
    /// a stripped prefix may simply not exist, in which case the link is
    /// wrong rather than the router. Running it for every generated page
    /// is what turns "some hand-written links 404" into a concrete list
    /// of the routes to fix on the content side.
    ///
    /// # Arguments
    ///
    /// - `&str` - A route the site serves.
    /// - `&str` - The prefix of the locale that owns it.
    ///
    /// # Returns
    ///
    /// - `Option<&'static RoutePage>` - The page the stripped route
    ///   reaches, or `None` when the root locale serves nothing there.
    pub fn find_page_without_locale_prefix(
        &self,
        route: &str,
        locale_prefix: &str,
    ) -> Option<&'static RoutePage> {
        let stripped: String = route_without_locale_prefix(route, locale_prefix);
        self.find_page(&stripped)
    }

    /// Audits every served route, returning the variants that still 404.
    ///
    /// Driving the audit over the whole generated site rather than a
    /// hand-picked sample is what makes it mechanical: a route shape
    /// nobody thought to enumerate is still covered, and a regression in
    /// any locale's routes shows up without a test being written for it.
    ///
    /// # Returns
    ///
    /// - `Vec<String>` - The variants that resolved to no page, each
    ///   prefixed with the route it was derived from.
    pub fn audit_all_route_variants(&self) -> Vec<String> {
        let mut missing: Vec<String> = Vec::new();
        for page in self.get_pages() {
            let route: &str = page.get_route();
            for (variant, resolved) in self.audit_route_variants(route) {
                if !resolved {
                    missing.push(format!("{route} -> {variant}"));
                }
            }
        }
        missing
    }
}
