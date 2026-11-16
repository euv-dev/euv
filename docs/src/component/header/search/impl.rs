use super::*;

/// Inherent implementation of [`HeaderSearchState`].
impl HeaderSearchState {
    /// Opens previews without implicitly selecting the first result.
    pub(crate) fn focus(&mut self) {
        self.set_open(true);
        self.set_active(None);
    }

    /// Closes previews and clears the selection on blur, outside press or Escape.
    pub(crate) fn dismiss(&mut self) {
        self.set_open(false);
        self.set_active(None);
    }

    /// Applies a keyboard intention without navigating on ordinary editing keys.
    ///
    /// # Arguments
    ///
    /// - `&str` - Unmodified keyboard key.
    /// - `bool` - Whether an IME composition is in progress.
    /// - `&[DocsPage]` - The currently previewed matches.
    ///
    /// # Returns
    ///
    /// - `(bool, Option<&'static str>)` - Whether default handling is consumed and an explicit navigation intent.
    pub(crate) fn key(
        &mut self,
        key: &str,
        composing: bool,
        results: &[DocsPage],
    ) -> (bool, Option<&'static str>) {
        if composing {
            return (false, None);
        }
        if key == HEADER_SEARCH_ESCAPE || key == HEADER_SEARCH_TAB {
            let consumed: bool = self.get_open() && key == HEADER_SEARCH_ESCAPE;
            self.dismiss();
            return (consumed, None);
        }
        if key == KEY_ENTER {
            let route: Option<&'static str> = if self.get_open() {
                self.get_active()
                    .and_then(|index: usize| results.get(index))
                    .map(|page: &DocsPage| page.route)
            } else {
                None
            };
            if route.is_some() {
                self.dismiss();
            }
            return (route.is_some(), route);
        }
        if results.is_empty() {
            return (false, None);
        }
        let active: Option<usize> = self
            .get_active()
            .filter(|index: &usize| *index < results.len());
        self.set_active(match key {
            HEADER_SEARCH_DOWN => {
                Some(active.map_or(0, |index: usize| (index + 1) % results.len()))
            }
            HEADER_SEARCH_UP => Some(active.map_or(results.len() - 1, |index: usize| {
                (index + results.len() - 1) % results.len()
            })),
            HEADER_SEARCH_HOME if self.get_open() => Some(0),
            HEADER_SEARCH_END if self.get_open() => Some(results.len() - 1),
            _ => return (false, None),
        });
        self.set_open(true);
        (true, None)
    }
}
