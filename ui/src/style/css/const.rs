/// Raw stylesheet for rendered markdown bodies ([`crate::euv_markdown`]).
///
/// `class!` cannot express descendant selectors, so markdown typography is
/// injected once at startup through `Css::inject_css`. All colors reference
/// the theme CSS variables, so light/dark switching works unchanged.
pub(crate) const EUV_MD_CSS: &str = r#".md-body{line-height:1.7;font-size:var(--font-base);word-wrap:break-word}.md-body h1,.md-body h2,.md-body h3,.md-body h4,.md-body h5,.md-body h6{position:relative;font-weight:700;letter-spacing:-.01em;margin-top:var(--gap-component);margin-bottom:var(--space-sm);scroll-margin-top:72px;line-height:1.3}.md-body h1{font-size:var(--font-3xl);margin-top:0;padding-bottom:.4em;border-bottom:1px solid var(--border)}.md-body h2{font-size:var(--font-2xl);padding-bottom:.3em;border-bottom:1px dashed var(--border)}.md-body h3{font-size:var(--font-xl)}.md-body h4{font-size:var(--font-lg)}.md-body h5,.md-body h6{font-size:var(--font-base)}.md-body .header-anchor{float:left;margin-left:-.9em;padding-right:.2em;opacity:0;color:var(--muted-foreground);font-weight:400;transition:opacity .15s ease-out;user-select:none;text-decoration:none}.md-body h1:hover .header-anchor,.md-body h2:hover .header-anchor,.md-body h3:hover .header-anchor,.md-body h4:hover .header-anchor,.md-body h5:hover .header-anchor,.md-body h6:hover .header-anchor{opacity:1}@media(max-width:767px){.md-body h1,.md-body h2,.md-body h3,.md-body h4,.md-body h5,.md-body h6{padding-left:1.6em}.md-body .header-anchor{float:none;margin-left:-1.6em;margin-right:0;padding:0;width:1.6em;display:inline-flex;align-items:flex-end;justify-content:flex-start;font-size:.85em;line-height:1;vertical-align:baseline}}.md-body p,.md-body ul,.md-body ol,.md-body blockquote,.md-body pre,.md-body table{margin:var(--space-sm) 0}.md-body ul,.md-body ol{padding-left:1.4em}.md-body ul{list-style:disc}.md-body ol{list-style:decimal}.md-body ul ul,.md-body ul ol,.md-body ol ul,.md-body ol ol,.md-body li{margin:var(--space-xs) 0}.md-body li input[type=checkbox]{margin-right:.4em;accent-color:var(--accent)}.md-body a{color:var(--accent);font-weight:500;text-decoration:underline;text-underline-offset:3px;text-decoration-style:dashed;text-decoration-color:var(--border)}.md-body strong{font-weight:700}.md-body em{font-style:italic}.md-body del{opacity:.6}.md-body hr{border:none;border-top:1px dashed var(--border);margin:2em 0}.md-body blockquote{margin:1em 0;padding:.4em 1em;border-left:4px solid var(--border);color:var(--muted-foreground)}.md-body blockquote p{margin:.4em 0}.md-body code{font-family:ui-monospace,monospace;font-size:.875em;padding:.45em .4em;margin:.15em 0;background:var(--accent-muted);border:1px solid var(--border);-webkit-box-decoration-break:clone;box-decoration-break:clone;display:inline-block;vertical-align:baseline;line-height:1;overflow-wrap:anywhere}.md-body td code,.md-body th code{display:inline-block;margin:0}.md-body pre{padding:1em 1.2em;overflow-x:auto;border:1px solid var(--border);background:var(--accent-muted)}.md-body pre code{padding:0;border:none;background:transparent;font-size:.875rem;line-height:1.6;margin:0}.md-body table{width:100%;border-collapse:collapse;font-size:var(--font-sm)}.md-body table thead{border-bottom:2px solid var(--border)}.md-body table th,.md-body table td{padding:.5em .9em;border:1px solid var(--border);text-align:left;white-space:nowrap}.md-body table th{font-weight:700}.md-body table tbody tr:nth-child(2n){background:var(--accent-muted)}.md-body img{max-width:100%}.md-body .docs-container{margin:1.2em 0;padding:.1em 1.2em;border-left:4px solid var(--foreground);background:var(--accent-muted)}.md-body .docs-container .docs-container-title{font-weight:700;font-size:var(--font-sm);letter-spacing:.04em;text-transform:uppercase;margin:.8em 0 .4em}.md-body .docs-container.tip{border-left-style:solid}.md-body .docs-container.warning{border-left-style:dashed}.md-body .docs-container.danger{border-left:4px double var(--foreground)}.md-body .docs-container.details{border-left:1px solid var(--border);background:transparent}.md-body .footnote-definition{font-size:var(--font-sm);color:var(--muted-foreground)}"#;

/// Global stylesheet: reset, base element defaults, and theme background.
///
/// The background is spliced from the `background` design token so the
/// page follows the active theme; the rest is a static reset.
pub(crate) const APP_GLOBAL_CSS_HEAD: &str =
    "html, body, #app { height: 100%; margin: 0; padding: 0; background: ";

/// Global stylesheet: box-sizing reset, form control inheritance, and link styling.
pub(crate) const APP_GLOBAL_CSS_RESET: &str = r#"*{-webkit-tap-highlight-color:transparent;box-sizing:border-box;margin:0;padding:0;border:0;font:inherit;vertical-align:baseline}"#;

