/// Browser cases exercising actual euv controls and observable state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Scenario {
    /// Trusted clicks update the reactive counter.
    Counter,
    /// Native editing updates the bound signal.
    Input,
    /// Enter and Tab preserve keyboard behavior and focus.
    Keyboard,
    /// Checked state round-trips through the euv component.
    Checkbox,
    /// The mounted dialog opens and closes through real controls.
    Modal,
    /// Native wheel events update the scroll-position signal.
    Scroll,
    /// Controls remain usable in a narrow browser viewport.
    Responsive,
    /// Deliberately incorrect expectation proves failure is observable.
    Negative,
}
