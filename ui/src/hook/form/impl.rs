use super::*;

/// Implements [`HookContextFormExt`] for [`HookContext`].
impl HookContextFormExt for HookContext {
    /// Returns a fresh [`FormState`] bound to the current component scope.
    ///
    /// # Returns
    ///
    /// - `FormState` - A `FormState` value.
    fn form() -> FormState {
        HookContext::use_hook(|| {
            FormState::new(
                Signal::create(HashMap::new()),
                Signal::create(HashMap::new()),
                Signal::create(HashSet::new()),
                Signal::create(false),
            )
        })
    }
}

/// Inherent implementation of [`FormState`].
impl FormState {
    /// Returns the current value of the named field, or
    /// `""` if the field has never been set.
    ///
    /// This is a snapshot read, not a subscription —
    /// callers inside a render closure that want to
    /// re-render on value changes should use
    /// `state.get_values().get().get(name).cloned().unwrap_or_default()`
    /// instead, so the closure actually subscribes.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - Shared reference to a `'static str`.
    ///
    /// # Returns
    ///
    /// - `String` - A `String` value.
    pub fn field(&self, name: &'static str) -> String {
        self.get_values()
            .get()
            .get(name)
            .cloned()
            .unwrap_or_default()
    }

    /// Returns the current error for the named field, or
    /// `""` if the field has no error.
    ///
    /// Snapshot read — see `field` for the subscription
    /// caveat.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - Shared reference to a `'static str`.
    ///
    /// # Returns
    ///
    /// - `String` - A `String` value.
    pub fn error(&self, name: &'static str) -> String {
        self.get_errors()
            .get()
            .get(name)
            .cloned()
            .unwrap_or_default()
    }

    /// Returns `true` if the user has interacted with the
    /// named field.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - Field name.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the field has been touched.
    pub fn is_touched(&self, name: &'static str) -> bool {
        self.get_touched().get().contains(name)
    }

    /// Sets the value of the named field.
    ///
    /// Marks the field as touched (mirroring the
    /// `oninput` event that triggered the call) and
    /// clears any prior error for the field. The error
    /// clear is a UX choice — the next `validate` call
    /// will repopulate it if the new value is still
    /// invalid.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - Shared reference to a `'static str`.
    /// - `&str` - Shared reference to a `str`.
    pub fn set_field(&self, name: &'static str, value: &str) {
        let mut current: HashMap<&'static str, String> = self.get_values().get();
        current.insert(name, value.to_string());
        self.get_values().set(current);

        let mut touched: HashSet<&'static str> = self.get_touched().get();
        touched.insert(name);
        self.get_touched().set(touched);

        let mut errors: HashMap<&'static str, String> = self.get_errors().get();
        errors.remove(name);
        self.get_errors().set(errors);
    }

    /// Marks the named field as touched without changing
    /// its value. Used by `onblur` handlers — "the user
    /// left this field, so it counts as interacted".
    ///
    /// # Arguments
    ///
    /// - `&'static str` - Shared reference to a `'static str`.
    pub fn touch(&self, name: &'static str) {
        let mut touched: HashSet<&'static str> = self.get_touched().get();
        touched.insert(name);
        self.get_touched().set(touched);
    }

    /// Runs every validator in `validators` and updates the
    /// `errors` signal.
    ///
    /// Returns `true` if every field validated
    /// successfully (i.e. every validator returned
    /// `None`), `false` otherwise. The errors signal is
    /// always updated, regardless of return value —
    /// callers should call `validate` and then branch on
    /// the boolean.
    ///
    /// Fields with no validator are silently skipped —
    /// they cannot produce an error.
    ///
    /// # Arguments
    ///
    /// - `&HashMap<&'static str, Validator>` - Per-field validator map. Each
    ///   validator is a closure that takes the current value and returns
    ///   `Some(error_message)` or `None`.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn validate(&self, validators: &HashMap<&'static str, Validator>) -> bool {
        let values: HashMap<&'static str, String> = self.get_values().get();
        let mut next_errors: HashMap<&'static str, String> = HashMap::new();
        let mut all_valid: bool = true;
        for (name, validator) in validators.iter() {
            let current_value: &str = values.get(name).map(String::as_str).unwrap_or("");
            if let Some(error_message) = validator(current_value) {
                if !error_message.is_empty() {
                    all_valid = false;
                }
                next_errors.insert(name, error_message);
            }
        }
        self.get_errors().set(next_errors);
        all_valid
    }

    /// Runs the user-supplied submit handler if all
    /// validators pass.
    ///
    /// Sets `submitting` to `true` for the duration of the
    /// call (so a `disabled={state.get_submitting().get()}`
    /// button stays disabled until the handler returns),
    /// then resets it to `false`. If validators were
    /// supplied AND at least one field failed validation,
    /// the submit handler is NOT invoked and `submitting`
    /// is left `false`.
    ///
    /// Returns `true` if the handler was invoked,
    /// `false` if validation failed and the handler was
    /// skipped.
    ///
    /// # Arguments
    ///
    /// - `&HashMap<&'static str, Validator>` - Validators to run before
    ///   invoking the handler. Pass an empty map to skip validation entirely
    ///   (the handler always runs).
    /// - `F` - The submit handler. Receives the current values map by
    ///   reference — clone what you need to keep past the call.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn submit<F>(&self, validators: &HashMap<&'static str, Validator>, on_submit: F) -> bool
    where
        F: FnOnce(&HashMap<&'static str, String>),
    {
        let all_valid: bool = if validators.is_empty() {
            true
        } else {
            self.validate(validators)
        };
        if !all_valid {
            return false;
        }
        self.get_submitting().set(true);
        let snapshot: HashMap<&'static str, String> = self.get_values().get();
        on_submit(&snapshot);
        self.get_submitting().set(false);
        true
    }

    /// Clears values, errors, and touched state. Leaves
    /// `submitting` untouched (it should already be
    /// `false`).
    ///
    /// Useful for "form submitted successfully, reset for
    /// the next entry" UX flows.
    pub fn reset(&self) {
        self.get_values().set(HashMap::new());
        self.get_errors().set(HashMap::new());
        self.get_touched().set(HashSet::new());
    }

    /// Returns the number of fields that currently have
    /// a non-empty error. Useful for "submit button stays
    /// disabled until form is valid" without re-running
    /// validation.
    ///
    /// # Returns
    ///
    /// - `usize` - Count of currently-registered errors.
    pub fn error_count(&self) -> usize {
        self.get_errors()
            .get()
            .values()
            .filter(|message: &&String| !message.is_empty())
            .count()
    }
}
