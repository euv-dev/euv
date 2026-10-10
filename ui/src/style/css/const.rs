/// Raw stylesheet for rendered markdown bodies ([`crate::euv_markdown`]).
///
/// `class!` cannot express descendant selectors, so markdown typography is
/// injected once at startup through `Css::inject_css`. All colors reference
/// the theme CSS variables, so light/dark switching works unchanged.
pub(crate) const EUV_MD_CSS: &str = r#"
.md-body {
    line-height: 1.7;
    font-size: var(--font-base);
    word-wrap: break-word;
}
.md-body h1, .md-body h2, .md-body h3, .md-body h4, .md-body h5, .md-body h6 {
    position: relative;
    font-weight: 700;
    letter-spacing: -0.01em;
    /* Vertical rhythm is token-driven so article copy sits on the same
       scale as every other surface in the framework (`space-sm` between
       text, `gap-component` before a block). em-based margins drift as
       heading font sizes change, which is what made the docs article
       spacing read wider than the rest of the UI. */
    margin-top: var(--gap-component);
    margin-bottom: var(--space-sm);
    scroll-margin-top: 72px;
    line-height: 1.3;
}
.md-body h1 {
    font-size: var(--font-3xl);
    margin-top: 0;
    padding-bottom: 0.4em;
    border-bottom: 1px solid var(--border);
}
.md-body h2 {
    font-size: var(--font-2xl);
    padding-bottom: 0.3em;
    border-bottom: 1px dashed var(--border);
}
.md-body h3 { font-size: var(--font-xl); }
.md-body h4 { font-size: var(--font-lg); }
.md-body h5, .md-body h6 { font-size: var(--font-base); }
.md-body .header-anchor {
    float: left;
    margin-left: -0.9em;
    padding-right: 0.2em;
    opacity: 0;
    color: var(--muted-foreground);
    font-weight: 400;
    transition: opacity 0.15s ease-out;
    user-select: none;
    /* The `.md-body a` rule below applies `text-decoration: underline` to
       every anchor. The header-anchor is a typographic icon, not a
       navigable link in the visual sense, so drop the underline. */
    text-decoration: none;
}
.md-body h1:hover .header-anchor,
.md-body h2:hover .header-anchor,
.md-body h3:hover .header-anchor,
.md-body h4:hover .header-anchor,
.md-body h5:hover .header-anchor,
.md-body h6:hover .header-anchor {
    opacity: 1;
}
/* On narrow viewports the negative `margin-left: -0.9em` pushes the `#`
   sign past the left edge of the viewport (anchor x ≈ -11px on a 380px
   viewport), so hover-revealed anchors get clipped. On mobile we drop
   the float + negative margin and absolutely position the anchor at
   the heading's left edge instead, with the heading content shifted
   right via `padding-left`. This way:

   * The `#` stays inside the viewport at all times.
   * The `#` baseline aligns with the heading's first line baseline
     (`vertical-align: middle` on a single-line-height inline-flex
     box puts the `#` on the same horizontal line as the heading
     text, not against the heading's overall vertical centre).
   * The heading text gets the full content width (minus the small
     reserved gutter), so a long heading like "Markdown Features" no
     longer wraps with one orphan word on its own line leaving a big
     empty space at the end of the first line.

   Earlier revisions used `height: 100%; align-items: center`, which
   stretched the anchor box to the full heading height and put the
   `#` on the heading's geometric centre. For a wrapped h1 like
   "Markdown Features" that ended up aligned with the second line,
   not the first — visually it looked like the `#` belonged to
   "Features" rather than "Markdown". */
