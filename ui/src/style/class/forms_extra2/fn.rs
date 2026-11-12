use super::*;

#[cfg(not(feature = "theme-custom"))]
class! {
    // A field wraps one control of a form. The name and the invalid state
    // live on the element so the label, the description and the message
    // inside it all read the same two facts.
    pub c_euv_form_field {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    // The box around one field of a form.
    pub c_euv_form_item {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    // One page of a page list. The current page is told apart by the
    // border it carries, the way the reference tells it apart by the
    // outline variant of its button.
    //
    // Page labels are non-wrapping numbers, centered in a fixed-size control.
    pub c_euv_pagination_page {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        line-height: "1";
        white-space: "nowrap";
        box-sizing: "border-box";
        width: var!(control-height);
        height: var!(control-height);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        font-size: var!(font-sm);
        text-decoration: "none";
        cursor: "pointer";
    }

    pub(crate) c_euv_pagination_page_shadcn {
        width: var!(control-height);
        height: var!(control-height);
        border-color: var!(card);
        border-radius: var!(radius-md);
    }

    // The page the reader is on.
    pub c_euv_pagination_page_active {
        border-color: var!(foreground);
        color: var!(foreground);
    }

    pub(crate) c_euv_pagination_page_active_shadcn {
        border-color: var!(border);
    }

    // The pages the reader can jump to.
    pub c_euv_pagination_page_inactive {
        color: var!(muted-foreground);
    }

    // The row of entries of a pagination footer.
    pub c_euv_pagination_content {
        display: "flex";
        align-items: "center";
        gap: var!(space-2xs);
        list-style: "none";
        margin: "0px";
        padding: "0px";
    }

    pub(crate) c_euv_pagination_content_shadcn {
        gap: var!(space-xs);
    }

    // One entry of a pagination footer.
    pub c_euv_pagination_item {
        list-style: "none";
    }

    // The mark that stands for the pages between two reachable ones.
    pub c_euv_pagination_ellipsis {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: var!(control-height);
        height: var!(control-height);
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    pub(crate) c_euv_pagination_ellipsis_shadcn {
        width: var!(control-height-sm);
        height: var!(control-height-sm);
    }

    // The mark that shows whether the answer is ticked. The round mark
    // belongs to the answer that takes one turn out of a list, the tick
    // to the one that can be ticked on its own.
    pub c_euv_questionnaire_choice_indicator_dot {
        display: "block";
        width: var!(space-2xs);
        height: var!(space-2xs);
        border-radius: var!(radius-full);
        background: var!(primary);
    }
    pub(crate) c_euv_questionnaire_choice_indicator_dot_shadcn {
        background-color: var!(primary-foreground);
        border-bottom-right-radius: "9999px";
        border-bottom-left-radius: "9999px";
        border-top-right-radius: "9999px";
        border-top-left-radius: "9999px";
    }


    // The tick that shows the answer is ticked.
    pub c_euv_questionnaire_choice_indicator_check {
        display: "block";
        width: var!(space-sm);
        height: var!(space-sm);
        border-radius: var!(radius-full);
    }

    // The control that takes the click on an answer. It lies over the
    // whole answer row, so the row reads as one thing to touch.
    pub c_euv_questionnaire_choice_input {
        position: "absolute";
        inset: "0px";
        width: "100%";
        height: "100%";
        cursor: "pointer";
        opacity: "0";
        z-index: "1";
    }


    // The keys that answer a question. They sit at the end of the answer
    // row and never take the click.
    pub c_euv_questionnaire_choice_shortcut {
        display: "inline-flex";
        align-items: "center";
        margin-left: "auto";
        padding-left: var!(space-sm);
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        pointer-events: "none";
    }

    // The box the free text answer sits in.
    pub c_euv_questionnaire_input_wrapper {
        position: "relative";
        min-width: "0px";
    }
}

#[cfg(feature = "theme-custom")]
class! {
    pub c_euv_form_field {}
    pub c_euv_form_item {}
    pub c_euv_pagination_page {}

    pub(crate) c_euv_pagination_page_shadcn {}
    pub c_euv_pagination_page_active {}
    pub(crate) c_euv_pagination_page_active_shadcn {}
    pub c_euv_pagination_page_inactive {}
    pub c_euv_pagination_content {}

    pub(crate) c_euv_pagination_content_shadcn {}

    pub c_euv_pagination_item {}
    pub c_euv_pagination_ellipsis {}
    pub(crate) c_euv_pagination_ellipsis_shadcn {}
    pub c_euv_questionnaire_choice_indicator_dot {}
    pub c_euv_questionnaire_choice_indicator_check {}
    pub c_euv_questionnaire_choice_input {}
    pub c_euv_questionnaire_choice_shortcut {}
    pub c_euv_questionnaire_input_wrapper {}
    pub(crate) c_euv_questionnaire_choice_indicator_dot_shadcn {}
}
