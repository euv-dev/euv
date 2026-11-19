use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Collapsible
    // ═══════════════════════════════════════════════════════════════════════════

    // A collapsible is a section that can be folded away. The trigger
    // is a plain button that reports the state of the section, and the
    // content below it is hidden while the section is closed.








    pub c_euv_collapsible_content {
        overflow: "hidden";
    }

    pub c_euv_collapsible_content_open {
        display: "block";
    }

    // The closed body is taken out of the flow, so the content it holds
    // is not read out and does not take room on the page.
    pub c_euv_collapsible_content_closed {
        display: "none";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Direction
    // ═══════════════════════════════════════════════════════════════════════════

    // The direction provider is a context in the reference, so it adds no
    // box of its own. `display: contents` keeps the wrapper out of the
    // layout while the `dir` attribute still inherits into every child.
    pub c_euv_direction {
        display: "contents";
    }

    pub c_euv_direction_ltr {
        direction: "ltr";
    }

    pub c_euv_direction_rtl {
        direction: "rtl";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Marker
    // ═══════════════════════════════════════════════════════════════════════════

    // A marker separates one block of content from the next. It is a row
    // that holds an optional icon and the text beside it.




    pub c_euv_marker_default {
        min-width: "0px";
    }

    // The separator variant is a centered rule: it takes no share of the
    // row and its text sits on the middle of the line.
    pub c_euv_marker_separator {
        flex: "none";
        text-align: "center";
    }

    // The border variant draws the line above the row instead of through
    // it, so the content still reads as one block.
    pub c_euv_marker_border {
        padding-top: var!(space-xs);
        padding-bottom: var!(space-xs);
        border-top: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub c_euv_marker_icon {
        display: "inline-flex";
        flex-shrink: "0";
        align-items: "center";
        justify-content: "center";
    }

    pub c_euv_marker_content {
        min-width: "0px";
        overflow-wrap: "break-word";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Native Select
    // ═══════════════════════════════════════════════════════════════════════════

    // The native select keeps the platform control and only dresses it: a
    // wrapper holds the control and the chevron that stands in for the
    // browser's own arrow.
    pub c_euv_native_select_wrapper {
        position: "relative";
        display: "inline-flex";
        width: "fit-content";
    }

    // The reference dims the control and its overlaid chevron together.
    pub(crate) c_euv_native_select_wrapper_disabled {
        opacity: "0.5";
    }










    // The native control reports its validation state through aria-invalid.
    pub(crate) c_euv_native_select_invalid {
        border-color: var!(destructive);
        :focus-visible {
            border-color: var!(destructive);
        }
    }





    // The chevron overlays the control without intercepting its input.
    pub c_euv_native_select_icon {
        position: "absolute";
        right: format!("calc({} + {})", var!(space-md), var!(space-2xs));
        top: "50%";
        display: "block";
        width: var!(space-lg);
        height: var!(space-lg);
        pointer-events: "none";
        color: var!(muted-foreground);
        opacity: "0.5";
        transform: "translateY(-50%)";
    }

    pub c_euv_native_select_option {
        background: var!(background);
        color: var!(foreground);
    }

    pub c_euv_native_select_optgroup {
        background: var!(background);
        color: var!(foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Attachment
    // ═══════════════════════════════════════════════════════════════════════════

    // An attachment is a chip that carries one uploaded file: a thumbnail,
    // the file's name and description, and the actions that go with it.




    pub c_euv_attachment_horizontal {
        flex-direction: "row";
    }

    pub c_euv_attachment_vertical {
        flex-direction: "column";
    }

    // An idle attachment has not been filled yet, so its outline is dashed
    // to say that the slot is waiting for a file.
    pub c_euv_attachment_state_idle {
        border-style: "dashed";
    }

    // A failed attachment says so in the destructive colour, which is the
    // only colour that reads as an error in this system.
    pub c_euv_attachment_state_error {
        border-color: var!(destructive);
        color: var!(destructive);
    }

    pub c_euv_attachment_state_uploading {
        opacity: "0.8";
    }

    pub c_euv_attachment_state_processing {
        opacity: "0.8";
    }

    pub c_euv_attachment_state_done {
        opacity: "1";
    }

    // The media is the square thumbnail that leads the chip.




    pub c_euv_attachment_media_size_default {
        width: var!(space-2xl);
        height: var!(space-2xl);
    }

    pub c_euv_attachment_media_size_sm {
        width: var!(space-2xl);
        height: var!(space-2xl);
    }

    // The extra small thumbnail is twenty-eight pixels wide, one step
    // under the small one.
    pub c_euv_attachment_media_size_xs {
        width: "28px";
        height: "28px";
    }

    pub c_euv_attachment_content {
        max-width: "100%";
        min-width: "0px";
        flex: "1";
    }

    pub c_euv_attachment_title {
        display: "block";
        max-width: "100%";
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        font-weight: var!(font-weight-medium);
    }

    pub c_euv_attachment_description {
        display: "block";
        max-width: "100%";
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        color: var!(muted-foreground);
        font-size: var!(font-xs);
    }

    pub c_euv_attachment_actions {
        display: "flex";
        flex-shrink: "0";
        align-items: "center";
        gap: var!(space-2xs);
    }

    pub c_euv_attachment_action {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
    }

    // The trigger covers the whole chip so that a click anywhere on it
    // opens the file, while the actions above it stay reachable.
    pub c_euv_attachment_trigger {
        position: "absolute";
        inset: "0px";
        outline: "none";
    }

    // The group scrolls sideways and snaps each attachment to its start.




    // ═══════════════════════════════════════════════════════════════════════════
    // Chart
    // ═══════════════════════════════════════════════════════════════════════════

    // The chart container is the frame the plot is drawn in. Its own text
    // is small because every label inside a chart is a caption.
    pub c_euv_chart {
        display: "flex";
        width: "100%";
        aspect-ratio: "16 / 9";
        justify-content: "center";
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The tooltip is a small floating panel that lists one row per series.
    pub c_euv_chart_tooltip {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        min-width: "120px";
        padding: var!(space-sm);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
        font-size: var!(font-xs);
    }

    pub c_euv_chart_tooltip_label {
        font-weight: var!(font-weight-medium);
    }

    pub c_euv_chart_tooltip_item {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
    }

    // The dot that stands for a series in the tooltip and the legend.
    pub c_euv_chart_indicator {
        display: "inline-flex";
        flex-shrink: "0";
        width: var!(space-sm);
        height: var!(space-sm);
        border-radius: var!(radius-full);
        background: var!(primary);
    }

    pub c_euv_chart_legend {
        display: "flex";
        flex-wrap: "wrap";
        align-items: "center";
        justify-content: "center";
        gap: var!(space-md);
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    pub c_euv_chart_legend_item {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-xs);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Bubble
    // ═══════════════════════════════════════════════════════════════════════════

    // A bubble is one message of a conversation. The group keeps the run
    // of bubbles in one column, and the bubble itself decides the
    // surface it is drawn on and the side of the row it sits on.








    pub c_euv_bubble_align_start {
        align-self: "flex-start";
    }

    pub c_euv_bubble_align_end {
        align-self: "flex-end";
    }

    pub c_euv_bubble_default {
        background: var!(background);
        color: var!(foreground);
    }

    pub c_euv_bubble_secondary {
        background: var!(secondary);
        color: var!(secondary-foreground);
    }

    pub c_euv_bubble_muted {
        background: var!(muted);
        color: var!(foreground);
    }

    pub c_euv_bubble_tinted {
        background: var!(primary);
        color: var!(primary-foreground);
    }

    pub c_euv_bubble_outline {
        background: "transparent";
        border: format!("{} solid {}", var!(border-width), var!(border));
        color: var!(foreground);
    }

    pub c_euv_bubble_ghost {
        background: "transparent";
        color: var!(foreground);
    }

    pub c_euv_bubble_destructive {
        background: var!(destructive);
        color: var!(destructive-foreground);
    }

    // The text of a bubble wraps rather than pushing the bubble wider
    // than the row it sits in.




    // The reactions float beside the bubble.




    pub c_euv_bubble_reactions_top {
        bottom: "100%";
    }

    pub c_euv_bubble_reactions_bottom {
        top: "100%";
    }

    pub c_euv_bubble_reactions_start {
        left: "0px";
    }

    pub c_euv_bubble_reactions_end {
        right: "0px";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Message Scroller
    // ═══════════════════════════════════════════════════════════════════════════

    // The scroller is the frame the list of messages is drawn in. The
    // viewport inside it keeps the scroll to itself, so a page that is
    // scrolled past the list does not drag the list with it.
    pub c_euv_message_scroller {
        position: "relative";
        display: "flex";
        width: "100%";
        height: "100%";
        min-height: "0px";
        flex-direction: "column";
        overflow: "hidden";
    }

    pub c_euv_message_scroller_viewport {
        width: "100%";
        height: "100%";
        min-height: "0px";
        min-width: "0px";
        overflow-y: "auto";
        overscroll-behavior: "contain";
    }





    // The item never shrinks, so a long message keeps its own height
    // instead of being squeezed into the row.
    pub c_euv_message_scroller_item {
        min-width: "0px";
        flex-shrink: "0";
    }

    // The jump button floats over the list.




    pub c_euv_message_scroller_button_start {
        top: var!(space-lg);
    }

    pub c_euv_message_scroller_button_end {
        bottom: var!(space-lg);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Trace
    // ═══════════════════════════════════════════════════════════════════════════

    // A trace is a column of steps. The line describes one step and the
    // collapsible inside it holds the long form output of that step.








    pub c_euv_trace_line_idle {
        color: var!(muted-foreground);
    }

    pub c_euv_trace_line_running {
        color: var!(muted-foreground);
    }

    // A step that failed is drawn in the destructive colour, which is the
    // only colour that reads as an error in this system.
    pub c_euv_trace_line_error {
        color: var!(destructive);
    }

    // A step that is still waiting is dimmed, so it does not read as
    // finished.
    pub c_euv_trace_line_pending {
        opacity: "0.6";
    }

    pub c_euv_trace_icon {
        display: "flex";
        flex-shrink: "0";
        align-items: "center";
        justify-content: "center";
    }

    pub c_euv_trace_title {
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    // The detail is the number beside the name of the step, so it is set
    // in figures that line up in a column.
    pub c_euv_trace_description {
        flex-shrink: "0";
        color: var!(muted-foreground);
        font-variant-numeric: "tabular-nums";
    }

    pub c_euv_trace_collapsible {
        display: "flex";
        width: "100%";
        min-width: "0px";
        flex-direction: "column";
    }

    pub c_euv_trace_collapsible_content {
        min-width: "0px";
        overflow: "hidden";
    }

    pub c_euv_trace_collapsible_content_open {
        display: "block";
    }

    // The closed output is taken out of the flow, so the long form
    // output of a step does not push the rest of the trace out of view.
    pub c_euv_trace_collapsible_content_closed {
        display: "none";
    }

    pub c_euv_trace_collapsible_content_inner {
        min-width: "0px";
        overflow-wrap: "break-word";
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Code Block
    // ═══════════════════════════════════════════════════════════════════════════

    // A code block is a column: the header names the snippet, the
    // content holds the code and the footer holds whatever the snippet
    // needs to say below it.




    pub c_euv_code_block_default {
        background: var!(muted);
    }

    pub c_euv_markdown_inline_code {
        border-radius: var!(radius-sm);
    }

    // A block that says the snippet does not run is drawn in the
    // destructive colour.
    pub c_euv_code_block_destructive {
        border-color: var!(destructive);
        color: var!(destructive);
    }

    pub c_euv_code_block_size_default {
        font-size: var!(font-sm);
    }

    pub c_euv_code_block_size_sm {
        font-size: var!(font-xs);
    }









    // The actions sit at the end of the header.








    // The code is read from the left even in a right to left page, so
    // the block is told to read from the left as well.




    pub c_euv_code_block_content_no_wrap {
        white-space: "pre";
    }

    pub c_euv_code_block_content_wrap {
        white-space: "pre-wrap";
        overflow-wrap: "break-word";
    }

    pub c_euv_code_block_code {
        display: "grid";
        min-width: "max-content";
    }





    // The text of one line. It carries the line number, so it has to be
    // able to shrink when the line is longer than the block is wide.
    pub c_euv_code_block_line_content {
        min-width: "0px";
    }

    pub c_euv_code_block_line_default {
        color: var!(foreground);
    }

    // A line that was added is drawn in the success colour and a line
    // that was removed in the destructive one, the way a diff reads.
    pub c_euv_code_block_line_added {
        color: var!(success);
    }

    pub c_euv_code_block_line_removed {
        color: var!(destructive);
    }

    pub c_euv_code_block_line_highlighted {
        background: var!(accent);
        color: var!(accent-foreground);
    }





    // ═══════════════════════════════════════════════════════════════════════════
    // Navigation Menu
    // ═══════════════════════════════════════════════════════════════════════════

    // A navigation menu is the frame the sections live in. A section
    // opens under its trigger, and the state of that section is carried
    // by the signal the caller owns.








    pub c_euv_navigation_menu_item {
        position: "relative";
    }

    pub c_euv_navigation_menu_trigger_open {
        background: var!(accent);
        color: var!(accent-foreground);
    }

    pub c_euv_navigation_menu_trigger_closed {
        background: "transparent";
    }

    // The panel is hidden while the section is closed, so the links it
    // holds are not read out before the reader asks for them.
    pub c_euv_navigation_menu_content {
        position: "absolute";
        left: "0px";
        top: "100%";
        min-width: var!(space-4xl);
        padding: var!(space-md);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(popover);
        color: var!(popover-foreground);
        box-shadow: var!(shadow-md);
    }

    pub c_euv_navigation_menu_content_open {
        display: "block";
    }

    pub c_euv_navigation_menu_content_closed {
        display: "none";
    }





    // The viewport is the frame the open panel is drawn in, so the panel
    // does not shift the rest of the menu while it is open.
    pub c_euv_navigation_menu_viewport {
        position: "relative";
        width: "100%";
    }

    pub c_euv_navigation_menu_indicator {
        position: "absolute";
        left: "0px";
        bottom: "0px";
        width: var!(space-md);
        height: "2px";
        border-radius: var!(radius-full);
        background: var!(primary);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Resizable
    // ═══════════════════════════════════════════════════════════════════════════

    // A resizable group lays its panels out in a row or in a column,
    // and the handles between them move the divider.
    pub c_euv_resizable_group {
        display: "flex";
        width: "100%";
        height: "100%";
    }

    pub c_euv_resizable_group_horizontal {
        flex-direction: "row";
    }

    pub c_euv_resizable_group_vertical {
        flex-direction: "column";
    }

    pub c_euv_resizable_panel {
        min-width: "0px";
        min-height: "0px";
        overflow: "hidden";
    }

    // The handle is the hairline the reader drags.
    pub c_euv_resizable_handle {
        position: "relative";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex-shrink: "0";
        background: var!(border);
        cursor: "col-resize";
    }

    pub c_euv_resizable_handle_horizontal {
        width: "1px";
    }

    pub c_euv_resizable_handle_vertical {
        height: "1px";
        cursor: "row-resize";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Questionnaire
    // ═══════════════════════════════════════════════════════════════════════════

    // A questionnaire is a column of questions. The progress above them
    // says how far along the reader is, and the actions below them move
    // from one question to the next.




    // The progress is a number, so it is set in figures that line up in
    // a column while the count changes.
    pub c_euv_questionnaire_progress {
        min-height: "1lh";
        width: "fit-content";
        min-width: "14ch";
        font-weight: var!(font-weight-medium);
        color: var!(muted-foreground);
        font-variant-numeric: "tabular-nums";
    }

    pub c_euv_questionnaire_description {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    pub c_euv_questionnaire_choices {
        display: "grid";
        min-width: "0px";
        gap: var!(space-sm);
    }

    // The choice is the row the reader presses to pick an answer.
















    pub c_euv_questionnaire_choice_description {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // The indicator is the mark that shows whether the answer is ticked.




    // The answer that takes one answer out of a list is marked with a
    // round mark, the one that can be ticked on its own with a square.
    pub c_euv_questionnaire_choice_indicator_radio {
        border-radius: var!(radius-full);
    }

    pub c_euv_questionnaire_choice_indicator_checkbox {
        border-radius: var!(radius-sm);
    }

    pub c_euv_questionnaire_choice_indicator_unchecked {
        background: var!(background);
    }

    pub c_euv_questionnaire_choice_indicator_checked {
        border-color: var!(primary);
        background: var!(primary);
        color: var!(primary-foreground);
    }

    pub c_euv_questionnaire_error {
        font-size: var!(font-sm);
        color: var!(destructive);
    }

    pub c_euv_questionnaire_actions {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
    }
}

#[cfg(not(feature = "theme-shadcn"))]
class! {
    pub c_euv_collapsible {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    pub c_euv_collapsible_trigger {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        text-align: "left";
        font-weight: var!(font-weight-medium);
        cursor: "pointer";
    }

    pub c_euv_marker {
        position: "relative";
        display: "flex";
        width: "100%";
        align-items: "center";
        gap: var!(space-sm);
    }

    pub c_euv_native_select {
        appearance: "none";
        outline: "none";
        width: "100%";
        min-width: "0px";
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        padding-left: var!(space-sm);
        padding-right: var!(space-2xl);
        font-size: var!(font-sm);
        cursor: "pointer";
        :disabled {
            pointer-events: "none";
            cursor: "not-allowed";
        }
    }

    pub c_euv_native_select_size_default {
        height: var!(control-height);
    }

    pub c_euv_native_select_size_sm {
        height: var!(control-height-sm);
    }

    pub c_euv_attachment {
        position: "relative";
        display: "flex";
        max-width: "100%";
        min-width: "0px";
        flex-shrink: "0";
        flex-wrap: "wrap";
        align-items: "center";
        gap: var!(space-sm);
        padding: var!(space-sm);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(card);
        color: var!(card-foreground);
        font-size: var!(font-sm);
    }

    pub c_euv_attachment_media {
        position: "relative";
        display: "flex";
        aspect-ratio: "1";
        flex-shrink: "0";
        align-items: "center";
        justify-content: "center";
        overflow: "hidden";
        border-radius: var!(radius-sm);
        background: var!(muted);
    }

    pub c_euv_attachment_media_variant_image {
        background: var!(muted);
    }

    pub c_euv_attachment_group {
        display: "flex";
        min-width: "0px";
        gap: var!(space-sm);
        overflow-x: "auto";
        overscroll-behavior-x: "contain";
        scroll-snap-type: "x mandatory";
    }

    pub c_euv_bubble_group {
        display: "flex";
        min-width: "0px";
        flex-direction: "column";
    }

    pub c_euv_bubble {
        position: "relative";
        display: "flex";
        width: "fit-content";
        min-width: "0px";
        flex-direction: "column";
    }

    pub c_euv_bubble_content {
        width: "fit-content";
        max-width: "100%";
        min-width: "0px";
        overflow: "hidden";
        overflow-wrap: "break-word";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border-radius: var!(radius-lg);
    }

    pub c_euv_bubble_reactions {
        position: "absolute";
        display: "flex";
        width: "fit-content";
        align-items: "center";
        justify-content: "center";
    }

    pub c_euv_message_scroller_content {
        display: "flex";
        min-height: "100%";
        flex-direction: "column";
    }

    pub c_euv_message_scroller_button {
        position: "absolute";
        left: "50%";
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        white-space: "nowrap";
        transform: "translateX(-50%)";
    }

    pub c_euv_trace {
        display: "flex";
        width: "100%";
        min-width: "0px";
        flex-direction: "column";
    }

    pub c_euv_trace_line {
        display: "flex";
        width: "fit-content";
        max-width: "100%";
        min-width: "0px";
        align-items: "center";
        gap: var!(space-xs);
        color: var!(muted-foreground);
    }

    pub c_euv_trace_collapsible_trigger {
        display: "flex";
        width: "fit-content";
        max-width: "100%";
        min-width: "0px";
        align-items: "center";
        text-align: "left";
        color: var!(muted-foreground);
        cursor: "pointer";
    }

    pub c_euv_code_block {
        position: "relative";
        display: "flex";
        width: "100%";
        min-width: "0px";
        flex-direction: "column";
        overflow: "hidden";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(muted);
        color: var!(foreground);
    }

    pub c_euv_code_block_header {
        display: "flex";
        min-width: "0px";
        flex-shrink: "0";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub c_euv_code_block_title {
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        font-weight: var!(font-weight-medium);
    }

    pub c_euv_code_block_actions {
        display: "flex";
        flex-shrink: "0";
        align-items: "center";
        gap: var!(space-2xs);
        margin-left: "auto";
    }

    pub c_euv_code_block_copy_button {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-sm);
        background: var!(background);
        color: var!(foreground);
        white-space: "nowrap";
        cursor: "pointer";
    }

    pub c_euv_code_block_content {
        min-height: "0px";
        min-width: "0px";
        overflow: "auto";
        text-align: "start";
        font-family: var!(font-family-mono);
        font-size: var!(font-sm);
        outline: "none";
    }

    pub c_euv_code_block_line {
        position: "relative";
        display: "block";
    }

    pub c_euv_code_block_footer {
        display: "flex";
        min-width: "0px";
        flex-shrink: "0";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border-top: format!("{} solid {}", var!(border-width), var!(border));
    }

    pub c_euv_navigation_menu {
        position: "relative";
        display: "flex";
        width: "100%";
        justify-content: "center";
    }

    pub c_euv_navigation_menu_list {
        display: "flex";
        flex-wrap: "wrap";
        align-items: "center";
        gap: var!(space-md);
        margin: "0px";
        padding: "0px";
        list-style: "none";
    }

    pub c_euv_navigation_menu_trigger {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-xs);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border: "none";
        background: "transparent";
        color: var!(foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        white-space: "nowrap";
        cursor: "pointer";
    }

    pub c_euv_navigation_menu_link {
        display: "block";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-radius: var!(radius-sm);
        color: var!(foreground);
        font-size: var!(font-sm);
        text-decoration: "none";
    }

    pub c_euv_questionnaire {
        display: "flex";
        width: "100%";
        min-width: "0px";
        flex-direction: "column";
        gap: var!(space-md);
    }

    pub c_euv_questionnaire_item {
        min-width: "0px";
        padding: "0px";
        border: "none";
        outline: "none";
    }

    pub c_euv_questionnaire_title {
        font-size: var!(font-lg);
        font-weight: var!(font-weight-semibold);
        line-height: "1.2";
    }

    pub c_euv_questionnaire_choice {
        position: "relative";
        display: "flex";
        min-height: var!(control-height);
        align-items: "flex-start";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        text-align: "start";
        cursor: "pointer";
    }

    pub c_euv_questionnaire_choice_enabled {
        cursor: "pointer";
    }

    pub c_euv_questionnaire_choice_disabled {
        pointer-events: "none";
        cursor: "not-allowed";
        opacity: "0.5";
    }

    pub c_euv_questionnaire_choice_label {
        display: "flex";
        min-width: "0px";
        flex: "1";
        flex-direction: "column";
        line-height: "1.3";
    }

    pub c_euv_questionnaire_choice_indicator {
        display: "flex";
        flex-shrink: "0";
        align-items: "center";
        justify-content: "center";
        width: var!(space-md);
        height: var!(space-md);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-sm);
        background: var!(background);
    }

    pub c_euv_questionnaire_input {
        width: "100%";
        height: var!(control-height);
        padding-left: var!(space-sm);
        padding-right: var!(space-sm);
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-sm);
        outline: "none";
    }

    pub c_euv_questionnaire_previous {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        height: var!(control-height);
        padding-left: var!(space-lg);
        padding-right: var!(space-lg);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-sm);
        white-space: "nowrap";
        cursor: "pointer";
    }

    pub c_euv_questionnaire_skip {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        height: var!(control-height);
        padding-left: var!(space-lg);
        padding-right: var!(space-lg);
        background: "transparent";
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        white-space: "nowrap";
        cursor: "pointer";
    }

    pub c_euv_questionnaire_next {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        height: var!(control-height);
        padding-left: var!(space-lg);
        padding-right: var!(space-lg);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-sm);
        white-space: "nowrap";
        cursor: "pointer";
    }

    pub c_euv_questionnaire_submit {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        height: var!(control-height);
        padding-left: var!(space-lg);
        padding-right: var!(space-lg);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-sm);
        white-space: "nowrap";
        cursor: "pointer";
    }
}

#[cfg(feature = "theme-shadcn")]
class! {
    pub c_euv_collapsible {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);

        gap: var!(space-sm);
    }

    pub c_euv_collapsible_trigger {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(space-sm);
        width: "100%";
        text-align: "left";
        font-weight: var!(font-weight-medium);
        cursor: "pointer";

        gap: var!(space-xs);
        border-color: var!(border);
        border-bottom-right-radius: var!(radius-lg);
        border-bottom-left-radius: var!(radius-lg);
        border-top-right-radius: var!(radius-lg);
        border-top-left-radius: var!(radius-lg);
        border-bottom-style: "solid";
        border-left-style: "solid";
        border-right-style: "solid";
        border-top-style: "solid";
        border-bottom-width: var!(border-width);
        border-left-width: var!(border-width);
        border-right-width: var!(border-width);
        border-top-width: var!(border-width);
        justify-content: "center";
        white-space: "nowrap";
    }

    pub c_euv_marker {
        position: "relative";
        display: "flex";
        width: "100%";
        align-items: "center";
        gap: var!(space-sm);

        color: var!(muted-foreground);
    }

    pub c_euv_native_select {
        appearance: "none";
        outline: "none";
        width: "100%";
        min-width: "0px";
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        padding-left: var!(space-sm);
        padding-right: var!(space-2xl);
        font-size: var!(font-sm);
        cursor: "pointer";
        :disabled {
            pointer-events: "none";
            cursor: "not-allowed";
        }

        min-height: var!(control-height);
        padding-block: var!(space-xs);
        padding-inline: var!(space-md);
        padding-right: format!("calc({} + {})", var!(control-pad-icon), var!(space-xs));
        border-radius: var!(radius-md);
        font-size: var!(font-sm);
        line-height: "1.428571";
        background-color: "transparent";
        transition: format!(
        "color {} {}, box-shadow {} {}",
        var!(duration-fast),
        var!(ease-out),
        var!(duration-fast),
        var!(ease-out)
        );
        :focus-visible {
            border-color: var!(ring);
        }

        :is([aria-invalid="true"]) {
            border-color: var!(destructive);
        }
        :is(".c_theme_dark .c_euv_native_select") {
            background-color: format!("color-mix(in srgb, {} 30%, transparent)", var!(input));
        }
        : is(".c_theme_dark .c_euv_native_select"):hover {
            background-color: format!("color-mix(in srgb, {} 50%, transparent)", var!(input));
        }
    }

    pub c_euv_native_select_size_default {
        height: var!(control-height);

        height: var!(control-height);
        min-height: var!(control-height);
    }

    pub c_euv_native_select_size_sm {
        height: var!(control-height-sm);

        height: var!(control-height-sm);
        min-height: var!(control-height-sm);
        padding-block: var!(space-xs);
        border-radius: var!(radius-md);
    }

    pub c_euv_attachment {
        position: "relative";
        display: "flex";
        max-width: "100%";
        min-width: "0px";
        flex-shrink: "0";
        flex-wrap: "wrap";
        align-items: "center";
        gap: var!(space-sm);
        padding: var!(space-sm);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(card);
        color: var!(card-foreground);
        font-size: var!(font-sm);

        border-radius: var!(radius-xl);
        gap: var!(space-xs-sm);
    }

    pub c_euv_attachment_media {
        position: "relative";
        display: "flex";
        aspect-ratio: "1";
        flex-shrink: "0";
        align-items: "center";
        justify-content: "center";
        overflow: "hidden";
        border-radius: var!(radius-sm);
        background: var!(muted);

        border-radius: var!(radius-lg);
    }

    pub c_euv_attachment_media_variant_image {
        background: var!(muted);

        border-bottom-right-radius: var!(radius-lg);
        border-bottom-left-radius: var!(radius-lg);
        border-top-right-radius: var!(radius-lg);
        border-top-left-radius: var!(radius-lg);
    }

    pub c_euv_attachment_group {
        display: "flex";
        min-width: "0px";
        gap: var!(space-sm);
        overflow-x: "auto";
        overscroll-behavior-x: "contain";
        scroll-snap-type: "x mandatory";

        gap: var!(space-md);
        padding-top: var!(space-xs);
    }

    pub c_euv_bubble_group {
        display: "flex";
        min-width: "0px";
        flex-direction: "column";

        gap: var!(space-sm);
    }

    pub c_euv_bubble {
        position: "relative";
        display: "flex";
        width: "fit-content";
        min-width: "0px";
        flex-direction: "column";

        gap: var!(space-xs);
    }

    pub c_euv_bubble_content {
        width: "fit-content";
        max-width: "100%";
        min-width: "0px";
        overflow: "hidden";
        overflow-wrap: "break-word";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border-radius: var!(radius-lg);

        padding-top: var!(space-sm-md);
        padding-right: var!(space-md);
        padding-bottom: var!(space-sm-md);
        padding-left: var!(space-md);
        border-color: var!(border);
        border-bottom-style: "solid";
        border-left-style: "solid";
        border-right-style: "solid";
        border-top-style: "solid";
        border-bottom-width: var!(border-width);
        border-left-width: var!(border-width);
        border-right-width: var!(border-width);
        border-top-width: var!(border-width);
        border-bottom-right-radius: "24px";
        border-bottom-left-radius: "24px";
        border-top-right-radius: "24px";
        border-top-left-radius: "24px";
    }

    pub c_euv_bubble_reactions {
        position: "absolute";
        display: "flex";
        width: "fit-content";
        align-items: "center";
        justify-content: "center";

        background-color: var!(muted);
        border-bottom-right-radius: "9999px";
        border-bottom-left-radius: "9999px";
        border-top-right-radius: "9999px";
        border-top-left-radius: "9999px";
        gap: var!(space-xs);
        padding-left: var!(space-xs-sm);
        padding-top: var!(space-2xs);
    }

    pub c_euv_message_scroller_content {
        display: "flex";
        min-height: "100%";
        flex-direction: "column";

        gap: var!(space-3xl);
    }

    pub c_euv_message_scroller_button {
        position: "absolute";
        left: "50%";
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        white-space: "nowrap";
        transform: "translateX(-50%)";

        border-bottom-right-radius: var!(radius-2xl);
        border-bottom-left-radius: var!(radius-2xl);
        border-top-right-radius: var!(radius-2xl);
        border-top-left-radius: var!(radius-2xl);
        font-weight: var!(font-weight-medium);
    }

    pub c_euv_trace {
        display: "flex";
        width: "100%";
        min-width: "0px";
        flex-direction: "column";

        gap: var!(space-sm);
    }

    pub c_euv_trace_line {
        display: "flex";
        width: "fit-content";
        max-width: "100%";
        min-width: "0px";
        align-items: "center";
        gap: var!(space-xs);
        color: var!(muted-foreground);

        gap: var!(space-sm);
        color: var!(muted-foreground);
    }

    pub c_euv_trace_collapsible_trigger {
        display: "flex";
        width: "fit-content";
        max-width: "100%";
        min-width: "0px";
        align-items: "center";
        text-align: "left";
        color: var!(muted-foreground);
        cursor: "pointer";

        border-bottom-right-radius: var!(radius-lg);
        border-bottom-left-radius: var!(radius-lg);
        border-top-right-radius: var!(radius-lg);
        border-top-left-radius: var!(radius-lg);
        gap: var!(space-xs);
    }

    pub c_euv_code_block {
        position: "relative";
        display: "flex";
        width: "100%";
        min-width: "0px";
        flex-direction: "column";
        overflow: "hidden";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(muted);
        color: var!(foreground);

        border-radius: var!(radius-lg);
    }

    pub c_euv_code_block_header {
        display: "flex";
        min-width: "0px";
        flex-shrink: "0";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border-bottom: format!("{} solid {}", var!(border-width), var!(border));

        padding: var!(space-md);
        color: var!(muted-foreground);
    }

    pub c_euv_code_block_title {
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        font-weight: var!(font-weight-medium);

        color: var!(foreground);
    }

    pub c_euv_code_block_actions {
        display: "flex";
        flex-shrink: "0";
        align-items: "center";
        gap: var!(space-2xs);
        margin-left: "auto";

        gap: var!(space-xs);
    }

    pub c_euv_code_block_copy_button {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-sm);
        background: var!(background);
        color: var!(foreground);
        white-space: "nowrap";
        cursor: "pointer";

        font-weight: var!(font-weight-medium);
    }

    pub c_euv_code_block_content {
        min-height: "0px";
        min-width: "0px";
        overflow: "auto";
        text-align: "start";
        font-family: var!(font-family-mono);
        font-size: var!(font-sm);
        outline: "none";

        padding-left: var!(space-lg);
        padding-top: var!(space-lg);
    }

    pub c_euv_code_block_line {
        position: "relative";
        display: "block";

        padding-left: var!(space-lg);
    }

    pub c_euv_code_block_footer {
        display: "flex";
        min-width: "0px";
        flex-shrink: "0";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border-top: format!("{} solid {}", var!(border-width), var!(border));

        padding: var!(space-lg);
    }

    pub c_euv_navigation_menu {
        position: "relative";
        display: "flex";
        width: "100%";
        justify-content: "center";

        align-items: "center";
    }

    pub c_euv_navigation_menu_list {
        display: "flex";
        flex-wrap: "wrap";
        align-items: "center";
        gap: var!(space-md);
        margin: "0px";
        padding: "0px";
        list-style: "none";

        align-items: "center";
        gap: "0px";
        justify-content: "center";
    }

    pub c_euv_navigation_menu_trigger {
        display: "inline-flex";
        align-items: "center";
        gap: var!(space-xs);
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border: "none";
        background: "transparent";
        color: var!(foreground);
        font-size: var!(font-sm);
        font-weight: var!(font-weight-medium);
        white-space: "nowrap";
        cursor: "pointer";

        border-bottom-right-radius: var!(radius-lg);
        border-bottom-left-radius: var!(radius-lg);
        border-top-right-radius: var!(radius-lg);
        border-top-left-radius: var!(radius-lg);
        justify-content: "center";
        padding-left: var!(control-pad-inline);
        padding-top: var!(space-xs-sm);
    }

    pub c_euv_navigation_menu_link {
        display: "block";
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        border-radius: var!(radius-sm);
        color: var!(foreground);
        font-size: var!(font-sm);
        text-decoration: "none";

        padding: var!(space-sm);
        border-radius: var!(radius-lg);
    }

    pub c_euv_questionnaire {
        display: "flex";
        width: "100%";
        min-width: "0px";
        flex-direction: "column";
        gap: var!(space-md);

        gap: var!(space-lg);
    }

    pub c_euv_questionnaire_item {
        min-width: "0px";
        padding: "0px";
        border: "none";
        outline: "none";

        gap: var!(space-lg);
    }

    pub c_euv_questionnaire_title {
        font-size: var!(font-lg);
        font-weight: var!(font-weight-semibold);
        line-height: "1.2";

        font-size: var!(font-base);
        font-weight: var!(font-weight-medium);
    }

    pub c_euv_questionnaire_choice {
        position: "relative";
        display: "flex";
        min-height: var!(control-height);
        align-items: "flex-start";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        text-align: "start";
        cursor: "pointer";

        padding-top: var!(space-sm-md);
        padding-right: var!(space-md);
        padding-bottom: var!(space-sm-md);
        padding-left: var!(space-md);
        border-radius: var!(radius-lg);
        background-color: "transparent";
        border-color: var!(border);
        border-bottom-style: "solid";
        border-left-style: "solid";
        border-right-style: "solid";
        border-top-style: "solid";
        border-bottom-width: var!(border-width);
        border-left-width: var!(border-width);
        border-right-width: var!(border-width);
        border-top-width: var!(border-width);
        gap: var!(space-sm-md);
    }

    pub c_euv_questionnaire_choice_enabled {
        cursor: "pointer";

        background-color: "transparent";
        border-color: var!(border);
        border-bottom-style: "solid";
        border-left-style: "solid";
        border-right-style: "solid";
        border-top-style: "solid";
        border-bottom-width: var!(border-width);
        border-left-width: var!(border-width);
        border-right-width: var!(border-width);
        border-top-width: var!(border-width);
        gap: var!(space-sm-md);
        padding-top: var!(space-sm-md);
    }

    pub c_euv_questionnaire_choice_disabled {
        pointer-events: "none";
        cursor: "not-allowed";
        opacity: "0.5";

        background-color: "transparent";
        border-color: var!(border);
        border-bottom-style: "solid";
        border-left-style: "solid";
        border-right-style: "solid";
        border-top-style: "solid";
        border-bottom-width: var!(border-width);
        border-left-width: var!(border-width);
        border-right-width: var!(border-width);
        border-top-width: var!(border-width);
        gap: var!(space-sm-md);
        padding-top: var!(space-sm-md);
    }

    pub c_euv_questionnaire_choice_label {
        display: "flex";
        min-width: "0px";
        flex: "1";
        flex-direction: "column";
        line-height: "1.3";

        gap: var!(space-2xs);
    }

    pub c_euv_questionnaire_choice_indicator {
        display: "flex";
        flex-shrink: "0";
        align-items: "center";
        justify-content: "center";
        width: var!(space-md);
        height: var!(space-md);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-sm);
        background: var!(background);

        border-color: var!(border);
        border-bottom-style: "solid";
        border-left-style: "solid";
        border-right-style: "solid";
        border-top-style: "solid";
        border-bottom-width: var!(border-width);
        border-left-width: var!(border-width);
        border-right-width: var!(border-width);
        border-top-width: var!(border-width);
    }

    pub c_euv_questionnaire_input {
        width: "100%";
        height: var!(control-height);
        padding-left: var!(space-sm);
        padding-right: var!(space-sm);
        border: format!("{} solid {}", var!(border-width), var!(input));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-sm);
        outline: "none";

        height: var!(control-height-sm);
        border-radius: var!(radius-lg);
        font-size: var!(font-base);
    }

    pub c_euv_questionnaire_previous {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        height: var!(control-height);
        padding-left: var!(space-lg);
        padding-right: var!(space-lg);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(background);
        color: var!(foreground);
        font-size: var!(font-sm);
        white-space: "nowrap";
        cursor: "pointer";

        height: var!(control-height-sm);
        border-radius: var!(radius-lg);
    }

    pub c_euv_questionnaire_skip {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        height: var!(control-height);
        padding-left: var!(space-lg);
        padding-right: var!(space-lg);
        background: "transparent";
        color: var!(muted-foreground);
        font-size: var!(font-sm);
        white-space: "nowrap";
        cursor: "pointer";

        height: var!(control-height-sm);
        gap: var!(space-xs-sm);
        border-bottom-right-radius: var!(radius-lg);
        border-bottom-left-radius: var!(radius-lg);
        border-top-right-radius: var!(radius-lg);
        border-top-left-radius: var!(radius-lg);
        background-color: var!(background);
        border-color: var!(border);
        border-bottom-style: "solid";
        border-left-style: "solid";
        border-right-style: "solid";
        border-top-style: "solid";
        border-bottom-width: var!(border-width);
        border-left-width: var!(border-width);
        border-right-width: var!(border-width);
        border-top-width: var!(border-width);
        font-weight: var!(font-weight-medium);
        padding-left: var!(control-pad-inline);
    }

    pub c_euv_questionnaire_next {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        height: var!(control-height);
        padding-left: var!(space-lg);
        padding-right: var!(space-lg);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-sm);
        white-space: "nowrap";
        cursor: "pointer";

        height: var!(control-height-sm);
        gap: var!(space-xs-sm);
        border-radius: var!(radius-lg);
        font-weight: var!(font-weight-medium);
        padding-left: var!(control-pad-inline);
    }

    pub c_euv_questionnaire_submit {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        height: var!(control-height);
        padding-left: var!(space-lg);
        padding-right: var!(space-lg);
        border: format!("{} solid {}", var!(border-width), var!(border));
        border-radius: var!(radius-md);
        background: var!(primary);
        color: var!(primary-foreground);
        font-size: var!(font-sm);
        white-space: "nowrap";
        cursor: "pointer";

        height: var!(control-height-sm);
        border-radius: var!(radius-lg);
    }
}
