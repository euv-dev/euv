use super::*;

/// One choice of a [`euv_cascader`] column.
///
/// A choice carries the machine value written into the selection signal,
/// the label the reader sees, and the choices one level below it. The
/// struct is recursive, so it cannot be `Copy`; it derives `Clone` so a
/// caller can build several pickers from one shared template.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvCascaderOption {
    /// The machine value of the choice.
    #[get(type(copy))]
    pub value: &'static str,
    /// The label the reader sees.
    #[get(type(copy))]
    pub label: &'static str,
    /// The choices one level below this one.
    pub children: Vec<EuvCascaderOption>,
}

/// Props for the [`euv_cascader`] component.
///
/// The chosen path lives in a caller-owned `Signal<Vec<String>>` so the
/// picker takes part in two-way binding; the open state of the panel and
/// the path the reader is browsing are owned by the component itself.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCascaderProps {
    /// The top-level choices.
    pub options: Vec<EuvCascaderOption>,
    /// The values of the chosen path, outermost first.
    pub value: Signal<Vec<String>>,
    /// The text shown when nothing is chosen.
    pub placeholder: &'static str,
    /// The group label above the control.
    pub label: &'static str,
    /// The callback fired when a choice is made.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}
