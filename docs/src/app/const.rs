//! String constants for the euv-docs crate root (rust-standards §1.3c).
//!
//! `lib.rs` is the crate root, so this module is declared from it as
//! `mod r#const;` — the `r#` prefix is required because `const` is a
//! Rust keyword. Every string literal the §1.3c verifier flags inside a
//! function body of the library crate lives here, verbatim and
//! byte-identical to the inline form it replaced.
//!
//! Values are re-exported by `lib.rs` through the existing
//! `pub(crate) use {.., r#const::*, ..}` glob, so every `use super::*`
//! module below the crate root sees them without any per-module wiring.
//! Constants that belong to a single view (e.g. the layout shell's CSS
//! selectors) live in that view's own `const.rs` instead, because those
//! are not on this crate root's whitelist.

/// The DOM element id the documentation app mounts into.
pub(crate) const APP_MOUNT_SELECTOR: &str = "#app";

/// The error description shown by the `euv_result` 404 component.
pub(crate) const NOT_FOUND_DESCRIPTION: &str = "页面不存在";

/// The label of the "back to the site home" link on the 404 page.
pub(crate) const NOT_FOUND_HOME_LABEL: &str = "首页";

/// The injected stylesheet making the viewport-locked docs shell scroll
/// inside its main column instead of the window.
///
/// Lives in `lib.rs` because the rules target `c_app_main` /
/// `c_mobile_main` in *both* shells and several `c_euv_*` primitives
/// owned by the `euv-ui` crate, so there is no single view module that
/// owns it.
pub(crate) const APP_GLOBAL_CSS: &str = "html, body { height: 100% !important; overflow: hidden !important; } \
         #app { height: 100% !important; } \
         .md-body h1, .md-body h2, .md-body h3, .md-body h4, .md-body h5, .md-body h6 { padding-left: 0 !important; } \
         .md-body .header-anchor, .md-body h1:hover .header-anchor, .md-body h2:hover .header-anchor, .md-body h3:hover .header-anchor, .md-body h4:hover .header-anchor, .md-body h5:hover .header-anchor, .md-body h6:hover .header-anchor { display: none !important; } \
             .md-body img { display: inline-block; width: auto !important; max-width: 100% !important; height: auto; vertical-align: baseline; } \
             .md-body a > img { display: inline-block; } \
             .md-body img[src$='.svg'], .md-body img[src*='shields.io'], .md-body img[src*='github.com'] { max-height: 20px; max-width: 100%; } \
             .md-body table img { max-height: 1.4em; } \
             \
             .c_app_main { padding-top: 4.75rem !important; display: flex !important; flex-direction: column !important; min-height: 0 !important; overflow-y: auto !important; overflow-x: hidden !important; } \
             \
             .c_nav_footer_divider { left: 0.75rem !important; right: 0.75rem !important; } \
             .c_nav_section_label { padding-left: 0.75rem !important; } \
             .c_nav_footer { padding-left: 0.75rem !important; } \
             .c_euv_sidebar_children { margin-left: 8px !important; padding-left: 8px !important; } \
             \
             .c_euv_sidebar_link, .c_euv_sidebar_group_title { position: relative !important; } \
                         .c_euv_sidebar_link::before, .c_euv_sidebar_group_title::before { content: '' !important; position: absolute !important; top: 0 !important; bottom: 0 !important; width: 5px !important; background: transparent !important; pointer-events: none !important; } \
                         .c_euv_sidebar_group_title::before { left: -9px !important; } \
                                      .c_euv_sidebar_link::before { left: -9px !important; } \
                         .c_euv_sidebar_link:hover::before, .c_euv_sidebar_group_title:hover::before { background: currentColor !important; } \
                         .c_euv_sidebar_link:hover, .c_euv_sidebar_group_title:hover { background: transparent !important; color: var(--foreground, #000) !important; border: 0 !important; box-shadow: none !important; } \
             .c_euv_sidebar_link:not(.c_euv_sidebar_link_active):not(.c_euv_sidebar_link_active_flush):hover::before, .c_euv_sidebar_group_title:not(.c_euv_sidebar_group_title_active):not(.c_euv_sidebar_group_title_root):not(.c_euv_sidebar_group_title_root_active):hover::before { background: currentColor !important; } \
                         .c_euv_sidebar_link_active, .c_euv_sidebar_group_title_active { background: var(--accent) !important; color: var(--text-on-accent) !important; box-shadow: none !important; } \
                         .c_euv_sidebar_link_active::before, .c_euv_sidebar_group_title_active::before { background: var(--accent, #000) !important; content: '' !important; position: absolute !important; width: 5px !important; top: 0 !important; bottom: 0 !important; pointer-events: none !important; left: -8px !important; } \
                         .c_euv_sidebar_group_title_active::before { left: -8px !important; } \
                         .c_euv_sidebar_link_active:hover, .c_euv_sidebar_group_title_active:hover { background: var(--accent) !important; color: var(--text-on-accent) !important; box-shadow: none !important; } \
                         .c_theme_dark .c_euv_sidebar_link:hover, .c_theme_dark .c_euv_sidebar_group_title:hover { background: transparent !important; color: var(--foreground, #fff) !important; box-shadow: none !important; } \
                         .c_theme_dark .c_euv_sidebar_link_active, .c_theme_dark .c_euv_sidebar_group_title_active { background: var(--accent) !important; color: var(--text-on-accent) !important; box-shadow: none !important; } \
                         .c_theme_dark .c_euv_sidebar_link_active::before, .c_theme_dark .c_euv_sidebar_group_title_active::before { background: var(--accent, #fff) !important; } \
                         .c_theme_dark .c_euv_sidebar_link_active:hover, .c_theme_dark .c_euv_sidebar_group_title_active:hover { background: var(--accent) !important; color: var(--text-on-accent) !important; box-shadow: none !important; } \
                         .c_euv_sidebar_group_title_root, .c_euv_sidebar_group_title_root_active { position: relative !important; } \
                         .c_euv_sidebar_group_title_root::before { content: '' !important; position: absolute !important; top: 0 !important; bottom: 0 !important; left: 0 !important; width: 4px !important; background: transparent !important; pointer-events: none !important; } \
                         .c_euv_sidebar_group_title_root:hover::before { background: var(--foreground, #000) !important; } \
                         .c_theme_dark .c_euv_sidebar_group_title_root:hover::before { background: var(--foreground, #fff) !important; } \
                         .c_euv_sidebar_group_title_root_active::before { background: var(--text-on-accent, #fff) !important; } \
                         .c_euv_sidebar_group_title_root:hover { background: var(--muted, #f4f4f5) !important; color: var(--foreground, #000) !important; } \
                         .c_euv_sidebar_group_title_root_active, .c_euv_sidebar_group_title_root_active:hover { background: var(--accent) !important; color: var(--text-on-accent) !important; } \
                         .c_theme_dark .c_euv_sidebar_group_title_root:hover { background: var(--muted, #27272a) !important; color: var(--foreground, #fff) !important; } \
                         .c_euv_sidebar_link_active_flush, .c_euv_sidebar_group_title_active { position: relative !important; margin-left: -9px !important; width: calc(100% + 9px) !important; padding-left: calc(12px + 9px) !important; } \
                         .c_euv_sidebar_link_active_flush::before, .c_euv_sidebar_group_title_active::before { left: 0 !important; } \
                         .c_euv_sidebar_link_active_flush:hover::before { background: var(--accent, #000) !important; } \
             \
             .c_euv_doc_layout { max-width: 1160px !important; display: flex !important; flex-direction: row !important; width: 100% !important; flex-shrink: 0 !important; } \
             /* `min-height: 100%` fills the visible area of `c_app_main` so the tail can be pushed to the bottom; a viewport unit overshoots by the header. `space-between` puts the tail at the bottom when the article is short and lets it follow the article when the article is long. Not sticky, not fixed: the tail scrolls with the page. */ \
             .c_euv_doc_content { display: flex !important; flex-direction: column !important; flex: 1 !important; justify-content: space-between !important; } \
             .c_euv_doc_content article.md-body { display: block !important; flex: 0 0 auto !important; min-height: 0 !important; overflow: visible !important; } \
             .c_euv_doc_content article.md-body > div { display: block !important; min-height: 0 !important; } \
             .c_euv_doc_tail { display: block !important; flex: 0 0 auto !important; } \
                         .c_euv_doc_toc { width: 280px !important; flex-shrink: 0 !important; position: sticky !important; top: var(--padding-main-top, 24px) !important; align-self: flex-start !important; max-height: 100% !important; overflow-y: auto !important; } \
                         .c_euv_toc_link_nested { padding-left: 0.75rem !important; font-size: var(--font-sm, 0.875rem) !important; color: var(--muted-foreground, #555) !important; line-height: 1.5 !important; } \
                         .c_euv_toc_link, .c_euv_toc_link_nested { font-weight: 400 !important; } \
                         .c_euv_toc_link:hover, .c_euv_toc_link_nested:hover { color: var(--accent, #000) !important; font-weight: 400 !important; } \
                         .c_euv_toc_link_active, .c_euv_toc_link_nested_active { color: var(--accent, #000) !important; font-weight: 700 !important; } \
                         .c_euv_toc_link_active:hover, .c_euv_toc_link_nested_active:hover { color: var(--accent, #000) !important; font-weight: 700 !important; } \
                         .c_theme_dark .c_euv_toc_link:hover, .c_theme_dark .c_euv_toc_link_nested:hover { color: var(--accent, #fff) !important; font-weight: 400 !important; } \
                         .c_theme_dark .c_euv_toc_link_active, .c_theme_dark .c_euv_toc_link_nested_active, .c_theme_dark .c_euv_toc_link_active:hover, .c_theme_dark .c_euv_toc_link_nested_active:hover { color: var(--accent, #fff) !important; font-weight: 700 !important; } \
             \
             .c_euv_pagination { padding-bottom: var(--space-xl, 1.25rem) !important; gap: var(--gap-component, 1rem) !important; flex-wrap: nowrap !important; align-items: stretch !important; width: 100% !important; } \
             .c_euv_pagination_link { padding: var(--space-md, 0.75rem) !important; gap: var(--space-2xs, 0.25rem) !important; min-width: 0 !important; max-width: none !important; } \
             .c_euv_footer { padding: var(--space-md, 0.75rem) 0 !important; display: flex !important; align-items: center !important; justify-content: center !important; flex: 0 0 auto !important; } \
             \
             .c_docs_page_title { font-size: 2.25rem; font-weight: 800; letter-spacing: -0.02em; margin: 0 0 var(--space-lg, 1rem) 0; padding-top: 0; color: var(--foreground, #000); } \
             .md-body > :first-child { margin-top: 0 !important; } \
             .md-body h1:first-of-type, .md-body h2:first-of-type, .md-body h3:first-of-type, .md-body h4:first-of-type, .md-body h5:first-of-type, .md-body h6:first-of-type { margin-top: 0 !important; } \
             .md-body > div:first-child > :first-child { margin-top: 0 !important; } \
             .md-body > div:first-child > :first-child > * { margin-top: 0 !important; } \
             \
             .c_feature_card { border: 1px dashed var(--foreground, #000) !important; border-radius: 0 !important; padding: 1rem !important; background: transparent !important; } \
             .c_home_btn_secondary { background: transparent !important; color: #000 !important; border: 1.5px solid #000 !important; } \
             .c_home_btn_secondary:hover { background: rgba(0,0,0,0.06) !important; } \
             .c_theme_dark .c_home_btn_secondary { background: transparent !important; color: #fff !important; border-color: #fff !important; } \
             .c_theme_dark .c_home_btn_secondary:hover { background: rgba(255,255,255,0.10) !important; } \
             \
             .c_docs_feature_grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 1rem; margin: 1rem 0; } \
             @media (max-width: 767px) { .c_docs_feature_grid { grid-template-columns: minmax(0, 1fr); } } \
             .c_docs_feature_card { display: flex; flex-direction: column; gap: 0.4rem; padding: 1rem; border: 1px dashed var(--foreground, #000); border-radius: 0; background: transparent; text-decoration: none; color: inherit; transition: background 0.15s ease-out, border-color 0.15s ease-out; min-width: 0; } \
             .c_docs_feature_card:hover { background: var(--accent-muted, rgba(0,0,0,0.06)); border-color: var(--foreground, #000); } \
             .c_theme_dark .c_docs_feature_card:hover { background: var(--accent-muted, rgba(255,255,255,0.08)); } \
             .c_docs_feature_card_inner { display: flex; flex-direction: column; gap: 0.4rem; min-width: 0; } \
             .c_docs_feature_card_icon { font-size: 1.5rem; line-height: 1; flex-shrink: 0; } \
             .c_docs_feature_card_title { font-size: 1.125rem; font-weight: 600; overflow-wrap: anywhere; } \
             .c_docs_feature_card_details { font-size: 0.875rem; color: var(--muted-foreground, #555); overflow-wrap: anywhere; } \
             \
             .docs-container-tip, .docs-container-note, .docs-container-important, .docs-container-info { border: 1px dashed var(--foreground, #000); border-left-width: 4px; padding: 0.75rem 1rem; margin: 1rem 0; background: var(--accent-muted, rgba(0,0,0,0.04)); } \
             .docs-container-warning, .docs-container-caution { border: 1px solid var(--foreground, #000); border-left-width: 4px; padding: 0.75rem 1rem; margin: 1rem 0; background: var(--accent-muted, rgba(0,0,0,0.04)); } \
             .docs-container-danger { border: 1px solid var(--foreground, #000); border-left-width: 4px; padding: 0.75rem 1rem; margin: 1rem 0; background: rgba(0,0,0,0.06); } \
             .docs-container-title { font-weight: 600; margin: 0 0 0.25rem 0; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em; } \
             .docs-container-title:empty { display: none; } \
             \
             .md-body img:not([data-loaded]) { height: 0px !important; margin: 0px !important; visibility: hidden; } \
             .md-body img[data-loaded] { transition: opacity 0.2s ease-out; }";

