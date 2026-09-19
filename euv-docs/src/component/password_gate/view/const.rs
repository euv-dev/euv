use super::*;

class! {
    pub c_pw_gate_wrapper {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        min-height: "60vh";
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
