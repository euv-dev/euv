use super::*;

/// Implements default configuration and state initialization for scheduler types.
impl Default for SchedulerConfig {
    /// Constructs a default [`SchedulerConfig`] value.
    ///
    /// # Returns
    ///
    /// - `SchedulerConfig` - A default-constructed instance with the documented initial state.
    fn default() -> SchedulerConfig {
        SchedulerConfig::new(DEFAULT_FIXED_TIMESTEP, DEFAULT_MAX_FRAME_TIME)
    }
}

/// Implements `Default` for `SchedulerState` as a freshly created stopped state.
impl Default for SchedulerState {
    /// Constructs a default [`SchedulerState`] value.
    ///
    /// # Returns
    ///
    /// - `SchedulerState` - A default-constructed instance with the documented initial state.
    fn default() -> SchedulerState {
        SchedulerState::new(UNINITIALIZED_TIME)
    }
}

/// Implements time retrieval and tick execution for `SchedulerState`.
impl SchedulerState {
    /// Returns the current high-resolution timestamp in seconds from `performance.now()`.
    ///
    /// Falls back to `0.0` when the global window or the `performance.now`
    /// API is unavailable (for example outside a browser window context).
    ///
    /// The `performance` object and its `now` `Function` are page-lifetime
    /// globals, so they are cached in a thread-local on first use — the
    /// per-frame cost is one `call0` crossing, not two `Reflect::get` +
    /// two `JsValue::from_str` allocations.
    ///
    /// No cache borrow is ever held across a call out to JS. `Reflect::get`
    /// resolves page-reachable properties, and `performance.now` is itself
    /// a writable property: a host page that wraps it (profiling shim,
    /// fake-timer test double) can call back into this module, and a
    /// re-entered `current_time` would hit an already-mutably-borrowed cell
    /// and panic the wasm instance, which has no unwinder to catch it. The
    /// cached pair is therefore cloned out and the guard dropped before
    /// `call0`; the miss path resolves the globals and fills the cache
    /// through `try_borrow_mut`, skipping the write rather than panicking
    /// if another call is in flight.
    ///
    /// # Returns
    ///
    /// - `f64` - The current time in seconds, or `0.0` when unavailable.
    pub fn current_time() -> f64 {
        if !cfg!(target_arch = "wasm32") {
            return 0.0;
        }
        thread_local! {
            static PERFORMANCE_NOW: RefCell<Option<(JsValue, Function)>> =
                const { RefCell::new(None) };
        }
        PERFORMANCE_NOW.with(|cell: &RefCell<Option<(JsValue, Function)>>| {
            let cached: Option<(JsValue, Function)> = cell.borrow().as_ref().cloned();
            let (performance, now_function): (JsValue, Function) = match cached {
                Some(pair) => pair,
                None => {
                    let Some(window_value) = window() else {
                        return 0.0;
                    };
                    let Ok(performance) = Reflect::get(
                        window_value.as_ref(),
                        &JsValue::from_str(PERFORMANCE_OBJECT),
                    ) else {
                        return 0.0;
                    };
                    let Ok(now_method) =
                        Reflect::get(&performance, &JsValue::from_str(PERFORMANCE_NOW_METHOD))
                    else {
                        return 0.0;
                    };
                    let pair: (JsValue, Function) = (performance, now_method.unchecked_into());
                    if let Ok(mut borrow) = cell.try_borrow_mut() {
                        *borrow = Some(pair.clone());
                    }
                    pair
                }
            };
            now_function
                .call0(&performance)
                .ok()
                .and_then(|v: JsValue| v.as_f64())
                .map(|millis: f64| millis / 1000.0)
                .unwrap_or(0.0)
        })
    }

