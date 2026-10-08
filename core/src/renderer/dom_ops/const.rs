/// Stable prefix shared by every injected DOM-op name.
///
/// The prefix is fixed on purpose: it is what makes an injected name
/// recognisable to euv itself, and to a human reading `globalThis` in a
/// debugger. What follows the prefix is built once per page load from a
/// microsecond clock and an encoded suffix, so the *full* name is not a
/// stable, pre-knowable target. See [`DomOpNames`](crate::DomOpNames) for the shape and
/// `dom_op_names` for the one-time construction.
pub(crate) const JS_DOM_OP_NAME_PREFIX: &str = "__euv_";

/// Character set handed to `bin-encode-decode` when encoding the suffix.
///
/// The crate requires exactly 64 *distinct* characters, and only 62
/// identifier-safe alphanumerics exist, so the last two slots go to `_` and
/// `$`. Both are legal in a JS identifier and both are legal in a property
/// key, which is what matters here: `-` would have been the obvious
/// Base64-alphabet filler and it is *not* a legal identifier character, so a
/// name containing one only works through bracket access.
///
/// The suffix never leads a name — it always follows the `__euv_` prefix —
/// so the rule that `_` and `$` may not appear first does not apply.
///
/// Fixed across builds so a cached wasm bundle always encodes with the same
/// alphabet.
pub(crate) const JS_DOM_OP_NAME_CHARSET: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_$";

/// Odd multiplier applied to the microsecond clock before encoding.
///
/// A raw timestamp is only as unpredictable as the page load time, which an
/// attacker can bracket. Mixing with an odd constant spreads the low bits so
/// successive page loads do not produce visibly related suffixes, and the
/// clock is read in microseconds rather than milliseconds so two loads in the
/// same millisecond still differ.
pub(crate) const JS_DOM_OP_NAME_MIX: u64 = 0x9E37_79B9_7F4A_7C15;

/// Fallback suffix used when the clock is unavailable.
///
/// Off-wasm there is no `Date.now()`, and a clock that fails must not stop
/// the table from being installed. The value is a constant, so a host-side
/// build is *not* protected against a pre-known name — wasm is the target
/// and the clock is always present there.
pub(crate) const JS_DOM_OP_NAME_FALLBACK_SUFFIX: &str = "0000000000000000";

/// Name of the JS global holding the batched helper table.
pub(crate) const JS_GLOBAL_THIS: &str = "globalThis";

/// JS expression returning the sub-millisecond fraction of the high-resolution
/// clock, used to add resolution [`JS_GLOBAL_THIS`]'s `Date.now()` lacks.
///
/// Only the wasm build evaluates JS, so this constant is unreachable on the
/// host and is gated with it; ungated it is a `dead_code` warning waiting to
/// happen on every `cargo clippy` that is not targeting wasm32.
#[cfg(target_arch = "wasm32")]
pub(crate) const JS_PERFORMANCE_NOW_FRACTION: &str = "performance.now() % 1";

/// The byte every printable character is offset from when a token is masked
/// for the DOM-operation lookup table.
pub(crate) const DOM_OPS_MASK_DIGIT: u8 = b'0';

/// The byte the mask starts from, so a printable run never contains the
/// separator the table keys on.
pub(crate) const DOM_OPS_MASK_FILL: u8 = b'!';