@media (max-width: 767px) {
    /* On narrow viewports the negative `margin-left: -0.9em` on the
       anchor pushes the `#` sign past the left edge of the viewport
       (anchor x ≈ -11px on a 380px viewport), so hover-revealed
       anchors get clipped. We rework the mobile anchor to live
       inside the heading's inline flow instead of floating to the
       left:

       * The heading itself gets a `padding-left` so its inline
         content (the actual heading text) still starts at the same
         visual x as before.
       * The anchor is the FIRST inline child of the heading, with
         a `margin-left: -1.6em` so it pulls itself into the gutter
         reserved by that padding.
       * With `vertical-align: baseline` and a single-line-height
         box (`line-height: 1`), the `#` glyph baseline lines up
         with the heading text baseline on the FIRST line — so the
         `#` and "Markdown" share the same horizontal line, instead
         of the `#` being vertically centred against the whole
         heading box (which, for wrapped headings like "Markdown
         Features", used to put `#` on the line between the two
         words).
       * The heading text gets the full content width (minus the
         gutter), so a long heading like "Markdown Features" no
         longer wraps with one orphan word on its own line leaving
         a big empty space at the end of the first line — because
         the anchor no longer occupies any inline space on the
         first line. */
    .md-body h1,
    .md-body h2,
    .md-body h3,
    .md-body h4,
    .md-body h5,
    .md-body h6 {
        padding-left: 1.6em;
    }
    .md-body .header-anchor {
        float: none;
        margin-left: -1.6em;
        margin-right: 0;
        padding: 0;
        width: 1.6em;
        display: inline-flex;
        align-items: flex-end;
        justify-content: flex-start;
        font-size: 0.85em;
        line-height: 1;
        vertical-align: baseline;
    }
}
.md-body p, .md-body ul, .md-body ol, .md-body blockquote, .md-body pre, .md-body table {
    /* Token rhythm: `space-sm` between blocks of copy, matching
       `c_page_title` / `c_page_subtitle` in the example app. */
    margin: var(--space-sm) 0;
}
.md-body ul, .md-body ol {
    padding-left: 1.4em;
}
.md-body ul { list-style: disc; }
.md-body ol { list-style: decimal; }
.md-body ul ul, .md-body ul ol, .md-body ol ul, .md-body ol ol {
    margin: var(--space-xs) 0;
}
.md-body li { margin: var(--space-xs) 0; }
.md-body li input[type="checkbox"] {
    margin-right: 0.4em;
    accent-color: var(--accent);
}
.md-body a {
    color: var(--accent);
    font-weight: 500;
    /* Static dashed underline, no hover variant. A link that restyles itself
       on hover reads as a state change rather than a link, and the dashed ->
       solid swap made every hovered link shimmer. Keep one treatment for both
       states; the accent colour alone carries the affordance. */
    text-decoration: underline;
    text-underline-offset: 3px;
    text-decoration-style: dashed;
    text-decoration-color: var(--border);
}
.md-body strong { font-weight: 700; }
.md-body em { font-style: italic; }
.md-body del { opacity: 0.6; }
.md-body hr {
    border: none;
    border-top: 1px dashed var(--border);
    margin: 2em 0;
}
.md-body blockquote {
    margin: 1em 0;
    padding: 0.4em 1em;
    border-left: 4px solid var(--border);
    color: var(--muted-foreground);
}
.md-body blockquote p { margin: 0.4em 0; }
.md-body code {
    font-family: ui-monospace, monospace;
    font-size: 0.875em;
    padding: 0.15em 0.4em;
    background: var(--accent-muted);
    border: 1px solid var(--border);
    /* Inline <code> that wraps to multiple lines must keep its border
       on every fragment, otherwise the first line loses its right border
       and subsequent lines lose their left border.

       box-decoration-break: clone alone is not enough in practice: the
       per-fragment borders stack against the next fragment's background
       and the visual gap between fragments collapses, making the right
       border of one line look like the left border of the next (and
       neither looks fully closed). Making the element inline-block
       guarantees each fragment draws its own box with its own four
       borders, exactly the same trick used by euv_tag. */
    -webkit-box-decoration-break: clone;
    box-decoration-break: clone;
    display: inline-block;
    /* `vertical-align: baseline` plus `line-height: 1` keeps the
       <code> box visually on the same baseline as the surrounding
       text. Earlier revisions used `vertical-align: text-top` with
       `line-height: 1.4`, which pushed the box downward and made the
       framed text look like it was sitting on a lower line than the
       surrounding body text — a small offset, but very noticeable in
       tight running prose like list items and paragraphs. With
       `line-height: 1` the inline-block has no extra leading so its
       baseline aligns with the baseline of the parent line. */
    vertical-align: baseline;
    line-height: 1;
}
/* In a table cell an inline-block <code> collapses to the width of
   its longest word plus padding, leaving a tall narrow box with large
   blank gaps on every other line. Promote it to `display: block` so
   the box fills the cell and the text wraps naturally, while still
   preserving the per-fragment border from the rule above.

   We do NOT use `td > code` here. euv-docs / VuePress insert
   `display: contents` <div> / <slot> wrappers between the cell and
   the <code> element at runtime, so the direct-child selector
   never matches in the deployed build. Using `td code` instead
   reaches the same <code> regardless of those transparent wrappers.

   Specificity check: `.md-body td code` is (0,1,2) and the base
   `.md-body code` rule is (0,1,1), so this rule wins without
   needing `!important` or repeated `.md-body` classes. */