    /// Performs one tick of the fixed-timestep scheduler.
    ///
    /// Calculates the elapsed frame time, clamps it to `max_frame_time`, accumulates it,
    /// then runs as many fixed updates as needed. Finally, computes the interpolation
    /// factor and calls the render callback. When an input cell is supplied, its
    /// per-frame edge state is cleared after the render callback so the next frame
    /// observes only the edges that happened during that frame.
    ///
    /// Registered tasks are advanced immediately after each handler
    /// `on_update` call, not once per frame: a frame may run several fixed
    /// steps, and every one of them must advance the tasks by the same
    /// `fixed_timestep` that gameplay logic receives. Driving tasks after
    /// the handler callback keeps ordering explicit — gameplay logic
    /// registered in `on_update` observes tasks that have already advanced
    /// for this step.
    ///
    /// # Arguments
    ///
    /// - `&SchedulerConfig` - The scheduler configuration.
    /// - `&TickHandlerRc` - The handler receiving update and render callbacks.
    /// - `Option<&TaskRegistryRc>` - The task registry to advance each fixed
    ///   step, or `None` when no tasks are registered.
    /// - `Option<&InputStateCell>` - The shared input state to close out, or `None`
    ///   when no input listeners are registered.
    pub fn tick(
        &mut self,
        config: &SchedulerConfig,
        handler: &TickHandlerRc,
        tasks: Option<&TaskRegistryRc>,
        input_cell: Option<&InputStateCell>,
    ) {
        let current_time: f64 = Self::current_time();
        let frame_time: f64 = if self.get_last_time() == UNINITIALIZED_TIME {
            config.get_fixed_timestep()
        } else {
            current_time - self.get_last_time()
        };
        self.set_last_time(current_time);
        let clamped_frame_time: f64 = frame_time.min(config.get_max_frame_time());
        *self.get_mut_accumulator() += clamped_frame_time;
        while self.get_accumulator() >= config.get_fixed_timestep() {
            handler.get_mut().on_update(config.get_fixed_timestep());
            if let Some(registry) = tasks {
                registry.get_mut().update_all(config.get_fixed_timestep());
            }
            *self.get_mut_accumulator() -= config.get_fixed_timestep();
            *self.get_mut_update_count() += 1;
        }
        let interpolation: f64 = self.get_accumulator() / config.get_fixed_timestep();
        handler.get_mut().on_render(interpolation);
        *self.get_mut_frame_count() += 1;
        if let Some(cell) = input_cell {
            cell.get_mut().end_frame();
        }
    }
}

/// Implements registration and per-step advancement for [`TaskRegistry`].
impl TaskRegistry {
    /// Registers an updater and returns a handle that can remove it again.
    ///
    /// The returned [`TaskHandle`] stores the task's insertion index, so
    /// unregistering does not scan the task list for a matching pointer.
    /// Because removal preserves the relative order of the surviving
    /// tasks, a handle only stays valid while no *earlier* task has been
    /// removed; [`TaskRegistry::unregister`] re-resolves the index against
    /// the current list rather than trusting a stale one.
    ///
    /// # Arguments
    ///
    /// - `T` - The updater to drive each fixed step. Must implement
    ///   [`Updatable`] and be `'static` so it can be boxed into the
    ///   heterogeneous task list.
    ///
    /// # Returns
    ///
    /// - `TaskHandle` - A handle used to unregister the task.
    pub fn register<T>(&mut self, task: T) -> TaskHandle
    where
        T: Updatable + 'static,
    {
        let id: u64 = self.get_mut_tasks().len() as u64;
        self.get_mut_tasks().push(Box::new(task));
        TaskHandle::new(id)
    }

    /// Removes a previously registered task, returning whether it was found.
    ///
    /// A task is identified by its [`TaskHandle`]. The removal keeps the
    /// insertion order of the remaining tasks intact, which is what the
    /// per-step update contract promises.
    ///
    /// # Arguments
    ///
    /// - `&TaskHandle` - The handle returned by [`TaskRegistry::register`].
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if a task was removed, `false` if the handle did
    ///   not match any registered task.
    pub fn unregister(&mut self, handle: &TaskHandle) -> bool {
        let index: usize = handle.get_id() as usize;
        if index >= self.get_tasks().len() {
            return false;
        }
        self.get_mut_tasks().remove(index);
        true
    }

    /// Advances every registered task by `delta_time` seconds.
    ///
    /// Tasks are updated in registration order. The whole list is walked
    /// even if a task unregisters another one later in the list, because
    /// the borrow of the task vector is held for the duration of the walk;
    /// deferring mutation to the next step keeps the iteration well-defined.
    ///
    /// # Arguments
    ///
    /// - `f64` - The fixed delta time in seconds.
    pub fn update_all(&mut self, delta_time: f64) {
        for task in self.get_mut_tasks().iter_mut() {
            task.update(delta_time);
        }
    }

    /// Returns the number of registered tasks.
    ///
    /// # Returns
    ///
    /// - `usize` - The task count.
    pub fn len(&self) -> usize {
        self.get_tasks().len()
    }

    /// Returns whether the registry holds no tasks.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if no tasks are registered.
    pub fn is_empty(&self) -> bool {
        self.get_tasks().is_empty()
    }

    /// Removes every registered task.
    pub fn clear(&mut self) {
        self.get_mut_tasks().clear();
    }
}

/// Implements lifecycle management for `SchedulerHandle`.
impl SchedulerHandle {
    /// Stops the scheduler and cancels any pending animation frame request.
    pub fn stop(&self) {
        let state: &mut SchedulerState = self.get_state().get_mut();
        state.set_running(false);
        if let Some(id) = state.get_mut_raf_id().take() {
            let Some(window_value) = window() else {
                // Drop the closure so the box can be collected.
                let _ = self.get_closure_cell().try_take();
                return;
            };
            let _: Result<(), JsValue> = window_value.cancel_animation_frame(id);
        }
        // Drop the closure so the box can be collected.
        let _ = self.get_closure_cell().try_take();
    }

