use super::*;

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




    // The page the reader is on.




    // The pages the reader can jump to.
    pub c_euv_pagination_page_inactive {
        color: var!(muted-foreground);
    }

    // The row of entries of a pagination footer.




    // One entry of a pagination footer.
    pub c_euv_pagination_item {
        list-style: "none";
        // Quoting preserves descendant whitespace through selector token reconstruction.
        :is(".c_euv_pagination_auto_content .c_euv_pagination_item") {
            display: "flex";
            min-width: "0px";
            align-items: "stretch";
        }
    }

    // The mark that stands for the pages between two reachable ones.




    // The mark that shows whether the answer is ticked. The round mark
    // belongs to the answer that takes one turn out of a list, the tick
    // to the one that can be ticked on its own.




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

#[cfg(not(feature = "theme-shadcn"))]
class! {
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

    pub c_euv_pagination_page_active {
        border-color: var!(foreground);
        color: var!(foreground);
    }

    pub c_euv_pagination_content {
        display: "flex";
        align-items: "center";
        gap: var!(space-2xs);
        list-style: "none";
        margin: "0px";
        padding: "0px";
    }

    pub c_euv_pagination_auto_content {
        display: "grid";
        grid-template-columns: "minmax(0px, 1fr) minmax(0px, 1fr)";
        align-items: "stretch";
        width: "100%";
        box-sizing: "border-box";
        list-style: "none";
        margin: "0px";
        padding: "0px";
        gap: var!(gap-component);
        // ~`space-2xl` (1.5rem) above the pagination — comfortable
        // breathing room without a huge gap before prev/next. The original
        // `space-4xl` (2.5rem) margin was doubled by a `padding-top` in the
        // docs app override, leaving ~100px of empty space above the
        // pagination boxes which made the page feel bottom-heavy.
        margin-top: var!(space-2xl);
        // `min-width: 0` lets prev/next text actually use ellipsis when the
        // container is narrow — without it the grid track blows out the row.
        min-width: "0px";
    }

    pub c_euv_pagination_ellipsis {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: var!(control-height);
        height: var!(control-height);
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    pub c_euv_questionnaire_choice_indicator_dot {
        display: "block";
        width: var!(space-2xs);
        height: var!(space-2xs);
        border-radius: var!(radius-full);
        background: var!(primary);
    }
}

#[cfg(feature = "theme-shadcn")]
class! {
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

        width: var!(control-height);
        height: var!(control-height);
        border-color: var!(card);
        border-radius: var!(radius-md);

        width: var!(control-height-sm);
        height: var!(control-height-sm);
        border-radius: var!(radius-lg);
    }

    pub c_euv_pagination_page_active {
        border-color: var!(foreground);
        color: var!(foreground);

        border-color: var!(border);
    }

    pub c_euv_pagination_content {
        display: "flex";
        align-items: "center";
        gap: var!(space-2xs);
        list-style: "none";
        margin: "0px";
        padding: "0px";

        gap: var!(space-xs);
    }

    pub c_euv_pagination_auto_content {
        display: "grid";
        grid-template-columns: "minmax(0px, 1fr) minmax(0px, 1fr)";
        align-items: "stretch";
        width: "100%";
        min-width: "0px";
        box-sizing: "border-box";
        list-style: "none";
        margin: "0px";
        padding: "0px";
        gap: var!(gap-component);
        margin-top: var!(space-2xl);
    }

    pub c_euv_pagination_ellipsis {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: var!(control-height);
        height: var!(control-height);
        font-size: var!(font-sm);
        color: var!(muted-foreground);

        width: var!(control-height-sm);
        height: var!(control-height-sm);
    }

    pub c_euv_questionnaire_choice_indicator_dot {
        display: "block";
        width: var!(space-2xs);
        height: var!(space-2xs);
        border-radius: var!(radius-full);
        background: var!(primary);

        background-color: var!(primary-foreground);
        border-bottom-right-radius: "9999px";
        border-bottom-left-radius: "9999px";
        border-top-right-radius: "9999px";
        border-top-left-radius: "9999px";
    }
}
