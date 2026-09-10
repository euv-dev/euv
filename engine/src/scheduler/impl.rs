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
    /// # Returns
    ///
    /// - `f64` - The current time in seconds, or `0.0` when unavailable.
    pub fn current_time() -> f64 {
        let Some(window_value) = window() else {
            return 0.0;
        };
        let Ok(performance) = Reflect::get(
            window_value.as_ref(),
            &JsValue::from_str(PERFORMANCE_OBJECT),
        ) else {
            return 0.0;
        };
        let Ok(now_method) = Reflect::get(&performance, &JsValue::from_str(PERFORMANCE_NOW_METHOD))
        else {
            return 0.0;
        };
        let now_function: js_sys::Function = now_method.unchecked_into();
        let Some(now_millis) = now_function
            .call0(&performance)
            .ok()
            .and_then(|v: JsValue| v.as_f64())
        else {
            return 0.0;
        };
        now_millis / 1000.0
    }

    /// Performs one tick of the fixed-timestep scheduler.
    ///
    /// Calculates the elapsed frame time, clamps it to `max_frame_time`, accumulates it,
    /// then runs as many fixed updates as needed. Finally, computes the interpolation
    /// factor and calls the render callback.
    ///
    /// # Arguments
    ///
    /// - `&SchedulerConfig` - The scheduler configuration.
    /// - `&TickHandlerRc` - The handler receiving update and render callbacks.
    pub fn tick(&mut self, config: &SchedulerConfig, handler: &TickHandlerRc) {
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
            *self.get_mut_accumulator() -= config.get_fixed_timestep();
            *self.get_mut_update_count() += 1;
        }
        let interpolation: f64 = self.get_accumulator() / config.get_fixed_timestep();
        handler.get_mut().on_render(interpolation);
        *self.get_mut_frame_count() += 1;
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
    ///
    /// # Returns
    ///
    /// - `SchedulerHandle` - A handle to control the running scheduler.
    pub fn start(config: SchedulerConfig, handler: TickHandlerRc) -> SchedulerHandle {
        let state: Rc<EngineCell<SchedulerState>> =
            Rc::new(EngineCell::new(SchedulerState::new(UNINITIALIZED_TIME)));
        let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
        // Install the initial closure once spawn starts.
        let state_ref_init: &mut SchedulerState = state.get_mut();
        state_ref_init.set_running(true);
        let state_clone: Rc<EngineCell<SchedulerState>> = state.clone();
        let closure_cell_clone: RafClosureCell = closure_cell.clone();
        let handler_clone: TickHandlerRc = handler.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            {
                let state_ref: &mut SchedulerState = state_clone.get_mut();
                if !state_ref.get_running() {
                    return;
                }
                state_ref.tick(&config, &handler_clone);
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