/// Lazy-loaded images: marks each image as loaded once decoded and
/// observes the DOM so images added later are marked too, preventing the
/// reserved-height flash before `img` reports its intrinsic size.
pub(crate) const IMAGE_LOAD_WATCHER_JS: &str = "(function(){var p=function(i){if(i.dataset.loaded)return;var m=function(){i.dataset.loaded='1';};if(i.complete&&i.naturalWidth>0){m();}else{i.addEventListener('load',m);i.addEventListener('error',m);}};var o=new MutationObserver(function(ms){ms.forEach(function(d){d.addedNodes.forEach(function(n){if(n.tagName==='IMG'){p(n);}if(n.querySelectorAll){n.querySelectorAll('img').forEach(p);}});});});o.observe(document.body,{childList:true,subtree:true});document.querySelectorAll('img').forEach(p);}());";

/// Table-of-contents scroll spy: marks the `.c_euv_toc_link_active`
/// class on the entry for the section currently at the top of the scroll
/// container, or the entry matching the URL fragment when one is present.
///
/// Driven by a short `setInterval` poll of the scroll container's
/// `scrollTop` rather than by `scroll` events or `requestAnimationFrame`:
/// the app shell does not reliably propagate scroll events (a delegated
/// `scroll` listener on `document` can stay silent while the container
/// scrolls), and rAF is paused entirely in hidden tabs. A 100ms interval
/// reads one property per tick and early-outs when nothing changed, so it
/// is effectively free. `hashchange` and a `MutationObserver` remain as
/// immediate accelerators. Activation is idempotent — class lists are only
/// touched when the active entry actually changes, so the observer cannot
/// re-trigger itself. A tick in which the TOC is momentarily absent from
/// the DOM (mid re-render) leaves the previous state untouched instead of
/// clearing it, and a tick whose tracked node was replaced by a re-render
/// re-marks the live node. The URL fragment is percent-decoded before
/// comparison because `location.hash` is encoded while the `href`
/// fragments and heading ids are raw UTF-8.
pub(crate) const TOC_SCROLL_SPY_JS: &str = "(function(){var current=null;var lastST=-1;var lastHash='';var links=function(){return document.querySelectorAll('.c_euv_doc_toc a, .c_euv_toc a');};var frag=function(a){var href=a.getAttribute('href')||'';var i=href.lastIndexOf('#');return i>0?href.slice(i+1):'';};var activate=function(t){if(t===current)return;if(current){current.classList.remove('c_euv_toc_link_active','c_euv_toc_link_nested_active');}current=t||null;if(current){current.classList.add(current.classList.contains('c_euv_toc_link_nested')?'c_euv_toc_link_nested_active':'c_euv_toc_link_active');}};var hashTarget=function(){var h=window.location.hash;var s=h.indexOf('#/');if(s<0)return null;var i=h.indexOf('#',s+2);if(i<0)return null;var anchor='';try{anchor=decodeURIComponent(h.slice(i+1));}catch(e){anchor=h.slice(i+1);}if(!anchor)return null;var best=null;var bestLen=-1;links().forEach(function(a){var f=frag(a);if(!f)return;if(f===anchor){best=a;bestLen=f.length;}else if(anchor.indexOf(f)===0&&f.length>bestLen){bestLen=f.length;best=a;}});return best;};var scrollTarget=function(c){var line=c.getBoundingClientRect().top+100;var best=null;var bestTop=-Infinity;var first=null;var last=null;links().forEach(function(a){var f=frag(a);if(!f)return;if(!first)first=a;last=a;var h=document.getElementById(f);if(!h)return;var top=h.getBoundingClientRect().top;if(top<=line&&top>bestTop){bestTop=top;best=a;}});if(c.scrollTop+c.clientHeight>=c.scrollHeight-4&&last)return last;return best||first;};var apply=function(){var t=hashTarget();if(t){activate(t);return;}if(!links().length)return;var c=document.querySelector('.c_app_main');if(!c)return;activate(scrollTarget(c));};var tick=function(){var c=document.querySelector('.c_app_main');var st=c?c.scrollTop:-1;var h=window.location.hash;var alive=current!==null&&document.contains(current);if(st!==lastST||h!==lastHash||!alive){lastST=st;lastHash=h;if(!alive)current=null;apply();}};apply();window.addEventListener('hashchange',apply);new MutationObserver(apply).observe(document.body,{childList:true,subtree:true});setInterval(tick,100);}());";

