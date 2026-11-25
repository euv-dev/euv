use super::*;

/// Easing evaluation functions for `Easing`.
impl Easing {
    /// Evaluates the easing curve for normalized progress `t` in `[0.0, 1.0]`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Easing curve variant.
    /// - `f32` - Linear progress clamped to `[0.0, 1.0]`.
    ///
    /// # Returns
    ///
    /// - `f32` - Eased interpolation factor.
    #[must_use]
    pub fn eval(&self, t: f32) -> f32 {
        let u: f32 = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => u,
            Self::EaseOutCubic => {
                let inv: f32 = 1.0 - u;
                1.0 - inv * inv * inv
            }
            Self::EaseInOutCubic => {
                if u < 0.5 {
                    4.0 * u * u * u
                } else {
                    let k: f32 = -2.0 * u + 2.0;
                    1.0 - (k * k * k) * 0.5
                }
            }
            Self::EaseOutBack => {
                let c1: f32 = 1.4;
                let c3: f32 = c1 + 1.0;
                let k: f32 = u - 1.0;
                1.0 + c3 * k * k * k + c1 * k * k
            }
        }
    }
}

/// Standard transition presets and sampling helpers for `NodeTransition`.
impl NodeTransition {
    /// Creates a standard route-switch entrance transition (`220ms EaseOutCubic`, fade + 12px upward glide).
    ///
    /// # Arguments
    ///
    /// - `NodeId` - Target page container node.
    ///
    /// # Returns
    ///
    /// - `Self` - Route entrance transition.
    #[must_use]
    pub fn route_enter(node: NodeId) -> Self {
        Self {
            node,
            elapsed: 0.0,
            duration: 0.22,
            easing: Easing::EaseOutCubic,
            from_opacity: 0.0,
            to_opacity: 1.0,
            from_tx: 0.0,
            to_tx: 0.0,
            from_ty: 12.0,
            to_ty: 0.0,
            from_scale: 1.0,
            to_scale: 1.0,
            hide_on_complete: false,
        }
    }

    /// Creates a pure opacity fade transition with optional `Display::None` on completion.
    ///
    /// # Arguments
    ///
    /// - `NodeId` - Target node handle.
    /// - `f32` - Starting opacity.
    /// - `f32` - Ending opacity.
    /// - `f32` - Duration in seconds.
    /// - `bool` - Whether to hide the node when the transition finishes.
    ///
    /// # Returns
    ///
    /// - `Self` - Fade transition.
    #[must_use]
    pub fn fade(node: NodeId, from: f32, to: f32, duration: f32, hide_on_complete: bool) -> Self {
        Self {
            node,
            elapsed: 0.0,
            duration: duration.max(0.016),
            easing: Easing::EaseOutCubic,
            from_opacity: from,
            to_opacity: to,
            from_tx: 0.0,
            to_tx: 0.0,
            from_ty: 0.0,
            to_ty: 0.0,
            from_scale: 1.0,
            to_scale: 1.0,
            hide_on_complete,
        }
    }

    /// Creates a slide-and-fade transition for drawers, sheets, popovers, dropdowns, and toasts.
    ///
    /// # Arguments
    ///
    /// - `NodeId` - Target node handle.
    /// - `Point` - Starting `(tx, ty)` offset in logical pixels.
    /// - `Point` - Ending `(tx, ty)` offset in logical pixels.
    /// - `f32` - Starting opacity.
    /// - `f32` - Ending opacity.
    /// - `f32` - Duration in seconds.
    /// - `bool` - Whether to hide the node upon completion.
    ///
    /// # Returns
    ///
    /// - `Self` - Slide + fade transition.
    #[must_use]
    pub fn slide_fade(
        node: NodeId,
        from_offset: Point,
        to_offset: Point,
        from_opacity: f32,
        to_opacity: f32,
        duration: f32,
        hide_on_complete: bool,
    ) -> Self {
        Self {
            node,
            elapsed: 0.0,
            duration: duration.max(0.016),
            easing: Easing::EaseOutCubic,
            from_opacity,
            to_opacity,
            from_tx: from_offset.get_x(),
            to_tx: to_offset.get_x(),
            from_ty: from_offset.get_y(),
            to_ty: to_offset.get_y(),
            from_scale: 1.0,
            to_scale: 1.0,
            hide_on_complete,
        }
    }

    /// Creates a subtle scale-and-rise modal dialog entrance transition (`200ms EaseOutCubic`).
    ///
    /// # Arguments
    ///
    /// - `NodeId` - Target dialog panel node.
    ///
    /// # Returns
    ///
    /// - `Self` - Modal dialog entrance transition.
    #[must_use]
    pub fn modal_enter(node: NodeId) -> Self {
        Self {
            node,
            elapsed: 0.0,
            duration: 0.20,
            easing: Easing::EaseOutCubic,
            from_opacity: 0.0,
            to_opacity: 1.0,
            from_tx: 0.0,
            to_tx: 0.0,
            from_ty: 14.0,
            to_ty: 0.0,
            from_scale: 0.95,
            to_scale: 1.0,
            hide_on_complete: false,
        }
    }

    /// Creates a modal dialog exit transition (`180ms EaseOutCubic`).
    ///
    /// # Arguments
    ///
    /// - `NodeId` - Target dialog panel node.
    ///
    /// # Returns
    ///
    /// - `Self` - Modal dialog exit transition.
    #[must_use]
    pub fn modal_exit(node: NodeId) -> Self {
        Self {
            node,
            elapsed: 0.0,
            duration: 0.18,
            easing: Easing::EaseOutCubic,
            from_opacity: 1.0,
            to_opacity: 0.0,
            from_tx: 0.0,
            to_tx: 0.0,
            from_ty: 0.0,
            to_ty: 10.0,
            from_scale: 1.0,
            to_scale: 0.96,
            hide_on_complete: false,
        }
    }

    /// Evaluates interpolated `(opacity, Transform2D)` at the current `elapsed` timestamp.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Transition reference.
    ///
    /// # Returns
    ///
    /// - `(f32, Transform2D)` - Current interpolated opacity and 2D transform.
    #[must_use]
    pub fn sample(&self) -> (f32, Transform2D) {
        let raw_t: f32 = (self.get_elapsed() / self.get_duration().max(0.001)).clamp(0.0, 1.0);
        let k: f32 = self.get_easing().eval(raw_t);
        let op: f32 = (self.get_from_opacity()
            + (self.get_to_opacity() - self.get_from_opacity()) * k)
            .clamp(0.0, 1.0);
        let tx: f32 = self.get_from_tx() + (self.get_to_tx() - self.get_from_tx()) * k;
        let ty: f32 = self.get_from_ty() + (self.get_to_ty() - self.get_from_ty()) * k;
        let sc: f32 = (self.get_from_scale() + (self.get_to_scale() - self.get_from_scale()) * k)
            .clamp(0.05, 4.0);
        (op, Transform2D::new(tx, ty, sc, sc, 0.0))
    }

    /// Returns `true` when the transition has reached or exceeded its target duration.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Transition reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether this transition is finished.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.get_elapsed() >= self.get_duration()
    }
}
