use super::*;

/// Formatting implementation for `DynamicUpdateError`.
impl fmt::Display for DynamicUpdateError {
    /// Formats a human-readable dynamic update failure message.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Error variant reference.
    /// - `&mut Formatter<'_>` - Output formatter.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Formatting result.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTargetId => write!(f, "dynamic update target ID or route cannot be empty"),
            Self::ChecksumMismatch {
                target,
                expected,
                actual,
            } => write!(
                f,
                "checksum mismatch for `{target}`: expected {expected:#018x}, computed {actual:#018x}"
            ),
            Self::VersionRegression {
                target,
                current,
                incoming,
            } => write!(
                f,
                "version regression for `{target}`: active v{current}, incoming v{incoming}"
            ),
            Self::InvalidJson(msg) => write!(f, "invalid dynamic update JSON manifest: {msg}"),
            Self::NoHistoryToRollback(target) => {
                write!(
                    f,
                    "no dynamic update history available to rollback for `{target}`"
                )
            }
        }
    }
}

/// Standard `Error` marker for `DynamicUpdateError`.
impl Error for DynamicUpdateError {}

/// Style accessor and `Style` conversion helpers for `DynamicStyleSpec`.
impl DynamicStyleSpec {
    /// Returns the optional explicit pixel width.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Style spec reference.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Width in logical pixels if set.
    #[must_use]
    pub fn width_px(&self) -> Option<f32> {
        let Self { width, .. } = *self;
        width
    }

    /// Returns the optional explicit pixel height.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Style spec reference.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Height in logical pixels if set.
    #[must_use]
    pub fn height_px(&self) -> Option<f32> {
        let Self { height, .. } = *self;
        height
    }

    /// Returns the optional uniform padding in logical pixels.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Style spec reference.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Padding in logical pixels if set.
    #[must_use]
    pub fn padding_px(&self) -> Option<f32> {
        let Self { padding, .. } = *self;
        padding
    }

    /// Returns the optional child gap in logical pixels.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Style spec reference.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Gap in logical pixels if set.
    #[must_use]
    pub fn gap_px(&self) -> Option<f32> {
        let Self { gap, .. } = *self;
        gap
    }

    /// Returns the optional uniform corner radius in logical pixels.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Style spec reference.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Corner radius in logical pixels if set.
    #[must_use]
    pub fn radius_px(&self) -> Option<f32> {
        let Self { radius, .. } = *self;
        radius
    }

    /// Returns the optional font size in logical pixels.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Style spec reference.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Font size in logical pixels if set.
    #[must_use]
    pub fn font_size_px(&self) -> Option<f32> {
        let Self { font_size, .. } = *self;
        font_size
    }

    /// Returns the optional 24-bit RGB hex background fill color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Style spec reference.
    ///
    /// # Returns
    ///
    /// - `Option<u32>` - RGB hex color if set.
    #[must_use]
    pub fn background_hex_value(&self) -> Option<u32> {
        let Self { background_hex, .. } = *self;
        background_hex
    }

    /// Returns the optional 24-bit RGB hex text foreground color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Style spec reference.
    ///
    /// # Returns
    ///
    /// - `Option<u32>` - RGB hex color if set.
    #[must_use]
    pub fn color_hex_value(&self) -> Option<u32> {
        let Self { color_hex, .. } = *self;
        color_hex
    }

    /// Returns the optional border stroke width in logical pixels.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Style spec reference.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Border width if set.
    #[must_use]
    pub fn border_width_px(&self) -> Option<f32> {
        let Self { border_width, .. } = *self;
        border_width
    }

    /// Returns the optional 24-bit RGB hex border stroke color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Style spec reference.
    ///
    /// # Returns
    ///
    /// - `Option<u32>` - RGB hex border color if set.
    #[must_use]
    pub fn border_hex_value(&self) -> Option<u32> {
        let Self { border_hex, .. } = *self;
        border_hex
    }

    /// Applies all configured style overrides onto `base` and returns the resolved native `Style`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Style override specification.
    /// - `Style` - Base native style for the node kind.
    ///
    /// # Returns
    ///
    /// - `Style` - Compiled native style.
    #[must_use]
    pub fn apply_to(&self, mut base: Style) -> Style {
        if self.get_full_width() {
            base = base.width(Length::Percent(100.0));
        }
        if let Some(w) = self.width_px() {
            base = base.width(Length::Px(w));
        }
        if let Some(h) = self.height_px() {
            base = base.height(Length::Px(h));
        }
        if let Some(p) = self.padding_px() {
            base = base.padding(Edges::all(p));
        }
        if let Some(g) = self.gap_px() {
            base = base.gap(g);
        }
        if let Some(r) = self.radius_px() {
            base = base.radius(r);
        }
        if let Some(sz) = self.font_size_px() {
            base = base.font_size(sz);
        }
        if self.get_bold() {
            base = base.font_weight(FontWeight::Bold);
        }
        if let Some(bg) = self.background_hex_value() {
            base = base.background(Color::hex(bg));
        }
        if let Some(fg) = self.color_hex_value() {
            base = base.color(Color::hex(fg));
        }
        if let (Some(bw), Some(bc)) = (self.border_width_px(), self.border_hex_value()) {
            base = base.border(bw, Color::hex(bc));
        }
        if self.get_flex_grow() > 0.0 {
            base = base.flex_grow(self.get_flex_grow());
        }
        base
    }
}

/// Declarative builders and deterministic checksum helpers for `DynamicNodeSpec`.
impl DynamicNodeSpec {
    /// Creates a vertical flexbox column node specification.
    ///
    /// # Returns
    ///
    /// - `Self` - Column specification.
    #[must_use]
    pub fn column() -> Self {
        Self::new(DynamicNodeKind::Column)
    }

    /// Creates a horizontal flexbox row node specification.
    ///
    /// # Returns
    ///
    /// - `Self` - Row specification.
    #[must_use]
    pub fn row() -> Self {
        Self::new(DynamicNodeKind::Row)
    }

    /// Creates a grid container node specification with `columns` equal-width columns.
    ///
    /// # Arguments
    ///
    /// - `u16` - Number of grid columns.
    ///
    /// # Returns
    ///
    /// - `Self` - Grid specification.
    #[must_use]
    pub fn grid(columns: u16) -> Self {
        let mut node: Self = Self::new(DynamicNodeKind::Grid);
        node.get_mut_style().set_columns(columns.max(1));
        node
    }

