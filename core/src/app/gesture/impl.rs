use super::*;

/// Default idle state for `GestureArena`.
impl Default for GestureArena {
    /// Constructs an idle `GestureArena` with empty pointer tracking tables.
    ///
    /// # Returns
    ///
    /// - `Self` - Idle gesture arbitration state machine.
    fn default() -> Self {
        Self {
            active: ActiveGesture::Idle,
            primary_pointer_id: None,
            primary_device: PointerDevice::Mouse,
            origin: Point::default(),
            last_pos: Point::default(),
            origin_hit: None,
            pressed_target: None,
            direct_drag_target: None,
            overlay_barrier: None,
            suppress_click: false,
            arrested_fling: false,
            route_swipe_allowed: false,
            started_ms: 0,
            wheel_isolation_secs: 0.0,
            pointer_idle_secs: HashMap::new(),
        }
    }
}

/// Core arbitration, priority-tier barrier resolution, and stale-contact hygiene for `GestureArena`.
impl GestureArena {
    /// Creates a fresh idle `GestureArena`.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized gesture arena.
    #[must_use]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Returns the primary pointer identifier currently arbitrated, if any.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Gesture arena reference.
    ///
    /// # Returns
    ///
    /// - `Option<u64>` - Active primary pointer identifier.
    #[must_use]
    pub(crate) fn primary_pointer(&self) -> Option<u64> {
        let Self {
            primary_pointer_id, ..
        } = self;
        *primary_pointer_id
    }

    /// Returns the initial leaf hit node captured at `PointerDown`, if any.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Gesture arena reference.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Origin hit target node.
    #[must_use]
    pub(crate) fn origin_hit_node(&self) -> Option<NodeId> {
        let Self { origin_hit, .. } = self;
        *origin_hit
    }

    /// Returns the eligible direct-drag target captured at `PointerDown`, if any.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Gesture arena reference.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Direct drag target node handle.
    #[must_use]
    pub(crate) fn direct_drag_node(&self) -> Option<NodeId> {
        let Self {
            direct_drag_target, ..
        } = self;
        *direct_drag_target
    }

    /// Returns the scene node currently owning the active gesture when applicable.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Gesture arena reference.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Owning scene node handle for node-bound gestures.
    #[must_use]
    pub(crate) fn active_owner_node(&self) -> Option<NodeId> {
        match self.get_active() {
            ActiveGesture::VerticalScroll(id)
            | ActiveGesture::HorizontalScroll(id)
            | ActiveGesture::DirectDrag(id)
            | ActiveGesture::ScrollbarDrag(id)
            | ActiveGesture::TextSelection(id) => Some(id),
            ActiveGesture::Idle
            | ActiveGesture::Pending
            | ActiveGesture::RouteSwipe
            | ActiveGesture::Pinch => None,
        }
    }

    /// Resets the current gesture sequence back to `ActiveGesture::Idle` while preserving wheel cooldown state.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable gesture arena.
    pub(crate) fn reset_sequence(&mut self) {
        self.set_active(ActiveGesture::Idle);
        self.set_primary_pointer_id(None);
        self.set_primary_device(PointerDevice::Mouse);
        self.set_origin(Point::default());
        self.set_last_pos(Point::default());
        self.set_origin_hit(None);
        self.set_pressed_target(None);
        self.set_direct_drag_target(None);
        self.set_overlay_barrier(None);
        self.set_suppress_click(false);
        self.set_arrested_fling(false);
        self.set_route_swipe_allowed(false);
        self.set_started_ms(0);
    }

    /// Resets all gesture sequence state, pointer idle timers, and wheel isolation cooldowns.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable gesture arena.
    pub(crate) fn reset_all(&mut self) {
        self.reset_sequence();
        self.set_wheel_isolation_secs(0.0);
        self.get_mut_pointer_idle_secs().clear();
    }

