use super::*;

/// Human-readable diagnostic formatting for `WasmDynamicError`.
impl fmt::Display for WasmDynamicError {
    /// Formats a descriptive error string for logs and UI error boundaries.
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
            Self::InvalidUrl(url) => write!(f, "invalid wasm URL: {url}"),
            Self::NetworkOffline(url) => write!(f, "network offline while fetching {url}"),
            Self::HttpStatus { status, url } => write!(f, "HTTP {status} downloading {url}"),
            Self::Timeout { url, timeout_ms } => {
                write!(f, "download timed out after {timeout_ms}ms for {url}")
            }
            Self::Cancelled { request_id } => write!(f, "wasm request #{request_id} was cancelled"),
            Self::StaleResponse {
                request_id,
                active_request_id,
            } => write!(
                f,
                "discarded stale wasm response #{request_id} (active #{active_request_id})"
            ),
            Self::BinaryTooLarge { actual, limit } => {
                write!(f, "wasm binary {actual} bytes exceeds limit {limit}")
            }
            Self::InvalidWasmBinary(msg) => write!(f, "invalid wasm binary: {msg}"),
            Self::ChecksumMismatch { expected, actual } => {
                write!(
                    f,
                    "wasm checksum mismatch: expected {expected:#018x}, got {actual:#018x}"
                )
            }
            Self::SignatureInvalid { expected, actual } => {
                write!(
                    f,
                    "wasm signature invalid: expected {expected}, got {actual}"
                )
            }
            Self::AbiMismatch { expected, actual } => {
                write!(f, "wasm ABI mismatch: host v{expected}, module v{actual}")
            }
            Self::MissingExport(name) => write!(f, "wasm module missing required export `{name}`"),
            Self::VersionRegression { current, incoming } => {
                write!(
                    f,
                    "wasm version regression blocked: active v{current}, incoming v{incoming}"
                )
            }
            Self::MemoryLimitExceeded { requested, limit } => {
                write!(f, "wasm memory {requested} bytes exceeds limit {limit}")
            }
            Self::MemoryOutOfBounds {
                addr,
                len,
                memory_size,
            } => write!(
                f,
                "wasm memory out of bounds: [{addr}..+{len}] > {memory_size}"
            ),
            Self::OutOfFuel { limit } => {
                write!(f, "wasm execution exceeded fuel budget ({limit} ops)")
            }
            Self::ExecutionTrap(msg) => write!(f, "wasm runtime trap: {msg}"),
            Self::InvalidUiPayload(msg) => write!(f, "invalid wasm native UI payload: {msg}"),
            Self::NoRollbackSnapshot => write!(f, "no prior verified wasm snapshot to roll back"),
        }
    }
}

/// Standard `Error` marker implementation for `WasmDynamicError`.
impl Error for WasmDynamicError {}

/// Safe production defaults for `WasmSandboxConfig`.
impl Default for WasmSandboxConfig {
    /// Returns balanced production sandbox ceilings (8 MiB binary, 4 MiB memory, 100k fuel, 2 retries, offline cache on).
    ///
    /// # Returns
    ///
    /// - `Self` - Default `WasmSandboxConfig`.
    fn default() -> Self {
        Self {
            max_binary_bytes: WASM_DEFAULT_MAX_BINARY_BYTES,
            max_memory_bytes: WASM_DEFAULT_MAX_MEMORY_BYTES,
            max_fuel: WASM_DEFAULT_MAX_FUEL,
            timeout_ms: WASM_DEFAULT_TIMEOUT_MS,
            max_retries: WASM_DEFAULT_MAX_RETRIES,
            allow_offline_cache: true,
            require_signature: false,
        }
    }
}

/// Scope key formatting helpers for `WasmDynamicScope`.
impl WasmDynamicScope {
    /// Returns a stable unique registry key identifying this scope.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scope reference.
    ///
    /// # Returns
    ///
    /// - `String` - Scope registry key.
    #[must_use]
    pub fn key(&self) -> String {
        match self {
            Self::Partial { slot_id } => format!("partial:{slot_id}"),
            Self::Global => "global".to_string(),
        }
    }
}

/// Fluent builder helpers for `WasmDownloadRequest`.
impl WasmDownloadRequest {
    /// Creates a scoped partial-slot `.wasm` download request.
    ///
    /// # Arguments
    ///
    /// - `String` - Source `.wasm` URL.
    /// - `String` - Target partial slot ID.
    /// - `u64` - Release version number.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured partial download request.
    #[must_use]
    pub fn partial(url: String, slot_id: String, version: u64) -> Self {
        Self::new(url, WasmDynamicScope::Partial { slot_id }, version)
    }

    /// Creates a global full-app `.wasm` download request.
    ///
    /// # Arguments
    ///
    /// - `String` - Source `.wasm` URL.
    /// - `u64` - Release version number.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured global download request.
    #[must_use]
    pub fn global(url: String, version: u64) -> Self {
        Self::new(url, WasmDynamicScope::Global, version)
    }

    /// Attaches an expected FNV-1a checksum for integrity verification.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source request.
    /// - `u64` - Expected checksum.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated request.
    #[must_use]
    pub fn with_checksum(mut self, checksum: u64) -> Self {
        self.set_expected_checksum(checksum);
        self
    }

    /// Attaches an expected signature tag for authenticated verification.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source request.
    /// - `String` - Expected signature string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated request.
    #[must_use]
    pub fn with_signature(mut self, signature: String) -> Self {
        self.set_expected_signature(signature);
        self
    }

    /// Allows installing a version lower than or equal to the currently mounted version.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source request.
    /// - `bool` - Downgrade permission flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated request.
    #[must_use]
    pub fn with_allow_downgrade(mut self, allow: bool) -> Self {
        self.set_allow_downgrade(allow);
        self
    }

    /// Resolves the effective cache lookup key (`cache_key` if non-empty, otherwise `url`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Request reference.
    ///
    /// # Returns
    ///
    /// - `&str` - Effective cache key.
    #[must_use]
    pub fn resolved_cache_key(&self) -> &str {
        if self.get_cache_key().is_empty() {
            self.get_url().as_str()
        } else {
            self.get_cache_key().as_str()
        }
    }
}

/// Accessor helpers for `WasmExecutionOutput`.
impl WasmExecutionOutput {
    /// Returns the optional emitted partial `DynamicNodeSpec`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Execution output reference.
    ///
    /// # Returns
    ///
    /// - `Option<&DynamicNodeSpec>` - Emitted spec reference.
    #[must_use]
    pub fn emitted_spec(&self) -> Option<&DynamicNodeSpec> {
        let Self { emitted_spec, .. } = self;
        emitted_spec.as_ref()
    }

    /// Returns the optional emitted global `DynamicUpdateBundle`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Execution output reference.
    ///
    /// # Returns
    ///
    /// - `Option<&DynamicUpdateBundle>` - Emitted bundle reference.
    #[must_use]
    pub fn emitted_bundle(&self) -> Option<&DynamicUpdateBundle> {
        let Self { emitted_bundle, .. } = self;
        emitted_bundle.as_ref()
    }
}

/// Accessor helpers for `WasmMountedSnapshot`.
impl WasmMountedSnapshot {
    /// Returns the mounted partial spec if present.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Snapshot reference.
    ///
    /// # Returns
    ///
    /// - `Option<&DynamicNodeSpec>` - Mounted spec reference.
    #[must_use]
    pub fn spec_ref(&self) -> Option<&DynamicNodeSpec> {
        let Self { spec, .. } = self;
        spec.as_ref()
    }

    /// Returns the mounted global bundle if present.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Snapshot reference.
    ///
    /// # Returns
    ///
    /// - `Option<&DynamicUpdateBundle>` - Mounted bundle reference.
    #[must_use]
    pub fn bundle_ref(&self) -> Option<&DynamicUpdateBundle> {
        let Self { bundle, .. } = self;
        bundle.as_ref()
    }
}