    /// Creates a styled card surface node specification with an optional title.
    ///
    /// # Arguments
    ///
    /// - `String` - Card heading text.
    ///
    /// # Returns
    ///
    /// - `Self` - Card specification.
    #[must_use]
    pub fn card(title: String) -> Self {
        let mut node: Self = Self::new(DynamicNodeKind::Card);
        node.set_text(title);
        node
    }

    /// Creates a native text node specification.
    ///
    /// # Arguments
    ///
    /// - `String` - UTF-8 text content.
    ///
    /// # Returns
    ///
    /// - `Self` - Text specification.
    #[must_use]
    pub fn text(content: String) -> Self {
        let mut node: Self = Self::new(DynamicNodeKind::Text);
        node.set_text(content);
        node
    }

    /// Creates an interactive button node specification dispatching `action` when clicked.
    ///
    /// # Arguments
    ///
    /// - `String` - Visible button label.
    /// - `String` - Action identifier emitted as `DynamicActionEvent`.
    ///
    /// # Returns
    ///
    /// - `Self` - Interactive button specification.
    #[must_use]
    pub fn button(label: String, action: String) -> Self {
        let mut node: Self = Self::new(DynamicNodeKind::Button);
        node.set_text(label).set_action(Some(action));
        node
    }

    /// Creates a compact pill badge node specification.
    ///
    /// # Arguments
    ///
    /// - `String` - Badge label text.
    ///
    /// # Returns
    ///
    /// - `Self` - Badge specification.
    #[must_use]
    pub fn badge(label: String) -> Self {
        let mut node: Self = Self::new(DynamicNodeKind::Badge);
        node.set_text(label);
        node
    }

    /// Creates a horizontal hairline divider node specification.
    ///
    /// # Returns
    ///
    /// - `Self` - Divider specification.
    #[must_use]
    pub fn divider() -> Self {
        Self::new(DynamicNodeKind::Divider)
    }

    /// Returns the optional action identifier attached to this node.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Node specification reference.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Action identifier if interactive.
    #[must_use]
    pub fn action_name(&self) -> Option<&str> {
        let Self { action, .. } = self;
        action.as_deref()
    }

    /// Appends a child `DynamicNodeSpec` and returns `self` for fluent composition.
    ///
    /// # Arguments
    ///
    /// - `Self` - Parent specification.
    /// - `Self` - Child specification to append.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated parent specification.
    #[must_use]
    pub fn child(mut self, child: Self) -> Self {
        self.get_mut_children().push(child);
        self
    }

    /// Computes the deterministic cross-platform 64-bit FNV-1a checksum of this node tree.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Node specification reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Deterministic checksum digest.
    #[must_use]
    pub fn checksum(&self) -> u64 {
        compute_spec_checksum(self)
    }
}

/// Signing and JSON serialization helpers for `DynamicComponentUpdate`.
impl DynamicComponentUpdate {
    /// Computes and stores the deterministic FNV-1a checksum of `self.spec`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable component update.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference for chaining.
    pub fn sign(&mut self) -> &mut Self {
        let digest: u64 = self.get_spec().checksum();
        self.set_checksum(digest);
        self
    }

    /// Serializes this component update to JSON for OTA / remote delivery.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component update reference.
    ///
    /// # Returns
    ///
    /// - `Result<String, DynamicUpdateError>` - JSON manifest string.
    pub fn to_json(&self) -> Result<String, DynamicUpdateError> {
        match to_string(self) {
            Ok(json) => Ok(json),
            Err(err) => Err(DynamicUpdateError::InvalidJson(err.to_string())),
        }
    }

    /// Deserializes a `DynamicComponentUpdate` from a JSON string.
    ///
    /// # Arguments
    ///
    /// - `&str` - JSON payload.
    ///
    /// # Returns
    ///
    /// - `Result<Self, DynamicUpdateError>` - Parsed component update.
    pub fn from_json(raw: &str) -> Result<Self, DynamicUpdateError> {
        match from_str(raw) {
            Ok(parsed) => Ok(parsed),
            Err(err) => Err(DynamicUpdateError::InvalidJson(err.to_string())),
        }
    }
}

/// Signing and JSON serialization helpers for `DynamicPageUpdate`.
impl DynamicPageUpdate {
    /// Computes and stores the deterministic FNV-1a checksum of `self.spec`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable page update.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference for chaining.
    pub fn sign(&mut self) -> &mut Self {
        let digest: u64 = self.get_spec().checksum();
        self.set_checksum(digest);
        self
    }

    /// Serializes this page update to JSON for OTA / remote delivery.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Page update reference.
    ///
    /// # Returns
    ///
    /// - `Result<String, DynamicUpdateError>` - JSON manifest string.
    pub fn to_json(&self) -> Result<String, DynamicUpdateError> {
        match to_string(self) {
            Ok(json) => Ok(json),
            Err(err) => Err(DynamicUpdateError::InvalidJson(err.to_string())),
        }
    }

    /// Deserializes a `DynamicPageUpdate` from a JSON string.
    ///
    /// # Arguments
    ///
    /// - `&str` - JSON payload.
    ///
    /// # Returns
    ///
    /// - `Result<Self, DynamicUpdateError>` - Parsed page update.
    pub fn from_json(raw: &str) -> Result<Self, DynamicUpdateError> {
        match from_str(raw) {
            Ok(parsed) => Ok(parsed),
            Err(err) => Err(DynamicUpdateError::InvalidJson(err.to_string())),
        }
    }
}

/// Builder and JSON manifest helpers for `DynamicUpdateBundle`.
impl DynamicUpdateBundle {
    /// Appends a component-level update to this atomic bundle.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source bundle.
    /// - `DynamicComponentUpdate` - Component update entry.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated bundle.
    #[must_use]
    pub fn push_component(mut self, update: DynamicComponentUpdate) -> Self {
        self.get_mut_components().push(update);
        self
    }

    /// Appends a page-level update to this atomic bundle.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source bundle.
    /// - `DynamicPageUpdate` - Page update entry.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated bundle.
    #[must_use]
    pub fn push_page(mut self, update: DynamicPageUpdate) -> Self {
        self.get_mut_pages().push(update);
        self
    }

    /// Serializes the complete bundle manifest to pretty-printed JSON.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Bundle reference.
    ///
    /// # Returns
    ///
    /// - `Result<String, DynamicUpdateError>` - JSON manifest string.
    pub fn to_json(&self) -> Result<String, DynamicUpdateError> {
        match to_string_pretty(self) {
            Ok(json) => Ok(json),
            Err(err) => Err(DynamicUpdateError::InvalidJson(err.to_string())),
        }
    }

