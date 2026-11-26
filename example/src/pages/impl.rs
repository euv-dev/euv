use super::*;

/// Construction helper for `ShowcasePage`.
impl ShowcasePage {
    /// Creates a typed `ShowcasePage` component bound to `route` with `PageRetention::Freeze` lifecycle semantics.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - Normalized route path.
    /// - `ShowcaseState` - Shared showcase reactive state.
    /// - `ThemeTokens` - Active design tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured lazy page component.
    #[must_use]
    pub fn new(route: &'static str, state: ShowcaseState, tokens: ThemeTokens) -> Self {
        Self {
            route,
            state,
            tokens,
        }
    }
}

/// Declarative `#[component(retention = Freeze)]` generating both `Lifecycle` (Freeze-on-leave + game pause/resume) and `Component` for every showcase route.
#[component(retention = Freeze)]
/// Inherent implementation of [`ShowcasePage`].
impl ShowcasePage {
    /// Records lazy page creation and activates embedded game engines only when their page is displayed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Page component reference.
    /// - `NodeId` - Mounted page root node handle.
    fn on_mount(&self, _node: NodeId) {
        let route: &str = self.get_route();
        let st: &ShowcaseState = self.get_state();
        let _: bool = st.get_lifecycle_mounts().update(|c: &mut u32| *c += 1);
        let _: bool = st
            .get_lifecycle_last_event()
            .set_if_changed(format!("MOUNT {route} (PageRetention::Freeze)"));
        if route == ROUTE_GAME {
            st.get_game_host().set_paused(false);
        } else if route == ROUTE_GAME_3D {
            st.get_game3d_host().set_paused(false);
        }
    }

    /// Ensures the active page's embedded game engine runs exclusively while visible.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Page component reference.
    /// - `NodeId` - Mounted page root node handle.
    fn on_activate(&self, _node: NodeId) {
        let route: &str = self.get_route();
        let st: &ShowcaseState = self.get_state();
        if route == ROUTE_GAME {
            st.get_game_host().set_paused(false);
        } else if route == ROUTE_GAME_3D {
            st.get_game3d_host().set_paused(false);
        }
    }

    /// Freezes the departed page's reactive effects and pauses embedded 2D/3D engines at zero CPU cost.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Page component reference.
    /// - `NodeId` - Frozen page root node handle.
    fn on_freeze(&self, _node: NodeId) {
        let route: &str = self.get_route();
        let st: &ShowcaseState = self.get_state();
        let _: bool = st.get_lifecycle_freezes().update(|c: &mut u32| *c += 1);
        let _: bool = st
            .get_lifecycle_last_event()
            .set_if_changed(format!("FREEZE {route} (effects & engines suspended)"));
        if route == ROUTE_GAME {
            st.get_game_host().set_paused(true);
        } else if route == ROUTE_GAME_3D {
            st.get_game3d_host().set_paused(true);
        }
    }

    /// Resumes a previously frozen page and atomically flushes coalesced cross-page signal updates.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Page component reference.
    /// - `NodeId` - Resumed page root node handle.
    fn on_resume(&self, _node: NodeId) {
        let route: &str = self.get_route();
        let st: &ShowcaseState = self.get_state();
        let _: bool = st.get_lifecycle_resumes().update(|c: &mut u32| *c += 1);
        let _: bool = st
            .get_lifecycle_last_event()
            .set_if_changed(format!("RESUME {route} (coalesced state synced)"));
        if route == ROUTE_GAME {
            st.get_game_host().set_paused(false);
        } else if route == ROUTE_GAME_3D {
            st.get_game3d_host().set_paused(false);
        }
    }

    /// Records final disposal and ensures embedded game engines are paused.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Page component reference.
    fn on_destroy(&self) {
        let route: &str = self.get_route();
        let st: &ShowcaseState = self.get_state();
        let _: bool = st.get_lifecycle_destroys().update(|c: &mut u32| *c += 1);
        if route == ROUTE_GAME {
            st.get_game_host().set_paused(true);
        } else if route == ROUTE_GAME_3D {
            st.get_game3d_host().set_paused(true);
        }
    }

    /// Renders this showcase route lazily inside the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Page component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted page node handle.
    fn render(&self) -> NodeId {
        mount_route_by_name(self.get_route(), self.get_state(), self.get_tokens())
    }
}