.md-body td code,
.md-body th code {
    display: block;
}
.md-body pre {
    padding: 1em 1.2em;
    overflow-x: auto;
    border: 1px solid var(--border);
    background: var(--accent-muted);
}
.md-body pre code {
    padding: 0;
    border: none;
    background: transparent;
    font-size: 0.875rem;
    line-height: 1.6;
}
.md-body table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--font-sm);
    display: block;
    overflow-x: auto;
}
.md-body table thead { border-bottom: 2px solid var(--border); }
.md-body table th, .md-body table td {
    padding: 0.5em 0.9em;
    border: 1px solid var(--border);
    text-align: left;
}
.md-body table th { font-weight: 700; }
.md-body table tbody tr:nth-child(2n) { background: var(--accent-muted); }
.md-body img { max-width: 100%; }
.md-body .docs-container {
    margin: 1.2em 0;
    padding: 0.1em 1.2em;
    border-left: 4px solid var(--foreground);
    background: var(--accent-muted);
}
.md-body .docs-container .docs-container-title {
    font-weight: 700;
    font-size: var(--font-sm);
    letter-spacing: 0.04em;
    text-transform: uppercase;
    margin: 0.8em 0 0.4em;
}
.md-body .docs-container.tip { border-left-style: solid; }
.md-body .docs-container.warning { border-left-style: dashed; }
.md-body .docs-container.danger {
    border-left: 4px double var(--foreground);
}
.md-body .docs-container.details {
    border-left: 1px solid var(--border);
    background: transparent;
}
.md-body .footnote-definition { font-size: var(--font-sm); color: var(--muted-foreground); }
"#;

/// Global stylesheet: reset, base element defaults, and theme background.
///
/// The background is spliced from the `background` design token so the
/// page follows the active theme; the rest is a static reset.
pub(crate) const APP_GLOBAL_CSS_HEAD: &str =
    "html, body, #app { height: 100%; margin: 0; padding: 0; background: ";

/// Global stylesheet: box-sizing reset, form control inheritance, and link styling.
pub(crate) const APP_GLOBAL_CSS_RESET: &str = "* { -webkit-tap-highlight-color: transparent; box-sizing: border-box; margin: 0; padding: 0; border: 0; font: inherit; vertical-align: baseline; } ";

/// Global stylesheet: root line-height and iOS text-size adjustment.
pub(crate) const APP_GLOBAL_CSS_ROOT: &str =
    "html { line-height: 1.5; -webkit-text-size-adjust: 100%; } ";

/// Global stylesheet: list markers removed from `ol` / `ul`.
pub(crate) const APP_GLOBAL_CSS_LIST: &str = "ol, ul { list-style: none; } ";

/// Global stylesheet: media elements block-level and width-constrained.
pub(crate) const APP_GLOBAL_CSS_MEDIA: &str =
    "img, picture, video, canvas, svg { display: block; max-width: 100%; } ";

/// Global stylesheet: form controls inherit font and color, transparent background.
pub(crate) const APP_GLOBAL_CSS_FORM: &str =
    "input, button, textarea, select { font: inherit; color: inherit; background: transparent; } ";

/// Global stylesheet: pointer cursor on buttons.
pub(crate) const APP_GLOBAL_CSS_BUTTON: &str = "button { cursor: pointer; } ";

