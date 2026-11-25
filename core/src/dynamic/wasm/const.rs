/// Standard 4-byte WebAssembly binary magic header (`\0asm`).
pub const WASM_BINARY_MAGIC: &[u8] = &[0x00, 0x61, 0x73, 0x6D];

/// Standard 4-byte WebAssembly MVP binary version (`1`).
pub const WASM_BINARY_VERSION: &[u8] = &[0x01, 0x00, 0x00, 0x00];

/// Current euv dynamic WASM host ABI contract version.
pub const WASM_ABI_VERSION: u32 = 1;

/// Default maximum downloadable `.wasm` binary size (`8 MiB`).
pub const WASM_DEFAULT_MAX_BINARY_BYTES: usize = 8 * 1024 * 1024;

/// Default maximum WASM linear memory ceiling (`4 MiB`).
pub const WASM_DEFAULT_MAX_MEMORY_BYTES: usize = 4 * 1024 * 1024;

/// Default per-invocation instruction fuel budget preventing infinite loops.
pub const WASM_DEFAULT_MAX_FUEL: u64 = 100_000;

/// Default network download timeout in milliseconds.
pub const WASM_DEFAULT_TIMEOUT_MS: u64 = 15_000;

/// Default transient download retry attempts before failing or serving cache.
pub const WASM_DEFAULT_MAX_RETRIES: u32 = 2;

/// WASM linear memory page size in bytes (`64 KiB`).
pub const WASM_PAGE_BYTES: usize = 65_536;

/// Exported render entrypoint function name inside dynamic `.wasm` modules.
pub const WASM_EXPORT_RENDER: &str = "euv_render";

/// Exported interactive event handler function name inside dynamic `.wasm` modules.
pub const WASM_EXPORT_ON_EVENT: &str = "euv_on_event";

/// Exported ABI version query function name inside dynamic `.wasm` modules.
pub const WASM_EXPORT_ABI: &str = "euv_abi_version";

/// Host import function emitting a partial `DynamicNodeSpec` JSON buffer from WASM memory.
pub const WASM_IMPORT_EMIT_UI: &str = "euv_emit_ui";

/// Host import function emitting a global `DynamicUpdateBundle` JSON buffer from WASM memory.
pub const WASM_IMPORT_EMIT_BUNDLE: &str = "euv_emit_bundle";

/// Host import function recording a diagnostic log line from WASM memory.
pub const WASM_IMPORT_LOG: &str = "euv_log";

/// Host import module namespace expected by euv dynamic `.wasm` modules.
pub const WASM_IMPORT_MODULE_ENV: &str = "env";

/// Reserved global dynamic WASM root slot identifier.
pub const WASM_GLOBAL_ROOT_SLOT: &str = "__euv_global_wasm_root__";

/// Custom section name carrying optional signed metadata.
pub const WASM_CUSTOM_META_SECTION: &str = "euv.wasm.meta";

/// WebAssembly custom section ID (`0`).
pub(crate) const WASM_SEC_CUSTOM: u8 = 0;

/// WebAssembly type section ID (`1`).
pub(crate) const WASM_SEC_TYPE: u8 = 1;

/// WebAssembly import section ID (`2`).
pub(crate) const WASM_SEC_IMPORT: u8 = 2;

/// WebAssembly function section ID (`3`).
pub(crate) const WASM_SEC_FUNCTION: u8 = 3;

/// WebAssembly memory section ID (`5`).
pub(crate) const WASM_SEC_MEMORY: u8 = 5;

/// WebAssembly export section ID (`7`).
pub(crate) const WASM_SEC_EXPORT: u8 = 7;

/// WebAssembly code section ID (`10`).
pub(crate) const WASM_SEC_CODE: u8 = 10;

/// WebAssembly data section ID (`11`).
pub(crate) const WASM_SEC_DATA: u8 = 11;

/// WebAssembly opcode: `unreachable`.
pub(crate) const OP_UNREACHABLE: u8 = 0x00;