    /// Returns whether the scheduler is currently running.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the scheduler is running.
    pub fn is_running(&self) -> bool {
        // SAFETY: caller contract - no mutable access to the same
        // SchedulerState can be alive alongside this call.
        self.get_state().get().get_running()
    }

    /// Returns the total number of fixed update steps executed.
    ///
    /// # Returns
    ///
    /// - `u64` - The update count.
    pub fn update_count(&self) -> u64 {
        self.get_state().get().get_update_count()
    }

    /// Returns the total number of render frames executed.
    ///
    /// # Returns
    ///
    /// - `u64` - The frame count.
    pub fn frame_count(&self) -> u64 {
        self.get_state().get().get_frame_count()
    }

    /// Registers a task to be advanced on every fixed step.
    ///
    /// The task is driven by [`SchedulerState::tick`] immediately after the
    /// handler's `on_update` callback, using the same
    /// [`SchedulerConfig::get_fixed_timestep`] delta.
    ///
    /// # Arguments
    ///
    /// - `&TaskRegistryRc` - The registry to add the task to.
    /// - `T` - The updater to register. Must implement [`Updatable`].
    ///
    /// # Returns
    ///
    /// - `TaskHandle` - A handle used to unregister the task.
    pub fn register_task<T>(registry: &TaskRegistryRc, task: T) -> TaskHandle
    where
        T: Updatable + 'static,
    {
        registry.get_mut().register(task)
    }

    /// Starts the scheduler with the given configuration and handler.
    ///
    /// Creates a `requestAnimationFrame`-driven loop that calls `tick`
    /// on each animation frame. The returned `SchedulerHandle` can be used to stop the scheduler.
    ///
    /// When no global window exists (non-browser context), the scheduler is
    /// not started and an already-stopped handle is returned instead.
    ///
    /// # Arguments
    ///
    /// - `SchedulerConfig` - The scheduler configuration.
    /// - `TickHandlerRc` - The handler receiving update and render callbacks.
    /// - `Option<&TaskRegistryRc>` - The task registry advanced on every
    ///   fixed step, or `None` when no tasks are registered.
    /// - `Option<&InputStateCell>` - The shared input state whose per-frame edge
    ///   state is cleared at the end of every frame, or `None` when input
    ///   listeners are not registered.
    ///
    /// # Returns
    ///
    /// - `SchedulerHandle` - A handle to control the running scheduler.
    pub fn start(
        config: SchedulerConfig,
        handler: TickHandlerRc,
        tasks: Option<&TaskRegistryRc>,
        input_cell: Option<&InputStateCell>,
    ) -> SchedulerHandle {
        let state: Rc<EngineCell<SchedulerState>> =
            Rc::new(EngineCell::new(SchedulerState::new(UNINITIALIZED_TIME)));
        let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
        // Install the initial closure once spawn starts.
        let state_ref_init: &mut SchedulerState = state.get_mut();
        state_ref_init.set_running(true);
        let state_clone: Rc<EngineCell<SchedulerState>> = state.clone();
        let closure_cell_clone: RafClosureCell = closure_cell.clone();
        let handler_clone: TickHandlerRc = handler.clone();
        let tasks_clone: Option<TaskRegistryRc> = tasks.map(Rc::clone);
        let input_cell_clone: Option<InputStateCell> = input_cell.map(Rc::clone);
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            {
                let state_ref: &mut SchedulerState = state_clone.get_mut();
                if !state_ref.get_running() {
                    return;
                }
                state_ref.tick(
                    &config,
                    &handler_clone,
                    tasks_clone.as_ref(),
                    input_cell_clone.as_ref(),
                );
            }
            let state_ro: &SchedulerState = state_clone.get();
            if state_ro.get_running() {
                let Some(window_value) = window() else {
                    return;
                };
                let cell: RafClosureCell = closure_cell_clone.clone();
                let Some(raf_closure) = cell.try_get() else {
                    return;
                };
                let id: i32 = window_value
                    .request_animation_frame(raf_closure.as_ref().unchecked_ref())
                    .unwrap_or_default();
                let state_ref_id: &mut SchedulerState = state_clone.get_mut();
                state_ref_id.set_raf_id(Some(id));
            }
        }));
        let Some(window_value) = window() else {
            // No window context: install the closure, mark the scheduler
            // stopped, and return an inert handle.
            let state_ref_stop: &mut SchedulerState = state.get_mut();
            state_ref_stop.set_running(false);
            let _ = closure_cell.try_set(raf_closure);
            return SchedulerHandle::new(state, closure_cell);
        };
        let id: i32 = window_value
            .request_animation_frame(raf_closure.as_ref().unchecked_ref())
            .unwrap_or_default();
        let state_ref_id: &mut SchedulerState = state.get_mut();
        state_ref_id.set_raf_id(Some(id));
        let _ = closure_cell.try_set(raf_closure);
        SchedulerHandle::new(state, closure_cell)
    }
}