/// The `class` of a documentation page's `<h1>` title.
pub(crate) const CLASS_DOCS_PAGE_TITLE: &str = "c_docs_page_title";

/// The `class` of the home-page feature-card grid.
pub(crate) const CLASS_FEATURE_GRID: &str = "c_docs_feature_grid";

/// The `class` of a home-page feature card (the tile and its anchor form).
pub(crate) const CLASS_FEATURE_CARD: &str = "c_docs_feature_card";

/// The `class` of the column holding a feature card's icon / title / details.
pub(crate) const CLASS_FEATURE_CARD_INNER: &str = "c_docs_feature_card_inner";

/// The `class` of a feature card's leading icon glyph.
pub(crate) const CLASS_FEATURE_CARD_ICON: &str = "c_docs_feature_card_icon";

/// The `class` of a feature card's bold title line.
pub(crate) const CLASS_FEATURE_CARD_TITLE: &str = "c_docs_feature_card_title";

/// The `class` of a feature card's muted detail line.
pub(crate) const CLASS_FEATURE_CARD_DETAILS: &str = "c_docs_feature_card_details";

/// The placeholder icon value that means "this feature has no icon", as
/// opposed to a real emoji. Such cards hide the icon slot entirely so the
/// placeholder does not leak as visible text.
pub(crate) const FEATURE_ICON_PLACEHOLDER: &str = "blog";

