use super::*;

/// Selector conversion and strict display helpers for `UiE2eLocator`.
impl UiE2eLocator {
    /// Converts this E2E locator into an equivalent `UiSelector`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Locator reference.
    ///
    /// # Returns
    ///
    /// - `UiSelector` - Semantic node selector.
    #[must_use]
    pub fn to_selector(&self) -> UiSelector {
        match self {
            Self::TestId(id) => UiSelector::test_id(id),
            Self::StableKey(key) => UiSelector::stable_key(key),
            Self::A11yLabel(label) => UiSelector::a11y_label(label),
            Self::Text(text) => UiSelector::text(text),
            Self::Placeholder(hint) => UiSelector::placeholder(hint),
            Self::RoleAndLabel { role, label } => {
                UiSelector::All(vec![UiSelector::role(role), UiSelector::a11y_label(label)])
            }
            Self::Selector(sel) => sel.clone(),
        }
    }

    /// Returns a human-readable descriptor for error diagnostics and secret reference normalization.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Locator reference.
    ///
    /// # Returns
    ///
    /// - `String` - Concise locator description.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::TestId(id) => id.clone(),
            Self::StableKey(key) => key.clone(),
            Self::A11yLabel(label) => label.clone(),
            Self::Text(text) => text.clone(),
            Self::Placeholder(hint) => hint.clone(),
            Self::RoleAndLabel { role, label } => format!("{role}:{label}"),
            Self::Selector(sel) => format!("{sel:?}"),
        }
    }

    /// Returns `true` when this locator's name indicates a sensitive password, secret, or token field.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Locator reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the locator identifies a sensitive field.
    #[must_use]
    pub fn indicates_secret(&self) -> bool {
        let desc: String = self.describe();
        AiSchemaProtocol::is_sensitive_identifier(&desc)
    }

    /// Resolves this locator strictly uniquely against `app`, returning `LOCATOR_NOT_FOUND` on 0 matches and `AMBIGUOUS_LOCATOR` on >1 matches.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Target locator.
    /// - `&App` - Active application runtime.
    ///
    /// # Returns
    ///
    /// - `Result<AiLayoutNode, (&'static str, String)>` - Unique matched node or `(error_code, actual_detail)`.
    pub fn resolve_unique(&self, app: &App) -> Result<AiLayoutNode, (&'static str, String)> {
        let selector: UiSelector = self.to_selector();
        let mut matches: Vec<AiLayoutNode> = AiSchemaProtocol::query_nodes_direct(app, &selector);
        if matches.is_empty() {
            return Err((E2E_ERR_LOCATOR_NOT_FOUND, E2E_MSG_ZERO_MATCHES.to_string()));
        }
        if matches.len() > 1 {
            return Err((
                E2E_ERR_AMBIGUOUS_LOCATOR,
                format!("matched {} nodes (unique locator required)", matches.len()),
            ));
        }
        match matches.pop() {
            Some(node) => Ok(node),
            None => Err((E2E_ERR_LOCATOR_NOT_FOUND, E2E_MSG_ZERO_MATCHES.to_string())),
        }
    }
}

/// Optional replacement-flow accessor for `UiE2eRetirementRecord`.
impl UiE2eRetirementRecord {
    /// Returns the optional successor flow identifier.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Retirement record reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Replacement flow ID when provided.
    #[must_use]
    pub fn replacement_flow_id_value(&self) -> Option<&str> {
        let Self {
            replacement_flow_id,
            ..
        } = self;
        replacement_flow_id.as_deref()
    }

    /// Returns `true` when this retirement record carries both non-empty approval authority and effective build metadata.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Retirement record reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` iff the retirement approval is complete and valid.
    #[must_use]
    pub fn is_approved(&self) -> bool {
        !self.get_approved_by().trim().is_empty() && !self.get_effective_build().trim().is_empty()
    }
}

/// Declarative step constructors for `UiE2eStepV1`.
impl UiE2eStepV1 {
    /// Clamps a requested timeout into the strict bounded range `[1, UI_E2E_MAX_TIMEOUT_MS]`.
    ///
    /// # Arguments
    ///
    /// - `u64` - Requested timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `u64` - Bounded timeout in milliseconds.
    #[must_use]
    pub const fn clamp_timeout_ms(timeout_ms: u64) -> u64 {
        if timeout_ms == 0 {
            UI_E2E_DEFAULT_TIMEOUT_MS
        } else if timeout_ms > UI_E2E_MAX_TIMEOUT_MS {
            UI_E2E_MAX_TIMEOUT_MS
        } else {
            timeout_ms
        }
    }

