/// Raw stylesheet for rendered markdown bodies ([`crate::euv_markdown`]).
///
/// `class!` cannot express descendant selectors, so markdown typography is
/// injected once at startup through `Css::inject_css`. All colors reference
/// the theme CSS variables, so light/dark switching works unchanged.
pub(crate) const EUV_MD_CSS: &str = r#".md-body{line-height:1.7;font-size:var(--font-base);word-wrap:break-word}.md-body h1,.md-body h2,.md-body h3,.md-body h4,.md-body h5,.md-body h6{position:relative;font-weight:700;letter-spacing:-.01em;margin-top:var(--gap-component);margin-bottom:var(--space-sm);scroll-margin-top:72px;line-height:1.3}.md-body h1{font-size:var(--font-3xl);margin-top:0;padding-bottom:.4em;border-bottom:1px solid var(--border)}.md-body h2{font-size:var(--font-2xl);padding-bottom:.3em;border-bottom:1px solid var(--border)}.md-body h3{font-size:var(--font-xl)}.md-body h4{font-size:var(--font-lg)}.md-body h5,.md-body h6{font-size:var(--font-base)}.md-body .header-anchor{float:left;margin-left:-.9em;padding-right:.2em;opacity:0;color:var(--muted-foreground);font-weight:400;transition:opacity .15s ease-out;user-select:none;text-decoration:none}.md-body h1:hover .header-anchor,.md-body h2:hover .header-anchor,.md-body h3:hover .header-anchor,.md-body h4:hover .header-anchor,.md-body h5:hover .header-anchor,.md-body h6:hover .header-anchor{opacity:1}@media(max-width:767px){.md-body h1,.md-body h2,.md-body h3,.md-body h4,.md-body h5,.md-body h6{padding-left:1.6em}.md-body .header-anchor{float:none;margin-left:-1.6em;margin-right:0;padding:0;width:1.6em;display:inline-flex;align-items:flex-end;justify-content:flex-start;font-size:.85em;line-height:1;vertical-align:baseline}}.md-body p,.md-body ul,.md-body ol,.md-body blockquote,.md-body pre,.md-body table{margin:var(--space-sm) 0}.md-body ul,.md-body ol{padding-left:1.4em}.md-body ul{list-style:disc}.md-body ol{list-style:decimal}.md-body ul ul,.md-body ul ol,.md-body ol ul,.md-body ol ol,.md-body li{margin:var(--space-xs) 0}.md-body li input[type=checkbox]{margin-right:.4em;accent-color:var(--primary)}.md-body a{color:var(--primary);font-weight:500;text-decoration:underline;text-underline-offset:3px;text-decoration-style:dashed;text-decoration-color:var(--border)}.md-body strong{font-weight:700}.md-body em{font-style:italic}.md-body del{opacity:.6}.md-body hr{border:none;border-top:1px solid var(--border);margin:2em 0}.md-body blockquote{margin:1em 0;padding:.4em 1em;border-left:4px solid var(--border);color:var(--muted-foreground)}.md-body blockquote p{margin:.4em 0}.md-body code{font-family:var(--font-family-mono);font-size:.875em;padding:.45em .4em;margin:.15em 0;background:var(--muted);border:1px solid var(--border);-webkit-box-decoration-break:clone;box-decoration-break:clone;display:inline-block;vertical-align:baseline;line-height:1;overflow-wrap:anywhere}.md-body td code,.md-body th code{display:inline-block;margin:0}.md-body pre{padding:1em 1.2em;overflow-x:auto;border:1px solid var(--border);background:var(--muted)}.md-body pre code{padding:0;border:none;background:transparent;font-size:.875rem;line-height:1.6;margin:0}.md-body table{width:100%;border-collapse:collapse;font-size:var(--font-sm)}.md-body table thead{border-bottom:2px solid var(--border)}.md-body table th,.md-body table td{padding:.5em .9em;border:1px solid var(--border);text-align:left;white-space:nowrap}.md-body table th{font-weight:700}.md-body table tbody tr:nth-child(2n){background:var(--muted)}.md-body img{max-width:100%}.md-body .docs-container{margin:1.2em 0;padding:.1em 1.2em;border-left:4px solid var(--foreground);background:var(--muted)}.md-body .docs-container .docs-container-title{font-weight:700;font-size:var(--font-sm);letter-spacing:.04em;text-transform:uppercase;margin:.8em 0 .4em}.md-body .docs-container.tip{border-left-style:solid}.md-body .docs-container.warning{border-left-style:dashed}.md-body .docs-container.danger{border-left:4px double var(--foreground)}.md-body .docs-container.details{border-left:1px solid var(--border);background:transparent}.md-body .footnote-definition{font-size:var(--font-sm);color:var(--muted-foreground)}"#;

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
pub(crate) const APP_SCROLLBAR_CSS_THUMB: &str =
    r#"::-webkit-scrollbar-thumb{border-radius:var(--radius-full)}"#;

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