/// Global stylesheet: root line-height and iOS text-size adjustment.
pub(crate) const APP_GLOBAL_CSS_ROOT: &str =
    "html { line-height: 1.5; -webkit-text-size-adjust: 100%; } ";

/// Global stylesheet: list markers removed from `ol` / `ul`.
pub(crate) const APP_GLOBAL_CSS_LIST: &str = r#"ol,ul{list-style:none}"#;

/// Global stylesheet: media elements block-level and width-constrained.
pub(crate) const APP_GLOBAL_CSS_MEDIA: &str =
    "img, picture, video, canvas, svg { display: block; max-width: 100%; } ";

/// Global stylesheet: form controls inherit font and color, transparent background.
pub(crate) const APP_GLOBAL_CSS_FORM: &str =
    "input, button, textarea, select { font: inherit; color: inherit; background: transparent; } ";

/// Global stylesheet: pointer cursor on buttons.
pub(crate) const APP_GLOBAL_CSS_BUTTON: &str = r#"button{cursor:pointer}"#;

/// Global stylesheet: anchors render as undecorated, inheriting link color.
pub(crate) const APP_GLOBAL_CSS_LINK: &str = r#"a{text-decoration:none;color:inherit}"#;

/// Global stylesheet: the separator that closes the background rule.
pub(crate) const APP_GLOBAL_CSS_BACKGROUND_CLOSE: &str = "; } ";

/// Scrollbar stylesheet: thin Firefox scrollbars.
pub(crate) const APP_SCROLLBAR_CSS_THIN: &str = r#"*{scrollbar-width:thin}"#;

/// Scrollbar stylesheet: WebKit scrollbar track and thumb dimensions.
pub(crate) const APP_SCROLLBAR_CSS_WEBKIT: &str =
    "::-webkit-scrollbar { width: 6px; height: 6px; } ";

/// Scrollbar stylesheet: transparent WebKit scrollbar track.
pub(crate) const APP_SCROLLBAR_CSS_TRACK: &str =
    "::-webkit-scrollbar-track { background: transparent; } ";

/// Scrollbar stylesheet: square WebKit scrollbar thumb.
pub(crate) const APP_SCROLLBAR_CSS_THUMB: &str = r#"::-webkit-scrollbar-thumb{border-radius:0}"#;

/// Scrollbar stylesheet: hidden WebKit scrollbar buttons.
pub(crate) const APP_SCROLLBAR_CSS_BUTTON: &str =
    r#"::-webkit-scrollbar-button{display:none!important;width:0!important;height:0!important}"#;

/// Scrollbar stylesheet: transparent WebKit scrollbar corner.
pub(crate) const APP_SCROLLBAR_CSS_CORNER: &str =
    "::-webkit-scrollbar-corner { background: transparent; }";

/// Scrollbar stylesheet: narrow viewports hide the scrollbar entirely.
pub(crate) const APP_SCROLLBAR_CSS_MOBILE: &str =
    r#"@media(max-width:767px){*{scrollbar-width:none}::-webkit-scrollbar{width:0px;height:0px}}"#;

/// Keyframes: continuous rotation used by spinners.
pub(crate) const APP_KEYFRAMES_SPIN: &str =
    "@keyframes euv-spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } } ";

/// Keyframes: short fade-and-rise used by entering content.
pub(crate) const APP_KEYFRAMES_FADE_IN: &str = r#"@keyframes euv-fade-in{0%{opacity:0;transform:translateY(4px)}to{opacity:1;transform:translateY(0)}}"#;

/// Keyframes: attention pulse used by status indicators.
pub(crate) const APP_KEYFRAMES_PULSE: &str =
    "@keyframes euv-pulse { 0%, 100% { transform: scale(1); } 50% { transform: scale(1.15); } } ";

/// Keyframes: bar width sweep used by progress indicators.
pub(crate) const APP_KEYFRAMES_PROGRESS: &str =
    "@keyframes euv-progress { from { width: 0%; } to { width: 100%; } } ";

/// Keyframes: scale-and-rise entrance used by modals.
pub(crate) const APP_KEYFRAMES_SCALE_IN_MODAL: &str = r#"@keyframes euv-scale-in-modal{0%{opacity:0;transform:translateY(24px) scale(.95)}to{opacity:1;transform:translateY(0) scale(1)}}"#;

/// Accessibility: suppress the focus ring on keyboard-focusable controls.
pub(crate) const APP_A11Y_CSS_FOCUS_VISIBLE: &str = r#":focus-visible{outline:none}"#;

/// Accessibility: disable animations for users who ask for reduced motion.
pub(crate) const APP_A11Y_CSS_REDUCED_MOTION: &str = r#"@media(prefers-reduced-motion:reduce){*,*:before,*:after{animation-iteration-count:1!important;scroll-behavior:auto!important}}"#;

/// Accessibility: suppress hover transforms on touch-primary devices.
pub(crate) const APP_A11Y_CSS_COARSE_POINTER: &str = r#"@media(hover:none)and (pointer:coarse){*{-webkit-tap-highlight-color:transparent}.c_card:hover,.c_home_stat_card:hover,.c_home_btn_primary:hover,.c_home_btn_secondary:hover{transform:none!important}}"#;