    /// Creates a `launch` step.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    ///
    /// # Returns
    ///
    /// - `Self` - Launch step.
    #[must_use]
    pub fn do_launch(id: &str) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Do {
                action: UiE2eActionKind::Launch,
                target: None,
                text: None,
                secret_ref: None,
                fixture_ref: None,
                key: None,
                dx: 0.0,
                dy: 0.0,
                width: 0,
                height: 0,
                scale: 0.0,
            },
            UI_E2E_DEFAULT_TIMEOUT_MS,
        )
    }

    /// Creates a strict unique-target `click` step.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `UiE2eLocator` - Target unique locator.
    ///
    /// # Returns
    ///
    /// - `Self` - Click step.
    #[must_use]
    pub fn do_click(id: &str, target: UiE2eLocator) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Do {
                action: UiE2eActionKind::Click,
                target: Some(target),
                text: None,
                secret_ref: None,
                fixture_ref: None,
                key: None,
                dx: 0.0,
                dy: 0.0,
                width: 0,
                height: 0,
                scale: 0.0,
            },
            UI_E2E_DEFAULT_TIMEOUT_MS,
        )
    }

    /// Creates a non-secret `type_text` step.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `UiE2eLocator` - Target input locator.
    /// - `&str` - Non-secret text payload.
    ///
    /// # Returns
    ///
    /// - `Self` - Type-text step.
    #[must_use]
    pub fn do_type_text(id: &str, target: UiE2eLocator, text: &str) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Do {
                action: UiE2eActionKind::TypeText,
                target: Some(target),
                text: Some(text.to_string()),
                secret_ref: None,
                fixture_ref: None,
                key: None,
                dx: 0.0,
                dy: 0.0,
                width: 0,
                height: 0,
                scale: 0.0,
            },
            UI_E2E_DEFAULT_TIMEOUT_MS,
        )
    }

    /// Creates a secret-referenced `type_text` step storing only `secret_ref` (never plaintext password/token).
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `UiE2eLocator` - Target input locator.
    /// - `&str` - Secret reference name (e.g. `"E2E_TEST_PASSWORD"`).
    ///
    /// # Returns
    ///
    /// - `Self` - Secret-referenced type step.
    #[must_use]
    pub fn do_type_secret(id: &str, target: UiE2eLocator, secret_ref: &str) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Do {
                action: UiE2eActionKind::TypeText,
                target: Some(target),
                text: None,
                secret_ref: Some(secret_ref.to_string()),
                fixture_ref: None,
                key: None,
                dx: 0.0,
                dy: 0.0,
                width: 0,
                height: 0,
                scale: 0.0,
            },
            UI_E2E_DEFAULT_TIMEOUT_MS,
        )
    }

    /// Creates a `scroll` step on a unique container locator.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `UiE2eLocator` - Target scrollable locator.
    /// - `f32` - Horizontal scroll delta.
    /// - `f32` - Vertical scroll delta.
    ///
    /// # Returns
    ///
    /// - `Self` - Scroll step.
    #[must_use]
    pub fn do_scroll(id: &str, target: UiE2eLocator, dx: f32, dy: f32) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Do {
                action: UiE2eActionKind::Scroll,
                target: Some(target),
                text: None,
                secret_ref: None,
                fixture_ref: None,
                key: None,
                dx,
                dy,
                width: 0,
                height: 0,
                scale: 0.0,
            },
            UI_E2E_DEFAULT_TIMEOUT_MS,
        )
    }

    /// Creates a `drag` step originating from a unique target locator by `(dx, dy)`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `UiE2eLocator` - Target draggable locator.
    /// - `f32` - Horizontal drag delta.
    /// - `f32` - Vertical drag delta.
    ///
    /// # Returns
    ///
    /// - `Self` - Drag step.
    #[must_use]
    pub fn do_drag(id: &str, target: UiE2eLocator, dx: f32, dy: f32) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Do {
                action: UiE2eActionKind::Drag,
                target: Some(target),
                text: None,
                secret_ref: None,
                fixture_ref: None,
                key: None,
                dx,
                dy,
                width: 0,
                height: 0,
                scale: 0.0,
            },
            UI_E2E_DEFAULT_TIMEOUT_MS,
        )
    }

    /// Creates a `press_key` step.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `&str` - Normalized key name.
    ///
    /// # Returns
    ///
    /// - `Self` - Key press step.
    #[must_use]
    pub fn do_press_key(id: &str, key: &str) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Do {
                action: UiE2eActionKind::PressKey,
                target: None,
                text: None,
                secret_ref: None,
                fixture_ref: None,
                key: Some(key.to_string()),
                dx: 0.0,
                dy: 0.0,
                width: 0,
                height: 0,
                scale: 0.0,
            },
            UI_E2E_DEFAULT_TIMEOUT_MS,
        )
    }

    /// Creates a bounded `wait: visible` step.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `UiE2eLocator` - Target unique locator.
    /// - `u64` - Bounded timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Wait-visible step.
    #[must_use]
    pub fn wait_visible(id: &str, target: UiE2eLocator, timeout_ms: u64) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Wait {
                condition: UiE2eWaitKind::Visible,
                target,
                expected: None,
            },
            Self::clamp_timeout_ms(timeout_ms),
        )
    }

    /// Creates a bounded `wait: actionable` step.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `UiE2eLocator` - Target unique locator.
    /// - `u64` - Bounded timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Wait-actionable step.
    #[must_use]
    pub fn wait_actionable(id: &str, target: UiE2eLocator, timeout_ms: u64) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Wait {
                condition: UiE2eWaitKind::Actionable,
                target,
                expected: None,
            },
            Self::clamp_timeout_ms(timeout_ms),
        )
    }

    /// Creates a bounded `wait: text_equals` step.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `UiE2eLocator` - Target unique locator.
    /// - `&str` - Expected text or input value.
    /// - `u64` - Bounded timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Wait-text-equals step.
    #[must_use]
    pub fn wait_text_equals(
        id: &str,
        target: UiE2eLocator,
        expected: &str,
        timeout_ms: u64,
    ) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Wait {
                condition: UiE2eWaitKind::TextEquals,
                target,
                expected: Some(expected.to_string()),
            },
            Self::clamp_timeout_ms(timeout_ms),
        )
    }

    /// Creates an `assert: visible` step.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `UiE2eLocator` - Target unique locator.
    ///
    /// # Returns
    ///
    /// - `Self` - Assert-visible step.
    #[must_use]
    pub fn assert_visible(id: &str, target: UiE2eLocator) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Assert {
                assertion: UiE2eAssertKind::Visible,
                target: Some(target),
                expected: None,
            },
            UI_E2E_DEFAULT_TIMEOUT_MS,
        )
    }

    /// Creates an `assert: not_visible` step.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `UiE2eLocator` - Target locator expected absent.
    ///
    /// # Returns
    ///
    /// - `Self` - Assert-not-visible step.
    #[must_use]
    pub fn assert_not_visible(id: &str, target: UiE2eLocator) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Assert {
                assertion: UiE2eAssertKind::NotVisible,
                target: Some(target),
                expected: None,
            },
            UI_E2E_DEFAULT_TIMEOUT_MS,
        )
    }

    /// Creates an `assert: text_equals` step.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `UiE2eLocator` - Target unique locator.
    /// - `&str` - Expected exact text value.
    ///
    /// # Returns
    ///
    /// - `Self` - Assert-text-equals step.
    #[must_use]
    pub fn assert_text_equals(id: &str, target: UiE2eLocator, expected: &str) -> Self {
        Self::new(
            id.to_string(),
            UiE2eStepOp::Assert {
                assertion: UiE2eAssertKind::TextEquals,
                target: Some(target),
                expected: Some(expected.to_string()),
            },
            UI_E2E_DEFAULT_TIMEOUT_MS,
        )
    }

    /// Returns `true` when this step is an assertion step.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Step reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether this step is `UiE2eStepOp::Assert`.
    #[must_use]
    pub fn is_assertion(&self) -> bool {
        matches!(self.get_operation(), UiE2eStepOp::Assert { .. })
    }
}

/// Flow construction, serialization, and retirement helpers for `UiE2eFlowV1`.
impl UiE2eFlowV1 {
    /// Creates a new `euv.ui-e2e.v1` flow specification.
    ///
    /// # Arguments
    ///
    /// - `&str` - Flow identifier.
    /// - `u32` - Monotonic flow revision number.
    /// - `UiE2eTargetV1` - Target viewport and adapter specification.
    /// - `Vec<UiE2eStepV1>` - Ordered flow steps.
    ///
    /// # Returns
    ///
    /// - `Self` - Versioned UI E2E flow.
    #[must_use]
    pub fn new(id: &str, revision: u32, target: UiE2eTargetV1, steps: Vec<UiE2eStepV1>) -> Self {
        Self {
            schema: UI_E2E_FLOW_SCHEMA_V1.to_string(),
            id: id.to_string(),
            revision,
            target,
            steps,
            retirement: None,
        }
    }

    /// Attaches an explicit retirement record to this flow fluently.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source flow.
    /// - `UiE2eRetirementRecord` - Approved retirement metadata.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated flow with retirement metadata.
    #[must_use]
    pub fn with_retirement(mut self, record: UiE2eRetirementRecord) -> Self {
        self.set_retirement(Some(record));
        self
    }

    /// Returns the optional retirement approval record attached to this flow.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Flow reference.
    ///
    /// # Returns
    ///
    /// - `Option<&UiE2eRetirementRecord>` - Retirement record if configured.
    #[must_use]
    pub fn retirement_record(&self) -> Option<&UiE2eRetirementRecord> {
        let Self { retirement, .. } = self;
        retirement.as_ref()
    }

    /// Serializes this flow to clean pretty-printed JSON.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Flow reference.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>` - Pretty JSON string or serialization error.
    pub fn to_json_pretty(&self) -> Result<String, String> {
        to_string_pretty(self).map_err(|err: Error| err.to_string())
    }

    /// Deserializes a `UiE2eFlowV1` from JSON and validates its schema version.
    ///
    /// # Arguments
    ///
    /// - `&str` - Raw JSON flow text.
    ///
    /// # Returns
    ///
    /// - `Result<Self, String>` - Parsed flow or validation error.
    pub fn from_json(raw: &str) -> Result<Self, String> {
        let parsed: Self = from_str(raw).map_err(|err: Error| err.to_string())?;
        if parsed.get_schema() != UI_E2E_FLOW_SCHEMA_V1 {
            return Err(format!(
                "{E2E_ERR_UNSUPPORTED_SCHEMA}: {}",
                parsed.get_schema()
            ));
        }
        Ok(parsed)
    }
}

/// Optional field accessors for `UiE2eStepResultV1`.
impl UiE2eStepResultV1 {
    /// Creates a passing step result bound to optional same-frame metadata.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `u64` - Step duration in milliseconds.
    /// - `Option<u64>` - Captured frame identifier.
    /// - `Option<u64>` - Captured scene revision.
    ///
    /// # Returns
    ///
    /// - `Self` - Passing step result.
    #[must_use]
    pub fn passed(
        id: &str,
        duration_ms: u64,
        frame_id: Option<u64>,
        scene_revision: Option<u64>,
    ) -> Self {
        Self {
            id: id.to_string(),
            status: UiE2eStepStatus::Passed,
            code: None,
            expected: None,
            actual: None,
            duration_ms,
            frame_id,
            scene_revision,
        }
    }

