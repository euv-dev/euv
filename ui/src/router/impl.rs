use super::*;

/// History stack navigation, pattern matching, and system-back integration for `EuvRouter`.
impl EuvRouter {
    /// Creates a new `EuvRouter` initialized at `initial_path`.
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
        };
        Self {
            current_path: Signal::new(start),
            can_go_back: Signal::new(false),
            can_go_forward: Signal::new(false),
            history_len: Signal::new(1),
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    /// Registers a declarative route `pattern` (supporting `:param` segments) with a page builder.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source router.
    /// - `&str` - Route pattern (e.g. `"/media"` or `"/items/:id"`).
    /// - `&str` - Human-readable page title.
    /// - `F` - Page builder receiving `&EuvRouteMatch`.
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
                builder: Rc::new(builder),
            });
        }
        self
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
                    let EuvRouteDef { builder, .. } = def;
                    return (
                        EuvRouteMatch {
                            path: clean_path.to_string(),
                            pattern: def.get_pattern().clone(),
                            title: def.get_title().clone(),
                            params,
                            query,
                        },
                        Some(Rc::clone(builder)),
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
        )
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
    /// - `EuvRouterOutlet` - Mountable reactive router outlet.
    #[must_use]
    pub fn outlet<F>(&self, fallback: F) -> EuvRouterOutlet
    where
        F: Fn(&EuvRouteMatch) -> NodeId + 'static,
    {
        EuvRouterOutlet {
            router: self.clone(),
            fallback: Rc::new(fallback),
        }
    }
}

/// `Mountable` implementation for `EuvRouterOutlet` with smooth slide-fade route transitions and automatic system-back wiring.
impl Mountable for EuvRouterOutlet {
    /// Mounts the reactive router outlet and attaches the unified OS Back / Escape listener.
    ///
    /// # Arguments
    ///
    /// - `Self` - Router outlet props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted router outlet container node.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let host: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .align(AlignItems::Center)
                .gap(SPACE_LG),
        );
        let Self { router, fallback } = self;
        let router_back: EuvRouter = router.clone();
        let sys_back_sig: Signal<u64> = ctx.get_system_back();
        let last_back_seq: Rc<Cell<u64>> = Rc::new(Cell::new(sys_back_sig.get_untracked()));
        let (back_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            Effect::new(move || {
                let seq: u64 = sys_back_sig.get();
                if seq != last_back_seq.replace(seq) {
                    let _: bool = router_back.handle_system_back();
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(host, back_scope);
            },
            (),
        );
        let path_sig: Signal<String> = router.get_current_path();
        let hub_opt: Option<DynamicUpdateHub> = router
            .get_inner()
            .try_borrow()
            .ok()
            .and_then(|g: Ref<'_, EuvRouterInner>| g.dynamic_hub.clone());
        let handle_anim: SceneHandle = ctx.get_scene().clone();
        let first_mount: Rc<Cell<bool>> = Rc::new(Cell::new(true));
        ctx.bind_children(host, move || {
            let active_path: String = path_sig.get();
            let (matched, builder_opt): (EuvRouteMatch, Option<EuvRouteBuilder>) =
                router.resolve(&active_path);
            let page_node: NodeId = if let Some(hub) = hub_opt.as_ref() {
                let _: u64 = hub.routes_signal().get();
                let fb: EuvRouteBuilder = Rc::clone(&fallback);
                let local_fallback: Rc<dyn Fn() -> NodeId> = Rc::new(move || {
                    if let Some(builder) = builder_opt.as_ref() {
                        builder(&matched)
                    } else {
                        fb(&matched)
                    }
                });
                hub.mount_page_or_fallback(&active_path, &local_fallback)
            } else if let Some(builder) = builder_opt {
                builder(&matched)
            } else {
                fallback(&matched)
            };
            if !first_mount.replace(false) {
                handle_anim.start_transition(NodeTransition::slide_fade(
                    page_node,
                    Point::new(0.0, 12.0),
                    Point::new(0.0, 0.0),
                    0.0,
                    1.0,
                    0.20,
                    false,
                ));
            }
            vec![page_node]
        });
        host
    }
}

/// `Mountable` implementation for `EuvLink`.
impl Mountable for EuvLink {
    /// Mounts a clickable link button that pushes `to` onto `router`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Link props.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted link node.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let Self {
            to, label, router, ..
        } = self;
        EuvButton::new(label, tokens)
            .ghost()
            .sm()
            .on_click(move |_: &UiEvent| {
                router.push(&to);
            })
            .mount(ctx)
    }
}
