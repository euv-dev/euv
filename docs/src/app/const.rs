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

/// Leaves shadcn row appearance entirely to the selected shared UI classes.
#[cfg(feature = "theme-shadcn")]
pub(crate) const APP_SIDEBAR_CSS: &str = "";

/// Preserves the existing monochrome navigation treatment outside shadcn builds.
#[cfg(not(feature = "theme-shadcn"))]
pub(crate) const APP_SIDEBAR_CSS: &str = r#"
#app .c_euv_sidebar_children { margin-left: var(--space-md, 12px); padding-left: var(--space-sm, 8px); }
#app .c_euv_sidebar_children .c_euv_sidebar_link::before, #app .c_euv_sidebar_children .c_euv_sidebar_group_title::before { left: -9px; }
#app .c_euv_sidebar_children .c_euv_sidebar_link_active::before { left: -8px; }
#app .c_euv_sidebar_children .c_euv_sidebar_link_active_flush::before, #app .c_euv_sidebar_children .c_euv_sidebar_group_title_active::before { left: 0; }
#app .c_euv_sidebar_link, #app .c_euv_sidebar_group_title, #app .c_euv_sidebar_link_active { position: relative; }
#app .c_euv_sidebar_link::before, #app .c_euv_sidebar_group_title::before { content: ''; position: absolute; top: 0; bottom: 0; width: 5px; background: transparent; pointer-events: none; }
#app .c_euv_sidebar_group_title::before { left: 0; }
#app .c_euv_sidebar_link::before { left: 0; }
#app .c_euv_sidebar_link:hover::before, #app .c_euv_sidebar_group_title:hover::before { background: currentColor; }
#app .c_euv_sidebar_link:hover, #app .c_euv_sidebar_group_title:hover { background: transparent; color: var(--foreground); box-shadow: none; }
#app .c_euv_sidebar_link:not(.c_euv_sidebar_link_active):not(.c_euv_sidebar_link_active_flush):hover::before, #app .c_euv_sidebar_group_title:not(.c_euv_sidebar_group_title_active):not(.c_euv_sidebar_group_title_root):not(.c_euv_sidebar_group_title_root_active):hover::before { background: currentColor; }
#app .c_euv_sidebar_link_active, #app .c_euv_sidebar_group_title_active { background: var(--primary); color: var(--primary-foreground); box-shadow: none; }
#app .c_euv_sidebar_link_active::before, #app .c_euv_sidebar_group_title_active::before { background: var(--primary); content: ''; position: absolute; width: 5px; top: 0; bottom: 0; pointer-events: none; left: 0; }
#app .c_euv_sidebar_group_title_active::before { left: 0; }
#app .c_euv_sidebar_link_active:hover, #app .c_euv_sidebar_group_title_active:hover { background: var(--primary); color: var(--primary-foreground); box-shadow: none; }
#app .c_euv_sidebar_group_title_root, #app .c_euv_sidebar_group_title_root_active { position: relative; }
#app .c_euv_sidebar_group_title_root::before { content: ''; position: absolute; top: 0; bottom: 0; left: 0; width: 5px; background: transparent; pointer-events: none; }
#app .c_euv_sidebar_group_title_root:hover::before { background: var(--foreground); }
#app .c_euv_sidebar_group_title_root_active::before { background: var(--primary-foreground); }
#app .c_euv_sidebar_group_title_root:hover { background: transparent; color: var(--foreground); }
#app .c_euv_sidebar_group_title_root_active, #app .c_euv_sidebar_group_title_root_active:hover { background: var(--primary); color: var(--primary-foreground); }
#app .c_euv_sidebar_link_active_flush, #app .c_euv_sidebar_group_title_active { position: relative; margin-left: -9px; width: calc(100% + 9px); padding-left: calc(12px + 9px); }
#app .c_euv_sidebar_link_active_flush::before, #app .c_euv_sidebar_group_title_active::before { left: 0; }
#app .c_euv_sidebar_link_active_flush:hover { background: var(--primary); }
"#;

