use super::*;

/// Implements static scene creation for `SceneManager`.
impl SceneManager {
    /// Creates a new `SceneRc` wrapping the given scene in a reference-counted cell.
    ///
    /// # Arguments
    ///
    /// - `T: Scene + 'static` - The concrete scene type.
    ///
    /// # Returns
    ///
    /// - `SceneRc` - The wrapped scene.
    pub fn create_scene<T>(scene: T) -> SceneRc
    where
        T: Scene + 'static,
    {
        // `EngineCell<T: ?Sized>` accepts `dyn Scene` directly,
        // so we do not need an intermediate `Box`.
        Rc::new(EngineCell::new(scene))
    }
}

/// Implements scene registration and lifecycle management for `SceneManager`.
impl SceneManager {
    /// Registers a scene under the given name.
    ///
    /// # Arguments
    ///
    /// - `String` - The name to register the scene under.
    /// - `SceneRc` - The scene to register.
    pub fn register(&mut self, name: String, scene: SceneRc) {
        self.get_mut_scenes().insert(name, scene);
    }

    /// Unregisters and removes the scene with the given name.
    ///
    /// # Arguments
    ///
    /// - `N: AsRef<str>` - The name of the scene to remove.
    pub fn unregister<N>(&mut self, name: N)
    where
        N: AsRef<str>,
    {
        self.get_mut_scenes().remove(name.as_ref());
    }

    /// Transitions to the scene with the given name.
    ///
    /// Calls `on_exit` on the current scene and `on_enter` on the new scene.
    /// Returns `false` if no scene with the given name is registered.
    ///
    /// # Arguments
    ///
    /// - `N: AsRef<str>` - The name of the scene to switch to.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the transition was successful.
    pub fn switch_to<N>(&mut self, name: N) -> bool
    where
        N: AsRef<str>,
    {
        let name_ref: &str = name.as_ref();
        if !self.get_scenes().contains_key(name_ref) {
            return false;
        }
        let current_name: Option<String> = self.get_mut_current_scene_name().clone();
        if let Some(name) = current_name.as_ref()
            && let Some(current_scene) = self.get_scenes().get(name)
        {
            current_scene.get_mut().on_exit();
        }
        self.set_current_scene_name(Some(name_ref.to_string()));
        if let Some(new_scene) = self.get_scenes().get(name_ref) {
            new_scene.get_mut().on_enter();
        }
        true
    }

    /// Requests a deferred scene transition to be applied on the next update.
    ///
    /// # Arguments
    ///
    /// - `String` - The name of the scene to switch to.
    pub fn request_transition(&mut self, name: String) {
        self.set_pending_scene_name(Some(name));
    }

    /// Processes a pending scene transition if one was requested.
    pub fn process_pending_transition(&mut self) {
        let Some(name) = self.get_mut_pending_scene_name().take() else {
            return;
        };
        self.switch_to(&name);
    }

    /// Calls `on_update` on the current scene.
    ///
    /// # Arguments
    ///
    /// - `f64` - The delta time in seconds.
    pub fn update(&mut self, delta_time: f64) {
        self.process_pending_transition();
        // OPT 38: borrow the active scene name (Option<&String>) instead of
        // cloning the whole String, then clone the `SceneRc` handle. The
        // old form `get_mut_current_scene_name().clone()` paid for a full
        // String heap allocation per frame on every scene, even though the
        // name is only used as a HashMap key.
        let Some(current_name) = self.try_get_current_scene_name().as_ref() else {
            return;
        };
        let Some(scene) = self.get_scenes().get(current_name).cloned() else {
            return;
        };
        scene.get_mut().on_update(delta_time);
    }

    /// Calls `on_render` on the current scene, recording into the shared draw list.
    ///
    /// The manager's reusable `DrawList` is cleared, filled by the scene, and
    /// left populated for the caller to replay (e.g. via
    /// `CanvasRenderer::replay`). Reusing the list avoids per-frame allocation.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas rendering context used to replay
    ///   the recorded commands.
    pub fn render(&mut self, context: &CanvasRenderingContext2d) {
        let Some(current_name) = self.try_get_current_scene_name().as_ref() else {
            return;
        };
        // Clone the `Rc` so the immutable borrow of the scenes map ends before
        // we mutably borrow the draw list.
        let Some(scene) = self.get_scenes().get(current_name).cloned() else {
            return;
        };
        self.get_mut_draw_list().clear();
        {
            let draw_list: &mut DrawList = self.get_mut_draw_list();
            scene.get().on_render(draw_list);
        }
        CanvasRenderer::replay_context(context, self.get_draw_list());
    }

    /// Returns whether a scene with the given name is registered.
    ///
    /// # Arguments
    ///
    /// - `N: AsRef<str>` - The scene name to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the scene is registered.
    pub fn has_scene<N>(&self, name: N) -> bool
    where
        N: AsRef<str>,
    {
        self.get_scenes().contains_key(name.as_ref())
    }

    /// Returns the name of the currently active scene.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - The current scene name, or `None`.
    pub fn current_name(&self) -> Option<&str> {
        self.try_get_current_scene_name().as_deref()
    }
}

/// Forwards `SceneManager::update` through the [`Updatable`] trait so that
/// scene managers can be driven alongside entities, animators, and physics
/// worlds in a single homogeneous update loop.
///
/// The inherent [`SceneManager::update`] method is the canonical implementation;
/// this impl exists purely for trait dispatch. The inherent call resolves
/// first when both are in scope, so there is no recursion.
impl Updatable for SceneManager {
    /// Advances the simulation by `delta_time` seconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - Seconds elapsed since the previous update.
    fn update(&mut self, delta_time: f64) {
        SceneManager::update(self, delta_time);
    }
}

/// Implements `Default` for `SceneManager` as a new empty manager.
impl Default for SceneManager {
    /// Constructs a default [`SceneManager`] value.
    ///
    /// # Returns
    ///
    /// - `SceneManager` - A default-constructed instance with the documented initial state.
    fn default() -> SceneManager {
        SceneManager::new()
    }
}
