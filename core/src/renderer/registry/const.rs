/// The DOM attribute name used to store the unique euv identifier on an element.
///
/// This attribute is set on every element that registers an event listener
/// so the framework can look up the element's identity across re-renders.
pub(crate) const DATA_EUV_ID: &str = "data-euv-id";

/// The maximum number of dispatch iterations allowed in `dispatch_updates`.
///
/// Prevents infinite loops when callbacks continuously re-queue themselves.
/// After this many passes, the dispatch loop breaks even if dirty entries remain.
pub(crate) const MAX_ITERATIONS: usize = 3;

/// Event names that do not bubble up to `window`.
///
/// These events must be attached directly on the target element
/// instead of using global event delegation on `window`.
///
/// Sources:
/// - W3C DOM Level 3 Events: `abort`, `blur`, `error`, `focus`, `load`, `resize`, `unload`
/// - Mouse: `mouseenter`, `mouseleave`
/// - Media (all non-bubbling): `loadstart`, `progress`, `loadend`, `emptied`, `stalled`,
///   `suspend`, `canplay`, `canplaythrough`, `loadedmetadata`, `waiting`, `playing`,
///   `pause`, `seeking`, `seeked`, `timeupdate`, `volumechange`, `durationchange`,
///   `ratechange`, `ended`
/// - UI: `beforeunload`, `scroll`, `resize`, `select`
/// - CSS: `transitionend`, `animationend`, `animationiteration`, `animationstart`
pub(crate) const NON_BUBBLING_EVENTS: [&str; 35] = [
    "abort",
    "animationend",
    "animationiteration",
    "animationstart",
    "beforeunload",
    "blur",
    "canplay",
    "canplaythrough",
    "durationchange",
    "emptied",
    "ended",
    "error",
    "focus",
    "mouseleave",
    "mouseenter",
    "load",
    "loadedmetadata",
    "loadend",
    "loadstart",
    "pause",
    "playing",
    "progress",
    "ratechange",
    "resize",
    "scroll",
    "seeked",
    "seeking",
    "select",
    "stalled",
    "suspend",
    "timeupdate",
    "transitionend",
    "unload",
    "volumechange",
    "waiting",
];
/// Event names that fire at very high frequency (mousemove, mousewheel,
/// pointermove, touchmove, wheel — see the array below). For these events
/// the
/// `dispatch_delegated_event` ancestor walk is capped at
/// `MAX_ANCESTOR_DEPTH_FOR_HIGH_FREQ` levels instead of walking all the
/// way to `<html>`, because:
///
/// - mousemove/touchmove handlers almost always live on the same element
///   they fire on, or 1-2 ancestors above (e.g. a draggable card inside
///   a scrollable container);
/// - paying the cost of `get_attribute` + `parse::<usize>` + HashMap
///   lookup on every intermediate element is pure overhead when the
///   handler lives near the target.
///
/// Events NOT on this list (`click`, `input`, `keydown`, etc.) keep the
/// original full-depth behaviour because their handler lookup is less
/// locality-preserving: a `click` on a deeply nested icon needs to
/// resolve to a button defined many ancestors up.
///
/// When adding a new entry to this list, document WHY in the comment
/// above the array, and consider whether the framework's other
/// event-delegation paths (e.g. portal marker resolution) need a
/// matching special-case.
pub(crate) const HIGH_FREQUENCY_EVENTS: [&str; 5] = [
    "mousemove",
    "mousewheel",
    "pointermove",
    "touchmove",
    "wheel",
];

/// Upper bound on the ancestor walk depth for events named in
/// `HIGH_FREQUENCY_EVENTS`. The walk starts at `event.target()` (depth 0)
/// and proceeds through at most this many `parent_element` hops before
/// giving up. The cap exists to bound the per-event CPU cost of
/// `get_attribute` + `parse::<usize>` + HashMap lookup at 4 hops, which
/// is empirically enough to reach a typical scroll/drag container while
/// keeping the worst case proportional to constant time rather than the
/// full DOM depth.
///
/// See the doc on `HIGH_FREQUENCY_EVENTS` for the rationale.
pub(crate) const MAX_ANCESTOR_DEPTH_FOR_HIGH_FREQ: usize = 4;

/// The global property name under which the event id-chain walker is
/// installed by euv's one-time startup injection.
pub(crate) const EVENT_ID_CHAIN_GLOBAL_NAME: &str = "__euvEventIdChain";

thread_local! {
    /// Cached handle to the injected global walker, resolved on first use.
    ///
    /// The outer `Option` records "we already looked"; the inner `None`
    /// records "looked, and the global is absent". That way a host which
    /// strips the helper costs one lookup per page instead of one per
    /// event, and the fallback path can be taken without a second probe.
    pub static EVENT_ID_CHAIN_FN: std::cell::RefCell<
        Option<Option<web_sys::js_sys::Function>>,
    > = const { std::cell::RefCell::new(None) };
}

/// The JS global object name resolved when no direct handle is available.
pub(crate) const GLOBAL_THIS_NAME: &str = "globalThis";

/// The DOM `Event` property holding the node the event was dispatched on.
pub(crate) const EVENT_TARGET_PROP: &str = "target";
