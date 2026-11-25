use super::*;

/// Computes a deterministic 64-bit FNV-1a checksum over raw `.wasm` binary bytes.
///
/// # Arguments
///
/// - `&[u8]` - Raw `.wasm` binary slice.
///
/// # Returns
///
/// - `u64` - Deterministic non-zero digest.
#[must_use]
pub fn compute_wasm_checksum(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes.iter() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash.max(1)
}

/// Computes a deterministic keyed integrity signature tag for a `.wasm` binary and release version.
///
/// # Arguments
///
/// - `&[u8]` - Raw `.wasm` binary bytes.
/// - `u64` - Release version number.
/// - `&str` - Shared verification key or signing secret.
///
/// # Returns
///
/// - `String` - Formatted signature tag (`"euv-sig-v1:<hex>"`).
#[must_use]
pub fn compute_wasm_signature(bytes: &[u8], version: u64, signing_key: &str) -> String {
    let mut h1: u64 = 0xcbf2_9ce4_8422_2325 ^ version.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let mut h2: u64 = 0x8422_2325_cbf2_9ce4 ^ (bytes.len() as u64);
    for byte in signing_key.as_bytes().iter() {
        h1 ^= *byte as u64;
        h1 = h1.wrapping_mul(0x0100_0000_01b3);
        h2 = h2.rotate_left(7) ^ (*byte as u64);
    }
    for byte in bytes.iter() {
        h1 ^= *byte as u64;
        h1 = h1.wrapping_mul(0x0100_0000_01b3);
        h2 ^= (*byte as u64).wrapping_mul(0x517c_c1b7_2722_0a95);
        h2 = h2.rotate_left(13);
    }
    format!("{WASM_SIGNATURE_PREFIX}{h1:016x}{h2:016x}")
}

