/// Raw stylesheet for rendered markdown bodies ([`crate::euv_markdown`]).
///
/// `class!` cannot express descendant selectors, so markdown typography is
/// injected once at startup through `Css::inject_css`. All colors reference
/// the theme CSS variables, so light/dark switching works unchanged.
pub(crate) const EUV_MD_CSS: &str = r#".md-body{line-height:1.7;font-size:var(--font-base);word-wrap:break-word}.md-body h1,.md-body h2,.md-body h3,.md-body h4,.md-body h5,.md-body h6{position:relative;font-weight:700;letter-spacing:-.01em;margin-top:var(--gap-component);margin-bottom:var(--space-sm);scroll-margin-top:72px;line-height:1.3}.md-body h1{font-size:var(--font-3xl);margin-top:0;padding-bottom:.4em;border-bottom:1px solid var(--border)}.md-body h2{font-size:var(--font-2xl);padding-bottom:.3em;border-bottom:1px solid var(--border)}.md-body h3{font-size:var(--font-xl)}.md-body h4{font-size:var(--font-lg)}.md-body h5,.md-body h6{font-size:var(--font-base)}.md-body .header-anchor{float:left;margin-left:-.9em;padding-right:.2em;opacity:0;color:var(--muted-foreground);font-weight:400;transition:opacity .15s ease-out;user-select:none;text-decoration:none}.md-body h1:hover .header-anchor,.md-body h2:hover .header-anchor,.md-body h3:hover .header-anchor,.md-body h4:hover .header-anchor,.md-body h5:hover .header-anchor,.md-body h6:hover .header-anchor{opacity:1}@media(max-width:767px){.md-body h1,.md-body h2,.md-body h3,.md-body h4,.md-body h5,.md-body h6{padding-left:1.6em}.md-body .header-anchor{float:none;margin-left:-1.6em;margin-right:0;padding:0;width:1.6em;display:inline-flex;align-items:flex-end;justify-content:flex-start;font-size:.85em;line-height:1;vertical-align:baseline}}.md-body p,.md-body ul,.md-body ol,.md-body blockquote,.md-body pre,.md-body table{margin:var(--space-sm) 0}.md-body ul,.md-body ol{padding-left:1.4em}.md-body ul{list-style:disc}.md-body ol{list-style:decimal}.md-body ul ul,.md-body ul ol,.md-body ol ul,.md-body ol ol,.md-body li{margin:var(--space-xs) 0}.md-body li input[type=checkbox]{margin-right:.4em;accent-color:var(--primary)}.md-body a{color:var(--primary);font-weight:500;text-decoration:underline;text-underline-offset:3px;text-decoration-style:dashed;text-decoration-color:var(--border)}.md-body strong{font-weight:700}.md-body em{font-style:italic}.md-body del{opacity:.6}.md-body hr{border:none;border-top:1px solid var(--border);margin:2em 0}.md-body blockquote{margin:1em 0;padding:.4em 1em;border-left:4px solid var(--border);color:var(--muted-foreground)}.md-body blockquote p{margin:.4em 0}.md-body code{font-family:var(--font-family-mono);font-size:.875em;padding:.45em .4em;margin:.15em 0;background:var(--muted);border:1px solid var(--border);-webkit-box-decoration-break:clone;box-decoration-break:clone;display:inline-block;vertical-align:baseline;line-height:1;overflow-wrap:anywhere}.md-body td code,.md-body th code{display:inline-block;margin:0}.md-body pre{padding:1em 1.2em;overflow-x:auto;border:1px solid var(--border);background:var(--muted)}.md-body pre code{padding:0;border:none;background:transparent;font-size:.875rem;line-height:1.6;margin:0}.md-body table{width:100%;border-collapse:collapse;font-size:var(--font-sm)}.md-body table thead{border-bottom:2px solid var(--border)}.md-body table th,.md-body table td{padding:.5em .9em;border:1px solid var(--border);text-align:left;white-space:nowrap}.md-body table th{font-weight:700}.md-body table tbody tr:nth-child(2n){background:var(--muted)}.md-body img{max-width:100%}.md-body .docs-container{margin:1.2em 0;padding:.1em 1.2em;border-left:4px solid var(--foreground);background:var(--muted)}.md-body .docs-container .docs-container-title{font-weight:700;font-size:var(--font-sm);letter-spacing:.04em;text-transform:uppercase;margin:.8em 0 .4em}.md-body .docs-container.tip{border-left-style:solid}.md-body .docs-container.warning{border-left-style:dashed}.md-body .docs-container.danger{border-left:4px double var(--foreground)}.md-body .docs-container.details{border-left:1px solid var(--border);background:transparent}.md-body .footnote-definition{font-size:var(--font-sm);color:var(--muted-foreground)}"#;

