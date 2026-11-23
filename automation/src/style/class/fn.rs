use super::*;

class! {
    pub(crate) c_automation_root {
        width: "100%";
        height: "100vh";
        overflow: "auto";
        padding: var!(space-lg);
        box-sizing: "border-box";
        color: var!(foreground);
        background: var!(background);
    }

    pub(crate) c_automation_visible {
        display: "block";
    }

    pub(crate) c_automation_hidden {
        display: "none";
    }

    pub(crate) c_automation_scroller {
        height: format!("calc({} * 8)", var!(space-4xl));
        overflow: "auto";
        border: format!("1px solid {}", var!(border));
    }

    pub(crate) c_automation_content {
        min-height: format!("calc({} * 40)", var!(space-4xl));
    }

    pub(crate) c_automation_caret_group {
        width: "320px";
    }
}
