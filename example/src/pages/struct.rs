use super::*;

/// Cross-component custom event emitted via `#[event]` / `emit!` and received via `on_event!` across showcase pages.
#[event]
#[derive(Data, Debug)]
pub struct ShowcasePulseEvent {
    /// Source page or control emitting the pulse.
    pub(crate) source: String,
    /// Monotonic pulse sequence counter.
    #[get(type(copy))]
    pub(crate) seq: u32,
}

/// App-wide reactive global state store registered via `#[global_state_store]` and observed with `watch!(global ...)`.
#[global_state_store]
#[derive(Data, Debug, Default, PartialEq)]
pub struct ShowcaseGlobalBanner {
    /// Total cross-component pulses received globally.
    #[get(type(copy))]
    pub(crate) pulses: u32,
    /// Latest event source label broadcast globally.
    pub(crate) last_source: String,
}

/// Typed `Component` + `Lifecycle` page wrapper enforcing `PageRetention::Freeze` for every showcase route.
#[derive(Clone, Getter)]
pub struct ShowcasePage {
    /// Static normalized route path owned by this page component.
    #[get(type(copy))]
    pub(crate) route: &'static str,
    /// Shared reactive showcase state.
    pub(crate) state: ShowcaseState,
    /// Active theme tokens snapshot for this render pass.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
