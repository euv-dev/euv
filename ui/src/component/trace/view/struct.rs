use super::*;

/// Props for the [`euv_trace`] component.
///
/// The trace takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTraceProps {}

/// Props for the [`euv_trace_line`] component.
///
/// The line carries the state of the step it describes.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTraceLineProps {
    /// The state of the step.
    pub status: Option<EuvTraceStatus>,
}

/// Props for the [`euv_trace_icon`] component.
///
/// The icon takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTraceIconProps {}

/// Props for the [`euv_trace_title`] component.
///
/// The title takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTraceTitleProps {}

/// Props for the [`euv_trace_description`] component.
///
/// The description takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTraceDescriptionProps {}

/// Props for the [`euv_trace_collapsible`] component.
///
/// The collapsible carries the state of the section it folds.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTraceCollapsibleProps {
    /// Whether the section is open.
    pub open: Signal<bool>,
}

/// Props for the [`euv_trace_collapsible_trigger`] component.
///
/// The trigger carries the state of the section it folds.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTraceCollapsibleTriggerProps {
    /// Whether the section is open.
    pub open: Signal<bool>,
}

/// Props for the [`euv_trace_collapsible_content`] component.
///
/// The content carries the state of the section it belongs to.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTraceCollapsibleContentProps {
    /// Whether the section is open.
    pub open: Signal<bool>,
}
