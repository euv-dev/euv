use super::*;

#[cfg(not(feature = "theme-custom"))]
class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Field — the parts of a group of form fields.
    // ═══════════════════════════════════════════════════════════════════════════

    // The set of fields that belong together. The reference stacks the
    // fields one under the other and gives the set a 24px gap, which is
    // the breathing room the reference gives a field group.




    // The legend of a set of fields.
    pub c_euv_field_legend {
        margin-bottom: var!(space-md);
        font-weight: var!(font-weight-medium);
    }

    // The legend of a set of fields, written at the size of a heading.
    pub c_euv_field_legend_legend {
        font-size: var!(font-base);
    }

    // The legend of a set of fields, written at the size of a caption.
    pub c_euv_field_legend_label {
        font-size: var!(font-sm);
    }

    // The group of fields. The reference stacks the rows of a group one
    // under the other and gives them a 28px gap.




    // A group whose rows are stacked one under the other.
    pub c_euv_field_group_vertical {
        flex-direction: "column";
    }

    // A group whose rows sit side by side.
    pub c_euv_field_group_horizontal {
        flex-direction: "row";
        align-items: "center";
    }

    // A group whose rows stack until the group is wide enough to hold
    // them side by side.
    pub c_euv_field_group_responsive {
        flex-direction: "column";
        @media ((min-width: 768px)) {
            flex-direction: "row";
            align-items: "center";
        }
    }

    // The body of one field, below its label.




    // The caption of the control of one field.




    // The caption of a field that carries a row of its own.




    // The explanation of one field.
    pub c_euv_field_description {
        margin: "0px";
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-normal);
        line-height: "1.5";
    }

    // The report of what is wrong with one field.
    pub c_euv_field_error {
        color: var!(destructive);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-normal);
    }

    // The rule that divides one part of a group of fields from the next.
    pub c_euv_field_separator {
        position: "relative";
        // The reference pulls the rule 8px into the rows above and
        // below it, so the rule sits between them instead of adding to
        // the space they already keep.
        margin-top: format!("calc(0px - {})", var!(space-sm));
        margin-bottom: format!("calc(0px - {})", var!(space-sm));
        height: "20px";
        font-size: var!(font-sm);
    }

    // The rule itself. It runs behind the text that sits in the middle
    // of it, so the text stays readable against the line.
    pub c_euv_field_separator_line {
        position: "absolute";
        top: "50%";
        left: "0px";
        right: "0px";
        border-top: format!("{} solid {}", var!(border-width), var!(border));
    }

    // The text that sits in the middle of the rule.
    pub c_euv_field_separator_content {
        position: "relative";
        display: "block";
        width: "fit-content";
        margin-left: "auto";
        margin-right: "auto";
        padding-left: var!(space-sm);
        padding-right: var!(space-sm);
        background: var!(background);
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Empty — the parts of an empty state.
    // ═══════════════════════════════════════════════════════════════════════════

    // The head of an empty state.
    pub c_euv_empty_header {
        display: "flex";
        max-width: "384px";
        flex-direction: "column";
        align-items: "center";
        gap: var!(space-sm);
        text-align: "center";
    }

    // The picture that marks an empty state.
    pub c_euv_empty_media {
        margin-bottom: var!(space-sm);
        display: "flex";
        flex-shrink: "0";
        align-items: "center";
        justify-content: "center";
    }

    // The picture that sits on the background as it is.
    pub c_euv_empty_media_default {
        flex-shrink: "0";
    }

    // The picture that sits in a rounded box.
    pub c_euv_empty_media_icon {
        width: "40px";
        height: "40px";
        background: var!(muted);
        color: var!(foreground);
        border-radius: var!(radius-lg);
    }

    // The headline of an empty state.
    pub c_euv_empty_title {
        font-size: var!(font-lg);
        font-weight: var!(font-weight-medium);
        letter-spacing: var!(letter-spacing-tight);
    }

    // The explanation of an empty state.
    pub c_euv_empty_description {
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        line-height: "1.625";
    }

    // The actions of an empty state.




    // ═══════════════════════════════════════════════════════════════════════════
    // Message — the parts of one message.
    // ═══════════════════════════════════════════════════════════════════════════

    // The cluster of messages.




    // The head of one message.




    // The picture of the sender of one message.
    pub c_euv_message_avatar {
        display: "flex";
        width: "fit-content";
        flex-shrink: "0";
        align-items: "center";
        justify-content: "center";
        align-self: "flex-end";
        overflow: "hidden";
        border-radius: var!(radius-full);
        background: var!(muted);
    }

    // The body of one message.




    // The trailing actions of one message.




    // The surface the words of one message sit on.
    pub c_euv_message_bubble {
        display: "flex";
        width: "fit-content";
        max-width: "100%";
        min-width: "0px";
        flex-direction: "column";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Kbd — the parts of a keyboard shortcut.
    // ═══════════════════════════════════════════════════════════════════════════

    // The group of keys of one shortcut.
    pub c_euv_kbd_group {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-xs);
    }
}

#[cfg(feature = "theme-custom")]
class! {
    pub c_euv_field_set {}

    pub c_euv_field_legend {}