/// Accessibility: the focus ring.
///
/// The design system is monochrome: `--border`, `--accent` and `--ring`
/// hold the same ink in both themes, so a focus that only changes the
/// border colour changes nothing on screen — the focus was invisible on
/// every form control.
///
/// The normal ring uses the scheme's `--ring` token. Controls that report
/// `aria-invalid="true"` switch to the destructive role at 20% opacity,
/// or 40% in dark mode, matching the field error treatment.
pub(crate) const APP_A11Y_CSS_FOCUS_RING: &str = r#"input:focus-visible,select:focus-visible,textarea:focus-visible,button:focus-visible,a[href]:focus-visible,[role="treeitem"]:focus-visible > [data-slot="tree-item-row"]{box-shadow:0 0 0 calc(var(--ring-width) + var(--ring-offset)) var(--ring)}input[aria-invalid="true"]:focus-visible,select[aria-invalid="true"]:focus-visible,textarea[aria-invalid="true"]:focus-visible,button[aria-invalid="true"]:focus-visible{box-shadow:0 0 0 var(--ring-width) color-mix(in oklab,var(--destructive) 20%,transparent)}.c_theme_dark input[aria-invalid="true"]:focus-visible,.c_theme_dark select[aria-invalid="true"]:focus-visible,.c_theme_dark textarea[aria-invalid="true"]:focus-visible,.c_theme_dark button[aria-invalid="true"]:focus-visible{box-shadow:0 0 0 var(--ring-width) color-mix(in oklab,var(--destructive) 40%,transparent)}"#;

/// Accessibility: disable animations for users who ask for reduced motion.
///
/// Collapsing the durations is what makes the request effective: the
/// animations and transitions this app ships — entrance fades, hover lifts,
/// press scales — are all short enough that they are gone rather than
/// merely calmed down, while an element that depends on an animation
/// ending (`animation-fill-mode`) still reaches its final state.
pub(crate) const APP_A11Y_CSS_REDUCED_MOTION: &str = r#"@media(prefers-reduced-motion:reduce){*,*:before,*:after{animation-iteration-count:1!important;scroll-behavior:auto!important;animation-duration:0.01ms!important;transition-duration:0.01ms!important}}"#;

/// Accessibility: suppress hover transforms on touch-primary devices.
pub(crate) const APP_A11Y_CSS_COARSE_POINTER: &str = r#"@media(hover:none)and (pointer:coarse){*{-webkit-tap-highlight-color:transparent}.c_card:hover,.c_home_stat_card:hover,.c_home_btn_primary:hover,.c_home_btn_secondary:hover{transform:none!important}}"#;

