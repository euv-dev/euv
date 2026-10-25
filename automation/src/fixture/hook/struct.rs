use super::*;

/// Reactive state shared by the fixture controls and visible assertions.
#[derive(Clone, Copy, Data, Debug, New)]
#[new(pub(crate))]
pub(crate) struct FixtureState {
    /// Counter updated by the real button event handler.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) count: Signal<i32>,
    /// Text synchronized from native input events.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) text: Signal<String>,
    /// Checked state owned by the euv checkbox component.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) checked: Signal<bool>,
    /// Visibility of the mounted modal subtree.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) modal: Signal<bool>,
    /// Records whether the browser delivered a trusted interaction.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) trusted: Signal<bool>,
    /// Number of Enter key events observed by the fixture.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) keys: Signal<i32>,
    /// Scroll offset propagated through the actual scroll event handler.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scroll: Signal<i32>,
}