/// Spec-compliant WebAssembly binary encoder methods for `WasmModuleBuilder`.
impl WasmModuleBuilder {
    /// Starts building a partial-slot `.wasm` module emitting `spec` through `env::euv_emit_ui`.
    ///
    /// # Arguments
    ///
    /// - `&DynamicNodeSpec` - Primary native UI subtree specification.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured `.wasm` builder.
    #[must_use]
    pub fn partial_spec(spec: &DynamicNodeSpec) -> Self {
        let json: String = to_string(spec).unwrap_or_else(|_| "{}".to_string());
        Self {
            abi_version: WASM_ABI_VERSION,
            export_render: true,
            is_global_bundle: false,
            ui_payload_json: json,
            alt_ui_payload_json: String::new(),
            log_message: String::new(),
            fault: WasmFaultKind::None,
            custom_meta: Vec::new(),
        }
    }

    /// Starts building a global full-app `.wasm` module emitting `bundle` through `env::euv_emit_bundle`.
    ///
    /// # Arguments
    ///
    /// - `&DynamicUpdateBundle` - Full-app dynamic bundle specification.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured global `.wasm` builder.
    #[must_use]
    pub fn global_bundle(bundle: &DynamicUpdateBundle) -> Self {
        let json: String = to_string(bundle).unwrap_or_else(|_| "{}".to_string());
        Self {
            abi_version: WASM_ABI_VERSION,
            export_render: true,
            is_global_bundle: true,
            ui_payload_json: json,
            alt_ui_payload_json: String::new(),
            log_message: String::new(),
            fault: WasmFaultKind::None,
            custom_meta: Vec::new(),
        }
    }

    /// Attaches an alternate interactive UI spec rendered whenever `euv_render(state)` receives a non-zero state.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source builder.
    /// - `&DynamicNodeSpec` - Alternate interactive state UI spec.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated builder.
    #[must_use]
    pub fn with_alt_spec(mut self, alt: &DynamicNodeSpec) -> Self {
        let json: String = to_string(alt).unwrap_or_else(|_| "{}".to_string());
        self.set_alt_ui_payload_json(json);
        self
    }

    /// Attaches an alternate global `DynamicUpdateBundle` JSON payload emitted when `euv_on_event` toggles state.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source builder.
    /// - `&DynamicUpdateBundle` - Alternate global bundle for odd state ticks.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated builder.
    #[must_use]
    pub fn with_alt_bundle(mut self, bundle: &DynamicUpdateBundle) -> Self {
        let json: String = bundle.to_json().unwrap_or_default();
        self.set_alt_ui_payload_json(json);
        self
    }

    /// Attaches a diagnostic log message emitted via `env::euv_log` during render.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source builder.
    /// - `String` - Log line text.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated builder.
    #[must_use]
    pub fn with_log(mut self, message: String) -> Self {
        self.set_log_message(message);
        self
    }

    /// Overrides the reported ABI version (used to test ABI mismatch rejection).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source builder.
    /// - `u32` - ABI version returned by `euv_abi_version`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated builder.
    #[must_use]
    pub fn with_abi_version(mut self, abi: u32) -> Self {
        self.set_abi_version(abi);
        self
    }

    /// Omits the `euv_render` export (used to test missing export rejection).
    ///
    /// # Arguments
    ///
    /// - `Self` - Source builder.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated builder.
    #[must_use]
    pub fn without_render_export(mut self) -> Self {
        self.set_export_render(false);
        self
    }

    /// Compiles a deterministic sandbox fault (`InfiniteLoopOutOfFuel` / `ExecutionTrap`) into `euv_render`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source builder.
    /// - `WasmFaultKind` - Fault behavior to compile.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated builder.
    #[must_use]
    pub fn with_fault(mut self, fault: WasmFaultKind) -> Self {
        self.set_fault(fault);
        self
    }

    /// Appends a WebAssembly section with length prefix into `out`.
    ///
    /// # Arguments
    ///
    /// - `&mut Vec<u8>` - Destination binary stream.
    /// - `u8` - WASM section ID.
    /// - `&[u8]` - Encoded section payload bytes.
    fn push_section(out: &mut Vec<u8>, sec_id: u8, payload: &[u8]) {
        out.push(sec_id);
        write_uleb128(out, payload.len() as u32);
        out.extend_from_slice(payload);
    }

    /// Encodes a UTF-8 WASM identifier name into `out`.
    ///
    /// # Arguments
    ///
    /// - `&mut Vec<u8>` - Destination byte vector.
    /// - `&str` - Identifier text.
    fn push_name(out: &mut Vec<u8>, name: &str) {
        write_uleb128(out, name.len() as u32);
        out.extend_from_slice(name.as_bytes());
    }

