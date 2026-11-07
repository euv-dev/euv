use super::*;

class! {
    // The code block frame: one rounded box holding the header and the
    // code, so the header reads as the block's own toolbar.
    pub c_docs_code {
        margin: "16px 0px";
        border: format!("1px solid {}", var!(border));
        border-radius: var!(radius-lg);
        background: var!(muted);
        overflow: "hidden";
        pre {
            margin: "0px";
            padding: "16px";
            border: "none";
            border-radius: "0px";
            background: "transparent";
            color: var!(foreground);
            font-family: var!(font-family-mono);
            font-size: var!(font-xs);
            line-height: "1.6";
            overflow-x: "auto";
            code {
                display: "block";
                margin: "0px";
                padding: "0px";
                border: "none";
                background: "transparent";
                color: "inherit";
                font-family: "inherit";
                font-size: "inherit";
                line-height: "inherit";
                white-space: "pre";
            }
        }
    }

    // The toolbar above the code: the language on the left, the copy
    // button on the right.
    pub c_docs_code_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: "8px";
        padding: "4px 8px";
        border-bottom: format!("1px solid {}", var!(border));
        background: var!(muted);
        color: var!(muted-foreground);
        font-family: var!(font-family-mono);
        font-size: var!(font-xs);
        line-height: "1.5";
    }

    // The language label of the toolbar.
    pub c_docs_code_lang {
        font-weight: "500";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    // The copy button of the toolbar.
    pub c_docs_code_copy {
        display: "inline-flex";
        align-items: "center";
        flex-shrink: "0";
        gap: "4px";
        padding: "2px 8px";
        border: format!("1px solid {}", var!(border));
        border-radius: var!(radius-sm);
        background: var!(background);
        color: var!(foreground);
        font-family: "inherit";
        font-size: var!(font-xs);
        line-height: "1.5";
        cursor: "pointer";
        transition: format!("background {} {}, color {} {}", var!(duration-fast), var!(ease-out), var!(duration-fast), var!(ease-out));
        :hover {
            background: var!(accent);
            color: var!(accent-foreground);
        }
    }

    // The visible label of the copy button.
    pub c_docs_code_label {
        display: "inline";
    }

    // The label of the copy button while it is not on screen. The
    // button carries one label per outcome and swaps them by class, so
    // an outcome never has to rewrite the text of a node.
    pub c_docs_code_label_hidden {
        display: "none";
    }
}

/// The label of the copy button while the code has not been copied.
pub(crate) const COPY_LABEL: &str = "Copy";

/// The class of the label that is on screen.
pub(crate) const COPY_LABEL_CLASS: &str = "c_docs_code_label";

/// The class of the label that is off screen.
pub(crate) const HIDDEN_LABEL_CLASS: &str = "c_docs_code_label_hidden";

/// The selector that finds every label of the copy button.
pub(crate) const LABEL_SELECTOR: &str = ".c_docs_code_label, .c_docs_code_label_hidden";

/// The selector that finds the copy button.
pub(crate) const COPY_BUTTON_SELECTOR: &str = ".c_docs_code_copy";

/// The state of a button that is waiting for a press.
pub(crate) const RESTING_STATE: u8 = 0;

/// The state of a button that has just copied the code.
pub(crate) const COPY_STATE: u8 = 1;

/// The state of a button whose copy was refused by the browser.
pub(crate) const FAILED_STATE: u8 = 2;

/// How long the copy button keeps reporting the copy, in milliseconds.
///
/// The reference copy button shows its confirmation for about two
/// seconds before it returns to its resting label.
pub(crate) const COPY_FEEDBACK_MILLIS: u32 = 2000;

/// The label of the copy button right after a successful copy.
pub(crate) const COPIED_LABEL: &str = "Copied";

/// The label of the copy button when the browser refused the copy.
///
/// A write that is rejected must not look like a write that never
/// happened: the reader pressed the button, and the button has to say
/// what came of it.
pub(crate) const FAILED_LABEL: &str = "Failed";

/// The accessible name of the copy button.
pub(crate) const COPY_BUTTON_LABEL: &str = "Copy code to clipboard";
