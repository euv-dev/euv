/// Stable prefix shared by every injected DOM-op name.
///
/// The prefix is fixed on purpose: it is what makes an injected name
/// recognisable to euv itself, and to a human reading `globalThis` in a
/// debugger. What follows the prefix is built once per page load from a
/// microsecond clock and an encoded suffix, so the *full* name is not a
/// stable, pre-knowable target. See [`DomOpNames`] for the shape and
/// [`dom_op_names`] for the one-time construction.
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

/// The four names euv used to publish the batched helpers under, before
/// [`DomOpNames`] made them per-load.
///
/// Kept as a named list because the point of the change is that these are
/// *no longer* what the helpers are called: anything that hard-codes one of
/// them finds nothing to hijack. Nothing reads this at runtime except a test
/// that asserts the retired names are gone.
#[cfg(test)]
pub(crate) const JS_DOM_OP_RETIRED_NAMES: [&str; 4] = [
    "__euv_dom_ops__",
    "__euv_dom_op_set_attrs",
    "__euv_dom_op_remove_attrs",
    "__euv_dom_op_child_ops",
];

/// Name of the JS global holding the batched helper table.
pub(crate) const JS_GLOBAL_THIS: &str = "globalThis";

/// JS expression returning the sub-millisecond fraction of the high-resolution
/// clock, used to add resolution [`JS_GLOBAL_THIS`]'s `Date.now()` lacks.
pub(crate) const JS_PERFORMANCE_NOW_FRACTION: &str = "performance.now() % 1";

/// A name set the tests pin, so the JS side can be asserted without depending
/// on the clock.
///
/// Only the suffixes matter; the roles are what a test distinguishes.
#[cfg(test)]
pub(crate) const JS_DOM_OP_TEST_NAMES: [&str; 4] = [
    "__euv_pinned_table",
    "__euv_pinned_set",
    "__euv_pinned_remove",
    "__euv_pinned_child",
];

/// A name the [`DomOpNames::set`] test asserts never wins, because only the
/// first `set` may claim the names.
#[cfg(test)]
pub(crate) const JS_DOM_OP_TEST_LOSER: &str = "__euv_should_not_win";