    /// Emits a complete spec-compliant executable `.wasm` binary buffer.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Configured builder.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - Valid WebAssembly MVP binary bytes (`\0asm\x01\0\0\0...`).
    #[must_use]
    pub fn build(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(512);
        out.extend_from_slice(WASM_BINARY_MAGIC);
        out.extend_from_slice(WASM_BINARY_VERSION);
        let mut type_sec: Vec<u8> = Vec::new();
        write_uleb128(&mut type_sec, 3);
        type_sec.extend_from_slice(&[WASM_FUNCTYPE_TAG, 2, WASM_VALTYPE_I32, WASM_VALTYPE_I32, 0]);
        type_sec.extend_from_slice(&[WASM_FUNCTYPE_TAG, 1, WASM_VALTYPE_I32, 1, WASM_VALTYPE_I32]);
        type_sec.extend_from_slice(&[WASM_FUNCTYPE_TAG, 0, 1, WASM_VALTYPE_I32]);
        Self::push_section(&mut out, WASM_SEC_TYPE, &type_sec);
        let mut imp_sec: Vec<u8> = Vec::new();
        write_uleb128(&mut imp_sec, 2);
        let emit_name: &str = if self.get_is_global_bundle() {
            WASM_IMPORT_EMIT_BUNDLE
        } else {
            WASM_IMPORT_EMIT_UI
        };
        Self::push_name(&mut imp_sec, WASM_IMPORT_MODULE_ENV);
        Self::push_name(&mut imp_sec, emit_name);
        imp_sec.push(0x00);
        write_uleb128(&mut imp_sec, 0);
        Self::push_name(&mut imp_sec, WASM_IMPORT_MODULE_ENV);
        Self::push_name(&mut imp_sec, WASM_IMPORT_LOG);
        imp_sec.push(0x00);
        write_uleb128(&mut imp_sec, 0);
        Self::push_section(&mut out, WASM_SEC_IMPORT, &imp_sec);
        let mut func_sec: Vec<u8> = Vec::new();
        write_uleb128(&mut func_sec, 3);
        write_uleb128(&mut func_sec, 1);
        write_uleb128(&mut func_sec, 1);
        write_uleb128(&mut func_sec, 2);
        Self::push_section(&mut out, WASM_SEC_FUNCTION, &func_sec);
        let payload1: &[u8] = self.get_ui_payload_json().as_bytes();
        let payload2: &[u8] = self.get_alt_ui_payload_json().as_bytes();
        let log_bytes: &[u8] = self.get_log_message().as_bytes();
        let ptr1: i32 = 256;
        let ptr2: i32 = ptr1 + (payload1.len() as i32) + 16;
        let ptr_log: i32 = ptr2 + (payload2.len() as i32) + 16;
        let total_mem_needed: usize = (ptr_log as usize) + log_bytes.len() + 256;
        let min_pages: u32 = ((total_mem_needed / WASM_PAGE_BYTES) as u32) + 1;
        let mut mem_sec: Vec<u8> = Vec::new();
        write_uleb128(&mut mem_sec, 1);
        mem_sec.push(0x00);
        write_uleb128(&mut mem_sec, min_pages);
        Self::push_section(&mut out, WASM_SEC_MEMORY, &mem_sec);
        let mut exp_sec: Vec<u8> = Vec::new();
        let exp_count: u32 = if self.get_export_render() { 3 } else { 2 };
        write_uleb128(&mut exp_sec, exp_count);
        if self.get_export_render() {
            Self::push_name(&mut exp_sec, WASM_EXPORT_RENDER);
            exp_sec.push(0x00);
            write_uleb128(&mut exp_sec, 2);
        }
        Self::push_name(&mut exp_sec, WASM_EXPORT_ON_EVENT);
        exp_sec.push(0x00);
        write_uleb128(&mut exp_sec, 3);
        Self::push_name(&mut exp_sec, WASM_EXPORT_ABI);
        exp_sec.push(0x00);
        write_uleb128(&mut exp_sec, 4);
        Self::push_section(&mut out, WASM_SEC_EXPORT, &exp_sec);
        let mut render_body: Vec<u8> = vec![0x00];
        match self.get_fault() {
            WasmFaultKind::InfiniteLoopOutOfFuel => {
                render_body.push(OP_LOOP);
                render_body.push(0x40);
                render_body.push(OP_BR);
                write_uleb128(&mut render_body, 0);
                render_body.push(OP_END);
                render_body.push(OP_I32_CONST);
                write_sleb128(&mut render_body, 0);
                render_body.push(OP_END);
            }
            WasmFaultKind::ExecutionTrap => {
                render_body.push(OP_UNREACHABLE);
                render_body.push(OP_END);
            }
            _ => {
                if !payload2.is_empty() {
                    render_body.push(OP_LOCAL_GET);
                    write_uleb128(&mut render_body, 0);
                    render_body.push(OP_IF);
                    render_body.push(0x40);
                    render_body.push(OP_I32_CONST);
                    write_sleb128(&mut render_body, ptr2);
                    render_body.push(OP_I32_CONST);
                    write_sleb128(&mut render_body, payload2.len() as i32);
                    render_body.push(OP_CALL);
                    write_uleb128(&mut render_body, 0);
                    render_body.push(OP_ELSE);
                    render_body.push(OP_I32_CONST);
                    write_sleb128(&mut render_body, ptr1);
                    render_body.push(OP_I32_CONST);
                    write_sleb128(&mut render_body, payload1.len() as i32);
                    render_body.push(OP_CALL);
                    write_uleb128(&mut render_body, 0);
                    render_body.push(OP_END);
                } else {
                    render_body.push(OP_I32_CONST);
                    write_sleb128(&mut render_body, ptr1);
                    render_body.push(OP_I32_CONST);
                    write_sleb128(&mut render_body, payload1.len() as i32);
                    render_body.push(OP_CALL);
                    write_uleb128(&mut render_body, 0);
                }
                if !log_bytes.is_empty() {
                    render_body.push(OP_I32_CONST);
                    write_sleb128(&mut render_body, ptr_log);
                    render_body.push(OP_I32_CONST);
                    write_sleb128(&mut render_body, log_bytes.len() as i32);
                    render_body.push(OP_CALL);
                    write_uleb128(&mut render_body, 1);
                }
                render_body.push(OP_I32_CONST);
                write_sleb128(&mut render_body, 1);
                render_body.push(OP_END);
            }
        }
        let mut event_body: Vec<u8> = vec![0x00];
        event_body.push(OP_LOCAL_GET);
        write_uleb128(&mut event_body, 0);
        event_body.push(OP_I32_CONST);
        write_sleb128(&mut event_body, 1);
        event_body.push(OP_I32_ADD);
        event_body.push(OP_END);
        let mut abi_body: Vec<u8> = vec![0x00];
        abi_body.push(OP_I32_CONST);
        write_sleb128(&mut abi_body, self.get_abi_version() as i32);
        abi_body.push(OP_END);
        let mut code_sec: Vec<u8> = Vec::new();
        write_uleb128(&mut code_sec, 3);
        for body in [&render_body, &event_body, &abi_body] {
            write_uleb128(&mut code_sec, body.len() as u32);
            code_sec.extend_from_slice(body);
        }
        Self::push_section(&mut out, WASM_SEC_CODE, &code_sec);
        let mut data_sec: Vec<u8> = Vec::new();
        let mut seg_count: u32 = 1;
        if !payload2.is_empty() {
            seg_count += 1;
        }
        if !log_bytes.is_empty() {
            seg_count += 1;
        }
        write_uleb128(&mut data_sec, seg_count);
        Self::push_data_segment(&mut data_sec, ptr1, payload1);
        if !payload2.is_empty() {
            Self::push_data_segment(&mut data_sec, ptr2, payload2);
        }
        if !log_bytes.is_empty() {
            Self::push_data_segment(&mut data_sec, ptr_log, log_bytes);
        }
        Self::push_section(&mut out, WASM_SEC_DATA, &data_sec);
        if !self.get_custom_meta().is_empty() {
            let mut custom_sec: Vec<u8> = Vec::new();
            Self::push_name(&mut custom_sec, WASM_CUSTOM_META_SECTION);
            custom_sec.extend_from_slice(self.get_custom_meta().as_slice());
            Self::push_section(&mut out, WASM_SEC_CUSTOM, &custom_sec);
        }
        out
    }

    /// Appends one active WASM linear memory data segment into `out`.
    ///
    /// # Arguments
    ///
    /// - `&mut Vec<u8>` - Data section accumulator.
    /// - `i32` - Linear memory base offset.
    /// - `&[u8]` - Data bytes to initialize.
    fn push_data_segment(out: &mut Vec<u8>, offset: i32, data: &[u8]) {
        write_uleb128(out, 0);
        out.push(OP_I32_CONST);
        write_sleb128(out, offset);
        out.push(OP_END);
        write_uleb128(out, data.len() as u32);
        out.extend_from_slice(data);
    }
}

/// Registry management methods for `RegistryWasmFetcher`.
impl RegistryWasmFetcher {
    /// Registers a `.wasm` payload under `url` with an ETag and optional fault behavior.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Registry fetcher reference.
    /// - `String` - Target bundle URL.
    /// - `Vec<u8>` - Raw `.wasm` bytes.
    /// - `String` - ETag identifier.
    /// - `WasmFaultKind` - Fault mode (`WasmFaultKind::None` for healthy delivery).
    pub fn register(&self, url: String, bytes: Vec<u8>, etag: String, fault: WasmFaultKind) {
        let Self { records, .. } = self;
        if let Ok(mut guard) = records.try_borrow_mut() {
            let _: Option<RegistryWasmRecord> =
                guard.insert(url, RegistryWasmRecord::new(bytes, etag, fault));
        }
    }

    /// Toggles simulated global network offline state.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Registry fetcher reference.
    /// - `bool` - When `true`, all fetch calls return `NetworkOffline`.
    pub fn set_offline(&self, is_offline: bool) {
        let Self { offline, .. } = self;
        if let Ok(mut guard) = offline.try_borrow_mut() {
            *guard = is_offline;
        }
    }

