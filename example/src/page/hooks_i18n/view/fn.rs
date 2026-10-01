use super::*;

/// A page demonstrating the i18n hook (handle + locale switching +
/// translation table).
///
/// # Arguments
///
/// - `VirtualNode<PageHooksI18nProps>` - The `node` argument.
/// # Returns
///
/// - `VirtualNode` - The value this function returns.
///
#[component]
pub(crate) fn page_hooks_i18n(node: VirtualNode<PageHooksI18nProps>) -> VirtualNode {
    let PageHooksI18nProps: PageHooksI18nProps = node.try_get_props().unwrap_or_default();
    let i18n: I18n = use_i18n(HOOKS_I18N_DEFAULT_LOCALE);
    // Subscribe to the locale signal so the active-state highlight
    // and the translation readout both re-render when the user
    // switches locales. `Signal<String>` is `Copy`, so we can copy
    // it directly from `get_locale()` without deref.
    let locale: Signal<String> = *i18n.get_locale();
    let locale_clone: String = String::from(HOOKS_I18N_DEFAULT_LOCALE);
    let en_entries_static: [(&'static str, &'static str); 2] = HOOKS_I18N_EN_MESSAGES;
    i18n_register(i18n, &locale_clone, &en_entries_static);
    let other_clone: String = String::from(HOOKS_I18N_OTHER_LOCALE);
    let zh_entries_static: [(&'static str, &'static str); 2] = HOOKS_I18N_ZH_MESSAGES;
    i18n_register(i18n, &other_clone, &zh_entries_static);
    let en_target: String = String::from(HOOKS_I18N_DEFAULT_LOCALE);
    let other_target: String = String::from(HOOKS_I18N_OTHER_LOCALE);
    let en_label: &'static str = HOOKS_I18N_LABEL_EN;
    let other_label: &'static str = HOOKS_I18N_LABEL_OTHER;
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🌐"
                title: "Hooks — i18n"
                subtitle: "Switch locales to see the same translation keys resolve to different messages. The handle's `locale` signal drives the reactive read."
            }
            euv_card {
                title: "Translation"
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: if { locale.get() == HOOKS_I18N_DEFAULT_LOCALE } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: en_label
                        onclick: hooks_i18n_switch(i18n, en_target.clone())
                    }
                    euv_button {
                        variant: if { locale.get() == HOOKS_I18N_OTHER_LOCALE } {
                            EuvButtonVariant::Primary
                        } else {
                            EuvButtonVariant::Outline
                        }
                        label: other_label
                        onclick: hooks_i18n_switch(i18n, other_target.clone())
                    }
                }
                p {
                    class: c_render_count_text()
                    "locale: "
                    span {
                        class: c_counter_value()
                        locale
                    }
                }
                p {
                    class: c_render_count_text()
                    "greeting: "
                    span {
                        class: c_counter_value()
                        hooks_i18n_translate(i18n, HOOKS_I18N_KEY_GREETING)
                    }
                }
                p {
                    class: c_render_count_text()
                    "farewell: "
                    span {
                        class: c_counter_value()
                        hooks_i18n_translate(i18n, HOOKS_I18N_KEY_FAREWELL)
                    }
                }
            }
        }
    }
}
