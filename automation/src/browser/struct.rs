use super::*;

/// A real WebDriver session; interactions use browser input commands, not DOM mocks.
#[derive(Data, Debug, New)]
#[new(pub(crate))]
pub struct Browser {
    /// Loopback endpoint of the task-owned ChromeDriver instance.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) endpoint: String,
    /// Session identifier returned by the actual WebDriver server.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) session: String,
    /// Maximum duration for observable assertion polling.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) timeout: Duration,
    /// Exact driver-reported browser metadata, including process and profile identity.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) capabilities: Value,
}