    /// Returns whether simulated offline mode is currently enabled.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Registry fetcher reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Offline flag.
    #[must_use]
    pub fn is_offline(&self) -> bool {
        let Self { offline, .. } = self;
        offline
            .try_borrow()
            .map(|g: Ref<'_, bool>| *g)
            .unwrap_or(false)
    }
}

/// `WasmFetcher` implementation serving verified `.wasm` payloads and deterministic fault cases across all 7 platforms.
impl WasmFetcher for RegistryWasmFetcher {
    /// Fetches a registered `.wasm` module with realistic chunked progress notifications and fault injection.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Registry fetcher reference.
    /// - `&WasmDownloadRequest` - Download request.
    /// - `u32` - Retry attempt number.
    /// - `&mut dyn FnMut(u64, u64)` - Streaming progress callback.
    ///
    /// # Returns
    ///
    /// - `Result<WasmFetchedPayload, WasmDynamicError>` - Downloaded payload or injected transport error.
    fn fetch(
        &self,
        request: &WasmDownloadRequest,
        attempt: u32,
        on_progress: &mut dyn FnMut(u64, u64),
    ) -> Result<WasmFetchedPayload, WasmDynamicError> {
        if self.is_offline() {
            return Err(WasmDynamicError::NetworkOffline(request.get_url().clone()));
        }
        let url_str: &str = request.get_url().as_str();
        if url_str.starts_with(WASM_SCHEME_HTTP) || url_str.starts_with(WASM_SCHEME_FILE) {
            return HttpWasmFetcher::default().fetch(request, attempt, on_progress);
        }
        let Self { records, .. } = self;
        let rec_opt: Result<(Vec<u8>, String, WasmFaultKind), WasmDynamicError> = {
            let guard: Ref<'_, HashMap<String, RegistryWasmRecord>> = records
                .try_borrow()
                .map_err(|_| WasmDynamicError::HttpStatus {
                    status: 404,
                    url: request.get_url().clone(),
                })?;
            let rec: &RegistryWasmRecord =
                guard
                    .get(request.get_url())
                    .ok_or_else(|| WasmDynamicError::HttpStatus {
                        status: 404,
                        url: request.get_url().clone(),
                    })?;
            match rec.get_fault() {
                WasmFaultKind::NetworkOffline => {
                    return Err(WasmDynamicError::NetworkOffline(request.get_url().clone()));
                }
                WasmFaultKind::TimeoutOnceThenSucceed if attempt == 0 => {
                    return Err(WasmDynamicError::Timeout {
                        url: request.get_url().clone(),
                        timeout_ms: WASM_DEFAULT_TIMEOUT_MS,
                    });
                }
                WasmFaultKind::Http500 => {
                    return Err(WasmDynamicError::HttpStatus {
                        status: 500,
                        url: request.get_url().clone(),
                    });
                }
                fault => Ok((rec.get_bytes().clone(), rec.get_etag().clone(), fault)),
            }
        };
        let (mut bytes, etag, fault): (Vec<u8>, String, WasmFaultKind) = rec_opt?;
        if fault == WasmFaultKind::TamperedChecksum && !bytes.is_empty() {
            let last_idx: usize = bytes.len() - 1;
            bytes[last_idx] ^= 0xFF;
        }
        let total: u64 = bytes.len() as u64;
        on_progress(total / 2, total);
        on_progress(total, total);
        let mut payload: WasmFetchedPayload = WasmFetchedPayload::new(bytes);
        payload.set_etag(etag);
        Ok(payload)
    }
}

/// Pure-std `http://` and `file://` transport implementation for `HttpWasmFetcher`.
impl WasmFetcher for HttpWasmFetcher {
    /// Downloads a `.wasm` binary over real TCP HTTP or local `file://` path with socket timeout and progress streaming.
    ///
    /// # Arguments
    ///
    /// - `&Self` - HTTP fetcher reference.
    /// - `&WasmDownloadRequest` - Target download descriptor.
    /// - `u32` - Retry attempt counter.
    /// - `&mut dyn FnMut(u64, u64)` - Byte progress callback.
    ///
    /// # Returns
    ///
    /// - `Result<WasmFetchedPayload, WasmDynamicError>` - Downloaded bytes or typed transport error.
    fn fetch(
        &self,
        request: &WasmDownloadRequest,
        _attempt: u32,
        on_progress: &mut dyn FnMut(u64, u64),
    ) -> Result<WasmFetchedPayload, WasmDynamicError> {
        let url: &str = request.get_url().as_str();
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = url.strip_prefix(WASM_SCHEME_FILE) {
                let bytes: Vec<u8> = fs::read(path).map_err(|_| WasmDynamicError::HttpStatus {
                    status: 404,
                    url: url.to_string(),
                })?;
                let total: u64 = bytes.len() as u64;
                on_progress(total, total);
                return Ok(WasmFetchedPayload::new(bytes));
            }
            if let Some(rest) = url.strip_prefix(WASM_SCHEME_HTTP) {
                let (host_port, req_path): (&str, String) = match rest.find('/') {
                    Some(idx) => (&rest[..idx], rest[idx..].to_string()),
                    None => (rest, "/".to_string()),
                };
                let connect_target: String = if host_port.contains(':') {
                    host_port.to_string()
                } else {
                    format!("{host_port}:80")
                };
                let timeout_ms: u64 = self.get_timeout_ms().max(1_000);
                let dur: Duration = Duration::from_millis(timeout_ms);
                let addr: SocketAddr = match connect_target.to_socket_addrs() {
                    Ok(mut it) => it.next(),
                    Err(_) => None,
                }
                .ok_or_else(|| WasmDynamicError::NetworkOffline(url.to_string()))?;
                let mut stream: TcpStream =
                    TcpStream::connect_timeout(&addr, dur).map_err(|_| {
                        WasmDynamicError::Timeout {
                            url: url.to_string(),
                            timeout_ms,
                        }
                    })?;
                let _: Result<(), io::Error> = stream.set_read_timeout(Some(dur));
                let _: Result<(), io::Error> = stream.set_write_timeout(Some(dur));
                let req_str: String = format!(
                    "GET {req_path} HTTP/1.1\r\nHost: {host_port}\r\nConnection: close\r\nAccept: application/wasm\r\n\r\n"
                );
                stream
                    .write_all(req_str.as_bytes())
                    .map_err(|_| WasmDynamicError::NetworkOffline(url.to_string()))?;
                let mut raw: Vec<u8> = Vec::new();
                let mut chunk: [u8; 4096] = [0; 4096];
                loop {
                    match stream.read(&mut chunk) {
                        Ok(0) => break,
                        Ok(n) => {
                            raw.extend_from_slice(&chunk[..n]);
                            on_progress(raw.len() as u64, 0);
                        }
                        Err(_) => {
                            return Err(WasmDynamicError::Timeout {
                                url: url.to_string(),
                                timeout_ms,
                            });
                        }
                    }
                }
                let sep_pos: usize = raw
                    .windows(4)
                    .position(|w: &[u8]| w == b"\r\n\r\n")
                    .ok_or_else(|| {
                        WasmDynamicError::InvalidWasmBinary("malformed HTTP response".to_string())
                    })?;
                let header_str: &str = from_utf8(&raw[..sep_pos]).unwrap_or("");
                let status_code: u16 = header_str
                    .lines()
                    .next()
                    .and_then(|line: &str| line.split_whitespace().nth(1))
                    .and_then(|code: &str| code.parse::<u16>().ok())
                    .unwrap_or(500);
                if !(200..300).contains(&status_code) {
                    return Err(WasmDynamicError::HttpStatus {
                        status: status_code,
                        url: url.to_string(),
                    });
                }
                let body: Vec<u8> = raw[sep_pos + 4..].to_vec();
                on_progress(body.len() as u64, body.len() as u64);
                return Ok(WasmFetchedPayload::new(body));
            }
            Err(WasmDynamicError::InvalidUrl(url.to_string()))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _: &mut dyn FnMut(u64, u64) = on_progress;
            Err(WasmDynamicError::NetworkOffline(url.to_string()))
        }
    }
}

/// Complete partial & global dynamic `.wasm` lifecycle orchestration for `WasmDynamicManager`.
impl WasmDynamicManager {
    /// Creates a `WasmDynamicManager` bound to `hub` and `fetcher` with `config`.
    ///
    /// # Arguments
    ///
    /// - `DynamicUpdateHub` - Target native scene dynamic update hub.
    /// - `Rc<dyn WasmFetcher>` - Transport fetcher for `.wasm` downloads.
    /// - `WasmSandboxConfig` - Sandbox limits and retry/cache configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized dynamic WASM manager.
    #[must_use]
    pub fn new(
        hub: DynamicUpdateHub,
        fetcher: Rc<dyn WasmFetcher>,
        config: WasmSandboxConfig,
    ) -> Self {
        Self {
            inner: Rc::new(RefCell::new(WasmDynamicManagerState {
                hub,
                fetcher,
                config,
                next_request_id: 0,
                active_request_ids: HashMap::new(),
                cache: HashMap::new(),
                parsed_cache: HashMap::new(),
                partial_stacks: HashMap::new(),
                global_stack: Vec::new(),
                state_signals: HashMap::new(),
                global_version_signal: Signal::new_detached(0),
                telemetry_signal: Signal::new_detached(WasmDynamicTelemetry::default()),
                last_logs: Signal::new_detached(Vec::new()),
            })),
        }
    }

    /// Returns the reactive `WasmLoadState` signal for `scope`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    /// - `&WasmDynamicScope` - Target partial slot or global scope.
    ///
    /// # Returns
    ///
    /// - `Signal<WasmLoadState>` - Live reactive lifecycle signal.
    #[must_use]
    pub fn state_signal(&self, scope: &WasmDynamicScope) -> Signal<WasmLoadState> {
        let key: String = scope.key();
        let Self { inner } = self;
        if let Ok(mut guard) = inner.try_borrow_mut() {
            return *guard
                .state_signals
                .entry(key)
                .or_insert_with(|| Signal::new_detached(WasmLoadState::Idle));
        }
        Signal::new_detached(WasmLoadState::Idle)
    }

