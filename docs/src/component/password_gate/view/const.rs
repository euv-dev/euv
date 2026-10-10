use super::*;

class! {
    pub c_pw_gate_wrapper {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        // 60% of the scroll container rather than 60% of the viewport: the
        // gate sits inside `c_app_main`, so a viewport unit would be taller
        // than the visible area and push the unlock form below the fold.
        min-height: "60%";
        padding: format!("{} {}", var!(space-4xl), var!(space-lg));
    }

    pub c_pw_gate_card {
        max-width: "28rem";
        width: "100%";
        padding: format!("{} {}", var!(space-3xl), var!(space-2xl));
        border: format!("1px dashed {}", var!(border));
        background: var!(background);
        color: var!(foreground);
        box-sizing: "border-box";
    }

    pub c_pw_gate_title {
        margin: format!("0px 0px {} 0px", var!(space-sm));
        font-size: var!(font-2xl);
        font-weight: "700";
        letter-spacing: "-0.02em";
        line-height: "1.3";
    }

    pub c_pw_gate_hint {
        margin: format!("0px 0px {} 0px", var!(space-xl));
        font-size: var!(font-sm);
        line-height: "1.5";
        color: var!(muted-foreground);
    }

    pub c_pw_gate_error {
        margin: format!("{} 0px 0px 0px", var!(space-sm));
        font-size: var!(font-sm);
        line-height: "1.4";
        font-weight: "500";
        color: var!(foreground);
    }

    pub c_pw_gate_actions {
        display: "flex";
        margin-top: var!(space-xl);
    }
}

/// localStorage key prefix used to record that a `route` has been
/// unlocked. The value itself is just a constant `"1"` sentinel — the
/// password digest lives only in `DocsPage::password_hash`.
///
/// Route is alphanumeric-only into the key so that slashes / locale
/// prefixes don't collide and so the key is plain ASCII (localStorage
/// keys must not contain newlines or other control chars).
pub(crate) const UNLOCK_KEY_PREFIX: &str = "euv-docs:unlocked:";

/// The `id` prefix of the password input element. The route is appended
/// so two gates on screen at once (a private page plus a private page in
/// the prefetched locale) keep distinct ids and distinct labels.
pub(crate) const INPUT_ID_PREFIX: &str = "pw-gate-";

/// The `autocomplete` value that keeps a password manager from
/// offering to fill or save the unlock field.
pub(crate) const INPUT_AUTOCOMPLETE_OFF: &str = "off";