/// Encodes an unsigned 32-bit integer as LEB128 into `out`.
///
/// # Arguments
///
/// - `&mut Vec<u8>` - Destination byte vector.
/// - `u32` - Unsigned integer to encode.
pub(crate) fn write_uleb128(out: &mut Vec<u8>, mut value: u32) {
    loop {
        let mut byte: u8 = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

/// Encodes a signed 32-bit integer as SLEB128 into `out`.
///
/// # Arguments
///
/// - `&mut Vec<u8>` - Destination byte vector.
/// - `i32` - Signed integer to encode.
pub(crate) fn write_sleb128(out: &mut Vec<u8>, mut value: i32) {
    loop {
        let byte: u8 = (value & 0x7F) as u8;
        value >>= 7;
        let done: bool = (value == 0 && (byte & 0x40) == 0) || (value == -1 && (byte & 0x40) != 0);
        if done {
            out.push(byte);
            break;
        }
        out.push(byte | 0x80);
    }
}

/// Decodes an unsigned 32-bit LEB128 integer starting at `*cursor` in `bytes`.
///
/// # Arguments
///
/// - `&[u8]` - Source WASM byte slice.
/// - `&mut usize` - Read cursor advanced past the encoded integer.
///
/// # Returns
///
/// - `Result<u32, WasmDynamicError>` - Decoded integer or `InvalidWasmBinary`.
pub(crate) fn read_uleb128(bytes: &[u8], cursor: &mut usize) -> Result<u32, WasmDynamicError> {
    let mut result: u32 = 0;
    let mut shift: u32 = 0;
    loop {
        let byte: u8 = *bytes.get(*cursor).ok_or_else(|| {
            WasmDynamicError::InvalidWasmBinary("unexpected EOF in uleb128".to_string())
        })?;
        *cursor += 1;
        if shift >= 35 {
            return Err(WasmDynamicError::InvalidWasmBinary(
                "uleb128 integer overflow".to_string(),
            ));
        }
        result |= ((byte & 0x7F) as u32).wrapping_shl(shift);
        if (byte & 0x80) == 0 {
            return Ok(result);
        }
        shift += 7;
    }
}

/// Decodes a signed 32-bit SLEB128 integer starting at `*cursor` in `bytes`.
///
/// # Arguments
///
/// - `&[u8]` - Source WASM byte slice.
/// - `&mut usize` - Read cursor advanced past the encoded integer.
///
/// # Returns
///
/// - `Result<i32, WasmDynamicError>` - Decoded signed integer or `InvalidWasmBinary`.
pub(crate) fn read_sleb128(bytes: &[u8], cursor: &mut usize) -> Result<i32, WasmDynamicError> {
    let mut result: i32 = 0;
    let mut shift: u32 = 0;
    loop {
        let byte: u8 = *bytes.get(*cursor).ok_or_else(|| {
            WasmDynamicError::InvalidWasmBinary("unexpected EOF in sleb128".to_string())
        })?;
        *cursor += 1;
        if shift >= 35 {
            return Err(WasmDynamicError::InvalidWasmBinary(
                "sleb128 integer overflow".to_string(),
            ));
        }
        result |= ((byte & 0x7F) as i32).wrapping_shl(shift);
        shift += 7;
        if (byte & 0x80) == 0 {
            if shift < 32 && (byte & 0x40) != 0 {
                result |= !0_i32 << shift;
            }
            return Ok(result);
        }
    }
}

/// Reads a length-prefixed UTF-8 WebAssembly name string from `bytes`.
///
/// # Arguments
///
/// - `&[u8]` - Source WASM byte slice.
/// - `&mut usize` - Read cursor advanced past the string.
///
/// # Returns
///
/// - `Result<String, WasmDynamicError>` - Decoded UTF-8 identifier or `InvalidWasmBinary`.
fn read_wasm_name(bytes: &[u8], cursor: &mut usize) -> Result<String, WasmDynamicError> {
    let len: usize = read_uleb128(bytes, cursor)? as usize;
    let end: usize = cursor.saturating_add(len);
    let slice: &[u8] = bytes
        .get(*cursor..end)
        .ok_or_else(|| WasmDynamicError::InvalidWasmBinary("truncated wasm name".to_string()))?;
    *cursor = end;
    from_utf8(slice)
        .map(str::to_string)
        .map_err(|_| WasmDynamicError::InvalidWasmBinary("non-UTF8 wasm name".to_string()))
}

/// Parses and validates a standard WebAssembly binary (`.wasm`) into a `WasmParsedModule`.
///
/// # Arguments
///
/// - `&[u8]` - Raw `.wasm` binary bytes.
/// - `&WasmSandboxConfig` - Sandbox memory and binary limits enforced during validation.
///
/// # Returns
///
/// - `Result<WasmParsedModule, WasmDynamicError>` - Validated executable module or typed error.
pub fn parse_wasm_binary(
    bytes: &[u8],
    config: &WasmSandboxConfig,
) -> Result<WasmParsedModule, WasmDynamicError> {
    if bytes.len() > config.get_max_binary_bytes() {
        return Err(WasmDynamicError::BinaryTooLarge {
            actual: bytes.len(),
            limit: config.get_max_binary_bytes(),
        });
    }
    if bytes.len() < 8 || &bytes[0..4] != WASM_BINARY_MAGIC || &bytes[4..8] != WASM_BINARY_VERSION {
        return Err(WasmDynamicError::InvalidWasmBinary(
            "missing \\0asm v1 header".to_string(),
        ));
    }
    let mut module: WasmParsedModule = WasmParsedModule::default();
    let mut func_type_indices: Vec<u32> = Vec::new();
    let mut cursor: usize = 8;
    while cursor < bytes.len() {
        let sec_id: u8 = bytes[cursor];
        cursor += 1;
        let sec_len: usize = read_uleb128(bytes, &mut cursor)? as usize;
        let sec_end: usize = cursor.saturating_add(sec_len);
        if sec_end > bytes.len() {
            return Err(WasmDynamicError::InvalidWasmBinary(
                "truncated section payload".to_string(),
            ));
        }
        let sec_slice: &[u8] = &bytes[cursor..sec_end];
        cursor = sec_end;
        let mut sc: usize = 0;
        match sec_id {
            WASM_SEC_CUSTOM => {
                let sec_name: String = read_wasm_name(sec_slice, &mut sc)?;
                let payload: Vec<u8> = sec_slice[sc..].to_vec();
                let _: Option<Vec<u8>> = module.get_mut_custom_sections().insert(sec_name, payload);
            }
            WASM_SEC_TYPE => {
                let count: u32 = read_uleb128(sec_slice, &mut sc)?;
                for _ in 0..count {
                    let tag: u8 = *sec_slice.get(sc).ok_or_else(|| {
                        WasmDynamicError::InvalidWasmBinary("truncated type entry".to_string())
                    })?;
                    sc += 1;
                    if tag != WASM_FUNCTYPE_TAG {
                        return Err(WasmDynamicError::InvalidWasmBinary(
                            "unsupported wasm type tag".to_string(),
                        ));
                    }
                    let params: usize = read_uleb128(sec_slice, &mut sc)? as usize;
                    sc = sc.saturating_add(params);
                    let results: usize = read_uleb128(sec_slice, &mut sc)? as usize;
                    sc = sc.saturating_add(results);
                    if sc > sec_slice.len() {
                        return Err(WasmDynamicError::InvalidWasmBinary(
                            "truncated type signature".to_string(),
                        ));
                    }
                    module
                        .get_mut_types()
                        .push(WasmFuncSignature::new(params, results));
                }
            }
            WASM_SEC_IMPORT => {
                let count: u32 = read_uleb128(sec_slice, &mut sc)?;
                for _ in 0..count {
                    let mod_name: String = read_wasm_name(sec_slice, &mut sc)?;
                    let field_name: String = read_wasm_name(sec_slice, &mut sc)?;
                    let kind: u8 = *sec_slice.get(sc).ok_or_else(|| {
                        WasmDynamicError::InvalidWasmBinary("truncated import kind".to_string())
                    })?;
                    sc += 1;
                    if kind == 0x00 {
                        let type_idx: u32 = read_uleb128(sec_slice, &mut sc)?;
                        module
                            .get_mut_imports()
                            .push(WasmImportEntry::new(mod_name, field_name, type_idx));
                    } else {
                        return Err(WasmDynamicError::InvalidWasmBinary(
                            "unsupported non-function import kind".to_string(),
                        ));
                    }
                }
            }
            WASM_SEC_FUNCTION => {
                let count: u32 = read_uleb128(sec_slice, &mut sc)?;
                for _ in 0..count {
                    func_type_indices.push(read_uleb128(sec_slice, &mut sc)?);
                }
            }
            WASM_SEC_MEMORY => {
                let count: u32 = read_uleb128(sec_slice, &mut sc)?;
                if count > 0 {
                    let flags: u8 = *sec_slice.get(sc).ok_or_else(|| {
                        WasmDynamicError::InvalidWasmBinary("truncated memory limits".to_string())
                    })?;
                    sc += 1;
                    let min_pages: u32 = read_uleb128(sec_slice, &mut sc)?;
                    if (flags & 0x01) != 0 {
                        let _: u32 = read_uleb128(sec_slice, &mut sc)?;
                    }
                    let req_bytes: usize = (min_pages as usize).saturating_mul(WASM_PAGE_BYTES);
                    if req_bytes > config.get_max_memory_bytes() {
                        return Err(WasmDynamicError::MemoryLimitExceeded {
                            requested: req_bytes,
                            limit: config.get_max_memory_bytes(),
                        });
                    }
                    module.set_memory_min_pages(min_pages.max(1));
                }
            }
            WASM_SEC_EXPORT => {
                let count: u32 = read_uleb128(sec_slice, &mut sc)?;
                for _ in 0..count {
                    let exp_name: String = read_wasm_name(sec_slice, &mut sc)?;
                    let kind: u8 = *sec_slice.get(sc).ok_or_else(|| {
                        WasmDynamicError::InvalidWasmBinary("truncated export kind".to_string())
                    })?;
                    sc += 1;
                    let idx: u32 = read_uleb128(sec_slice, &mut sc)?;
                    if kind == 0x00 {
                        let _: Option<u32> = module.get_mut_exports().insert(exp_name, idx);
                    }
                }
            }
            WASM_SEC_CODE => {
                let count: usize = read_uleb128(sec_slice, &mut sc)? as usize;
                if count != func_type_indices.len() {
                    return Err(WasmDynamicError::InvalidWasmBinary(
                        "function/code count mismatch".to_string(),
                    ));
                }
                for type_idx in func_type_indices.iter().copied() {
                    let body_len: usize = read_uleb128(sec_slice, &mut sc)? as usize;
                    let body_end: usize = sc.saturating_add(body_len);
                    if body_end > sec_slice.len() {
                        return Err(WasmDynamicError::InvalidWasmBinary(
                            "truncated code body".to_string(),
                        ));
                    }
                    let local_decl_groups: u32 = read_uleb128(sec_slice, &mut sc)?;
                    let mut extra_locals: usize = 0;
                    for _ in 0..local_decl_groups {
                        let lcount: usize = read_uleb128(sec_slice, &mut sc)? as usize;
                        sc = sc.saturating_add(1);
                        extra_locals = extra_locals.saturating_add(lcount);
                    }
                    if sc > body_end {
                        return Err(WasmDynamicError::InvalidWasmBinary(
                            "invalid local declarations".to_string(),
                        ));
                    }
                    let code: Vec<u8> = sec_slice[sc..body_end].to_vec();
                    sc = body_end;
                    module.get_mut_functions().push(WasmParsedFunction::new(
                        type_idx,
                        extra_locals,
                        code,
                    ));
                }
            }
            WASM_SEC_DATA => {
                let count: u32 = read_uleb128(sec_slice, &mut sc)?;
                for _ in 0..count {
                    let flags: u32 = read_uleb128(sec_slice, &mut sc)?;
                    if flags != 0 {
                        return Err(WasmDynamicError::InvalidWasmBinary(
                            "unsupported passive data segment".to_string(),
                        ));
                    }
                    let op: u8 = *sec_slice.get(sc).ok_or_else(|| {
                        WasmDynamicError::InvalidWasmBinary("truncated data offset".to_string())
                    })?;
                    sc += 1;
                    if op != OP_I32_CONST {
                        return Err(WasmDynamicError::InvalidWasmBinary(
                            "expected i32.const in data offset".to_string(),
                        ));
                    }
                    let offset: u32 = read_sleb128(sec_slice, &mut sc)?.max(0) as u32;
                    let end_op: u8 = *sec_slice.get(sc).ok_or_else(|| {
                        WasmDynamicError::InvalidWasmBinary(
                            "missing end in data offset".to_string(),
                        )
                    })?;
                    sc += 1;
                    if end_op != OP_END {
                        return Err(WasmDynamicError::InvalidWasmBinary(
                            "malformed data offset expr".to_string(),
                        ));
                    }
                    let data_len: usize = read_uleb128(sec_slice, &mut sc)? as usize;
                    let data_end: usize = sc.saturating_add(data_len);
                    if data_end > sec_slice.len() {
                        return Err(WasmDynamicError::InvalidWasmBinary(
                            "truncated data segment bytes".to_string(),
                        ));
                    }
                    let seg_bytes: Vec<u8> = sec_slice[sc..data_end].to_vec();
                    sc = data_end;
                    module
                        .get_mut_data_segments()
                        .push(WasmDataSegment::new(offset, seg_bytes));
                }
            }
            _ => {}
        }
    }
    if module.get_memory_min_pages() == 0 {
        module.set_memory_min_pages(1);
    }
    Ok(module)
}

/// Reads a bounded UTF-8 slice from WASM linear memory, returning `MemoryOutOfBounds` on invalid ranges.
///
/// # Arguments
///
/// - `&[u8]` - Active linear memory buffer.
/// - `u32` - Start pointer offset.
/// - `u32` - Byte length to read.
///
/// # Returns
///
/// - `Result<&str, WasmDynamicError>` - Validated UTF-8 slice or memory error.
fn read_memory_str(memory: &[u8], ptr: u32, len: u32) -> Result<&str, WasmDynamicError> {
    let start: usize = ptr as usize;
    let end: usize = start.saturating_add(len as usize);
    if end > memory.len() {
        return Err(WasmDynamicError::MemoryOutOfBounds {
            addr: ptr,
            len,
            memory_size: memory.len(),
        });
    }
    from_utf8(&memory[start..end])
        .map_err(|_| WasmDynamicError::InvalidUiPayload("non-UTF8 memory buffer".to_string()))
}

/// Executes an exported WebAssembly function inside a fuel-metered, memory-bounded sandbox.
///
/// # Arguments
///
/// - `&WasmParsedModule` - Validated `.wasm` module.
/// - `&str` - Exported function name (e.g. `"euv_render"`, `"euv_on_event"`, `"euv_abi_version"`).
/// - `&[i32]` - Positional `i32` arguments passed to the export.
/// - `&WasmSandboxConfig` - Fuel and memory limits.
///
/// # Returns
///
/// - `Result<WasmExecutionOutput, WasmDynamicError>` - Execution output including emitted native UI spec or bundle.
pub fn execute_wasm_export(
    module: &WasmParsedModule,
    export_name: &str,
    args: &[i32],
    config: &WasmSandboxConfig,
) -> Result<WasmExecutionOutput, WasmDynamicError> {
    let func_idx: u32 = *module
        .get_exports()
        .get(export_name)
        .ok_or_else(|| WasmDynamicError::MissingExport(export_name.to_string()))?;
    let mem_bytes: usize =
        (module.get_memory_min_pages().max(1) as usize).saturating_mul(WASM_PAGE_BYTES);
    if mem_bytes > config.get_max_memory_bytes() {
        return Err(WasmDynamicError::MemoryLimitExceeded {
            requested: mem_bytes,
            limit: config.get_max_memory_bytes(),
        });
    }
    let mut memory: Vec<u8> = vec![0; mem_bytes];
    for seg in module.get_data_segments().iter() {
        let start: usize = seg.get_offset() as usize;
        let end: usize = start.saturating_add(seg.get_bytes().len());
        if end > memory.len() {
            return Err(WasmDynamicError::MemoryOutOfBounds {
                addr: seg.get_offset(),
                len: seg.get_bytes().len() as u32,
                memory_size: memory.len(),
            });
        }
        memory[start..end].copy_from_slice(seg.get_bytes().as_slice());
    }
    let mut out: WasmExecutionOutput = WasmExecutionOutput::default();
    let mut exec: WasmExecContext<'_> = WasmExecContext {
        memory: &mut memory,
        out: &mut out,
        fuel_used: 0,
        max_fuel: config.get_max_fuel(),
        depth: 0,
    };
    let ret: i32 = execute_wasm_func_index(module, func_idx, args, &mut exec)?;
    let fuel_used: u64 = exec.fuel_used;
    out.set_return_value(ret);
    out.set_fuel_used(fuel_used);
    Ok(out)
}

/// Evaluates an `i32` binary WebAssembly opcode if `op` matches one.
///
/// # Arguments
///
/// - `u8` - Bytecode opcode.
/// - `&mut Vec<i32>` - Active operand stack.
///
/// # Returns
///
/// - `Result<Option<i32>, WasmDynamicError>` - Computed result if `op` is a supported binop, `None` otherwise.
fn eval_i32_binop(op: u8, stack: &mut Vec<i32>) -> Result<Option<i32>, WasmDynamicError> {
    let is_binop: bool = op == OP_I32_EQ
        || op == OP_I32_NE
        || op == OP_I32_LT_S
        || op == OP_I32_GT_S
        || op == OP_I32_ADD
        || op == OP_I32_SUB
        || op == OP_I32_MUL
        || op == OP_I32_DIV_S;
    if !is_binop {
        return Ok(None);
    }
    let b: i32 = stack
        .pop()
        .ok_or_else(|| WasmDynamicError::ExecutionTrap(WASM_ERR_STACK_BINOP.to_string()))?;
    let a: i32 = stack
        .pop()
        .ok_or_else(|| WasmDynamicError::ExecutionTrap(WASM_ERR_STACK_BINOP.to_string()))?;
    let res: i32 = match op {
        OP_I32_EQ => i32::from(a == b),
        OP_I32_NE => i32::from(a != b),
        OP_I32_LT_S => i32::from(a < b),
        OP_I32_GT_S => i32::from(a > b),
        OP_I32_ADD => a.wrapping_add(b),
        OP_I32_SUB => a.wrapping_sub(b),
        OP_I32_MUL => a.wrapping_mul(b),
        OP_I32_DIV_S => {
            if b == 0 {
                return Err(WasmDynamicError::ExecutionTrap(
                    "integer divide by zero".to_string(),
                ));
            }
            a.wrapping_div(b)
        }
        _ => 0,
    };
    Ok(Some(res))
}

/// Internal recursive bytecode interpreter executing a single WASM function or host import call.
///
/// # Arguments
///
/// - `&WasmParsedModule` - Active parsed module.
/// - `u32` - Absolute function index (imports first, then defined functions).
/// - `&[i32]` - Call arguments.
/// - `&mut WasmExecContext<'_>` - Shared mutable memory, host output, fuel, and depth state.
///
/// # Returns
///
/// - `Result<i32, WasmDynamicError>` - Return value or sandbox error.
fn execute_wasm_func_index(
    module: &WasmParsedModule,
    func_idx: u32,
    args: &[i32],
    exec: &mut WasmExecContext<'_>,
) -> Result<i32, WasmDynamicError> {
    if exec.depth > 32 {
        return Err(WasmDynamicError::ExecutionTrap(
            "call stack depth overflow".to_string(),
        ));
    }
    let import_count: usize = module.get_imports().len();
    if (func_idx as usize) < import_count {
        let imp: &WasmImportEntry = &module.get_imports()[func_idx as usize];
        let ptr: u32 = args.first().copied().unwrap_or(0).max(0) as u32;
        let len: u32 = args.get(1).copied().unwrap_or(0).max(0) as u32;
        let text: &str = read_memory_str(exec.memory, ptr, len)?;
        if imp.get_name() == WASM_IMPORT_EMIT_UI {
            let spec: DynamicNodeSpec =
                from_str::<DynamicNodeSpec>(text).map_err(|err: serde_json::Error| {
                    WasmDynamicError::InvalidUiPayload(err.to_string())
                })?;
            exec.out.set_emitted_spec(Some(spec));
            return Ok(1);
        }
        if imp.get_name() == WASM_IMPORT_EMIT_BUNDLE {
            let bundle: DynamicUpdateBundle =
                from_str::<DynamicUpdateBundle>(text).map_err(|err: serde_json::Error| {
                    WasmDynamicError::InvalidUiPayload(err.to_string())
                })?;
            exec.out.set_emitted_bundle(Some(bundle));
            return Ok(1);
        }
        if imp.get_name() == WASM_IMPORT_LOG {
            exec.out.get_mut_logs().push(text.to_string());
            return Ok(1);
        }
        return Err(WasmDynamicError::ExecutionTrap(format!(
            "unknown host import {}::{}",
            imp.get_module(),
            imp.get_name()
        )));
    }
    let local_idx: usize = (func_idx as usize) - import_count;
    let func: &WasmParsedFunction = module
        .get_functions()
        .get(local_idx)
        .ok_or_else(|| WasmDynamicError::ExecutionTrap("invalid function index".to_string()))?;
    let sig: WasmFuncSignature = module
        .get_types()
        .get(func.get_type_index() as usize)
        .copied()
        .unwrap_or_default();
    let total_locals: usize = sig
        .get_param_count()
        .saturating_add(func.get_extra_locals());
    let mut locals: Vec<i32> = vec![0; total_locals.max(args.len())];
    for (idx, val) in args.iter().copied().enumerate() {
        if idx < locals.len() {
            locals[idx] = val;
        }
    }
    let code: &[u8] = func.get_body().as_slice();
    let mut stack: Vec<i32> = Vec::with_capacity(32);
    let mut labels: Vec<(u8, usize)> = Vec::with_capacity(16);
    let mut pc: usize = 0;
    while pc < code.len() {
        exec.fuel_used = exec.fuel_used.saturating_add(1);
        if exec.fuel_used > exec.max_fuel {
            return Err(WasmDynamicError::OutOfFuel {
                limit: exec.max_fuel,
            });
        }
        let op: u8 = code[pc];
        pc += 1;
        match op {
            OP_UNREACHABLE => {
                return Err(WasmDynamicError::ExecutionTrap(
                    "wasm unreachable instruction trapped".to_string(),
                ));
            }
            OP_NOP => {}
            OP_BLOCK | OP_LOOP => {
                let _: u8 = *code.get(pc).unwrap_or(&0x40);
                pc = pc.saturating_add(1);
                labels.push((op, pc));
            }
            OP_IF => {
                let _: u8 = *code.get(pc).unwrap_or(&0x40);
                pc = pc.saturating_add(1);
                let cond: i32 = stack.pop().ok_or_else(|| {
                    WasmDynamicError::ExecutionTrap("stack underflow on if".to_string())
                })?;
                if cond != 0 {
                    labels.push((OP_IF, pc));
                } else {
                    let mut nesting: usize = 0;
                    while pc < code.len() {
                        let scan: u8 = code[pc];
                        pc += 1;
                        if scan == OP_BLOCK || scan == OP_LOOP || scan == OP_IF {
                            nesting += 1;
                            pc = pc.saturating_add(1);
                        } else if scan == OP_ELSE && nesting == 0 {
                            labels.push((OP_ELSE, pc));
                            break;
                        } else if scan == OP_END {
                            if nesting == 0 {
                                break;
                            }
                            nesting -= 1;
                        } else if scan == OP_I32_CONST {
                            let _: i32 = read_sleb128(code, &mut pc)?;
                        } else if scan == OP_LOCAL_GET
                            || scan == OP_LOCAL_SET
                            || scan == OP_CALL
                            || scan == OP_BR
                            || scan == OP_BR_IF
                        {
                            let _: u32 = read_uleb128(code, &mut pc)?;
                        } else if scan == OP_I32_LOAD || scan == OP_I32_STORE {
                            let _: u32 = read_uleb128(code, &mut pc)?;
                            let _: u32 = read_uleb128(code, &mut pc)?;
                        }
                    }
                }
            }
            OP_ELSE => {
                let _: Option<(u8, usize)> = labels.pop();
                let mut nesting: usize = 0;
                while pc < code.len() {
                    let scan: u8 = code[pc];
                    pc += 1;
                    if scan == OP_BLOCK || scan == OP_LOOP || scan == OP_IF {
                        nesting += 1;
                        pc = pc.saturating_add(1);
                    } else if scan == OP_END {
                        if nesting == 0 {
                            break;
                        }
                        nesting -= 1;
                    } else if scan == OP_I32_CONST {
                        let _: i32 = read_sleb128(code, &mut pc)?;
                    } else if scan == OP_LOCAL_GET
                        || scan == OP_LOCAL_SET
                        || scan == OP_CALL
                        || scan == OP_BR
                        || scan == OP_BR_IF
                    {
                        let _: u32 = read_uleb128(code, &mut pc)?;
                    } else if scan == OP_I32_LOAD || scan == OP_I32_STORE {
                        let _: u32 = read_uleb128(code, &mut pc)?;
                        let _: u32 = read_uleb128(code, &mut pc)?;
                    }
                }
            }
            OP_END => {
                if labels.pop().is_none() {
                    break;
                }
            }
            OP_BR | OP_BR_IF => {
                let target_depth: usize = read_uleb128(code, &mut pc)? as usize;
                let take_branch: bool = if op == OP_BR_IF {
                    stack.pop().unwrap_or(0) != 0
                } else {
                    true
                };
                if take_branch {
                    if target_depth >= labels.len() {
                        break;
                    }
                    let keep_len: usize = labels.len() - target_depth;
                    let (lbl_op, lbl_pc): (u8, usize) = labels[keep_len - 1];
                    if lbl_op == OP_LOOP {
                        labels.truncate(keep_len);
                        pc = lbl_pc;
                    } else {
                        labels.truncate(keep_len - 1);
                        let mut nesting: usize = 0;
                        while pc < code.len() {
                            let scan: u8 = code[pc];
                            pc += 1;
                            if scan == OP_BLOCK || scan == OP_LOOP || scan == OP_IF {
                                nesting += 1;
                                pc = pc.saturating_add(1);
                            } else if scan == OP_END {
                                if nesting == 0 {
                                    break;
                                }
                                nesting -= 1;
                            }
                        }
                    }
                }
            }
            OP_RETURN => break,
            OP_CALL => {
                let callee: u32 = read_uleb128(code, &mut pc)?;
                let callee_type: u32 = if (callee as usize) < import_count {
                    module.get_imports()[callee as usize].get_type_index()
                } else {
                    module
                        .get_functions()
                        .get((callee as usize) - import_count)
                        .map(WasmParsedFunction::get_type_index)
                        .unwrap_or(0)
                };
                let csig: WasmFuncSignature = module
                    .get_types()
                    .get(callee_type as usize)
                    .copied()
                    .unwrap_or_default();
                let mut call_args: Vec<i32> = vec![0; csig.get_param_count()];
                for idx in (0..csig.get_param_count()).rev() {
                    call_args[idx] = stack.pop().ok_or_else(|| {
                        WasmDynamicError::ExecutionTrap("stack underflow on call".to_string())
                    })?;
                }
                exec.depth = exec.depth.saturating_add(1);
                let call_res: Result<i32, WasmDynamicError> =
                    execute_wasm_func_index(module, callee, call_args.as_slice(), exec);
                exec.depth = exec.depth.saturating_sub(1);
                let call_ret: i32 = call_res?;
                if csig.get_result_count() > 0 {
                    stack.push(call_ret);
                }
            }
            OP_DROP => {
                let _: Option<i32> = stack.pop();
            }
            OP_LOCAL_GET => {
                let idx: usize = read_uleb128(code, &mut pc)? as usize;
                let val: i32 = *locals.get(idx).unwrap_or(&0);
                stack.push(val);
            }
            OP_LOCAL_SET => {
                let idx: usize = read_uleb128(code, &mut pc)? as usize;
                let val: i32 = stack.pop().ok_or_else(|| {
                    WasmDynamicError::ExecutionTrap("stack underflow on local.set".to_string())
                })?;
                if idx < locals.len() {
                    locals[idx] = val;
                }
            }
            OP_I32_LOAD => {
                let _: u32 = read_uleb128(code, &mut pc)?;
                let offset: u32 = read_uleb128(code, &mut pc)?;
                let base: u32 = stack.pop().unwrap_or(0).max(0) as u32;
                let addr: usize = (base.saturating_add(offset)) as usize;
                if addr.saturating_add(4) > exec.memory.len() {
                    return Err(WasmDynamicError::MemoryOutOfBounds {
                        addr: addr as u32,
                        len: 4,
                        memory_size: exec.memory.len(),
                    });
                }
                let val: i32 = i32::from_le_bytes([
                    exec.memory[addr],
                    exec.memory[addr + 1],
                    exec.memory[addr + 2],
                    exec.memory[addr + 3],
                ]);
                stack.push(val);
            }
            OP_I32_STORE => {
                let _: u32 = read_uleb128(code, &mut pc)?;
                let offset: u32 = read_uleb128(code, &mut pc)?;
                let val: i32 = stack.pop().unwrap_or(0);
                let base: u32 = stack.pop().unwrap_or(0).max(0) as u32;
                let addr: usize = (base.saturating_add(offset)) as usize;
                if addr.saturating_add(4) > exec.memory.len() {
                    return Err(WasmDynamicError::MemoryOutOfBounds {
                        addr: addr as u32,
                        len: 4,
                        memory_size: exec.memory.len(),
                    });
                }
                exec.memory[addr..addr + 4].copy_from_slice(&val.to_le_bytes());
            }
            OP_I32_CONST => {
                let val: i32 = read_sleb128(code, &mut pc)?;
                stack.push(val);
            }
            OP_I32_EQZ => {
                let a: i32 = stack.pop().unwrap_or(0);
                stack.push(if a == 0 { 1 } else { 0 });
            }
            other => {
                if let Some(res) = eval_i32_binop(other, &mut stack)? {
                    stack.push(res);
                } else {
                    return Err(WasmDynamicError::ExecutionTrap(format!(
                        "unsupported wasm opcode 0x{other:02x}"
                    )));
                }
            }
        }
    }
    Ok(stack.pop().unwrap_or(0))
}

/// Builds a genuine signed partial-slot `.wasm` binary rendering a rich native `DynamicNodeSpec` with live interactive state.
///
/// # Arguments
///
/// - `&str` - Display title rendered by the WASM widget.
/// - `u64` - Release version number.
/// - `u32` - Accent RGB hex color.
///
/// # Returns
///
/// - `Vec<u8>` - Spec-compliant executable `.wasm` binary bytes.
#[must_use]
pub fn build_partial_widget_wasm(title: &str, version: u64, accent_hex: u32) -> Vec<u8> {
    let mut badge: DynamicNodeSpec = DynamicNodeSpec::badge(format!("WASM v{version} · NATIVE"));
    badge
        .get_mut_style()
        .set_background_hex(Some(accent_hex))
        .set_color_hex(Some(0xFFFFFF));
    let mut heading: DynamicNodeSpec = DynamicNodeSpec::text(title.to_string());
    heading
        .get_mut_style()
        .set_bold(true)
        .set_font_size(Some(14.0));
    let sub: DynamicNodeSpec = DynamicNodeSpec::text(format!(
        "Downloaded .wasm binary executed in sandboxed VM and rendered directly to native SceneTree (rev v{version}, state=0)."
    ));
    let mut btn_interact: DynamicNodeSpec = DynamicNodeSpec::button(
        "Invoke WASM euv_on_event (+1)".to_string(),
        "wasm_interact".to_string(),
    );
    btn_interact.set_payload(format!("wasm-partial-v{version}"));
    let mut root_even: DynamicNodeSpec = DynamicNodeSpec::column()
        .child(
            DynamicNodeSpec::row()
                .child(badge.clone())
                .child(heading.clone()),
        )
        .child(sub)
        .child(btn_interact.clone());
    root_even
        .get_mut_style()
        .set_full_width(true)
        .set_padding(Some(14.0))
        .set_gap(Some(10.0))
        .set_radius(Some(10.0))
        .set_border_width(Some(1.0))
        .set_border_hex(Some(accent_hex));
    let sub_alt: DynamicNodeSpec = DynamicNodeSpec::text(format!(
        "Interactive WASM state toggled inside sandbox (rev v{version}, active pulse state=1) — zero host rebuild!"
    ));
    let mut root_odd: DynamicNodeSpec = DynamicNodeSpec::column()
        .child(DynamicNodeSpec::row().child(badge).child(heading))
        .child(sub_alt)
        .child(btn_interact);
    root_odd
        .get_mut_style()
        .set_full_width(true)
        .set_padding(Some(14.0))
        .set_gap(Some(10.0))
        .set_radius(Some(10.0))
        .set_background_hex(Some(0xECFDF5))
        .set_border_width(Some(1.0))
        .set_border_hex(Some(0x10B981));
    WasmModuleBuilder::partial_spec(&root_even)
        .with_alt_spec(&root_odd)
        .with_log(format!("partial wasm v{version} rendered natively"))
        .build()
}

/// Builds a genuine global full-app `.wasm` binary emitting a complete `DynamicUpdateBundle` that atomically takes over the root shell.
///
/// # Arguments
///
/// - `&str` - Global release title.
/// - `u64` - Monotonic global release version.
///
/// # Returns
///
/// - `Vec<u8>` - Spec-compliant executable global `.wasm` binary bytes.
#[must_use]
pub fn build_global_app_wasm(title: &str, version: u64) -> Vec<u8> {
    let mut badge: DynamicNodeSpec = DynamicNodeSpec::badge(format!("GLOBAL WASM APP v{version}"));
    badge
        .get_mut_style()
        .set_background_hex(Some(0x0F766E))
        .set_color_hex(Some(0xFFFFFF));
    let mut hero_title: DynamicNodeSpec = DynamicNodeSpec::text(title.to_string());
    hero_title
        .get_mut_style()
        .set_bold(true)
        .set_font_size(Some(22.0));
    let hero_sub: DynamicNodeSpec = DynamicNodeSpec::text(format!(
        "Entire native UI shell hot-replaced atomically from downloaded global .wasm release v{version}. Zero WebView, instant LKG rollback ready."
    ));
    let mut kpi_grid: DynamicNodeSpec =
        DynamicNodeSpec::grid(3)
            .child(DynamicNodeSpec::card("Execution Mode".to_string()).child(
                DynamicNodeSpec::text("Sandboxed WASM -> Native SceneTree".to_string()),
            ))
            .child(
                DynamicNodeSpec::card("Integrity Gate".to_string())
                    .child(DynamicNodeSpec::text(format!("Verified Signed v{version}"))),
            )
            .child(
                DynamicNodeSpec::card("Safety Guard".to_string()).child(DynamicNodeSpec::text(
                    "Fuel + Memory + Atomic LKG Rollback".to_string(),
                )),
            );
    kpi_grid
        .get_mut_style()
        .set_full_width(true)
        .set_gap(Some(12.0));
    let mut btn_action: DynamicNodeSpec = DynamicNodeSpec::button(
        "Trigger Global WASM Action".to_string(),
        "global_wasm_action".to_string(),
    );
    btn_action.set_payload(format!("global-v{version}"));
    let mut btn_restore: DynamicNodeSpec = DynamicNodeSpec::button(
        "Restore Built-in Native Shell".to_string(),
        "global_wasm_restore".to_string(),
    );
    btn_restore.set_payload("restore-native".to_string());
    let mut action_row: DynamicNodeSpec =
        DynamicNodeSpec::row().child(btn_action).child(btn_restore);
    action_row.get_mut_style().set_gap(Some(10.0));
    let mut root_spec: DynamicNodeSpec = DynamicNodeSpec::column()
        .child(badge)
        .child(hero_title)
        .child(hero_sub)
        .child(kpi_grid)
        .child(action_row);
    root_spec
        .get_mut_style()
        .set_full_width(true)
        .set_padding(Some(20.0))
        .set_gap(Some(14.0))
        .set_radius(Some(12.0))
        .set_border_width(Some(1.0))
        .set_border_hex(Some(0x14B8A6));
    let mut global_comp: DynamicComponentUpdate = DynamicComponentUpdate::new(
        WASM_GLOBAL_ROOT_SLOT.to_string(),
        version,
        root_spec.clone(),
    );
    global_comp.sign();
    let mut global_page: DynamicPageUpdate = DynamicPageUpdate::new(
        "/wasm-global-live".to_string(),
        format!("Global WASM v{version}"),
        version,
        root_spec,
    );
    global_page.sign();
    let mut bundle: DynamicUpdateBundle =
        DynamicUpdateBundle::new(format!("global-wasm-v{version}"), version);
    bundle.get_mut_components().push(global_comp);
    bundle.get_mut_pages().push(global_page);
    WasmModuleBuilder::global_bundle(&bundle)
        .with_log(format!("global wasm bundle v{version} mounted natively"))
        .build()
}

/// Builds a faulty `.wasm` binary exercising a specific security, integrity, ABI, or sandbox trap case.
///
/// # Arguments
///
/// - `WasmFaultKind` - Specific failure case to compile into the test binary.
///
/// # Returns
///
/// - `Vec<u8>` - Crafted `.wasm` binary bytes triggering `fault`.
#[must_use]
pub fn build_faulty_wasm(fault: WasmFaultKind) -> Vec<u8> {
    match fault {
        WasmFaultKind::CorruptedMagic => b"NOT_A_VALID_WASM_BINARY".to_vec(),
        WasmFaultKind::AbiMismatch => {
            WasmModuleBuilder::partial_spec(&DynamicNodeSpec::text("Incompatible ABI".to_string()))
                .with_abi_version(99)
                .build()
        }
        WasmFaultKind::MissingExport => {
            WasmModuleBuilder::partial_spec(&DynamicNodeSpec::text("Missing Export".to_string()))
                .without_render_export()
                .build()
        }
        WasmFaultKind::InfiniteLoopOutOfFuel => {
            WasmModuleBuilder::partial_spec(&DynamicNodeSpec::text("Infinite Loop".to_string()))
                .with_fault(WasmFaultKind::InfiniteLoopOutOfFuel)
                .build()
        }
        WasmFaultKind::ExecutionTrap => {
            WasmModuleBuilder::partial_spec(&DynamicNodeSpec::text("Trap".to_string()))
                .with_fault(WasmFaultKind::ExecutionTrap)
                .build()
        }
        WasmFaultKind::OversizedBinary => {
            let mut big: Vec<u8> = build_partial_widget_wasm("Oversized", 1, 0xDC2626);
            big.resize(WASM_DEFAULT_MAX_BINARY_BYTES + 64, 0);
            big
        }
        _ => build_partial_widget_wasm("Fault Fixture", 1, 0x2563EB),
    }
}