    /// Deserializes a `DynamicUpdateBundle` from a JSON manifest string.
    ///
    /// # Arguments
    ///
    /// - `&str` - JSON bundle manifest.
    ///
    /// # Returns
    ///
    /// - `Result<Self, DynamicUpdateError>` - Parsed bundle manifest.
    pub fn from_json(raw: &str) -> Result<Self, DynamicUpdateError> {
        match from_str(raw) {
            Ok(parsed) => Ok(parsed),
            Err(err) => Err(DynamicUpdateError::InvalidJson(err.to_string())),
        }
    }
}

/// Default empty `DynamicUpdateHub`.
impl Default for DynamicUpdateHub {
    /// Creates an empty `DynamicUpdateHub` backed by fresh reactive signals.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized hub.
    fn default() -> Self {
        Self::new()
    }
}

/// Component-level, page-level, and atomic bundle dynamic update orchestration on `DynamicUpdateHub`.
impl DynamicUpdateHub {
    /// Creates a new `DynamicUpdateHub` with empty component/page registries and zero telemetry.
    ///
    /// # Returns
    ///
    /// - `Self` - Shared dynamic update hub handle.
    #[must_use]
    pub fn new() -> Self {
        let state: DynamicHubState = DynamicHubState {
            components: HashMap::new(),
            component_signals: HashMap::new(),
            pages: HashMap::new(),
            page_signals: HashMap::new(),
            routes_signal: Signal::new(0),
            telemetry_signal: Signal::new(DynamicTelemetry::default()),
            last_action: Signal::new(None),
            action_handlers: Vec::new(),
        };
        Self {
            inner: Rc::new(RefCell::new(state)),
        }
    }

    /// Applies a validated component-level dynamic update and surgically re-renders only that component slot.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `DynamicComponentUpdate` - Incoming versioned component update.
    ///
    /// # Returns
    ///
    /// - `Result<u64, DynamicUpdateError>` - Newly active component version on success.
    pub fn apply_component(
        &self,
        update: DynamicComponentUpdate,
    ) -> Result<u64, DynamicUpdateError> {
        let id: String = update.get_id().trim().to_string();
        if id.is_empty() {
            self.record_rejection();
            return Err(DynamicUpdateError::EmptyTargetId);
        }
        let actual_checksum: u64 = update.get_spec().checksum();
        let expected_checksum: u64 = update.get_checksum();
        if expected_checksum != 0 && expected_checksum != actual_checksum {
            self.record_rejection();
            return Err(DynamicUpdateError::ChecksumMismatch {
                target: id,
                expected: expected_checksum,
                actual: actual_checksum,
            });
        }
        let incoming_ver: u64 = update.get_version();
        let allow_downgrade: bool = update.get_allow_downgrade();
        let rev_sig_opt: Result<
            (Signal<u64>, Signal<DynamicTelemetry>, DynamicTelemetry),
            DynamicUpdateError,
        > = self.with_state_mut(|state: &mut DynamicHubState| {
            let active_ver: u64 = state
                .components
                .get(&id)
                .and_then(|stack: &Vec<DynamicSlotRecord>| stack.last())
                .map(|rec: &DynamicSlotRecord| rec.version)
                .unwrap_or(0);
            if !allow_downgrade && incoming_ver <= active_ver {
                return Err(DynamicUpdateError::VersionRegression {
                    target: id.clone(),
                    current: active_ver,
                    incoming: incoming_ver,
                });
            }
            let record: DynamicSlotRecord = DynamicSlotRecord {
                version: incoming_ver,
                title: String::new(),
                checksum: actual_checksum,
                payload: DynamicPayload::Spec(update.spec),
            };
            state.components.entry(id.clone()).or_default().push(record);
            let sig: Signal<u64> = *state
                .component_signals
                .entry(id)
                .or_insert_with(|| Signal::new(0));
            let mut tel: DynamicTelemetry = state.telemetry_signal.get_untracked();
            tel.set_components_applied(tel.get_components_applied().saturating_add(1));
            Ok((sig, state.telemetry_signal, tel))
        });
        match rev_sig_opt {
            Ok((sig, tel_sig, tel)) => {
                batch(|| {
                    let _: bool = sig.update(|v: &mut u64| *v = v.wrapping_add(1));
                    let _: bool = tel_sig.set(tel);
                });
                Ok(incoming_ver)
            }
            Err(err) => {
                self.record_rejection();
                Err(err)
            }
        }
    }

    /// Parses and applies a JSON-encoded component-level dynamic update manifest.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - JSON string encoding a `DynamicComponentUpdate`.
    ///
    /// # Returns
    ///
    /// - `Result<u64, DynamicUpdateError>` - Newly active component version on success.
    pub fn apply_component_json(&self, json: &str) -> Result<u64, DynamicUpdateError> {
        let update: DynamicComponentUpdate = match DynamicComponentUpdate::from_json(json) {
            Ok(parsed) => parsed,
            Err(err) => {
                self.record_rejection();
                return Err(err);
            }
        };
        self.apply_component(update)
    }

    /// Hot-swaps a component slot with a native Rust builder closure without rebuilding sibling UI.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `String` - Target component slot ID.
    /// - `u64` - Monotonic version number.
    /// - `F` - Zero-context native Rust UI builder closure.
    ///
    /// # Returns
    ///
    /// - `Result<u64, DynamicUpdateError>` - Newly active component version on success.
    pub fn swap_component_builder<F>(
        &self,
        id: String,
        version: u64,
        builder: F,
    ) -> Result<u64, DynamicUpdateError>
    where
        F: Fn() -> NodeId + 'static,
    {
        let trimmed: String = id.trim().to_string();
        if trimmed.is_empty() {
            self.record_rejection();
            return Err(DynamicUpdateError::EmptyTargetId);
        }
        let builder_rc: DynamicBuilderFn = Rc::new(builder);
        let res: Result<
            (Signal<u64>, Signal<DynamicTelemetry>, DynamicTelemetry),
            DynamicUpdateError,
        > = self.with_state_mut(|state: &mut DynamicHubState| {
            let active_ver: u64 = state
                .components
                .get(&trimmed)
                .and_then(|stack: &Vec<DynamicSlotRecord>| stack.last())
                .map(|rec: &DynamicSlotRecord| rec.version)
                .unwrap_or(0);
            if version <= active_ver {
                return Err(DynamicUpdateError::VersionRegression {
                    target: trimmed.clone(),
                    current: active_ver,
                    incoming: version,
                });
            }
            let record: DynamicSlotRecord = DynamicSlotRecord {
                version,
                title: String::new(),
                checksum: 0,
                payload: DynamicPayload::Native(builder_rc),
            };
            state
                .components
                .entry(trimmed.clone())
                .or_default()
                .push(record);
            let sig: Signal<u64> = *state
                .component_signals
                .entry(trimmed)
                .or_insert_with(|| Signal::new(0));
            let mut tel: DynamicTelemetry = state.telemetry_signal.get_untracked();
            tel.set_components_applied(tel.get_components_applied().saturating_add(1));
            Ok((sig, state.telemetry_signal, tel))
        });
        match res {
            Ok((sig, tel_sig, tel)) => {
                batch(|| {
                    let _: bool = sig.update(|v: &mut u64| *v = v.wrapping_add(1));
                    let _: bool = tel_sig.set(tel);
                });
                Ok(version)
            }
            Err(err) => {
                self.record_rejection();
                Err(err)
            }
        }
    }

