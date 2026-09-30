use super::*;

/// Global set of CSS class names that have already been injected into the DOM.
///
/// Used by `Css::inject_style` to skip redundant injections when the same
/// class is encountered multiple times (e.g., 100 `<li>` elements sharing
/// `c_list_item`). Without this dedup, each occurrence would append a
/// duplicate text node to the `<style>` element, causing both memory
/// bloat and O(N) style-recalc cost in the browser.
///
/// The set is behind an `RwLock` rather than the previous
/// `static mut` + `UnsafeCell` + `unsafe impl Sync` triple. This global is
/// shared by every thread in the process, and the old form was genuine
/// undefined behaviour under concurrent access: parallel test runs mutated
/// the `HashSet` from several threads at once, corrupting it and killing
/// the test binary with SIGSEGV / SIGTRAP. `RwLock` keeps the same
/// single-init semantics with no unsafe code at all.
pub(crate) static INJECTED_CLASSES: LazyLock<RwLock<HashSet<String>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));
