use super::*;

/// Reactive state for the parent-child props passing demo.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub(crate) struct UsePropsDemo {
    /// The parent message to pass down to child.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) parent_message: Signal<String>,
}

/// Reactive state for the two-way binding demo.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseTwoWayDemo {
    /// The shared text value bound between parent and child.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) shared_text: Signal<String>,
    /// The shared counter value bound between parent and child.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) shared_count: Signal<i32>,
}

/// Reactive state for the cross-component reactive binding demo.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub(crate) struct UseCrossComponentDemo {
    /// The temperature in Celsius.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) celsius: Signal<f64>,
    /// The temperature in Fahrenheit (derived).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fahrenheit: Signal<f64>,
    /// The RGB red channel value.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) red: Signal<i32>,
    /// The RGB green channel value.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) green: Signal<i32>,
    /// The RGB blue channel value.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) blue: Signal<i32>,
    /// The hex color string (derived).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) hex_color: Signal<String>,
}

/// Reactive state for the typed props demo.
///
/// Demonstrates passing non-String props (bool, i32) to child components.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseTypedPropsDemo {
    /// Whether the child input is disabled.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) disabled: Signal<bool>,
    /// The maximum count allowed.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) max_count: Signal<i32>,
    /// The current count value.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) current_count: Signal<i32>,
}