    /// Rolls back a component slot to its previous revision (or `0` compiled fallback when the stack empties).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - Target component slot ID.
    ///
    /// # Returns
    ///
    /// - `Result<u64, DynamicUpdateError>` - Restored active version (`0` means compiled fallback).
    pub fn rollback_component(&self, id: &str) -> Result<u64, DynamicUpdateError> {
        let key: String = id.trim().to_string();
        let res: Result<
            (u64, Signal<u64>, Signal<DynamicTelemetry>, DynamicTelemetry),
            DynamicUpdateError,
        > = self.with_state_mut(|state: &mut DynamicHubState| {
            let popped: bool = state
                .components
                .get_mut(&key)
                .and_then(|stack: &mut Vec<DynamicSlotRecord>| stack.pop())
                .is_some();
            if !popped {
                return Err(DynamicUpdateError::NoHistoryToRollback(key.clone()));
            }
            let next_ver: u64 = state
                .components
                .get(&key)
                .and_then(|stack: &Vec<DynamicSlotRecord>| stack.last())
                .map(|rec: &DynamicSlotRecord| rec.version)
                .unwrap_or(0);
            let sig: Signal<u64> = *state
                .component_signals
                .entry(key)
                .or_insert_with(|| Signal::new(0));
            let mut tel: DynamicTelemetry = state.telemetry_signal.get_untracked();
            tel.set_rollbacks(tel.get_rollbacks().saturating_add(1));
            Ok((next_ver, sig, state.telemetry_signal, tel))
        });
        let (next_ver, sig, tel_sig, tel): (
            u64,
            Signal<u64>,
            Signal<DynamicTelemetry>,
            DynamicTelemetry,
        ) = res?;
        batch(|| {
            let _: bool = sig.update(|v: &mut u64| *v = v.wrapping_add(1));
            let _: bool = tel_sig.set(tel);
        });
        Ok(next_ver)
    }

    /// Applies a validated page-level dynamic update (overriding an existing route or injecting a new one).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `DynamicPageUpdate` - Incoming versioned page update.
    ///
    /// # Returns
    ///
    /// - `Result<u64, DynamicUpdateError>` - Newly active page version on success.
    pub fn apply_page(&self, update: DynamicPageUpdate) -> Result<u64, DynamicUpdateError> {
        let route: String = normalize_dynamic_route(update.get_route());
        if route.is_empty() {
            self.record_rejection();
            return Err(DynamicUpdateError::EmptyTargetId);
        }
        let actual_checksum: u64 = update.get_spec().checksum();
        let expected_checksum: u64 = update.get_checksum();
        if expected_checksum != 0 && expected_checksum != actual_checksum {
            self.record_rejection();
            return Err(DynamicUpdateError::ChecksumMismatch {
                target: route,
                expected: expected_checksum,
                actual: actual_checksum,
            });
        }
        let incoming_ver: u64 = update.get_version();
        let allow_downgrade: bool = update.get_allow_downgrade();
        let res: Result<PageApplyCommit, DynamicUpdateError> =
            self.with_state_mut(|state: &mut DynamicHubState| {
                let is_new_route: bool = state
                    .pages
                    .get(&route)
                    .map(|stack: &Vec<DynamicSlotRecord>| stack.is_empty())
                    .unwrap_or(true);
                let active_ver: u64 = state
                    .pages
                    .get(&route)
                    .and_then(|stack: &Vec<DynamicSlotRecord>| stack.last())
                    .map(|rec: &DynamicSlotRecord| rec.version)
                    .unwrap_or(0);
                if !allow_downgrade && incoming_ver <= active_ver {
                    return Err(DynamicUpdateError::VersionRegression {
                        target: route.clone(),
                        current: active_ver,
                        incoming: incoming_ver,
                    });
                }
                let record: DynamicSlotRecord = DynamicSlotRecord {
                    version: incoming_ver,
                    title: update.title,
                    checksum: actual_checksum,
                    payload: DynamicPayload::Spec(update.spec),
                };
                state.pages.entry(route.clone()).or_default().push(record);
                let page_sig: Signal<u64> = *state
                    .page_signals
                    .entry(route)
                    .or_insert_with(|| Signal::new(0));
                let mut tel: DynamicTelemetry = state.telemetry_signal.get_untracked();
                tel.set_pages_applied(tel.get_pages_applied().saturating_add(1));
                Ok((
                    page_sig,
                    state.routes_signal,
                    state.telemetry_signal,
                    tel,
                    is_new_route,
                ))
            });
        match res {
            Ok((page_sig, routes_sig, tel_sig, tel, is_new_route)) => {
                batch(|| {
                    let _: bool = page_sig.update(|v: &mut u64| *v = v.wrapping_add(1));
                    if is_new_route {
                        let _: bool = routes_sig.update(|v: &mut u64| *v = v.wrapping_add(1));
                    }
                    let _: bool = tel_sig.set(tel);
                });
                Ok(incoming_ver)
            }
            Err(err) => {
                self.record_rejection();
                Err(err)
            }
        }
    }

    /// Parses and applies a JSON-encoded page-level dynamic update manifest.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - JSON string encoding a `DynamicPageUpdate`.
    ///
    /// # Returns
    ///
    /// - `Result<u64, DynamicUpdateError>` - Newly active page version on success.
    pub fn apply_page_json(&self, json: &str) -> Result<u64, DynamicUpdateError> {
        let update: DynamicPageUpdate = match DynamicPageUpdate::from_json(json) {
            Ok(parsed) => parsed,
            Err(err) => {
                self.record_rejection();
                return Err(err);
            }
        };
        self.apply_page(update)
    }

