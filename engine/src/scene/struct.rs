use super::*;

/// Manages scene registration, transitions, and lifecycle.
#[derive(Clone, Data, New)]
pub struct SceneManager {
    /// All registered scenes keyed by name.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) scenes: HashMap<String, SceneRc>,
    /// The name of the currently active scene, if any.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) current_scene_name: Option<String>,
    /// The name of the next scene to switch to (for deferred transitions).
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    #[get(pub(crate))]
    pub(crate) pending_scene_name: Option<String>,
    /// The reusable per-frame draw list. Cleared and refilled each render call
    /// so the backing `Vec` capacity is reused across frames.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    #[get(pub(crate))]
    pub(crate) draw_list: DrawList,
}