/// Removes UA decoration without changing semantic display or intrinsic media sizing.
///
/// Zero-specificity element selectors and type-specificity pseudo-elements let
/// component classes win independently of stylesheet insertion order.
pub(crate) const APP_GLOBAL_CSS_RESET: &str = r#"
:where(*),:where(*)::before,:where(*)::after {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    border: 0 solid;
    border-radius: 0;
    font: inherit;
    font-feature-settings: inherit;
    font-variation-settings: inherit;
    letter-spacing: inherit;
    color: inherit;
    background: transparent;
    box-shadow: none;
    text-decoration: none;
    text-transform: none;
    text-shadow: none;
}
:where(*)::backdrop {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    border: 0 solid;
    border-radius: 0;
    font: inherit;
    font-feature-settings: inherit;
    font-variation-settings: inherit;
    letter-spacing: inherit;
    color: inherit;
    background: transparent;
    box-shadow: none;
    text-decoration: none;
    text-transform: none;
    text-shadow: none;
}
:where([hidden]:not([hidden="until-found" i]),input[type="hidden" i],script,style,template,dialog:not([open])) {
    display: none !important;
}
"#;

/// Retains full-height application shells and disables mobile text inflation.
pub(crate) const APP_GLOBAL_CSS_ROOT: &str = r#"
:where(html,body,[id="app"]) { height: 100%; }
:where(html) {
    -webkit-text-size-adjust: 100%;
    text-size-adjust: 100%;
}
"#;

/// Removes UA list, table, quotation, and fieldset decoration.
pub(crate) const APP_GLOBAL_CSS_LIST: &str = r#"
:where(ol,ul,menu,summary) { list-style: none; }
:where(summary)::-webkit-details-marker { display: none; }
:where(table) { border-collapse: collapse; border-spacing: 0; text-indent: 0; }
:where(caption,th) { text-align: inherit; }
:where(blockquote,q) { quotes: none; }
:where(blockquote,q)::before,:where(blockquote,q)::after { content: none; }
:where(fieldset) { min-inline-size: 0; }
:where(legend) { max-inline-size: 100%; white-space: normal; }
:where(hr) { height: 0; }
:where(abbr[title]) { text-decoration: none; }
:where(sub,sup) { position: static; font-size: inherit; vertical-align: baseline; }
"#;

/// Normalizes replaced-element alignment while retaining width, height, fill, and stroke.
pub(crate) const APP_GLOBAL_CSS_MEDIA: &str = r#"
:where(img,picture,video,canvas,svg,audio,iframe,embed,object) { vertical-align: baseline; }
"#;

