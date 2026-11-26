use super::*;

/// Default router initialized at `"/"`.
impl Default for Router {
    /// Creates a router starting at `DEFAULT_ROUTE_PATH`.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized router.
    fn default() -> Self {
        Self::at(DEFAULT_ROUTE_PATH.to_string())
    }
}

/// Navigation history operations for `Router`.
impl Router {
    /// Creates a router initialized at `initial_path`.
    ///
    /// # Arguments
    ///
    /// - `String` - Initial route path.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed router.
    #[must_use]
    pub fn at(initial_path: String) -> Self {
        let normalized: String = if initial_path.starts_with('/') {
            initial_path
        } else {
            format!("/{initial_path}")
        };
        Self {
            current: Signal::new(normalized.clone()),
            history: vec![normalized],
            cursor: 0,
        }
    }

    /// Returns the current route path reactively.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router reference.
    ///
    /// # Returns
    ///
    /// - `String` - Active route path.
    #[must_use]
    pub fn path(&self) -> String {
        self.get_current().get()
    }

    /// Pushes a new route path onto the history stack and updates the reactive signal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable router.
    /// - `String` - Destination route path.
    pub fn push(&mut self, path: String) {
        let normalized: String = if path.starts_with('/') {
            path
        } else {
            format!("/{path}")
        };
        if self.get_current().get_untracked() == normalized {
            return;
        }
        let next_len: usize = self.get_cursor() + 1;
        self.get_mut_history().truncate(next_len);
        self.get_mut_history().push(normalized.clone());
        self.set_cursor(next_len);
        let _: bool = self.get_current().set(normalized);
    }

    /// Navigates backward in history if possible.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable router.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the route changed.
    pub fn back(&mut self) -> bool {
        if self.get_cursor() == 0 {
            return false;
        }
        let next_idx: usize = self.get_cursor() - 1;
        if let Some(target) = self.get_history().get(next_idx).cloned() {
            self.set_cursor(next_idx);
            let _: bool = self.get_current().set(target);
            true
        } else {
            false
        }
    }

    /// Navigates forward in history if possible.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable router.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the route changed.
    pub fn forward(&mut self) -> bool {
        let next_idx: usize = self.get_cursor() + 1;
        if let Some(target) = self.get_history().get(next_idx).cloned() {
            self.set_cursor(next_idx);
            let _: bool = self.get_current().set(target);
            true
        } else {
            false
        }
    }

    /// Returns whether backward navigation is available.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router reference.
    ///
    /// # Returns
    ///
    /// - `bool` - True when `cursor > 0`.
    #[must_use]
    pub fn can_go_back(&self) -> bool {
        self.get_cursor() > 0
    }
}
