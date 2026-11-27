use super::*;

/// Default empty root router initialized at `"/"` for component prop initialization.
impl Default for EuvRouter {
    /// Creates a default router starting at `"/"`.
    ///
    /// # Returns
    ///
    /// - `Self` - Default router handle.
    fn default() -> Self {
        Self::new("/")
    }
}

/// History stack navigation, pattern matching, lazy page lifecycle, and system-back integration for `EuvRouter`.
impl EuvRouter {
    /// Creates a new `EuvRouter` initialized at `initial_path` with `PageRetention::Freeze` as default retention.
    ///
    /// # Arguments
    ///
    /// - `&str` - Starting route path (e.g. `"/"`).
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized reactive router handle.
    #[must_use]
    pub fn new(initial_path: &str) -> Self {
        let start: String = initial_path.to_string();
        let inner: EuvRouterInner = EuvRouterInner {
            routes: Vec::new(),
            history: vec![start.clone()],
            cursor: 0,
            dynamic_hub: None,
            default_retention: PageRetention::Freeze,
        };
        Self {
            current_path: Signal::new(start),
            can_go_back: Signal::new(false),
            can_go_forward: Signal::new(false),
            history_len: Signal::new(1),
            frozen_count: Signal::new(0),
            alive_count: Signal::new(0),
            lifecycle_entries: Signal::new(Vec::new()),
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    /// Configures the default `PageRetention` policy for routes whose components do not specify an override.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source router.
    /// - `PageRetention` - Default retention strategy (`Destroy`, `Freeze`, or `Alive`).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated router.
    #[must_use]
    pub fn with_default_retention(self, retention: PageRetention) -> Self {
        if let Ok(mut guard) = self.get_inner().try_borrow_mut() {
            guard.set_default_retention(retention);
        }
        self
    }

    /// Registers a declarative route `pattern` (supporting `:param` segments) with a lazy page builder.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source router.
    /// - `&str` - Route pattern (e.g. `"/media"` or `"/items/:id"`).
    /// - `&str` - Human-readable page title.
    /// - `F` - Lazy page builder receiving `&EuvRouteMatch`.
    ///
    /// # Returns
    ///
    /// - `Self` - Router with the route registered.
    #[must_use]
    pub fn route<F>(self, pattern: &str, title: &str, builder: F) -> Self
    where
        F: Fn(&EuvRouteMatch) -> NodeId + 'static,
    {
        if let Ok(mut guard) = self.get_inner().try_borrow_mut() {
            guard.get_mut_routes().push(EuvRouteDef {
                pattern: pattern.to_string(),
                title: title.to_string(),
                retention: None,
                builder: Rc::new(builder),
            });
        }
        self
    }

    /// Registers a declarative route with an explicit `PageRetention` policy override (`Destroy`, `Freeze`, or `Alive`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source router.
    /// - `&str` - Route pattern.
    /// - `&str` - Human-readable page title.
    /// - `PageRetention` - Explicit retention strategy when leaving this route.
    /// - `F` - Lazy page builder receiving `&EuvRouteMatch`.
    ///
    /// # Returns
    ///
    /// - `Self` - Router with the route registered.
    #[must_use]
    pub fn route_with_retention<F>(
        self,
        pattern: &str,
        title: &str,
        retention: PageRetention,
        builder: F,
    ) -> Self
    where
        F: Fn(&EuvRouteMatch) -> NodeId + 'static,
    {
        if let Ok(mut guard) = self.get_inner().try_borrow_mut() {
            guard.get_mut_routes().push(EuvRouteDef {
                pattern: pattern.to_string(),
                title: title.to_string(),
                retention: Some(retention),
                builder: Rc::new(builder),
            });
        }
        self
    }

    /// Registers a typed `Component` page factory whose `Lifecycle::retention_policy` drives freeze/destroy/alive behavior.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source router.
    /// - `&str` - Route pattern.
    /// - `&str` - Human-readable page title.
    /// - `F` - Lazy factory constructing a `Component` page instance.
    ///
    /// # Returns
    ///
    /// - `Self` - Router with the typed component route registered.
    #[must_use]
    pub fn route_component<C, F>(self, pattern: &str, title: &str, factory: F) -> Self
    where
        C: Component,
        F: Fn(&EuvRouteMatch) -> C + 'static,
    {
        self.route(pattern, title, move |matched: &EuvRouteMatch| {
            factory(matched).mount()
        })
    }

    /// Attaches an optional `DynamicUpdateHub` so OTA hot-updated routes seamlessly override local builders.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source router.
    /// - `DynamicUpdateHub` - Live dynamic update hub.
    ///
    /// # Returns
    ///
    /// - `Self` - Router connected to `hub`.
    #[must_use]
    pub fn with_dynamic_hub(self, hub: DynamicUpdateHub) -> Self {
        if let Ok(mut guard) = self.get_inner().try_borrow_mut() {
            guard.dynamic_hub = Some(hub);
        }
        self
    }

    /// Synchronizes reactive router signals after any history mutation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router reference.
    fn sync_signals(&self) {
        let snapshot: Option<(String, bool, bool, usize)> =
            self.get_inner()
                .try_borrow()
                .ok()
                .map(|guard: Ref<'_, EuvRouterInner>| {
                    let path: String = guard
                        .get_history()
                        .get(guard.get_cursor())
                        .cloned()
                        .unwrap_or_else(|| "/".to_string());
                    let len: usize = guard.get_history().len();
                    let back: bool = guard.get_cursor() > 0;
                    let fwd: bool = guard.get_cursor() + 1 < len;
                    (path, back, fwd, len)
                });
        if let Some((path, back, fwd, len)) = snapshot {
            batch(|| {
                let _: bool = self.get_current_path().set_if_changed(path);
                let _: bool = self.get_can_go_back().set_if_changed(back);
                let _: bool = self.get_can_go_forward().set_if_changed(fwd);
                let _: bool = self.get_history_len().set_if_changed(len);
            });
        }
    }

    /// Publishes live cached route lifecycle telemetry (`frozen_count`, `alive_count`, `lifecycle_entries`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router reference.
    /// - `&HashMap<String, EuvCachedPage>` - Active outlet page cache.
    fn publish_lifecycle_telemetry(&self, cache: &HashMap<String, EuvCachedPage>) {
        let mut frozen: usize = 0;
        let mut alive: usize = 0;
        let mut entries: Vec<EuvRouteLifecycleEntry> = Vec::with_capacity(cache.len());
        for (path, entry) in cache.iter() {
            if entry.phase == LifecyclePhase::Frozen {
                frozen += 1;
            } else if entry.phase == LifecyclePhase::Mounted
                && entry.retention == PageRetention::Alive
            {
                alive += 1;
            }
            entries.push(EuvRouteLifecycleEntry {
                path: path.clone(),
                retention: entry.retention,
                phase: entry.phase,
                activations: entry.activations,
            });
        }
        entries.sort_by(|a: &EuvRouteLifecycleEntry, b: &EuvRouteLifecycleEntry| {
            a.get_path().cmp(b.get_path())
        });
        let frozen_sig: Signal<usize> = self.get_frozen_count();
        let alive_sig: Signal<usize> = self.get_alive_count();
        let entries_sig: Signal<Vec<EuvRouteLifecycleEntry>> = self.get_lifecycle_entries();
        batch(|| {
            let _: bool = frozen_sig.set_if_changed(frozen);
            let _: bool = alive_sig.set_if_changed(alive);
            let _: bool = entries_sig.set_if_changed(entries);
        });
    }

    /// Pushes `path` onto the navigation history stack (no-op if already on `path`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router reference.
    /// - `&str` - Destination route path.
    pub fn push(&self, path: &str) {
        let mut changed: bool = false;
        if let Ok(mut guard) = self.get_inner().try_borrow_mut() {
            let cur: String = guard
                .get_history()
                .get(guard.get_cursor())
                .cloned()
                .unwrap_or_default();
            if cur != path {
                let next_idx: usize = guard.get_cursor() + 1;
                guard.get_mut_history().truncate(next_idx);
                guard.get_mut_history().push(path.to_string());
                guard.set_cursor(next_idx);
                changed = true;
            }
        }
        if changed {
            self.sync_signals();
        }
    }

    /// Replaces the current history entry with `path` without growing stack depth.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router reference.
    /// - `&str` - Replacement route path.
    pub fn replace(&self, path: &str) {
        if let Ok(mut guard) = self.get_inner().try_borrow_mut() {
            let idx: usize = guard.get_cursor();
            if let Some(slot) = guard.get_mut_history().get_mut(idx) {
                *slot = path.to_string();
            } else {
                guard.get_mut_history().push(path.to_string());
                guard.set_cursor(0);
            }
        }
        self.sync_signals();
    }

    /// Navigates one step backward in history if available.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if navigated back; `false` if already at root.
    pub fn back(&self) -> bool {
        let moved: bool = if let Ok(mut guard) = self.get_inner().try_borrow_mut() {
            if guard.get_cursor() > 0 {
                let prev: usize = guard.get_cursor() - 1;
                guard.set_cursor(prev);
                true
            } else {
                false
            }
        } else {
            false
        };
        if moved {
            self.sync_signals();
        }
        moved
    }

    /// Navigates one step forward in history if available.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if navigated forward; `false` if at head of stack.
    pub fn forward(&self) -> bool {
        let moved: bool = if let Ok(mut guard) = self.get_inner().try_borrow_mut() {
            if guard.get_cursor() + 1 < guard.get_history().len() {
                let next: usize = guard.get_cursor() + 1;
                guard.set_cursor(next);
                true
            } else {
                false
            }
        } else {
            false
        };
        if moved {
            self.sync_signals();
        }
        moved
    }

    /// Unified OS Back handler: closes topmost overlay in `EuvOverlayStack` first, then pops router history.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when consumed by an overlay or route pop; `false` when at app root.
    pub fn handle_system_back(&self) -> bool {
        if EuvOverlayStack::dismiss_top() {
            return true;
        }
        self.back()
    }

    /// Parses and matches `raw_path` against registered routes, extracting `:param`, `?query`, and optional retention override.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router reference.
    /// - `&str` - Raw path possibly containing `?k=v&k2=v2`.
    ///
    /// # Returns
    ///
    /// - `EuvRouteResolvePolicy` - Match metadata, optional builder, and optional retention policy.
    #[must_use]
    pub fn resolve_with_policy(&self, raw_path: &str) -> EuvRouteResolvePolicy {
        let (clean_path, query_part): (&str, &str) = match raw_path.split_once('?') {
            Some((p, q)) => (p, q),
            None => (raw_path, ""),
        };
        let mut query: HashMap<String, String> = HashMap::new();
        for pair in query_part.split('&') {
            if let Some((k, v)) = pair.split_once('=')
                && !k.is_empty()
            {
                let _: Option<String> = query.insert(k.to_string(), v.to_string());
            }
        }
        let path_segs: Vec<&str> = clean_path
            .split('/')
            .filter(|s: &&str| !s.is_empty())
            .collect();
        if let Ok(guard) = self.get_inner().try_borrow() {
            for def in guard.get_routes().iter() {
                let pat_segs: Vec<&str> = def
                    .get_pattern()
                    .split('/')
                    .filter(|s: &&str| !s.is_empty())
                    .collect();
                if pat_segs.len() != path_segs.len() {
                    continue;
                }
                let mut params: HashMap<String, String> = HashMap::new();
                let mut matched: bool = true;
                for (pat, seg) in pat_segs.iter().zip(path_segs.iter()) {
                    if let Some(key) = pat.strip_prefix(':') {
                        let _: Option<String> = params.insert(key.to_string(), (*seg).to_string());
                    } else if pat != seg {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    let EuvRouteDef {
                        builder, retention, ..
                    } = def;
                    return (
                        EuvRouteMatch {
                            path: clean_path.to_string(),
                            pattern: def.get_pattern().clone(),
                            title: def.get_title().clone(),
                            params,
                            query,
                        },
                        Some(Rc::clone(builder)),
                        *retention,
                    );
                }
            }
        }
        (
            EuvRouteMatch {
                path: clean_path.to_string(),
                pattern: String::new(),
                title: String::new(),
                params: HashMap::new(),
                query,
            },
            None,
            None,
        )
    }

    /// Parses and matches `raw_path` against registered routes, extracting `:param` and `?query` values.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router reference.
    /// - `&str` - Raw path possibly containing `?k=v&k2=v2`.
    ///
    /// # Returns
    ///
    /// - `(EuvRouteMatch, Option<EuvRouteBuilder>)` - Parsed match metadata and matched builder if found.
    #[must_use]
    pub fn resolve(&self, raw_path: &str) -> (EuvRouteMatch, Option<EuvRouteBuilder>) {
        let (matched, builder, _): (
            EuvRouteMatch,
            Option<EuvRouteBuilder>,
            Option<PageRetention>,
        ) = self.resolve_with_policy(raw_path);
        (matched, builder)
    }

    /// Creates a reactive `EuvRouterOutlet` bound to this router with a fallback builder.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router reference.
    /// - `F` - Fallback page builder for unmatched paths.
    ///
    /// # Returns
    ///
    /// - `EuvRouterOutlet` - Component-backed reactive router outlet.
    #[must_use]
    pub fn outlet<F>(&self, fallback: F) -> EuvRouterOutlet
    where
        F: Fn(&EuvRouteMatch) -> NodeId + 'static,
    {
        EuvRouterOutlet {
            router: self.clone(),
            fallback: Rc::new(fallback),
            theme: None,
            double_back_exit: false,
            exit_prompt: DOUBLE_BACK_EXIT_PROMPT.to_string(),
            route_order: Vec::new(),
            page_padding: None,
        }
    }
}

/// Builder modifiers and internal helpers for `EuvRouterOutlet`.
impl EuvRouterOutlet {
    /// Connects a reactive `UiThemeState` so pages restyle in-place without teardown or scroll loss.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source outlet.
    /// - `UiThemeState` - Reactive theme state.
    ///
    /// # Returns
    ///
    /// - `Self` - Theme-aware router outlet.
    #[must_use]
    pub fn with_theme(mut self, theme: UiThemeState) -> Self {
        self.set_theme(Some(theme));
        self
    }

    /// Enables standard app double-back-to-exit behavior at the root route when no overlay or previous route remains.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source outlet.
    /// - `bool` - Whether pressing Back twice consecutively at the root route shows a bottom prompt then exits the app.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated router outlet.
    #[must_use]
    pub fn with_double_back_exit(mut self, enabled: bool) -> Self {
        self.set_double_back_exit(enabled);
        self
    }

    /// Customizes the bottom prompt text displayed on the first Back press at the root route.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source outlet.
    /// - `String` - Prompt copy displayed at the bottom of the app.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated router outlet.
    #[must_use]
    pub fn with_exit_prompt(mut self, prompt: String) -> Self {
        self.set_exit_prompt(prompt);
        self
    }

    /// Sets an ordered list of route paths so horizontal left/right page transitions follow tab and swipe order 1:1.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source outlet.
    /// - `Vec<String>` - Ordered route path list.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated router outlet.
    #[must_use]
    pub fn with_route_order(mut self, routes: Vec<String>) -> Self {
        self.set_route_order(routes);
        self
    }

    /// Binds a reactive `Edges` padding signal applied to every per-page `ScrollView` wrapper.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source outlet.
    /// - `Signal<Edges>` - Reactive inner padding signal synchronized with the hosting shell.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated router outlet.
    #[must_use]
    pub fn with_page_padding(mut self, padding: Signal<Edges>) -> Self {
        self.set_page_padding(Some(padding));
        self
    }

    /// Resolves the live logical width of the outlet viewport for exact 1:1 edge-to-edge slide coupling.
    ///
    /// # Arguments
    ///
    /// - `&SceneHandle` - Active scene handle.
    /// - `NodeId` - Outlet host container handle.
    /// - `Signal<Size>` - Viewport size signal fallback.
    ///
    /// # Returns
    ///
    /// - `f32` - Exact horizontal slide span in logical pixels (`>= 320.0`).
    fn resolve_slide_width(scene: &SceneHandle, host: NodeId, vp_sig: Signal<Size>) -> f32 {
        scene
            .node_screen_bounds(host)
            .map(|rect: Rect| rect.get_width())
            .filter(|w: &f32| *w > 40.0)
            .unwrap_or_else(|| vp_sig.get_untracked().get_width().max(320.0))
    }

    /// Builds a new cached page record with its own `ScrollView` wrapper and reactive ownership `Scope`.
    ///
    /// # Arguments
    ///
    /// - `&EuvOutletMountEnv` - Shared outlet mounting environment.
    /// - `&str` - Target route path to mount.
    /// - `Option<ThemeTokens>` - Active theme tokens.
    /// - `u64` - Active dynamic revision.
    /// - `Edges` - Current per-page scroll wrapper padding.
    /// - `bool` - When `true`, mounts hidden/frozen as an adjacent drag preview without counting as active yet.
    ///
    /// # Returns
    ///
    /// - `EuvCachedPage` - Newly mounted cached page record.
    fn mount_cached_page(
        env: &EuvOutletMountEnv,
        target_path: &str,
        cur_tokens: Option<ThemeTokens>,
        cur_dyn_rev: u64,
        page_pad: Edges,
        as_preview: bool,
    ) -> EuvCachedPage {
        let (matched, builder_opt, route_ret): (
            EuvRouteMatch,
            Option<EuvRouteBuilder>,
            Option<PageRetention>,
        ) = env.router.resolve_with_policy(target_path);
        let fb_c: EuvRouteBuilder = Rc::clone(&env.fallback);
        let hub_c: Option<DynamicUpdateHub> = env.hub_opt.clone();
        let target_p_c: String = target_path.to_string();
        let (page_scope, page_node): (Scope, NodeId) = Scope::run(|_: Scope| {
            with_context(env.ctx.clone(), || {
                if let Some(hub) = hub_c.as_ref() {
                    let local_fb: Rc<dyn Fn() -> NodeId> = Rc::new(move || {
                        if let Some(builder) = builder_opt.as_ref() {
                            builder(&matched)
                        } else {
                            fb_c(&matched)
                        }
                    });
                    hub.mount_page_or_fallback(&target_p_c, &local_fb)
                } else if let Some(builder) = builder_opt {
                    builder(&matched)
                } else {
                    fb_c(&matched)
                }
            })
        });
        let comp_ret: Option<PageRetention> = env.scene.with_read(
            |tree: &SceneTree| {
                tree.get_node(page_node).and_then(|n: &SceneNode| {
                    n.lifecycle_observers()
                        .last()
                        .map(|h: &Rc<dyn Lifecycle>| h.retention_policy())
                })
            },
            None,
        );
        let resolved_ret: PageRetention = route_ret.or(comp_ret).unwrap_or(env.default_ret);
        let cur_bg: Option<Color> =
            cur_tokens.map(|t: ThemeTokens| t.get_palette().get_background());
        let host_id: NodeId = env.host;
        let wrapper: NodeId = env.scene.with_write(
            |tree: &mut SceneTree| {
                let mut w_style: Style = Style::default()
                    .column()
                    .width(Length::Percent(100.0))
                    .height(Length::Percent(100.0))
                    .padding(page_pad)
                    .align(AlignItems::Center)
                    .gap(SPACE_LG)
                    .overflow(Overflow::Scroll)
                    .z_index(1);
                if let Some(bg) = cur_bg {
                    w_style = w_style.background(bg);
                }
                if as_preview {
                    w_style
                        .get_mut_layout()
                        .set_position(Position::Absolute)
                        .set_top(Some(0.0))
                        .set_right(Some(0.0))
                        .set_bottom(Some(0.0))
                        .set_left(Some(0.0))
                        .set_z_index(2);
                    w_style
                        .get_mut_paint()
                        .set_pointer_events(PointerEvents::None);
                }
                let w: NodeId = tree.create_scroll_view(w_style);
                tree.append_child(w, page_node);
                tree.append_child(host_id, w);
                w
            },
            NodeId::default(),
        );
        let phase: LifecyclePhase = if as_preview {
            page_scope.freeze();
            LifecyclePhase::Frozen
        } else {
            LifecyclePhase::Active
        };
        EuvCachedPage {
            wrapper,
            page_node,
            scope: page_scope,
            retention: resolved_ret,
            phase,
            applied_tokens: cur_tokens.unwrap_or_default(),
            has_applied_tokens: cur_tokens.is_some(),
            dynamic_rev: cur_dyn_rev,
            activations: if as_preview { 0 } else { 1 },
        }
    }
}

/// Declarative `#[component]` implementation for `EuvRouterOutlet` providing lazy page mounting, coupled 1:1 edge-to-edge transitions, per-page scroll preservation across `PageRetention::Freeze`, in-place theme restyling, and automatic OS Back + double-back-to-exit wiring.
#[component]
/// Inherent implementation of [`EuvRouterOutlet`].
impl EuvRouterOutlet {
    /// Renders the lazy lifecycle-managed router outlet into the ambient `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Router outlet reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted router outlet container handle.
    fn render(&self) -> NodeId {
        let mut ctx: Context = current_context();
        let host: NodeId = ctx.view(
            Style::default()
                .column()
                .flex_grow(1.0)
                .flex_shrink(1.0)
                .width(Length::Percent(100.0))
                .height(Length::Percent(100.0))
                .min_height(Length::Px(0.0))
                .overflow(Overflow::Hidden),
        );
        let router: EuvRouter = self.get_router().clone();
        let route_order: Vec<String> = self.get_route_order().clone();
        let vp_sig: Signal<Size> = ctx.get_viewport_size();
        let swipe_drag_sig: Signal<f32> = ctx.get_swipe_drag_offset();
        let fallback: EuvRouteBuilder = {
            let Self { fallback, .. } = self;
            Rc::clone(fallback)
        };
        let theme_opt: Option<UiThemeState> = {
            let Self { theme, .. } = self;
            theme.clone()
        };
        let page_pad_opt: Option<Signal<Edges>> = {
            let Self { page_padding, .. } = *self;
            page_padding
        };
        if let Some(theme_ref) = theme_opt.as_ref() {
            provide_theme(theme_ref.clone());
        }
        let double_back_enabled: bool = self.get_double_back_exit();
        let exit_prompt_text: String = self.get_exit_prompt().clone();
        let exit_toast_opt: Option<NodeId> = if double_back_enabled {
            let safe_area_sig: Signal<SafeAreaInsets> = ctx.get_safe_area();
            let toast_text_clone: String = exit_prompt_text.clone();
            let toast_node: NodeId = view! {
                portal {
                    display: Display::None
                    position: Position::Fixed
                    inset: (
                        None,
                        Some(0.0),
                        Some(EXIT_TOAST_BOTTOM_OFFSET + safe_area_sig.get_untracked().get_bottom()),
                        Some(0.0)
                    )
                    width: Length::Percent(100.0)
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    pointer_events: PointerEvents::None
                    z_index: Z_INDEX_EXIT_TOAST
                    row {
                        padding: Edges::symmetric(10.0, 20.0)
                        background: Color::rgba(17, 24, 39, 236)
                        border: (1.0, Color::rgba(255, 255, 255, 36))
                        radius: EXIT_TOAST_RADIUS
                        shadow: Shadow::new(0.0, 8.0, 24.0, 0.0, Color::rgba(0, 0, 0, 84))
                        text {
                            font_size: 13.5
                            font_weight: FontWeight::Medium
                            color: Color::rgb(248, 250, 252)
                            text_align: TextAlign::Center
                            { toast_text_clone }
                        }
                    }
                }
            };
            ctx.append(host, toast_node);
            Some(toast_node)
        } else {
            None
        };
        let router_back: EuvRouter = router.clone();
        let sys_back_sig: Signal<u64> = ctx.get_system_back();
        let last_back_seq: Rc<Cell<u64>> = Rc::new(Cell::new(sys_back_sig.get_untracked()));
        let armed_exit_at_ms: Rc<Cell<Option<u64>>> = Rc::new(Cell::new(None));
        let armed_exit_route: Rc<Cell<Option<u64>>> = Rc::clone(&armed_exit_at_ms);
        let (hub_opt, default_ret): (Option<DynamicUpdateHub>, PageRetention) = router
            .get_inner()
            .try_borrow()
            .ok()
            .map(|g: Ref<'_, EuvRouterInner>| (g.dynamic_hub.clone(), g.get_default_retention()))
            .unwrap_or((None, PageRetention::Freeze));
        let path_sig: Signal<String> = router.get_current_path();
        let ctx_spawn: Context = ctx.clone();
        let ctx_drag: Context = ctx.clone();
        let ctx_back: Context = ctx.clone();
        let scene: SceneHandle = ctx.get_scene().clone();
        let scene_drag: SceneHandle = scene.clone();
        let scene_pad: SceneHandle = scene.clone();
        let scene_back: SceneHandle = scene.clone();
        let cache: Rc<RefCell<HashMap<String, EuvCachedPage>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let active_key_cell: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        let drag_preview_cell: Rc<RefCell<Option<EuvDragPreviewState>>> =
            Rc::new(RefCell::new(None));
        let released_drag_cell: Rc<RefCell<Option<EuvDragPreviewState>>> =
            Rc::new(RefCell::new(None));
        let cache_cleanup: Rc<RefCell<HashMap<String, EuvCachedPage>>> = Rc::clone(&cache);
        let cache_pad: Rc<RefCell<HashMap<String, EuvCachedPage>>> = Rc::clone(&cache);
        let cache_drag: Rc<RefCell<HashMap<String, EuvCachedPage>>> = Rc::clone(&cache);
        let active_key_drag: Rc<RefCell<Option<String>>> = Rc::clone(&active_key_cell);
        let drag_preview_route: Rc<RefCell<Option<EuvDragPreviewState>>> =
            Rc::clone(&drag_preview_cell);
        let released_drag_route: Rc<RefCell<Option<EuvDragPreviewState>>> =
            Rc::clone(&released_drag_cell);
        let mount_env: EuvOutletMountEnv = EuvOutletMountEnv {
            router: router.clone(),
            fallback: Rc::clone(&fallback),
            hub_opt: hub_opt.clone(),
            default_ret,
            ctx: ctx_spawn,
            scene: scene.clone(),
            host,
        };
        let mount_env_drag: EuvOutletMountEnv = EuvOutletMountEnv {
            router: router.clone(),
            fallback,
            hub_opt: hub_opt.clone(),
            default_ret,
            ctx: ctx_drag,
            scene: scene_drag.clone(),
            host,
        };
        let hub_drag: Option<DynamicUpdateHub> = hub_opt.clone();
        let theme_drag: Option<UiThemeState> = theme_opt.clone();
        let route_order_drag: Vec<String> = route_order.clone();
        let (outlet_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            on_cleanup(move || {
                if let Ok(mut map) = cache_cleanup.try_borrow_mut() {
                    for (_, entry) in map.drain() {
                        entry.scope.dispose();
                    }
                }
            });
            if let Some(pad_sig) = page_pad_opt {
                let _: Effect = Effect::new(move || {
                    let pad: Edges = pad_sig.get();
                    let wrappers: Vec<NodeId> = cache_pad
                        .try_borrow()
                        .ok()
                        .map(|m: Ref<'_, HashMap<String, EuvCachedPage>>| {
                            m.values().map(|e: &EuvCachedPage| e.wrapper).collect()
                        })
                        .unwrap_or_default();
                    if wrappers.is_empty() {
                        return;
                    }
                    scene_pad.with_write(
                        |tree: &mut SceneTree| {
                            for wid in wrappers.into_iter() {
                                if let Some(node) = tree.get_node_mut(wid) {
                                    let mut st: Style = node.get_style();
                                    st.get_mut_layout().set_padding(pad);
                                    node.set_style(st);
                                    let dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                                    tree.mark_dirty(wid, dirty | DIRTY_PAINT);
                                }
                            }
                        },
                        (),
                    );
                });
            }
            let _: Effect = Effect::new(move || {
                let raw_dx: f32 = swipe_drag_sig.get();
                if route_order_drag.is_empty() || EuvOverlayStack::active_count() > 0 {
                    if let Some(prev_drag) = drag_preview_cell
                        .try_borrow_mut()
                        .ok()
                        .and_then(|mut g: RefMut<'_, Option<EuvDragPreviewState>>| g.take())
                    {
                        let active_w: Option<NodeId> = active_key_drag.try_borrow().ok().and_then(
                            |ak: Ref<'_, Option<String>>| {
                                ak.as_ref().and_then(|p: &String| {
                                    cache_drag.try_borrow().ok().and_then(
                                        |m: Ref<'_, HashMap<String, EuvCachedPage>>| {
                                            m.get(p).map(|e: &EuvCachedPage| e.wrapper)
                                        },
                                    )
                                })
                            },
                        );
                        scene_drag.with_write(
                            |tree: &mut SceneTree| {
                                if let Some(aw) = active_w
                                    && let Some(n) = tree.get_node_mut(aw)
                                {
                                    let mut st: Style = n.get_style();
                                    st.get_mut_paint().set_transform(Transform2D::default());
                                    n.set_style(st);
                                    tree.mark_dirty(aw, DIRTY_PAINT);
                                }
                                if let Some(pn) = tree.get_node_mut(prev_drag.target_wrapper) {
                                    let mut st: Style = pn.get_style();
                                    st.get_mut_layout().set_display(Display::None);
                                    st.get_mut_paint().set_transform(Transform2D::default());
                                    pn.set_style(st);
                                    tree.mark_dirty(prev_drag.target_wrapper, DIRTY_PAINT);
                                }
                            },
                            (),
                        );
                    }
                    return;
                }
                let cur_path_opt: Option<String> = active_key_drag
                    .try_borrow()
                    .ok()
                    .and_then(|g: Ref<'_, Option<String>>| g.clone());
                let Some(cur_path) = cur_path_opt else {
                    return;
                };
                let cur_wrapper_opt: Option<NodeId> = cache_drag.try_borrow().ok().and_then(
                    |m: Ref<'_, HashMap<String, EuvCachedPage>>| {
                        m.get(&cur_path).map(|e: &EuvCachedPage| e.wrapper)
                    },
                );
                let Some(active_wrapper) = cur_wrapper_opt else {
                    return;
                };
                let w: f32 = Self::resolve_slide_width(&scene_drag, host, vp_sig);
                if raw_dx.abs() > 1.0 {
                    let cur_idx_opt: Option<usize> = route_order_drag
                        .iter()
                        .position(|r: &String| r.as_str() == cur_path.as_str());
                    let Some(cur_idx) = cur_idx_opt else {
                        return;
                    };
                    let (dir, adj_opt, clamped_dx): (f32, Option<String>, f32) = if raw_dx < 0.0 {
                        (
                            1.0,
                            route_order_drag.get(cur_idx + 1).cloned(),
                            raw_dx.clamp(-w, 0.0),
                        )
                    } else {
                        let prev_r: Option<String> = cur_idx
                            .checked_sub(1)
                            .and_then(|i: usize| route_order_drag.get(i).cloned());
                        (-1.0, prev_r, raw_dx.clamp(0.0, w))
                    };
                    let Some(adj_path) = adj_opt else {
                        return;
                    };
                    let prev_preview_opt: Option<EuvDragPreviewState> = drag_preview_cell
                        .try_borrow()
                        .ok()
                        .and_then(|g: Ref<'_, Option<EuvDragPreviewState>>| g.clone());
                    if let Some(old_prev) = prev_preview_opt.as_ref()
                        && old_prev.target_path != adj_path
                    {
                        let old_tw: NodeId = old_prev.target_wrapper;
                        scene_drag.with_write(
                            |tree: &mut SceneTree| {
                                tree.cancel_subtree_transitions(old_tw);
                                if let Some(n) = tree.get_node_mut(old_tw) {
                                    let mut st: Style = n.get_style();
                                    st.get_mut_layout().set_display(Display::None);
                                    st.get_mut_paint().set_transform(Transform2D::default());
                                    n.set_style(st);
                                    tree.mark_dirty(old_tw, DIRTY_LAYOUT | DIRTY_PAINT);
                                }
                            },
                            (),
                        );
                    }
                    let cur_tokens: Option<ThemeTokens> = theme_drag
                        .as_ref()
                        .map(|t: &UiThemeState| untracked(|| t.get_tokens().get()));
                    let cur_pad: Edges = page_pad_opt
                        .map(|s: Signal<Edges>| s.get_untracked())
                        .unwrap_or_default();
                    let cur_dyn_rev: u64 = hub_drag
                        .as_ref()
                        .map(|h: &DynamicUpdateHub| {
                            h.page_revision_signal(&adj_path).get_untracked()
                        })
                        .unwrap_or(0);
                    let existing_adj: Option<(NodeId, Option<ThemeTokens>)> = cache_drag
                        .try_borrow()
                        .ok()
                        .and_then(|m: Ref<'_, HashMap<String, EuvCachedPage>>| {
                            m.get(&adj_path).map(|e: &EuvCachedPage| {
                                let prev: Option<ThemeTokens> = if e.has_applied_tokens {
                                    Some(e.applied_tokens)
                                } else {
                                    None
                                };
                                (e.wrapper, prev)
                            })
                        });
                    let adj_wrapper: NodeId = match existing_adj {
                        Some((tw, prev_tok)) => {
                            if let (Some(old_t), Some(new_t)) = (prev_tok, cur_tokens)
                                && old_t != new_t
                            {
                                scene_drag.with_write(
                                    |tree: &mut SceneTree| {
                                        remap_subtree_theme_in_place(tree, tw, old_t, new_t);
                                    },
                                    (),
                                );
                                if let Ok(mut map) = cache_drag.try_borrow_mut()
                                    && let Some(entry) = map.get_mut(&adj_path)
                                {
                                    entry.applied_tokens = new_t;
                                    entry.has_applied_tokens = true;
                                }
                            }
                            tw
                        }
                        None => {
                            let built: EuvCachedPage = Self::mount_cached_page(
                                &mount_env_drag,
                                &adj_path,
                                cur_tokens,
                                cur_dyn_rev,
                                cur_pad,
                                true,
                            );
                            let tw: NodeId = built.wrapper;
                            if let Ok(mut map) = cache_drag.try_borrow_mut() {
                                let _: Option<EuvCachedPage> = map.insert(adj_path.clone(), built);
                            }
                            tw
                        }
                    };
                    let adj_tx: f32 = clamped_dx + dir * w;
                    scene_drag.with_write(
                        |tree: &mut SceneTree| {
                            tree.cancel_subtree_transitions(active_wrapper);
                            tree.cancel_subtree_transitions(adj_wrapper);
                            if let Some(an) = tree.get_node_mut(active_wrapper) {
                                let mut st: Style = an.get_style();
                                st.get_mut_paint().set_opacity(1.0).set_transform(
                                    Transform2D::new(clamped_dx, 0.0, 1.0, 1.0, 0.0),
                                );
                                an.set_style(st);
                                tree.mark_dirty(active_wrapper, DIRTY_PAINT);
                            }
                            if let Some(pn) = tree.get_node_mut(adj_wrapper) {
                                let mut st: Style = pn.get_style();
                                let was_hidden: bool =
                                    st.get_layout().get_display() == Display::None;
                                st.get_mut_layout()
                                    .set_display(Display::Flex)
                                    .set_position(Position::Absolute)
                                    .set_top(Some(0.0))
                                    .set_right(Some(0.0))
                                    .set_bottom(Some(0.0))
                                    .set_left(Some(0.0))
                                    .set_z_index(2);
                                st.get_mut_paint()
                                    .set_pointer_events(PointerEvents::None)
                                    .set_opacity(1.0)
                                    .set_transform(Transform2D::new(adj_tx, 0.0, 1.0, 1.0, 0.0));
                                pn.set_style(st);
                                let layout_dirty: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                                let mask: u16 = if was_hidden {
                                    layout_dirty | DIRTY_PAINT
                                } else {
                                    DIRTY_PAINT
                                };
                                tree.mark_dirty(adj_wrapper, mask);
                            }
                        },
                        (),
                    );
                    if let Ok(mut slot) = drag_preview_cell.try_borrow_mut() {
                        *slot = Some(EuvDragPreviewState {
                            target_path: adj_path,
                            target_wrapper: adj_wrapper,
                            direction: dir,
                            offset_x: clamped_dx,
                            width: w,
                        });
                    }
                } else if let Some(released) = drag_preview_cell
                    .try_borrow_mut()
                    .ok()
                    .and_then(|mut g: RefMut<'_, Option<EuvDragPreviewState>>| g.take())
                {
                    let frac: f32 =
                        (released.offset_x.abs() / released.width.max(1.0)).clamp(0.2, 1.0);
                    let cancel_dur: f32 =
                        (ROUTE_DRAG_CANCEL_SECS * frac).clamp(0.10, ROUTE_DRAG_CANCEL_SECS);
                    let start_adj_x: f32 = released.offset_x + released.direction * released.width;
                    let end_adj_x: f32 = released.direction * released.width;
                    if let Ok(mut rel_slot) = released_drag_cell.try_borrow_mut() {
                        *rel_slot = Some(released.clone());
                    }
                    scene_drag.start_transition(
                        NodeTransition::slide_fade(
                            active_wrapper,
                            Point::new(released.offset_x, 0.0),
                            Point::new(0.0, 0.0),
                            1.0,
                            1.0,
                            cancel_dur,
                            false,
                        )
                        .with_easing(Easing::EaseOutCubic),
                    );
                    scene_drag.start_transition(
                        NodeTransition::slide_fade(
                            released.target_wrapper,
                            Point::new(start_adj_x, 0.0),
                            Point::new(end_adj_x, 0.0),
                            1.0,
                            1.0,
                            cancel_dur,
                            true,
                        )
                        .with_easing(Easing::EaseOutCubic),
                    );
                }
            });
            let _: Effect = Effect::new(move || {
                let seq: u64 = sys_back_sig.get();
                if seq != last_back_seq.replace(seq) {
                    let consumed: bool = router_back.handle_system_back();
                    if consumed {
                        armed_exit_at_ms.set(None);
                        if let Some(tid) = exit_toast_opt {
                            scene_back.with_write(
                                |tree: &mut SceneTree| {
                                    if let Some(n) = tree.get_node_mut(tid) {
                                        let mut st: Style = n.get_style();
                                        st.get_mut_layout().set_display(Display::None);
                                        n.set_style(st);
                                    }
                                    tree.cancel_subtree_transitions(tid);
                                    tree.mark_dirty(tid, DIRTY_LAYOUT | DIRTY_PAINT);
                                },
                                (),
                            );
                        }
                    } else if double_back_enabled {
                        let now_ms: u64 = ctx_back.elapsed_ms();
                        let second_press: bool = armed_exit_at_ms
                            .get()
                            .map(|prev: u64| {
                                now_ms.saturating_sub(prev) <= DOUBLE_BACK_EXIT_WINDOW_MS
                            })
                            .unwrap_or(false);
                        if second_press {
                            armed_exit_at_ms.set(None);
                            if let Some(tid) = exit_toast_opt {
                                scene_back.with_write(
                                    |tree: &mut SceneTree| {
                                        if let Some(n) = tree.get_node_mut(tid) {
                                            let mut st: Style = n.get_style();
                                            st.get_mut_layout().set_display(Display::None);
                                            n.set_style(st);
                                        }
                                        tree.cancel_subtree_transitions(tid);
                                        tree.mark_dirty(tid, DIRTY_LAYOUT | DIRTY_PAINT);
                                    },
                                    (),
                                );
                            }
                            ctx_back.request_exit();
                        } else {
                            armed_exit_at_ms.set(Some(now_ms));
                            let handled_by_os: bool =
                                dispatch_native_action(NATIVE_ACTION_TOAST, &exit_prompt_text, "");
                            if !handled_by_os && let Some(tid) = exit_toast_opt {
                                scene_back.with_write(
                                    |tree: &mut SceneTree| {
                                        if let Some(n) = tree.get_node_mut(tid) {
                                            let mut st: Style = n.get_style();
                                            st.get_mut_layout().set_display(Display::Flex);
                                            st.get_mut_paint().set_opacity(1.0);
                                            n.set_style(st);
                                        }
                                        tree.mark_dirty(tid, DIRTY_LAYOUT | DIRTY_PAINT);
                                    },
                                    (),
                                );
                                scene_back.start_transition(
                                    NodeTransition::fade(
                                        tid,
                                        1.0,
                                        0.0,
                                        DOUBLE_BACK_TOAST_FADE_SECS,
                                        true,
                                    )
                                    .with_delay(DOUBLE_BACK_TOAST_HOLD_SECS)
                                    .with_easing(Easing::EaseOutCubic),
                                );
                            }
                        }
                    }
                }
            });
            Effect::new(move || {
                let active_path: String = path_sig.get();
                let cur_tokens: Option<ThemeTokens> = theme_opt
                    .as_ref()
                    .map(|t: &UiThemeState| t.get_tokens().get());
                let cur_dyn_rev: u64 = hub_opt
                    .as_ref()
                    .map(|h: &DynamicUpdateHub| {
                        let _routes_rev: u64 = h.routes_signal().get();
                        h.page_revision_signal(&active_path).get()
                    })
                    .unwrap_or(0);
                let prev_path_opt: Option<String> = active_key_cell
                    .try_borrow()
                    .ok()
                    .and_then(|g: Ref<'_, Option<String>>| g.clone());
                let route_changed: bool = prev_path_opt.as_deref() != Some(active_path.as_str());
                let active_drag_opt: Option<EuvDragPreviewState> = drag_preview_route
                    .try_borrow_mut()
                    .ok()
                    .and_then(|mut g: RefMut<'_, Option<EuvDragPreviewState>>| g.take())
                    .or_else(|| {
                        released_drag_route
                            .try_borrow_mut()
                            .ok()
                            .and_then(|mut g: RefMut<'_, Option<EuvDragPreviewState>>| g.take())
                    });
                if route_changed {
                    armed_exit_route.set(None);
                    if let Some(tid) = exit_toast_opt {
                        scene.with_write(
                            |tree: &mut SceneTree| {
                                if let Some(n) = tree.get_node_mut(tid)
                                    && n.get_style().get_layout().get_display() != Display::None
                                {
                                    let mut st: Style = n.get_style();
                                    st.get_mut_layout().set_display(Display::None);
                                    n.set_style(st);
                                    tree.cancel_subtree_transitions(tid);
                                    tree.mark_dirty(tid, DIRTY_LAYOUT | DIRTY_PAINT);
                                }
                            },
                            (),
                        );
                    }
                }
                let slide_dir: f32 = if route_changed && let Some(prev_p) = prev_path_opt.as_deref()
                {
                    let from_idx: Option<usize> = route_order
                        .iter()
                        .position(|r: &String| r.as_str() == prev_p);
                    let to_idx: Option<usize> = route_order
                        .iter()
                        .position(|r: &String| r.as_str() == active_path.as_str());
                    match (from_idx, to_idx) {
                        (Some(a), Some(b)) if b > a => 1.0,
                        (Some(a), Some(b)) if b < a => -1.0,
                        _ => {
                            let reg_dir: Option<f32> =
                                router.get_inner().try_borrow().ok().and_then(
                                    |g: Ref<'_, EuvRouterInner>| {
                                        let pa: Option<usize> = g
                                            .get_routes()
                                            .iter()
                                            .position(|d: &EuvRouteDef| d.get_pattern() == prev_p);
                                        let pb: Option<usize> =
                                            g.get_routes().iter().position(|d: &EuvRouteDef| {
                                                d.get_pattern().as_str() == active_path.as_str()
                                            });
                                        match (pa, pb) {
                                            (Some(a), Some(b)) if b > a => Some(1.0),
                                            (Some(a), Some(b)) if b < a => Some(-1.0),
                                            _ => None,
                                        }
                                    },
                                );
                            reg_dir.unwrap_or(1.0)
                        }
                    }
                } else {
                    0.0
                };
                let slide_w: f32 = Self::resolve_slide_width(&scene, host, vp_sig);
                let (start_exit_tx, start_enter_tx, slide_dur): (f32, f32, f32) = if route_changed
                    && slide_dir.abs() > 0.01
                {
                    if let Some(drag) = active_drag_opt.as_ref()
                        && drag.target_path == active_path
                        && (drag.direction - slide_dir).abs() < 0.01
                    {
                        let w_used: f32 = drag.width.max(slide_w);
                        let rem: f32 =
                            (1.0 - (drag.offset_x.abs() / w_used.max(1.0))).clamp(0.15, 1.0);
                        let dur: f32 =
                            (ROUTE_COUPLED_SLIDE_SECS * rem).clamp(0.10, ROUTE_COUPLED_SLIDE_SECS);
                        (drag.offset_x, drag.offset_x + slide_dir * w_used, dur)
                    } else {
                        if let Some(stray) = active_drag_opt.as_ref()
                            && stray.target_path != active_path
                        {
                            let sw: NodeId = stray.target_wrapper;
                            scene.with_write(
                                |tree: &mut SceneTree| {
                                    tree.cancel_subtree_transitions(sw);
                                    if let Some(n) = tree.get_node_mut(sw) {
                                        let mut st: Style = n.get_style();
                                        st.get_mut_layout().set_display(Display::None);
                                        st.get_mut_paint().set_transform(Transform2D::default());
                                        n.set_style(st);
                                        tree.mark_dirty(sw, DIRTY_LAYOUT | DIRTY_PAINT);
                                    }
                                },
                                (),
                            );
                        }
                        (0.0, slide_dir * slide_w, ROUTE_COUPLED_SLIDE_SECS)
                    }
                } else {
                    (0.0, 0.0, ROUTE_COUPLED_SLIDE_SECS)
                };
                if route_changed && let Some(prev_path) = prev_path_opt {
                    let leave_entry: Option<EuvCachedPage> = cache.try_borrow_mut().ok().and_then(
                        |mut map: RefMut<'_, HashMap<String, EuvCachedPage>>| {
                            let ret: PageRetention = map
                                .get(&prev_path)
                                .map(|e: &EuvCachedPage| e.retention)
                                .unwrap_or(default_ret);
                            if ret == PageRetention::Destroy {
                                map.remove(&prev_path)
                            } else if let Some(entry) = map.get_mut(&prev_path) {
                                entry.phase = if ret == PageRetention::Freeze {
                                    LifecyclePhase::Frozen
                                } else {
                                    LifecyclePhase::Mounted
                                };
                                Some(entry.clone())
                            } else {
                                None
                            }
                        },
                    );
                    if let Some(prev_entry) = leave_entry {
                        let coupled_exit: NodeTransition = NodeTransition::slide_fade(
                            prev_entry.wrapper,
                            Point::new(start_exit_tx, 0.0),
                            Point::new(-slide_dir * slide_w, 0.0),
                            1.0,
                            1.0,
                            slide_dur,
                            true,
                        )
                        .with_easing(Easing::EaseOutCubic);
                        match prev_entry.retention {
                            PageRetention::Destroy => {
                                scene.remove_and_dispose_child(host, prev_entry.wrapper);
                                prev_entry.scope.dispose();
                            }
                            PageRetention::Freeze => {
                                prev_entry.scope.freeze();
                                scene.freeze_subtree_animated(prev_entry.wrapper, coupled_exit);
                            }
                            PageRetention::Alive => {
                                scene.with_write(
                                    |tree: &mut SceneTree| {
                                        tree.cancel_subtree_transitions(prev_entry.wrapper);
                                        if let Some(n) = tree.get_node_mut(prev_entry.wrapper) {
                                            let mut st: Style = n.get_style();
                                            st.get_mut_layout()
                                                .set_position(Position::Absolute)
                                                .set_top(Some(0.0))
                                                .set_right(Some(0.0))
                                                .set_bottom(Some(0.0))
                                                .set_left(Some(0.0))
                                                .set_z_index(0);
                                            st.get_mut_paint()
                                                .set_pointer_events(PointerEvents::None)
                                                .set_opacity(1.0);
                                            n.set_style(st);
                                        }
                                        tree.start_transition(coupled_exit);
                                        tree.mark_dirty(
                                            prev_entry.wrapper,
                                            DIRTY_LAYOUT | DIRTY_PAINT,
                                        );
                                    },
                                    (),
                                );
                            }
                        }
                    }
                }
                let cached_hit: Option<EuvCachedPage> = cache.try_borrow().ok().and_then(
                    |map: Ref<'_, HashMap<String, EuvCachedPage>>| map.get(&active_path).cloned(),
                );
                let reusable: bool = cached_hit
                    .as_ref()
                    .map(|e: &EuvCachedPage| e.dynamic_rev == cur_dyn_rev)
                    .unwrap_or(false);
                if reusable && let Some(hit) = cached_hit {
                    let prev_tok_opt: Option<ThemeTokens> = if hit.has_applied_tokens {
                        Some(hit.applied_tokens)
                    } else {
                        None
                    };
                    let theme_changed: bool = prev_tok_opt != cur_tokens;
                    scene.with_write(
                        |tree: &mut SceneTree| {
                            if route_changed {
                                tree.cancel_subtree_transitions(hit.wrapper);
                                if let Some(n) = tree.get_node_mut(hit.wrapper) {
                                    let mut st: Style = n.get_style();
                                    st.get_mut_layout()
                                        .set_display(Display::Flex)
                                        .set_position(Position::Relative)
                                        .set_top(None)
                                        .set_right(None)
                                        .set_bottom(None)
                                        .set_left(None)
                                        .set_z_index(1);
                                    st.get_mut_paint()
                                        .set_pointer_events(PointerEvents::Auto)
                                        .set_opacity(1.0)
                                        .set_transform(Transform2D::default());
                                    n.set_style(st);
                                }
                                let dirty_l: u16 = DIRTY_LAYOUT | DIRTY_SUBTREE_LAYOUT;
                                tree.mark_dirty(hit.wrapper, dirty_l | DIRTY_PAINT);
                            }
                            if theme_changed
                                && let (Some(old_t), Some(new_t)) = (prev_tok_opt, cur_tokens)
                            {
                                remap_subtree_theme_in_place(tree, hit.wrapper, old_t, new_t);
                            }
                        },
                        (),
                    );
                    if route_changed && hit.phase == LifecyclePhase::Frozen {
                        scene.resume_subtree(hit.wrapper);
                        hit.scope.resume();
                    }
                    if let Ok(mut map) = cache.try_borrow_mut()
                        && let Some(entry) = map.get_mut(&active_path)
                    {
                        if let Some(new_t) = cur_tokens {
                            entry.applied_tokens = new_t;
                            entry.has_applied_tokens = true;
                        }
                        if route_changed {
                            entry.phase = LifecyclePhase::Active;
                            entry.activations = entry.activations.saturating_add(1);
                        }
                    }
                    if route_changed {
                        scene.activate_subtree(hit.page_node);
                        let tr: NodeTransition = if slide_dir.abs() > 0.01 {
                            NodeTransition::slide_fade(
                                hit.wrapper,
                                Point::new(start_enter_tx, 0.0),
                                Point::new(0.0, 0.0),
                                1.0,
                                1.0,
                                slide_dur,
                                false,
                            )
                            .with_easing(Easing::EaseOutCubic)
                        } else {
                            NodeTransition::route_enter(hit.wrapper)
                        };
                        scene.start_transition(tr);
                    }
                } else {
                    if let Some(stale) = cached_hit {
                        if let Ok(mut map) = cache.try_borrow_mut() {
                            let _: Option<EuvCachedPage> = map.remove(&active_path);
                        }
                        scene.remove_and_dispose_child(host, stale.wrapper);
                        stale.scope.dispose();
                    }
                    let cur_pad: Edges = page_pad_opt
                        .map(|s: Signal<Edges>| s.get_untracked())
                        .unwrap_or_default();
                    let mounted: EuvCachedPage = Self::mount_cached_page(
                        &mount_env,
                        &active_path,
                        cur_tokens,
                        cur_dyn_rev,
                        cur_pad,
                        false,
                    );
                    let wrapper: NodeId = mounted.wrapper;
                    let page_node: NodeId = mounted.page_node;
                    if let Ok(mut map) = cache.try_borrow_mut() {
                        let _: Option<EuvCachedPage> = map.insert(active_path.clone(), mounted);
                    }
                    scene.activate_subtree(page_node);
                    let is_initial: bool = active_key_cell
                        .try_borrow()
                        .ok()
                        .map(|g: Ref<'_, Option<String>>| g.is_none())
                        .unwrap_or(true);
                    if !is_initial {
                        let tr: NodeTransition = if slide_dir.abs() > 0.01 {
                            NodeTransition::slide_fade(
                                wrapper,
                                Point::new(start_enter_tx, 0.0),
                                Point::new(0.0, 0.0),
                                1.0,
                                1.0,
                                slide_dur,
                                false,
                            )
                            .with_easing(Easing::EaseOutCubic)
                        } else {
                            NodeTransition::route_enter(wrapper)
                        };
                        scene.start_transition(tr);
                    }
                }
                if let Ok(mut active_slot) = active_key_cell.try_borrow_mut() {
                    *active_slot = Some(active_path);
                }
                if let Ok(map) = cache.try_borrow() {
                    router.publish_lifecycle_telemetry(&map);
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(host, outlet_scope);
            },
            (),
        );
        host
    }
}

/// Declarative `#[component]` implementation for `EuvLink`.
#[component]
/// Inherent implementation of [`EuvLink`].
impl EuvLink {
    /// Renders a clickable link button that pushes `to` onto `router`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Link component reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted link node handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let to: String = self.get_to().clone();
        let label: String = self.get_label().clone();
        let router: EuvRouter = self.get_router().clone();
        let on_link_click = move |_: &UiEvent| {
            router.push(&to);
        };
        EuvButton::new(label, tokens)
            .ghost()
            .sm()
            .on_click(on_link_click)
            .mount()
    }
}