    /// Returns the reactive global WASM active version signal (`0` = built-in native shell).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    ///
    /// # Returns
    ///
    /// - `Signal<u64>` - Global active version signal.
    #[must_use]
    pub fn global_version_signal(&self) -> Signal<u64> {
        let Self { inner } = self;
        inner
            .try_borrow()
            .map(|g: Ref<'_, WasmDynamicManagerState>| g.global_version_signal)
            .unwrap_or_else(|_| Signal::new_detached(0))
    }

    /// Returns the reactive `WasmDynamicTelemetry` signal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    ///
    /// # Returns
    ///
    /// - `Signal<WasmDynamicTelemetry>` - Cumulative telemetry signal.
    #[must_use]
    pub fn telemetry(&self) -> Signal<WasmDynamicTelemetry> {
        let Self { inner } = self;
        inner
            .try_borrow()
            .map(|g: Ref<'_, WasmDynamicManagerState>| g.telemetry_signal)
            .unwrap_or_else(|_| Signal::new_detached(WasmDynamicTelemetry::default()))
    }

    /// Returns the reactive signal carrying diagnostic logs from the last executed WASM module.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    ///
    /// # Returns
    ///
    /// - `Signal<Vec<String>>` - Diagnostic log lines signal.
    #[must_use]
    pub fn last_logs(&self) -> Signal<Vec<String>> {
        let Self { inner } = self;
        inner
            .try_borrow()
            .map(|g: Ref<'_, WasmDynamicManagerState>| g.last_logs)
            .unwrap_or_else(|_| Signal::new_detached(Vec::new()))
    }

    /// Returns the number of verified cached `.wasm` entries.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Cached module count.
    #[must_use]
    pub fn cache_len(&self) -> usize {
        let Self { inner } = self;
        inner
            .try_borrow()
            .map(|g: Ref<'_, WasmDynamicManagerState>| g.cache.len())
            .unwrap_or(0)
    }

    /// Cancels any active request for `scope`, advancing its generation ID so stale responses are discarded.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    /// - `&WasmDynamicScope` - Scope to cancel.
    ///
    /// # Returns
    ///
    /// - `u64` - Newly allocated cancellation generation ID.
    pub fn cancel_scope(&self, scope: &WasmDynamicScope) -> u64 {
        let state_sig: Signal<WasmLoadState> = self.state_signal(scope);
        let key: String = scope.key();
        let Self { inner } = self;
        let next_id: u64 = if let Ok(mut guard) = inner.try_borrow_mut() {
            guard.next_request_id = guard.next_request_id.saturating_add(1);
            let id: u64 = guard.next_request_id;
            let _: Option<u64> = guard.active_request_ids.insert(key, id);
            id
        } else {
            0
        };
        let _: bool = state_sig.set(WasmLoadState::Failed(WasmDynamicError::Cancelled {
            request_id: next_id,
        }));
        next_id
    }

    /// Downloads, verifies, compiles, executes in sandbox, and natively mounts a partial or global `.wasm` module.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    /// - `WasmDownloadRequest` - Target URL, scope, version, checksum, and signature policy.
    /// - `&str` - Signing key used when verifying `expected_signature`.
    ///
    /// # Returns
    ///
    /// - `Result<u64, WasmDynamicError>` - Mounted version number on success, or typed error (with automatic global LKG rollback).
    pub fn load_and_mount(
        &self,
        request: WasmDownloadRequest,
        signing_key: &str,
    ) -> Result<u64, WasmDynamicError> {
        let url_str: &str = request.get_url().trim();
        let valid_scheme: bool = url_str.starts_with("https://")
            || url_str.starts_with(WASM_SCHEME_HTTP)
            || url_str.starts_with("euv-cdn://")
            || url_str.starts_with(WASM_SCHEME_FILE);
        if url_str.is_empty() || !valid_scheme {
            let err: WasmDynamicError = WasmDynamicError::InvalidUrl(request.get_url().clone());
            self.record_failure(request.get_scope(), &err);
            return Err(err);
        }
        let scope: WasmDynamicScope = request.get_scope().clone();
        let scope_key: String = scope.key();
        let (req_id, config, fetcher, active_ver, state_sig): (
            u64,
            WasmSandboxConfig,
            Rc<dyn WasmFetcher>,
            u64,
            Signal<WasmLoadState>,
        ) = {
            let Self { inner } = self;
            let mut guard: RefMut<'_, WasmDynamicManagerState> = inner
                .try_borrow_mut()
                .map_err(|_| WasmDynamicError::ExecutionTrap("manager re-entrancy".to_string()))?;
            let sig: Signal<WasmLoadState> = *guard
                .state_signals
                .entry(scope_key.clone())
                .or_insert_with(|| Signal::new_detached(WasmLoadState::Idle));
            guard.next_request_id = guard.next_request_id.saturating_add(1);
            let id: u64 = guard.next_request_id;
            let _: Option<u64> = guard.active_request_ids.insert(scope_key.clone(), id);
            let cur_v: u64 = match &scope {
                WasmDynamicScope::Partial { slot_id } => guard
                    .partial_stacks
                    .get(slot_id)
                    .and_then(|s: &Vec<WasmMountedSnapshot>| s.last())
                    .map(WasmMountedSnapshot::get_version)
                    .unwrap_or(0),
                WasmDynamicScope::Global => guard
                    .global_stack
                    .last()
                    .map(WasmMountedSnapshot::get_version)
                    .unwrap_or(0),
            };
            let _: bool = guard
                .telemetry_signal
                .update(|t: &mut WasmDynamicTelemetry| {
                    t.downloads_started = t.downloads_started.saturating_add(1);
                });
            (id, guard.config, guard.fetcher.clone(), cur_v, sig)
        };
        if !request.get_allow_downgrade() && active_ver > 0 && request.get_version() <= active_ver {
            let err: WasmDynamicError = WasmDynamicError::VersionRegression {
                current: active_ver,
                incoming: request.get_version(),
            };
            self.record_failure(&scope, &err);
            return Err(err);
        }
        if state_sig.has_subscribers() {
            let _: bool = state_sig.set(WasmLoadState::Downloading {
                received_bytes: 0,
                total_bytes: 0,
            });
        }
        let (fetched, cached_parsed_opt): (WasmFetchedShare, Option<Rc<WasmParsedModule>>) =
            match self.fetch_with_retry(&request, config, &fetcher, state_sig) {
                Ok(res) => res,
                Err(err) => {
                    self.record_failure(&scope, &err);
                    return Err(err);
                }
            };
        let current_active_id: u64 = self.active_request_id(&scope_key);
        if current_active_id != req_id {
            let err: WasmDynamicError = WasmDynamicError::StaleResponse {
                request_id: req_id,
                active_request_id: current_active_id,
            };
            return Err(err);
        }
        if state_sig.has_subscribers() {
            let _: bool = state_sig.set(WasmLoadState::Verifying);
        }
        let raw_bytes: &[u8] = fetched.bytes.as_slice();
        if raw_bytes.len() > config.get_max_binary_bytes() {
            let err: WasmDynamicError = WasmDynamicError::BinaryTooLarge {
                actual: raw_bytes.len(),
                limit: config.get_max_binary_bytes(),
            };
            self.record_failure(&scope, &err);
            return Err(err);
        }
        let computed_checksum: u64 = compute_wasm_checksum(raw_bytes);
        if request.get_expected_checksum() != 0
            && computed_checksum != request.get_expected_checksum()
        {
            let err: WasmDynamicError = WasmDynamicError::ChecksumMismatch {
                expected: request.get_expected_checksum(),
                actual: computed_checksum,
            };
            self.record_failure(&scope, &err);
            return Err(err);
        }
        if config.get_require_signature() || !request.get_expected_signature().is_empty() {
            let computed_sig: String =
                compute_wasm_signature(raw_bytes, request.get_version(), signing_key);
            if computed_sig != *request.get_expected_signature() {
                let err: WasmDynamicError = WasmDynamicError::SignatureInvalid {
                    expected: request.get_expected_signature().clone(),
                    actual: computed_sig,
                };
                self.record_failure(&scope, &err);
                return Err(err);
            }
        }
        if state_sig.has_subscribers() {
            let _: bool = state_sig.set(WasmLoadState::Compiling);
        }
        let cached_module: Option<Rc<WasmParsedModule>> = cached_parsed_opt.or_else(|| {
            self.lookup_cached_parsed_module(
                request.resolved_cache_key(),
                request.get_url().as_str(),
                computed_checksum,
            )
        });
        let parsed: Rc<WasmParsedModule> = match cached_module {
            Some(hit) => hit,
            None => match parse_wasm_binary(raw_bytes, &config) {
                Ok(m) => Rc::new(m),
                Err(err) => {
                    self.record_failure(&scope, &err);
                    return Err(err);
                }
            },
        };
        if parsed.get_exports().contains_key(WASM_EXPORT_ABI) {
            let abi_res: Result<WasmExecutionOutput, WasmDynamicError> =
                execute_wasm_export(parsed.as_ref(), WASM_EXPORT_ABI, &[], &config);
            match abi_res {
                Ok(abi_out) => {
                    let mod_abi: u32 = abi_out.get_return_value().max(0) as u32;
                    if mod_abi != WASM_ABI_VERSION {
                        let err: WasmDynamicError = WasmDynamicError::AbiMismatch {
                            expected: WASM_ABI_VERSION,
                            actual: mod_abi,
                        };
                        self.record_failure(&scope, &err);
                        return Err(err);
                    }
                }
                Err(err) => {
                    self.record_failure(&scope, &err);
                    return Err(err);
                }
            }
        }
        if state_sig.has_subscribers() {
            let _: bool = state_sig.set(WasmLoadState::Executing);
        }
        let exec_out: WasmExecutionOutput =
            match execute_wasm_export(parsed.as_ref(), WASM_EXPORT_RENDER, &[0], &config) {
                Ok(out) => out,
                Err(err) => {
                    self.record_failure(&scope, &err);
                    return Err(err);
                }
            };
        self.commit_mounted_output(
            &request,
            fetched,
            computed_checksum,
            parsed,
            exec_out,
            state_sig,
        )
    }

    /// Executes an interactive click/event inside a mounted WASM module (`euv_on_event`), re-rendering native UI live.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    /// - `&WasmDynamicScope` - Mounted partial or global WASM scope.
    /// - `i32` - Action code passed into `euv_on_event`.
    ///
    /// # Returns
    ///
    /// - `Result<i32, WasmDynamicError>` - Updated WASM module state value or sandbox error.
    pub fn dispatch_wasm_event(
        &self,
        scope: &WasmDynamicScope,
        action_code: i32,
    ) -> Result<i32, WasmDynamicError> {
        let (config, hub, snap_version, snap_state, snap_module): (
            WasmSandboxConfig,
            DynamicUpdateHub,
            u64,
            i32,
            Rc<WasmParsedModule>,
        ) = {
            let Self { inner } = self;
            let guard: Ref<'_, WasmDynamicManagerState> = inner.try_borrow().map_err(|_| {
                WasmDynamicError::ExecutionTrap(WASM_ERR_MANAGER_BORROW.to_string())
            })?;
            let snap: &WasmMountedSnapshot = match scope {
                WasmDynamicScope::Partial { slot_id } => guard
                    .partial_stacks
                    .get(slot_id)
                    .and_then(|s: &Vec<WasmMountedSnapshot>| s.last())
                    .ok_or(WasmDynamicError::NoRollbackSnapshot)?,
                WasmDynamicScope::Global => guard
                    .global_stack
                    .last()
                    .ok_or(WasmDynamicError::NoRollbackSnapshot)?,
            };
            (
                guard.config,
                guard.hub.clone(),
                snap.get_version(),
                snap.get_state_value(),
                Rc::clone(snap.get_module()),
            )
        };
        let next_state: i32 = if snap_module.get_exports().contains_key(WASM_EXPORT_ON_EVENT) {
            let ev_out: WasmExecutionOutput = execute_wasm_export(
                snap_module.as_ref(),
                WASM_EXPORT_ON_EVENT,
                &[snap_state],
                &config,
            )?;
            let _: i32 = action_code;
            ev_out.get_return_value() % 2
        } else {
            (snap_state + 1) % 2
        };
        let render_out: WasmExecutionOutput = execute_wasm_export(
            snap_module.as_ref(),
            WASM_EXPORT_RENDER,
            &[next_state],
            &config,
        )?;
        let WasmExecutionOutput {
            emitted_spec,
            emitted_bundle,
            ..
        } = render_out;
        match scope {
            WasmDynamicScope::Partial { slot_id } => {
                if let Some(spec) = emitted_spec {
                    let mut upd: DynamicComponentUpdate =
                        DynamicComponentUpdate::new(slot_id.clone(), snap_version, spec);
                    upd.set_allow_downgrade(true);
                    upd.sign();
                    let _: Result<u64, DynamicUpdateError> = hub.apply_component(upd);
                }
            }
            WasmDynamicScope::Global => {
                if let Some(mut patched) = emitted_bundle {
                    for comp in patched.get_mut_components().iter_mut() {
                        comp.set_allow_downgrade(true);
                        comp.sign();
                    }
                    for page in patched.get_mut_pages().iter_mut() {
                        page.set_allow_downgrade(true);
                        page.sign();
                    }
                    let _: Result<DynamicTelemetry, DynamicUpdateError> = hub.apply_bundle(patched);
                }
            }
        }
        let Self { inner } = self;
        if let Ok(mut guard) = inner.try_borrow_mut() {
            match scope {
                WasmDynamicScope::Partial { slot_id } => {
                    if let Some(stack) = guard.partial_stacks.get_mut(slot_id)
                        && let Some(top) = stack.last_mut()
                    {
                        top.set_state_value(next_state);
                    }
                }
                WasmDynamicScope::Global => {
                    if let Some(top) = guard.global_stack.last_mut() {
                        top.set_state_value(next_state);
                    }
                }
            }
            let _: bool = guard
                .telemetry_signal
                .update(|t: &mut WasmDynamicTelemetry| {
                    t.events_handled = t.events_handled.saturating_add(1);
                });
        }
        Ok(next_state)
    }

    /// Rolls back a partial slot to its previous verified WASM version or compiled native fallback.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    /// - `&str` - Target partial slot ID.
    /// - `&str` - Human-readable rollback reason.
    ///
    /// # Returns
    ///
    /// - `Result<u64, WasmDynamicError>` - Active version after rollback (`0` = compiled fallback).
    pub fn rollback_partial(&self, slot_id: &str, reason: &str) -> Result<u64, WasmDynamicError> {
        let scope: WasmDynamicScope = WasmDynamicScope::Partial {
            slot_id: slot_id.to_string(),
        };
        let state_sig: Signal<WasmLoadState> = self.state_signal(&scope);
        let (hub, next_ver): (DynamicUpdateHub, u64) = {
            let Self { inner } = self;
            let mut guard: RefMut<'_, WasmDynamicManagerState> =
                inner.try_borrow_mut().map_err(|_| {
                    WasmDynamicError::ExecutionTrap(WASM_ERR_MANAGER_BORROW.to_string())
                })?;
            let stack: &mut Vec<WasmMountedSnapshot> = guard
                .partial_stacks
                .get_mut(slot_id)
                .ok_or(WasmDynamicError::NoRollbackSnapshot)?;
            if stack.pop().is_none() {
                return Err(WasmDynamicError::NoRollbackSnapshot);
            }
            let ver: u64 = stack
                .last()
                .map(WasmMountedSnapshot::get_version)
                .unwrap_or(0);
            let _: bool = guard
                .telemetry_signal
                .update(|t: &mut WasmDynamicTelemetry| {
                    t.rollbacks_executed = t.rollbacks_executed.saturating_add(1);
                });
            (guard.hub.clone(), ver)
        };
        let _: Result<u64, DynamicUpdateError> = hub.rollback_component(slot_id);
        let _: bool = state_sig.set(WasmLoadState::RolledBack {
            active_version: next_ver,
            reason: reason.to_string(),
        });
        Ok(next_ver)
    }

    /// Unmounts a partial WASM slot completely, freeing its snapshot stack and restoring the compiled native fallback.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    /// - `&str` - Target partial slot ID.
    pub fn unmount_partial(&self, slot_id: &str) {
        let scope: WasmDynamicScope = WasmDynamicScope::Partial {
            slot_id: slot_id.to_string(),
        };
        let state_sig: Signal<WasmLoadState> = self.state_signal(&scope);
        let hub: Option<DynamicUpdateHub> = {
            let Self { inner } = self;
            if let Ok(mut guard) = inner.try_borrow_mut() {
                let _: Option<Vec<WasmMountedSnapshot>> = guard.partial_stacks.remove(slot_id);
                Some(guard.hub.clone())
            } else {
                None
            }
        };
        if let Some(h) = hub {
            while h.rollback_component(slot_id).is_ok() {}
        }
        let _: bool = state_sig.set(WasmLoadState::Idle);
    }

    /// Rolls back the global full-app WASM shell to the previous LKG snapshot (or built-in native shell when empty).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    /// - `&str` - Rollback reason string.
    ///
    /// # Returns
    ///
    /// - `Result<u64, WasmDynamicError>` - Remaining global version (`0` = built-in native shell).
    pub fn rollback_global(&self, reason: &str) -> Result<u64, WasmDynamicError> {
        let state_sig: Signal<WasmLoadState> = self.state_signal(&WasmDynamicScope::Global);
        let (hub, global_sig, next_ver): (DynamicUpdateHub, Signal<u64>, u64) = {
            let Self { inner } = self;
            let mut guard: RefMut<'_, WasmDynamicManagerState> =
                inner.try_borrow_mut().map_err(|_| {
                    WasmDynamicError::ExecutionTrap(WASM_ERR_MANAGER_BORROW.to_string())
                })?;
            if guard.global_stack.pop().is_none() {
                return Err(WasmDynamicError::NoRollbackSnapshot);
            }
            let ver: u64 = guard
                .global_stack
                .last()
                .map(WasmMountedSnapshot::get_version)
                .unwrap_or(0);
            let _: bool = guard
                .telemetry_signal
                .update(|t: &mut WasmDynamicTelemetry| {
                    t.rollbacks_executed = t.rollbacks_executed.saturating_add(1);
                });
            (guard.hub.clone(), guard.global_version_signal, ver)
        };
        let _: Result<u64, DynamicUpdateError> = hub.rollback_component(WASM_GLOBAL_ROOT_SLOT);
        let _: bool = global_sig.set_if_changed(next_ver);
        let _: bool = state_sig.set(WasmLoadState::RolledBack {
            active_version: next_ver,
            reason: reason.to_string(),
        });
        Ok(next_ver)
    }

    /// Restores the built-in compiled native application shell, clearing all global WASM overlays cleanly.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    pub fn restore_native_global(&self) {
        let state_sig: Signal<WasmLoadState> = self.state_signal(&WasmDynamicScope::Global);
        let pair: Option<(DynamicUpdateHub, Signal<u64>)> = {
            let Self { inner } = self;
            if let Ok(mut guard) = inner.try_borrow_mut() {
                guard.global_stack.clear();
                let _: bool = guard
                    .telemetry_signal
                    .update(|t: &mut WasmDynamicTelemetry| {
                        t.rollbacks_executed = t.rollbacks_executed.saturating_add(1);
                    });
                Some((guard.hub.clone(), guard.global_version_signal))
            } else {
                None
            }
        };
        if let Some((hub, global_sig)) = pair {
            while hub.rollback_component(WASM_GLOBAL_ROOT_SLOT).is_ok() {}
            let _: bool = global_sig.set_if_changed(0);
        }
        let _: bool = state_sig.set(WasmLoadState::RolledBack {
            active_version: 0,
            reason: "restored built-in native shell".to_string(),
        });
    }

    /// Returns the currently active request generation ID for `scope_key`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    /// - `&str` - Scope registry key.
    ///
    /// # Returns
    ///
    /// - `u64` - Active request ID.
    #[must_use]
    fn active_request_id(&self, scope_key: &str) -> u64 {
        let Self { inner } = self;
        inner
            .try_borrow()
            .ok()
            .and_then(|g: Ref<'_, WasmDynamicManagerState>| {
                g.active_request_ids.get(scope_key).copied()
            })
            .unwrap_or(0)
    }