    /// Hot-swaps a page route with a native Rust page builder closure.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `String` - Target route path.
    /// - `String` - Page title.
    /// - `u64` - Monotonic page version number.
    /// - `F` - Zero-context native Rust page builder closure.
    ///
    /// # Returns
    ///
    /// - `Result<u64, DynamicUpdateError>` - Newly active page version on success.
    pub fn swap_page_builder<F>(
        &self,
        route: String,
        title: String,
        version: u64,
        builder: F,
    ) -> Result<u64, DynamicUpdateError>
    where
        F: Fn() -> NodeId + 'static,
    {
        let normalized: String = normalize_dynamic_route(&route);
        if normalized.is_empty() {
            self.record_rejection();
            return Err(DynamicUpdateError::EmptyTargetId);
        }
        let builder_rc: DynamicBuilderFn = Rc::new(builder);
        let res: Result<PageApplyCommit, DynamicUpdateError> =
            self.with_state_mut(|state: &mut DynamicHubState| {
                let is_new_route: bool = state
                    .pages
                    .get(&normalized)
                    .map(|stack: &Vec<DynamicSlotRecord>| stack.is_empty())
                    .unwrap_or(true);
                let active_ver: u64 = state
                    .pages
                    .get(&normalized)
                    .and_then(|stack: &Vec<DynamicSlotRecord>| stack.last())
                    .map(|rec: &DynamicSlotRecord| rec.version)
                    .unwrap_or(0);
                if version <= active_ver {
                    return Err(DynamicUpdateError::VersionRegression {
                        target: normalized.clone(),
                        current: active_ver,
                        incoming: version,
                    });
                }
                let record: DynamicSlotRecord = DynamicSlotRecord {
                    version,
                    title,
                    checksum: 0,
                    payload: DynamicPayload::Native(builder_rc),
                };
                state
                    .pages
                    .entry(normalized.clone())
                    .or_default()
                    .push(record);
                let page_sig: Signal<u64> = *state
                    .page_signals
                    .entry(normalized)
                    .or_insert_with(|| Signal::new(0));
                let mut tel: DynamicTelemetry = state.telemetry_signal.get_untracked();
                tel.set_pages_applied(tel.get_pages_applied().saturating_add(1));
                Ok((
                    page_sig,
                    state.routes_signal,
                    state.telemetry_signal,
                    tel,
                    is_new_route,
                ))
            });
        match res {
            Ok((page_sig, routes_sig, tel_sig, tel, is_new_route)) => {
                batch(|| {
                    let _: bool = page_sig.update(|v: &mut u64| *v = v.wrapping_add(1));
                    if is_new_route {
                        let _: bool = routes_sig.update(|v: &mut u64| *v = v.wrapping_add(1));
                    }
                    let _: bool = tel_sig.set(tel);
                });
                Ok(version)
            }
            Err(err) => {
                self.record_rejection();
                Err(err)
            }
        }
    }

    /// Rolls back a dynamically updated page to its previous revision (or `0` compiled fallback).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - Target route path.
    ///
    /// # Returns
    ///
    /// - `Result<u64, DynamicUpdateError>` - Restored page version (`0` means fallback / removed).
    pub fn rollback_page(&self, route: &str) -> Result<u64, DynamicUpdateError> {
        let normalized: String = normalize_dynamic_route(route);
        let res: Result<PageRollbackCommit, DynamicUpdateError> =
            self.with_state_mut(|state: &mut DynamicHubState| {
                let popped: bool = state
                    .pages
                    .get_mut(&normalized)
                    .and_then(|stack: &mut Vec<DynamicSlotRecord>| stack.pop())
                    .is_some();
                if !popped {
                    return Err(DynamicUpdateError::NoHistoryToRollback(normalized.clone()));
                }
                let next_ver: u64 = state
                    .pages
                    .get(&normalized)
                    .and_then(|stack: &Vec<DynamicSlotRecord>| stack.last())
                    .map(|rec: &DynamicSlotRecord| rec.version)
                    .unwrap_or(0);
                let route_removed: bool = next_ver == 0;
                let page_sig: Signal<u64> = *state
                    .page_signals
                    .entry(normalized)
                    .or_insert_with(|| Signal::new(0));
                let mut tel: DynamicTelemetry = state.telemetry_signal.get_untracked();
                tel.set_rollbacks(tel.get_rollbacks().saturating_add(1));
                Ok((
                    next_ver,
                    page_sig,
                    state.routes_signal,
                    state.telemetry_signal,
                    tel,
                    route_removed,
                ))
            });
        let (next_ver, page_sig, routes_sig, tel_sig, tel, route_removed): PageRollbackCommit =
            res?;
        batch(|| {
            let _: bool = page_sig.update(|v: &mut u64| *v = v.wrapping_add(1));
            if route_removed {
                let _: bool = routes_sig.update(|v: &mut u64| *v = v.wrapping_add(1));
            }
            let _: bool = tel_sig.set(tel);
        });
        Ok(next_ver)
    }