/// The injected stylesheet making the viewport-locked docs shell scroll
/// inside its main column instead of the window.
///
/// Lives in `lib.rs` because the rules target `c_app_main` /
/// `c_mobile_main` in *both* shells and several `c_euv_*` primitives
/// owned by the `euv-ui` crate, so there is no single view module that
/// owns it.
pub(crate) const APP_GLOBAL_CSS: &str = "html, body { height: 100%; overflow: hidden; } \
         #app { height: 100%; } \
         #app .c_app_main { padding-top: var(--padding-main-top, 24px); padding-bottom: var(--edge-gutter-bottom, 20px); display: flex; flex-direction: column; min-height: 0; overflow-y: auto; overflow-x: hidden; } \
         #app .c_mobile_main { display: flex; flex-direction: column; min-height: 0; overflow-y: auto; overflow-x: hidden; } \
         #app .c_euv_footer { flex-wrap: wrap; flex-shrink: 0; overflow: visible; } \
         #app .c_euv_footer_item_link { white-space: normal; overflow-wrap: anywhere; max-width: 100%; min-width: 0; } \
         #app .c_euv_footer_item_icon { max-width: 100%; object-fit: contain; } \
         #app .md-body h1, #app .md-body h2, #app .md-body h3, #app .md-body h4, #app .md-body h5, #app .md-body h6 { padding-left: 0; } \
         #app .md-body .header-anchor { display: none; } \
             #app .md-body img { display: inline-block; width: auto; max-width: 100%; height: auto; vertical-align: baseline; } \
             #app .md-body a > img { display: inline-block; } \
             #app .md-body img[src$='.svg'], #app .md-body img[src*='shields.io'], #app .md-body img[src*='github.com'] { max-height: 20px; max-width: 100%; } \
             #app .md-body table img { max-height: 1.4em; } \
             \
             #app .c_nav_footer_divider { left: 0.75rem; right: 0.75rem; } \
             /* The header is the reference documentation site's one full-width \
                sticky row: brand, top-level links, search field, theme toggle. \
                Its metrics follow the reference — a 56px row, 24px inline \
                padding and 32px controls; corner radii follow the active scheme. \
                It spans the viewport, so the shell stacks the header above the \
                sidebar-and-content row instead of letting the two columns sit \
                beside it. */ \
             #app .c_docs_header { position: sticky; top: 0; z-index: 100; height: 56px; display: flex; align-items: center; gap: 16px; padding: 0 24px; background: var(--background); border-bottom: 1px solid var(--border); flex-shrink: 0; } \
             #app .c_docs_header_brand { display: inline-flex; align-items: center; gap: 8px; font-size: 16px; font-weight: 500; color: var(--foreground); text-decoration: none; } \
             #app .c_docs_header_brand_title { white-space: nowrap; } \
             #app .c_docs_header_links { display: flex; align-items: center; gap: 2px; } \
             #app .c_docs_header_link, #app .c_docs_header_link_active { display: inline-flex; align-items: center; height: 32px; padding: 0 10px; border-radius: var(--radius-md, 0px); font-size: 14px; font-weight: 500; text-decoration: none; transition: background 150ms ease, color 150ms ease; } \
             #app .c_docs_header_link:hover { background: var(--accent); color: var(--accent-foreground); } \
             #app .c_docs_header_actions { margin-left: auto; display: flex; align-items: center; gap: 8px; } \
             #app .c_docs_header_search { position: relative; display: flex; align-items: center; height: 32px; width: 256px; padding: 0 12px; border-radius: var(--radius-md, 0px); background: var(--muted); border: 1px solid transparent; } \
             #app .c_docs_header_search:focus-within { box-shadow: 0 0 0 calc(var(--ring-width) + var(--ring-offset)) var(--ring); } \
             #app .c_docs_header_search_input { width: 100%; border: none; background: transparent; color: var(--foreground); font-size: 14px; font-family: inherit; outline: none; } \
             #app .c_docs_header_search_input::placeholder { color: var(--muted-foreground); } \
             #app .c_docs_header_separator { width: 1px; height: 16px; background: var(--border); } \
             #app .c_docs_header_icon_button { display: inline-flex; align-items: center; justify-content: center; width: 32px; height: 32px; border-radius: var(--radius-md, 0px); color: var(--foreground); cursor: pointer; } \
             #app .c_docs_header_icon_button:hover { background: var(--accent); color: var(--accent-foreground); } \
             #app .c_docs_shell { flex-direction: column; } \
             #app .c_docs_body { display: flex; flex: 1; min-height: 0; width: 100%; } \
             #app .c_nav_section_label { padding-left: 0.75rem; } \
             #app .c_nav_footer { padding-left: 0.75rem; } \
             #app .c_euv_doc_layout { max-width: 1160px; display: flex; flex-direction: row; width: 100%; flex-shrink: 0; } \
             /* `min-height: 100%` fills the visible area of `c_app_main` so the tail can be pushed to the bottom; a viewport unit overshoots by the header. `space-between` puts the tail at the bottom when the article is short and lets it follow the article when the article is long. Not sticky, not fixed: the tail scrolls with the page. */ \
             #app .c_euv_doc_content { display: flex; flex-direction: column; flex: 1; justify-content: space-between; } \
             #app .c_euv_doc_content article.md-body { display: block; flex: 0 0 auto; min-height: 0; overflow: visible; } \
             #app .c_euv_doc_content article.md-body > div { display: block; min-height: 0; } \
             #app .c_euv_doc_tail { display: block; flex: 0 0 auto; } \
                         #app .c_euv_doc_toc { width: 280px; min-width: 0; box-sizing: border-box; flex-shrink: 0; position: sticky; top: var(--padding-main-top, 24px); align-self: flex-start; max-height: 100%; overflow-y: auto; overflow-x: hidden; } \
                         #app .c_euv_doc_toc .c_euv_toc { width: 100%; min-width: 0; box-sizing: border-box; } \
                         #app .c_euv_toc_link_nested { padding-left: 0.75rem; font-size: var(--font-sm, 0.875rem); color: var(--muted-foreground); line-height: 1.5; } \
                         #app .c_euv_doc_toc .c_euv_toc_link, #app .c_euv_doc_toc .c_euv_toc_link_active, #app .c_euv_doc_toc .c_euv_toc_link_nested, #app .c_euv_doc_toc .c_euv_toc_link_nested_active { display: block; width: 100%; min-width: 0; max-width: 100%; box-sizing: border-box; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; } \
                         #app .c_euv_toc_link:hover, #app .c_euv_toc_link_nested:hover { color: var(--primary); } \
                         #app .c_euv_toc_link_active, #app .c_euv_toc_link_nested_active { color: var(--primary); font-weight: 700; } \
                         #app .c_euv_toc_link_active:hover, #app .c_euv_toc_link_nested_active:hover { color: var(--primary); } \
             \
             #app .c_euv_pagination { padding-bottom: var(--space-xl, 1.25rem); gap: var(--gap-component, 1rem); flex-wrap: nowrap; align-items: stretch; width: 100%; } \
             #app .c_euv_pagination_content .c_euv_pagination_link { padding: var(--space-md, 0.75rem); gap: var(--space-2xs, 0.25rem); min-width: 0; max-width: none; } \
             \
             /* The page head carries the title and the muted description \
                line under it. The head owns the gap to the body, so a page \
                without a description keeps the same rhythm as before. */ \
             #app .c_docs_page_head { margin: 0 0 var(--space-lg, 1rem) 0; } \
             #app .c_docs_page_title { font-size: 1.875rem; font-weight: 600; letter-spacing: -0.025em; margin: 0; padding-top: 0; color: var(--foreground); } \
             #app .c_docs_page_description { margin: var(--space-xs, 0.25rem) 0 0 0; font-size: var(--font-base, 1rem); line-height: 1.6; color: var(--muted-foreground); } \
             #app .md-body > :first-child { margin-top: 0; } \
             #app .md-body h1:first-of-type, #app .md-body h2:first-of-type, #app .md-body h3:first-of-type, #app .md-body h4:first-of-type, #app .md-body h5:first-of-type, #app .md-body h6:first-of-type { margin-top: 0; } \
             #app .md-body > div:first-child > :first-child { margin-top: 0; } \
             #app .md-body > div:first-child > :first-child > * { margin-top: 0; } \
             \
             #app .c_feature_card { border: 1px dashed var(--foreground); border-radius: 0; padding: 1rem; background: transparent; } \
             #app .c_home_btn_secondary { background: transparent; color: var(--foreground); border: 1px solid var(--border); } \
             #app .c_home_btn_secondary:hover { background: var(--accent); color: var(--accent-foreground); } \
             \
             #app .c_docs_feature_grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 1rem; margin: 1rem 0; } \
             @media (max-width: 767px) { #app .c_docs_feature_grid { grid-template-columns: minmax(0, 1fr); } } \
             #app .c_docs_feature_card { display: flex; flex-direction: column; gap: 0.4rem; padding: 1rem; border: 1px dashed var(--foreground); border-radius: 0; background: transparent; text-decoration: none; color: inherit; transition: background 0.15s ease-out, border-color 0.15s ease-out; min-width: 0; } \
             #app .c_docs_feature_card:hover { background: var(--muted); border-color: var(--foreground); } \
             #app .c_docs_feature_card_inner { display: flex; flex-direction: column; gap: 0.4rem; min-width: 0; } \
             #app .c_docs_feature_card_icon { font-size: 1.5rem; line-height: 1; flex-shrink: 0; } \
             #app .c_docs_feature_card_title { font-size: 1.125rem; font-weight: 600; overflow-wrap: anywhere; } \
             #app .c_docs_feature_card_details { font-size: 0.875rem; color: var(--muted-foreground); overflow-wrap: anywhere; } \
             \
             #app .docs-container-tip, #app .docs-container-note, #app .docs-container-important, #app .docs-container-info { border: 1px dashed var(--foreground); border-left-width: 4px; padding: 0.75rem 1rem; margin: 1rem 0; background: var(--muted); } \
             #app .docs-container-warning, #app .docs-container-caution { border: 1px solid var(--foreground); border-left-width: 4px; padding: 0.75rem 1rem; margin: 1rem 0; background: var(--muted); } \
             #app .docs-container-danger { border: 1px solid var(--foreground); border-left-width: 4px; padding: 0.75rem 1rem; margin: 1rem 0; background: var(--muted); } \
             #app .docs-container-title { font-weight: 600; margin: 0 0 0.25rem 0; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em; } \
             #app .docs-container-title:empty { display: none; } \
             \
             #app .md-body img:not([data-loaded]) { height: 0px; margin: 0px; visibility: hidden; } \
             #app .md-body img[data-loaded] { transition: opacity 0.2s ease-out; }";