    /// Creates a failing step result carrying a stable error code, expected/actual diagnostics, and same-frame metadata.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    /// - `&str` - Stable machine-readable error code.
    /// - `String` - Expected condition summary.
    /// - `String` - Observed actual state summary.
    /// - `u64` - Step duration in milliseconds.
    /// - `Option<u64>` - Captured frame identifier.
    /// - `Option<u64>` - Captured scene revision.
    ///
    /// # Returns
    ///
    /// - `Self` - Failing step result.
    #[must_use]
    pub fn failed(
        id: &str,
        code: &str,
        expected: String,
        actual: String,
        duration_ms: u64,
        frame_id: Option<u64>,
        scene_revision: Option<u64>,
    ) -> Self {
        Self {
            id: id.to_string(),
            status: UiE2eStepStatus::Failed,
            code: Some(code.to_string()),
            expected: Some(expected),
            actual: Some(actual),
            duration_ms,
            frame_id,
            scene_revision,
        }
    }

    /// Creates a `NotRun` step result for trailing steps skipped after an earlier failure or retirement.
    ///
    /// # Arguments
    ///
    /// - `&str` - Step identifier.
    ///
    /// # Returns
    ///
    /// - `Self` - Not-run step result.
    #[must_use]
    pub fn not_run(id: &str) -> Self {
        Self {
            id: id.to_string(),
            status: UiE2eStepStatus::NotRun,
            code: None,
            expected: None,
            actual: None,
            duration_ms: 0,
            frame_id: None,
            scene_revision: None,
        }
    }

    /// Returns the optional error code slice.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Step result reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Error code when failed.
    #[must_use]
    pub fn code_value(&self) -> Option<&str> {
        let Self { code, .. } = self;
        code.as_deref()
    }

    /// Returns the optional expected condition description.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Step result reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Expected summary when failed.
    #[must_use]
    pub fn expected_value(&self) -> Option<&str> {
        let Self { expected, .. } = self;
        expected.as_deref()
    }

    /// Returns the optional observed actual state description.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Step result reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Observed summary when failed.
    #[must_use]
    pub fn actual_value(&self) -> Option<&str> {
        let Self { actual, .. } = self;
        actual.as_deref()
    }

    /// Returns the optional captured `frame_id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Step result reference.
    ///
    /// # Returns
    ///
    /// - `Option<u64>` - Frame identifier when captured.
    #[must_use]
    pub fn frame_id_value(&self) -> Option<u64> {
        let Self { frame_id, .. } = self;
        *frame_id
    }

    /// Returns the optional captured `scene_revision`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Step result reference.
    ///
    /// # Returns
    ///
    /// - `Option<u64>` - Scene revision when captured.
    #[must_use]
    pub fn scene_revision_value(&self) -> Option<u64> {
        let Self { scene_revision, .. } = self;
        *scene_revision
    }
}

/// Optional field accessors for `UiE2eAttemptRecord`.
impl UiE2eAttemptRecord {
    /// Returns the optional first failing step ID for this attempt.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Attempt record reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Step ID if this attempt failed.
    #[must_use]
    pub fn first_failure_step_id_value(&self) -> Option<&str> {
        let Self {
            first_failure_step_id,
            ..
        } = self;
        first_failure_step_id.as_deref()
    }

    /// Returns the optional first failing error code for this attempt.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Attempt record reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Error code if this attempt failed.
    #[must_use]
    pub fn first_failure_code_value(&self) -> Option<&str> {
        let Self {
            first_failure_code, ..
        } = self;
        first_failure_code.as_deref()
    }
}

/// Assertion-preservation guard and candidate verification for `UiE2eMigrationProposal`.
impl UiE2eMigrationProposal {
    /// Returns the verdict observed when replaying the candidate migrated flow.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Migration proposal reference.
    ///
    /// # Returns
    ///
    /// - `Option<UiE2eVerdict>` - Candidate rerun verdict when executed.
    #[must_use]
    pub fn candidate_rerun_verdict_value(&self) -> Option<UiE2eVerdict> {
        let Self {
            candidate_rerun_verdict,
            ..
        } = self;
        *candidate_rerun_verdict
    }

    /// Verifies that `candidate` preserves every assertion step and expected outcome from `original` identically.
    ///
    /// # Arguments
    ///
    /// - `&UiE2eFlowV1` - Original flow before locator migration.
    /// - `&UiE2eFlowV1` - Candidate migrated flow.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - `Ok(())` when all assertions are unchanged, or `ASSERTION_MUTATION_FORBIDDEN`.
    pub fn validate_preserves_assertions(
        original: &UiE2eFlowV1,
        candidate: &UiE2eFlowV1,
    ) -> Result<(), String> {
        let orig_asserts: Vec<&UiE2eStepV1> = original
            .get_steps()
            .iter()
            .filter(|s: &&UiE2eStepV1| s.is_assertion())
            .collect();
        let cand_asserts: Vec<&UiE2eStepV1> = candidate
            .get_steps()
            .iter()
            .filter(|s: &&UiE2eStepV1| s.is_assertion())
            .collect();
        if orig_asserts.len() != cand_asserts.len() {
            return Err(format!(
                "{E2E_ERR_ASSERTION_MUTATION_FORBIDDEN}: {E2E_MSG_ASSERTION_MUTATED}"
            ));
        }
        for (orig, cand) in orig_asserts.iter().zip(cand_asserts.iter()) {
            if orig.get_id() != cand.get_id() || orig.get_operation() != cand.get_operation() {
                return Err(format!(
                    "{E2E_ERR_ASSERTION_MUTATION_FORBIDDEN}: step '{}' changed assertion",
                    orig.get_id()
                ));
            }
        }
        Ok(())
    }

    /// Applies locator-only `changes` to `original`, enforces `assertions_changed == false`, and verifies the candidate flow on `candidate_app`.
    ///
    /// # Arguments
    ///
    /// - `&UiE2eFlowV1` - Original flow whose locator broke.
    /// - `&str` - Previous build tag.
    /// - `&str` - New build tag.
    /// - `Vec<UiE2eLocatorChange>` - Proposed locator replacements.
    /// - `&mut App` - Clean application instance on `new_build` for candidate verification.
    /// - `&str` - Run identifier for candidate rerun.
    ///
    /// # Returns
    ///
    /// - `Result<(Self, UiE2eFlowV1), String>` - Verified migration proposal (`assertions_changed == false`) and candidate flow.
    pub fn propose_and_verify(
        original: &UiE2eFlowV1,
        old_build: &str,
        new_build: &str,
        changes: Vec<UiE2eLocatorChange>,
        candidate_app: &mut App,
        candidate_run_id: &str,
    ) -> Result<(Self, UiE2eFlowV1), String> {
        let mut candidate: UiE2eFlowV1 = original.clone();
        let next_rev: u32 = original.get_revision().saturating_add(1);
        candidate.set_revision(next_rev);
        for change in changes.iter() {
            let Some(step) = candidate
                .get_mut_steps()
                .iter_mut()
                .find(|s: &&mut UiE2eStepV1| s.get_id() == change.get_step_id())
            else {
                return Err(format!(
                    "{E2E_ERR_LOCATOR_NOT_FOUND}: step '{}' not found in flow",
                    change.get_step_id()
                ));
            };
            if step.is_assertion() {
                return Err(format!(
                    "{E2E_ERR_ASSERTION_MUTATION_FORBIDDEN}: refusing to alter assertion step '{}'",
                    step.get_id()
                ));
            }
            match step.get_mut_operation() {
                UiE2eStepOp::Do { target, .. } => {
                    *target = Some(change.get_new_locator().clone());
                }
                UiE2eStepOp::Wait { target, .. } => {
                    *target = change.get_new_locator().clone();
                }
                UiE2eStepOp::Assert { .. } => {
                    return Err(format!(
                        "{E2E_ERR_ASSERTION_MUTATION_FORBIDDEN}: {E2E_MSG_ASSERTION_MUTATED}"
                    ));
                }
            }
        }
        Self::validate_preserves_assertions(original, &candidate)?;
        let replayer: UiE2eReplayer = UiE2eReplayer::for_build(new_build);
        let rerun: UiE2eResultV1 =
            replayer.replay_once(candidate_app, &candidate, candidate_run_id);
        let rerun_verdict: UiE2eVerdict = rerun.get_verdict();
        let proposal: Self = Self {
            schema: UI_E2E_MIGRATION_SCHEMA_V1.to_string(),
            flow_id: original.get_id().clone(),
            from_revision: original.get_revision(),
            old_build: old_build.to_string(),
            new_build: new_build.to_string(),
            status: UiE2eMigrationStatus::Proposed,
            assertions_changed: false,
            changes,
            candidate_rerun_verdict: Some(rerun_verdict),
        };
        Ok((proposal, candidate))
    }
}