    /// Validates all component and page entries in `bundle` before committing them atomically in a single `batch`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `DynamicUpdateBundle` - Multi-target atomic release bundle.
    ///
    /// # Returns
    ///
    /// - `Result<DynamicTelemetry, DynamicUpdateError>` - Updated telemetry snapshot on full commit, or error with zero partial mutation.
    pub fn apply_bundle(
        &self,
        bundle: DynamicUpdateBundle,
    ) -> Result<DynamicTelemetry, DynamicUpdateError> {
        let validation: Result<BundleApplyCommit, DynamicUpdateError> =
            self.with_state_mut(|state: &mut DynamicHubState| {
                let mut staged_comp_vers: HashMap<String, u64> = HashMap::new();
                for comp in bundle.components.iter() {
                    let id: String = comp.get_id().trim().to_string();
                    if id.is_empty() {
                        return Err(DynamicUpdateError::EmptyTargetId);
                    }
                    let actual: u64 = comp.get_spec().checksum();
                    let expected: u64 = comp.get_checksum();
                    if expected != 0 && expected != actual {
                        return Err(DynamicUpdateError::ChecksumMismatch {
                            target: id,
                            expected,
                            actual,
                        });
                    }
                    let cur: u64 = staged_comp_vers.get(&id).copied().unwrap_or_else(|| {
                        state
                            .components
                            .get(&id)
                            .and_then(|s: &Vec<DynamicSlotRecord>| s.last())
                            .map(|r: &DynamicSlotRecord| r.version)
                            .unwrap_or(0)
                    });
                    if !comp.get_allow_downgrade() && comp.get_version() <= cur {
                        return Err(DynamicUpdateError::VersionRegression {
                            target: id,
                            current: cur,
                            incoming: comp.get_version(),
                        });
                    }
                    staged_comp_vers.insert(id, comp.get_version());
                }
                let mut staged_page_vers: HashMap<String, u64> = HashMap::new();
                for page in bundle.pages.iter() {
                    let route: String = normalize_dynamic_route(page.get_route());
                    if route.is_empty() {
                        return Err(DynamicUpdateError::EmptyTargetId);
                    }
                    let actual: u64 = page.get_spec().checksum();
                    let expected: u64 = page.get_checksum();
                    if expected != 0 && expected != actual {
                        return Err(DynamicUpdateError::ChecksumMismatch {
                            target: route,
                            expected,
                            actual,
                        });
                    }
                    let cur: u64 = staged_page_vers.get(&route).copied().unwrap_or_else(|| {
                        state
                            .pages
                            .get(&route)
                            .and_then(|s: &Vec<DynamicSlotRecord>| s.last())
                            .map(|r: &DynamicSlotRecord| r.version)
                            .unwrap_or(0)
                    });
                    if !page.get_allow_downgrade() && page.get_version() <= cur {
                        return Err(DynamicUpdateError::VersionRegression {
                            target: route,
                            current: cur,
                            incoming: page.get_version(),
                        });
                    }
                    staged_page_vers.insert(route, page.get_version());
                }
                let mut dirty_signals: Vec<Signal<u64>> =
                    Vec::with_capacity(bundle.components.len() + bundle.pages.len());
                let mut tel: DynamicTelemetry = state.telemetry_signal.get_untracked();
                for comp in bundle.components.into_iter() {
                    let id: String = comp.get_id().trim().to_string();
                    let digest: u64 = comp.get_spec().checksum();
                    let record: DynamicSlotRecord = DynamicSlotRecord {
                        version: comp.version,
                        title: String::new(),
                        checksum: digest,
                        payload: DynamicPayload::Spec(comp.spec),
                    };
                    state.components.entry(id.clone()).or_default().push(record);
                    let sig: Signal<u64> = *state
                        .component_signals
                        .entry(id)
                        .or_insert_with(|| Signal::new(0));
                    dirty_signals.push(sig);
                    tel.set_components_applied(tel.get_components_applied().saturating_add(1));
                }
                let mut added_route: bool = false;
                for page in bundle.pages.into_iter() {
                    let route: String = normalize_dynamic_route(page.get_route());
                    let is_new: bool = state
                        .pages
                        .get(&route)
                        .map(|s: &Vec<DynamicSlotRecord>| s.is_empty())
                        .unwrap_or(true);
                    if is_new {
                        added_route = true;
                    }
                    let digest: u64 = page.get_spec().checksum();
                    let record: DynamicSlotRecord = DynamicSlotRecord {
                        version: page.version,
                        title: page.title,
                        checksum: digest,
                        payload: DynamicPayload::Spec(page.spec),
                    };
                    state.pages.entry(route.clone()).or_default().push(record);
                    let sig: Signal<u64> = *state
                        .page_signals
                        .entry(route)
                        .or_insert_with(|| Signal::new(0));
                    dirty_signals.push(sig);
                    tel.set_pages_applied(tel.get_pages_applied().saturating_add(1));
                }
                Ok((
                    dirty_signals,
                    added_route,
                    state.routes_signal,
                    state.telemetry_signal,
                    tel,
                ))
            });
        match validation {
            Ok((dirty_signals, added_route, routes_sig, tel_sig, tel)) => {
                batch(|| {
                    for sig in dirty_signals.into_iter() {
                        let _: bool = sig.update(|v: &mut u64| *v = v.wrapping_add(1));
                    }
                    if added_route {
                        let _: bool = routes_sig.update(|v: &mut u64| *v = v.wrapping_add(1));
                    }
                    let _: bool = tel_sig.set(tel);
                });
                Ok(tel)
            }
            Err(err) => {
                self.record_rejection();
                Err(err)
            }
        }
    }

    /// Parses and atomically applies a JSON-encoded `DynamicUpdateBundle` manifest.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - JSON bundle manifest string.
    ///
    /// # Returns
    ///
    /// - `Result<DynamicTelemetry, DynamicUpdateError>` - Updated telemetry snapshot on success.
    pub fn apply_bundle_json(&self, json: &str) -> Result<DynamicTelemetry, DynamicUpdateError> {
        let bundle: DynamicUpdateBundle = match DynamicUpdateBundle::from_json(json) {
            Ok(parsed) => parsed,
            Err(err) => {
                self.record_rejection();
                return Err(err);
            }
        };
        self.apply_bundle(bundle)
    }

    /// Registers an action callback invoked whenever a button inside a dynamically updated component or page is clicked.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `F` - Action callback receiving `&DynamicActionEvent`.
    pub fn on_action<F>(&self, handler: F)
    where
        F: FnMut(&DynamicActionEvent) + 'static,
    {
        let cb: DynamicActionCallback = Rc::new(RefCell::new(handler));
        self.with_state_mut(|state: &mut DynamicHubState| {
            state.action_handlers.push(cb);
        });
    }

    /// Dispatches a `DynamicActionEvent` to all registered handlers and updates `last_action` + telemetry.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `DynamicActionEvent` - Action event emitted by a dynamic control.
    pub fn dispatch_action(&self, event: DynamicActionEvent) {
        let (handlers, last_sig, tel_sig, next_tel): (
            Vec<DynamicActionCallback>,
            Signal<Option<DynamicActionEvent>>,
            Signal<DynamicTelemetry>,
            DynamicTelemetry,
        ) = self.with_state_mut(|state: &mut DynamicHubState| {
            let mut tel: DynamicTelemetry = state.telemetry_signal.get_untracked();
            tel.set_actions_dispatched(tel.get_actions_dispatched().saturating_add(1));
            (
                state.action_handlers.clone(),
                state.last_action,
                state.telemetry_signal,
                tel,
            )
        });
        batch(|| {
            let _: bool = last_sig.set(Some(event.clone()));
            let _: bool = tel_sig.set(next_tel);
        });
        for cb in handlers.into_iter() {
            if let Ok(mut func) = cb.try_borrow_mut() {
                func(&event);
            }
        }
    }

    /// Returns the dedicated reactive revision signal for component slot `id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - Component slot identifier.
    ///
    /// # Returns
    ///
    /// - `Signal<u64>` - Surgical revision signal bumped only when `id` updates.
    #[must_use]
    pub fn component_revision_signal(&self, id: &str) -> Signal<u64> {
        let key: String = id.trim().to_string();
        self.with_state_mut(|state: &mut DynamicHubState| {
            *state
                .component_signals
                .entry(key)
                .or_insert_with(|| Signal::new(0))
        })
    }