    /// Enters `ActiveGesture::Pending` for a new primary contact at `pos`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable gesture arena.
    /// - `u64` - Primary pointer identifier.
    /// - `PointerDevice` - Input device class.
    /// - `Point` - Starting logical coordinates.
    /// - `u64` - Monotonic runtime timestamp in milliseconds.
    /// - `bool` - Whether this contact arrested an active inertial fling.
    pub(crate) fn begin_primary(
        &mut self,
        pointer_id: u64,
        device: PointerDevice,
        pos: Point,
        started_ms: u64,
        arrested_fling: bool,
    ) {
        self.set_active(ActiveGesture::Pending);
        self.set_primary_pointer_id(Some(pointer_id));
        self.set_primary_device(device);
        self.set_origin(pos);
        self.set_last_pos(pos);
        self.set_origin_hit(None);
        self.set_pressed_target(None);
        self.set_direct_drag_target(None);
        self.set_overlay_barrier(None);
        self.set_suppress_click(arrested_fling);
        self.set_arrested_fling(arrested_fling);
        self.set_route_swipe_allowed(false);
        self.set_started_ms(started_ms);
        self.set_wheel_isolation_secs(0.0);
        let _: Option<f32> = self.get_mut_pointer_idle_secs().insert(pointer_id, 0.0);
    }

    /// Attaches scene-resolved targets discovered during `PointerDown` hit testing.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable gesture arena.
    /// - `Option<NodeId>` - Leaf hit node under the pointer.
    /// - `Option<NodeId>` - Pressed interactive target node.
    /// - `Option<NodeId>` - Eligible direct-drag control node.
    /// - `Option<NodeId>` - Tier 0 enclosing `Portal` or `Position::Fixed` barrier.
    /// - `bool` - Whether route swipe is allowed if no horizontal scroll ancestor claims the drag.
    pub(crate) fn bind_targets(
        &mut self,
        origin_hit: Option<NodeId>,
        pressed_target: Option<NodeId>,
        direct_drag_target: Option<NodeId>,
        overlay_barrier: Option<NodeId>,
        route_swipe_allowed: bool,
    ) {
        self.set_origin_hit(origin_hit);
        self.set_pressed_target(pressed_target);
        self.set_direct_drag_target(direct_drag_target);
        self.set_overlay_barrier(overlay_barrier);
        self.set_route_swipe_allowed(route_swipe_allowed);
    }

    /// Assigns exclusive gesture ownership to `winner`, optionally locking out click emission.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable gesture arena.
    /// - `ActiveGesture` - Winning exclusive gesture state.
    /// - `bool` - When `true`, forces `suppress_click` to remain raised for the rest of the sequence.
    pub(crate) fn claim(&mut self, winner: ActiveGesture, suppress_click: bool) {
        self.set_active(winner);
        if suppress_click {
            self.set_suppress_click(true);
        }
    }

    /// Marks `pointer_id` as freshly active to prevent stale-contact pruning.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable gesture arena.
    /// - `u64` - Active pointer identifier.
    pub(crate) fn record_pointer_activity(&mut self, pointer_id: u64) {
        let _: Option<f32> = self.get_mut_pointer_idle_secs().insert(pointer_id, 0.0);
    }

    /// Removes `pointer_id` from per-pointer idle tracking on release or cancel.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable gesture arena.
    /// - `u64` - Released or cancelled pointer identifier.
    pub(crate) fn remove_pointer(&mut self, pointer_id: u64) {
        let _: Option<f32> = self.get_mut_pointer_idle_secs().remove(&pointer_id);
    }

    /// Advances wheel isolation decay and per-pointer idle timers by `dt_secs`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable gesture arena.
    /// - `f32` - Elapsed frame timestep in seconds.
    pub(crate) fn tick_timers(&mut self, dt_secs: f32) {
        let step: f32 = dt_secs.max(0.0);
        let cur_wheel: f32 = self.get_wheel_isolation_secs();
        if cur_wheel > 0.0 {
            self.set_wheel_isolation_secs((cur_wheel - step).max(0.0));
        }
        for idle in self.get_mut_pointer_idle_secs().values_mut() {
            *idle = (*idle + step).min(60.0);
        }
    }

    /// Returns `true` when `pointer_id` has exceeded the stale-contact inactivity threshold.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Gesture arena reference.
    /// - `u64` - Pointer identifier to check.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the pointer has been inactive past `GESTURE_STALE_POINTER_TIMEOUT_SECS`.
    #[must_use]
    pub(crate) fn is_pointer_stale(&self, pointer_id: u64) -> bool {
        self.get_pointer_idle_secs()
            .get(&pointer_id)
            .copied()
            .unwrap_or(0.0)
            >= GESTURE_STALE_POINTER_TIMEOUT_SECS
    }

