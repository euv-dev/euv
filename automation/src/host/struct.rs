use super::*;

/// Owns one helper process and reaps it on success or early failure.
#[derive(Data, Debug, New)]
#[new(pub(crate))]
pub(crate) struct OwnedProcess {
    /// Process handle created by this runner, never a borrowed system process.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) child: Child,
    /// Diagnostic identity of the owned helper.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: &'static str,
}

/// Serves only the generated fixture directory over loopback HTTP.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct FixtureFiles;
