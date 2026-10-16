/// The document event the scroll spy polls on.
///
/// Deliberately a `setInterval` rather than a `scroll` listener or
/// `requestAnimationFrame`: the app shell locks the viewport and scrolls
/// an inner container, and a delegated `scroll` listener on `document`
/// can stay silent while that container scrolls, while rAF is paused
/// outright in a hidden tab. A short interval reads one property per
/// tick and early-outs when nothing moved, so it costs effectively
/// nothing and behaves the same in a background tab.
pub(crate) const TOC_SPY_INTERVAL_MILLIS: i32 = 100;

/// How far below the top of the scroll container a heading has to sit
/// before it counts as the active section, in CSS pixels.
///
/// Matches the visual reading line: a heading a little scrolled past the
/// top edge has already been read, so the next one lights up. A heading
/// still hidden above the edge does not.
pub(crate) const TOC_SPY_READING_LINE_PX: f64 = 100.0;

/// Tolerance in CSS pixels for treating the container as scrolled to its
/// end, where the last heading should win rather than the first one that
/// is technically still on screen.
pub(crate) const TOC_SPY_END_TOLERANCE_PX: f64 = 4.0;

/// Selector of the scrollable prose column the spy measures against.
pub(crate) const TOC_SPY_CONTAINER_SELECTOR: &str = ".c_euv_doc_content";

/// Selector matching every TOC anchor, flat or nested.
pub(crate) const TOC_SPY_LINK_SELECTOR: &str = ".c_euv_doc_toc a, .c_euv_toc a";

/// The `href` attribute read from each TOC anchor.
pub(crate) const HREF_ATTR: &str = "href";
