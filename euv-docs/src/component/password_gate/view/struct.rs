use super::*;

/// Props for the [`docs_password_gate`] component.
#[derive(Clone, Default)]
pub(crate) struct DocsPasswordGateProps {
    /// Page route (used as the unlock key in localStorage).
    pub route: &'static str,
    /// Hex-encoded SHA-256 of the correct password.
    pub expected_hash: &'static str,
    /// Page title (shown above the form so the user knows which article
    /// they're being asked to unlock).
    pub title: &'static str,
}