    /// Looks up a previously validated `Rc<WasmParsedModule>` matching `(url, checksum)` without re-parsing binary bytes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    /// - `&str` - Resolved cache key.
    /// - `&str` - Source module URL.
    /// - `u64` - Verified FNV-1a checksum of the incoming bytes.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<WasmParsedModule>>` - Shared cached parsed module if `(url, checksum)` already validated.
    #[must_use]
    fn lookup_cached_parsed_module(
        &self,
        cache_key: &str,
        url: &str,
        checksum: u64,
    ) -> Option<Rc<WasmParsedModule>> {
        let Self { inner } = self;
        let guard: Ref<'_, WasmDynamicManagerState> = inner.try_borrow().ok()?;
        if let Some(entry) = guard.cache.get(cache_key)
            && entry.get_checksum() == checksum
            && entry.get_url().as_str() == url
        {
            return Some(Rc::clone(entry.get_parsed()));
        }
        if guard.parsed_cache.is_empty() {
            return None;
        }
        guard
            .parsed_cache
            .get(&(url.to_string(), checksum))
            .map(Rc::clone)
    }

    /// Executes the download with exponential retry on transient failures and zero-clone verified offline cache fallback.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    /// - `&WasmDownloadRequest` - Target download request.
    /// - `WasmSandboxConfig` - Retry and offline cache settings.
    /// - `&Rc<dyn WasmFetcher>` - Transport implementation.
    /// - `Signal<WasmLoadState>` - Live progress signal.
    ///
    /// # Returns
    ///
    /// - `Result<(WasmFetchedShare, Option<Rc<WasmParsedModule>>), WasmDynamicError>` - Shared payload bundle and optional cached parsed module.
    fn fetch_with_retry(
        &self,
        request: &WasmDownloadRequest,
        config: WasmSandboxConfig,
        fetcher: &Rc<dyn WasmFetcher>,
        state_sig: Signal<WasmLoadState>,
    ) -> Result<(WasmFetchedShare, Option<Rc<WasmParsedModule>>), WasmDynamicError> {
        let mut last_err: WasmDynamicError =
            WasmDynamicError::NetworkOffline(request.get_url().clone());
        let max_attempts: u32 = config.get_max_retries().saturating_add(1);
        for attempt in 0..max_attempts {
            if attempt > 0 {
                let Self { inner } = self;
                if let Ok(guard) = inner.try_borrow() {
                    let _: bool = guard
                        .telemetry_signal
                        .update(|t: &mut WasmDynamicTelemetry| {
                            t.retries_performed = t.retries_performed.saturating_add(1);
                        });
                }
            }
            match fetcher.fetch(request, attempt, &mut |received: u64, total: u64| {
                if state_sig.has_subscribers() {
                    let _: bool = state_sig.set(WasmLoadState::Downloading {
                        received_bytes: received,
                        total_bytes: total,
                    });
                }
            }) {
                Ok(payload) => {
                    let WasmFetchedPayload {
                        bytes,
                        etag,
                        from_cache,
                    } = payload;
                    let shared: WasmFetchedShare = WasmFetchedShare {
                        bytes: Rc::new(bytes),
                        etag,
                        from_cache,
                    };
                    return Ok((shared, None));
                }
                Err(err) => {
                    let retryable: bool = matches!(
                        err,
                        WasmDynamicError::Timeout { .. }
                            | WasmDynamicError::HttpStatus {
                                status: 500..=599,
                                ..
                            }
                    );
                    last_err = err;
                    if !retryable {
                        break;
                    }
                }
            }
        }
        if config.get_allow_offline_cache() {
            let Self { inner } = self;
            let hit: Option<(WasmFetchedShare, Rc<WasmParsedModule>)> = if let Ok(guard) =
                inner.try_borrow()
            {
                if let Some(cached) = guard.cache.get(request.resolved_cache_key()) {
                    let _: bool = guard
                        .telemetry_signal
                        .update(|t: &mut WasmDynamicTelemetry| {
                            t.cache_hits = t.cache_hits.saturating_add(1);
                        });
                    let shared: WasmFetchedShare = WasmFetchedShare {
                        bytes: Rc::clone(cached.get_bytes()),
                        etag: cached.get_etag().clone(),
                        from_cache: true,
                    };
                    Some((shared, Rc::clone(cached.get_parsed())))
                } else {
                    None
                }
            } else {
                None
            };
            if let Some((shared, parsed)) = hit {
                return Ok((shared, Some(parsed)));
            }
        }
        Err(last_err)
    }

