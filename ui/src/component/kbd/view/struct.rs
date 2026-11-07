use super::*;

/// Props for the `euv_kbd` component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvKbdProps {
    /// The key the element stands for.
    #[get(type(copy))]
    pub text: &'static str,
}