/// Optional failure and migration accessors for `UiE2eResultV1`.
impl UiE2eResultV1 {
    /// Returns the preserved first failing step record (even when a retry produced `UiE2eVerdict::Flaky`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Verdict result reference.
    ///
    /// # Returns
    ///
    /// - `Option<&UiE2eStepResultV1>` - Preserved initial failure step record.
    #[must_use]
    pub fn first_failure_record(&self) -> Option<&UiE2eStepResultV1> {
        let Self { first_failure, .. } = self;
        first_failure.as_ref()
    }

    /// Returns the optional attached locator migration proposal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Verdict result reference.
    ///
    /// # Returns
    ///
    /// - `Option<&UiE2eMigrationProposal>` - Migration proposal when present.
    #[must_use]
    pub fn migration_proposal(&self) -> Option<&UiE2eMigrationProposal> {
        let Self { migration, .. } = self;
        migration.as_ref()
    }

    /// Attaches a locator migration proposal without mutating the original failed verdict or failure history.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable verdict result.
    /// - `UiE2eMigrationProposal` - Verified proposal with `assertions_changed == false`.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - `Ok(())` when `assertions_changed == false`, error otherwise.
    pub fn attach_migration(&mut self, proposal: UiE2eMigrationProposal) -> Result<(), String> {
        if proposal.get_assertions_changed() {
            return Err(format!(
                "{E2E_ERR_ASSERTION_MUTATION_FORBIDDEN}: {E2E_MSG_ASSERTION_MUTATED}"
            ));
        }
        self.set_migration(Some(proposal));
        Ok(())
    }

    /// Serializes this `euv.ui-e2e.result.v1` verdict artifact into pretty JSON.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Verdict result reference.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>` - Pretty JSON string or error.
    pub fn to_json_pretty(&self) -> Result<String, String> {
        to_string_pretty(self).map_err(|err: Error| err.to_string())
    }
}

/// Semantic action recording, gesture coalescing, secret redaction, and JSON export for `UiE2eRecorder`.
impl UiE2eRecorder {
    /// Creates a new `UiE2eRecorder` targeting `flow_id`, `revision`, and `target`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Stable flow identifier.
    /// - `u32` - Initial flow revision.
    /// - `UiE2eTargetV1` - Target viewport and adapter specification.
    ///
    /// # Returns
    ///
    /// - `Self` - Empty recorder ready to capture semantic UI actions.
    #[must_use]
    pub fn new(flow_id: &str, revision: u32, target: UiE2eTargetV1) -> Self {
        Self {
            flow_id: flow_id.to_string(),
            revision,
            target,
            steps: Vec::new(),
            next_seq: 1,
            redacted_count: 0,
            incomplete: false,
        }
    }

    /// Allocates the next deterministic step identifier for `prefix`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recorder.
    /// - `&str` - Step prefix label.
    ///
    /// # Returns
    ///
    /// - `String` - Unique step ID (`"{prefix}_{seq}"`).
    fn next_step_id(&mut self, prefix: &str) -> String {
        let seq: u32 = self.get_next_seq();
        self.set_next_seq(seq.saturating_add(1));
        format!("{prefix}_{seq}")
    }

    /// Normalizes a locator descriptor into an uppercase environment-style secret reference identifier.
    ///
    /// # Arguments
    ///
    /// - `&UiE2eLocator` - Sensitive input locator.
    ///
    /// # Returns
    ///
    /// - `String` - Uppercase `E2E_SECRET_*` placeholder name.
    #[must_use]
    fn secret_ref_for(target: &UiE2eLocator) -> String {
        let desc: String = target.describe();
        let mut suffix: String = String::with_capacity(desc.len());
        for ch in desc.chars() {
            if ch.is_ascii_alphanumeric() {
                suffix.push(ch.to_ascii_uppercase());
            } else {
                suffix.push('_');
            }
        }
        format!("{E2E_SECRET_REF_PREFIX}{suffix}")
    }

    /// Records a semantic click on `target`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recorder.
    /// - `UiE2eLocator` - Target unique locator.
    pub fn record_click(&mut self, target: UiE2eLocator) {
        let id: String = self.next_step_id(E2E_STEP_PREFIX_CLICK);
        self.get_mut_steps()
            .push(UiE2eStepV1::do_click(&id, target));
    }

    /// Records text entry into `target`, automatically replacing password or sensitive inputs with a `secret_ref` placeholder so secrets never enter the flow.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recorder.
    /// - `UiE2eLocator` - Target input locator.
    /// - `&str` - Entered UTF-8 text (never persisted when `is_secret` or `target.indicates_secret()` is true).
    /// - `bool` - Explicit sensitivity/password flag from the target input node.
    pub fn record_type_text(&mut self, target: UiE2eLocator, text: &str, is_secret: bool) {
        let id: String = self.next_step_id(E2E_STEP_PREFIX_TYPE);
        if is_secret || target.indicates_secret() {
            let secret_ref: String = Self::secret_ref_for(&target);
            let next_redacted: usize = self.get_redacted_count().saturating_add(1);
            self.set_redacted_count(next_redacted);
            self.get_mut_steps()
                .push(UiE2eStepV1::do_type_secret(&id, target, &secret_ref));
        } else {
            self.get_mut_steps()
                .push(UiE2eStepV1::do_type_text(&id, target, text));
        }
    }

    /// Records a scroll delta on `target`, coalescing consecutive scroll events targeting the same container into one step.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recorder.
    /// - `UiE2eLocator` - Target scroll container locator.
    /// - `f32` - Horizontal scroll delta.
    /// - `f32` - Vertical scroll delta.
    pub fn record_scroll(&mut self, target: UiE2eLocator, dx: f32, dy: f32) {
        if let Some(last) = self.get_mut_steps().last_mut()
            && let UiE2eStepOp::Do {
                action: UiE2eActionKind::Scroll,
                target: Some(prev_target),
                dx: prev_dx,
                dy: prev_dy,
                ..
            } = last.get_mut_operation()
            && *prev_target == target
        {
            *prev_dx += dx;
            *prev_dy += dy;
            return;
        }
        let id: String = self.next_step_id(E2E_STEP_PREFIX_SCROLL);
        self.get_mut_steps()
            .push(UiE2eStepV1::do_scroll(&id, target, dx, dy));
    }

    /// Records a pointer drag movement on `target`, coalescing consecutive drag moves targeting the same node into one gesture step.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recorder.
    /// - `UiE2eLocator` - Target draggable locator.
    /// - `f32` - Horizontal drag displacement delta.
    /// - `f32` - Vertical drag displacement delta.
    pub fn record_drag_move(&mut self, target: UiE2eLocator, dx: f32, dy: f32) {
        if let Some(last) = self.get_mut_steps().last_mut()
            && let UiE2eStepOp::Do {
                action: UiE2eActionKind::Drag,
                target: Some(prev_target),
                dx: prev_dx,
                dy: prev_dy,
                ..
            } = last.get_mut_operation()
            && *prev_target == target
        {
            *prev_dx += dx;
            *prev_dy += dy;
            return;
        }
        let id: String = self.next_step_id(E2E_STEP_PREFIX_DRAG);
        self.get_mut_steps()
            .push(UiE2eStepV1::do_drag(&id, target, dx, dy));
    }