    /// Commits a verified executed WASM output into `DynamicUpdateHub`, updates cache & rollback stacks, and emits telemetry.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    /// - `&WasmDownloadRequest` - Origin request.
    /// - `WasmFetchedShare` - Shared verified `.wasm` payload bundle.
    /// - `u64` - Verified FNV-1a checksum.
    /// - `Rc<WasmParsedModule>` - Shared validated `.wasm` module.
    /// - `WasmExecutionOutput` - Executed render output.
    /// - `Signal<WasmLoadState>` - Live scope status signal.
    ///
    /// # Returns
    ///
    /// - `Result<u64, WasmDynamicError>` - Mounted version number.
    fn commit_mounted_output(
        &self,
        request: &WasmDownloadRequest,
        fetched: WasmFetchedShare,
        checksum: u64,
        parsed: Rc<WasmParsedModule>,
        exec_out: WasmExecutionOutput,
        state_sig: Signal<WasmLoadState>,
    ) -> Result<u64, WasmDynamicError> {
        let WasmFetchedShare {
            bytes: fetched_bytes,
            etag: fetched_etag,
            from_cache,
        } = fetched;
        let ver: u64 = request.get_version();
        let WasmExecutionOutput {
            fuel_used,
            emitted_spec,
            emitted_bundle,
            logs,
            ..
        } = exec_out;
        let hub: DynamicUpdateHub = {
            let Self { inner } = self;
            inner
                .try_borrow()
                .map(|g: Ref<'_, WasmDynamicManagerState>| g.hub.clone())
                .map_err(|_| WasmDynamicError::ExecutionTrap(WASM_ERR_MANAGER_BORROW.to_string()))?
        };
        let (spec_opt, bundle_opt): (Option<DynamicNodeSpec>, Option<DynamicUpdateBundle>) =
            match request.get_scope() {
                WasmDynamicScope::Partial { slot_id } => {
                    let spec: DynamicNodeSpec = emitted_spec.ok_or_else(|| {
                        WasmDynamicError::InvalidUiPayload(
                            "partial wasm did not call env::euv_emit_ui".to_string(),
                        )
                    })?;
                    let mut upd: DynamicComponentUpdate =
                        DynamicComponentUpdate::new(slot_id.clone(), ver, spec.clone());
                    upd.set_allow_downgrade(request.get_allow_downgrade());
                    upd.sign();
                    hub.apply_component(upd).map_err(|e: DynamicUpdateError| {
                        WasmDynamicError::InvalidUiPayload(e.to_string())
                    })?;
                    (Some(spec), None)
                }
                WasmDynamicScope::Global => {
                    let bundle: DynamicUpdateBundle = emitted_bundle.ok_or_else(|| {
                        WasmDynamicError::InvalidUiPayload(
                            "global wasm did not call env::euv_emit_bundle".to_string(),
                        )
                    })?;
                    let mut patched: DynamicUpdateBundle = bundle.clone();
                    if request.get_allow_downgrade() {
                        for comp in patched.get_mut_components().iter_mut() {
                            comp.set_allow_downgrade(true);
                        }
                        for page in patched.get_mut_pages().iter_mut() {
                            page.set_allow_downgrade(true);
                        }
                    }
                    let _: DynamicTelemetry =
                        hub.apply_bundle(patched).map_err(|e: DynamicUpdateError| {
                            WasmDynamicError::InvalidUiPayload(e.to_string())
                        })?;
                    (None, Some(bundle))
                }
            };
        let snap: WasmMountedSnapshot = WasmMountedSnapshot {
            version: ver,
            url: request.get_url().clone(),
            checksum,
            module: Rc::clone(&parsed),
            state_value: 0,
            spec: spec_opt,
            bundle: bundle_opt,
        };
        let Self { inner } = self;
        if let Ok(mut guard) = inner.try_borrow_mut() {
            let cache_key: &str = request.resolved_cache_key();
            let same_cached: bool =
                guard
                    .cache
                    .get(cache_key)
                    .is_some_and(|e: &WasmCachedEntry| {
                        e.get_checksum() == checksum && e.get_url() == request.get_url()
                    });
            if same_cached {
                if let Some(existing) = guard.cache.get_mut(cache_key) {
                    existing.set_version(ver);
                    existing.set_etag(fetched_etag);
                }
            } else {
                let cache_entry: WasmCachedEntry = WasmCachedEntry::new(
                    request.get_url().clone(),
                    ver,
                    checksum,
                    fetched_bytes,
                    Rc::clone(&parsed),
                    fetched_etag,
                );
                let _: Option<WasmCachedEntry> =
                    guard.cache.insert(cache_key.to_string(), cache_entry);
                let _: Option<Rc<WasmParsedModule>> = guard
                    .parsed_cache
                    .insert((request.get_url().clone(), checksum), parsed);
            }
            match request.get_scope() {
                WasmDynamicScope::Partial { slot_id } => {
                    guard
                        .partial_stacks
                        .entry(slot_id.clone())
                        .or_default()
                        .push(snap);
                }
                WasmDynamicScope::Global => {
                    guard.global_stack.push(snap);
                    let _: bool = guard.global_version_signal.set_if_changed(ver);
                }
            }
            let _: bool = guard.last_logs.set_if_changed(logs);
            let _: bool = guard
                .telemetry_signal
                .update(|t: &mut WasmDynamicTelemetry| {
                    t.downloads_succeeded = t.downloads_succeeded.saturating_add(1);
                });
        }
        let _: bool = state_sig.set(WasmLoadState::Mounted {
            version: ver,
            from_cache,
            fuel_used,
        });
        Ok(ver)
    }

    /// Records telemetry counters and updates scope state (with automatic Global LKG rollback signaling).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Manager reference.
    /// - `&WasmDynamicScope` - Failing scope.
    /// - `&WasmDynamicError` - Typed error.
    fn record_failure(&self, scope: &WasmDynamicScope, err: &WasmDynamicError) {
        let state_sig: Signal<WasmLoadState> = self.state_signal(scope);
        let mut active_global_lkg: u64 = 0;
        let Self { inner } = self;
        if let Ok(guard) = inner.try_borrow() {
            let _: bool = guard
                .telemetry_signal
                .update(|t: &mut WasmDynamicTelemetry| match err {
                    WasmDynamicError::OutOfFuel { .. } => {
                        t.fuel_exhaustions_caught = t.fuel_exhaustions_caught.saturating_add(1);
                    }
                    WasmDynamicError::ExecutionTrap(_)
                    | WasmDynamicError::MemoryOutOfBounds { .. }
                    | WasmDynamicError::MemoryLimitExceeded { .. } => {
                        t.sandbox_traps_caught = t.sandbox_traps_caught.saturating_add(1);
                    }
                    WasmDynamicError::ChecksumMismatch { .. }
                    | WasmDynamicError::SignatureInvalid { .. }
                    | WasmDynamicError::InvalidWasmBinary(_)
                    | WasmDynamicError::AbiMismatch { .. }
                    | WasmDynamicError::MissingExport(_)
                    | WasmDynamicError::BinaryTooLarge { .. }
                    | WasmDynamicError::VersionRegression { .. } => {
                        t.integrity_rejections = t.integrity_rejections.saturating_add(1);
                    }
                    _ => {}
                });
            if *scope == WasmDynamicScope::Global {
                active_global_lkg = guard
                    .global_stack
                    .last()
                    .map(WasmMountedSnapshot::get_version)
                    .unwrap_or(0);
            }
        }
        if *scope == WasmDynamicScope::Global && active_global_lkg > 0 {
            let Self { inner } = self;
            if let Ok(guard) = inner.try_borrow() {
                let _: bool = guard
                    .telemetry_signal
                    .update(|t: &mut WasmDynamicTelemetry| {
                        t.rollbacks_executed = t.rollbacks_executed.saturating_add(1);
                    });
            }
            let _: bool = state_sig.set(WasmLoadState::RolledBack {
                active_version: active_global_lkg,
                reason: format!("auto-kept LKG v{active_global_lkg} after error: {err}"),
            });
        } else {
            let _: bool = state_sig.set(WasmLoadState::Failed(err.clone()));
        }
    }
}
