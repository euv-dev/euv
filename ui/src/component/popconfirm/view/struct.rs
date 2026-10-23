use super::*;

/// Props for the [`euv_popconfirm`] component.
///
/// The trigger is supplied as the component's children, so the bubble is
/// always anchored to whatever the caller rendered — a button, a link or
/// a row. The open state is owned by the component itself.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvPopconfirmProps {
    /// The question shown in the bubble, rendered in `c_euv_popconfirm_text`.
    pub title: &'static str,
    /// The label of the confirm button.
    ///
    /// An empty label renders no confirm button at all.
    pub ok_label: &'static str,
    /// The label of the cancel button.
    ///
    /// An empty label renders no cancel button at all.
    pub cancel_label: &'static str,
    /// The callback fired when the confirm button is clicked.
    #[debug(skip)]
    pub on_ok: Option<Rc<dyn Fn()>>,
    /// The callback fired when the cancel button is clicked.
    #[debug(skip)]
    pub on_cancel: Option<Rc<dyn Fn()>>,
}