/// Global stylesheet: anchors render as undecorated, inheriting link color.
pub(crate) const APP_GLOBAL_CSS_LINK: &str = "a { text-decoration: none; color: inherit; }";

/// Global stylesheet: the separator that closes the background rule.
pub(crate) const APP_GLOBAL_CSS_BACKGROUND_CLOSE: &str = "; } ";

/// Scrollbar stylesheet: thin Firefox scrollbars.
pub(crate) const APP_SCROLLBAR_CSS_THIN: &str = "* { scrollbar-width: thin; } ";

/// Scrollbar stylesheet: WebKit scrollbar track and thumb dimensions.
pub(crate) const APP_SCROLLBAR_CSS_WEBKIT: &str =
    "::-webkit-scrollbar { width: 6px; height: 6px; } ";

/// Scrollbar stylesheet: transparent WebKit scrollbar track.
pub(crate) const APP_SCROLLBAR_CSS_TRACK: &str =
    "::-webkit-scrollbar-track { background: transparent; } ";

/// Scrollbar stylesheet: square WebKit scrollbar thumb.
pub(crate) const APP_SCROLLBAR_CSS_THUMB: &str = "::-webkit-scrollbar-thumb { border-radius: 0; } ";

/// Scrollbar stylesheet: hidden WebKit scrollbar buttons.
pub(crate) const APP_SCROLLBAR_CSS_BUTTON: &str = "::-webkit-scrollbar-button { display: none !important; width: 0 !important; height: 0 !important; } ";

/// Scrollbar stylesheet: transparent WebKit scrollbar corner.
pub(crate) const APP_SCROLLBAR_CSS_CORNER: &str =
    "::-webkit-scrollbar-corner { background: transparent; }";

/// Scrollbar stylesheet: narrow viewports hide the scrollbar entirely.
pub(crate) const APP_SCROLLBAR_CSS_MOBILE: &str = "@media (max-width: 767px) { * { scrollbar-width: none; } ::-webkit-scrollbar { width: 0px; height: 0px; } }";

/// Keyframes: continuous rotation used by spinners.
pub(crate) const APP_KEYFRAMES_SPIN: &str =
    "@keyframes euv-spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } } ";

/// Keyframes: short fade-and-rise used by entering content.
pub(crate) const APP_KEYFRAMES_FADE_IN: &str = "@keyframes euv-fade-in { from { opacity: 0; transform: translateY(4px); } to { opacity: 1; transform: translateY(0); } } ";

/// Keyframes: attention pulse used by status indicators.
pub(crate) const APP_KEYFRAMES_PULSE: &str =
    "@keyframes euv-pulse { 0%, 100% { transform: scale(1); } 50% { transform: scale(1.15); } } ";

/// Keyframes: bar width sweep used by progress indicators.
pub(crate) const APP_KEYFRAMES_PROGRESS: &str =
    "@keyframes euv-progress { from { width: 0%; } to { width: 100%; } } ";

/// Keyframes: scale-and-rise entrance used by modals.
pub(crate) const APP_KEYFRAMES_SCALE_IN_MODAL: &str = "@keyframes euv-scale-in-modal { from { opacity: 0; transform: translateY(24px) scale(0.95); } to { opacity: 1; transform: translateY(0) scale(1); } } ";

/// Accessibility: suppress the focus ring on keyboard-focusable controls.
pub(crate) const APP_A11Y_CSS_FOCUS_VISIBLE: &str = ":focus-visible { outline: none }";

/// Accessibility: disable animations for users who ask for reduced motion.
pub(crate) const APP_A11Y_CSS_REDUCED_MOTION: &str = "@media (prefers-reduced-motion: reduce) { *, *::before, *::after { animation-iteration-count: 1 !important; scroll-behavior: auto !important; } } ";

/// Accessibility: suppress hover transforms on touch-primary devices.
pub(crate) const APP_A11Y_CSS_COARSE_POINTER: &str = "@media (hover: none) and (pointer: coarse) { * { -webkit-tap-highlight-color: transparent; } .c_card:hover, .c_home_stat_card:hover { transform: none !important; } .c_home_btn_primary:hover, .c_home_btn_secondary:hover { transform: none !important; } } ";