/// The `target` that opens a link in a new browsing context.
pub(crate) const LINK_TARGET_BLANK: &str = "_blank";

/// The `rel` applied to every `target="_blank"` link, so the opened page
/// cannot reach back through `window.opener`.
pub(crate) const LINK_REL_NOOPENER: &str = "noopener noreferrer";

/// The `title` (tooltip) of the desktop and mobile theme-toggle buttons.
pub(crate) const THEME_TOGGLE_TITLE: &str = "切换主题";

/// Selector matching the scrollable main column of the desktop shell.
pub(crate) const MAIN_CONTAINER_SELECTOR_DESKTOP: &str = "[class*=c_app_main]";

/// Selector matching the scrollable main column of the mobile shell.
pub(crate) const MAIN_CONTAINER_SELECTOR_MOBILE: &str = "[class*=c_mobile_main]";

/// The `type` attribute of the password gate's input element.
pub(crate) const INPUT_TYPE_PASSWORD: &str = "password";

/// The `KeyboardEvent.key` value that submits the password form.
pub(crate) const KEY_ENTER: &str = "Enter";

/// The URL scheme prefix that marks a link as external (leaving the SPA),
/// tested with `starts_with` so both `http` and `https` match.
pub(crate) const URL_SCHEME_HTTP_PREFIX: &str = "http";

/// The root URL path, used as the site-root fallback when the location is
/// unavailable (non-browser unit tests).
pub(crate) const URL_PATH_ROOT: &str = "/";