/// The stylesheet that carries the shadcn/ui scheme's component overrides.
///
/// A class injects its stylesheet on its first call, so the order the base
/// class and its `_shadcn` variant are resolved in is the order their rules
/// land in, and the variant does not reliably win the cascade. Scoping each
/// override to the scheme class fixes that by construction: the scheme
/// class sits on the root element, so a rule under it has a higher
/// specificity than the plain class rule and wins whatever order the two
/// were injected in.
///
/// `class!` cannot express a descendant selector, so the rules are
/// injected as one stylesheet.
#[cfg(feature = "theme-shadcn")]
pub(crate) const APP_SCHEME_SHADCN_CSS: &str = r#".c_scheme_shadcn .c_card { display: flex; flex-direction: column; gap: var(--space-lg); border: none; padding: var(--space-lg); overflow: hidden; }
.c_scheme_shadcn .c_card_title { margin: 0px; font-size: var(--font-base); font-weight: var(--font-weight-medium); letter-spacing: var(--letter-spacing-normal); }
.c_scheme_shadcn .c_euv_accordion_content { line-height: 1.428571; }
.c_scheme_shadcn .c_euv_accordion_item { padding-left: var(--space-lg); }
.c_scheme_shadcn .c_euv_accordion_trigger { padding: 10px 0px; border: 1px solid transparent; border-radius: var(--radius-lg); line-height: 1.428571; align-items: flex-start; border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); }
.c_scheme_shadcn .c_euv_alert_default { padding: 8px 10px; background: var(--card); }
.c_scheme_shadcn .c_euv_alert_destructive { padding: 8px 10px; color: var(--destructive); background: var(--card); border-color: var(--border); }
.c_scheme_shadcn .c_euv_alert_dialog_trigger { height: var(--control-height-sm); gap: var(--space-xs-sm); border-radius: var(--radius-lg); font-weight: var(--font-weight-medium); padding-left: var(--control-pad-inline); }
.c_scheme_shadcn .c_euv_aspect_ratio { background-color: var(--muted); border-bottom-right-radius: var(--radius-lg); border-bottom-left-radius: var(--radius-lg); border-top-right-radius: var(--radius-lg); border-top-left-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_attachment_group { gap: var(--space-md); padding-top: var(--space-xs); }
.c_scheme_shadcn .c_euv_attachment_media { border-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_attachment_media_variant_image { border-bottom-right-radius: var(--radius-lg); border-bottom-left-radius: var(--radius-lg); border-top-right-radius: var(--radius-lg); border-top-left-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_attachment { border-radius: var(--radius-xl); gap: var(--space-xs-sm); }
.c_scheme_shadcn .c_euv_avatar_fallback { background-color: var(--muted); border-bottom-right-radius: 9999px; border-bottom-left-radius: 9999px; border-top-right-radius: 9999px; border-top-left-radius: 9999px; color: var(--muted-foreground); }
.c_scheme_shadcn .c_euv_badge { font-weight: var(--font-weight-medium); }
.c_scheme_shadcn .c_euv_bubble_content { padding-top: var(--space-sm-md); padding-right: var(--space-md); padding-bottom: var(--space-sm-md); padding-left: var(--space-md); border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); border-bottom-right-radius: 24px; border-bottom-left-radius: 24px; border-top-right-radius: 24px; border-top-left-radius: 24px; }
.c_scheme_shadcn .c_euv_bubble_group { gap: var(--space-sm); }
.c_scheme_shadcn .c_euv_bubble_reactions { background-color: var(--muted); border-bottom-right-radius: 9999px; border-bottom-left-radius: 9999px; border-top-right-radius: 9999px; border-top-left-radius: 9999px; gap: var(--space-xs); padding-left: var(--space-xs-sm); padding-top: var(--space-2xs); }
.c_scheme_shadcn .c_euv_bubble { gap: var(--space-xs); }
.c_scheme_shadcn .c_euv_button { border-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_button_size_small, .c_scheme_shadcn .c_euv_button_size_xs, .c_scheme_shadcn .c_euv_button_size_icon, .c_scheme_shadcn .c_euv_button_size_icon_xs, .c_scheme_shadcn .c_euv_button_size_icon_sm { border-radius: var(--radius-md); }
.c_scheme_shadcn .c_euv_card_header { align-items: flex-start; }
.c_scheme_shadcn .c_euv_carousel_next { border-bottom-right-radius: 9999px; border-bottom-left-radius: 9999px; border-top-right-radius: 9999px; border-top-left-radius: 9999px; font-weight: var(--font-weight-medium); }
.c_scheme_shadcn .c_euv_carousel_next_vertical { border-bottom-right-radius: 9999px; border-bottom-left-radius: 9999px; border-top-right-radius: 9999px; border-top-left-radius: 9999px; font-weight: var(--font-weight-medium); }
.c_scheme_shadcn .c_euv_carousel_previous { border-bottom-right-radius: 9999px; border-bottom-left-radius: 9999px; border-top-right-radius: 9999px; border-top-left-radius: 9999px; font-weight: var(--font-weight-medium); }
.c_scheme_shadcn .c_euv_carousel_previous_vertical { border-bottom-right-radius: 9999px; border-bottom-left-radius: 9999px; border-top-right-radius: 9999px; border-top-left-radius: 9999px; font-weight: var(--font-weight-medium); }
.c_scheme_shadcn .c_euv_code_block_actions { gap: var(--space-xs); }
.c_scheme_shadcn .c_euv_code_block_content { padding-left: var(--space-lg); padding-top: var(--space-lg); }
.c_scheme_shadcn .c_euv_code_block_copy_button { font-weight: var(--font-weight-medium); }
.c_scheme_shadcn .c_euv_code_block_footer { padding: var(--space-lg); }
.c_scheme_shadcn .c_euv_code_block_header { padding: var(--space-md); color: var(--muted-foreground); }
.c_scheme_shadcn .c_euv_code_block_line { padding-left: var(--space-lg); }
.c_scheme_shadcn .c_euv_code_block { border-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_code_block_title { color: var(--foreground); }
.c_scheme_shadcn .c_euv_collapsible { gap: var(--space-sm); }
.c_scheme_shadcn .c_euv_collapsible_trigger { gap: var(--space-xs); border-color: var(--border); border-bottom-right-radius: var(--radius-lg); border-bottom-left-radius: var(--radius-lg); border-top-right-radius: var(--radius-lg); border-top-left-radius: var(--radius-lg); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); justify-content: center; white-space: nowrap; }
.c_scheme_shadcn .c_euv_combobox_chip_remove { border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); font-weight: var(--font-weight-medium); opacity: 0.5; }
.c_scheme_shadcn .c_euv_combobox_chip { gap: var(--space-xs); font-size: var(--font-xs); color: var(--foreground); font-weight: var(--font-weight-medium); justify-content: center; padding-left: var(--space-xs-sm); }
.c_scheme_shadcn .c_euv_combobox_chips { border-radius: var(--radius-lg); background-color: transparent; padding-left: var(--control-pad-inline); }
.c_scheme_shadcn .c_euv_combobox_content { border-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_combobox_input { height: var(--control-height-sm); min-height: var(--control-height-sm); padding: 4px 10px; border-radius: var(--radius-lg); font-size: var(--font-sm); line-height: 1.428571; }
.c_scheme_shadcn .c_euv_combobox_item { padding: 4px calc(4px + 2px); border-radius: var(--radius-md); font-size: var(--font-sm); line-height: 1.428571; }
.c_scheme_shadcn .c_euv_combobox_trigger { height: var(--control-height-sm); min-height: var(--control-height-sm); padding: 8px 10px; border-radius: var(--radius-lg); font-size: var(--font-sm); line-height: 1.428571; gap: var(--space-xs-sm); }
.c_scheme_shadcn .c_euv_command_input_wrapper { padding: var(--space-xs); }
.c_scheme_shadcn .c_euv_command_item { padding: var(--space-sm); border-bottom-right-radius: var(--radius-sm); border-bottom-left-radius: var(--radius-sm); border-top-right-radius: var(--radius-sm); border-top-left-radius: var(--radius-sm); padding-top: var(--space-xs-sm); }
.c_scheme_shadcn .c_euv_command { border-radius: var(--radius-lg); border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); padding: var(--space-xs); }
.c_scheme_shadcn .c_euv_context_menu_trigger { border-radius: var(--radius-xl); border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); justify-content: center; }
.c_scheme_shadcn .c_euv_empty_content { gap: var(--space-sm); justify-content: center; }
.c_scheme_shadcn .c_euv_empty { gap: var(--space-lg); padding: var(--space-2xl); border-bottom-right-radius: var(--radius-xl); border-bottom-left-radius: var(--radius-xl); border-top-right-radius: var(--radius-xl); border-top-left-radius: var(--radius-xl); }
.c_scheme_shadcn .c_euv_field_content { gap: var(--space-2xs); }
.c_scheme_shadcn .c_euv_field_group { gap: var(--space-xl); }
.c_scheme_shadcn .c_euv_field_label { align-items: center; }
.c_scheme_shadcn .c_euv_field_set { gap: var(--space-lg); }
.c_scheme_shadcn .c_euv_field_title { align-items: center; }
.c_scheme_shadcn .c_euv_hover_card_trigger { border-bottom-right-radius: var(--radius-md); border-bottom-left-radius: var(--radius-md); border-top-right-radius: var(--radius-md); border-top-left-radius: var(--radius-md); gap: var(--space-xs); justify-content: center; }
.c_scheme_shadcn .c_euv_input_error { background-color: transparent; }
.c_scheme_shadcn .c_euv_input_group_addon_block_end { padding-top: var(--space-xs-sm); }
.c_scheme_shadcn .c_euv_input_group_addon_block_start { padding-top: var(--space-xs-sm); }
.c_scheme_shadcn .c_euv_input_group_addon_inline_end { padding-top: var(--space-xs-sm); }
.c_scheme_shadcn .c_euv_input_group_addon_inline_start { padding-top: var(--space-xs-sm); }
.c_scheme_shadcn .c_euv_input_group_button_icon_sm { border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); font-weight: var(--font-weight-medium); gap: var(--space-sm); }
.c_scheme_shadcn .c_euv_input_group_button_icon_xs { border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); font-weight: var(--font-weight-medium); gap: var(--space-sm); }
.c_scheme_shadcn .c_euv_input_group_button_sm { border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); font-weight: var(--font-weight-medium); gap: var(--space-sm); }
.c_scheme_shadcn .c_euv_input_group_button_xs { border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); font-weight: var(--font-weight-medium); gap: var(--space-sm); padding-left: 0px; padding-right: 0px; }
.c_scheme_shadcn .c_euv_input_group { border-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_input_no_transition { background-color: transparent; }
.c_scheme_shadcn .c_euv_input_otp_slot { border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); }
.c_scheme_shadcn .c_euv_input { min-height: var(--control-height-sm); padding: 4px 10px; font-size: var(--font-sm); border-radius: var(--radius-lg); background-color: transparent; }
.c_scheme_shadcn .c_euv_input_wrapper { gap: var(--space-sm); }
.c_scheme_shadcn .c_euv_item_default { border-color: var(--border); border-bottom-right-radius: var(--radius-lg); border-bottom-left-radius: var(--radius-lg); border-top-right-radius: var(--radius-lg); border-top-left-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_item_group { gap: var(--space-lg); }
.c_scheme_shadcn .c_euv_item_muted { border-bottom-right-radius: var(--radius-lg); border-bottom-left-radius: var(--radius-lg); border-top-right-radius: var(--radius-lg); border-top-left-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_item_outline { border-color: var(--border); border-bottom-right-radius: var(--radius-lg); border-bottom-left-radius: var(--radius-lg); border-top-right-radius: var(--radius-lg); border-top-left-radius: var(--radius-lg); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); }
.c_scheme_shadcn .c_euv_item_size_default { gap: var(--space-sm-md); padding: var(--space-sm-md) var(--space-md); }
.c_scheme_shadcn .c_euv_item_size_sm { gap: var(--space-sm-md); padding: var(--space-sm-md) var(--space-md); }
.c_scheme_shadcn .c_euv_item_size_xs { gap: var(--space-sm); padding: var(--space-sm) var(--space-sm-md); }
.c_scheme_shadcn .c_euv_kbd { gap: var(--space-xs); padding-left: var(--space-xs); }
.c_scheme_shadcn .c_euv_marker { color: var(--muted-foreground); }
.c_scheme_shadcn .c_euv_menubar { border-color: var(--border); border-bottom-right-radius: var(--radius-lg); border-bottom-left-radius: var(--radius-lg); border-top-right-radius: var(--radius-lg); border-top-left-radius: var(--radius-lg); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); gap: var(--space-2xs); }
.c_scheme_shadcn .c_euv_menubar_trigger_open { font-weight: var(--font-weight-medium); padding-left: var(--space-xs-sm); }
.c_scheme_shadcn .c_euv_menubar_trigger { font-weight: var(--font-weight-medium); padding-left: var(--space-xs-sm); }
.c_scheme_shadcn .c_euv_message_content { gap: var(--space-sm-md); }
.c_scheme_shadcn .c_euv_message_footer { color: var(--muted-foreground); font-weight: var(--font-weight-medium); padding-left: var(--space-md); }
.c_scheme_shadcn .c_euv_message_group { gap: var(--space-sm); }
.c_scheme_shadcn .c_euv_message_header { color: var(--muted-foreground); font-weight: var(--font-weight-medium); padding-left: var(--space-md); }
.c_scheme_shadcn .c_euv_message_scroller_button { border-bottom-right-radius: var(--radius-2xl); border-bottom-left-radius: var(--radius-2xl); border-top-right-radius: var(--radius-2xl); border-top-left-radius: var(--radius-2xl); font-weight: var(--font-weight-medium); }
.c_scheme_shadcn .c_euv_message_scroller_content { gap: var(--space-3xl); }
.c_scheme_shadcn .c_euv_native_select { min-height: var(--control-height-sm); padding-block: var(--space-xs); padding-inline: var(--control-pad-inline); padding-right: var(--control-pad-icon); border-radius: var(--radius-lg); font-size: var(--font-sm); line-height: 1.428571; background-color: transparent; }
.c_scheme_shadcn .c_euv_native_select_size_default { height: var(--control-height-sm); min-height: var(--control-height-sm); }
.c_scheme_shadcn .c_euv_native_select_size_sm { height: calc(32px - 4px); min-height: calc(32px - 4px); padding-block: var(--space-2xs); border-radius: var(--radius-md); }
.c_scheme_shadcn .c_euv_navigation_menu_link { padding: var(--space-sm); border-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_navigation_menu_list { align-items: center; gap: 0px; justify-content: center; }
.c_scheme_shadcn .c_euv_navigation_menu { align-items: center; }
.c_scheme_shadcn .c_euv_navigation_menu_trigger { border-bottom-right-radius: var(--radius-lg); border-bottom-left-radius: var(--radius-lg); border-top-right-radius: var(--radius-lg); border-top-left-radius: var(--radius-lg); justify-content: center; padding-left: var(--control-pad-inline); padding-top: var(--space-xs-sm); }
.c_scheme_shadcn .c_euv_pagination_link { border-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_pagination_page { width: var(--control-height-sm); height: var(--control-height-sm); border-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_pagination_ellipsis { width: var(--control-height-sm); height: var(--control-height-sm); }
.c_scheme_shadcn .c_euv_questionnaire_choice_disabled { background-color: transparent; border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); gap: var(--space-sm-md); padding-top: var(--space-sm-md); }
.c_scheme_shadcn .c_euv_questionnaire_choice_enabled { background-color: transparent; border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); gap: var(--space-sm-md); padding-top: var(--space-sm-md); }
.c_scheme_shadcn .c_euv_questionnaire_choice_indicator_dot { background-color: var(--primary-foreground); border-bottom-right-radius: 9999px; border-bottom-left-radius: 9999px; border-top-right-radius: 9999px; border-top-left-radius: 9999px; }
.c_scheme_shadcn .c_euv_questionnaire_choice_indicator { border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); }
.c_scheme_shadcn .c_euv_questionnaire_choice_label { gap: var(--space-2xs); }
.c_scheme_shadcn .c_euv_questionnaire_choice { padding-top: var(--space-sm-md); padding-right: var(--space-md); padding-bottom: var(--space-sm-md); padding-left: var(--space-md); border-radius: var(--radius-lg); background-color: transparent; border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); gap: var(--space-sm-md); }
.c_scheme_shadcn .c_euv_questionnaire_input { height: var(--control-height-sm); border-radius: var(--radius-lg); font-size: var(--font-base); }
.c_scheme_shadcn .c_euv_questionnaire_item { gap: var(--space-lg); }
.c_scheme_shadcn .c_euv_questionnaire_next { height: var(--control-height-sm); gap: var(--space-xs-sm); border-radius: var(--radius-lg); font-weight: var(--font-weight-medium); padding-left: var(--control-pad-inline); }
.c_scheme_shadcn .c_euv_questionnaire_previous { height: var(--control-height-sm); border-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_questionnaire { gap: var(--space-lg); }
.c_scheme_shadcn .c_euv_questionnaire_skip { height: var(--control-height-sm); gap: var(--space-xs-sm); border-bottom-right-radius: var(--radius-lg); border-bottom-left-radius: var(--radius-lg); border-top-right-radius: var(--radius-lg); border-top-left-radius: var(--radius-lg); background-color: var(--background); border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); font-weight: var(--font-weight-medium); padding-left: var(--control-pad-inline); }
.c_scheme_shadcn .c_euv_questionnaire_submit { height: var(--control-height-sm); border-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_questionnaire_title { font-size: var(--font-base); font-weight: var(--font-weight-medium); }
.c_scheme_shadcn .c_euv_radio_group_indicator { align-items: center; justify-content: center; }
.c_scheme_shadcn .c_euv_radio_group_item_checked { border-color: var(--border); border-bottom-right-radius: 9999px; border-bottom-left-radius: 9999px; border-top-right-radius: 9999px; border-top-left-radius: 9999px; border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); }
.c_scheme_shadcn .c_euv_radio_group_item { border-color: var(--border); border-bottom-right-radius: 9999px; border-bottom-left-radius: 9999px; border-top-right-radius: 9999px; border-top-left-radius: 9999px; border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); }
.c_scheme_shadcn .c_euv_radio_group_root { gap: var(--space-sm); }
.c_scheme_shadcn .c_euv_select_option_active { background: transparent; color: var(--foreground); }
.c_scheme_shadcn .c_euv_select_option_indicator { display: inline-flex; flex-shrink: 0; align-items: center; justify-content: center; width: var(--space-lg); color: var(--muted-foreground); font-size: var(--font-sm); line-height: 1; }
.c_scheme_shadcn .c_euv_select_option { border-radius: var(--radius-md); font-size: var(--font-sm); line-height: 1.428571; }
.c_scheme_shadcn .c_euv_select_panel { margin-top: var(--space-xs); border-radius: var(--radius-lg); }
.c_scheme_shadcn .c_euv_select_trigger { height: var(--control-height-sm); gap: var(--space-xs-sm); min-height: var(--control-height-sm); padding: 8px 10px; border-radius: var(--radius-lg); background: transparent; font-size: var(--font-sm); line-height: 1.428571; background-color: transparent; padding-left: var(--control-pad-inline); white-space: nowrap; }
.c_scheme_shadcn .c_euv_sheet_trigger { height: var(--control-height-sm); gap: var(--space-xs-sm); border-radius: var(--radius-lg); font-weight: var(--font-weight-medium); padding-left: var(--control-pad-inline); }
.c_scheme_shadcn .c_euv_sidebar_content { padding-left: var(--control-pad-inline); }
.c_scheme_shadcn .c_euv_sidebar_group_label { border-bottom-right-radius: var(--radius-md); border-bottom-left-radius: var(--radius-md); border-top-right-radius: var(--radius-md); border-top-left-radius: var(--radius-md); }
.c_scheme_shadcn .c_euv_sidebar_menu_button_active { border-color: var(--border); border-bottom-right-radius: var(--radius-md); border-bottom-left-radius: var(--radius-md); border-top-right-radius: var(--radius-md); border-top-left-radius: var(--radius-md); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); font-weight: var(--font-weight-medium); }
.c_scheme_shadcn .c_euv_sidebar_menu_button { border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); font-weight: var(--font-weight-medium); }
.c_scheme_shadcn .c_euv_tab_item_active { background: var(--background); color: var(--foreground); box-shadow: var(--shadow-xs); }
.c_scheme_shadcn .c_euv_tab_item { display: inline-flex; flex: 1; height: calc(100% - 1px); align-items: center; justify-content: center; gap: calc(8px - 2px); padding: 2px calc(4px + 2px); border: 1px solid transparent; border-bottom: none; border-radius: var(--radius-md); background: transparent; color: var(--foreground); font-size: var(--font-sm); font-weight: var(--font-weight-medium); white-space: nowrap; line-height: 1; box-sizing: border-box; user-select: none; -webkit-user-select: none; border-color: var(--border); border-bottom-style: solid; border-left-style: solid; border-right-style: solid; border-top-style: solid; border-bottom-width: var(--border-width); border-left-width: var(--border-width); border-right-width: var(--border-width); border-top-width: var(--border-width); }
.c_scheme_shadcn .c_euv_table_cell { padding: var(--space-sm); }
.c_scheme_shadcn .c_euv_table_header_cell { padding: 0px var(--space-sm); }
.c_scheme_shadcn .c_euv_table { font-size: var(--font-sm); }
.c_scheme_shadcn .c_euv_tabs_bar { width: fit-content; height: var(--control-height-sm); align-items: center; box-sizing: border-box; margin-bottom: 0px; padding: var(--tablist-inset); gap: 0px; color: var(--muted-foreground); justify-content: center; }
.c_scheme_shadcn .c_euv_tabs_panel { padding: 0px; }
.c_scheme_shadcn .c_euv_tabs { gap: var(--space-sm); }
.c_scheme_shadcn .c_euv_textarea { padding: 8px 10px; border-radius: var(--radius-lg); line-height: 1.428571; }
.c_scheme_shadcn .c_euv_toggle_group { width: fit-content; gap: var(--space-sm); border-radius: var(--radius-lg); background-color: transparent; }
.c_scheme_shadcn .c_euv_toggle_outline { background-color: transparent; border-color: var(--border); }
.c_scheme_shadcn .c_euv_toggle_pressed { background: var(--muted); color: var(--foreground); background-color: transparent; }
.c_scheme_shadcn .c_euv_toggle { width: fit-content; gap: var(--space-xs); border-radius: var(--radius-lg); background-color: transparent; }
.c_scheme_shadcn .c_euv_toggle_size_default { height: var(--control-height-sm); min-width: var(--control-height-sm); padding-inline: var(--control-pad-inline); background-color: transparent; }
.c_scheme_shadcn .c_euv_toggle_size_large { height: calc(32px + 4px); min-width: calc(32px + 4px); padding-inline: var(--control-pad-inline); background-color: transparent; }
.c_scheme_shadcn .c_euv_toggle_size_small { height: calc(32px - 4px); min-width: calc(32px - 4px); padding-inline: var(--control-pad-inline); font-size: 0.8rem; background-color: transparent; }
.c_scheme_shadcn .c_euv_trace_collapsible_trigger { border-bottom-right-radius: var(--radius-lg); border-bottom-left-radius: var(--radius-lg); border-top-right-radius: var(--radius-lg); border-top-left-radius: var(--radius-lg); gap: var(--space-xs); }
.c_scheme_shadcn .c_euv_trace_line { gap: var(--space-sm); color: var(--muted-foreground); }
.c_scheme_shadcn .c_euv_trace { gap: var(--space-sm); }
.c_scheme_shadcn .c_form_checkbox_indicator_checked { opacity: 1; }
.c_scheme_shadcn .c_form_checkbox_indicator { position: absolute; top: 50%; left: calc((16px - 4px) / 2); width: var(--space-xs); height: var(--space-sm); border-right: 2px solid var(--primary-foreground); border-bottom: 2px solid var(--primary-foreground); transform: translateY(-50%) rotate(45deg); opacity: 0; pointer-events: none; }
.c_scheme_shadcn .c_form_checkbox_row { position: relative; }
.c_scheme_shadcn .c_form_checkbox { position: relative; display: grid; place-items: center; appearance: none; -webkit-appearance: none; background: var(--background); border: 1px solid var(--input); border-radius: calc(6px - 2px); box-sizing: border-box; transition: 0.15s cubic-bezier(0, 0, 0.2, 1); }
.c_scheme_shadcn .c_form_switch_thumb { position: relative; top: auto; left: auto; width: var(--space-lg); height: var(--space-lg); flex-shrink: 0; background: var(--background); box-shadow: var(--shadow-xs); }
.c_scheme_shadcn .c_form_switch_track_checked { background: var(--primary); }
.c_scheme_shadcn .c_form_switch_track { display: flex; align-items: center; width: var(--control-height-sm); height: var(--switch-track-height); padding: var(--border-width); border: none; border-radius: var(--radius-full); background: var(--input); box-sizing: border-box; }
.c_scheme_shadcn .c_progress_container { flex-wrap: wrap; gap: var(--space-md); }
"#;

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