    /// Returns the dedicated reactive revision signal for page `route`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - Target route path.
    ///
    /// # Returns
    ///
    /// - `Signal<u64>` - Surgical revision signal bumped only when `route` updates.
    #[must_use]
    pub fn page_revision_signal(&self, route: &str) -> Signal<u64> {
        let key: String = normalize_dynamic_route(route);
        self.with_state_mut(|state: &mut DynamicHubState| {
            *state
                .page_signals
                .entry(key)
                .or_insert_with(|| Signal::new(0))
        })
    }

    /// Returns the currently active dynamic version for component slot `id` (`0` means compiled fallback).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - Component slot identifier.
    ///
    /// # Returns
    ///
    /// - `u64` - Active component version.
    #[must_use]
    pub fn component_version(&self, id: &str) -> u64 {
        let key: &str = id.trim();
        self.with_state(|state: &DynamicHubState| {
            state
                .components
                .get(key)
                .and_then(|stack: &Vec<DynamicSlotRecord>| stack.last())
                .map(|rec: &DynamicSlotRecord| rec.version)
                .unwrap_or(0)
        })
    }

    /// Returns the number of stacked dynamic revisions retained for component slot `id`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - Component slot identifier.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of revisions on the rollback stack.
    #[must_use]
    pub fn component_history_len(&self, id: &str) -> usize {
        let key: &str = id.trim();
        self.with_state(|state: &DynamicHubState| {
            state
                .components
                .get(key)
                .map(|stack: &Vec<DynamicSlotRecord>| stack.len())
                .unwrap_or(0)
        })
    }

    /// Returns the currently active dynamic version for page `route` (`0` means compiled fallback).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - Page route path.
    ///
    /// # Returns
    ///
    /// - `u64` - Active page version.
    #[must_use]
    pub fn page_version(&self, route: &str) -> u64 {
        let key: String = normalize_dynamic_route(route);
        self.with_state(|state: &DynamicHubState| {
            state
                .pages
                .get(&key)
                .and_then(|stack: &Vec<DynamicSlotRecord>| stack.last())
                .map(|rec: &DynamicSlotRecord| rec.version)
                .unwrap_or(0)
        })
    }

    /// Returns the active dynamic page title for `route` if a dynamic page is mounted for it.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - Page route path.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Active dynamic page title if any.
    #[must_use]
    pub fn page_title(&self, route: &str) -> Option<String> {
        let key: String = normalize_dynamic_route(route);
        self.with_state(|state: &DynamicHubState| {
            state
                .pages
                .get(&key)
                .and_then(|stack: &Vec<DynamicSlotRecord>| stack.last())
                .map(|rec: &DynamicSlotRecord| rec.title.clone())
        })
    }

    /// Returns `true` when `route` currently has an active dynamic page override or injected page.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - Page route path.
    ///
    /// # Returns
    ///
    /// - `bool` - True when a dynamic page payload is active for `route`.
    #[must_use]
    pub fn has_dynamic_page(&self, route: &str) -> bool {
        self.page_version(route) > 0
    }

    /// Returns the sorted list of all routes currently carrying active dynamic page payloads.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    ///
    /// # Returns
    ///
    /// - `Vec<String>` - Sorted active dynamic route paths.
    #[must_use]
    pub fn registered_routes(&self) -> Vec<String> {
        self.with_state(|state: &DynamicHubState| {
            let mut routes: Vec<String> = state
                .pages
                .iter()
                .filter(|(_, stack): &(&String, &Vec<DynamicSlotRecord>)| !stack.is_empty())
                .map(|(route, _): (&String, &Vec<DynamicSlotRecord>)| route.clone())
                .collect();
            routes.sort();
            routes
        })
    }