/// The `class` of a documentation page's `<h1>` title.
pub(crate) const CLASS_DOCS_PAGE_TITLE: &str = "c_docs_page_title";

/// The `class` of the block holding a documentation page's title and
/// its description line.
pub(crate) const CLASS_DOCS_PAGE_HEAD: &str = "c_docs_page_head";

/// The `class` of a documentation page's muted description line.
pub(crate) const CLASS_DOCS_PAGE_DESCRIPTION: &str = "c_docs_page_description";

/// The `class` of the site header row.
pub(crate) const CLASS_DOCS_HEADER: &str = "c_docs_header";

/// The `class` of the header's brand block (logo plus site title).
pub(crate) const CLASS_DOCS_HEADER_BRAND: &str = "c_docs_header_brand";

/// The `class` of the brand block's title line.
pub(crate) const CLASS_DOCS_HEADER_BRAND_TITLE: &str = "c_docs_header_brand_title";

/// The `class` of the header's top-level navigation list.
pub(crate) const CLASS_DOCS_HEADER_LINKS: &str = "c_docs_header_links";

/// The `class` of the header's trailing action row.
pub(crate) const CLASS_DOCS_HEADER_ACTIONS: &str = "c_docs_header_actions";

/// The `class` of the header's search field.
pub(crate) const CLASS_DOCS_HEADER_SEARCH: &str = "c_docs_header_search";

/// The `class` of the search field's input element.
pub(crate) const CLASS_DOCS_HEADER_SEARCH_INPUT: &str = "c_docs_header_search_input";

/// The `class` of the vertical rule between the header's action groups.
pub(crate) const CLASS_DOCS_HEADER_SEPARATOR: &str = "c_docs_header_separator";

/// The `class` of the header's icon button (the theme toggle).
pub(crate) const CLASS_DOCS_HEADER_ICON_BUTTON: &str = "c_docs_header_icon_button";

/// The `class` of the shell that stacks the header above the content row.
pub(crate) const CLASS_DOCS_SHELL: &str = "c_docs_shell";

/// The `class` of the row holding the sidebar and the content column.
pub(crate) const CLASS_DOCS_BODY: &str = "c_docs_body";

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