/// Removes native form chrome and engine-specific control decoration.
///
/// Vendor pseudo-elements remain separate rules so an unsupported selector
/// cannot invalidate another engine's reset. Native focus outlines are retained.
pub(crate) const APP_GLOBAL_CSS_FORM: &str = r#"
:where(input,button,textarea,select,optgroup,option,output,meter,progress) {
    font: inherit;
    line-height: inherit;
    text-align: inherit;
    color: inherit;
    background: transparent;
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
}
:where(button,input,select,textarea) { text-transform: none; }
:where(button,input) { overflow: visible; }
:where(textarea) { resize: none; }
:where(select) { background-image: none; }
:where(input)::placeholder { color: inherit; opacity: 1; }
:where(textarea)::placeholder { color: inherit; opacity: 1; }
:where(input[type="number" i]) { -moz-appearance: textfield; }
:where(input[type="search" i]) { outline-offset: 0; }
:where(input)::-webkit-search-decoration { -webkit-appearance: none; }
:where(input)::-webkit-search-cancel-button { -webkit-appearance: none; }
:where(input)::-webkit-search-results-button { -webkit-appearance: none; }
:where(input)::-webkit-search-results-decoration { -webkit-appearance: none; }
:where(input)::-webkit-inner-spin-button { -webkit-appearance: none; margin: 0; }
:where(input)::-webkit-outer-spin-button { -webkit-appearance: none; margin: 0; }
:where(button,input)::-moz-focus-inner { border: 0; padding: 0; }
:where(input)::-moz-number-spin-box { -moz-appearance: none; }
:where(input)::-webkit-file-upload-button {
    -webkit-appearance: none;
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    border: 0 solid;
    border-radius: 0;
    font: inherit;
    letter-spacing: inherit;
    color: inherit;
    background: transparent;
    box-shadow: none;
    text-decoration: none;
    text-transform: none;
}
:where(input)::file-selector-button {
    appearance: none;
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    border: 0 solid;
    border-radius: 0;
    font: inherit;
    letter-spacing: inherit;
    color: inherit;
    background: transparent;
    box-shadow: none;
    text-decoration: none;
    text-transform: none;
}
:where(input)::-webkit-date-and-time-value { min-height: 0; text-align: inherit; }
:where(input)::-webkit-datetime-edit { display: inline-flex; padding: 0; }
:where(input)::-webkit-datetime-edit-fields-wrapper { padding: 0; }
:where(input)::-webkit-datetime-edit-text { padding: 0; }
:where(input)::-webkit-datetime-edit-year-field { padding: 0; }
:where(input)::-webkit-datetime-edit-month-field { padding: 0; }
:where(input)::-webkit-datetime-edit-day-field { padding: 0; }
:where(input)::-webkit-datetime-edit-hour-field { padding: 0; }
:where(input)::-webkit-datetime-edit-minute-field { padding: 0; }
:where(input)::-webkit-datetime-edit-second-field { padding: 0; }
:where(input)::-webkit-datetime-edit-millisecond-field { padding: 0; }
:where(input)::-webkit-datetime-edit-ampm-field { padding: 0; }
:where(input)::-webkit-calendar-picker-indicator { margin: 0; padding: 0; }
:where(progress) { vertical-align: baseline; }
"#;

/// Keyframes: continuous rotation used by spinners.
pub(crate) const APP_KEYFRAMES_SPIN: &str =
    "@keyframes euv-spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } } ";

/// Keyframes: short fade-and-rise used by entering content.
pub(crate) const APP_KEYFRAMES_FADE_IN: &str = r#"@keyframes euv-fade-in{0%{opacity:0;transform:translateY(4px)}to{opacity:1;transform:translateY(0)}}"#;

/// Keyframes: attention pulse used by status indicators.
pub(crate) const APP_KEYFRAMES_PULSE: &str =
    "@keyframes euv-pulse { 0%, 100% { transform: scale(1); } 50% { transform: scale(1.15); } } ";

/// Keyframes: opacity pulse used by skeleton placeholders.
pub(crate) const APP_KEYFRAMES_SKELETON_PULSE: &str =
    "@keyframes euv-skeleton-pulse { 0%, 100% { opacity: 0.55; } 50% { opacity: 1; } } ";

/// Keyframes: bar width sweep used by progress indicators.
pub(crate) const APP_KEYFRAMES_PROGRESS: &str =
    "@keyframes euv-progress { from { width: 0%; } to { width: 100%; } } ";

/// Keyframes: scale-and-rise entrance used by modals.
pub(crate) const APP_KEYFRAMES_SCALE_IN_MODAL: &str = r#"@keyframes euv-scale-in-modal{0%{opacity:0;transform:translateY(24px) scale(.95)}to{opacity:1;transform:translateY(0) scale(1)}}"#;
/// Accessibility: disable animations for users who ask for reduced motion.
///
/// Collapsing the durations is what makes the request effective: the
/// animations and transitions this app ships — entrance fades and press
/// scales — are all short enough that they are gone rather than
/// merely calmed down, while an element that depends on an animation
/// ending (`animation-fill-mode`) still reaches its final state.
pub(crate) const APP_A11Y_CSS_REDUCED_MOTION: &str = r#"@media(prefers-reduced-motion:reduce){*,*:before,*:after{animation-iteration-count:1!important;scroll-behavior:auto!important;animation-duration:0.01ms!important;transition-duration:0.01ms!important}}"#;
use super::*;

/// The empty class, used where a theme variant does not apply.
///
/// A class carries its stylesheet with it, so a theme variant that is not
/// selected must still resolve to a class — the empty one, which adds no
/// rule of its own. The variant is resolved through a reference so the
/// base class it overrides is always resolved first: a class injects its
/// stylesheet on its first call, so the order the two are resolved in is
/// the order their rules land in.
pub(crate) static EMPTY_CLASS: LazyLock<Css> = LazyLock::new(Css::default);
