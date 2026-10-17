use super::*;

/// Reactive state for the select and textarea demo feature.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseSelect {
    /// The selected fruit value.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) selected_fruit: Signal<String>,
    /// The selected country value.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) selected_country: Signal<String>,
    /// The selected city value.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) selected_city: Signal<String>,
    /// The available city options for the selected country.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cities: Signal<Vec<(String, String)>>,
    /// The feedback result message.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) feedback: Signal<String>,
    /// The textarea content.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) textarea_content: Signal<String>,
    /// The textarea validation error message.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) textarea_error: Signal<String>,
}