    /// Activates the brief wheel/trackpad scroll-burst hover isolation cooldown.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable gesture arena.
    pub(crate) fn mark_wheel_scroll_burst(&mut self) {
        self.set_wheel_isolation_secs(GESTURE_WHEEL_ISOLATION_SECS);
    }

    /// Returns `true` while a wheel or trackpad scroll burst is suppressing spurious hover transitions.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Gesture arena reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` during active wheel scroll isolation.
    #[must_use]
    pub(crate) fn is_wheel_isolated(&self) -> bool {
        self.get_wheel_isolation_secs() > 0.0
    }

    /// Finds the nearest Tier 0 `Portal` or `Position::Fixed` barrier enclosing `start`.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained scene tree.
    /// - `Option<NodeId>` - Leaf node handle.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Enclosing overlay barrier node, if any.
    #[must_use]
    pub(crate) fn find_overlay_barrier(tree: &SceneTree, start: Option<NodeId>) -> Option<NodeId> {
        let mut cursor: Option<NodeId> = start;
        while let Some(aid) = cursor {
            let Some(node) = tree.get_node(aid) else {
                break;
            };
            let is_barrier: bool = matches!(node.get_kind(), NodeKind::Portal)
                || node.style_ref().layout_ref().get_position() == Position::Fixed;
            if is_barrier {
                return Some(aid);
            }
            cursor = node.parent_id();
        }
        None
    }

    /// Evaluates touch motion from `origin` to `pos` using axis-locked slop and hysteresis to determine the exclusive gesture winner.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Gesture arena reference.
    /// - `&SceneTree` - Live retained scene tree.
    /// - `Point` - Current logical touch coordinates.
    ///
    /// # Returns
    ///
    /// - `GestureResolution` - Whether the contact stays `Pending`, yields an exclusive winner, or cancels tap/click.
    #[must_use]
    pub(crate) fn resolve_touch_move(&self, tree: &SceneTree, pos: Point) -> GestureResolution {
        if self.get_active() != ActiveGesture::Pending {
            return GestureResolution::StillPending;
        }
        let origin: Point = self.get_origin();
        let total_dx: f32 = pos.get_x() - origin.get_x();
        let total_dy: f32 = pos.get_y() - origin.get_y();
        let total_dist: f32 = total_dx.hypot(total_dy);
        if total_dist <= GESTURE_TOUCH_SLOP_DP {
            return GestureResolution::StillPending;
        }
        let abs_x: f32 = total_dx.abs();
        let abs_y: f32 = total_dy.abs();
        let vertical_dom: bool = abs_y >= abs_x * GESTURE_AXIS_HYSTERESIS_RATIO;
        let horizontal_dom: bool = abs_x >= abs_y * GESTURE_AXIS_HYSTERESIS_RATIO;
        if !vertical_dom && !horizontal_dom && total_dist < GESTURE_DIAGONAL_RESOLVE_DP {
            return GestureResolution::StillPending;
        }
        let is_vertical: bool = vertical_dom || (!horizontal_dom && abs_y >= abs_x);
        if is_vertical {
            match Self::find_vertical_scroll_winner(tree, self.origin_hit_node()) {
                Some(scroll_id) => GestureResolution::Won(ActiveGesture::VerticalScroll(scroll_id)),
                None => GestureResolution::CancelPending,
            }
        } else {
            match Self::find_horizontal_scroll_winner(tree, self.origin_hit_node(), total_dx) {
                Some(scroll_id) => {
                    GestureResolution::Won(ActiveGesture::HorizontalScroll(scroll_id))
                }
                None if self.get_route_swipe_allowed() => {
                    GestureResolution::Won(ActiveGesture::RouteSwipe)
                }
                None => GestureResolution::CancelPending,
            }
        }
    }

