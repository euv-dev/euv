use super::*;

/// Holds the steps of a trace.
///
/// The trace is a column of lines, one per step. The line describes the
/// step, and the collapsible inside it holds the long form output of
/// that step.
///
/// # Arguments
///
/// - `VirtualNode<EuvTraceProps>` - The props node carrying the settings
///   of the trace.
///
/// # Returns
///
/// - `VirtualNode` - The euv trace virtual DOM tree.
#[component]
pub fn euv_trace(node: VirtualNode<EuvTraceProps>) -> VirtualNode {
    let EuvTraceProps {}: EuvTraceProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "trace"
            class: c_euv_trace()
            children
        }
    }
}

/// Holds one step of a trace.
///
/// The line says how the step is going: a step that failed is drawn in
/// the destructive colour, and a step that is still waiting is dimmed.
///
/// # Arguments
///
/// - `VirtualNode<EuvTraceLineProps>` - The props node carrying the
///   state of the step.
///
/// # Returns
///
/// - `VirtualNode` - The euv trace line virtual DOM tree.
#[component]
pub fn euv_trace_line(node: VirtualNode<EuvTraceLineProps>) -> VirtualNode {
    let EuvTraceLineProps { status }: EuvTraceLineProps = node.try_get_props().unwrap_or_default();
    let status_attr: &str = match status {
        Some(EuvTraceStatus::Idle) => TRACE_STATUS_IDLE,
        Some(EuvTraceStatus::Running) => TRACE_STATUS_RUNNING,
        Some(EuvTraceStatus::Error) => TRACE_STATUS_ERROR,
        Some(EuvTraceStatus::Pending) => TRACE_STATUS_PENDING,
        None => TRACE_STATUS_IDLE,
    };
    let status_class: fn() -> &'static Css = match status {
        Some(EuvTraceStatus::Idle) => c_euv_trace_line_idle,
        Some(EuvTraceStatus::Running) => c_euv_trace_line_running,
        Some(EuvTraceStatus::Error) => c_euv_trace_line_error,
        Some(EuvTraceStatus::Pending) => c_euv_trace_line_pending,
        None => c_euv_trace_line_idle,
    };
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_trace_line();

    html! {
        span {
            "data-slot": "trace-line"
            "data-status": status_attr
            class: base_class

            class: status_class()
            children
        }
    }
}

/// Holds the icon that leads a trace line.
///
/// The icon is decorative: the line already says what the step is.
///
/// # Arguments
///
/// - `VirtualNode<EuvTraceIconProps>` - The props node carrying the
///   settings of the icon.
///
/// # Returns
///
/// - `VirtualNode` - The euv trace icon virtual DOM tree.
#[component]
pub fn euv_trace_icon(node: VirtualNode<EuvTraceIconProps>) -> VirtualNode {
    let EuvTraceIconProps {}: EuvTraceIconProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        span {
            "data-slot": "trace-icon"
            "aria-hidden": "true"
            class: c_euv_trace_icon()
            children
        }
    }
}

/// Holds the name of a trace step.
///
/// # Arguments
///
/// - `VirtualNode<EuvTraceTitleProps>` - The props node carrying the
///   settings of the title.
///
/// # Returns
///
/// - `VirtualNode` - The euv trace title virtual DOM tree.
#[component]
pub fn euv_trace_title(node: VirtualNode<EuvTraceTitleProps>) -> VirtualNode {
    let EuvTraceTitleProps {}: EuvTraceTitleProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        span {
            "data-slot": "trace-title"
            class: c_euv_trace_title()
            children
        }
    }
}

/// Holds the detail of a trace step.
///
/// The detail is the number beside the name of the step, so it is set in
/// figures that line up in a column.
///
/// # Arguments
///
/// - `VirtualNode<EuvTraceDescriptionProps>` - The props node carrying the
///   settings of the description.
///
/// # Returns
///
/// - `VirtualNode` - The euv trace description virtual DOM tree.
#[component]
pub fn euv_trace_description(node: VirtualNode<EuvTraceDescriptionProps>) -> VirtualNode {
    let EuvTraceDescriptionProps {}: EuvTraceDescriptionProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        span {
            "data-slot": "trace-description"
            class: c_euv_trace_description()
            children
        }
    }
}

/// Holds a folded section of a trace.
///
/// The collapsible carries the state of the section. The trigger and the
/// content inside it read that same state.
///
/// # Arguments
///
/// - `VirtualNode<EuvTraceCollapsibleProps>` - The props node carrying
///   the state of the section.
///
/// # Returns
///
/// - `VirtualNode` - The euv trace collapsible virtual DOM tree.
#[component]
pub fn euv_trace_collapsible(node: VirtualNode<EuvTraceCollapsibleProps>) -> VirtualNode {
    let EuvTraceCollapsibleProps { open }: EuvTraceCollapsibleProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "trace-collapsible"
            "data-state": if { open.get() } { "open" } else { "closed" }
            class: c_euv_trace_collapsible()
            children
        }
    }
}

/// Folds a section of a trace away.
///
/// The trigger reports the state of the section through
/// `aria-expanded`, so a reader knows whether the output below it is
/// there before they press it.
///
/// # Arguments
///
/// - `VirtualNode<EuvTraceCollapsibleTriggerProps>` - The props node
///   carrying the state of the section.
///
/// # Returns
///
/// - `VirtualNode` - The euv trace collapsible trigger virtual DOM tree.
#[component]
pub fn euv_trace_collapsible_trigger(
    node: VirtualNode<EuvTraceCollapsibleTriggerProps>,
) -> VirtualNode {
    let EuvTraceCollapsibleTriggerProps { open }: EuvTraceCollapsibleTriggerProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            "data-slot": "trace-collapsible-trigger"
            "aria-expanded": if { open.get() } { "true" } else { "false" }
            type: "button"
            class: c_euv_trace_collapsible_trigger()
            onclick: on_trace_toggle(open)
            children
        }
    }
}

/// Holds the output of a folded trace section.
///
/// The output is hidden while the section is closed, so the long form
/// output of a step does not push the rest of the trace out of view.
///
/// # Arguments
///
/// - `VirtualNode<EuvTraceCollapsibleContentProps>` - The props node
///   carrying the state of the section.
///
/// # Returns
///
/// - `VirtualNode` - The euv trace collapsible content virtual DOM tree.
#[component]
pub fn euv_trace_collapsible_content(
    node: VirtualNode<EuvTraceCollapsibleContentProps>,
) -> VirtualNode {
    let EuvTraceCollapsibleContentProps { open }: EuvTraceCollapsibleContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "trace-collapsible-content"
            "data-state": if { open.get() } { "open" } else { "closed" }
            class: c_euv_trace_collapsible_content()
            class: if { open.get() } {
                c_euv_trace_collapsible_content_open()
            } else {
                c_euv_trace_collapsible_content_closed()
            }
            div {
                "data-slot": "trace-collapsible-content-inner"
                class: c_euv_trace_collapsible_content_inner()
                children
            }
        }
    }
}

/// Builds the handler that folds a trace section away or opens it.
///
/// # Arguments
///
/// - `Signal<bool>` - The state of the section.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The press handler.
fn on_trace_toggle(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let next: bool = !open.get();
        open.set(next);
    }))
}