    /// Returns the reactive signal bumped whenever dynamic routes are added or removed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    ///
    /// # Returns
    ///
    /// - `Signal<u64>` - Reactive route-set revision signal.
    #[must_use]
    pub fn routes_signal(&self) -> Signal<u64> {
        let Self { inner } = self;
        inner
            .try_borrow()
            .map(|state: Ref<'_, DynamicHubState>| state.routes_signal)
            .unwrap_or_else(|_| Signal::new(0))
    }

    /// Returns the reactive telemetry snapshot signal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    ///
    /// # Returns
    ///
    /// - `Signal<DynamicTelemetry>` - Reactive dynamic update telemetry signal.
    #[must_use]
    pub fn telemetry(&self) -> Signal<DynamicTelemetry> {
        let Self { inner } = self;
        inner
            .try_borrow()
            .map(|state: Ref<'_, DynamicHubState>| state.telemetry_signal)
            .unwrap_or_else(|_| Signal::new(DynamicTelemetry::default()))
    }

    /// Returns the reactive signal storing the most recently dispatched `DynamicActionEvent`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    ///
    /// # Returns
    ///
    /// - `Signal<Option<DynamicActionEvent>>` - Reactive latest action signal.
    #[must_use]
    pub fn last_action(&self) -> Signal<Option<DynamicActionEvent>> {
        let Self { inner } = self;
        inner
            .try_borrow()
            .map(|state: Ref<'_, DynamicHubState>| state.last_action)
            .unwrap_or_else(|_| Signal::new(None))
    }

    /// Mounts the active dynamic payload for component slot `id`, or executes `fallback` if none is active.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - Component slot identifier.
    /// - `&DynamicBuilderFn` - Compiled fallback builder closure.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted native scene node handle.
    #[must_use]
    pub fn mount_component_or_fallback(&self, id: &str, fallback: &DynamicBuilderFn) -> NodeId {
        let key: String = id.trim().to_string();
        let payload_opt: Option<DynamicPayload> = self.with_state(|state: &DynamicHubState| {
            state
                .components
                .get(&key)
                .and_then(|stack: &Vec<DynamicSlotRecord>| stack.last())
                .map(|rec: &DynamicSlotRecord| rec.payload.clone())
        });
        match payload_opt {
            Some(DynamicPayload::Spec(spec)) => mount_dynamic_spec(&spec, &key, self),
            Some(DynamicPayload::Native(builder)) => builder(),
            None => fallback(),
        }
    }

    /// Mounts the active dynamic payload for page `route`, or executes `fallback` if none is active.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `&str` - Target route path.
    /// - `&DynamicBuilderFn` - Compiled fallback page builder closure.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted native page node handle.
    #[must_use]
    pub fn mount_page_or_fallback(&self, route: &str, fallback: &DynamicBuilderFn) -> NodeId {
        let key: String = normalize_dynamic_route(route);
        let payload_opt: Option<DynamicPayload> = self.with_state(|state: &DynamicHubState| {
            state
                .pages
                .get(&key)
                .and_then(|stack: &Vec<DynamicSlotRecord>| stack.last())
                .map(|rec: &DynamicSlotRecord| rec.payload.clone())
        });
        match payload_opt {
            Some(DynamicPayload::Spec(spec)) => mount_dynamic_spec(&spec, &key, self),
            Some(DynamicPayload::Native(builder)) => builder(),
            None => fallback(),
        }
    }

    /// Increments the rejected-update telemetry counter safely outside any active state borrow.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    fn record_rejection(&self) {
        let tel_sig: Signal<DynamicTelemetry> = self.telemetry();
        let mut next_tel: DynamicTelemetry = tel_sig.get_untracked();
        next_tel.set_rejected(next_tel.get_rejected().saturating_add(1));
        let _: bool = tel_sig.set(next_tel);
    }

    /// Executes a read-only closure on the internal `DynamicHubState` via safe `try_borrow`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `F` - Reader closure.
    ///
    /// # Returns
    ///
    /// - `R` - Closure return value or `R::default()` if re-entrantly borrowed.
    fn with_state<F, R>(&self, operation: F) -> R
    where
        F: FnOnce(&DynamicHubState) -> R,
        R: Default,
    {
        let Self { inner } = self;
        match inner.try_borrow() {
            Ok(guard) => operation(&guard),
            Err(_) => R::default(),
        }
    }

    /// Executes a mutable closure on the internal `DynamicHubState` via safe `try_borrow_mut`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hub reference.
    /// - `F` - Mutator closure.
    ///
    /// # Returns
    ///
    /// - `R` - Closure return value.
    fn with_state_mut<F, R>(&self, operation: F) -> R
    where
        F: FnOnce(&mut DynamicHubState) -> R,
    {
        let Self { inner } = self;
        if let Ok(mut guard) = inner.try_borrow_mut() {
            return operation(&mut guard);
        }
        let mut fallback: DynamicHubState = DynamicHubState {
            components: HashMap::new(),
            component_signals: HashMap::new(),
            pages: HashMap::new(),
            page_signals: HashMap::new(),
            routes_signal: Signal::new(0),
            telemetry_signal: Signal::new(DynamicTelemetry::default()),
            last_action: Signal::new(None),
            action_handlers: Vec::new(),
        };
        operation(&mut fallback)
    }
}

/// `Mountable` implementation mounting a surgically updatable component slot.
impl Mountable for DynamicComponentSlot {
    /// Mounts the host container and subscribes ONLY to this slot's dedicated revision signal.
    ///
    /// # Arguments
    ///
    /// - `Self` - Component slot descriptor.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted slot host handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let host_style: Style = self.get_style();
        let Self {
            id, fallback, hub, ..
        } = self;
        let active_hub: DynamicUpdateHub = hub.unwrap_or_else(|| ctx.get_dynamic_hub().clone());
        let rev_sig: Signal<u64> = active_hub.component_revision_signal(&id);
        let effective_style: Style = if host_style == Style::empty() {
            Style::empty().column().width(Length::Percent(100.0))
        } else {
            host_style
        };
        let host: NodeId = ctx.view(effective_style);
        ctx.bind_children(host, move || {
            let _: u64 = rev_sig.get();
            vec![active_hub.mount_component_or_fallback(&id, &fallback)]
        });
        host
    }
}

/// `Mountable` implementation mounting a surgically updatable page slot for a fixed route.
impl Mountable for DynamicPageSlot {
    /// Mounts the page container and subscribes ONLY to this route's dedicated revision signal.
    ///
    /// # Arguments
    ///
    /// - `Self` - Page slot descriptor.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted page slot host handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let host_style: Style = self.get_style();
        let Self {
            route,
            fallback,
            hub,
            ..
        } = self;
        let normalized: String = normalize_dynamic_route(&route);
        let active_hub: DynamicUpdateHub = hub.unwrap_or_else(|| ctx.get_dynamic_hub().clone());
        let rev_sig: Signal<u64> = active_hub.page_revision_signal(&normalized);
        let effective_style: Style = if host_style == Style::empty() {
            Style::empty().column().width(Length::Percent(100.0))
        } else {
            host_style
        };
        let host: NodeId = ctx.view(effective_style);
        ctx.bind_children(host, move || {
            let _: u64 = rev_sig.get();
            vec![active_hub.mount_page_or_fallback(&normalized, &fallback)]
        });
        host
    }
}

/// `Mountable` implementation mounting a reactive route-driven dynamic page outlet.
impl Mountable for DynamicPageOutlet {
    /// Mounts a dynamic route outlet reacting to both route switches and live page-level updates.
    ///
    /// # Arguments
    ///
    /// - `Self` - Page outlet descriptor.
    /// - `&mut BuildContext` - Active build context.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted outlet host handle.
    fn mount(self, ctx: &mut BuildContext) -> NodeId {
        let route_sig: Signal<String> = self.get_route();
        let host_style: Style = self.get_style();
        let Self { fallback, hub, .. } = self;
        let active_hub: DynamicUpdateHub = hub.unwrap_or_else(|| ctx.get_dynamic_hub().clone());
        let effective_style: Style = if host_style == Style::empty() {
            Style::empty().column().width(Length::Percent(100.0))
        } else {
            host_style
        };
        let host: NodeId = ctx.view(effective_style);
        let scene_for_anim: SceneHandle = ctx.get_scene().clone();
        let prev_route_cell: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        ctx.bind_children(host, move || {
            let active_route: String = normalize_dynamic_route(&route_sig.get());
            let page_rev_sig: Signal<u64> = active_hub.page_revision_signal(&active_route);
            let _: u64 = page_rev_sig.get();
            let fallback_c: DynamicRouteFallbackFn = fallback.clone();
            let route_for_fb: String = active_route.clone();
            let fb: DynamicBuilderFn = Rc::new(move || fallback_c(&route_for_fb));
            let page_node: NodeId = active_hub.mount_page_or_fallback(&active_route, &fb);
            let should_animate: bool = match prev_route_cell.try_borrow_mut() {
                Ok(mut slot) => {
                    let changed: bool = slot
                        .as_deref()
                        .map(|prev: &str| prev != active_route.as_str())
                        .unwrap_or(false);
                    *slot = Some(active_route.clone());
                    changed
                }
                Err(_) => false,
            };
            if should_animate {
                scene_for_anim.start_transition(NodeTransition::route_enter(page_node));
            }
            vec![page_node]
        });
        host
    }
}