    /// Records a keyboard key press step.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recorder.
    /// - `&str` - Key name string.
    pub fn record_key_press(&mut self, key: &str) {
        let id: String = self.next_step_id(E2E_STEP_PREFIX_KEY);
        self.get_mut_steps()
            .push(UiE2eStepV1::do_press_key(&id, key));
    }

    /// Explicitly ignores idle user pause durations so recordings never turn human thinking pauses into brittle arbitrary sleeps.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recorder.
    /// - `u64` - Observed idle pause duration in milliseconds (intentionally ignored).
    pub fn record_idle_pause(&mut self, _pause_ms: u64) {}

    /// Records an observable bounded wait condition.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recorder.
    /// - `UiE2eLocator` - Target unique locator.
    /// - `u64` - Bounded timeout in milliseconds.
    pub fn record_wait_visible(&mut self, target: UiE2eLocator, timeout_ms: u64) {
        let id: String = self.next_step_id(E2E_STEP_PREFIX_WAIT);
        self.get_mut_steps()
            .push(UiE2eStepV1::wait_visible(&id, target, timeout_ms));
    }

    /// Records an explicit text equality assertion.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recorder.
    /// - `UiE2eLocator` - Target unique locator.
    /// - `&str` - Expected text value.
    pub fn record_assert_text_equals(&mut self, target: UiE2eLocator, expected: &str) {
        let id: String = self.next_step_id(E2E_STEP_PREFIX_ASSERT);
        self.get_mut_steps()
            .push(UiE2eStepV1::assert_text_equals(&id, target, expected));
    }

    /// Marks this recording as incomplete when an input event queue gap occurred.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recorder.
    pub fn mark_event_gap(&mut self) {
        self.set_incomplete(true);
    }

    /// Exports the recorded semantic steps into a versioned `UiE2eFlowV1`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Recorder reference.
    ///
    /// # Returns
    ///
    /// - `UiE2eFlowV1` - Clean executable E2E flow.
    #[must_use]
    pub fn export_flow(&self) -> UiE2eFlowV1 {
        UiE2eFlowV1::new(
            self.get_flow_id(),
            self.get_revision(),
            self.get_target().clone(),
            self.get_steps().clone(),
        )
    }

    /// Exports the recorded semantic flow directly to pretty-printed `euv.ui-e2e.v1` JSON.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Recorder reference.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>` - Clean JSON string or serialization error.
    pub fn export_json(&self) -> Result<String, String> {
        self.export_flow().to_json_pretty()
    }
}

/// Deterministic `euv.ui-e2e.v1` execution, bounded retry classification, and verdict reporting for `UiE2eReplayer`.
impl UiE2eReplayer {
    /// Creates a replayer targeting `app_build` with zero retries and bounded headless wait frames (`8`).
    ///
    /// # Arguments
    ///
    /// - `&str` - Application build tag.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready UI E2E replayer.
    #[must_use]
    pub fn for_build(app_build: &str) -> Self {
        Self {
            app_build: app_build.to_string(),
            max_retries: 0,
            max_wait_frames: 8,
            secrets: HashMap::new(),
            fixtures: HashMap::new(),
            supported_adapters: Vec::new(),
            blocked_reason: None,
        }
    }