    pub c_euv_field_legend_legend {}

    pub c_euv_field_legend_label {}

    pub c_euv_field_group {}

    pub c_euv_field_group_vertical {}

    pub c_euv_field_group_horizontal {}

    pub c_euv_field_group_responsive {}

    pub c_euv_field_content {}

    pub c_euv_field_label {}

    pub c_euv_field_title {}

    pub c_euv_field_description {}

    pub c_euv_field_error {}

    pub c_euv_field_separator {}

    pub c_euv_field_separator_line {}

    pub c_euv_field_separator_content {}

    pub c_euv_empty_header {}

    pub c_euv_empty_media {}

    pub c_euv_empty_media_default {}

    pub c_euv_empty_media_icon {}

    pub c_euv_empty_title {}

    pub c_euv_empty_description {}

    pub c_euv_empty_content {}

    pub c_euv_message_group {}

    pub c_euv_message_header {}

    pub c_euv_message_avatar {}

    pub c_euv_message_content {}

    pub c_euv_message_footer {}

    pub c_euv_message_bubble {}

    pub c_euv_kbd_group {}
}

#[cfg(all(not(feature = "theme-shadcn"), not(feature = "theme-custom")))]
class! {
    pub c_euv_field_set {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xl);
    }

    pub c_euv_field_group {
        display: "flex";
        width: "100%";
        flex-direction: "column";
        gap: format!("calc({} + {})", var!(space-2xl), var!(space-xs));
    }

    pub c_euv_field_content {
        display: "flex";
        flex: "1 1 0%";
        flex-direction: "column";
        // The reference sets a 6px gap between the description and the
        // control; 6px sits between the 4px and 8px steps of the scale.
        gap: var!(space-xs-sm);
        line-height: "1.375";
    }

    pub c_euv_field_label {
        display: "flex";
        width: "fit-content";
        gap: var!(space-sm);
        line-height: "1.375";
    }

    pub c_euv_field_title {
        display: "flex";
        width: "fit-content";
        align-items: "center";
        gap: var!(space-sm);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        line-height: "1.375";
    }

    pub c_euv_empty_content {
        display: "flex";
        width: "100%";
        min-width: "0px";
        max-width: "384px";
        flex-direction: "column";
        align-items: "center";
        gap: var!(space-lg);
        text-align: "center";
        font-size: var!(font-sm);
    }

    pub c_euv_message_group {
        display: "flex";
        min-width: "0px";
        flex-direction: "column";
    }

    pub c_euv_message_header {
        display: "flex";
        max-width: "100%";
        min-width: "0px";
        align-items: "center";
    }

    pub c_euv_message_content {
        display: "flex";
        width: "100%";
        min-width: "0px";
        flex-direction: "column";
        overflow-wrap: "break-word";
    }

    pub c_euv_message_footer {
        display: "flex";
        max-width: "100%";
        min-width: "0px";
        align-items: "center";
    }
}

#[cfg(all(feature = "theme-shadcn", not(feature = "theme-custom")))]
class! {
    pub c_euv_field_set {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xl);

        gap: var!(space-lg);
    }

    pub c_euv_field_group {
        display: "flex";
        width: "100%";
        flex-direction: "column";
        gap: format!("calc({} + {})", var!(space-2xl), var!(space-xs));

        gap: var!(space-xl);
    }

    pub c_euv_field_content {
        display: "flex";
        flex: "1 1 0%";
        flex-direction: "column";
        // The reference sets a 6px gap between the description and the
        // control; 6px sits between the 4px and 8px steps of the scale.
        gap: var!(space-xs-sm);
        line-height: "1.375";

        gap: var!(space-2xs);
    }

    pub c_euv_field_label {
        display: "flex";
        width: "fit-content";
        gap: var!(space-sm);
        line-height: "1.375";

        align-items: "center";
    }

    pub c_euv_field_title {
        display: "flex";
        width: "fit-content";
        align-items: "center";
        gap: var!(space-sm);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        line-height: "1.375";

        align-items: "center";
    }

    pub c_euv_empty_content {
        display: "flex";
        width: "100%";
        min-width: "0px";
        max-width: "384px";
        flex-direction: "column";
        align-items: "center";
        gap: var!(space-lg);
        text-align: "center";
        font-size: var!(font-sm);

        gap: var!(space-sm);
        justify-content: "center";
    }

    pub c_euv_message_group {
        display: "flex";
        min-width: "0px";
        flex-direction: "column";

        gap: var!(space-sm);
    }

    pub c_euv_message_header {
        display: "flex";
        max-width: "100%";
        min-width: "0px";
        align-items: "center";

        color: var!(muted-foreground);
        font-weight: var!(font-weight-medium);
        padding-left: var!(space-md);
    }

    pub c_euv_message_content {
        display: "flex";
        width: "100%";
        min-width: "0px";
        flex-direction: "column";
        overflow-wrap: "break-word";

        gap: var!(space-sm-md);
    }

    pub c_euv_message_footer {
        display: "flex";
        max-width: "100%";
        min-width: "0px";
        align-items: "center";

        color: var!(muted-foreground);
        font-weight: var!(font-weight-medium);
        padding-left: var!(space-md);
    }
}