/// WebAssembly opcode: `nop`.
pub(crate) const OP_NOP: u8 = 0x01;

/// WebAssembly opcode: `block`.
pub(crate) const OP_BLOCK: u8 = 0x02;

/// WebAssembly opcode: `loop`.
pub(crate) const OP_LOOP: u8 = 0x03;

/// WebAssembly opcode: `if`.
pub(crate) const OP_IF: u8 = 0x04;

/// WebAssembly opcode: `else`.
pub(crate) const OP_ELSE: u8 = 0x05;

/// WebAssembly opcode: `end`.
pub(crate) const OP_END: u8 = 0x0B;

/// WebAssembly opcode: `br`.
pub(crate) const OP_BR: u8 = 0x0C;

/// WebAssembly opcode: `br_if`.
pub(crate) const OP_BR_IF: u8 = 0x0D;

/// WebAssembly opcode: `return`.
pub(crate) const OP_RETURN: u8 = 0x0F;

/// WebAssembly opcode: `call`.
pub(crate) const OP_CALL: u8 = 0x10;

/// WebAssembly opcode: `drop`.
pub(crate) const OP_DROP: u8 = 0x1A;

/// WebAssembly opcode: `local.get`.
pub(crate) const OP_LOCAL_GET: u8 = 0x20;

/// WebAssembly opcode: `local.set`.
pub(crate) const OP_LOCAL_SET: u8 = 0x21;

/// WebAssembly opcode: `i32.load`.
pub(crate) const OP_I32_LOAD: u8 = 0x28;

/// WebAssembly opcode: `i32.store`.
pub(crate) const OP_I32_STORE: u8 = 0x36;

/// WebAssembly opcode: `i32.const`.
pub(crate) const OP_I32_CONST: u8 = 0x41;

/// WebAssembly opcode: `i32.eqz`.
pub(crate) const OP_I32_EQZ: u8 = 0x45;

/// WebAssembly opcode: `i32.eq`.
pub(crate) const OP_I32_EQ: u8 = 0x46;

/// WebAssembly opcode: `i32.ne`.
pub(crate) const OP_I32_NE: u8 = 0x47;

/// WebAssembly opcode: `i32.lt_s`.
pub(crate) const OP_I32_LT_S: u8 = 0x48;

/// WebAssembly opcode: `i32.gt_s`.
pub(crate) const OP_I32_GT_S: u8 = 0x4A;

/// WebAssembly opcode: `i32.add`.
pub(crate) const OP_I32_ADD: u8 = 0x6A;

/// WebAssembly opcode: `i32.sub`.
pub(crate) const OP_I32_SUB: u8 = 0x6B;

/// WebAssembly opcode: `i32.mul`.
pub(crate) const OP_I32_MUL: u8 = 0x6C;

/// WebAssembly opcode: `i32.div_s`.
pub(crate) const OP_I32_DIV_S: u8 = 0x6D;

/// WebAssembly `i32` value type tag (`0x7F`).
pub(crate) const WASM_VALTYPE_I32: u8 = 0x7F;

/// WebAssembly function type tag (`0x60`).
pub(crate) const WASM_FUNCTYPE_TAG: u8 = 0x60;

/// Signing salt prefix used by `compute_wasm_signature`.
pub(crate) const WASM_SIGNATURE_PREFIX: &str = "euv-sig-v1:";

/// Supported `http://` URL scheme prefix for native WASM downloads.
pub(crate) const WASM_SCHEME_HTTP: &str = "http://";

/// Supported `file://` URL scheme prefix for local WASM bundles.
pub(crate) const WASM_SCHEME_FILE: &str = "file://";

/// Diagnostic message when `WasmDynamicManager` encounters a conflicting interior borrow.
pub(crate) const WASM_ERR_MANAGER_BORROW: &str = "manager borrow";

/// Diagnostic message when a WASM binary opcode encounters a value stack underflow.
pub(crate) const WASM_ERR_STACK_BINOP: &str = "stack underflow on binop";