    /// Configures the maximum bounded clean retry count for transient `WAIT_TIMEOUT` failures (clamped to `UI_E2E_MAX_RETRIES`).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source replayer.
    /// - `u32` - Maximum retries (`0..=3`).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated replayer.
    #[must_use]
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.set_max_retries(retries.min(UI_E2E_MAX_RETRIES));
        self
    }

    /// Configures maximum frames stepped while waiting on a `Wait` step in deterministic execution.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source replayer.
    /// - `usize` - Maximum wait frames (`>= 1`).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated replayer.
    #[must_use]
    pub fn with_max_wait_frames(mut self, frames: usize) -> Self {
        self.set_max_wait_frames(frames.max(1));
        self
    }

    /// Registers an in-memory test secret for `secret_ref` lookup during replay.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source replayer.
    /// - `&str` - Secret reference name.
    /// - `&str` - Secret value supplied strictly at runtime.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated replayer.
    #[must_use]
    pub fn with_secret(mut self, key: &str, value: &str) -> Self {
        let _: Option<String> = self
            .get_mut_secrets()
            .insert(key.to_string(), value.to_string());
        self
    }

    /// Registers an in-memory test fixture value for `fixture_ref` lookup during replay.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source replayer.
    /// - `&str` - Fixture reference name.
    /// - `&str` - Fixture value.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated replayer.
    #[must_use]
    pub fn with_fixture(mut self, key: &str, value: &str) -> Self {
        let _: Option<String> = self
            .get_mut_fixtures()
            .insert(key.to_string(), value.to_string());
        self
    }

    /// Restricts execution to the given list of supported adapter identifiers.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source replayer.
    /// - `Vec<String>` - Supported adapter names (e.g. `vec!["headless".into()]`).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated replayer.
    #[must_use]
    pub fn with_supported_adapters(mut self, adapters: Vec<String>) -> Self {
        self.set_supported_adapters(adapters);
        self
    }

    /// Marks the runtime environment as blocked with `reason`, causing runs to report `UiE2eVerdict::Blocked`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source replayer.
    /// - `&str` - Blocker diagnostic reason.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated replayer.
    #[must_use]
    pub fn with_blocked_reason(mut self, reason: &str) -> Self {
        self.set_blocked_reason(Some(reason.to_string()));
        self
    }

    /// Returns the optional environment blocker reason.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Replayer reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Blocker diagnostic slice.
    #[must_use]
    fn blocked_reason_value(&self) -> Option<&str> {
        let Self { blocked_reason, .. } = self;
        blocked_reason.as_deref()
    }

    /// Captures the current `(frame_id, scene_revision)` boundary from `app`.
    ///
    /// # Arguments
    ///
    /// - `&App` - Active application runtime.
    ///
    /// # Returns
    ///
    /// - `(Option<u64>, Option<u64>)` - `(Some(frame_id), Some(scene_revision))`.
    #[must_use]
    fn capture_frame_pair(app: &App) -> (Option<u64>, Option<u64>) {
        let frame_id: u64 = app.get_renderer().get_last_stats().get_frame_index();
        let rev: u64 = app
            .get_scene()
            .with_read(SceneTree::get_revision, app.get_last_rendered_revision());
        (Some(frame_id), Some(rev))
    }

    /// Executes a single `UiE2eStepV1` against `app` and returns its structured `UiE2eStepResultV1`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Replayer reference.
    /// - `&mut App` - Active application runtime.
    /// - `&UiE2eStepV1` - Step to execute.
    ///
    /// # Returns
    ///
    /// - `UiE2eStepResultV1` - Step pass/fail result with error code and same-frame metadata.
    fn execute_step(&self, app: &mut App, step: &UiE2eStepV1) -> UiE2eStepResultV1 {
        #[cfg(not(target_arch = "wasm32"))]
        let started: Instant = Instant::now();
        let outcome: Result<(), (&'static str, String, String)> = match step.get_operation() {
            UiE2eStepOp::Do { .. } => self.execute_do_step(app, step.get_operation()),
            UiE2eStepOp::Wait {
                condition,
                target,
                expected,
            } => self.execute_wait_step(app, *condition, target, expected.as_deref()),
            UiE2eStepOp::Assert {
                assertion,
                target,
                expected,
            } => self.execute_assert_step(app, *assertion, target.as_ref(), expected.as_deref()),
        };
        #[cfg(not(target_arch = "wasm32"))]
        let duration_ms: u64 = started.elapsed().as_millis() as u64;
        #[cfg(target_arch = "wasm32")]
        let duration_ms: u64 = 0;
        let (frame_id, scene_rev): (Option<u64>, Option<u64>) = Self::capture_frame_pair(app);
        match outcome {
            Ok(()) => UiE2eStepResultV1::passed(step.get_id(), duration_ms, frame_id, scene_rev),
            Err((code, expected, actual)) => UiE2eStepResultV1::failed(
                step.get_id(),
                code,
                expected,
                actual,
                duration_ms,
                frame_id,
                scene_rev,
            ),
        }
    }

    /// Executes a `UiE2eStepOp::Do` action enforcing strict unique-locator matching.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Replayer reference.
    /// - `&mut App` - Active application runtime.
    /// - `&UiE2eStepOp` - Declarative `Do` step operation payload.
    ///
    /// # Returns
    ///
    /// - `Result<(), (&'static str, String, String)>` - `Ok(())` or `(error_code, expected, actual)`.
    fn execute_do_step(
        &self,
        app: &mut App,
        step_op: &UiE2eStepOp,
    ) -> Result<(), (&'static str, String, String)> {
        let UiE2eStepOp::Do {
            action,
            target,
            text,
            secret_ref,
            fixture_ref,
            key,
            dx,
            dy,
            width,
            height,
            scale,
        } = step_op
        else {
            return Ok(());
        };
        match *action {
            UiE2eActionKind::Launch => UiAutomator::new(app)
                .step()
                .map(|_: FrameStats| ())
                .map_err(|err: AutomationError| {
                    (
                        E2E_ERR_RUNTIME_ERROR,
                        E2E_MSG_LAUNCH_STEP.to_string(),
                        err.to_string(),
                    )
                }),
            UiE2eActionKind::PressKey => {
                let key_name: &str = key.as_deref().unwrap_or("");
                UiAutomator::new(app)
                    .press_key_name(key_name)
                    .map(|_: FrameStats| ())
                    .map_err(|err: AutomationError| {
                        (
                            E2E_ERR_RUNTIME_ERROR,
                            format!("press key {key_name}"),
                            err.to_string(),
                        )
                    })
            }
            UiE2eActionKind::Resize => UiAutomator::new(app)
                .resize(*width, *height, scale.max(1.0))
                .map(|_: FrameStats| ())
                .map_err(|err: AutomationError| {
                    (
                        E2E_ERR_RUNTIME_ERROR,
                        format!("resize to {width}x{height}"),
                        err.to_string(),
                    )
                }),
            UiE2eActionKind::Click
            | UiE2eActionKind::DoubleClick
            | UiE2eActionKind::Hover
            | UiE2eActionKind::Focus
            | UiE2eActionKind::TypeText
            | UiE2eActionKind::ClearInput
            | UiE2eActionKind::Scroll
            | UiE2eActionKind::Drag => {
                let Some(loc) = target.as_ref() else {
                    return Err((
                        E2E_ERR_LOCATOR_NOT_FOUND,
                        E2E_MSG_MISSING_TARGET.to_string(),
                        E2E_MSG_ZERO_MATCHES.to_string(),
                    ));
                };
                let node: AiLayoutNode =
                    loc.resolve_unique(app)
                        .map_err(|(code, actual): (&'static str, String)| {
                            (code, format!("one unique node: {}", loc.describe()), actual)
                        })?;
                let center: Point = node.get_screen_bounds().center();
                let mut auto: UiAutomator<'_> = UiAutomator::new(app);
                let res: Result<FrameStats, AutomationError> = match *action {
                    UiE2eActionKind::Click | UiE2eActionKind::Focus => {
                        auto.click_at(center.get_x(), center.get_y())
                    }
                    UiE2eActionKind::DoubleClick => {
                        let _: FrameStats = auto.click_at(center.get_x(), center.get_y()).map_err(
                            |err: AutomationError| {
                                (E2E_ERR_RUNTIME_ERROR, loc.describe(), err.to_string())
                            },
                        )?;
                        auto.click_at(center.get_x(), center.get_y())
                    }
                    UiE2eActionKind::Hover => {
                        auto.hover(&UiSelector::ByNodeIndex(node.get_node_index()))
                    }
                    UiE2eActionKind::TypeText => {
                        let payload: String = if let Some(sref) = secret_ref.as_deref() {
                            self.get_secrets().get(sref).cloned().ok_or_else(|| {
                                (
                                    E2E_ERR_ENVIRONMENT_BLOCKED,
                                    format!("secret_ref:{sref}"),
                                    E2E_MSG_MISSING_SECRET.to_string(),
                                )
                            })?
                        } else if let Some(fref) = fixture_ref.as_deref() {
                            self.get_fixtures().get(fref).cloned().ok_or_else(|| {
                                (
                                    E2E_ERR_ENVIRONMENT_BLOCKED,
                                    format!("fixture_ref:{fref}"),
                                    E2E_MSG_MISSING_FIXTURE.to_string(),
                                )
                            })?
                        } else {
                            text.as_deref().unwrap_or("").to_string()
                        };
                        auto.type_text(&UiSelector::ByNodeIndex(node.get_node_index()), &payload)
                    }
                    UiE2eActionKind::ClearInput => {
                        auto.clear_input(&UiSelector::ByNodeIndex(node.get_node_index()))
                    }
                    UiE2eActionKind::Scroll => {
                        auto.scroll(&UiSelector::ByNodeIndex(node.get_node_index()), *dx, *dy)
                    }
                    UiE2eActionKind::Drag => auto.drag_points(
                        center.get_x(),
                        center.get_y(),
                        center.get_x() + *dx,
                        center.get_y() + *dy,
                    ),
                    _ => auto.step(),
                };
                res.map(|_: FrameStats| ()).map_err(|err: AutomationError| {
                    (E2E_ERR_RUNTIME_ERROR, loc.describe(), err.to_string())
                })
            }
        }
    }

    /// Evaluates a bounded `UiE2eStepOp::Wait` condition up to `max_wait_frames` frames.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Replayer reference.
    /// - `&mut App` - Active application runtime.
    /// - `UiE2eWaitKind` - Observable wait predicate.
    /// - `&UiE2eLocator` - Target unique locator.
    /// - `Option<&str>` - Optional expected text value.
    ///
    /// # Returns
    ///
    /// - `Result<(), (&'static str, String, String)>` - `Ok(())` when satisfied, `WAIT_TIMEOUT` or `AMBIGUOUS_LOCATOR` otherwise.
    fn execute_wait_step(
        &self,
        app: &mut App,
        condition: UiE2eWaitKind,
        target: &UiE2eLocator,
        expected: Option<&str>,
    ) -> Result<(), (&'static str, String, String)> {
        let mut last_actual: String = E2E_MSG_ZERO_MATCHES.to_string();
        for frame_idx in 0..=self.get_max_wait_frames() {
            match condition {
                UiE2eWaitKind::Visible => match target.resolve_unique(app) {
                    Ok(_) => return Ok(()),
                    Err((E2E_ERR_AMBIGUOUS_LOCATOR, detail)) => {
                        return Err((
                            E2E_ERR_AMBIGUOUS_LOCATOR,
                            format!("one visible node: {}", target.describe()),
                            detail,
                        ));
                    }
                    Err((_, detail)) => {
                        last_actual = detail;
                    }
                },
                UiE2eWaitKind::NotVisible => {
                    let found: Vec<AiLayoutNode> =
                        AiSchemaProtocol::query_nodes_direct(app, &target.to_selector());
                    if found.is_empty() {
                        return Ok(());
                    }
                    last_actual = format!("{} visible match(es)", found.len());
                }
                UiE2eWaitKind::Actionable => match target.resolve_unique(app) {
                    Ok(node) if node.get_interactive() && !node.get_disabled() => return Ok(()),
                    Ok(_) => {
                        last_actual = E2E_MSG_NOT_ACTIONABLE.to_string();
                    }
                    Err((E2E_ERR_AMBIGUOUS_LOCATOR, detail)) => {
                        return Err((
                            E2E_ERR_AMBIGUOUS_LOCATOR,
                            format!("one actionable node: {}", target.describe()),
                            detail,
                        ));
                    }
                    Err((_, detail)) => {
                        last_actual = detail;
                    }
                },
                UiE2eWaitKind::TextEquals => {
                    let want: &str = expected.unwrap_or("");
                    match target.resolve_unique(app) {
                        Ok(node) => {
                            let got: Option<&str> =
                                node.text_value().or_else(|| node.input_value());
                            if got == Some(want) {
                                return Ok(());
                            }
                            last_actual = format!("text={got:?}");
                        }
                        Err((E2E_ERR_AMBIGUOUS_LOCATOR, detail)) => {
                            return Err((
                                E2E_ERR_AMBIGUOUS_LOCATOR,
                                format!("unique node with text {want:?}"),
                                detail,
                            ));
                        }
                        Err((_, detail)) => {
                            last_actual = detail;
                        }
                    }
                }
            }
            if frame_idx < self.get_max_wait_frames() {
                let _: FrameStats =
                    UiAutomator::new(app)
                        .step()
                        .map_err(|err: AutomationError| {
                            (E2E_ERR_RUNTIME_ERROR, target.describe(), err.to_string())
                        })?;
            }
        }
        Err((
            E2E_ERR_WAIT_TIMEOUT,
            format!("wait {:?} on {}", condition, target.describe()),
            last_actual,
        ))
    }

    /// Evaluates a `UiE2eStepOp::Assert` predicate strictly.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Replayer reference.
    /// - `&mut App` - Active application runtime.
    /// - `UiE2eAssertKind` - Assertion predicate.
    /// - `Option<&UiE2eLocator>` - Optional target locator.
    /// - `Option<&str>` - Optional expected text value.
    ///
    /// # Returns
    ///
    /// - `Result<(), (&'static str, String, String)>` - `Ok(())` or structured failure details.
    fn execute_assert_step(
        &self,
        app: &mut App,
        assertion: UiE2eAssertKind,
        target: Option<&UiE2eLocator>,
        expected: Option<&str>,
    ) -> Result<(), (&'static str, String, String)> {
        match assertion {
            UiE2eAssertKind::NoHorizontalOverflow => UiAutomator::new(app)
                .assert_no_horizontal_overflow()
                .map_err(|err: AutomationError| {
                    (
                        E2E_ERR_ASSERTION_FAILED,
                        E2E_MSG_ZERO_OVERFLOW.to_string(),
                        err.to_string(),
                    )
                }),
            UiE2eAssertKind::NoTextOverlap => UiAutomator::new(app)
                .assert_no_text_overlap()
                .map_err(|err: AutomationError| {
                    (
                        E2E_ERR_ASSERTION_FAILED,
                        E2E_MSG_ZERO_OVERLAP.to_string(),
                        err.to_string(),
                    )
                }),
            UiE2eAssertKind::Visible => {
                let Some(loc) = target else {
                    return Err((
                        E2E_ERR_LOCATOR_NOT_FOUND,
                        E2E_MSG_MISSING_TARGET.to_string(),
                        E2E_MSG_ZERO_MATCHES.to_string(),
                    ));
                };
                loc.resolve_unique(app).map(|_: AiLayoutNode| ()).map_err(
                    |(code, actual): (&'static str, String)| {
                        (
                            code,
                            format!("one visible node: {}", loc.describe()),
                            actual,
                        )
                    },
                )
            }
            UiE2eAssertKind::NotVisible => {
                let Some(loc) = target else {
                    return Err((
                        E2E_ERR_LOCATOR_NOT_FOUND,
                        E2E_MSG_MISSING_TARGET.to_string(),
                        E2E_MSG_ZERO_MATCHES.to_string(),
                    ));
                };
                let found: Vec<AiLayoutNode> =
                    AiSchemaProtocol::query_nodes_direct(app, &loc.to_selector());
                if found.is_empty() {
                    Ok(())
                } else {
                    Err((
                        E2E_ERR_ASSERTION_FAILED,
                        format!("absent node: {}", loc.describe()),
                        format!("{} ({} match)", E2E_MSG_NODE_STILL_VISIBLE, found.len()),
                    ))
                }
            }
            UiE2eAssertKind::TextEquals => {
                let Some(loc) = target else {
                    return Err((
                        E2E_ERR_LOCATOR_NOT_FOUND,
                        E2E_MSG_MISSING_TARGET.to_string(),
                        E2E_MSG_ZERO_MATCHES.to_string(),
                    ));
                };
                let want: &str = expected.unwrap_or("");
                let node: AiLayoutNode =
                    loc.resolve_unique(app)
                        .map_err(|(code, actual): (&'static str, String)| {
                            (code, format!("one unique node: {}", loc.describe()), actual)
                        })?;
                let got: Option<&str> = node.text_value().or_else(|| node.input_value());
                if got == Some(want) {
                    Ok(())
                } else {
                    Err((
                        E2E_ERR_ASSERTION_FAILED,
                        format!("text == {want:?}"),
                        format!("text == {got:?}"),
                    ))
                }
            }
        }
    }

    /// Builds a short-circuited `UiE2eResultV1` where every step is marked `NotRun` (used for `Retired`, `Blocked`, or `Unsupported`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Replayer reference.
    /// - `&UiE2eFlowV1` - Target flow.
    /// - `&str` - Run identifier.
    /// - `UiE2eVerdict` - Pre-execution verdict.
    /// - `&str` - Diagnostic code.
    /// - `String` - Diagnostic detail.
    ///
    /// # Returns
    ///
    /// - `UiE2eResultV1` - Short-circuited verdict artifact.
    #[must_use]
    fn build_non_executed_result(
        &self,
        flow: &UiE2eFlowV1,
        run_id: &str,
        verdict: UiE2eVerdict,
        code: &str,
        detail: String,
    ) -> UiE2eResultV1 {
        let steps: Vec<UiE2eStepResultV1> = flow
            .get_steps()
            .iter()
            .map(|s: &UiE2eStepV1| UiE2eStepResultV1::not_run(s.get_id()))
            .collect();
        let attempt: UiE2eAttemptRecord = UiE2eAttemptRecord {
            attempt: 1,
            verdict,
            steps: steps.clone(),
            first_failure_step_id: None,
            first_failure_code: Some(code.to_string()),
        };
        let first_failure: Option<UiE2eStepResultV1> = if verdict == UiE2eVerdict::Retired {
            None
        } else {
            Some(UiE2eStepResultV1::failed(
                flow.get_id(),
                code,
                flow.get_schema().clone(),
                detail,
                0,
                None,
                None,
            ))
        };
        UiE2eResultV1 {
            schema: UI_E2E_RESULT_SCHEMA_V1.to_string(),
            run_id: run_id.to_string(),
            flow_id: flow.get_id().clone(),
            flow_revision: flow.get_revision(),
            app_build: self.get_app_build().clone(),
            verdict,
            steps,
            attempts: vec![attempt],
            first_failure,
            migration: None,
        }
    }

    /// Runs a single attempt of `flow` against `app`, returning `NotRun` for every step following the first failure.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Replayer reference.
    /// - `&mut App` - Active application runtime.
    /// - `&UiE2eFlowV1` - Flow specification to execute.
    /// - `u32` - One-based attempt number.
    ///
    /// # Returns
    ///
    /// - `(UiE2eAttemptRecord, Option<UiE2eStepResultV1>)` - Attempt record and optional first failing step record.
    fn run_single_attempt(
        &self,
        app: &mut App,
        flow: &UiE2eFlowV1,
        attempt_no: u32,
    ) -> (UiE2eAttemptRecord, Option<UiE2eStepResultV1>) {
        let mut step_results: Vec<UiE2eStepResultV1> = Vec::with_capacity(flow.get_steps().len());
        let mut first_fail: Option<UiE2eStepResultV1> = None;
        for step in flow.get_steps().iter() {
            if first_fail.is_some() {
                step_results.push(UiE2eStepResultV1::not_run(step.get_id()));
                continue;
            }
            let res: UiE2eStepResultV1 = self.execute_step(app, step);
            if res.get_status() == UiE2eStepStatus::Failed {
                first_fail = Some(res.clone());
            }
            step_results.push(res);
        }
        let verdict: UiE2eVerdict = if first_fail.is_some() {
            UiE2eVerdict::Failed
        } else {
            UiE2eVerdict::Passed
        };
        let record: UiE2eAttemptRecord = UiE2eAttemptRecord {
            attempt: attempt_no,
            verdict,
            steps: step_results,
            first_failure_step_id: first_fail
                .as_ref()
                .map(|r: &UiE2eStepResultV1| r.get_id().clone()),
            first_failure_code: first_fail
                .as_ref()
                .and_then(UiE2eStepResultV1::code_value)
                .map(str::to_string),
        };
        (record, first_fail)
    }

    /// Evaluates pre-execution flow gates (`schema`, `retirement`, `blocked`, `adapter`) before touching the runtime.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Replayer reference.
    /// - `&UiE2eFlowV1` - Target flow specification.
    /// - `&str` - Run identifier.
    ///
    /// # Returns
    ///
    /// - `Option<UiE2eResultV1>` - Early verdict artifact when pre-execution classification applies.
    #[must_use]
    fn check_preconditions(&self, flow: &UiE2eFlowV1, run_id: &str) -> Option<UiE2eResultV1> {
        if flow.get_schema() != UI_E2E_FLOW_SCHEMA_V1 {
            return Some(self.build_non_executed_result(
                flow,
                run_id,
                UiE2eVerdict::Unsupported,
                E2E_ERR_UNSUPPORTED_SCHEMA,
                format!("{E2E_MSG_UNSUPPORTED_SCHEMA}: {}", flow.get_schema()),
            ));
        }
        if let Some(ret) = flow.retirement_record()
            && ret.is_approved()
        {
            return Some(self.build_non_executed_result(
                flow,
                run_id,
                UiE2eVerdict::Retired,
                E2E_ERR_FLOW_RETIRED,
                ret.get_reason().clone(),
            ));
        }
        if let Some(reason) = self.blocked_reason_value() {
            return Some(self.build_non_executed_result(
                flow,
                run_id,
                UiE2eVerdict::Blocked,
                E2E_ERR_ENVIRONMENT_BLOCKED,
                reason.to_string(),
            ));
        }
        let target_adapter: &str = if flow.get_target().get_adapter().trim().is_empty() {
            E2E_DEFAULT_ADAPTER
        } else {
            flow.get_target().get_adapter().as_str()
        };
        if !self.get_supported_adapters().is_empty()
            && !self
                .get_supported_adapters()
                .iter()
                .any(|a: &String| a.eq_ignore_ascii_case(target_adapter))
        {
            return Some(self.build_non_executed_result(
                flow,
                run_id,
                UiE2eVerdict::Unsupported,
                E2E_ERR_UNSUPPORTED_ADAPTER,
                format!("unsupported adapter: {target_adapter}"),
            ));
        }
        None
    }

    /// Replays `flow` once against `app` and produces an `euv.ui-e2e.result.v1` verdict artifact.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Replayer reference.
    /// - `&mut App` - Active application runtime.
    /// - `&UiE2eFlowV1` - Versioned UI E2E flow.
    /// - `&str` - Run identifier.
    ///
    /// # Returns
    ///
    /// - `UiE2eResultV1` - Structured verdict artifact.
    pub fn replay_once(&self, app: &mut App, flow: &UiE2eFlowV1, run_id: &str) -> UiE2eResultV1 {
        if let Some(early) = self.check_preconditions(flow, run_id) {
            return early;
        }
        let (attempt, first_fail): (UiE2eAttemptRecord, Option<UiE2eStepResultV1>) =
            self.run_single_attempt(app, flow, 1);
        let verdict: UiE2eVerdict = attempt.get_verdict();
        let steps: Vec<UiE2eStepResultV1> = attempt.get_steps().clone();
        UiE2eResultV1 {
            schema: UI_E2E_RESULT_SCHEMA_V1.to_string(),
            run_id: run_id.to_string(),
            flow_id: flow.get_id().clone(),
            flow_revision: flow.get_revision(),
            app_build: self.get_app_build().clone(),
            verdict,
            steps,
            attempts: vec![attempt],
            first_failure: first_fail,
            migration: None,
        }
    }

    /// Replays `flow` with bounded clean retries strictly for transient `WAIT_TIMEOUT` failures, classifying recovered runs as `Flaky` while preserving attempt #1's failure record.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Replayer reference.
    /// - `&UiE2eFlowV1` - Versioned UI E2E flow.
    /// - `&str` - Run identifier.
    /// - `F` - Clean factory creating a fresh `App` for each 1-based attempt number.
    ///
    /// # Returns
    ///
    /// - `UiE2eResultV1` - Final verdict artifact preserving complete bounded retry history and initial failure.
    pub fn replay_with_clean_retries<F>(
        &self,
        flow: &UiE2eFlowV1,
        run_id: &str,
        mut build_clean_app: F,
    ) -> UiE2eResultV1
    where
        F: FnMut(u32) -> App,
    {
        if let Some(early) = self.check_preconditions(flow, run_id) {
            return early;
        }
        let mut first_app: App = build_clean_app(1);
        let (attempt_one, initial_fail): (UiE2eAttemptRecord, Option<UiE2eStepResultV1>) =
            self.run_single_attempt(&mut first_app, flow, 1);
        let primary_steps: Vec<UiE2eStepResultV1> = attempt_one.get_steps().clone();
        let mut attempts: Vec<UiE2eAttemptRecord> = vec![attempt_one];
        if initial_fail.is_none() {
            return UiE2eResultV1 {
                schema: UI_E2E_RESULT_SCHEMA_V1.to_string(),
                run_id: run_id.to_string(),
                flow_id: flow.get_id().clone(),
                flow_revision: flow.get_revision(),
                app_build: self.get_app_build().clone(),
                verdict: UiE2eVerdict::Passed,
                steps: primary_steps,
                attempts,
                first_failure: None,
                migration: None,
            };
        }
        let is_transient: bool = initial_fail
            .as_ref()
            .and_then(UiE2eStepResultV1::code_value)
            == Some(E2E_ERR_WAIT_TIMEOUT);
        let mut final_verdict: UiE2eVerdict = UiE2eVerdict::Failed;
        if is_transient && self.get_max_retries() > 0 {
            let retries: u32 = self.get_max_retries().min(UI_E2E_MAX_RETRIES);
            for retry_idx in 1..=retries {
                let attempt_no: u32 = 1 + retry_idx;
                let mut retry_app: App = build_clean_app(attempt_no);
                let (retry_rec, retry_fail): (UiE2eAttemptRecord, Option<UiE2eStepResultV1>) =
                    self.run_single_attempt(&mut retry_app, flow, attempt_no);
                attempts.push(retry_rec);
                if retry_fail.is_none() {
                    final_verdict = UiE2eVerdict::Flaky;
                    break;
                }
            }
        }
        UiE2eResultV1 {
            schema: UI_E2E_RESULT_SCHEMA_V1.to_string(),
            run_id: run_id.to_string(),
            flow_id: flow.get_id().clone(),
            flow_revision: flow.get_revision(),
            app_build: self.get_app_build().clone(),
            verdict: final_verdict,
            steps: primary_steps,
            attempts,
            first_failure: initial_fail,
            migration: None,
        }
    }
}
