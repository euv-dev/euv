use super::*;

/// Viewport-local immediate-mode 2D/framebuffer drawing surface passed to `GameScene::render`.
#[derive(CustomDebug)]
pub struct GameRenderContext<'a> {
    /// Viewport bounds in root logical coordinates.
    pub(crate) bounds: Rect,
    /// Active display list receiving hardware/software/hybrid draw commands.
    #[debug(skip)]
    pub(crate) display_list: &'a mut DisplayList,
}

/// Shared host coordinating a `GameScene`, fixed-timestep `FrameClock`, and `InputState` inside `euv`.
#[derive(Clone, CustomDebug, Data)]
pub struct GameEngineHost {
    /// Fixed-timestep simulation clock.
    pub(crate) clock: Rc<RefCell<FrameClock>>,
    /// Real-time polled input state.
    pub(crate) input: Rc<RefCell<InputState>>,
    /// Hosted game scene implementation.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) scene: Rc<RefCell<dyn GameScene>>,
    /// Mounted viewport canvas `(scene, node)` handles used to gate stepping and auto-dirty visible viewports.
    #[debug(skip)]
    pub(crate) viewport_nodes: Rc<RefCell<Vec<(SceneHandle, NodeId)>>>,
}
