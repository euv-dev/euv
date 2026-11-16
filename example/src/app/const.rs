//! String constants for the crate root (rust-standards §1.3c).
//!
//! `lib.rs` is the crate root, so this module is declared from it as
//! `mod r#const;` — the `r#` prefix is required because `const` is a Rust
//! keyword. Every string literal that the §1.3c verifier flags in a
//! function body lives here, verbatim and byte-identical to its original
//! inline form.

/// The CSS selector of the DOM element the example app mounts into.
///
/// Must match the container element declared by the host page's markup.
pub(crate) const APP_MOUNT_SELECTOR: &str = "#app";

/// Accessible name of the light/dark theme action in either navigation layout.
pub(crate) const APP_THEME_TOGGLE_LABEL: &str = "Toggle color theme";

/// Consumer-owned geometry for the headless custom build, without a component skin.
#[cfg(feature = "theme-custom")]
pub(crate) const APP_CUSTOM_LAYOUT_CSS: &str = r#"

#app .c_app_root { display: flex; height: 100%; min-height: 0; overflow: hidden; padding-bottom: var(--safe-area-inset-bottom, 0px); }
#app .c_mobile_app_root { display: flex; flex-direction: column; height: 100%; min-height: 0; overflow: hidden; padding-bottom: var(--safe-area-inset-bottom, 0px); }
#app .c_app_nav { display: flex; flex-direction: column; flex: 0 0 var(--nav-width, 248px); width: var(--nav-width, 248px); height: 100%; min-height: 0; overflow: hidden; }
#app .c_nav_items_scroll { flex: 1 1 0%; min-height: 0; overflow-y: auto; overflow-x: hidden; }
#app .c_nav_header, #app .c_nav_locale_row, #app .c_nav_section_label, #app .c_nav_theme_toggle, #app .c_nav_footer { flex-shrink: 0; }
#app .c_nav_header { display: flex; align-items: center; gap: var(--space-md, 12px); padding: var(--space-xl, 20px) var(--edge-gutter-nav, 16px); }
#app .c_nav_theme_toggle { display: flex; align-items: center; justify-content: space-between; gap: var(--space-sm, 8px); padding-inline: var(--edge-gutter-nav, 16px); }
#app .c_nav_theme_button { display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0; min-width: 44px; min-height: 44px; }
#app .c_theme_icon_sun, #app .c_theme_icon_moon { display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0; width: var(--space-xl, 20px); height: var(--space-xl, 20px); font-size: var(--space-xl, 20px); line-height: 1; }
#app .c_theme_icon_sun::before { content: "\2600\FE0E"; }
#app .c_theme_icon_moon::before { content: "\263E\FE0E"; }
#app .c_nav_item_active, #app .c_nav_item_inactive { display: flex; align-items: center; gap: var(--space-sm, 8px); padding: var(--space-sm, 8px) var(--edge-gutter-nav, 16px); }
#app .c_app_main, #app .c_mobile_main { flex: 1 1 0%; min-width: 0; min-height: 0; overflow-y: auto; overflow-x: hidden; padding: var(--padding-main-top, 24px) var(--padding-main-horizontal, 28px) var(--edge-gutter-bottom, 20px) var(--edge-gutter, 28px); }
#app .c_home { display: flex; flex-direction: column; min-height: 100%; min-width: 0; }
#app .c_home_content { display: flex; flex-direction: column; flex: 1 0 auto; width: 100%; max-width: var(--content-max-width, 820px); margin-inline: auto; min-width: 0; }
#app .c_home_body { flex: 1 0 auto; min-width: 0; }
#app .c_home_tail { flex: 0 0 auto; margin-top: auto; }
#app .c_home_actions { display: flex; flex-wrap: wrap; gap: var(--gap-element, 8px); }
#app .c_home_stats { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: var(--space-md, 12px); }
#app .c_home_feature_grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: var(--gap-component, 16px); }
@media (max-width: 767px) {
#app .c_home_stats { grid-template-columns: repeat(2, minmax(0, 1fr)); }
#app .c_home_feature_grid { grid-template-columns: minmax(0, 1fr); }
}
#app .c_page_router { display: flex; flex-direction: column; min-width: 0; }
#app .c_page_container { width: 100%; max-width: var(--content-max-width, 820px); margin: 0 auto; min-width: 0; }
#app .c_euv_card { display: flex; flex-direction: column; gap: var(--gap-component, 16px); margin-top: var(--page-block-gap, 24px); }
#app .c_euv_card_header, #app .c_euv_card_content, #app .c_euv_card_footer { min-width: 0; }
#app .c_button_controls { display: flex; flex-wrap: wrap; gap: var(--gap-element, 8px); margin-block: var(--gap-component, 16px); }
#app .c_mobile_header { display: flex; align-items: center; justify-content: space-between; flex-shrink: 0; height: calc(var(--mobile-header-height, 56px) + var(--euv-mobile-safe-top, 0px)); padding: var(--euv-mobile-safe-top, 0px) var(--edge-gutter-mobile, 16px) 0; position: sticky; top: 0; z-index: 100; }
#app .c_mobile_header_left, #app .c_mobile_header_logo { display: flex; align-items: center; min-width: 0; gap: var(--space-sm, 8px); }
#app .c_mobile_menu_button, #app .c_mobile_menu_button_active, #app .c_mobile_theme_button, #app .c_mobile_drawer_close_button { display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0; width: var(--mobile-header-height, 40px); height: var(--mobile-header-height, 40px); }
#app .c_mobile_theme_button { min-width: 44px; min-height: 44px; }
#app .c_mobile_overlay { position: fixed; inset: 0; z-index: 200; pointer-events: auto; }
#app .c_mobile_overlay_hidden { visibility: hidden; pointer-events: none; }
#app .c_mobile_nav_drawer { position: fixed; inset: 0 auto 0 0; width: var(--nav-width, 248px); max-width: 100%; display: flex; flex-direction: column; min-height: 0; overflow: hidden; z-index: 201; padding-top: var(--euv-mobile-safe-top, 0px); padding-bottom: var(--padding-shell-bottom, 0px); transform: translateX(0); visibility: visible; pointer-events: auto; }
#app .c_mobile_nav_drawer_closed { transform: translateX(-100%); visibility: hidden; pointer-events: none; }
#app .c_mobile_nav_drawer_header { display: flex; align-items: center; justify-content: space-between; flex-shrink: 0; height: var(--mobile-header-height, 56px); padding-inline: var(--edge-gutter-mobile, 16px); }
#app .c_euv_select_trigger { display: inline-flex; align-items: center; gap: var(--space-sm, 8px); max-width: 100%; }
#app .c_euv_select_caret, .c_euv_select_panel [data-slot="select-item-indicator"] { display: inline-flex; width: var(--space-lg, 16px); height: var(--space-lg, 16px); flex-shrink: 0; }
#app .c_euv_select_caret svg, .c_euv_select_panel svg { width: var(--space-lg, 16px); height: var(--space-lg, 16px); }
.c_euv_select_panel_open { display: flex; flex-direction: column; position: fixed; z-index: 150; max-height: 240px; overflow: hidden; }
.c_euv_select_panel_closed { display: none; pointer-events: none; }
.c_euv_select_viewport { min-height: 0; max-height: 240px; overflow-y: auto; overflow-x: hidden; }
#app .c_vconsole_fab { position: fixed; bottom: calc(var(--edge-gutter-bottom, 20px) + var(--safe-area-inset-bottom, 0px)); right: var(--edge-gutter, 28px); z-index: 9999; }
#app .c_euv_logo_fab { position: relative; display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0; min-width: 44px; min-height: 44px; }
#app .c_vconsole_overlay { position: fixed; inset: 0; z-index: 10000; visibility: visible; pointer-events: auto; }
#app .c_vconsole_overlay_hidden { visibility: hidden; pointer-events: none; }
#app .c_vconsole_panel { position: fixed; inset: auto 0 0; height: calc(76% - var(--safe-area-inset-bottom, 0px)); min-height: 0; display: flex; flex-direction: column; overflow: hidden; z-index: 10001; visibility: visible; pointer-events: auto; padding: var(--padding-main-top, 24px) var(--edge-gutter, 28px); }
#app .c_vconsole_panel_closed { visibility: hidden; pointer-events: none; }
#app .c_vconsole_header, #app .c_vconsole_header_actions, #app .c_vconsole_filter_bar { display: flex; align-items: center; gap: var(--space-sm, 8px); flex-shrink: 0; }
#app .c_vconsole_header { justify-content: space-between; }
#app .c_vconsole_filter_bar { flex-wrap: wrap; }
#app .c_vconsole_close_button { display: inline-flex; align-items: center; justify-content: center; min-width: 44px; min-height: 44px; }
#app .c_vconsole_body { flex: 1 1 0%; min-height: 0; overflow-y: auto; }
#app .c_vconsole_empty_hidden, #app .c_vconsole_log_list_hidden { display: none; }
@media (max-width: 767px) {
#app .c_vconsole_fab { bottom: calc(var(--edge-gutter-mobile, 16px) + var(--safe-area-inset-bottom, 0px)); right: var(--edge-gutter-mobile, 16px); }
#app .c_vconsole_panel { padding: var(--space-md, 12px) var(--edge-gutter-mobile, 16px); }
#app .c_app_nav { display: none; }
#app .c_mobile_main, #app .c_app_main { padding: var(--padding-main-top-mobile, 16px) var(--padding-main-horizontal-mobile, 16px) var(--edge-gutter-bottom, 20px) var(--edge-gutter-mobile, 16px); }
}
"#;