    /// Finds the innermost vertically scrollable ancestor up to any Tier 0 overlay or scroll-isolation barrier.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Live retained scene tree.
    /// - `Option<NodeId>` - Starting hit node handle.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Winning vertical scroll container handle, if any.
    #[must_use]
    pub(crate) fn find_vertical_scroll_winner(
        tree: &SceneTree,
        start: Option<NodeId>,
    ) -> Option<NodeId> {
        let mut fallback_scroll: Option<NodeId> = None;
        let mut hit_barrier: bool = false;
        let mut cursor: Option<NodeId> = start;
        while let Some(aid) = cursor {
            let Some(node) = tree.get_node(aid) else {
                break;
            };
            let style: &Style = node.style_ref();
            let is_scroll: bool = style.layout_ref().get_overflow() == Overflow::Scroll;
            if is_scroll {
                let lbox: LayoutBox = node.get_layout_box();
                let max_y: f32 = (lbox.get_content_size().get_height()
                    - lbox.get_content_bounds().get_height())
                .max(0.0);
                let has_scroll_cb: bool = node
                    .get_handlers()
                    .handler_for(&UiEventKind::Scroll(Point::default()))
                    .is_some();
                if max_y > 1.0 || node.virtual_scroll_metrics().is_some() || has_scroll_cb {
                    return Some(aid);
                }
                if fallback_scroll.is_none() {
                    fallback_scroll = Some(aid);
                }
            }
            let stops: bool = style.paint_ref().get_stop_scroll_propagation()
                || matches!(node.get_kind(), NodeKind::Portal)
                || style.layout_ref().get_position() == Position::Fixed;
            if stops {
                hit_barrier = true;
                break;
            }
            cursor = node.parent_id();
        }
        if fallback_scroll.is_some() || hit_barrier {
            return fallback_scroll;
        }
        if let Some(start_id) = start
            && let Some(start_node) = tree.get_node(start_id)
        {
            for &child_id in start_node.get_children().iter().rev() {
                if !tree.is_node_visible(child_id) {
                    continue;
                }
                if let Some(child) = tree.get_node(child_id) {
                    let cstyle: &Style = child.style_ref();
                    let blocked: bool = cstyle.paint_ref().get_pointer_events()
                        == PointerEvents::None
                        || cstyle.paint_ref().get_stop_scroll_propagation()
                        || matches!(child.get_kind(), NodeKind::Portal)
                        || cstyle.layout_ref().get_position() == Position::Fixed;
                    if !blocked && cstyle.layout_ref().get_overflow() == Overflow::Scroll {
                        let lbox: LayoutBox = child.get_layout_box();
                        let max_y: f32 = (lbox.get_content_size().get_height()
                            - lbox.get_content_bounds().get_height())
                        .max(0.0);
                        if max_y > 1.0 || child.virtual_scroll_metrics().is_some() {
                            return Some(child_id);
                        }
                    }
                }
            }
        }
        None
    }

    /// Finds the innermost horizontally scrollable ancestor with remaining extent in the drag direction.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Live retained scene tree.
    /// - `Option<NodeId>` - Starting hit node handle.
    /// - `f32` - Horizontal finger displacement (`pos.x - origin.x`; negative drags content right).
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Winning horizontal scroll container handle, if any.
    #[must_use]
    pub(crate) fn find_horizontal_scroll_winner(
        tree: &SceneTree,
        start: Option<NodeId>,
        finger_dx: f32,
    ) -> Option<NodeId> {
        let mut cursor: Option<NodeId> = start;
        while let Some(aid) = cursor {
            let Some(node) = tree.get_node(aid) else {
                break;
            };
            let style: &Style = node.style_ref();
            let is_scroll: bool = style.layout_ref().get_overflow() == Overflow::Scroll;
            if is_scroll {
                let lbox: LayoutBox = node.get_layout_box();
                let max_x: f32 = (lbox.get_content_size().get_width()
                    - lbox.get_content_bounds().get_width())
                .max(0.0);
                if max_x > 1.0 {
                    let cur_x: f32 = lbox.get_scroll_offset().get_x();
                    let can_scroll_dir: bool = if finger_dx < 0.0 {
                        cur_x < max_x - 0.5
                    } else if finger_dx > 0.0 {
                        cur_x > 0.5
                    } else {
                        false
                    };
                    if can_scroll_dir || style.paint_ref().get_stop_scroll_propagation() {
                        return Some(aid);
                    }
                }
            }
            let stops: bool = style.paint_ref().get_stop_scroll_propagation()
                || matches!(node.get_kind(), NodeKind::Portal)
                || style.layout_ref().get_position() == Position::Fixed;
            if stops {
                break;
            }
            cursor = node.parent_id();
        }
        None
    }
}
